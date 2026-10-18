use crate::lex::iterval::{End, Start};
use crate::lex::token::Token;
use crate::par::ast::ast::NONE;
use crate::par::ast::node::Key;
use crate::par::parser::parser::{NodeIdx, Parser};
use crate::par::parser::ty::Type;

impl Parser{
    
    pub fn parse_stmts(&mut self) -> NodeIdx{
        let start_idx = self.func_recorder.len();
        while self.peek() != &Token::Rbrace{
            let stmt_idx = self.matches_stmts();
            self.func_recorder.push(stmt_idx);
        }

        let edge_start : Start = self.ast.edge_start();
        self.ast.edges.extend(self.func_recorder.drain(start_idx..));
        let edge_end : End = self.ast.edge_end();

        self.push_node(Token::BlockStmt, NONE, edge_start, edge_end)
    }

    fn matches_stmts(&mut self) -> NodeIdx{
        match self.peek(){
            Token::Var    => self.parse_var(),
            Token::If     => self.parse_if(),
            Token::For    => self.parse_for(),
            Token::While  => self.parse_while(),
            Token::Ident  => self.ident_branch(),
            Token::Ret    => self.parse_ret(),
            Token::Brk   => self.parse_break(),
            Token::Ctn   => self.parse_ctn(),
            _ => panic!("[Line {:?}] Find not exist statement {:?}", self.current_line(), self.peek())
        }
    }


    fn parse_var(&mut self) -> NodeIdx{
        self.eat();

        let var_name_key = self.current as Key;
        let var_name_idx = self.ret_matches_token_idx(
            &Token::Ident, "Expected the variable name"
        );
        self.scope.sign_var(var_name_key);

        let declared_type = if self.peek() != &Token::Equal{
            self.matches_token(&Token::Colon, "Expected :");
            let ty = self.type_mapping(self.current as Key);
            self.eat();
            ty
        }else{
            Type::void
        };

        self.matches_token(&Token::Equal, "Expected =");

        let expr_idx = self.parse_expression();
        self.matches_and_insert_var_type(var_name_key, declared_type);

        self.matches_token(&Token::Scolon, "Expected ;");


        let edge_start = self.ast.edge_start();
        self.ast.edges.push(var_name_idx);
        self.ast.edges.push(expr_idx);
        let edge_end = self.ast.edge_end();

        self.push_node(Token::LetStmt, NONE, edge_start, edge_end)
    }


    fn ident_branch(&mut self) -> NodeIdx{
        match self.peek_next(){
            Token::Lparen => self.parse_func_call(),
            Token::Equal |Token::PlusEq | Token::MinusEq | Token::MulEq | Token::DivEq
            |Token::ModEq | Token::Scolon => self.parse_var_update(),
            _ => panic!("[Line {:?}] Expected ; ", self.current_line())
        }
    }

    fn parse_var_update(&mut self) -> NodeIdx{
        let key = self.find_same_ident_key(self.current as Key);

        let ident_name_idx = {
            self.eat();
            self.push_leaf(Token::Ident, key)
        };

        let op_tok_key = self.current as Key;
        let op_tok = self.eat();

        let expr_idx = self.parse_expression();
        let ident_type = self.table.look_var_type(key);
        if !self.is_matches_type(ident_type.clone()){
            panic!("[Line {:?}] {:?} Mismatch Type ", self.current_line(), self.look_string(key))
        }

        self.matches_token(&Token::Scolon, "Expected ;");

        let start = self.ast.edge_start();
        self.ast.edges.push(ident_name_idx);
        self.ast.edges.push(expr_idx);
        let end = self.ast.edge_end();

        self.push_node(op_tok, op_tok_key, start, end)
    }
 

    fn parse_break(&mut self) -> NodeIdx{
        if !self.scope.is_in_loop(){
            panic!("[Line {:?}] 'break' can't not in loop", self.current_line());
        }

        self.eat();

        self.matches_token(&Token::Scolon, "Expected ;");

        self.push_leaf(Token::BrkStmt, NONE)
    }

    fn parse_ctn(&mut self) -> NodeIdx{
        if !self.scope.is_in_loop(){
            panic!("[Line {:?}] 'break' can't not in loop", self.current_line());
        }

        self.eat();

        self.matches_token(&Token::Scolon, "Expected ;");

        self.push_leaf(Token::CtnStmt, NONE)

    }




    pub fn ret_matches_token_idx(&mut self, tok : &Token, error_msg : &str) -> NodeIdx{
        let idx = self.push_leaf(
            tok.clone(), self.current as Key
        );
        self.matches_token(tok, error_msg);
        idx
    }
    pub fn matches_token(&mut self , tok : &Token, error_msg : &str){
        if self.peek() == tok{
            self.eat();
        }else{
            panic!("[Line {:?}] {:?}", self.current_line(), error_msg);
        }

    }


}
