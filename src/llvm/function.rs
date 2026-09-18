use inkwell::values::{BasicValueEnum, FunctionValue};
use crate::par::ty::Type;
use crate::llvm::codegen::{Codegen, NONE};
use crate::lex::iterval::Interval;

impl<'ctx>Codegen<'ctx>{
    pub fn compile_function(&mut self, func_idx : u32){
        let func_node = &self.nodes[func_idx as usize];
        let l_edge_idx = func_node.l_edge_idx as usize;
        let r_edge_idx = func_node.r_edge_idx as usize;

        let nodes = &self.edges[l_edge_idx..=r_edge_idx];
        let name_node = &self.nodes[nodes[0] as usize];
        let name_key = name_node.key;

        let func = self.get_function(name_key);

        let entry = self.context.append_basic_block(func, "function");
        self.builder.position_at_end(entry);

        let param_type_interval = self.get_param_interval(name_key);

        let start = param_type_interval.start as usize;
        let end = param_type_interval.end as usize;

        let has_param = start != NONE && end != NONE && l_edge_idx + 1 != r_edge_idx;

        let param_types = if has_param{&self.types[start..=end]}else{&Vec::new()};

        let param_idxs   = if has_param{&self.edges[l_edge_idx + 1..=r_edge_idx - 1]}else{&Vec::new()};
        
        for (i, _) in param_types.iter().enumerate(){
            let var_name_node = &self.nodes[param_idxs[i] as usize];
            let var_name_key = var_name_node.key;
            let var_interval = &self.intervals[var_name_key as usize];
            let start = var_interval.start as usize;
            let end = var_interval.end as usize;
            let name_str = &self.input[start..end];
            
            let param_value = func.get_nth_param(i as u32).unwrap();
            let ty = &param_types[i];

            
            match param_value{
                BasicValueEnum::IntValue(val) => {
                    let ty_value = if ty == &Type::i32{
                        self.context.i32_type()
                    }else if ty == &Type::i64{
                        self.context.i64_type()
                    }else if ty == &Type::bool{
                        self.context.bool_type()
                    }else{
                        unreachable!()
                    };

                    let alloca = self.builder.build_alloca(ty_value, name_str).unwrap();
                    self.builder.build_store(alloca, val).unwrap();

                    self.variables.push(alloca);
                },
                BasicValueEnum::FloatValue(val) => {
                    let ty_value = if ty == &Type::f32{
                        self.context.f32_type()
                    }else if ty == &Type::f64{
                        self.context.f64_type()
                    }else{
                        unreachable!()
                    };

                    let alloca = self.builder.build_alloca(ty_value, name_str).unwrap();
                    self.builder.build_store(alloca, val).unwrap();

                    self.variables.push(alloca);
                }
                _ => unreachable!()
            }
        }

        let stmts_idx = self.edges[r_edge_idx];
        dbg!(stmts_idx);
        self.compile_stmts(stmts_idx);
    }


    
    pub fn get_function(&mut self, key : u32) -> FunctionValue<'ctx>{
        let mut current = 0;
        while key != self.func_assigner[current]{
            current += 1;
        }

        self.functions[current]
    }

    pub fn get_param_interval(&mut self, key : u32) -> &Interval{
        let mut current = 0;
        while key != self.func_assigner[current]{
            current += 1;
        }

        &self.param_type_idxs[current]
    }

}
