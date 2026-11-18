use crate::Semantic::scope::data::scopekind::ScopeKind;

pub struct ScopeData {
    kind : ScopeKind,
    depth : u8,
}

impl ScopeData {
    pub fn new(kind : ScopeKind, depth : u8) -> Self{
        Self{
            kind,
            depth
        }
    }
}