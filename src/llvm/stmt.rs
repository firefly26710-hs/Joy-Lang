use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, PointerValue};
use crate::lex::token::Token;
use crate::llvm::codegen::Codegen;
use crate::par::ty::Type;

impl<'ctx> Codegen<'ctx>{
    pub fn build_block(&mut self, stmts_idx : u32){
        let block_node = &self.parser.nodes[stmts_idx as usize];
		let idxs = &block_node.idxs;
		let start = idxs.start as usize;
		let end = idxs.end as usize;
		let childs = &self.parser.edges[start..=end];
		let len = childs.len();
		let ptr = childs.as_ptr();
		for i in 0..len{
			let stmt_node = &self.parser.nodes[unsafe{*ptr.add(i)} as usize];
			let tok = &stmt_node.tok;
			let idxs = &stmt_node.idxs;
			let start = idxs.start as usize;
			let end = idxs.end as usize;
			match tok{
				Token::LetStmt => self.build_let_stmt(start, end),
				Token::IfStmt => self.build_if_stmt(start, end),
				Token::RetStmt => self.build_ret_stmt(start, end),
				Token::CallStmt => self.build_call_stmt(start, end),
				Token::ForStmt => self.build_for_stmt(start, end),
				Token::WhileStmt => self.build_while_stmt(start, end),
				Token::BrkStmt => self.build_brk_stmt(),
				Token::CtnStmt => self.build_ctn_stmt(),
				Token::PrintStmt => self.build_print_stmt(start, end),
				_ => unreachable!()
			}
		}

    }

	fn build_let_stmt(&mut self, start : usize, end : usize){
        let childs = &self.parser.edges[start..=end];
        let name_node = &self.parser.nodes[childs[0] as usize];
        let name_key = name_node.key;

        //build expression 
        let declared_type = self.parser.table.get_var_type(name_key).clone();
		let expr_idx = childs[1] as usize;
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

	fn build_if_stmt(&mut self, start : usize, end : usize){
		let func = self.current_fn.unwrap();
		self.builder.position_at_end(func.get_last_basic_block().unwrap());

		let branch_num = &self.parser.edges[start..=end].len() >> 1;
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


				let curr_expr_idx = self.parser.edges[start..=end][i << 1];
				let curr_stmt_idx = self.parser.edges[start..=end][(i << 1) + 1];
				let prev_expr_idx = self.parser.edges[start..=end][(i - 1) << 1];
				let prev_stmt_idx = self.parser.edges[start..=end][((i - 1) << 1) + 1];


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
					self.build_block(prev_stmt_idx);
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
					self.build_block(prev_stmt_idx);

					//cur cond
					self.builder.position_at_end(cur_cond_bb);
					self.builder.build_unconditional_branch(self.body_blocks[i]).unwrap();

					self.builder.position_at_end(self.body_blocks[i]);
					self.build_block(curr_stmt_idx );
					self.builder.build_unconditional_branch(end_bb).unwrap();
				}
			}

			let last_expr_idx = self.parser.edges[start..=end][(branch_num - 1) << 1];
			let last_stmt_idx = self.parser.edges[start..=end][((branch_num - 1) << 1) + 1];

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
				self.build_block(last_stmt_idx);
				self.builder.build_unconditional_branch(end_bb).unwrap();
			}
		}else{
				let if_cond = self.cod_blocks[0];
				let if_body = self.body_blocks[0];

				let expr_idx = self.parser.edges[start..=end][0];
				let stmt_idx = self.parser.edges[start..=end][1];
				self.builder.position_at_end(if_cond);
				let cond_val = self
					.build_expr(expr_idx as usize, &Type::i32)
					.into_int_value();

				self.builder.build_conditional_branch(cond_val, if_body, end_bb).unwrap();

				self.builder.position_at_end(if_body);
				self.build_block(stmt_idx);
				self.builder.build_unconditional_branch(end_bb).unwrap();
		}

		self.builder.position_at_end(end_bb);
	}
	fn build_ret_stmt(&mut self, start : usize, end : usize){
		let is_return_void = self.ret_type == Type::void;

		if!is_return_void{
			let childs = &self.parser.edges[start..=end];
			let expr_idx = childs[1] as usize;
			let expr = self.build_expr(expr_idx, &self.ret_type.clone());
			self.builder.build_return(Some(&expr)).unwrap();
		}else{
			self.builder.build_return(None).unwrap();
		}
	}

	pub fn build_call_stmt(&mut self, start : usize, end : usize){

		let name_idx = self.parser.edges[start..=end][0];
		let name_key = self.parser.nodes[name_idx as usize].key;
		let has_param = self.parser.edges[start..=end].len() > 1;
		let len = &self.parser.edges[start..=end].len() - 1;

		if has_param{
			for i in 0..len{
				let ty = &self.parser.table.get_param_types(name_key)[i].clone();
				let expr_idx = self.parser.edges[start + 1 +i] as usize;
				let val = if ty != &Type::bool {
					self.build_expr(expr_idx, &ty)
				} else {
					self.build_expr(expr_idx, &Type::i64)
				};

				self.call_param_types.push(val.into());
			}
		}

		let func = self.get_function(name_key);
		let str_interval = &self.parser.lexer.intervals[name_key as usize];
		let str_s = str_interval.start as usize;
		let str_e = str_interval.end as usize;
		let name_str = &self.parser.lexer.input[str_s..str_e];
		self.builder.build_call(func, &self.call_param_types, format!("function {name_str}").as_str()).unwrap();
		self.call_param_types.clear();

	}
	fn build_for_stmt(&mut self, start : usize, end : usize){

		//get name str
		let name_node_idx = self.parser.edges[start..=end][0];
		let name_node =  &self.parser.nodes[name_node_idx as usize];
		let name_key = name_node.key;

		let str_interval = &self.parser.lexer.intervals[name_key as usize];
		let str_s = str_interval.start as usize;
		let str_e = str_interval.end as usize;

		let s_number_idx = self.parser.edges[start..=end][1] as usize;
		let e_number_idx =  self.parser.edges[start..=end][2] as usize;
		let step_number_idx = self.parser.edges[start..=end][3] as usize;
		//build range number





		let s_number : BasicValueEnum = self.build_expr(s_number_idx, &Type::i64);
		let e_number : BasicValueEnum = self.build_expr(e_number_idx, &Type::i64);
		let step_number : BasicValueEnum = self.build_expr(step_number_idx, &Type::i64);


		let i64_type = self.context.i64_type();
		let name_str = &self.parser.lexer.input[str_s..str_e];
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
		let stmts_idx = self.parser.edges[start..=end][4];
		self.builder.position_at_end(body_bb);
		self.build_block(stmts_idx);
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
	fn build_while_stmt(&mut self, start : usize, end : usize){
		let child = &self.parser.edges[start..=end];
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
		self.build_block(stmt_idx);
		self.builder.build_unconditional_branch(cond_bb).unwrap();

		//while.end
		self.builder.position_at_end(end_bb);
	}
	fn build_brk_stmt(&mut self){
		let exit_bb = self.brk_goto_block.pop().unwrap();
		self.builder.build_unconditional_branch(exit_bb).unwrap();
	}
	fn build_ctn_stmt(&mut self){
		let bb = self.ctn_goto_block.pop().unwrap();
		self.builder.build_unconditional_branch(bb).unwrap();
	}

	fn build_print_stmt(&mut self, start : usize, end : usize){
		let i32_type = self.context.i32_type();
		let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
		let print_type = i32_type.fn_type(
			&[i8_ptr_type.into()], true
		);
		let print_func = self.module.add_function("printf", print_type, None);

		let expr_idx = self.parser.edges[start..=end][1] as usize;
		let tok = &self.parser.nodes[expr_idx].tok;
		let ty = match tok{
			Token::Int    => &Type::i32,
			Token::Float  => &Type::f64,
			Token::String => &Type::String,
			Token::Bool   => &Type::bool,
			Token::Ident  => {
				let key = self.parser.nodes[expr_idx].key;
				&self.parser.table.get_var_type(key).clone()
			},
			_ => unreachable!()
		};

		let format = match ty{
			Type::i32 | Type::bool    => "%d",
			Type::f64    			  => "%f",
			Type::String 			  => "&s",
			_ => unreachable!()
		};

		let expr_val = self.build_expr(expr_idx, ty);
		let format_str = self.builder.build_global_string_ptr(
			format, "print_fmt"
		).unwrap();
		self.builder.build_call(
			print_func,
			&[
				format_str.as_pointer_value().into(),
				expr_val.into()
			],
			"print_call"
		).unwrap();



	}

	pub fn get_variable(&self, key : u32) -> PointerValue<'ctx>{
		let mut current = 0;
		while current < self.parser.table.var_keys.len(){
			if self.parser.table.var_keys[current] != key{
				current += 1;
				continue;
			}else{
				break;
			}
		}
		self.variables[current]
	}
}
