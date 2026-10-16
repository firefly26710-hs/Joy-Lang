use crate::lex::iterval::Interval;
use crate::lex::token::Token;
use crate::par::ast::node::Node;
use crate::par::ast::ast::NONE;
use crate::par::ast::ast::AST;
use crate::par::parser::NodeIdx;

impl AST{
    pub fn expr(&mut self, min_bp : f32) -> NodeIdx{
        
    }

    #[inline(always)]
    fn infix_binding_power(op : &Token) -> (f32, f32){
        match op{
            Token::CmpAnd | Token::CmpOr     => (0.1, 0.2),
            Token::EqEq   | Token::NotEq     => (0.3, 0.3),
            Token::Grt    | Token::Les       => (0.5, 0.6),
            Token::GrtEq  | Token::LesEq     => (0.7, 0.8),
            Token::Plus   | Token::Minus     => (1.0, 2.0),
            Token::Div    | Token::Mod       => (3.0, 3.1),
            Token::Mul                       => (4.0, 4.0),
            _         => panic!("Unknown Operator : {:?}", op)
        }
    }
}