use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;

impl Parser {
    pub fn get_tok_node_idx(&mut self, tok: &Token) -> NodeIdx{
        let idx = self.push_leaf(
            tok.clone(), self.current as Key
        );
        idx.unwrap()
    }
    


}