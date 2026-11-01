use crate::parser::center::par::Parser;
use crate::parser::tree::node::Key;

impl Parser {
    pub fn find_same_ident_key(&mut self, input_key : Key) -> Key{
        for key in self.scope.var_scope_stack.iter().rev(){
            if self.is_matches_string(input_key, *key){
                return *key;
            }
        }
        
        input_key
    }

    pub fn find_same_func_key(&self, input_key : Key) -> Key{
        for key in self.scope.func_assigner.iter(){
            if self.is_matches_string(input_key, *key){
                return *key;
            }
        }

        input_key
    }


    pub fn sign_var(&mut self, var_key : Key) {
        self.scope.var_scope_stack.push(var_key);
    }

    pub fn sign_func(&mut self, func_key : Key){
        self.scope.func_assigner.push(func_key);
    }


    pub fn build_scope(&mut self){
        self.scope.prev_var_stack_len  = self.scope.var_scope_stack.len();
    }
    
    pub fn destroy_scope(&mut self){
        self.scope.curr_var_stack_len = self.scope.var_scope_stack.len();
        self.scope.var_scope_stack.drain(
            self.scope.prev_var_stack_len..self.scope.curr_var_stack_len
        );
    }

    pub fn increase_loop_counter(&mut self){
        self.scope.loop_counter += 1;
    }
    
    pub fn decrease_loop_counter(&mut self){
        self.scope.loop_counter -= 1;
    }
    
    pub fn in_loop(&self) -> bool{
        self.scope.loop_counter >= 1
    }



}