use inkwell::values::{BasicValueEnum};
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
                Token::BrkStmt => self.brk_stmt(l_edge_idx, r_edge_idx),
                Token::CtnStmt => self.ctn_stmt(l_edge_idx, r_edge_idx),
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
        let variable = self.expr(expr_idx, ty);
        let var_type : inkwell::types::BasicTypeEnum<'ctx> = match variable{
            BasicValueEnum::IntValue(v) => v.get_type().into(),
            BasicValueEnum::FloatValue(v) => v.get_type().into(),
            _ => unreachable!()
        };

        let interval = &self.intervals[name_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let name_str = &self.input[start..end];

        let alloca =self.builder.build_alloca(var_type, name_str).unwrap();
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


        let s_number : BasicValueEnum = self.expr(idxs[1] as usize, Type::i64);
        let e_number : BasicValueEnum = self.expr(idxs[2] as usize, Type::i64);
        let step_number : BasicValueEnum = self.expr(idxs[3] as usize, Type::i64);


        let name_str = &self.input[start..end];
        let i64_type = self.context.i64_type();
        let ident_alloca = self.builder.build_alloca(i64_type, name_str).unwrap();
        self.builder.build_store(ident_alloca, s_number).unwrap();

        let func = self.current_fn.unwrap();
        let cond_bb = self.context.append_basic_block(func, "cond");
        let body_bb = self.context.append_basic_block(func, "block");
        let step_bb = self.context.append_basic_block(func, "step");
        let end_bb = self.context.append_basic_block(func, "end");

        self.builder.build_unconditional_branch(cond_bb).unwrap();

        //Cond
        self.builder.position_at_end(cond_bb);
        let cur_ident_number = self.builder.build_load(i64_type, ident_alloca, name_str);
        let end_number = e_number.build_int_value();

        let cond_bool = self.builder.build_int_compare(
            inkwell::IntPredicate::SLT,
            cur_ident_number,
            end_number,
            "cond"
        ).unwrap();

        self.builder.build_conditional_branch(cond_bb, body_bb, end_bb).unwrap();


        //body
        self.builder.position_at_end(body_bb);
        self.compile_stmts(idxs[4] as usize);


        
    }
    fn if_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    fn while_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    fn brk_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    fn ctn_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    fn ret_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    pub fn call_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}




    #[inline(always)]
    fn type_get(&mut self, var_key : &u32) -> Type{
        let mut current = 0;
        while var_key != &self.key_checker[current] && current < self.key_checker.len(){
            current += 1;
        }

         self.types[current].clone()

        
    }

}
