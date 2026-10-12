use std::thread::current;
use crate::lex::iterval::Interval;
use crate::par::parser::NONE;
use crate::par::ty::Type;

const VAR_PART_ALLOC_SIZE: usize = 256;
const FUNC_PART_ALLOC_SIZE : usize = 128;
pub struct Table{
    pub var_keys          : Vec<u32>,
    var_types     : Vec<Type>,
    pub func_keys         : Vec<u32>,
    param_idxs    : Vec<Interval>,
    pub ret_types     : Vec<Type>,
}

impl Table{
    pub fn init() -> Self{
        Self{
            var_keys    : Vec::with_capacity(VAR_PART_ALLOC_SIZE),
            var_types   : Vec::with_capacity(VAR_PART_ALLOC_SIZE),
            func_keys   : Vec::with_capacity(FUNC_PART_ALLOC_SIZE),
            param_idxs  : Vec::with_capacity(FUNC_PART_ALLOC_SIZE),
            ret_types   : Vec::with_capacity(FUNC_PART_ALLOC_SIZE),
        }
    }
    
    #[inline(always)]
    pub fn var_insert(&mut self, key : u32, ty : Type){
        self.var_keys.push(key);
        self.var_types.push(ty);
    }

    #[inline(always)]
    pub fn get_var_type(&self, key : u32) -> &Type{
        let mut current = 0;
        while current < self.var_keys.len(){
            if self.var_keys[current] != key{
                current += 1;
                continue;
            }else{
                break;
            }
        }

        &self.var_types[current]

    }

    pub fn get_ret_type(&self, key : u32 ) -> &Type{
        let mut current = 0;
        while current < self.func_keys.len(){
            if self.func_keys[current] != key{
                current += 1;
                continue
            }else{
                break;
            }
        }

        &self.ret_types[current]
    }

    pub fn get_param_types(&self, key : u32) -> &[Type]{
        let mut current = 0;
        while current < self.func_keys.len(){
            if self.func_keys[current] != key{
                current += 1;
                continue
            }else{
                break;
            }
        }

        let idxs = &self.param_idxs[current];
        let start = idxs.start;
        let end = idxs.end;
        if start != NONE && end != NONE{
            &self.var_types[start as usize..=end as usize]
        }else{
           &[]
        }

    }


    pub fn func_insert(
        &mut self,
        key : u32,
        param_start : u32,
        param_end : u32,
        ret_ty : Type
    ){
        self.func_keys.push(key);
        self.param_idxs.push(Interval::new(param_start, param_end));
        self.ret_types.push(ret_ty);
    }

    pub fn print_table(&mut self){
        for (i, ty) in self.var_types.iter().enumerate(){
            println!("Var Type {i} : {:?} ", ty);
        }


        for (i, idx) in self.param_idxs.iter().enumerate(){
            println!("param idx {i} : {:?} ", idx);
        }

        for (i, ty) in self.ret_types.iter().enumerate(){
            println!("Ret Type {i} : {:?} ", ty);
        }

    }




    
    
}