use crate::lex::token::Token;
use crate::par::parser::Parser;
use crate::par::ty::Type;

impl Parser{
    pub fn expr_type(&self) -> Token{


        let is_only_one_type_leaf = self.int_leaf_sum & self.flt_leaf_sum == 0;
        let is_int = self.int_leaf_sum == self.regular_op_sum + 1 && is_only_one_type_leaf;
        let is_flt = self.flt_leaf_sum == self.regular_op_sum + 1 && is_only_one_type_leaf;
        let is_bool = self.compare_op_sum == self.connect_op_sum + 1 && is_only_one_type_leaf;

            if is_int{
            return Token::Int
        }else if is_flt{
            return Token::Float
        }else if is_bool{
            return Token::Bool
        };
        Token::EOF //unreachable
    }
    pub fn type_mapping(&self, declared_type_key: usize) -> Type{
        let interval = &self.str_intervals[declared_type_key];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let type_str = &self.input[start..end];

        match type_str {
            "i32" => Type::i32,
            "bool"=> Type::bool,
            "f32" => Type::f32,
            "i64" => Type::i64,
            "f64" => Type::f64,
            _ => unreachable!()
        }
    }
}