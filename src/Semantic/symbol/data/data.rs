use crate::lexer::tokenizer::source::interval::Interval;
use crate::parser::tree::node::Key;
use crate::Semantic::symbol::ty::Type;

pub struct Symbol{
    pub var_keys      : Vec<Key>,
    pub var_types     : Vec<Type>,
    pub func_keys     : Vec<Key>,
    pub param_idxs    : Vec<Interval>,
    pub ret_types     : Vec<Type>,
}

impl Symbol{
    
}