use inkwell::values::BasicValueEnum;
use inkwell::{IntPredicate, FloatPredicate};
use crate::llvm::codegen::Codegen;
use crate::par::parser::NONE;
use crate::par::ty::Type;
use crate::lex::token::Token;

impl<'ctx> Codegen<'ctx>{
    pub fn expr(&mut self, expr_idx : usize, ty : Type) -> BasicValueEnum<'ctx>{
        let node  = &self.nodes[expr_idx];
        let token = &node.token.clone();
        let key = node.key as usize;
        let l_idx = self.edges[node.l_edge_idx as usize];
        let r_idx = self.edges[node.r_edge_idx as usize];
        dbg!(node);

        if !matches!(token, &Token::CallStmt){
            match (l_idx, r_idx){
                (NONE, NONE) =>{
                    let interval = &self.intervals[key];
                    let start = interval.start as usize;
                    let end = interval.end as usize;
                    let var_str = &self.input[start..end];
                    
                    match token{
                        Token::Int => {
                            let ty = self.int_type_mapping(&ty);
                            let integer : i64 = var_str.parse().unwrap();
                            return ty.const_int(integer as u64, false).into();
                        },
                        Token::Ident => {todo!()},
                        Token::True  => {todo!()},
                        Token::False => {todo!()}
                        Token::Float => {
                            let ty = self.float_type_mapping(&ty);
                            let float : f64 = var_str.parse().unwrap();
                            return ty.const_float(float).into()
                        },
                        _ => unreachable!(),
                    }
                },
                (NONE, r_idx) => {todo!()},
                (l_idx, r_idx) => {
                    let lhs = self.expr(l_idx as usize, ty.clone());
                    let rhs = self.expr(r_idx as usize, ty);

                    match (lhs, rhs){
                        (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                            match token{
                                Token::Plus   =>{
                                    self.builder.build_int_add(l, r, "INT_ADD").unwrap().into()
                                }, 
                                Token::Minus  =>{
                                    self.builder.build_int_sub(l, r, "INT_SUB").unwrap().into()
                                },
                                Token::Mul    =>{
                                    self.builder.build_int_mul(l, r, "INT_MUL").unwrap().into()
                                },
                                Token::Div    => {
                                    self.builder.build_int_signed_div(l, r, "INT_DIV").unwrap().into()
                                },
                                Token::Mod    => {
                                    self.builder.build_int_signed_rem(l, r, "INT_MOD").unwrap().into()
                                },
                                Token::Les    => {
                                    self.builder.build_int_compare(
                                        IntPredicate::SLT,
                                        l, r,
                                        "INT_LESSER"
                                    ).unwrap().into()
                                },
                                Token::Grt => {
                                    self.builder.build_int_compare(
                                        IntPredicate::SGT,
                                        l, r,
                                        "INT_LESSER"
                                    ).unwrap().into()
                                },
                                Token::Grt => {
                                    self.builder.build_int_compare(
                                        IntPredicate::SGT,
                                        l, r,
                                        "INT_GREATER"
                                    ).unwrap().into()
                                },
                                Token::LesEq => {
                                    self.builder.build_int_compare(
                                        IntPredicate::SLE,
                                        l, r,
                                        "INT_GREATER"
                                    ).unwrap().into()
                                },
                                Token::GrtEq => {
                                    self.builder.build_int_compare(
                                        IntPredicate::SGE,
                                        l, r,
                                        "INT_GREATER"
                                    ).unwrap().into()
                                },
                                Token::EqEq => {
                                    self.builder.build_int_compare(
                                        IntPredicate::EQ,
                                        l, r,
                                        "INT_EQUELEQUEL"
                                    ).unwrap().into()
                                },
                                _ => unreachable!()
                            }
                        },
                        (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                            match token{
                                Token::Plus   =>{
                                    self.builder.build_float_add(l, r, "FLOAT_ADD").unwrap().into()
                                }, 
                                Token::Minus  =>{
                                    self.builder.build_float_sub(l, r, "FLOAT_SUB").unwrap().into()
                                },
                                Token::Mul    =>{
                                    self.builder.build_float_mul(l, r, "FLOAT_MUL").unwrap().into()
                                },
                                Token::Div    => {
                                    self.builder.build_float_div(l, r,"FLOAT_DIV").unwrap().into()
                                },
                                Token::Mod    => {
                                    self.builder.build_float_rem(l, r,"FLOAT_MOD").unwrap().into()
                                },
                                Token::Les    => {
                                    self.builder.build_float_compare(
                                        FloatPredicate::OLT,
                                        l, r,
                                        "FLOAT_LESSER"
                                    ).unwrap().into()
                                },
                                Token::Grt => {
                                    self.builder.build_float_compare(
                                        FloatPredicate::OGT,
                                        l, r,
                                        "FLOAT_GREATER"
                                    ).unwrap().into()
                                },
                                Token::LesEq => {
                                    self.builder.build_float_compare(
                                        FloatPredicate::OLE,
                                        l, r,
                                        "FLOAT_GREATER"
                                    ).unwrap().into()
                                },
                                Token::GrtEq => {
                                    self.builder.build_float_compare(
                                        FloatPredicate::OGE,
                                        l, r,
                                        "FLOAT_GREATER"
                                    ).unwrap().into()
                                },
                                Token::EqEq => {
                                    self.builder.build_float_compare(
                                        FloatPredicate::OEQ,
                                        l, r,
                                        "FLOAT_EQUELEQUEL"
                                    ).unwrap().into()
                                },
                                _ => unreachable!()
                            }
                            
                        },
                        _ => unreachable!()
                    }

                },
                _ => unreachable!(),
            }
        } else{
           todo!()// self.call_stmt(l_idx as usize, r_idx as usize);
        }
        
    }

    #[inline(always)]
    pub fn int_type_mapping(&self ,ty : &Type) -> inkwell::types::IntType<'ctx>{
        match ty{
            Type::i32  => return self.context.i32_type().into(),
            Type::i64  => return self.context.i64_type().into(),
            _ => unreachable!()
        }
    }

    #[inline(always)]
    pub fn float_type_mapping(&self ,ty : &Type) -> inkwell::types::FloatType<'ctx>{
        match ty{
            Type::f32  => return self.context.f32_type().into(),
            Type::f64  => return self.context.f64_type().into(),
            _ => unreachable!()
        }
    }

    #[inline(always)]
    pub fn bool_type_mapping(&self, ty : &Type) -> inkwell::types::IntType<'ctx>{
        self.context.bool_type().into()
    }

}
