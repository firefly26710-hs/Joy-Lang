use crate::lexer::tokenizer::source::interval::Interval;
use crate::parser::types::logic::Type;
const VAR_PART_ALLOC_SIZE: usize = 256;
const FUNC_PART_ALLOC_SIZE : usize = 128;

type KeyId = u32;
pub struct Table{
    pub var_keys      : Vec<KeyId>,
    pub var_types     : Vec<Type>,
    pub func_keys     : Vec<KeyId>,
    pub param_idxs    : Vec<Interval>,
    pub ret_types     : Vec<Type>,
}

impl Table{
    pub fn init() -> Self{
        Self{
            var_keys    : Vec::with_capacity(VAR_PART_ALLOC_SIZE),
            var_types   : Vec::with_capacity(VAR_PART_ALLOC_SIZE),
            func_keys   : Vec::with_capacity(FUNC_PART_ALLOC_SIZE),
            param_idxs  : Vec::with_capacity(FUNC_PART_ALLOC_SIZE),
            ret_types   : Vec::with_capacity(FUNC_PART_ALLOC_SIZE),
        }
    }

    

    
}

