use crate::par::tree::node::Key;
use crate::par::tree::ast::{NodeIdx, NONE};
use crate::par::build_tree::parser::Parser;
use crate::lex::tokenize::token::Token;
use crate::par::build_tree::ty::Type;

impl Parser{
    pub fn parse_for(&mut self) -> NodeIdx{
        self.scope.build_loop_scope();
        self.eat();

        let ident_key = self.current as Key;
        let loop_ident_idx = self.ret_matches_token_idx(&Token::Ident ,"Expected the identifier");
        self.scope.sign_var(ident_key);


        self.matches_token(&Token::In, "Expected in");
        self.matches_token(&Token::Lbracket, "Expected [");

        let start_number_idx = {
            let expr_idx = self.parse_expression();
            if !self.is_matches_type(Type::i32){
                panic!("[Line {:?}] Expected Integer", self.current_line());
            }
            expr_idx
        };

        self.matches_token(&Token::Arrow, "Expected ->");

        let end_number_idx = {
            let expr_idx = self.parse_expression();
            if !self.is_matches_type(Type::i32){
                panic!("[Line {:?}] Expected Integer", self.current_line());
            }
            expr_idx
        };

        self.matches_token(&Token::Vertical, "Expected |");

        let step_number_idx = {
            let expr_idx = self.parse_expression();
            if !self.is_matches_type(Type::i32){
                panic!("[Line {:?}] Expected Integer", self.current_line());
            }
            expr_idx
        };

        self.matches_token(&Token::Rbracket, "Expected ]");


        self.matches_token(&Token::Lbrace, "Expected {");
        let block_idx = self.parse_stmts();
        self.matches_token(&Token::Rbrace, "Expected }");

        let start = self.ast.edge_start();
        self.ast.edges.push(loop_ident_idx);
        self.ast.edges.push(start_number_idx);
        self.ast.edges.push(end_number_idx);
        self.ast.edges.push(step_number_idx);
        self.ast.edges.push(block_idx);
        let end = self.ast.edge_end();


        self.scope.destroy_loop_scope();

        self.push_node(Token::ForStmt, NONE, start, end)
    }

    pub fn parse_while(&mut self) -> NodeIdx{
        self.scope.build_loop_scope();

        self.eat();

        let expr_idx = {
            let expr_idx = self.parse_expression();
            if !self.is_matches_type(Type::bool){
                panic!("[Line {:?}] Expected the bool type", self.current_line())
            }
            expr_idx
        };

        self.matches_token(&Token::Lbrace, "Expected {");
        let stmts_idx = self.parse_stmts();
        self.matches_token(&Token::Rbrace, "Expected }");

        self.scope.destroy_loop_scope();

        let start = self.ast.edge_start();
        self.ast.edges.push(expr_idx);
        self.ast.edges.push(stmts_idx);
        let end = self.ast.edge_end();

        self.push_node(Token::WhileStmt, NONE, start, end)
    }



    
    
    
}
