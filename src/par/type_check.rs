use crate::lex::token::Token;
use crate::par::parser::Parser;
use crate::par::ty::Type;

impl Parser{
    #[inline(always)]
    pub fn type_check(&self, declared_type: &Type, expr_type : Token) -> Type{

        match(&declared_type, &expr_type){
            (Type::void, Token::Int)   => Type::i32,
            (Type::void, Token::Bool)  => Type::bool,
            (Type::void, Token::Float) => Type::f64,
            (Type::i64,  Token::Int)   => Type::i64,
            (Type::f32,  Token::Float) => Type::f32,
            (Type::i32,  Token::Int)   => Type::i32,
            (Type::bool, Token::Bool)  => Type::bool,
            (Type::f64,  Token::Float) => Type::f64,
            _ => {
                panic!("[Line {:?}]Declared {:?}, but find {:?}", self.line(), declared_type, expr_type)
            }
        }
    }

    #[inline(always)]
    pub fn get_expr_token_type(&mut self) -> Token{
        let is_only_one_type_leaf = self.int_leaf_sum & self.flt_leaf_sum == 0;
        let is_int = self.int_leaf_sum == self.regular_op_sum + 1
                && is_only_one_type_leaf
                && self.bool_type_node_sum == 0
                && self.connect_op_sum == 0;
        
        let is_flt = self.flt_leaf_sum == self.regular_op_sum + 1
            && is_only_one_type_leaf
            && self.bool_type_node_sum == 0
            && self.connect_op_sum == 0;
        
        let is_bool = self.bool_type_node_sum == self.connect_op_sum + 1
            && is_only_one_type_leaf;

        let expr_type = if is_int{
            Token::Int
        }else if is_flt{
            Token::Float
        }else if is_bool{
            Token::Bool
        }else{
            panic!("[Line {:?}] Unknown Type", self.line());
        };

        self.int_leaf_sum = 0;
        self.flt_leaf_sum = 0;
        self.regular_op_sum = 0;

        self.bool_type_node_sum = 0;
        self.connect_op_sum = 0;

        expr_type
    }


    #[inline(always)]
    pub fn type_mapping(&self, declared_type_key: usize) -> Type{
        let interval = &self.lexer.intervals[declared_type_key];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let type_str = &self.lexer.input[start..end];

        match type_str {
            "i32" => Type::i32,
            "bool"=> Type::bool,
            "f32" => Type::f32,
            "i64" => Type::i64,
            "f64" => Type::f64,
            _ => unreachable!()
        }
    }

    pub fn is_int(&self, expr_type : Token  )->bool{
        let match_ty = self.type_check(&Type::void, expr_type);
        match_ty == Type::i32
    }
    
    pub fn is_bool(&self, expr_type : Token) -> bool{
        let match_ty = self.type_check(&Type::void, expr_type);
        match_ty == Type::bool
    }



    #[inline]
    pub fn is_regular_op(&self ,tok : &Token) -> bool{
        match tok{
            Token::Plus | Token::Minus | Token::Mul |
            Token::Div  | Token::Mod => {
                true
            }
            _ => false
        }
    }

    #[inline(always)]
    pub fn is_compare_op(&self, tok : &Token) -> bool{
        match tok{
            Token::Les | Token::LesEq | 
            Token::Grt | Token::GrtEq |
            Token::NotEq | Token::EqEq => true,
            _ => false
        }
    }


    #[inline(always)]
    pub fn is_connect_op(&self, tok : &Token) -> bool{
        match tok{
            Token::CmpAnd | Token::CmpOr => true,
            _ => false
        }
    }
    
    
}