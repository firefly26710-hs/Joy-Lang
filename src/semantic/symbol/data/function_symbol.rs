use crate::parser::tree::node::Key;
use crate::semantic::symbol::data::ty::Type;

pub struct FunctionSymbol{
    pub func_keys : Vec<Key>,
    pub ret_types : Vec<Type>,
}

impl FunctionSymbol{
    pub fn init() -> Self{
        Self{
            func_keys       : Vec::with_capacity(64),
            ret_types       : Vec::with_capacity(64)
        }
    }
}