use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::llvm::codegen::Codegen;
use crate::parser::tree::ast::NONE;
use crate::parser::types::logic::Type;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;
use inkwell::{FloatPredicate, IntPredicate};
use crate::lexer::tokenizer::source::token::Token;

impl<'ctx>Codegen<'ctx>{
    pub fn build_expr(&mut self, expr_idx : usize, ty : &Type) -> BasicValueEnum<'ctx>{
        let tok = self.parser.ast.nodes[expr_idx].token.clone();
        let key = self.parser.ast.nodes[expr_idx].key;
        let idxs = &self.parser.ast.nodes[expr_idx].edge_span;
        let start = idxs.start;
        let end = idxs.end;

        if matches!(tok, Token::CallStmt){
           return self.build_expr_call(start , end)
        }

        match(start, end){
            (NONE, NONE) => match tok{
                Token::Int => {
                    let str_interval = &self.parser.lexer.tokenizer.intervals[key as usize];
                    let str_s = str_interval.start as usize;
                    let str_e = str_interval.end as usize;
                    let int_ty = self.int_type_mapping(&ty);
                    let var_str = &self.parser.lexer.tokenizer.input[str_s..str_e];
                    let u_integer : u64 = var_str.parse().unwrap();
                    int_ty.const_int(u_integer, false).into()
                },
                Token::Float => {
                    let str_interval = &self.parser.lexer.tokenizer.intervals[key as usize];
                    let str_s = str_interval.start as usize;
                    let str_e = str_interval.end as usize;
                    let flt_ty = self.flt_type_mapping(&ty);
                    let var_str = &self.parser.lexer.tokenizer.input[str_s..str_e];
                    let float : f64 = var_str.parse().unwrap();
                    flt_ty.const_float(float).into()
                },
                Token::Ident => {
                    let str_interval = &self.parser.lexer.tokenizer.intervals[key as usize];
                    let str_s = str_interval.start as usize;
                    let str_e = str_interval.end as usize;
                    let name_str = &self.parser.lexer.tokenizer.input[str_s ..str_e];
                    let alloc = self.get_var_ptr(key);
                    let var_type = self.parser.look_var_type(&key);

                    let basic_ty: BasicTypeEnum<'ctx> = match var_type {
                        Type::i8 |Type::i16|Type::i32 | Type::i64 | Type::i128 => self.int_type_mapping(var_type).into(),
                        Type::f32 | Type::f64 => self.flt_type_mapping(var_type).into(),
                        Type::bool => self.context.bool_type().into(),
                        Type::String => self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
                        _ => panic!("Expected Type{:?}", var_type)
                    };

                    self.builder.build_load(basic_ty, alloc, name_str).unwrap()

                    },

                    Token::True => {
                        let bool_ty = self.context.bool_type();
                        bool_ty.const_int(1, true).into()
                    },
                    Token::False => {
                        let bool_ty = self.context.bool_type();
                        bool_ty.const_int(0, false).into()
                    }
                    Token::String =>{
                        let str_interval = &self.parser.lexer.tokenizer.intervals[key as usize];
                        let str_s = str_interval.start as usize;
                        let str_e = str_interval.end as usize;
                        let string_content = &self.parser.lexer.tokenizer.input[str_s..str_e];

                        self.builder.build_global_string_ptr(string_content, "str_lit")
                            .unwrap()
                            .as_pointer_value()
                            .into()
                    },
                    _ => unreachable!()
                },
                (start, end) =>{
                    let childs = self.get_edge_slice(start, end);
                    match (childs[0], childs[1]){
                        (NONE, r_idx) => {
                            let rhs = self.build_expr(r_idx as usize, ty);
                            match tok{
                                Token::Sub => match rhs{
                                    BasicValueEnum::IntValue(i_val) => {
                                        self.builder.build_int_neg(i_val, "Neg_Int").unwrap().into()
                                    },
                                    BasicValueEnum::FloatValue(f_val) => {
                                        self.builder.build_float_neg(f_val, "Neg_Float").unwrap().into()
                                    },
                                    _ => unreachable!()
                                },
                                Token::CmpNot => match rhs{
                                    BasicValueEnum::IntValue(i_val) => {
                                        self.builder.build_not(i_val,"Not_Val").unwrap().into()
                                    }
                                    _ => unreachable!()
                                }
                                _ => unreachable!()
                            }
                        },
                        (__l_idx__,__r_idx__) => {
                            let idxs = self.get_edge_slice(start, end);
                            let lhs_idx = idxs[0] as usize;
                            let rhs_idx = idxs[1] as usize;

                            let lhs = if is_bool_op(&tok){
                                self.build_expr(lhs_idx, &Type::i32)
                            }else{
                                self.build_expr(lhs_idx, ty)
                            };

                            let rhs = if is_bool_op(&tok){
                                self.build_expr(rhs_idx, &Type::i32)
                            }else{
                                self.build_expr(rhs_idx, ty)
                            };

                            match(lhs, rhs){
                                (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => match tok{
                                    Token::Add => {
                                        self.builder.build_int_add(l, r, "Int_Add").unwrap().into()
                                    },
                                    Token::Sub => {
                                        self.builder.build_int_sub(l, r, "Int_Sub").unwrap().into()
                                    },
                                    Token::Mul => {
                                        self.builder.build_int_mul(l, r, "Int_Mul").unwrap().into()
                                    },
                                    Token::Div    => {
                                        self.builder.build_int_signed_div(l, r, "Int_Div").unwrap().into()
                                    },
                                    Token::Mod    => {
                                        self.builder.build_int_signed_rem(l, r, "Int_Mod").unwrap().into()
                                    },
                                    Token::Les    => {
                                        self.builder.build_int_compare(
                                            IntPredicate::SLT,
                                            l, r,
                                            "Int_Lesser"
                                        ).unwrap().into()
                                    },
                                    Token::Grt => {
                                        self.builder.build_int_compare(
                                            IntPredicate::SGT,
                                            l, r,
                                            "Int_Greater"
                                        ).unwrap().into()
                                    },
                                    Token::LesEq => {
                                        self.builder.build_int_compare(
                                            IntPredicate::SLE,
                                            l, r,
                                            "Int_LesEq"
                                        ).unwrap().into()
                                    },
                                    Token::GrtEq => {
                                        self.builder.build_int_compare(
                                            IntPredicate::SGE,
                                            l, r,
                                            "Int_GrtEq"
                                        ).unwrap().into()
                                    },
                                    Token::EqEq => {
                                        self.builder.build_int_compare(
                                            IntPredicate::EQ,
                                            l, r,
                                            "Int_EqEq"
                                        ).unwrap().into()
                                    },
                                    _ => unreachable!()
                                },
                                (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => match tok{
                                    Token::Add =>{
                                        self.builder.build_float_add(l, r, "Flt_Add").unwrap().into()
                                    },
                                    Token::Sub =>{
                                        self.builder.build_float_sub(l, r, "Flt_Sub").unwrap().into()
                                    },
                                    Token::Mul    =>{
                                        self.builder.build_float_mul(l, r, "Flt_Mul").unwrap().into()
                                    },
                                    Token::Div    => {
                                        self.builder.build_float_div(l, r, "Flt_Div").unwrap().into()
                                    },
                                    Token::Mod    => {
                                        self.builder.build_float_rem(l, r,"Flt_Mod").unwrap().into()
                                    },
                                    Token::Les    => {
                                        self.builder.build_float_compare(
                                            FloatPredicate::OLT,
                                            l, r,
                                            "Flt_Lesser"
                                        ).unwrap().into()
                                    },
                                    Token::Grt => {
                                        self.builder.build_float_compare(
                                            FloatPredicate::OGT,
                                            l, r,
                                            "Flt_Greater"
                                        ).unwrap().into()
                                    },
                                    Token::LesEq => {
                                        self.builder.build_float_compare(
                                            FloatPredicate::OLE,
                                            l, r,
                                            "Flt_LesEq"
                                        ).unwrap().into()
                                    },
                                    Token::GrtEq => {
                                        self.builder.build_float_compare(
                                            FloatPredicate::OGE,
                                            l, r,
                                            "FLOAT_GrtEq"
                                        ).unwrap().into()
                                    },
                                    Token::EqEq => {
                                        self.builder.build_float_compare(
                                            FloatPredicate::OEQ,
                                            l, r,
                                            "FLOAT_EqEq"
                                        ).unwrap().into()
                                    },
                                    _ => unreachable!()
                                },
                                _  => unreachable!()
                            }

                        }
                    }
                }

            }

    }

    fn build_expr_call(&mut self ,start : Start, end : End) -> BasicValueEnum<'ctx>{
        let name_idx = self.get_edge_slice(start, end)[0];
        let name_key = self.get_node(name_idx).key;
        let has_param = self.get_edge_slice(start, end).len() > 1;
        let len = self.get_edge_slice(start, end).len() - 1;


        if has_param{
            let params_idxs = &self.parser.look_param_interval(name_key);
            let param_start = params_idxs.start as usize;
            let param_end = params_idxs.end as usize;

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
        let func_call_val = self.builder.build_call(func, &self.call_param_types, format!("function {name_str}").as_str()).unwrap();
        self.call_param_types.clear();
        func_call_val.try_as_basic_value().unwrap_basic()
    }



    fn int_type_mapping(&self, ty : &Type)  -> inkwell::types::IntType<'ctx>{
        match ty{
            Type::i8 => self.context.i8_type(),
            Type::i16 => self.context.i16_type(),
            Type::i32 =>  self.context.i32_type(),
            Type::i64 =>  self.context.i64_type(),
            Type::i128 => self.context.i8_type(),
            _ => unreachable!()
        }
    }

    fn flt_type_mapping(&self, ty : &Type) -> inkwell::types::FloatType<'ctx>{
        match ty{
            Type::f32 => self.context.f32_type().into(),
            Type::f64 => self.context.f64_type().into(),
            _ => unreachable!()
        }
    }

}

pub fn is_bool_op(tok : &Token) -> bool{
    matches!(tok, Token::EqEq   | Token::NotEq | Token::Grt | Token::Les| Token::GrtEq  | Token::LesEq )
}
    
