use crate::lexer::interval::Interval;
use crate::lexer::token::Token;

pub type Key = u32;

#[derive(Debug)]
pub struct Node{
    pub token    : Token,
    pub key      : Key,
    pub edge_span: Interval
}

impl Node {
    pub fn new(token: Token, key : Key, childs : Interval) -> Self{
        Self{
            token,
            key,
            edge_span: childs
        }
    }
}
