use crate::lexer::tokenizer::source::interval::{End, Interval, Start};
use crate::parser::center::par::Parser;
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;

impl Parser{

    pub fn look_var_type(&self, input_key : &Key) -> &Type{

        for i in 0..self.table.var_keys.len(){
            if self.table.var_keys[i] == *input_key{
                return &self.table.var_types[i];
            }
        }

        panic!("[Line {:?}] Can't found variable type", self.current)
    }

    pub fn insert_var_type(&mut self, var_key: Key, ty : Type){
        self.table.var_keys.push(var_key);
        self.table.var_types.push(ty);
    }

    pub fn insert_func_type(
        &mut self,
        func_key: Key,
        start : Start,
        end   : End,
        ret_type : Type
    ){
        self.table.func_keys.push(func_key);
        self.table.param_idxs.push(Interval::new(start, end));
        self.table.ret_types.push(ret_type);
    }

    pub fn params_start(&self) -> Start{
        self.table.var_types.len() as Start
    }

    pub fn params_end(&self) -> End{
        (self.table.var_types.len() - 1) as End
    }

    pub fn look_param_interval(&self, key : u32 ) -> &Interval {

        for i in 0.. self.table.func_keys.len(){
            if self.table.func_keys[i] == key{
                return &self.table.param_idxs[i];
            }
        }

        unreachable!()

    }



    pub fn look_ret_type(&self, key : u32 ) -> &Type{
        for i in 0.. self.table.func_keys.len(){
            if self.table.func_keys[i] == key{
                return &self.table.ret_types[i];
            }
        }

        unreachable!()

    }

    pub fn last_ret_type(&self) -> &Type{
        self.table.ret_types.last().unwrap()
    }
    
    
    
    
}


