use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::llvm::codegen::Codegen;
use crate::parser::tree::ast::NONE;
use crate::parser::types::typ::Type;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue};

impl<'ctx>Codegen<'ctx>{
    pub fn build_function(&mut self, func_idx : u32){
        let func_node = &self.parser.ast.nodes[func_idx as usize];
        let idxs = &func_node.edge_span;
        let edge_start = idxs.start;
        let edge_end = idxs.end;

        //function name
        let name_idx = self.get_edge_slice(edge_start, edge_end)[0];
        let name_node = self.get_node(name_idx);
        let name_key = name_node.key;
        let (str_start, str_end) = self.get_str_interval(name_key);
        let name_str = &self.parser.lexer.tokenizer.input[str_start as usize..str_end as usize];

        //function param
        let param_type_interval = self.parser.look_param_interval(name_key as u32);
        let param_start = param_type_interval.start;
        let param_end = param_type_interval.end;

        let has_params = param_start != NONE && param_end != NONE;
        if has_params {
            let len =
                self.parser.table.var_types[param_start as usize..=param_end as usize].len();

            for i in 0..len{
                let ty = &self.parser.table.var_types[param_start as usize..=param_end as usize][i];
                let type_context : BasicMetadataTypeEnum = match ty{
                    Type::i8 => self.context.i8_type().into(),
                    Type::i16 => self.context.i16_type().into(),
                    Type::i32 => self.context.i32_type().into(),
                    Type::i64 => self.context.i64_type().into(),
                    Type::i128 => self.context.i128_type().into(),
                    Type::bool => self.context.bool_type().into(),
                    Type::f64 => self.context.f64_type().into(),
                    Type::f32 => self.context.f32_type().into(),
                    Type::String => self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),

                    _ => unreachable!()
                };
                self.param_types.push(type_context);
            }
        }



        //ret type
        let ret_type = self.parser.look_ret_type(name_key as u32);

        let fn_type = match ret_type {
            Type::i8 => self.context.i8_type().fn_type(&self.param_types, false),
            Type::i16 => self.context.i16_type().fn_type(&self.param_types, false),
            Type::i32 => self.context.i32_type().fn_type(&self.param_types, false),
            Type::i64 => self.context.i64_type().fn_type(&self.param_types, false),
            Type::i128 => self.context.i128_type().fn_type(&self.param_types, false),
            Type::f32 => self.context.f32_type().fn_type(&self.param_types, false),
            Type::f64 => self.context.f64_type().fn_type(&self.param_types, false),
            Type::bool => self.context.bool_type().fn_type(&self.param_types, false),
            Type::void => self.context.void_type().fn_type(&self.param_types, false),
            Type::String => {
                let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
                i8_ptr_type.fn_type(&self.param_types, false)
            },

        };

        let func = self.module.add_function(name_str, fn_type, None);

        //creat function
        self.param_types.clear();
        if has_params{
            let len = self.get_edge_slice(edge_start + 1,  edge_end- 1).len();

            for i in 0..len{
                let param_idx = self.get_edge_slice(edge_start + 1, edge_end- 1)[i];
                let param_key = self.get_node(param_idx).key;
                let (start, end) = self.get_str_interval(param_key);
                let param_name_str = &self.parser.lexer.tokenizer.input[start as usize..end as usize];
                func.get_nth_param(i as u32).unwrap().set_name(param_name_str);
            }


        }
        self.functions.push(func);
    }

    pub fn compile_function(&mut self, func_idx : u32){
        let func_node = &self.parser.ast.nodes[func_idx as usize];
        let idxs = &func_node.edge_span;
        let func_child_start = idxs.start;
        let func_child_end = idxs.end ;

        let func_node_edges = self.get_edge_slice(func_child_start, func_child_end);

        let name_node = &self.parser.ast.nodes[func_node_edges[0] as usize];
        let name_key = name_node.key;
        let func = self.get_func_ptr(name_key);

        //create entry
        let entry = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);

        //build param
        let param_types_intervals = self.parser.look_param_interval(name_key);
        let type_start = param_types_intervals.start;
        let type_end = param_types_intervals.end;

        let has_param = self.get_edge_slice(func_child_start + 1, func_child_end - 1).len() > 0;
        
        if has_param{
            let len = self.get_edge_slice(func_child_start + 1, func_child_end - 1).len();

            for i in 0..len{
                let idx = self.get_edge_slice(func_child_start + 1, func_child_end - 1)[i];
                let param_key = self.get_node(idx).key;

                let (start, end) = self.get_str_interval(param_key);
                let name_str = &self.parser.lexer.tokenizer.input[start as usize..end as usize];

                let param_value = func.get_nth_param(i as u32);
                let param_type = &self.parser.table.var_types[type_start as usize..=type_end as usize][i];

                match param_value.unwrap(){
                    BasicValueEnum::IntValue(int_val) =>{
                        let int_value_type  = match param_type{
                            Type::i8 => self.context.i8_type(),
                            Type::i16 => self.context.i16_type(),
                            Type::i32 => self.context.i32_type(),
                            Type::i64 => self.context.i64_type(),
                            Type::i128 => self.context.i128_type(),
                            Type::bool => self.context.bool_type(),
                            _ => panic!("Unknown Int Type : {:?}", param_type),
                        };

                        let alloc = self.builder.build_alloca(int_value_type, name_str).unwrap();
                        self.builder.build_store(alloc, int_val).unwrap();
                        self.variables.push(alloc);

                    },
                    BasicValueEnum::FloatValue(flt_val) => {
                        let flt_val_type =match param_type{
                            Type::f32 => self.context.f32_type(),
                            Type::f64 => self.context.f64_type(),
                            _ => panic!("Unknown Float Type : {:?}", param_type)
                        };

                        let alloc = self.builder.build_alloca(flt_val_type, name_str).unwrap();
                        self.builder.build_store(alloc, flt_val).unwrap();
                        self.variables.push(alloc);
                    },
                    BasicValueEnum::PointerValue(str_val) => {
                        let pointer_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());

                        let alloc = self.builder.build_alloca(pointer_type, name_str).unwrap();
                        self.builder.build_store(alloc, str_val).unwrap();
                        self.variables.push(alloc);
                    },
                    _ => panic!("Unknown Type : {:?}", param_type),
                };

            }

        }

        self.current_fn = Some(func);
        self.ret_type = self.parser.look_ret_type(name_key).clone();
        let block_idx = self.get_edge_slice(func_child_end, func_child_end)[0];
        self.build_stmts(block_idx);

        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_return(None).unwrap();
        }
    }

    pub fn build_ret_stmt(&mut self ,start : Start, end : End){
        let is_return_void = self.ret_type == Type::void;

        if!is_return_void{
            let childs = &self.get_edge_slice(start, end);
            let expr_idx = childs[1] as usize;
            let expr = self.build_expr(expr_idx, &self.ret_type.clone());
            self.builder.build_return(Some(&expr)).unwrap();
        }else{
            self.builder.build_return(None).unwrap();
        }

    }

    pub fn build_call_stmt(&mut self ,start : Start, end : End){
        let name_idx = self.get_edge_slice(start, end)[0];
        let name_key = self.get_node(name_idx).key;
        let has_param = self.get_edge_slice(start, end).len() > 1;
        let len = self.get_edge_slice(start, end).len() - 1;

        if has_param{
            let param_type_intervals = self.parser.look_param_interval(name_key);
            let param_start = param_type_intervals.start as usize;
            let param_end = param_type_intervals.end as usize;

            for i in 0..len{
                let ty = &self.parser.table.var_types[param_start..=param_end][i].clone();
                let expr_idx = self.parser.ast.edges[(start as usize) + 1 + i] as usize;
                let val = if ty != &Type::bool {
                    self.build_expr(expr_idx, &ty)
                } else {
                    self.build_expr(expr_idx, &Type::i64)
                };

                self.call_param_types.push(val.into());
            }
        }

        let func = self.get_func_ptr(name_key);
        let str_interval = &self.parser.lexer.tokenizer.intervals[name_key as usize];
        let str_s = str_interval.start as usize;
        let str_e = str_interval.end as usize;
        let name_str = &self.parser.lexer.tokenizer.input[str_s..str_e];
        self.builder.build_call(func, &self.call_param_types, format!("function {name_str}").as_str()).unwrap();
        self.call_param_types.clear();


    }

    pub fn get_func_ptr(&self, key : u32) -> FunctionValue<'ctx> {
        for i in 0..self.parser.table.func_keys.len() {
            if self.parser.table.func_keys[i] == key {
                return self.functions[i];
            }
        }
        unreachable!()
    }

}
