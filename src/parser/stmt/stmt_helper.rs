use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;

impl Parser {
    pub fn get_tok_node_idx(&mut self, tok: &Token, error_msg: &str) -> NodeIdx{
        let idx = self.push_leaf(
            tok.clone(), self.current as Key
        );
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