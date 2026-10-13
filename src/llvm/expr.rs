use crate::llvm::codegen::Codegen;
use inkwell::values::{BasicValue, BasicValueEnum};
use inkwell::{IntPredicate, FloatPredicate};
use crate::par::ty::Type;
use crate::par::parser::NONE;
use crate::lex::token::Token;

impl<'ctx>Codegen<'ctx>{

    pub fn build_expr(&mut self, expr_idx : usize, ty : &Type) -> BasicValueEnum<'ctx>{
        let node = &self.parser.nodes[expr_idx];
        let tok = &node.tok;
        let key = node.key;
        let idxs = &node.idxs;
        let start = idxs.start;
        let end = idxs.end;
        let str_interval = &self.parser.lexer.intervals[key as usize];
        let str_s = str_interval.start as usize;
        let str_e = str_interval.end as usize;
        
        if !matches!(tok, &Token::CallStmt){
            match(start, end){
                (NONE, NONE) => match tok{
                    Token::Int => {
                        let int_ty = self.int_type_mapping(&ty);
                        let var_str = &self.parser.lexer.input[str_s..str_e];
                        let u_integer : u64 = var_str.parse().unwrap();
                        int_ty.const_int(u_integer, false).into()
                    },
                    Token::Ident => {
                        todo!()
                    },
                    Token::Float => {
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
                            let lhs = self.build_expr(idxs[0] as usize, ty);
                            let rhs = self.build_expr(idxs[1] as usize, ty);

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
                        _ => unreachable!()
                    }
                }

            }



        }else{
            todo!()//self.build_call_stmt(l_idx, r_idx);
        }

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
            Type::f64 => return self.context.f64_type().into(),
            Type::f32 => return self.context.f32_type().into(),
            _ => unreachable!()
        }
    }
}
    
