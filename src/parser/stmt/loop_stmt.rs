use crate::lexer::diagnostic::logic::issue::IssueType;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::ast::{NONE, NodeIdx};
use crate::parser::tree::node::Key;
use crate::parser::types::logic::Type;

impl Parser{
    pub fn parse_for(&mut self) -> Result<NodeIdx, IssueType>{
        self.build_scope();
        self.eat();

        let ident_key = self.current as Key;
        let loop_ident_idx = self.get_tok_node_idx(&Token::Ident);
        self.sign_var(ident_key);
        self.unexpected_token(&Token::Ident)?;

        self.unexpected_token(&Token::In)?;

        let start_number_idx = {
            let expr_idx = self.parse_expression();
            if !self.is_matches_type(Type::i32){
                panic!("[Line {:?}] Expected Integer", self.current_line());
            }
            expr_idx
        };

        self.unexpected_token(&Token::Range)?;

        let end_number_idx = {
            let expr_idx = self.parse_expression();
            if !self.is_matches_type(Type::i32){
                panic!("[Line {:?}] Expected Integer", self.current_line());
            }
            expr_idx
        };

       self.unexpected_token(&Token::Vertical)?;

        let step_number_idx = {
            let expr_idx = self.parse_expression();
            if !self.is_matches_type(Type::i32){
                panic!("[Line {:?}] Expected Integer", self.current_line());
            }
            expr_idx
        };


        self.unexpected_token(&Token::Lbrace)?;
        let block_idx = self.parse_stmts()?;
        self.unclosed_delimiter(&Token::Rbrace)?;

        let start = self.ast.edge_start();
        self.ast.edges.push(loop_ident_idx);
        self.ast.edges.push(start_number_idx?);
        self.ast.edges.push(end_number_idx?);
        self.ast.edges.push(step_number_idx?);
        self.ast.edges.push(block_idx);
        let end = self.ast.edge_end();


        self.destroy_scope();
        Ok(self.push_node(Token::ForStmt, NONE, start, end))
    }

    pub fn parse_while(&mut self) -> Result<NodeIdx, IssueType>{
        self.build_scope();

        self.eat();

        let expr_idx = {
            let expr_idx = self.parse_expression()?;
            if !self.is_matches_type(Type::bool){
                panic!("[Line {:?}] Expected the bool type", self.current_line())
            }
            expr_idx
        };

        self.unexpected_token(&Token::Lbrace)?;
        let stmts_idx = self.parse_stmts()?;
        self.unclosed_delimiter(&Token::Rbrace)?;

        self.destroy_scope();

        let start = self.ast.edge_start();
        self.ast.edges.push(expr_idx);
        self.ast.edges.push(stmts_idx);
        let end = self.ast.edge_end();

        Ok(self.push_node(Token::WhileStmt, NONE, start, end))
    }



    
    
    
}
