use crate::par::ast::node::Key;

const VAR_SCOPE_STACK_SIZE : usize = 2048;
const ASSIGNER_SIZE : usize = 512;


pub struct  Scope{
    pub var_scope_stack     : Vec<Key>,
    pub func_assigner       : Vec<Key>,
    pub current_start       : Key,
    pub loop_counter        : u32,
    pub prev_var_stack_len  : usize,
    pub curr_var_stack_len  : usize,
    pub is_in_func          : bool,
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
            is_in_func          : false
        }
    }

    pub fn build_func_scope(&mut self){
        self.prev_var_stack_len = self.var_scope_stack.len();
        self.is_in_func = true;
    }

    pub fn sign_var(&mut self, var_key : Key){
        self.var_scope_stack.push(var_key);
    }

    pub fn sign_func(&mut self, func_key : Key){
        self.func_assigner.push(func_key);
    }

    pub fn destroy_func_scope(&mut self){
        self.curr_var_stack_len = self.var_scope_stack.len();
        while self.curr_var_stack_len != self.prev_var_stack_len {
            self.var_scope_stack.pop();
            self.curr_var_stack_len -= 1;
        }
        self.is_in_func = false;
    }









}