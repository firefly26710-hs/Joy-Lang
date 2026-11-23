use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::{NONE, NodeIdx};
use crate::semantic::center::sema::Semantic;
use crate::semantic::symbol::data::ty::Type;
use crate::semantic::type_check::data::type_state::TypeState;
use crate::semantic::type_check::helper::{is_in_same_set, weight};

impl Semantic {
    pub fn type_check(&mut self, declared_type: &Type, expr_idx: NodeIdx, depth : u8) -> Type{
        let expr_type_state = self.get_expr_type_state(expr_idx, depth);
        if matches!(declared_type, Type::void){
            match expr_type_state{
                TypeState::Integer(key, is_neg) => {
                    match self.parse_int_type(key, is_neg){
                        Type::i8 | Type::i16 | Type::i32 => Type::i32,
                        Type::i64 => Type::i64,
                        Type::i128=> Type::i128,
                        _ => unreachable!()
                    }
                },
                TypeState::Float(key) => {
                    match self.parse_flt_type(key){
                        Type::f32 | Type::f64 => Type::f64,
                        _ => unreachable!()
                    }
                },
                TypeState::Bool(_) => {
                    Type::bool
                },
                TypeState::Stable(ty) => {
                    ty
                },
                TypeState::Phi(key) => {
                    Type::phi
                }
            }
        }else{
            match expr_type_state{
                TypeState::Integer(key, is_neg) => {
                    let expr_type = &self.parse_int_type(key, is_neg);
                    if !is_in_same_set(declared_type, expr_type){
                        self.error_mismatch_type_with_expression(expr_idx, declared_type, expr_type);
                        return declared_type.clone();
                    }

                    if weight(declared_type) <  weight(expr_type) {
                        self.error_mismatch_type_with_expression(expr_idx, declared_type, expr_type);
                    }

                    declared_type.clone()
                },
                TypeState::Float(key) => {
                    let expr_type = &self.parse_flt_type(key);
                    if !is_in_same_set(declared_type, expr_type){
                         self.error_mismatch_type_with_expression(expr_idx, declared_type, expr_type);
                        return declared_type.clone();
                    }

                    if weight(declared_type) < weight(expr_type) {
                        self.error_mismatch_type_with_expression(expr_idx, declared_type, expr_type);
                    }

                    declared_type.clone()
                },
                TypeState::Bool(key) => {
                    if declared_type != &Type::bool{
                        self.error_mismatch_type_with_expression(expr_idx, declared_type, &Type::bool);
                    }

                    declared_type.clone()

                },
                TypeState::Stable(expr_type) => {
                    if !is_in_same_set(declared_type, &expr_type){
                        panic!("Expected {:?}, find {:?}", declared_type, expr_type);
                        return declared_type.clone();
                    }

                    if weight(declared_type) < weight(&expr_type){
                        panic!("Expression out of range for {:?}", declared_type);
                    }

                    declared_type.clone()
                }
                TypeState::Phi(_) => {
                    Type::phi
                }
            }
        }
    }
    fn get_expr_type_state(&mut self, expr_idx : NodeIdx, var_depth : u8) -> TypeState{
        let node = &self.nodes[expr_idx as usize];
        let tok = &node.token;
        let key = node.key;
        let edge_span = &node.edge_span;
        let left_span = edge_span.start;
        let right_span = edge_span.end;

        match(left_span, right_span){
            (NONE, NONE) => match tok{
                Token::Int                  => TypeState::Integer(key, false),
                Token::Float                => TypeState::Float(key),
                Token::True | Token::False  => TypeState::Bool(key),
                Token::Ident                => self.look_up_var_type(key, var_depth),
                _ => unreachable!()
            },
            (l_span, r_span) => {
                let l_idx = self.edges[l_span as usize..=r_span as usize][0];
                let r_idx = self.edges[l_span as usize..=r_span as usize][1];

                match (l_idx, r_idx){
                    (NONE, r_idx) => {
                        match tok{
                            Token::Sub => {
                                let r_state = self.get_expr_type_state(r_idx, var_depth);
                                match r_state{
                                    TypeState::Integer(val_key, is_reg) => {
                                        match is_reg{
                                            true => TypeState::Integer(val_key, false),
                                            false=> TypeState::Integer(val_key, true)
                                        }
                                    },
                                    TypeState::Float(val_key) => {
                                        TypeState::Float(val_key)
                                    },
                                    TypeState::Bool(_) => {
                                        self.error_unary_mismatch_type(key, r_idx, Type::bool);
                                        TypeState::Phi(key)

                                    },
                                    TypeState::Stable(ty) => {
                                        todo!()
                                    },
                                    TypeState::Phi(_) => {
                                        TypeState::Phi(key)
                                    }
                                }
                            },
                            Token::LogicNot => {
                                let r_state = self.get_expr_type_state(r_idx, var_depth);
                                match r_state{
                                    TypeState::Integer(val_key, is_reg) => {
                                        let int_type = self.parse_int_type(val_key, is_reg);
                                        self.error_unary_mismatch_type(key, r_idx, int_type);
                                        TypeState::Phi(key)
                                    },
                                    TypeState::Float(val_key) => {
                                        let flt_type = self.parse_flt_type(val_key);
                                        self.error_unary_mismatch_type(key, r_idx, flt_type);
                                        TypeState::Phi(key)
                                    },
                                    TypeState::Bool(_) => {
                                        TypeState::Bool(key)
                                    },
                                    TypeState::Stable(ty) => {
                                        todo!()
                                    },
                                    TypeState::Phi(_) => {
                                        TypeState::Phi(key)
                                    }
                                }
                            }
                            _ => unreachable!()
                        }
                    },
                    (l_idx, r_idx) => {
                        match tok{
                            Token::Add | Token::Sub | Token::Mul | Token::Div => {

                                let l_state = self.get_expr_type_state(l_idx, var_depth);
                                let r_state = self.get_expr_type_state(r_idx, var_depth);
                                dbg!(&l_state);
                                dbg!(&r_state);
                                self.check_regular_op_matches(&l_state, &r_state, expr_idx, key)
                            },
                            Token::Grt| Token::GrtEq | Token::Les | Token::LesEq|
                            Token::EqEq | Token::NotEq => {
                                let l_state = self.get_expr_type_state(l_idx, var_depth);
                                let r_state = self.get_expr_type_state(r_idx, var_depth);
                                self.check_comparison_op_matches(&l_state, &r_state, expr_idx, key)
                            },
                            Token::LogicAnd | Token::LogicOr => {
                                let l_state = self.get_expr_type_state(l_idx, var_depth);
                                let r_state = self.get_expr_type_state(r_idx, var_depth);
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





