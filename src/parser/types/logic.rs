use crate::lexer::diagnostic::source::issue::IssueType;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::node::Key;
use crate::parser::center::par::Parser;
use crate::parser::types::typ::Type;

impl Parser{

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

        if !is_in_same_set(&curr_type, &prev_type){
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


}

pub fn is_in_same_set(type_a : &Type, type_b : &Type) -> bool{
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
        Type::i128 => 5,
        Type::f32 => 11,
        Type::f64 => 12,
        Type::void => i32::MIN,
        _ => unreachable!()
    }
}





