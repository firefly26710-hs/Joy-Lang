use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;
use crate::semantic::symbol::data::ty::Type;

impl Semantic{
    fn type_check(&mut self, declared_type : &Type){
        
    }
    fn get_expr_type(&mut self, expr_idx : NodeIdx) -> Type{
        let node = &self.nodes[expr_idx as usize];
        let tok = &node.token;
        let key = node.key;
        let edge_span = &node.edge_span;
        let left_span = edge_span.start;
        let right_span = edge_span.end;

        match(left_span, right_span){
            (NONE, NONE) => match tok{
                Token::Int => self.parse_int_type(key),
                Token::Float => self.parse_flt_type(key),
                Token::True | Token::False => Type::bool,
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
                                let l_type = self.get_expr_type(l_idx);
                                let r_type = self.get_expr_type(r_idx);
                                if !is_two_types_in_same_set(&l_type, &r_type){
                                    todo!()
                                }

                                if left_bigger_equal_than_right(&l_type, &r_type) {
                                    l_type
                                }else{
                                    r_type
                                }
                            },
                            Token::Grt| Token::GrtEq | Token::Les | Token::LesEq|
                            Token::EqEq | Token::NotEq => todo!(),

                            Token::LogicAnd | Token::LogicOr => todo!(),
                            _ => unreachable!()
                        }
                    }

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

pub fn is_two_types_in_same_set(type_a : &Type, type_b : &Type) -> bool{
    let a_num = weight(type_a);
    let b_num = weight(type_b);

    if a_num == i32::MAX || b_num == i32::MAX{
        return true;
    }

    //integer type set
    if a_num > 0 && a_num < 10 && b_num > 0 && b_num < 10 {
        true

    //float type set
    }else if a_num > 10 && a_num < 20 && b_num > 10 && b_num < 20{
        true
    }else if a_num == 0 && b_num == 0 {
        true
    }else{
        false
    }

}

pub fn left_bigger_equal_than_right(type_a : &Type, type_b : &Type) -> bool{

    let weight_a = weight(type_a);
    let weight_b = weight(type_b);

    if weight_a >= weight_b {
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
        Type::i128 => 5,
        Type::f32 => 11,
        Type::f64 => 12,
        Type::bool => 0,
        Type::void => i32::MAX,
    }
}