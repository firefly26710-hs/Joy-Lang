use crate::lexer::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;

impl Parser {
    pub fn ret_matches_token_idx(&mut self, tok: &Token, error_msg: &str) -> NodeIdx{
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