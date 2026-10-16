use crate::lex::iterval::{Start, End, Interval};
use crate::par::ast::node::Key;
use crate::par::ty::tys::Type;
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

    pub fn look_var_type(&self, input_key : Key) -> &Type{
        for i in 0..self.var_keys.len(){
            if self.var_keys[i] != input_key{
                return &self.var_types[i];
            }
        }

        unreachable!()
    }

    pub fn insert_var_type(&mut self, var_key: Key, ty : Type){
        self.var_keys.push(var_key);
        self.var_types.push(ty);
    }

    pub fn insert_func_type(
        &mut self,
        func_key: Key,
        start : Start,
        end   : End,
        ret_type : Type
    ){
        self.func_keys.push(func_key);
        self.param_idxs.push(Interval::new(start, end));
        self.ret_types.push(ret_type);
    }

    pub fn params_start(&self) -> Start{
        self.param_idxs.len() as Start
    }

    pub fn params_end(&self) -> End{
        (self.param_idxs.len() - 1) as End
    }


}

