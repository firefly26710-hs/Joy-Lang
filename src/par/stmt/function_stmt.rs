use crate::lex::token::Token;
use crate::par::ast::ast::NONE;
use crate::par::ast::node::Key;
use crate::par::parser::parser::NodeIdx;
use crate::par::parser::parser::Parser;
use crate::par::parser::ty::Type;

impl Parser{
    pub fn parse_function(&mut self) -> NodeIdx{
        self.scope.build_func_scope();


        self.matches_token(&Token::Func, "Can't find function");
        let func_name_key = self.current as Key;
        let func_name_node_idx = self.ret_matches_token_idx(
            &Token::Ident, "Expected a function name"
        );
        self.func_recorder.push(func_name_node_idx);
        self.scope.sign_func((self.current - 1) as Key);
        self.matches_token(&Token::Lparen, "Expected a (");



        let has_declared_arg = self.peek() == &Token::Ident && self.peek_next() == &Token::Colon;
        let param_start = if has_declared_arg{ self.table.params_start() }else{NONE};

        if has_declared_arg{
            loop{
                let arg_name_key = self.current as Key;
                let arg_idx =self.ret_matches_token_idx(&Token::Ident, "");
                self.matches_token(&Token::Colon, "Expected : ");

                let arg_type = self.type_mapping(self.current as Key);
                self.eat();


                self.func_recorder.push(arg_idx);
                self.scope.var_scope_stack.push(arg_name_key);
                self.table.insert_var_type(arg_name_key, arg_type);

                match self.peek(){
                    Token::Rparen => break,
                    Token::Comma => self.eat(),
                    _ => break,
                };
            }
        }

        let param_end = if has_declared_arg{self.table.params_end()}else{NONE};

        self.matches_token(&Token::Rparen, "Expected a )");
        self.matches_token(&Token::Equal, "Expected =");

        let ret_type = {
            self.matches_token(&Token::Type, "Expected a type");
            self.type_mapping((self.current - 1) as Key)
        };

        self.table.insert_func_type(func_name_key, param_start, param_end, ret_type);


        self.matches_token(&Token::Lbrace, "Expected {");
        let stmt_idx = self.parse_stmts();
        self.matches_token(&Token::Rbrace, "Expected }");

        let start = self.ast.edge_start();
        self.func_recorder.push(stmt_idx);
        self.ast.edges.extend(self.func_recorder.drain(..));
        self.scope.destroy_func_scope();

        let end = self.ast.edge_end();

        self.push_node(Token::FuncStmt, NONE, start, end)
    }



    pub fn parse_ret(&mut self) -> NodeIdx{
        self.eat();

        let expr_idx = if !matches!(self.table.get_last_ret_type(), Type::void){
            let expr_idx = self.parse_expression();

            let ret_type = self.table.get_last_ret_type().clone();
            self.matches_var_type(&ret_type);
            expr_idx
        }else if !matches!(self.table.get_last_ret_type(), Type::void) &&
            self.is_expr(self.peek()) {
            panic!(
                "[Line {:?}] Expected {:?} but found ()"
                , self.current_line(),
                self.table.get_last_ret_type()
            )
        }else {
            NONE
        };

        self.matches_token(&Token::Scolon, "Expected ;");

        let start = self.ast.edge_start();
        self.ast.edges.push(NONE);
        self.ast.edges.push(expr_idx);
        let end = self.ast.edge_end();

        self.push_node(Token::RetStmt, NONE, start, end)
    }

    pub fn parse_func_call(&mut self) -> NodeIdx{
        let func_call_idx = self.expr_func_call();
        self.matches_token(&Token::Scolon, "Expected ;");
        func_call_idx
    }

    pub fn expr_func_call(&mut self) -> NodeIdx {
        let func_name_key = self.find_same_func_key(self.current as Key);
        let func_name_idx = {
            self.eat();
            self.push_leaf(Token::Ident, func_name_key)
        };
        self.func_call_recorder.push(func_name_idx);
        self.matches_token(&Token::Lparen, "Expected '('");

        let param_type_intervals = self.table.look_param_interval(func_name_key);
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
                self.func_call_recorder.push(expr_idx);

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
        self.ast.edges.extend(self.func_call_recorder.drain(..));
        let end = self.ast.edge_end();

        self.push_node(Token::CallStmt, NONE, start, end)
    }


}