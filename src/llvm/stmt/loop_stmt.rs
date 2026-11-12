use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::llvm::codegen::Codegen;
use crate::parser::types::typ::Type;
use inkwell::values::BasicValueEnum;

impl<'ctx>Codegen<'ctx>{
    pub fn build_for_stmt(&mut self ,start : Start, end : End){
        let name_node_idx = self.get_edge_slice(start, end)[0];
        let name_node =  &self.get_node(name_node_idx);
        let name_key = name_node.key;

        let str_interval = &self.parser.lexer.tokenizer.intervals[name_key as usize];
        let str_s = str_interval.start as usize;
        let str_e = str_interval.end as usize;

        let s_number_idx = self.get_edge_slice(start, end)[1] as usize;
        let e_number_idx =  self.get_edge_slice(start, end)[2] as usize;
        let step_number_idx = self.get_edge_slice(start, end)[3] as usize;


        let s_number : BasicValueEnum = self.build_expr(s_number_idx, &Type::i64);
        let e_number : BasicValueEnum = self.build_expr(e_number_idx, &Type::i64);
        let step_number : BasicValueEnum = self.build_expr(step_number_idx, &Type::i64);


        let i64_type = self.context.i64_type();
        let name_str = &self.parser.lexer.tokenizer.input[str_s..str_e];
        let ident_alloc = self.builder.build_alloca(i64_type, name_str).unwrap();

        self.builder.build_store(ident_alloc, s_number).unwrap();

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
            ident_alloc,
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
        let stmts_idx = self.get_edge_slice(start, end)[4];
        self.builder.position_at_end(body_bb);
        self.build_stmts(stmts_idx);
        self.builder.build_unconditional_branch(step_bb).unwrap();


        //for.step
        self.builder.position_at_end(step_bb);
        let step_number = step_number.into_int_value();
        let next_ident_number = self.builder.build_int_add(
            cur_ident_number,
            step_number,
            "next_ident_number"
        ).unwrap();

        self.builder.build_store(ident_alloc, next_ident_number).unwrap();
        self.builder.build_unconditional_branch(cond_bb).unwrap();


        //for.end
        self.builder.position_at_end(end_bb);
    }

    pub fn build_while_stmt(&mut self ,start : Start, end : End){
        let child = self.get_edge_slice(start, end);
        let func = self.current_fn.unwrap();

        let cond_idx = child[0];
        let stmt_idx = child[1];

        let cond_bb = self.context.append_basic_block(func, "while.cond");
        let body_bb = self.context.append_basic_block(func, "while.body");
        let end_bb = self.context.append_basic_block(func, "while.end");
        self.ctn_goto_block.push(cond_bb);
        self.brk_goto_block.push(end_bb);


        //while.cond
        self.builder.build_unconditional_branch(cond_bb).unwrap();
        self.builder.position_at_end(cond_bb);
        let cond_val = self.build_expr(cond_idx as usize, &Type::i32).into_int_value();
        self.builder.build_conditional_branch(cond_val, body_bb, end_bb).unwrap();

        //while.body
        self.builder.position_at_end(body_bb);
        self.build_stmts(stmt_idx);
        self.builder.build_unconditional_branch(cond_bb).unwrap();

        //while.end
        self.builder.position_at_end(end_bb);

    }



}

