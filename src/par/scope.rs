pub const SCOPE_MARK: u32 = 1919810114;
const VAR_SCOPE_STACK_SIZE : usize = 2048;
const FUNC_ASSIGNER_SIZE : usize = 512;
const RECORDER_SIZE : usize = 256;


pub struct Scope{
    pub var_scope_stack               : Vec<u32>,
    pub stmt_scope_stack              : Vec<u32>,
    pub func_assigner                 : Vec<u32>,
    pub loop_counter                  : u32,
    pub prev_var_stack_top            : usize,
    pub prev_stmt_stack_top           : usize,
    pub curr_var_stack_top            : usize,
    pub curr_stmt_stack_top            : usize
}


impl Scope{
    pub fn init() -> Self{
        Self{
            var_scope_stack               : Vec::with_capacity(VAR_SCOPE_STACK_SIZE),
            stmt_scope_stack              : Vec::with_capacity(RECORDER_SIZE),
            func_assigner                 : Vec::with_capacity(FUNC_ASSIGNER_SIZE),
            loop_counter                  : 0,
            prev_var_stack_top: 0,
            prev_stmt_stack_top: 0,
            curr_var_stack_top: 0,
            curr_stmt_stack_top: 0
        }
    }
    
    #[inline(always)]
    pub fn var_scope_assign(&mut self, key : u32) {
        self.var_scope_stack.push(key);
    }

    pub fn build_func_scope(&mut self, in_loop : bool) {
        self.prev_var_stack_top = self.var_scope_stack.len();
        if !in_loop{
            return
        }else{
            self.loop_counter += 1
        }
    }

    pub fn build_branch_scope(&mut self){
        todo!()
    }

    pub fn build_loop_scope(&mut self){
        todo!()
    }



    #[inline(always)]
    pub fn destroy_scope(&mut self, in_loop : bool){
        self.curr_var_stack_top = self.var_scope_stack.len() - 1;
        while self.var_scope_stack[self.curr_var_stack_top] != SCOPE_MARK {
            self.var_scope_stack.pop();
            self.curr_var_stack_top -= 1;
        }
        self.var_scope_stack.pop();
        
        if !in_loop{
            return
        }else{
            self.loop_counter -= 1;
        }
    }

    pub fn is_in_loop_scope(&self) -> bool{
        self.loop_counter >= 1
    }



}