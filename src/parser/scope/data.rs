use crate::parser::tree::node::Key;

const VAR_SCOPE_STACK_SIZE : usize = 2048;
const ASSIGNER_SIZE : usize = 512;


pub struct Scope{
    pub var_scope_stack     : Vec<Key>,
    pub func_assigner       : Vec<Key>,
    pub current_start       : Key,
    pub loop_counter        : u32,
    pub prev_var_stack_len  : usize,
    pub curr_var_stack_len  : usize,
    pub start               : usize,
}

impl Scope{
    pub fn init() -> Self{
        Self{
            var_scope_stack     : Vec::with_capacity(VAR_SCOPE_STACK_SIZE),
            func_assigner       : Vec::with_capacity(ASSIGNER_SIZE),
            current_start       : 0,
            loop_counter        : 0,

            prev_var_stack_len  : 0,
            curr_var_stack_len  : 0,
            start               : 0
        }
    }


    


}
