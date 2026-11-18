use crate::parser::tree::node::Key;
use crate::semantic::scope::data::scopekind::ScopeKind;

pub struct Scope {
    kinds : Vec<ScopeKind>,
    kw_key: Vec<Key>,
    depths : Vec<u8>,
}

impl Scope {
    pub fn init(alloc_size : usize) -> Self{
        Self{
            kw_key: Vec::with_capacity(alloc_size),
            kinds : Vec::with_capacity(alloc_size),
            depths: Vec::with_capacity(alloc_size),
        }
    }
}