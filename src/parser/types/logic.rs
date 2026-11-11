use crate::lexer::tokenizer::source::token::Token;
use crate::llvm::expr::is_comparison_op;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;

impl Parser{
    pub fn matches_type(&mut self, expected_type : &Type, expr_idx : u32) -> Result<Type, ParserIssue>{
        self.stmt.already_has_comparison_op = false;

        let expr_type = &self.parse_expr_type(expr_idx)?;

        if matches!(expected_type, Type::void){

            if self.stmt.has_ident_in_expr{
                Ok(expr_type.clone())
            }else{
                match expr_type{
                    Type::i8 | Type::i16 | Type::i32 => Ok(Type::i32),
                    Type::i64 => Ok(Type::i64),
                    Type::i128 => Ok(Type::i128),
                    Type::bool => Ok(Type::bool),
                    Type::f32 | Type::f64 => Ok(Type::f64),
                    _ => panic!("[Line {:?}]Todo this type : {:?}", self.current_line(), expr_type)
                }
            }
        }else{
            if self.stmt.has_ident_in_expr{
                if left_bigger_than_right(&expected_type, &expr_type){
                    Err(self.mismatch_type_issue(self.stmt.expr_start_key, &expected_type, &expr_type))
                }else{
                    Ok(expr_type.clone())
                }
            }else{
                if left_bigger_than_right(&expected_type, expr_type) || expected_type == expr_type{
                    Ok(expected_type.clone())
                }else{
                    Err(self.type_out_of_range(self.stmt.expr_start_key, &expected_type))
                }
            }
        }

    }
    

    pub fn parse_expr_type(&mut self, expr_idx : u32) -> Result<Type, ParserIssue> {
        let node = &self.ast.nodes[expr_idx as usize];
        let tok = &node.token;
        let key = node.key;
        let edge_span = &node.edge_span;
        let left_span = edge_span.start;
        let right_span = edge_span.end;
        match(left_span, right_span){
            (NONE, NONE) => match tok{
                Token::Int => Ok(self.parse_int_type(key)),
                Token::Float => Ok(self.parse_flt_type(key)),
                Token::Ident => {
                    self.stmt.has_ident_in_expr = true;
                    Ok(self.parse_ident_type(key))
                },
                Token::True | Token::False => Ok(Type::bool),
                _ => panic!("unknown Leaf : {:?}", tok)
            }

            (left_span, right_span) => {
                let left_idx = self.ast.edges[left_span as usize..=right_span as usize][0];
                let right_idx = self.ast.edges[left_span as usize..=right_span as usize][1];

                match (left_idx, right_idx){
                    (NONE, rhs) => match tok{
                        Token::Sub => {
                            let expr_type = self.parse_expr_type(rhs)?;
                            if expr_type == Type::bool{
                                return Err(self.mismatch_expr_type_in_unary_issue(key, self.get_key(rhs), Type::bool));
                            }

                            Ok(expr_type)

                        }

                        Token::CmpNot  => {
                            let expr_type = self.parse_expr_type(rhs)?;
                            if expr_type != Type::bool{
                                return Err(self.mismatch_expr_type_in_unary_issue(key, self.get_key(rhs), Type::bool));
                            }

                            Ok(expr_type)
                        }
                        _ => unreachable!()
                    },
                    (lhs, rhs) =>{
                        let tok = tok.clone();

                        let l_ty = self.parse_expr_type(lhs)?;
                        let r_ty = self.parse_expr_type(rhs)?;

                        if is_comparison_op(&tok) && self.stmt.already_has_comparison_op{
                            return Err(self.comparison_op_chained_issue(key));
                        }
                        if is_comparison_op(&tok){
                            self.stmt.already_has_comparison_op = true;
                        }
                        
                        
                        
                        if !is_two_types_in_same_set(&r_ty, &l_ty){
                            return Err(self.mismatch_node_type_issue(&l_ty, &r_ty, self.get_key(rhs) as usize));
                        }

                        

                        if self.is_ident(lhs) && left_bigger_than_right(&r_ty, &l_ty){
                            Err(self.type_out_of_range(self.get_key(lhs), &l_ty))
                        }else if self.is_ident(rhs) && left_bigger_than_right(&l_ty, &r_ty){
                            Err(self.type_out_of_range(self.get_key(rhs), &r_ty))
                        }else{
                            if left_bigger_than_right(&l_ty, &r_ty){
                                if self.stmt.already_has_comparison_op{
                                    return Ok(Type::bool);
                                }
                                Ok(l_ty)
                            } else{
                                if self.stmt.already_has_comparison_op{
                                    return Ok(Type::bool);
                                }
                                Ok(r_ty)
                            }
                        }

                    },
                    _ => unreachable!()
                }


            }
        }
    }



    fn parse_int_type(&self, key : Key) -> Type{
        let interval = &self.lexer.tokenizer.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let int_val: i128 = match self.lexer.tokenizer.input[start..end].parse() {
            Ok(val) => val,
            Err(val) => {
                panic!("[Line {:?}] Expected Int but find : {:?}", self.current_line(), val)
            }
        };

        match int_val{
            -128..=127 => Type::i8,
            -32768..=32767 => Type::i16,
            -2147483648..=2147483647 => Type::i32,
            -9223372036854775808..9223372036854775807 => Type::i64,
            -170141183460469231731687303715884105728..=170141183460469231731687303715884105727 => Type::i128,
        }
    }

    fn parse_flt_type(&self, key: Key) -> Type{
        let interval = &self.lexer.tokenizer.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let flt_val: f64 = match self.lexer.tokenizer.input[start..end].parse() {
            Ok(val) => val,
            Err(val) => {
                panic!("[Line {:?}] Expected Int but find : {:?}", self.current_line(), val)
            }
        };

        match flt_val{
            -3.40282347e+38..=3.40282347e+38 => Type::f32,
            -1.7976931348623157e+308..=1.7976931348623157e+308=> Type::f64,
            _ => panic!(
                "[Line {:?}] Mismatch Float value {:?}", self.current_line(), flt_val
            )
        }
    }

    fn parse_ident_type(&self, key : Key) -> Type{
        let ident_ty = self.look_var_type(&key);

        match ident_ty{
            Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128
            => self.parse_int_type(key),
            Type::f32 | Type::f64 => self.parse_flt_type(key),
            Type::bool => Type::bool,
            _ => panic!("unknown ident type : {:?}", ident_ty)
        }
    }

    fn is_ident(&self ,idx : u32) -> bool{
        self.ast.nodes[idx as usize].token == Token::Ident
    }

    fn get_key(&self, idx : NodeIdx) -> Key{
        self.ast.nodes[idx as usize].key
    }

}

pub fn is_two_types_in_same_set(type_a : &Type, type_b : &Type) -> bool{
    let a_num = weight(type_a);
    let b_num = weight(type_b);

    if a_num == i32::MIN || b_num == i32::MIN{
        return true;
    }

    //integer type set
    if a_num > 0 && a_num < 10 && b_num > 0 && b_num < 10 {
        true

    //float type set
    }else if a_num > 10 && a_num < 20 && b_num > 10 && b_num < 20{
        true
    }else{
        false
    }

}

pub fn left_bigger_than_right(type_a : &Type, type_b : &Type) -> bool{
    let weight_a = weight(type_a);
    let weight_b = weight(type_b);

    if weight_a > weight_b {
        true
    }else{
        false
    }
}


fn weight(ty : &Type) -> i32{
    match ty {
        Type::i8 => 1,
        Type::i16 => 2,
        Type::i32 => 3,
        Type::i64 => 4,
        Type::i128 => 11,
        Type::f32 => 12,
        Type::bool => 0,
        Type::void => i32::MIN,
        _ => unreachable!()
    }
}





