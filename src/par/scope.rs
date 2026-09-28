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
    pub fn var_scope_assign(&mut self, key : u32){
        self.var_scope_stack.push(key);
    }
}