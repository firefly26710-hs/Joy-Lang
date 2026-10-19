use crate::lexer::interval::{End, Start};
use crate::llvm::codegen::Codegen;
use crate::parser::types::logic::Type;

impl<'ctx>Codegen<'ctx>{
    pub fn build_if_stmt(&mut self ,start: Start, end : End){
        let func = self.current_fn.unwrap();
        self.builder.position_at_end(func.get_last_basic_block().unwrap());

        let branch_num = &self.get_edge_slice(start, end).len() >> 1;
        for i in 0..branch_num{
            let cond_block = self.context
                .append_basic_block(func, format!("cond.{i}").as_str());

            let body_block = self.context
                .append_basic_block(func, format!("body.{i}").as_str());

            self.cod_blocks.push(cond_block);
            self.body_blocks.push(body_block)
        }

        let end_bb = self.context.append_basic_block(func, "end.bb");

        self.builder.build_unconditional_branch(self.cod_blocks[0]).unwrap();

        if self.cod_blocks.len() > 2 && self.body_blocks.len() > 2{
            for i in 1..branch_num{
                let prev_cond_bb = self.cod_blocks[i - 1];
                let prev_body_bb = self.body_blocks[i - 1];
                let cur_cond_bb = self.cod_blocks[i];


                let curr_expr_idx = self.get_edge_slice(start, end)[i << 1];
                let curr_stmt_idx = self.get_edge_slice(start, end)[(i << 1) + 1];
                let prev_expr_idx = self.get_edge_slice(start, end)[(i - 1) << 1];
                let prev_stmt_idx = self.get_edge_slice(start, end)[((i - 1) << 1) + 1];


                let is_else = curr_expr_idx == 4294967295;

                if !is_else{
                    //prev cond
                    self.builder.position_at_end(prev_cond_bb);

                    let prev_cond_val = self
                        .build_expr(prev_expr_idx as usize, &Type::i32)
                        .into_int_value();
                    self.builder.build_conditional_branch(prev_cond_val, prev_body_bb, cur_cond_bb).unwrap();

                    //prev body
                    self.builder.position_at_end(prev_body_bb);
                    self.build_stmts(prev_stmt_idx);
                    self.builder.build_unconditional_branch(end_bb).unwrap();
                }else{
                    //prev cond
                    self.builder.position_at_end(prev_cond_bb);
                    let prev_cond_val = self
                        .build_expr(prev_expr_idx as usize, &Type::i32)
                        .into_int_value();
                    self.builder.build_conditional_branch(prev_cond_val, prev_body_bb, cur_cond_bb).unwrap();

                    //prev body
                    self.builder.position_at_end(prev_body_bb);
                    self.build_stmts(prev_stmt_idx);

                    //cur cond
                    self.builder.position_at_end(cur_cond_bb);
                    self.builder.build_unconditional_branch(self.body_blocks[i]).unwrap();

                    self.builder.position_at_end(self.body_blocks[i]);
                    self.build_stmts(curr_stmt_idx );
                    self.builder.build_unconditional_branch(end_bb).unwrap();
                }
            }

            let last_expr_idx = self.get_edge_slice(start, end)[(branch_num - 1) << 1];
            let last_stmt_idx = self.get_edge_slice(start, end)[((branch_num - 1) << 1) + 1];

            if last_expr_idx != 4294967295 {
                let prev_cond_bb = *self.cod_blocks.last().unwrap();
                let prev_body_bb = *self.body_blocks.last().unwrap();

                //prev cond
                self.builder.position_at_end(prev_cond_bb);
                let cond_val = self
                    .build_expr(last_expr_idx as usize, &Type::i32)
                    .into_int_value();

                self.builder.build_conditional_branch(cond_val, prev_body_bb, end_bb).unwrap();

                //prev body
                self.builder.position_at_end(prev_body_bb);
                self.build_stmts(last_stmt_idx);
                self.builder.build_unconditional_branch(end_bb).unwrap();
            }
        }else{
            let if_cond = self.cod_blocks[0];
            let if_body = self.body_blocks[0];

            let expr_idx = self.get_edge_slice(start, end)[0];
            let stmt_idx = self.get_edge_slice(start, end)[1];
            self.builder.position_at_end(if_cond);
            let cond_val = self
                .build_expr(expr_idx as usize, &Type::i32)
                .into_int_value();

            self.builder.build_conditional_branch(cond_val, if_body, end_bb).unwrap();

            self.builder.position_at_end(if_body);
            self.build_stmts(stmt_idx);
            self.builder.build_unconditional_branch(end_bb).unwrap();
        }

        self.builder.position_at_end(end_bb);
    }

    pub fn build_brk_stmt(&mut self){
        let exit_bb = self.brk_goto_block.pop().unwrap();
        self.builder.build_unconditional_branch(exit_bb).unwrap();


    }
    pub fn build_ctn_stmt(&mut self){
        let bb = self.ctn_goto_block.pop().unwrap();
        self.builder.build_unconditional_branch(bb).unwrap();
    }
}