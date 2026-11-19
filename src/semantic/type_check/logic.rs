use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;
use crate::semantic::diagnostic::data::issue::TypeIssue;
use crate::semantic::symbol::data::ty::Type;

impl Semantic{
    pub fn type_check(&mut self, declared_type : &Type, expr_idx : NodeIdx) -> Result<Type, TypeIssue>{
        let expr_type_state = self.get_expr_type_state(expr_idx)?;
        if matches!(declared_type, Type::void){
            match expr_type_state{
                TypeState::Integer(key) => {
                    match self.parse_int_type(key){
                        Type::i8 | Type::i16 | Type::i32 => Ok(Type::i32),
                        Type::i64 => Ok(Type::i64),
                        Type::i128=> Ok(Type::i128),
                        _ => unreachable!()
                    }
                },
                TypeState::Float(key) => {
                    match self.parse_flt_type(key){
                        Type::f32 | Type::f64 => Ok(Type::f64),
                        _ => unreachable!()
                    }
                },
                TypeState::Stable(ty) => {
                    Ok(ty)
                }
            }
        }else{
            match expr_type_state{
                TypeState::Integer(key) => {
                    let expr_type = &self.parse_int_type(key);
                    if !is_in_same_set(declared_type, expr_type){
                        return Err(self.error_mismatch_type(expr_idx, declared_type, expr_type));
                    }

                    if weight(declared_type) >= weight(expr_type){
                        Ok(declared_type.clone())
                    }else{
                        Err(self.error_mismatch_type(expr_idx,declared_type, expr_type))
                    }
                },
                TypeState::Float(key) => {
                    let expr_type = &self.parse_flt_type(key);
                    if !is_in_same_set(declared_type, expr_type){
                        return Err(self.error_mismatch_type(expr_idx, declared_type, expr_type));
                    }

                    if weight(declared_type) >= weight(expr_type){
                        Ok(declared_type.clone())
                    }else{
                        Err(self.error_mismatch_type(expr_idx,declared_type, expr_type))
                    }
                },
                TypeState::Stable(expr_type) => {
                    if !is_in_same_set(declared_type, &expr_type){
                        panic!("Expected {:?}, find {:?}", declared_type, expr_type);
                    }

                    if weight(declared_type) >= weight(&expr_type){
                        Ok(declared_type.clone())
                    }else{
                        panic!("Expression out of range for {:?}", declared_type)
                    }
                }
            }
        }
    }
    fn get_expr_type_state(&mut self, expr_idx : NodeIdx) -> Result<TypeState, TypeIssue>{
        let node = &self.nodes[expr_idx as usize];
        let tok = &node.token;
        let key = node.key;
        let edge_span = &node.edge_span;
        let left_span = edge_span.start;
        let right_span = edge_span.end;

        match(left_span, right_span){
            (NONE, NONE) => match tok{
                Token::Int                  => Ok(TypeState::Integer(key)),
                Token::Float                => Ok(TypeState::Float(key)),
                Token::True | Token::False  => Ok(TypeState::Stable(Type::bool)),
                Token::Ident => todo!(),
                _ => unreachable!()
            },
            (l_span, r_span) => {
                let l_idx = self.edges[l_span as usize..=r_span as usize][0];
                let r_idx = self.edges[l_span as usize..=r_span as usize][1];

                match (l_idx, r_idx){
                    (NONE, r_idx) => {
                        match tok{
                            Token::Sub => todo!(),
                            Token::LogicNot => todo!(),
                            _ => unreachable!()
                        }
                    },
                    (l_idx, r_idx) => {
                        match tok{
                            Token::Add | Token::Sub | Token::Mul | Token::Div => {
                                let l_state = self.get_expr_type_state(l_idx)?;
                                let r_state = self.get_expr_type_state(r_idx)?;
                                self.check_normal_op_type_issue(&l_state, &r_state, key)
                            },
                            Token::Grt| Token::GrtEq | Token::Les | Token::LesEq|
                            Token::EqEq | Token::NotEq => {
                                let l_state = self.get_expr_type_state(l_idx)?;
                                let r_state = self.get_expr_type_state(r_idx)?;
                                todo!()
                            },

                            Token::LogicAnd | Token::LogicOr => todo!(),
                            _ => unreachable!()
                        }
                    }

                }

            }
        }

    }

    pub fn check_normal_op_type_issue(
        &mut self,
        state_a : &TypeState,
        state_b : &TypeState,
        op_key : Key
    ) -> Result<TypeState, TypeIssue>{
        match(state_a, state_b) {
            (TypeState::Integer(l_key), TypeState::Integer(r_key)) =>{
                let l_type = self.parse_int_type(*l_key);
                let r_type = self.parse_int_type(*r_key);

                if weight(&l_type) >= (weight(&r_type)) {
                    Ok(TypeState::Integer(*l_key))
                }else{
                    Ok(TypeState::Integer(*r_key))
                }
            } ,
            (TypeState::Float(l_key), TypeState::Float(r_key)) => {
                let l_type = self.parse_flt_type(*l_key);
                let r_type = self.parse_flt_type(*r_key);
                if weight(&l_type) >= (weight(&r_type)) {
                    Ok(TypeState::Float(*l_key))
                }else{
                    Ok(TypeState::Float(*r_key))
                }
            },
            (TypeState::Integer(l_key), TypeState::Float(r_key)) => {
                let l_type = self.parse_int_type(*l_key);
                let r_type = self.parse_flt_type(*r_key);
                Err(self.error_difference_const_literal(*r_key, &l_type, &r_type))
            },
            (TypeState::Float(l_key), TypeState::Integer(r_key)) => {
                let l_type = self.parse_flt_type(*l_key);
                let r_type =  self.parse_int_type(*r_key);
                Err(self.error_difference_const_literal(*r_key, &l_type, &r_type))
            },
            (TypeState::Stable(ty), TypeState::Integer(r_key)) => {
                let l_ty = ty;
                let r_ty = &self.parse_int_type(*r_key);
                if !is_in_same_set(l_ty, r_ty){
                    return Err(self.error_difference_const_literal(*r_key, &l_ty, &r_ty));
                }

                if weight(l_ty) >= weight(r_ty){
                    Ok(TypeState::Stable(l_ty.clone()))
                }else{
                    panic!("literal out of range {:?} ", l_ty)
                }
            }
            (TypeState::Stable(ty), TypeState::Float(r_key)) => {
                let l_ty = ty;
                let r_ty = &self.parse_int_type(*r_key);
                if !is_in_same_set(l_ty, r_ty){
                    return Err(self.error_difference_const_literal(*r_key, &l_ty, &r_ty));
                }

                if weight(l_ty) >= weight(r_ty){
                    Ok(TypeState::Stable(l_ty.clone()))
                }else{
                    panic!("literal out of range {:?} ", l_ty)
                }
            },
            (TypeState::Integer(l_key), TypeState::Stable(ty)) => {
                let l_ty = &self.parse_int_type(*l_key);
                let r_ty = ty;

                if !is_in_same_set(l_ty, r_ty){
                    return Err(self.error_difference_const_literal(*l_key, &l_ty, &r_ty));
                }

                if weight(l_ty) >= weight(r_ty){
                    Ok(TypeState::Stable(l_ty.clone()))
                }else{
                    panic!("literal out of range {:?} ", l_ty)
                }
            },
            (TypeState::Float(l_key), TypeState::Stable(ty)) => {
                let l_ty = &self.parse_flt_type(*l_key);
                let r_ty = ty;

                if !is_in_same_set(l_ty, r_ty){
                    return Err(self.error_difference_const_literal(*l_key, &l_ty, &r_ty));
                }

                if weight(l_ty) >= weight(r_ty){
                    Ok(TypeState::Stable(l_ty.clone()))
                }else{
                    panic!("literal out of range {:?} ", l_ty)
                }
            }
            (TypeState::Stable(l_ty), TypeState::Stable(r_ty)) => {
                if !is_in_same_set(l_ty, r_ty){
                    return Err(self.error_difference_const_literal(op_key, &l_ty, &r_ty));
                }

                if weight(l_ty) >= weight(r_ty){
                    Ok(TypeState::Stable(l_ty.clone()))
                }else{
                    panic!("Mismatch Type {:?} ", l_ty)
                }
            }

        }



    }
    fn parse_int_type(&mut self, key : Key) -> Type{
        let interval = &self.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let int_val: u128 = match self.input[start..end].parse() {
            Ok(val) => val,
            Err(val) => {
                panic!("Expected Int but find {:?}", val)
            }
        };

        match int_val{
            0..=127 => Type::i8,
            0..=32767 => Type::i16,
            0..=2147483647 => Type::i32,
            0..=9223372036854775807 => Type::i64,
            0..=170141183460469231731687303715884105727 => Type::i128,
            _ => unreachable!()
        }
    }

    fn parse_flt_type(&mut self, key: Key) -> Type{
        let interval = &self.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let flt_val: f64 = match self.input[start..end].parse() {
            Ok(val) => val,
            _ => panic!("Parse Float Error")
        };



        match flt_val{
            0.0..=3.40282347e+38 => Type::f32,
            0.0..=1.7976931348623157e+308=> Type::f64,
            _ => unreachable!()
        }
    }

}

pub fn is_in_same_set(type_a : &Type, type_b : &Type) -> bool{
    let weight_a = weight(type_a);
    let weight_b = weight(type_b);
    if weight_a > 1 && weight_a < 10 && weight_b > 1 && weight_b < 10{
        true
    }else if weight_a > 10 && weight_a < 20 && weight_b > 10 && weight_b < 20{
        true
    }else if weight_a == 0 && weight_b == 0{
        true
    }else{
        false
    }
}






fn weight(ty : &Type) -> i32{
    match ty {
        Type::bool => 0,
        Type::i8   => 1,
        Type::i16  => 2,
        Type::i32  => 3,
        Type::i64  => 4,
        Type::i128 => 5,
        Type::f32  => 11,
        Type::f64  => 12,
        _ => unreachable!()
    }
}
#[derive(Debug, Clone, PartialEq)]
enum TypeState{
    Stable(Type),
    Integer(Key),
    Float(Key)
    //AplhaBool
    //OmagaBool
}
