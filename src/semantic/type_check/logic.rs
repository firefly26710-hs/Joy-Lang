use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::semantic::center::sema::Semantic;
use crate::semantic::diagnostic::data::issue::TypeIssue;
use crate::semantic::symbol::data::ty::Type;
use crate::semantic::type_check::helper::{is_in_same_set, weight};
use crate::semantic::type_check::type_state::TypeState;

impl Semantic{
    pub fn type_check(&mut self, declared_type : &Type, expr_idx : NodeIdx) -> Result<Type, TypeIssue>{
        let expr_type_state = self.get_expr_type_state(expr_idx)?;
        if matches!(declared_type, Type::void){
            match expr_type_state{
                TypeState::Integer(key, _) => {
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
                TypeState::OmegaBool(_) => Ok(Type::bool),
                TypeState::Stable(ty) => {
                    Ok(ty)
                }
            }
        }else{
            match expr_type_state{
                TypeState::Integer(key, _) => {
                    let expr_type = &self.parse_int_type(key);
                    if !is_in_same_set(declared_type, expr_type){
                        return Err(self.error_mismatch_type_with_expression(expr_idx, declared_type, expr_type));
                    }

                    if weight(declared_type) >= weight(expr_type){
                        Ok(declared_type.clone())
                    }else{
                        Err(self.error_mismatch_type_with_expression(expr_idx, declared_type, expr_type))
                    }
                },
                TypeState::Float(key) => {
                    let expr_type = &self.parse_flt_type(key);
                    if !is_in_same_set(declared_type, expr_type){
                        return Err(self.error_mismatch_type_with_expression(expr_idx, declared_type, expr_type));
                    }

                    if weight(declared_type) >= weight(expr_type){
                        Ok(declared_type.clone())
                    }else{
                        Err(self.error_mismatch_type_with_expression(expr_idx, declared_type, expr_type))
                    }
                },
                TypeState::OmegaBool(key) => {
                    if declared_type == &Type::bool{
                        Ok(Type::bool)
                    }else{
                        Err(self.error_mismatch_type_with_expression(expr_idx, declared_type, &Type::bool))
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
                Token::Int                  => Ok(TypeState::Integer(key, false)),
                Token::Float                => Ok(TypeState::Float(key)),
                Token::True | Token::False  => Ok(TypeState::OmegaBool(key)),
                Token::Ident => todo!(),
                _ => unreachable!()
            },
            (l_span, r_span) => {
                let l_idx = self.edges[l_span as usize..=r_span as usize][0];
                let r_idx = self.edges[l_span as usize..=r_span as usize][1];

                match (l_idx, r_idx){
                    (NONE, r_idx) => {
                        match tok{
                            Token::Sub => {
                                let r_state = self.get_expr_type_state(r_idx)?;
                                match r_state{
                                    TypeState::Integer(val_key, is_reg) => {
                                        match is_reg{
                                            true => Ok(TypeState::Integer(val_key, false)),
                                            false=> Ok(TypeState::Integer(val_key, true))
                                        }
                                    },
                                    TypeState::Float(val_key) => {
                                        Ok(TypeState::Float(val_key))
                                    },
                                    TypeState::OmegaBool(_) => {
                                        Err(self.error_unary_mismatch_type(key, r_idx, Type::bool))
                                    },
                                    TypeState::Stable(ty) => {
                                        todo!()
                                    }
                                }
                            },
                            Token::LogicNot => {
                                let r_state = self.get_expr_type_state(r_idx)?;
                                match r_state{
                                    TypeState::Integer(val_key, is_reg) => {
                                        let int_type = if is_reg{self.parse_neg_type(val_key)}
                                                             else{self.parse_int_type(val_key)};
                                        Err(self.error_unary_mismatch_type(key, r_idx, int_type))
                                    },
                                    TypeState::Float(val_key) => {
                                        let flt_type = self.parse_flt_type(val_key);
                                        Err(self.error_unary_mismatch_type(key, r_idx, flt_type))
                                    },
                                    TypeState::OmegaBool(_) => {
                                        Ok(TypeState::OmegaBool(key))
                                    },
                                    TypeState::Stable(ty) => {
                                        todo!()
                                    }
                                }
                            },
                            _ => unreachable!()
                        }
                    },
                    (l_idx, r_idx) => {
                        match tok{
                            Token::Add | Token::Sub | Token::Mul | Token::Div => {
                                let l_state = self.get_expr_type_state(l_idx)?;
                                let r_state = self.get_expr_type_state(r_idx)?;
                                self.check_regular_op_matches(&l_state, &r_state, expr_idx, key)
                            },
                            Token::Grt| Token::GrtEq | Token::Les | Token::LesEq|
                            Token::EqEq | Token::NotEq => {
                                let l_state = self.get_expr_type_state(l_idx)?;
                                let r_state = self.get_expr_type_state(r_idx)?;
                                self.check_comparison_op_matches(&l_state, &r_state, expr_idx, key)
                            },
                            Token::LogicAnd | Token::LogicOr => {
                                let l_state = self.get_expr_type_state(l_idx)?;
                                let r_state = self.get_expr_type_state(r_idx)?;
                                self.check_binary_op_matches(&l_state, &r_state, expr_idx, key)
                            },
                            _ => unreachable!()
                        }
                    }

                }

            }
        }

    }

}





