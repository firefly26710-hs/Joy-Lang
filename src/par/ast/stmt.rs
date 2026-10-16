use crate::lex::token::Token;
use crate::par::ast::ast::NONE;
use crate::par::ast::node::Key;
use crate::par::parser::{NodeIdx, Parser};

impl Parser{
    pub fn parse_function(&mut self) -> NodeIdx{
        self.scope.build_func_scope();


        self.matches_token(&Token::Func, "Can't find function");
        let func_name_key = self.current as Key;
        let func_name_node_idx = self.ret_matches_token_idx(
            &Token::Ident, "Expected a function name"
        );
        self.stmt_recorder.push(func_name_node_idx);
        self.scope.sign_func((self.current - 1) as Key);
        self.matches_token(&Token::Lparen, "Expected a (");



        let has_declared_arg = self.peek() == &Token::Ident;

        let param_start = if has_declared_arg{ self.table.params_start() }else{NONE};

        if has_declared_arg{
            loop{
                let arg_name_key = self.current as Key;
                let arg_idx =self.ret_matches_token_idx(&Token::Ident, "");
                self.matches_token(&Token::Colon, "Expected : ");

                let arg_type = self.type_mapping(self.current as Key);
                self.eat();


                self.stmt_recorder.push(arg_idx);
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
        self.matches_token(&Token::Arrow, "Expected ->");

        let ret_type = {
            self.matches_token(&Token::Type, "Expected a type");
             self.type_mapping((self.current - 1) as Key)
        };

        self.table.insert_func_type(func_name_key, param_start, param_end, ret_type);


        let start = self.ast.childs_start();

        self.matches_token(&Token::Lbrace, "Expected {");
        let stmt_idx = self.block_stmt();
        self.matches_token(&Token::Rbrace, "Expected }");

        self.stmt_recorder.push(stmt_idx);
        self.ast.childs.extend(self.stmt_recorder.drain(self.prev_end..));
        self.scope.destroy_func_scope();

        let end = self.ast.childs_end();

        self.push_node(Token::FuncStmt, NONE, start, end)
    }

    pub fn block_stmt(&mut self) -> NodeIdx{
        todo!()
    }

    #[inline(always)]
    pub fn ret_matches_token_idx(&mut self, tok : &Token, error_msg : &str) -> NodeIdx{
        dbg!(tok);
        dbg!(self.peek());
        let idx = self.push_leaf(
            tok.clone(), self.current as Key
        );
        self.matches_token(tok, error_msg);
        idx
    }
    #[inline(always)]
    pub fn matches_token(&mut self , tok : &Token, error_msg : &str){
        if self.peek() == tok{
            self.eat();
        }else{
            panic!("[Line {:?}] {:?}", self.current_line(), error_msg);
        }

    }


}
