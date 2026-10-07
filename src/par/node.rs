use crate::lex::iterval::Interval;
use crate::lex::token::Token;

pub type KeyId = u32;

#[derive(Debug)]
pub struct Node{
    pub tok  : Token,
    pub key  : KeyId,
    pub idxs : Interval
}

impl Node{
    pub fn new(tok : Token, key : KeyId, idxs : Interval) -> Self{
        Self{
            tok,
            key,
            idxs
        }
    }
}