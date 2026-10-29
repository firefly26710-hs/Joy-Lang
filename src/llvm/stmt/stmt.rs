use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::llvm::codegen::Codegen;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, PointerValue};
use crate::lexer::tokenizer::source::token::Token;

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
                Token::Eq|Token::AddEq | Token::SubEq | Token::MulEq | Token::DivEq|
                Token::ModEq => self.build_update_data(start, end, tok.clone()),
                Token::IfStmt => self.build_if_stmt(start, end),
                Token::RetStmt => self.build_ret_stmt(start, end),
                Token::CallStmt => self.build_call_stmt(start, end),
                Token::ForStmt => self.build_for_stmt(start, end),
                Token::WhileStmt => self.build_while_stmt(start, end),
                Token::PrintStmt => self.build_print_stmt(start, end),
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
        let declared_type = self.parser.look_var_type(&name_key).clone();
        let expr_idx = edges[1] as usize;
        let expr = self.build_expr(expr_idx, &declared_type);


        //get variable name
        let str_interval = &self.parser.lexer.tokenizer.intervals[name_key as usize];
        let str_s = str_interval.start as usize;
        let str_e = str_interval.end as usize;
        let name_str = &self.parser.lexer.tokenizer.input[str_s..str_e];


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

        let name_str = &self.parser.lexer.tokenizer.input[str_start as usize..str_end as usize];
        let var_ptr = self.get_var_ptr(key);
        let ty = self.parser.look_var_type(&key).clone();

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

        if matches!(op_tok, Token::Eq){
            self.builder.build_store(var_ptr, rhs_val).unwrap();
            return;
        }


        let new_val : BasicValueEnum = match (prev_val, rhs_val){
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => match op_tok{
                Token::AddEq => self.builder.build_int_add(l, r, "add_tmp").unwrap().into(),
                Token::SubEq => self.builder.build_int_sub(l, r, "sub_tmp").unwrap().into(),
                Token::MulEq => self.builder.build_int_mul(l, r, "mul_tmp").unwrap().into(),
                Token::DivEq => self.builder.build_int_signed_div(l, r, "div_tmp").unwrap().into(),
                Token::ModEq => self.builder.build_int_signed_rem(l, r, "rem_tmp").unwrap().into(),
                _ => panic!("Expected Op : {:?}", op_tok)
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => match op_tok{
                Token::AddEq => self.builder.build_float_add(l, r, "add_tmp").unwrap().into(),
                Token::SubEq => self.builder.build_float_sub(l, r, "sub_tmp").unwrap().into(),
                Token::MulEq => self.builder.build_float_mul(l, r, "mul_tmp").unwrap().into(),
                Token::DivEq => self.builder.build_float_div(l, r, "div_tmp").unwrap().into(),
                Token::ModEq => self.builder.build_float_rem(l, r, "rem_tmp").unwrap().into(),
                _ => panic!("Expected Op : {:?}", op_tok)
            }
            _ => unreachable!()
        };

        self.builder.build_store(var_ptr, new_val).unwrap();
    }

    pub fn build_print_stmt(&mut self, start: Start, end: End){
        let print_func = match self.module.get_function("printf") {
            Some(func) => func,
            None => {
                let i32_type = self.context.i32_type();
                let i8_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
                let print_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
                self.module.add_function("printf", print_type, None)
            }
        };

        let type_idx = self.get_edge_slice(start, end)[0];
        let key = self.get_node(type_idx).key;
        let ty= self.parser.type_mapping(key);

        let format = match ty{
            Type::i32 | Type::bool    => "%d",
            Type::f64    			  => "%f",
            Type::String 			  => "%s",
            _ => unreachable!()
        };

        let expr_idx = self.get_edge_slice(start, end)[1] as usize;

        let expr_val = self.build_expr(expr_idx, &ty);
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


    pub fn get_var_ptr(&self, input_key : Key) -> PointerValue<'ctx>{
        for (i, key) in self.parser.table.var_keys.iter().enumerate(){
            if input_key == *key{
                return self.variables[i];
            }
        }

        panic!("Can't find variable")
    }


}


