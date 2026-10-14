use crate::lex::iterval::Interval;
use crate::lex::token::Token;
use crate::par::node::Node;
use crate::par::parser::{Parser, NONE};

impl Parser {
    pub fn parse_string(&mut self) -> u32{
        self.is_string = true;
        let key = self.current as u32;
        self.eat();
        self.nodes.push(
            Node::new(Token::String, key, Interval::new(NONE, NONE))
        );
        (self.nodes.len() - 1) as u32
    }

    #[inline(always)]
    pub fn is_string(&self, peeker : &Token ) -> bool{
        peeker == &Token::String
    }
}