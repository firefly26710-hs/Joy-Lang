use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;
use crate::semantic::symbol::data::ty::Type;
use crate::semantic::type_check::data::type_state::TypeState;
use crate::semantic::type_check::helper::{is_in_same_set, weight};

impl Semantic {
    pub fn check_regular_op_matches(
        &mut self,
        state_a: &TypeState,
        state_b: &TypeState,
        expr_idx : NodeIdx,
        op_key: Key
    ) -> TypeState {
        match (state_a, state_b) {
            (TypeState::Integer(l_key, l_is_neg), TypeState::Integer(r_key, r_is_neg)) => {
                let l_type = self.parse_int_type(*l_key, *l_is_neg);
                let r_type =  self.parse_int_type(*r_key, *r_is_neg);
                if weight(&l_type) >= (weight(&r_type)) {
                    TypeState::Integer(*l_key, *l_is_neg)
                } else {
                    TypeState::Integer(*r_key, *r_is_neg)
                }
            },
            (TypeState::Integer(l_key, is_neg), TypeState::Float(r_key)) => {
                let l_type =  self.parse_int_type(*l_key, *is_neg);
                let r_type = self.parse_flt_type(*r_key);
                self.error_between_op_literal(op_key, *r_key, l_type, r_type);
                TypeState::Phi(op_key)
            },
            (TypeState::Integer(l_key, is_neg), TypeState::Bool(r_key)) => {
                let l_type = self.parse_int_type(*l_key, *is_neg);
                let r_type = Type::bool;
                self.error_between_op_literal(op_key, *r_key, l_type, r_type);
                TypeState::Phi(op_key)
            },
            (TypeState::Integer(l_key, is_neg), TypeState::Stable(ty)) => {
                let l_ty = &self.parse_int_type(*l_key, *is_neg);
                let r_ty = ty;

                if !is_in_same_set(l_ty, r_ty) {
                    self.error_between_identifier_literal(*l_key, &l_ty, &r_ty);
                    return TypeState::Phi(op_key);
                }

                if weight(l_ty) > weight(r_ty) {
                    panic!("literal out of range {:?} ", l_ty)
                }

                TypeState::Stable(l_ty.clone())
            },
            (TypeState::Integer(_, _), TypeState::Phi(key)) => {
                TypeState::Phi(*key)
            }

            (TypeState::Float(l_key), TypeState::Float(r_key)) => {
                let l_type = self.parse_flt_type(*l_key);
                let r_type = self.parse_flt_type(*r_key);

                if weight(&l_type) >= (weight(&r_type)) {
                    TypeState::Float(*l_key)
                } else {
                    TypeState::Float(*r_key)
                }
            },
            (TypeState::Float(l_key), TypeState::Integer(r_key, is_neg)) => {
                let l_type = self.parse_flt_type(*l_key);
                let r_type = self.parse_int_type(*r_key, *is_neg);
                self.error_between_op_literal(op_key, *r_key, l_type, r_type);
                TypeState::Phi(op_key)
            },
            (TypeState::Float(l_key), TypeState::Bool(r_key)) => {
                let l_type = self.parse_flt_type(*l_key);
                let r_type = Type::bool;
                self.error_between_op_literal(op_key, *r_key, l_type, r_type);
                TypeState::Phi(op_key)
            },
            (TypeState::Float(l_key), TypeState::Stable(ty)) => {
                let l_ty = &self.parse_flt_type(*l_key);
                let r_ty = ty;

                if !is_in_same_set(l_ty, r_ty) {
                    self.error_between_identifier_literal(op_key, &l_ty, &r_ty);
                    return TypeState::Phi(op_key);
                }

                if weight(l_ty) > weight(r_ty) {
                    panic!("literal out of range {:?} ", l_ty)
                }

                TypeState::Stable(l_ty.clone())
            },
            (TypeState::Float(_), TypeState::Phi(key)) => {
                TypeState::Phi(*key)
            }

            (TypeState::Bool(l_key), TypeState::Integer(r_key, is_neg)) => {
                let l_type = Type::bool;
                let r_type = self.parse_int_type(*r_key, *is_neg);
                self.error_between_op_literal(op_key, *l_key, r_type, l_type);
                TypeState::Phi(op_key)
            },
            (TypeState::Bool(l_key), TypeState::Float(r_key)) => {
                let l_type = Type::bool;
                let r_type = self.parse_flt_type(*r_key);
                self.error_between_op_literal(op_key, *l_key, r_type, l_type);
                TypeState::Phi(op_key)
            }
            (TypeState::Bool(_), TypeState::Bool(_)) => {
                let l_type = Type::bool;
                let r_type = Type::bool;
                self.error_between_op_literal_literal(op_key, expr_idx, l_type, r_type );
                TypeState::Phi(op_key)
            }
            (TypeState::Bool(key), TypeState::Stable(ty)) => {
                let l_ty = &Type::bool;
                let r_ty = ty;
                if is_in_same_set(l_ty, r_ty){
                    self.error_between_op_literal_literal(op_key, expr_idx, l_ty.clone(), r_ty.clone());
                    return TypeState::Phi(op_key);
                }

                self.error_between_op_literal(op_key, *key, r_ty.clone(), l_ty.clone());
                TypeState::Phi(op_key)


            },
            (TypeState::Bool(_),TypeState::Phi(key)) => {
                TypeState::Phi(*key)
            }
            (TypeState::Stable(l_ty), TypeState::Stable(r_ty)) => {
                if !is_in_same_set(l_ty, r_ty) {
                    self.error_between_identifier_literal(op_key, &l_ty, &r_ty);
                    return TypeState::Phi(op_key);
                }

                if weight(l_ty) < weight(r_ty) {
                    panic!("Mismatch Type {:?} ", l_ty)
                }

                TypeState::Stable(l_ty.clone())
            },
            (TypeState::Stable(ty), TypeState::Integer(r_key, is_neg)) => {

                let l_ty = ty;
               
                let r_ty = &self.parse_int_type(*r_key, *is_neg);
                if !is_in_same_set(l_ty, r_ty) {
                    self.error_between_identifier_literal(op_key, &l_ty, &r_ty);
                    return TypeState::Phi(op_key);
                }

                if weight(l_ty) < weight(r_ty) {
                    panic!("literal out of range {:?} ", l_ty)
                }
                TypeState::Stable(l_ty.clone())
            }
            (TypeState::Stable(ty), TypeState::Float(r_key)) => {
                let l_ty = ty;
                let r_ty = &self.parse_flt_type(*r_key);
                if !is_in_same_set(l_ty, r_ty) {
                    self.error_between_identifier_literal(op_key, &l_ty, &r_ty);
                    return TypeState::Phi(op_key)
                }

                if weight(l_ty) < weight(r_ty) {
                    panic!("literal out of range {:?} ", l_ty);
                }

                TypeState::Stable(l_ty.clone())
            },
            (TypeState::Stable(l_ty), TypeState::Bool(r_key)) => {
                let r_ty = &Type::bool;
                if is_in_same_set(l_ty, r_ty) {
                    self.error_between_op_literal_literal(op_key, expr_idx, l_ty.clone(), r_ty.clone());
                    return TypeState::Phi(op_key);
                }

                self.error_between_op_literal(op_key, *r_key, l_ty.clone(), r_ty.clone());
                TypeState::Phi(op_key)



            },
            (TypeState::Stable(_), TypeState::Phi(key)) => {
                TypeState::Phi(*key)
            }
            (TypeState::Phi(key),TypeState::Integer(_, _)) => {
                TypeState::Phi(*key)
            },
            (TypeState::Phi(key),TypeState::Float(_)) => {
                TypeState::Phi(*key)
            },
            (TypeState::Phi(key),TypeState::Bool(_)) => {
                TypeState::Phi(*key)
            },
            (TypeState::Phi(key), TypeState::Stable(_)) => {
                TypeState::Phi(*key)
            },
            (TypeState::Phi(key),TypeState::Phi(_)) => {
                TypeState::Phi(*key)
            }
        }
    }
}
