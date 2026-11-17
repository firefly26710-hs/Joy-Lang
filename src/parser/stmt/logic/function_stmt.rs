use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::NONE;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;

use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::stmt::logic::expr::is_expr_val;
use crate::parser::types::typ::Type;

impl Parser{
    pub fn parse_function(&mut self) -> Result<NodeIdx, ParserIssue>{
        self.build_scope();
        self.table.var_key_start = self.table.var_keys.len();
        
        self.matches_token(&Token::Fn)?;
        let func_name_key = self.current as Key;
        let func_name_node_idx = self.get_tok_node_idx(&Token::Ident);
        self.sign_func(self.current as Key);
        self.stmt.func_recorder.push(func_name_node_idx);
        self.matches_token(&Token::Ident)?;

        self.matches_token(&Token::Lparen)?;

        let has_declared_arg = self.peek() == Token::Ident;
        let param_start = if has_declared_arg{ self.params_start() }else{NONE};

        if has_declared_arg{
            loop{
                let arg_name_key = self.current as Key;
                let arg_idx =self.get_tok_node_idx(&Token::Ident);

                self.matches_token(&Token::Ident)?;
                self.matches_token(&Token::Colon)?;
                self.matches_token(&Token::Type)?;

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

        self.matches_token(&Token::Rparen)?;
        self.matches_token(&Token::Eq)?;
        self.matches_token(&Token::Type)?;
        let ret_type = self.type_mapping((self.current - 1) as Key);

        self.insert_func_type(func_name_key, param_start, param_end, ret_type.clone());


        let stmt_idx = self.parse_scope()?;

        let start = self.ast.edge_start();
        self.stmt.func_recorder.push(stmt_idx);
        self.ast.edges.extend(self.stmt.func_recorder.drain(..));
        self.destroy_scope();

        let end = self.ast.edge_end();

        self.not_ret_void_but_no_ret_stmt(&ret_type, stmt_idx, func_name_key)?;
        Ok( self.push_node(Token::FuncStmt, NONE, start, end))


    }



    pub fn parse_ret(&mut self) -> Result<NodeIdx, ParserIssue>{
        self.eat();
        let expr_idx : Option<NodeIdx> = if is_expr_val(&self.peek()){
            Some(self.parse_expression()?)
        }else{
            None
        };
        let ret_type = self.last_ret_type();
        self.matches_ret_type(expr_idx, &ret_type.clone(), self.current)?;
        self.matches_token(&Token::Scolon)?;

        let start = self.ast.edge_start();
        self.ast.edges.push(NONE);
        if expr_idx != None{
            self.ast.edges.push(expr_idx.unwrap())
        }else{
            self.ast.edges.push(NONE);
        }
        let end = self.ast.edge_end();

       Ok( self.push_node(Token::RetStmt, NONE, start, end))
    }

    pub fn parse_func_call(&mut self) -> Result<NodeIdx, ParserIssue>{
        let expr_func_idx = self.expr_func_call()?;

       self.matches_token(&Token::Scolon)?;

        Ok(expr_func_idx)
    }

    pub fn expr_func_call(&mut self) -> Result<NodeIdx, ParserIssue> {
        let func_name_key = self.find_same_func_key(self.current as Key);
        self.fn_not_exist(func_name_key)?;

        let func_name_idx = {
            self.eat();
            self.push_leaf(Token::Ident, func_name_key)
        };

        

        self.stmt.func_call_recorder.push(func_name_idx?);
        self.matches_token(&Token::Lparen)?;

        let param_type_intervals = self.look_param_interval(func_name_key);
        let start = param_type_intervals.start ;
        let end = param_type_intervals.end ;

        let has_param = start != NONE && end != NONE;


        if has_param {
            let len = self.table.var_types[start as usize..=end as usize].len();
            let mut counter = 0;

            for i in 0..len {
                let expr_idx = self.parse_expression()?;

                let param_type = &self.table.var_types[start as usize..=end as usize][i];
                let ty = &param_type.clone();
                
                self.matches_type(ty, expr_idx)?;
                counter += 1;
                self.stmt.func_call_recorder.push(expr_idx);

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
        self.matches_token(&Token::Rparen)?;

        let start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.func_call_recorder.drain(..));
        let end = self.ast.edge_end();

        Ok(self.push_node(Token::CallStmt, NONE, start, end))
    }


    pub fn parse_print(&mut self) -> Result<NodeIdx, ParserIssue>{
        self.eat();

       self.matches_token(&Token::Lparen)?;
        let type_idx = self.get_tok_node_idx(&Token::Type);
        self.matches_token(&Token::Type)?;

        let ty = self.type_mapping((self.current - 1) as Key);
        self.matches_token(&Token::Comma)?;
        let expr_idx = self.parse_expression()?;
        self.matches_type(&ty, expr_idx)?;
        self.matches_token(&Token::Rparen)?;
        self.matches_token(&Token::Scolon)?;

        let edges_start = self.ast.edge_start();
        self.ast.edges.push(type_idx);
        self.ast.edges.push(expr_idx);
        let edges_end = self.ast.edge_end();

        Ok(self.push_node(Token::PrintStmt, NONE, edges_start, edges_end))


    }

    fn not_ret_void_but_no_ret_stmt(
        &mut self,
        ret_type : &Type,
        stmt_idx : NodeIdx,
        func_name_key : Key
    ) -> Result<(), ParserIssue>{
        if matches!(ret_type, Type::void){
            return Ok(());
        }


        if self.stmt.already_full_ret_in_if {
            return Ok(());
        }

        let stmt_edge = &self.ast.nodes[stmt_idx as usize].edge_span;
        let start = stmt_edge.start as usize;
        let end = stmt_edge.end as usize;
        let stmt_node_idx = *self.ast.edges[start..=end].last().unwrap();
        if stmt_node_idx == NONE{
            return Err(self.no_return_void_but_no_return_issue(func_name_key, ret_type))
        }
        let stmt_tok = &self.ast.nodes[stmt_node_idx as usize].token;

        if matches!(stmt_tok, Token::RetStmt){
            Ok(())
        }else{
            Err(self.no_return_void_but_no_return_issue(func_name_key, ret_type))
        }


    }


}