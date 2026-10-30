use crate::lexer::diagnostic::source::issue::IssueType;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::node::Key;
use crate::parser::center::par::Parser;
use crate::parser::types::typ::Type;

impl Parser{

    pub fn is_matches_type(&self, expected_type: Type) -> bool{
        match (expected_type, self.get_expr_type()){
            (Type::i8, Type::i8) => true,
            (Type::i8, Type::i16) => {
                self.out_of_type_range(&Type::i8);
                false
            },
            (Type::i8, Type::i32) => {
                self.out_of_type_range(&Type::i8);
                false
            },
            (Type::i8, Type::i64) => {
                self.out_of_type_range(&Type::i8);
                false
            },
            (Type::i8, Type::i128) => {
                self.out_of_type_range(&Type::i8);
                false
            },
            (Type::i16, Type::i8) => true,
            (Type::i16, Type::i16) => true,
            (Type::i16, Type::i32) => {
                self.out_of_type_range(&Type::i16);
                false
            },
            (Type::i16, Type::i64) => {
                self.out_of_type_range(&Type::i16);
                false
            },
            (Type::i16, Type::i128) => {
                self.out_of_type_range(&Type::i16);
                false
            },

            (Type::i32, Type::i8) => true,
            (Type::i32, Type::i16) => true,
            (Type::i32, Type::i32) => true,
            (Type::i32, Type::i64) => {
                self.out_of_type_range(&Type::i32);
                false
            },
            (Type::i32, Type::i128) => {
                self.out_of_type_range(&Type::i32);
                false
            },

            (Type::i64, Type::i8) => true,
            (Type::i64, Type::i16) => true,
            (Type::i64, Type::i32) => true,
            (Type::i64, Type::i64) => true,
            (Type::i64, Type::i128) => {
                self.out_of_type_range(&Type::i32);
                false
            },

            (Type::i128, Type::i8) => true,
            (Type::i128, Type::i16) => true,
            (Type::i128, Type::i32) => true,
            (Type::i128, Type::i64) => true,
            (Type::i128, Type::i128) => true,

            (Type::f32, Type::f32) => true,
            (Type::f32, Type::f64) => {
                self.out_of_type_range(&Type::i32);
                false
            },

            (Type::f64, Type::f32) => true,
            (Type::f64, Type::f64) => true,

            (Type::bool, Type::bool) => true,
            (Type::String, Type::String) => true,

            (ty1,ty2) if is_int_type(&ty1) &&
                is_flt_type(&ty2) || is_bool_type(&ty2) || is_string_type(&ty2) => {
                self.expected(&ty1, ty2);
                false
            },

            (ty1,ty2) if is_flt_type(&ty1) &&
                is_int_type(&ty2) || is_bool_type(&ty2) || is_string_type(&ty2) => {
                self.expected(&ty1, ty2);
                false
            },

            (ty1,ty2) if is_bool_type(&ty1) &&
                (is_int_type(&ty2) || is_flt_type(&ty2) || is_string_type(&ty2)) => {
                self.expected(&ty1, ty2);
                false
            },

            (ty1,ty2) if is_string_type(&ty1) &&
                (is_flt_type(&ty2) || is_flt_type(&ty2) || is_bool_type(&ty2)) => {
                self.expected(&ty1, ty2);
                false
            },

            _ => panic!("[Line {:?}] Unknown Type", self.current_line())

        }
    }

    pub fn get_expr_type(&self) -> Type{
        self.stmt.prev_node_type.clone()
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



    pub fn update_node_type(&mut self, key : Key) -> Result<(), IssueType>{
        let prev_type = self.stmt.prev_node_type.clone();

        let node_type = &self.lexer.tokenizer.tokens[key as usize];
        let curr_type = if matches!(node_type, Token::Int){
            self.parse_int_type(key)
        }else if matches!(node_type, Token::Float){
            self.parse_flt_type(key)
        }else if matches!(node_type, Token::Ident){
            self.look_var_type(&key).clone()
        }else{
            panic!("[Line {:?}] To do this node type case in error msg!!! : {:?}", self.current_line(), node_type)
        };

        if !is_two_types_in_same_set(&curr_type, &prev_type){
            return Err(self.mismatch_node_type_issue(&curr_type, &prev_type, self.stmt.prev_node_key as usize))
        }

        if self.stmt.is_prev_node_is_ident && left_bigger_than_right(&curr_type, &prev_type){
            return Err(self.type_out_of_range(self.current as Key, &prev_type))
        }else{
            if left_bigger_than_right(&curr_type, &prev_type){
                self.stmt.prev_node_type = curr_type
            }else if !left_bigger_than_right(&curr_type, &prev_type){
                self.stmt.prev_node_type = prev_type;
            }
        }

        Ok(())
    }


    fn expected(&self, expected : &Type, found_ty : Type) -> Type{
        panic!("[Line {:?}] Expected {:?}, but found {:?}", self.current_line(), expected, found_ty)
    }

     fn out_of_type_range(&self, declared_type: &Type) -> Type{
         panic!("[Line {:?}] Out of Type Range {:?}", self.current_line(), declared_type)
     }

    pub fn matches_to_int(&self) -> bool{
        self.stmt.int_node_sum == self.stmt.regular_op_sum + 1
            && self.stmt.int_node_sum & self.stmt.flt_node_sum == 0
            && self.stmt.bool_node_sum == 0
            && self.stmt.compare_op_sum == 0
            && !self.stmt.is_string
    }
    
    pub fn matches_to_flt(&self) -> bool{
        self.stmt.flt_node_sum == self.stmt.regular_op_sum + 1
            && self.stmt.int_node_sum & self.stmt.flt_node_sum == 0
            && self.stmt.bool_node_sum == 0
            && self.stmt.compare_op_sum == 0
            && !self.stmt.is_string
    }

    pub fn matches_to_bool(&self) -> bool{
        self.stmt.bool_node_sum == self.stmt.compare_op_sum + 1
            && self.stmt.int_node_sum & self.stmt.flt_node_sum == 0
            && !self.stmt.is_string
    }


    pub fn matches_to_string(&self) -> bool{
        self.stmt.compare_op_sum == 0
            && self.stmt.bool_node_sum == 0
            && self.stmt.int_node_sum == 0
            && self.stmt.flt_node_sum == 0
            && self.stmt.regular_op_sum == 0
            && self.stmt.is_string
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
        Type::void => i32::MIN,
        _ => unreachable!()
    }
}


pub fn is_int_type(ty : &Type) -> bool{
    matches!(ty, Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128)
}

pub fn is_flt_type(ty : &Type) -> bool{
    matches!(ty, Type::f32 | Type::f64)
}

pub fn is_bool_type(ty : &Type) -> bool{
    matches!(ty, Type::bool)
}

pub fn is_string_type(ty : &Type) -> bool{
    matches!(ty, Type::String)
}




