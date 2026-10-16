use crate::lex::iterval::Interval;
use crate::lex::token::Token;
use crate::par::node::KeyId;
#[derive(Debug)]
pub struct Node{
    pub tok    : Token,
    pub key    : KeyId,
    pub childs : Interval
}

impl Node {
    pub fn new(tok : Token, key : KeyId, childs : Interval) -> Self{
        Self{
            tok,
            key,
            childs
        }
    }
}
