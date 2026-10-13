pub const SCOPE_MARK: u32 = 1919810114;
const VAR_SCOPE_STACK_SIZE : usize = 2048;
const ASSIGNER_SIZE : usize = 512;



pub struct Scope{
    pub var_scope_stack     : Vec<u32>,
    pub func_assigner       : Vec<u32>,
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
    #[inline(always)]
    pub fn build_branch_scope(&mut self) {
        self.prev_var_stack_len = self.var_scope_stack.len();
        self.is_in_func = false;
    }

    pub fn build_loop_scope(&mut self){
        self.prev_var_stack_len = self.var_scope_stack.len();
        self.loop_counter += 1;
        self.is_in_func = false;
    }

    pub fn destroy_func_scope(&mut self){
        self.curr_var_stack_len = self.var_scope_stack.len();
        while self.curr_var_stack_len != self.prev_var_stack_len {
            self.var_scope_stack.pop();
            self.curr_var_stack_len -= 1;
        }
        self.is_in_func = false;
    }

    #[inline(always)]
    pub fn destroy_branch_scope(&mut self){
        self.curr_var_stack_len = self.var_scope_stack.len();
        while self.curr_var_stack_len != self.prev_var_stack_len {
            self.var_scope_stack.pop();
            self.curr_var_stack_len -= 1;
        }
        self.is_in_func = true;
    }


    pub fn destroy_loop_scope(&mut self){
        self.curr_var_stack_len = self.var_scope_stack.len();
        while self.curr_var_stack_len != self.prev_var_stack_len {
            self.var_scope_stack.pop();
            self.curr_var_stack_len -= 1;
        }
        self.loop_counter -= 1;
        self.is_in_func = true;
    }

    pub fn is_in_loop_scope(&self) -> bool {
        self.loop_counter >= 1
    }

    

}