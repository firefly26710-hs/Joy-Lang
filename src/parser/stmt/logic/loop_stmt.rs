use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::tree::ast::{NONE, NodeIdx};
use crate::parser::tree::node::Key;

impl Parser{
    pub fn parse_for(&mut self) -> Result<NodeIdx, ParserIssue>{
        self.eat();
        let loop_ident_idx = self.push_leaf(Token::Ident, self.current as Key)?;
        self.matches_token(&Token::Ident)?;

        self.matches_token(&Token::In)?;

        let start_number_idx =self.parse_expression()?;


        self.matches_token(&Token::Range)?;

        let end_number_idx = self.parse_expression()?;

       self.matches_token(&Token::Vertical)?;

        let step_number_idx = self.parse_expression()?;



        let block_idx = self.parse_scope()?;

        let start = self.ast.edge_start();
        self.ast.edges.push(loop_ident_idx);
        self.ast.edges.push(start_number_idx);
        self.ast.edges.push(end_number_idx);
        self.ast.edges.push(step_number_idx);
        self.ast.edges.push(block_idx);
        let end = self.ast.edge_end();
        Ok(self.push_node(Token::ForStmt, NONE, start, end))
    }

    pub fn parse_while(&mut self) -> Result<NodeIdx, ParserIssue>{
        self.eat();
        let expr_idx = self.parse_expression()?;

        let stmts_idx = self.parse_scope()?;

        let start = self.ast.edge_start();
        self.ast.edges.push(expr_idx);
        self.ast.edges.push(stmts_idx);
        let end = self.ast.edge_end();
        Ok(self.push_node(Token::WhileStmt, NONE, start, end))
    }



    
    
    
}
