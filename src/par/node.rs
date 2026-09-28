use crate::lex::iterval::Interval;
use crate::lex::token::Token;

#[derive(Debug)]
pub struct Node{
    pub tok  : Token,
    pub key  : u32,
    pub idxs : Interval
}

impl Node{
    pub fn new(tok : Token, key : u32, idxs : Interval) -> Self{
        Self{
            tok,
            key,
            idxs
        }
    }
}