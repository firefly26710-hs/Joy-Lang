use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;
use crate::semantic::symbol::data::ty::Type;
use crate::semantic::type_check::data::type_state::TypeState;
use crate::semantic::type_check::helper::is_in_same_set;

impl Semantic {
    pub fn check_binary_op_matches(
        &mut self,
        state_a: &TypeState,
        state_b: &TypeState,
        expr_idx : NodeIdx,
        op_key: Key
    ) -> TypeState{
        match (state_a, state_b) {
            (TypeState::Integer(l_key,l_is_neg), TypeState::Integer(r_key, r_is_neg)) => {
                let l_type =  self.parse_int_type(*l_key, *l_is_neg);
                let r_type =  self.parse_int_type(*l_key, *r_is_neg);
                self.error_between_op_literal_literal(op_key, expr_idx, l_type, r_type);
                TypeState::Phi(op_key)
            },
            (TypeState::Integer(l_key, is_neg), TypeState::Float(r_key)) => {
                let l_type =  self.parse_int_type(*l_key, *is_neg);
                let r_type = self.parse_flt_type(*r_key);
                self.error_between_op_literal_literal(op_key, expr_idx, l_type, r_type);
                TypeState::Phi(op_key)
            },
            (TypeState::Integer(l_key, is_neg),  TypeState::Bool(_)) => {
                let l_type =  self.parse_int_type(*l_key, *is_neg);
                let r_type = Type::bool;
                self.error_mismatch_type_between_literal_literal(*l_key, &l_type, &r_type);
                TypeState::Phi(op_key)
            },
            (TypeState::Integer(l_key, is_neg), TypeState::Stable(ty)) => {
                todo!()
            },
            (TypeState::Integer(_, _), TypeState::Phi(key)) => {
                TypeState::Phi(*key)
            }
            (TypeState::Float(l_key), TypeState::Float(r_key)) => {
                let l_ty = self.parse_flt_type(*l_key);
                let r_ty = self.parse_flt_type(*r_key);
                self.error_between_op_literal_literal(op_key, expr_idx, l_ty, r_ty);
                TypeState::Phi(op_key)
            },
            (TypeState::Float(l_key), TypeState::Integer(r_key, is_neg)) => {
                let l_type = self.parse_flt_type(*l_key);
                let r_type = self.parse_int_type(*r_key, *is_neg);
                self.error_between_op_literal_literal(op_key, expr_idx, l_type, r_type);
                TypeState::Phi(op_key)
            },
            (TypeState::Float(l_key),  TypeState::Bool(_)) => {
                let l_type = self.parse_flt_type(*l_key);
                let r_type = Type::bool;
                self.error_mismatch_type_between_literal_literal(*l_key, &l_type, &r_type);
                TypeState::Phi(op_key)
            },
            (TypeState::Float(l_key), TypeState::Stable(ty)) => {
                todo!()
                // let l_ty = &self.parse_flt_type(*l_key);
                // let r_ty = ty;
                //
                // if !is_in_same_set(l_ty, r_ty) {
                //     return Err(self.error_between_literal_and_literal(*l_key, &l_ty, &r_ty));
                // }
                //
                // if weight(l_ty) >= weight(r_ty) {
                //     Ok(TypeState::Stable(l_ty.clone()))
                // } else {
                //     panic!("literal out of range {:?} ", l_ty)
                // }
            },
            (TypeState::Float(_), TypeState::Phi(key)) => {
                TypeState::Phi(*key)
            }
            (TypeState::Bool(_), TypeState::Integer(r_key, is_neg)) => {
                let l_type = Type::bool;
                let r_type =  self.parse_int_type(*r_key, *is_neg);
                self.error_mismatch_type_between_literal_literal(*r_key, &l_type, &r_type);
                TypeState::Bool(op_key)
            },
            (TypeState::Bool(_), TypeState::Float(r_key)) => {
                let l_type = Type::bool;
                let r_type = self.parse_flt_type(*r_key);
                self.error_mismatch_type_between_literal_literal(*r_key, &l_type, &r_type);
                TypeState::Phi(op_key)
            }
            (TypeState::Bool(_),  TypeState::Bool(_)) => {
                TypeState::Bool(op_key)
            }
            (TypeState::Bool(l_key), TypeState::Stable(r_type)) => {
                todo!()
            },
            (TypeState::Bool(_), TypeState::Phi(key)) => {
                TypeState::Phi(*key)
            }
            (TypeState::Stable(l_ty), TypeState::Stable(r_ty)) => {
                todo!()
                // if !is_in_same_set(l_ty, r_ty) {
                //     return Err(self.error_between_literal_and_literal(op_key, &l_ty, &r_ty));
                // }
                //
                // if weight(l_ty) >= weight(r_ty) {
                //     Ok(TypeState::Stable(l_ty.clone()))
                // } else {
                //     panic!("Mismatch Type {:?} ", l_ty)
                // }
            },
            (TypeState::Stable(ty), TypeState::Integer(r_key, is_neg)) => {
                let l_ty = ty;
                let r_ty = &self.parse_int_type(*r_key, *is_neg);
                if !is_in_same_set(l_ty, r_ty) {
                    self.error_mismatch_type_between_literal_literal(*r_key, &l_ty, &r_ty);
                    return TypeState::Phi(op_key);
                }

                TypeState::Bool(op_key)
            }
            (TypeState::Stable(ty), TypeState::Float(r_key)) => {
                let l_ty = ty;
                let r_ty = &self.parse_flt_type(*r_key);
                if !is_in_same_set(l_ty, r_ty) {
                    self.error_between_literal_and_literal(*r_key, &l_ty, r_ty);
                    return TypeState::Phi(op_key);
                }

                TypeState::Bool(op_key)
            },
            (TypeState::Stable(ty), TypeState::Bool(r_key)) => {
                todo!()
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