use crate::lex::iterval::Interval;
use crate::lex::token::Token;

pub type Key = u32;

#[derive(Debug)]
pub struct Node{
    pub token    : Token,
    pub key      : Key,
    pub childs   : Interval
}

impl Node {
    pub fn new(token: Token, key : Key, childs : Interval) -> Self{
        Self{
            token,
            key,
            childs
        }
    }
}
