use std::env::var;
use inkwell::values::{BasicValueEnum, PointerValue};
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

    fn for_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
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
