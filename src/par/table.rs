use crate::lex::iterval::Interval;
use crate::par::ty::Type;

const VAR_PART_ALLOC_SIZE: usize = 256;
const FUNC_PART_ALLOC_SIZE : usize = 128;
pub struct Table{
    var_keys          : Vec<u32>,
    func_keys         : Vec<u32>,
    pub param_idxs    : Vec<Interval>,
    pub var_types     : Vec<Type>,
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
    
    #[inline(always)]
    pub fn var_insert(&mut self, key : u32, ty : Type){
        self.var_keys.push(key);
        self.var_types.push(ty);
    }
}