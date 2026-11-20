use crate::parser::tree::ast::NodeIdx;
use crate::semantic::symbol::data::ty::Type;
use crate::parser::tree::node::Key;
use crate::semantic::diagnostic::data::issue::TypeIssue;
use crate::semantic::type_check::helper::{weight, is_in_same_set};
use crate::semantic::center::sema::Semantic;
use crate::semantic::type_check::type_state::TypeState;

impl Semantic {
    pub fn check_regular_op_matches(
        &mut self,
        state_a: &TypeState,
        state_b: &TypeState,
        expr_idx : NodeIdx,
        op_key: Key
    ) -> Result<TypeState, TypeIssue> {
        match (state_a, state_b) {
            (TypeState::Integer(l_key, l_is_neg), TypeState::Integer(r_key, r_is_neg)) => {
                let l_type = if *l_is_neg{self.parse_neg_type(*l_key)}
                else{self.parse_int_type(*l_key)};
                let r_type = if *l_is_neg{self.parse_neg_type(*r_key)}
                else{self.parse_int_type(*r_key)};
                if weight(&l_type) >= (weight(&r_type)) {
                    Ok(TypeState::Integer(*l_key, *l_is_neg))
                } else {
                    Ok(TypeState::Integer(*r_key, *r_is_neg))
                }
            },
            (TypeState::Integer(l_key, is_neg), TypeState::Float(r_key)) => {
                let l_type = if *is_neg{self.parse_neg_type(*l_key)}
                else{self.parse_int_type(*l_key)};
                let r_type = self.parse_flt_type(*r_key);
                Err(self.error_between_literal_and_literal(*r_key, &l_type, &r_type))
            },
            (TypeState::Integer(l_key, is_neg), TypeState::OmegaBool(r_key)) => {
                let l_type = if *is_neg{self.parse_int_type(*l_key)}
                else{self.parse_neg_type(*l_key)};
                let r_type = Type::bool;
                Err(self.error_between_op_literal(op_key, *r_key, l_type, r_type))
            },
            (TypeState::Integer(l_key, is_neg), TypeState::Stable(ty)) => {

                let l_ty = if *is_neg{&self.parse_neg_type(*l_key)}
                else{&self.parse_int_type(*l_key)};
                let r_ty = ty;

                if !is_in_same_set(l_ty, r_ty) {
                    return Err(self.error_between_literal_and_literal(*l_key, &l_ty, &r_ty));
                }

                if weight(l_ty) >= weight(r_ty) {
                    Ok(TypeState::Stable(l_ty.clone()))
                } else {
                    panic!("literal out of range {:?} ", l_ty)
                }
            },

            (TypeState::Float(l_key), TypeState::Float(r_key)) => {
                let l_type = self.parse_flt_type(*l_key);
                let r_type = self.parse_flt_type(*r_key);

                if weight(&l_type) >= (weight(&r_type)) {
                    Ok(TypeState::Float(*l_key))
                } else {
                    Ok(TypeState::Float(*r_key))
                }
            },
            (TypeState::Float(l_key), TypeState::Integer(r_key, is_neg)) => {
                let l_type = self.parse_flt_type(*l_key);
                let r_type = if *is_neg{self.parse_neg_type(*r_key)}
                else{self.parse_int_type(*r_key)};
                Err(self.error_between_literal_and_literal(*r_key, &l_type, &r_type))
            },
            (TypeState::Float(l_key), TypeState::OmegaBool(r_key)) => {
                let l_type = self.parse_flt_type(*l_key);
                let r_type = Type::bool;
                Err(self.error_between_op_literal(op_key, *r_key, l_type, r_type))
            },
            (TypeState::Float(l_key), TypeState::Stable(ty)) => {
                let l_ty = &self.parse_flt_type(*l_key);
                let r_ty = ty;

                if !is_in_same_set(l_ty, r_ty) {
                    return Err(self.error_between_literal_and_literal(*l_key, &l_ty, &r_ty));
                }

                if weight(l_ty) >= weight(r_ty) {
                    Ok(TypeState::Stable(l_ty.clone()))
                } else {
                    panic!("literal out of range {:?} ", l_ty)
                }
            },

            (TypeState::OmegaBool(l_key), TypeState::Integer(r_key, is_neg)) => {
                let l_type = Type::bool;
                let r_type = if *is_neg{self.parse_neg_type(*r_key)}
                else{self.parse_int_type(*r_key)};
                Err(self.error_between_op_literal(op_key, *l_key, r_type, l_type))
            },
            (TypeState::OmegaBool(l_key), TypeState::Float(r_key)) => {
                let l_type = Type::bool;
                let r_type = self.parse_flt_type(*r_key);
                Err(self.error_between_op_literal(op_key, *l_key, r_type, l_type))
            }
            (TypeState::OmegaBool(_), TypeState::OmegaBool(_)) => {
                let l_type = Type::bool;
                let r_type = Type::bool;
                Err(self.error_between_op_literal_literal(op_key, expr_idx, l_type, r_type ))
            }
            (TypeState::OmegaBool(_), TypeState::Stable(ty)) => {
                let l_ty = &Type::bool;
                let r_ty = ty;
                todo!()
            }


            (TypeState::Stable(l_ty), TypeState::Stable(r_ty)) => {
                if !is_in_same_set(l_ty, r_ty) {
                    return Err(self.error_between_literal_and_literal(op_key, &l_ty, &r_ty));
                }

                if weight(l_ty) >= weight(r_ty) {
                    Ok(TypeState::Stable(l_ty.clone()))
                } else {
                    panic!("Mismatch Type {:?} ", l_ty)
                }
            },
            (TypeState::Stable(ty), TypeState::Integer(r_key, is_neg)) => {

                let l_ty = ty;
                let r_ty = if *is_neg{&self.parse_neg_type(*r_key)}
                else{&self.parse_int_type(*r_key)};
                if !is_in_same_set(l_ty, r_ty) {
                    return Err(self.error_between_literal_and_literal(*r_key, &l_ty, &r_ty));
                }

                if weight(l_ty) >= weight(r_ty) {
                    Ok(TypeState::Stable(l_ty.clone()))
                } else {
                    panic!("literal out of range {:?} ", l_ty)
                }
            }
            (TypeState::Stable(ty), TypeState::Float(r_key)) => {
                let l_ty = ty;
                let r_ty = &self.parse_int_type(*r_key);
                if !is_in_same_set(l_ty, r_ty) {
                    return Err(self.error_between_literal_and_literal(*r_key, &l_ty, r_ty));
                }

                if weight(l_ty) >= weight(r_ty) {
                    Ok(TypeState::Stable(l_ty.clone()))
                } else {
                    panic!("literal out of range {:?} ", l_ty)
                }
            },
            (TypeState::Stable(l_ty), TypeState::OmegaBool(r_key)) => {
                todo!()
            }

        }
    }
}
