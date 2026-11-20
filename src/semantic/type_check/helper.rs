use crate::parser::tree::node::Key;
use crate::semantic::symbol::data::ty::Type;
use crate::semantic::center::sema::Semantic;

impl Semantic{
    pub fn parse_neg_type(&mut self, key : Key) -> Type{
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
            0..=128 => Type::i8,
            0..=32768 => Type::i16,
            0..=2147483648 => Type::i32,
            0..=9223372036854775808 => Type::i64,
            0..=170141183460469231731687303715884105728 => Type::i128,
            _ => unreachable!()
        }
    }
    pub fn parse_int_type(&mut self, key : Key) -> Type{
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
    

    pub fn parse_flt_type(&mut self, key: Key) -> Type{
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
    if weight_a > 0 && weight_a < 10 && weight_b > 0 && weight_b < 10{
        true
    }else if weight_a > 10 && weight_a < 20 && weight_b > 10 && weight_b < 20{
        true
    }else if weight_a == 0 && weight_b == 0{
        true
    }else{
        false
    }
}






pub fn weight(ty : &Type) -> i32{
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
