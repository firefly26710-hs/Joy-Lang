use inkwell::types::{BasicTypeEnum};
use inkwell::values::{BasicValueEnum, PointerValue};
use crate::lex::iterval::{End, Start};
use crate::lex::token::Token;
use crate::llvm::codegen::Codegen;
use crate::par::ast::node::Key;
use crate::par::parser::parser::NodeIdx;
use crate::par::parser::ty::Type;

impl<'ctx> Codegen<'ctx>{
    pub fn build_stmts(&mut self, stmts_idx : NodeIdx){
        let stmts_node_edges = &self.get_node(stmts_idx).edge_span;
        let edges_start = stmts_node_edges.start ;
        let edges_end = stmts_node_edges.end;

        let len = self.get_edge_slice(edges_start, edges_end).len();

        for i in 0..len{
            let stmt_node_idx = self.get_edge_slice(edges_start, edges_end)[i];
            let stmt_node = self.get_node(stmt_node_idx);

            let tok = &stmt_node.token;
            let idxs = &stmt_node.edge_span;
            let start = idxs.start ;
            let end = idxs.end ;

            match tok{
                Token::LetStmt => self.build_var_stmt(start, end),
                Token::Equal|Token::PlusEq | Token::MinusEq | Token::MulEq | Token::DivEq|
                Token::ModEq => self.build_update_data(start, end, tok.clone()),
                Token::IfStmt => self.build_if_stmt(start, end),
                Token::RetStmt => self.build_ret_stmt(start, end),
                Token::CallStmt => self.build_call_stmt(start, end),
                Token::ForStmt => self.build_for_stmt(start, end),
                Token::WhileStmt => self.build_while_stmt(start, end),
                Token::BrkStmt => self.build_brk_stmt(),
                Token::CtnStmt => self.build_ctn_stmt(),
                _ => unreachable!()
            }
        }
    }

    pub fn build_var_stmt(&mut self, start : Start, end : End){
        let edges = self.get_edge_slice(start, end);
        let name_node = &self.parser.ast.nodes[edges[0] as usize];
        let name_key = name_node.key;

        //build expression
        let declared_type = self.parser.table.look_var_type(name_key).clone();
        let expr_idx = edges[1] as usize;
        let expr = self.build_expr(expr_idx, &declared_type);


        //get variable name
        let str_interval = &self.parser.lexer.intervals[name_key as usize];
        let str_s = str_interval.start as usize;
        let str_e = str_interval.end as usize;
        let name_str = &self.parser.lexer.input[str_s..str_e];


        //get type
        let var_type : BasicTypeEnum<'ctx> = match expr{
            BasicValueEnum::IntValue(v) => v.get_type().into(),
            BasicValueEnum::FloatValue(v) => v.get_type().into(),
            BasicValueEnum::PointerValue(v) => v.get_type().into(),
            _ => unreachable!()
        };


        //build alloc
        let alloc = self.builder.build_alloca(var_type, name_str).unwrap();
        self.builder.build_store(alloc, expr).unwrap();
        self.variables.push(alloc);
    }

    pub fn build_update_data(&mut self ,start: Start, end: End, op_tok : Token){
        let edges = self.get_edge_slice(start, end);
        let key = self.get_node(edges[0]).key;

        let (str_start, str_end) = self.get_str_interval(key);

        let name_str = &self.parser.lexer.input[str_start as usize..str_end as usize];
        let var_ptr = self.get_var_ptr(key);
        let ty = self.parser.table.look_var_type(key).clone();

        let ty_context : BasicTypeEnum = match ty{
            Type::i8 => self.context.i8_type().into(),
            Type::i16 => self.context.i16_type().into(),
            Type::i32 =>  self.context.i32_type().into(),
            Type::i64 =>  self.context.i64_type().into(),
            Type::i128 => self.context.i8_type().into(),
            Type::f32 => self.context.f32_type().into(),
            Type::f64 => self.context.f64_type().into(),
            _ => panic!()
        };

        let prev_val = self.builder.build_load(ty_context, var_ptr, name_str).unwrap();

        let rhs_val = self.build_expr(edges[1] as usize, &ty);

        if matches!(op_tok, Token::Equal){
            self.builder.build_store(var_ptr, rhs_val).unwrap();
            return;
        }


        let new_val : BasicValueEnum = match (prev_val, rhs_val){
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => match op_tok{
                Token::PlusEq => self.builder.build_int_add(l, r, "add_tmp").unwrap().into(),
                Token::MinusEq => self.builder.build_int_sub(l, r, "sub_tmp").unwrap().into(),
                Token::MulEq => self.builder.build_int_mul(l, r, "mul_tmp").unwrap().into(),
                Token::DivEq => self.builder.build_int_signed_div(l, r, "div_tmp").unwrap().into(),
                Token::ModEq => self.builder.build_int_signed_rem(l, r, "rem_tmp").unwrap().into(),
                _ => panic!("Expected Op : {:?}", op_tok)
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => match op_tok{
                Token::PlusEq => self.builder.build_float_add(l, r, "add_tmp").unwrap().into(),
                Token::MinusEq => self.builder.build_float_sub(l, r, "sub_tmp").unwrap().into(),
                Token::MulEq => self.builder.build_float_mul(l, r, "mul_tmp").unwrap().into(),
                Token::DivEq => self.builder.build_float_div(l, r, "div_tmp").unwrap().into(),
                Token::ModEq => self.builder.build_float_rem(l, r, "rem_tmp").unwrap().into(),
                _ => panic!("Expected Op : {:?}", op_tok)
            }
            _ => unreachable!()
        };

        self.builder.build_store(var_ptr, new_val).unwrap();
    }

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


    pub fn get_var_ptr(&self, input_key : Key) -> PointerValue<'ctx>{
        for (i, key) in self.parser.table.var_keys.iter().enumerate(){
            if input_key == *key{
                return self.variables[i];
            }
        }

        panic!("Can't find variable")
    }


}


