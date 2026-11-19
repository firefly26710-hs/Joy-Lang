use crate::lexer::tokenizer::source::token::Token;
use crate::lexer::tokenizer::source::token::Token::UpdateKw;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::tree::ast::{NONE, NodeIdx};
use crate::parser::tree::node::Key;

impl Parser{
    


    pub fn parse_var(&mut self) -> Result<NodeIdx, ParserIssue>{
        self.eat();
        //var name
        let var_name_idx = self.push_leaf(Token::Ident, self.current as Key)?;


        self.matches_token(&Token::Ident)?;

        //type
        let var_type_idx = if !matches!(self.peek(), Token::Eq){
            self.matches_token(&Token::Colon)?;
            self.matches_token(&Token::Type)?;
             self.push_leaf(Token::Type, (self.current - 1) as Key)?
        }else{
            self.push_leaf(Token::Type, NONE)?
        };

        self.matches_token(&Token::Eq)?;

        let expr_idx = self.parse_expression()?;


        self.matches_token(&Token::Scolon)?;

        let edge_start = self.ast.edge_start();
        self.ast.edges.push(var_name_idx);
        self.ast.edges.push(var_type_idx);
        self.ast.edges.push(expr_idx);
        let edge_end = self.ast.edge_end();

        Ok(self.push_node(Token::VarKw, NONE, edge_start, edge_end))
    }


    pub fn ident_branch(&mut self) -> Result<NodeIdx, ParserIssue>{
        match self.peek_next(){
            Token::Lparen => self.parse_func_call(),
            Token::Eq |Token::AddEq | Token::SubEq | Token::MulEq | Token::DivEq
            |Token::ModEq | Token::Scolon => self.parse_var_update(),
            _ => panic!("[Line {:?}] Expected ; ", self.current_line())
        }
    }

    fn parse_var_update(&mut self) -> Result<NodeIdx, ParserIssue>{

        let ident_name_idx = self.push_leaf(Token::Ident, self.current as Key)?;
        self.matches_token(&Token::Ident)?;
        
        let op_tok =self.eat();
        let op_idx = 
            self.push_leaf(op_tok, (self.current - 1) as Key)?;

        let expr_idx = self.parse_expression()?;

       self.matches_token(&Token::Scolon)?;

        let start = self.ast.edge_start();
        self.ast.edges.push(ident_name_idx);
        self.ast.edges.push(op_idx);
        self.ast.edges.push(expr_idx);
        let end = self.ast.edge_end();


       Ok( self.push_node(UpdateKw, NONE, start, end))
    }



}