use crate::llvm::codegen::Codegen;
use inkwell::values::{BasicValueEnum};
use inkwell::{IntPredicate, FloatPredicate};
use inkwell::types::BasicTypeEnum;
use crate::par::ty::Type;
use crate::par::parser::NONE;
use crate::lex::token::Token;

impl<'ctx>Codegen<'ctx>{

    pub fn build_expr(&mut self, expr_idx : usize, ty : &Type) -> BasicValueEnum<'ctx>{
        let tok = self.parser.nodes[expr_idx].tok.clone();
        let key = &self.parser.nodes[expr_idx].key;
        let idxs = &self.parser.nodes[expr_idx].idxs;
        let start = idxs.start;
        let end = idxs.end;

        
        if !matches!(tok, Token::CallStmt){
            match(start, end){
                (NONE, NONE) => match tok{
                    Token::Int => {
                        let str_interval = &self.parser.lexer.intervals[*key as usize];
                        let str_s = str_interval.start as usize;
                        let str_e = str_interval.end as usize;
                        let int_ty = self.int_type_mapping(&ty);
                        let var_str = &self.parser.lexer.input[str_s..str_e];
                        let u_integer : u64 = var_str.parse().unwrap();
                        int_ty.const_int(u_integer, false).into()
                    },
                    Token::Ident => {
                        let str_interval = &self.parser.lexer.intervals[*key as usize];
                        let str_s = str_interval.start as usize;
                        let str_e = str_interval.end as usize;
                        let name_str = &self.parser.lexer.input[str_s ..str_e];
                        let alloc = self.get_variable(*key);
                        let var_type = self.parser.table.get_var_type(*key);

                        let basic_ty: BasicTypeEnum<'ctx> = match var_type {
                            Type::i32 | Type::i64 => self.int_type_mapping(var_type).into(),
                            Type::f32 | Type::f64 => self.flt_type_mapping(var_type).into(),
                            Type::bool => self.context.bool_type().into(),
                            Type::String => self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(), // 記得字串也是 ptr
                            _ => unreachable!()
                        };


                        self.builder.build_load(basic_ty, alloc, name_str).unwrap()

                    },
                    Token::Float => {
                        let str_interval = &self.parser.lexer.intervals[*key as usize];
                        let str_s = str_interval.start as usize;
                        let str_e = str_interval.end as usize;
                        let flt_ty = self.flt_type_mapping(&ty);
                        let var_str = &self.parser.lexer.input[str_s..str_e];
                        let float : f64 = var_str.parse().unwrap();
                        flt_ty.const_float(float).into()
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
                        let str_interval = &self.parser.lexer.intervals[*key as usize];
                        let str_s = str_interval.start as usize;
                        let str_e = str_interval.end as usize;
                        let string_content = &self.parser.lexer.input[str_s..str_e];

                        self.builder.build_global_string_ptr(string_content, "str_lit")
                            .unwrap()
                            .as_pointer_value()
                            .into()
                    },
                    _ => unreachable!()
                },
                (start, end) =>{
                    let childs = &self.parser.edges[start as usize..=end as usize];
                    match (childs[0], childs[1]){
                        (NONE, r_idx) => {
                            let rhs = self.build_expr(r_idx as usize, ty);
                            match tok{
                                Token::Minus => match rhs{
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
                        (l_idx,r_idx) => {
                            let idxs = &self.parser.edges[start as usize..=end as usize];
                            let lhs_idx = idxs[0] as usize;
                            let rhs_idx = idxs[1] as usize;
                            let lhs = self.build_expr(lhs_idx, ty);
                            let rhs = self.build_expr(rhs_idx, ty);

                            match(lhs, rhs){
                                (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => match tok{
                                    Token::Plus => {
                                        self.builder.build_int_add(l, r, "Int_Add").unwrap().into()
                                    },
                                    Token::Minus => {
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
                                    Token::Plus   =>{
                                        self.builder.build_float_add(l, r, "Flt_Add").unwrap().into()
                                    }, 
                                    Token::Minus  =>{
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



        }else{
            self.build_call(start as usize, end as usize)
        }

    }

    pub fn build_call(&mut self, start : usize, end : usize) -> BasicValueEnum<'ctx> {
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
        let func_call_val = self.builder.build_call(func, &self.call_param_types, format!("function {name_str}").as_str()).unwrap();
        self.call_param_types.clear();
        func_call_val.try_as_basic_value().unwrap_basic()

    }


    #[inline(always)]
    fn int_type_mapping(&self, ty : &Type)  -> inkwell::types::IntType<'ctx>{
        match ty{
            Type::i32 => return self.context.i32_type().into(),
            Type::i64 => return self.context.i64_type().into(),
            _ => unreachable!()
        }
    }

    #[inline(always)]
    fn flt_type_mapping(&self, ty : &Type) -> inkwell::types::FloatType<'ctx>{
        match ty{
            Type::f64 => self.context.f64_type().into(),
            Type::f32 => self.context.f32_type().into(),
            _ => unreachable!()
        }
    }
}
    
