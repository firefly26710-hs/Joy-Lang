use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue};
use crate::par::ty::Type;
use crate::llvm::codegen::Codegen;

impl<'ctx>Codegen<'ctx>{
    pub fn build_function(&mut self, func_idx : u32){
        let func_node = &self.parser.nodes[func_idx as usize];
        let idxs = &func_node.idxs;
        let func_child_start = idxs.start as usize;
        let func_child_end = idxs.end as usize;
        let func_node_childs = &self.parser.edges[func_child_start..=func_child_end];


        //function name
        let name_idx = func_node_childs[0] as usize;
        let name_node = &self.parser.nodes[name_idx];
        let name_key = name_node.key as usize;
        let name_str_interval = &self.parser.lexer.intervals[name_key];
        let str_start = name_str_interval.start as usize;
        let str_end = name_str_interval.end as usize;
        let name_str = &self.parser.lexer.input[str_start..str_end];

        //function param
        let param_types = self.parser.table.get_param_types(name_key as u32);
        let len = param_types.len();
        let ptr = param_types.as_ptr();

        for i in 0..len{
            let ty  = unsafe{&*ptr.add(i)};
            let type_context : BasicMetadataTypeEnum = match ty{
                Type::i32 => self.context.i32_type().into(),
                Type::i64 => self.context.i64_type().into(),
                Type::bool => self.context.bool_type().into(),
                Type::f64 => self.context.f64_type().into(),
                Type::f32 => self.context.f32_type().into(),
                _ => unreachable!()
            };
            self.param_types_vec.push(type_context);
        }

        //ret type
        let ret_type = self.parser.table.get_ret_type(name_key as u32);

        //creat function
        let func  = match ret_type{
            Type::i32 =>{
                let ret_type = self.context.i32_type();
                let fn_type = ret_type.fn_type(&self.param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            },
            Type::i64 =>{
                let ret_type = self.context.i64_type();
                let fn_type = ret_type.fn_type(&self.param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            },
            Type::f32 =>{
                let ret_type = self.context.f32_type();
                let fn_type = ret_type.fn_type(&self.param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            },
            Type::f64 =>{
                let ret_type = self.context.f64_type();
                let fn_type = ret_type.fn_type(&self.param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            }
            Type::bool =>{
                let ret_type = self.context.bool_type();
                let fn_type = ret_type.fn_type(&self.param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            },
            Type::void =>{
                let ret_type = self.context.void_type();
                let fn_type = ret_type.fn_type(&self.param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            },
            _ => unreachable!()
        };

        self.param_types_vec.clear();

        let has_param = param_types.len() > 0;

        if has_param{
            let param_idxs = &self.parser.edges[func_child_start + 1..=func_child_end - 1];
            let len = param_idxs.len();
            let param_idx_ptr = param_idxs.as_ptr();
            for i in 0..len{
                let param_idx = unsafe{*param_idx_ptr.add(i)} as usize;
                let param_node = &self.parser.nodes[param_idx];
                let param_key = param_node.key;
                let str_interval = &self.parser.lexer.intervals[param_key as usize];
                let start = str_interval.start as usize;
                let end = str_interval.end as usize;
                let param_name_str = &self.parser.lexer.input[start..end];
                func.get_nth_param(i as u32).unwrap().set_name(param_name_str);
            }
        }

        self.functions.push(func);
    }

    pub fn compile_function(&mut self, func_idx : u32){
        let func_node = &self.parser.nodes[func_idx as usize];
        let idxs = &func_node.idxs;
        let func_child_start = idxs.start as usize;
        let func_child_end = idxs.end as usize;
        let func_node_childs = &self.parser.edges[func_child_start..=func_child_end];

        let name_node = &self.parser.nodes[func_node_childs[0] as usize];
        let name_key = name_node.key;
        let func = self.get_function(name_key);

        //create entry
        let entry = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);

	    //build param
	    let param_types = self.parser.table.get_param_types(name_key);

	    let has_param = param_types.len() > 0;

	    if has_param{
	        let param_nodes = &self.parser.edges[func_child_start + 1..=func_child_end - 1];

            for (i, _) in param_types.iter().enumerate(){
                let param_name_node = &self.parser.nodes[param_nodes[i] as usize];
                let param_name_key = param_name_node.key;

                let str_interval = &self.parser.lexer.intervals[param_name_key as usize];
                let start = str_interval.start as usize;
                let end = str_interval.end as usize;
                let name_str = &self.parser.lexer.input[start..end];

                let param_value = func.get_nth_param(i as u32).unwrap();
                let param_type = &param_types[i];

                match param_value{
                    BasicValueEnum::IntValue(val) => {
                        let int_type = if param_type == &Type::i32{
                            self.context.i32_type()
                        }else if param_type == &Type::bool{
                            self.context.bool_type()
                        }else if param_type == &Type::i64{
                          self.context.i64_type()
                         }else{
                            unreachable!()
                        };

                        let alloc = self.builder.build_alloca(int_type, name_str).unwrap();
                        self.builder.build_store(alloc, val).unwrap();
                        self.variables.push(alloc);

                    },
                    BasicValueEnum::FloatValue(val) => {
                         let flt_type = if param_type == &Type::f32{
                            self.context.f32_type()
                        }else if param_type == &Type::f64{
                            self.context.f64_type()
                        }else{
                            unreachable!()
                        };

                        let alloc = self.builder.build_alloca(flt_type, name_str).unwrap();
                        self.builder.build_store(alloc, val).unwrap();
                        self.variables.push(alloc);
                    }
                    _ => unreachable!()
                };
            }
    	}

        self.current_fn = Some(func);
	    self.ret_type = self.parser.table.get_ret_type(name_key).clone();
        let block_idx = *func_node_childs.last().unwrap();
        //self.compile_block(block_idx);
    }


    fn get_function(&self, key : u32) -> FunctionValue<'ctx>{
        let mut current = 0;
        while current < self.parser.table.var_keys.len(){
            if self.parser.table.var_keys[current] != key{
                current += 1;
                continue;
            }else{
                break;
            }
        }
        self.functions[current]
    }



}
