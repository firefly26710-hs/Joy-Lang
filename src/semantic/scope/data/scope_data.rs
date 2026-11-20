use crate::semantic::scope::data::scope_kind::ScopeKind;
#[derive(Debug, Clone, PartialEq)]

pub struct ScopeData {
    pub kind   : ScopeKind,
    pub depth  : u8
}

impl ScopeData {
    pub fn new(kind: ScopeKind, depth : u8) -> Self{

        Self{
            kind,
            depth
        }
    }

}