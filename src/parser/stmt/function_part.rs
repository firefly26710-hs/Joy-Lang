use crate::lexer::diagnostic::logic::issue::IssueType;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::NONE;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;

use crate::parser::center::par::Parser;
use crate::parser::types::logic::Type;

impl Parser{
    pub fn parse_function(&mut self) -> Result<NodeIdx, IssueType>{
        self.build_scope();


        self.matches_token(&Token::Fn, "Can't find function");
        let func_name_key = self.current as Key;
        let func_name_node_idx = self.get_tok_node_idx(&Token::Ident);
        self.sign_func((self.current) as Key);
        self.stmt.func_recorder.push(func_name_node_idx);
        self.unexpected_token(&Token::Ident)?;


        self.unexpected_token(&Token::Lparen)?;

        let has_declared_arg = self.peek() == &Token::Ident;
        let param_start = if has_declared_arg{ self.params_start() }else{NONE};

        if has_declared_arg{
            loop{
                let arg_name_key = self.current as Key;
                let arg_idx =self.get_tok_node_idx(&Token::Ident);

                self.unexpected_token(&Token::Ident)?;
                self.unexpected_token(&Token::Colon)?;
                self.unexpected_token(&Token::Type)?;

                let arg_type = self.type_mapping((self.current - 1) as Key);


                self.stmt.func_recorder.push(arg_idx);
                self.scope.var_scope_stack.push(arg_name_key);
                self.insert_var_type(arg_name_key, arg_type);

                match self.peek(){
                    Token::Rparen => break,
                    Token::Comma => self.eat(),
                    _ => break,
                };
            }
        }

        let param_end = if has_declared_arg{self.params_end()}else{NONE};

        self.unexpected_token(&Token::Rparen)?;
        self.unexpected_token(&Token::Eq)?;

        let ret_type = {
            self.unexpected_token(&Token::Type)?;
            self.type_mapping((self.current - 1) as Key)
        };

        self.insert_func_type(func_name_key, param_start, param_end, ret_type.clone());


        self.unexpected_token(&Token::Lbrace)?;
        let stmt_idx = self.parse_stmts()?;
        self.unclosed_delimiter(&Token::Rbrace)?;

        let start = self.ast.edge_start();
        self.stmt.func_recorder.push(stmt_idx);
        self.ast.edges.extend(self.stmt.func_recorder.drain(..));
        self.destroy_scope();

        let end = self.ast.edge_end();

        self.check_legal_ret_stmt_position(&ret_type, stmt_idx);
       Ok( self.push_node(Token::FuncStmt, NONE, start, end))


    }



    pub fn parse_ret(&mut self) -> Result<NodeIdx, IssueType>{
        self.eat();

        let expr_idx = if matches!(self.last_ret_type(), Type::void){
            NONE
        }else if !matches!(self.last_ret_type(), Type::void) && self.is_expr(self.peek()){
            let expr_idx = self.parse_expression()?;
            self.is_matches_type(self.last_ret_type().clone());
            expr_idx
        }else {
            panic!("[Line {:?}] Expected {:?} but found ()", self.current_line(), self.last_ret_type())
        };


        self.matches_token(&Token::Scolon, "Expected ;");

        let start = self.ast.edge_start();
        self.ast.edges.push(NONE);
        self.ast.edges.push(expr_idx);
        let end = self.ast.edge_end();

       Ok( self.push_node(Token::RetStmt, NONE, start, end))
    }

    pub fn parse_func_call(&mut self) -> Result<NodeIdx, IssueType>{
        let expr_func_idx = self.expr_func_call();

        self.matches_token(&Token::Scolon, "Expected ;");

        expr_func_idx
    }

    pub fn expr_func_call(&mut self) -> Result<NodeIdx, IssueType> {
        let func_name_key = self.find_same_func_key(self.current as Key)?;
        let func_name_idx = {
            self.eat();
            self.push_leaf(Token::Ident, func_name_key)
        };

        match self.look_ret_type(func_name_key){
            Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128 => self.ast.int_node_sum += 1,
            Type::f32 | Type::f64 => self.ast.flt_node_sum += 1,
            Type::bool => self.ast.bool_node_sum += 1,
            Type::String => self.ast.is_string = true,
            Type::void => panic!("[Line {:?}] Expression can't with void type", self.current_line())
        };

        self.stmt.func_call_recorder.push(func_name_idx?);
        self.matches_token(&Token::Lparen, "Expected '('");

        let param_type_intervals = self.look_param_interval(func_name_key);
        let start = param_type_intervals.start ;
        let end = param_type_intervals.end ;

        let has_param = start != NONE && end != NONE;


        if has_param {
            let len = self.table.var_types[start as usize..=end as usize].len();
            let mut counter = 0;

            for i in 0..len {
                let expr_idx = self.parse_expression();

                let param_type = &self.table.var_types[start as usize..=end as usize][i];
                let ty = &param_type.clone();
                if !self.is_matches_type(ty.clone()) {
                    panic!("[Line {:?}] Mismatch Type, found {:?}", self.current_line(),[i])
                }
                counter += 1;
                self.stmt.func_call_recorder.push(expr_idx?);

                match self.peek() {
                    Token::Comma => self.eat(),
                    Token::Rparen => break,
                    _ => break,
                };
            }

            if counter != len {
                panic!("[Line {:?}] Not Declared all args in function call", self.current_line())
            }
        }
        self.matches_token(&Token::Rparen, "Expected )");

        let start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.func_call_recorder.drain(..));
        let end = self.ast.edge_end();

        Ok(self.push_node(Token::CallStmt, NONE, start, end))
    }


    pub fn parse_print(&mut self) -> Result<NodeIdx, IssueType>{
        self.eat();

        self.matches_token(&Token::Lparen, "Expected (");
        let type_idx = self.get_tok_node_idx(&Token::Type);

        let ty = self.type_mapping((self.current - 1) as Key);
        self.matches_token(&Token::Comma, "Expected ,");
        let expr_idx = self.parse_expression();

        if !self.is_matches_type(ty){
            panic!("[Line {:?}] Expected", self.current_line());
        }

        self.matches_token(&Token::Rparen, "Expected )");
        self.matches_token(&Token::Scolon, "Expected ;");

        let edges_start = self.ast.edge_start();
        self.ast.edges.push(type_idx);
        self.ast.edges.push(expr_idx?);
        let edges_end = self.ast.edge_end();

        Ok(self.push_node(Token::PrintStmt, NONE, edges_start, edges_end))


    }

    fn check_legal_ret_stmt_position(&self, ret_type : &Type, stmt_idx : NodeIdx){
        if matches!(ret_type, Type::void){
            return;
        }


        if self.stmt.already_full_ret_in_if {
            return;
        }

        let stmt_edge = &self.ast.nodes[stmt_idx as usize].edge_span;
        let start = stmt_edge.start as usize;
        let end = stmt_edge.end as usize;
        let stmt_node_idx = *self.ast.edges[start..=end].last().unwrap();
        let stmt_tok = &self.ast.nodes[stmt_node_idx as usize].token;

        if matches!(stmt_tok, Token::RetStmt){
            return;
        }else{
            panic!("Expected return {:?}",  ret_type );
        }

    }


}