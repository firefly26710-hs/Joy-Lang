use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, PointerValue, ValueKind};
use crate::par::parser::NONE;
use crate::llvm::codegen::Codegen;
use crate::lex::token::Token;
use crate::par::ty::Type;


impl<'ctx> Codegen<'ctx>{
    pub fn compile_stmts(&mut self, stmt_idx : usize){
        let block_node = &self.nodes[stmt_idx];
        let l_edge_idx = block_node.l_edge_idx as usize;
        let r_edge_idx = block_node.r_edge_idx as usize;

        let stmt_idxs = &self.edges[l_edge_idx..=r_edge_idx].to_vec();

        for idx in stmt_idxs{
            let node = &self.nodes[*idx as usize];
            let tok = &node.token;
            let l_edge_idx = node.l_edge_idx as usize;
            let r_edge_idx = node.r_edge_idx as usize;

            match tok{
                Token::LetStmt => self.let_stmt(l_edge_idx, r_edge_idx),
                Token::ForStmt => self.for_stmt(l_edge_idx, r_edge_idx),
                Token::IfStmt => self.if_stmt(l_edge_idx, r_edge_idx),
                Token::WhileStmt => self.while_stmt(l_edge_idx, r_edge_idx),
                Token::BrkStmt => self.brk_stmt(),
                Token::CtnStmt => self.ctn_stmt(),
                Token::RetStmt => self.ret_stmt(l_edge_idx, r_edge_idx),
                Token::CallStmt => self.call_stmt(l_edge_idx, r_edge_idx),
                _ => unreachable!()
            }
        }
    }

    fn let_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){
        let idxs = &self.edges[l_edge_idx..=r_edge_idx];
        let name_node = &self.nodes[idxs[0] as usize];
        let expr_idx = idxs[1] as usize;
        let name_key = name_node.key;
        let ty = self.type_get(&name_key);
        let variable = self.expr(expr_idx, &ty);
        let var_type : BasicTypeEnum<'ctx> = match variable{
            BasicValueEnum::IntValue(v) => v.get_type().into(),
            BasicValueEnum::FloatValue(v) => v.get_type().into(),
            _ => unreachable!()
        };

        let interval = &self.intervals[name_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let name_str = &self.input[start..end];


        let entry = self.current_fn.unwrap().get_first_basic_block().unwrap();
        self.builder.position_at_end(entry);
        let alloca = self.builder.build_alloca(var_type, name_str).unwrap();
        self.builder.build_store(alloca, variable).unwrap();
        self.variables.push(alloca);
    }

    fn for_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){
        
        let idxs = &self.edges[l_edge_idx..=r_edge_idx].to_vec();

        //name node
        let name_node = &self.nodes[idxs[0] as usize];
        let interval = &self.intervals[name_node.key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let s_number : BasicValueEnum = self.expr(idxs[1] as usize, &Type::i64);
        let e_number : BasicValueEnum = self.expr(idxs[2] as usize, &Type::i64);
        let step_number : BasicValueEnum = self.expr(idxs[3] as usize, &Type::i64);




        let name_str = &self.input[start..end];
        let i64_type = self.context.i64_type();
        let ident_alloca = self.builder.build_alloca(i64_type, name_str).unwrap();

        self.builder.build_store(ident_alloca, s_number).unwrap();

        let func = self.current_fn.unwrap();
        let cond_bb = self.context.append_basic_block(func, "for.cond");
        let body_bb = self.context.append_basic_block(func, "for.body");
        let step_bb = self.context.append_basic_block(func, "for.step");
        let end_bb = self.context.append_basic_block(func, "for.end");
        self.ctn_goto_block.push(step_bb);
        self.brk_goto_block.push(end_bb);


        self.builder.build_unconditional_branch(cond_bb).unwrap();

        //for.cond
        self.builder.position_at_end(cond_bb);
        let cur_ident_number = self.builder.build_load(
            i64_type,
            ident_alloca,
            name_str
        ).unwrap().into_int_value();

        let end_number = e_number.into_int_value();

        let cond_bool = self.builder.build_int_compare(
            inkwell::IntPredicate::SLT,
            cur_ident_number,
            end_number,
            "is_loop_end"
        ).unwrap();

        self.builder.build_conditional_branch(cond_bool, body_bb, end_bb).unwrap();


        //for.body
        self.builder.position_at_end(body_bb);
        self.compile_stmts(idxs[4] as usize);
        self.builder.build_unconditional_branch(step_bb).unwrap();


        //for.step
        self.builder.position_at_end(step_bb);
        let step_number = step_number.into_int_value();
        let next_ident_number = self.builder.build_int_add(
            cur_ident_number,
            step_number,
            "next_ident_number"
        ).unwrap();

        self.builder.build_store(ident_alloca, next_ident_number).unwrap();
        self.builder.build_unconditional_branch(cond_bb).unwrap();


        //for.end
        self.builder.position_at_end(end_bb);
    }
    fn if_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){
        let idxs = &self.edges[l_edge_idx..=r_edge_idx];
        let func = self.current_fn.unwrap();


    }
    fn while_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){
        let idxs = &self.edges[l_edge_idx..=r_edge_idx];
        let func = self.current_fn.unwrap();
        let cond_idx = idxs[0];
        let stmt_idx = idxs[1];

        let cond_bb = self.context.append_basic_block(func, "while.cond");
        let body_bb = self.context.append_basic_block(func, "while.body");
        let end_bb = self.context.append_basic_block(func, "while.end");
        self.ctn_goto_block.push(cond_bb);
        self.brk_goto_block.push(end_bb);


        //while.cond
        self.builder.build_unconditional_branch(cond_bb).unwrap();
        self.builder.position_at_end(cond_bb);
        let cond_val = self.expr(cond_idx as usize, &Type::i64).into_int_value();
        self.builder.build_conditional_branch(cond_val, body_bb, end_bb).unwrap();

        //while.body
        self.builder.position_at_end(body_bb);
        self.compile_stmts(stmt_idx as usize);
        self.builder.build_unconditional_branch(cond_bb).unwrap();

        //while.end
        self.builder.position_at_end(end_bb);


    }

    fn ctn_stmt(&mut self){
        let bb = self.ctn_goto_block.pop().unwrap();
        self.builder.build_unconditional_branch(bb).unwrap();
    }
    fn brk_stmt(&mut self){
        let exit_bb = self.brk_goto_block.pop().unwrap();
        self.builder.build_unconditional_branch(exit_bb).unwrap();
    }

    fn ret_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){
        let idxs = &self.edges[l_edge_idx..=r_edge_idx];
        let l_idx = idxs[0];
        let r_idx = idxs[1];
        let last_basic_block = self.current_fn.unwrap().get_last_basic_block().unwrap();
        self.builder.position_at_end(last_basic_block);

        //return void
        if l_idx == NONE && r_idx == NONE{
            self.builder.build_return(None).unwrap();
        }else{
            let expr = self.expr(r_idx as usize, &self.ret_type.clone());
            self.builder.build_return(Some(&expr)).unwrap();
        }
    }

    pub fn call_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){
        let idxs = &self.edges[l_edge_idx..=r_edge_idx];
        let name_idx = idxs[0] as usize;
        let name_key = self.nodes[name_idx].key;
        let param_type_interval = self.get_param_interval(name_key);
        let start = param_type_interval.start;
        let end = param_type_interval.end;
        let has_param = (start != NONE) && (end != NONE);

        let param_idxs : Vec<usize> = self.edges[l_edge_idx + 1..=r_edge_idx]
            .iter()
            .map(|&i| i as usize)
            .collect();

        let param_type: Vec<Type> = self.types[start as usize..=end as usize]
            .iter()
            .map(|t| t.clone())
            .collect();

        let mut params : Vec<BasicMetadataValueEnum> = Vec::new();

        if has_param{
            for (i, &idx) in param_idxs.iter().enumerate() {
                let ty = &param_type[i];
                let val = if ty != &Type::bool {
                    self.expr(idx, &ty)
                } else {
                    self.expr(idx, &Type::i64)
                };

                params.push(val.into());
            }
        }

        let func = self.get_function(name_key);
        let interval  = &self.intervals[name_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let name_str = &self.input[start..end];
        self.builder.build_call(func, &params, format!("function {name_str}").as_str()).unwrap();
    }
    pub fn call(&mut self, l_edge_idx : usize, r_edge_idx : usize) -> BasicValueEnum<'ctx> {
        let idxs = &self.edges[l_edge_idx..=r_edge_idx];
        let name_idx = idxs[0] as usize;
        let name_key = self.nodes[name_idx].key;
        let param_type_interval = self.get_param_interval(name_key);
        let start = param_type_interval.start;
        let end = param_type_interval.end;
        let has_param = (start != NONE) && (end != NONE);

        let param_idxs : Vec<usize> = self.edges[l_edge_idx + 1..=r_edge_idx]
            .iter()
            .map(|&i| i as usize)
            .collect();

        let param_type: Vec<Type> = self.types[start as usize..=end as usize]
            .iter()
            .map(|t| t.clone())
            .collect();

        let mut params : Vec<BasicMetadataValueEnum> = Vec::new();

        if has_param {
            for (i, &idx) in param_idxs.iter().enumerate() {
                let ty = &param_type[i];
                let val = if ty != &Type::bool {
                    self.expr(idx, &ty)
                } else {
                    self.expr(idx, &Type::i64)
                };

                params.push(val.into());
            }
        }

        let func = self.get_function(name_key);
        let interval  = &self.intervals[name_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let name_str = &self.input[start..end];
        let func_call_val = self.builder.build_call(func, &params, format!("func_{name_str}").as_str()).unwrap();
        func_call_val.try_as_basic_value().unwrap_basic()
    }


    #[inline(always)]
    pub fn type_get(&self, var_key : &u32) -> Type{
        let mut current = 0;
        while var_key != &self.key_checker[current] && current < self.key_checker.len(){
            current += 1;
        }

         self.types[current].clone()
    }

    #[inline(always)]
    pub fn variable_get(&self, var_key : &u32) -> PointerValue<'ctx>{
        let mut current = 0;
        while var_key != &self.key_checker[current] && current < self.key_checker.len(){
            current += 1;
        }

        self.variables[current]
    }
    
    
    
    

}
