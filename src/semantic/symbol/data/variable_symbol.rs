use crate::semantic::symbol::data::ty::Type;
use crate::parser::tree::node::Key;
use crate::semantic::scope::data::scope_data::ScopeData;

const VAR_PART_ALLOC_SIZE: usize = 128;
pub struct VariableSymbol {
    pub var_keys      : Vec<Key>,
    pub var_types     : Vec<Type>,
    pub var_scope     : Vec<ScopeData>,

}

impl VariableSymbol {
    pub fn init() -> Self{
        Self{
            var_keys    : Vec::with_capacity(VAR_PART_ALLOC_SIZE),
            var_types   : Vec::with_capacity(VAR_PART_ALLOC_SIZE),
            var_scope   : Vec::with_capacity(VAR_PART_ALLOC_SIZE),
        }
    }
}