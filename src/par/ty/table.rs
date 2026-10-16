use comp::lex::iterval::Interval;
use crate::par::tys::Type;

type KeyId = u32;
pub struct Table{
    pub var_keys      : Vec<KeyId>,
    pub var_types     : Vec<Type>,
    pub func_keys     : Vec<KeyId>,
    param_idxs        : Vec<Interval>,
    pub ret_types     : Vec<Type>,
}

