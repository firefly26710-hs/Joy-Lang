pub const SCOPE_MARK: u32 = 1919810114;

pub struct Scope{
    pub var_scope_stack     : Vec<u32>,
    pub func_assigner       : Vec<u32>,
    pub loop_counter        : u8,
}


impl Scope{
    pub fn init() -> Self{
        Self{
            var_scope_stack : Vec::new(),
            func_assigner   : Vec::new(),
            loop_counter    : 0
        }
    }
    
    #[inline(always)]
    pub fn var_scope_assign(&mut self, key : u32) {
        self.var_scope_stack.push(key);
    }

    #[inline(always)]
    pub fn build_scope(&mut self){
        self.var_scope_stack.push(SCOPE_MARK)
    }

    #[inline(always)]
    pub fn destroy_scope(&mut self){
        let mut i = self.var_scope_stack.len() - 1;
        while self.var_scope_stack[i] != SCOPE_MARK {
            self.var_scope_stack.pop();
            i -= 1;
        }
        self.var_scope_stack.pop();
    }

    pub fn is_in_loop_stmt(&self) -> bool{
        self.loop_counter >= 1
    }

}