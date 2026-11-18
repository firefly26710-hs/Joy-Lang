use crate::lexer::tokenizer::source::token::Token;
use crate::semantic::scope::data::scope_kind::ScopeKind;

pub struct ScopeData {
    kind   : ScopeKind,
    depth  : u8
}

impl ScopeData {
    pub fn new(kind: ScopeKind, depth : u8) -> Self{

        Self{
            kind,
            depth
        }
    }

}