use crate::lex::iterval::Interval;
use crate::par::ty::Type;

pub struct Table{
    var_keys  : Vec<u32>,
    func_keys : Vec<u32>,
    pub types               : Vec<Type>,
    pub param_type_idxs     : Vec<Interval>,
    pub ret_types           : Vec<Type>,
}

impl Table{
    pub fn init() -> Self{
        Self{
            var_keys        : Vec::new(),
            func_keys       : Vec::new(),
            types           : Vec::new(),
            param_type_idxs : Vec::new(),
            ret_types       : Vec::new()
        }
    }
    
    #[inline(always)]
    pub fn var_insert(&mut self, key : u32, ty : Type){
        self.var_keys.push(key);
        self.types.push(ty);
    }
}