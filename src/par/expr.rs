use crate::lex::iterval::Interval;
use crate::par::parser::{Parser, NONE};
use crate::lex::token::Token;
use crate::par::node::Node;

impl Parser{
    pub fn expr(&mut self, min_bp : f32) -> u32{
        let mut lhs_idx = match self.peek(){
            Token::Int =>{
                self.int_leaf_sum += 1;

                let key = self.current as u32;
                self.eat();

                self.nodes.push(Node::new(Token::Int, key, Interval::new(NONE, NONE)));
                (self.nodes.len() - 1) as u32
            },
            Token::Float => {
                self.flt_leaf_sum += 1;

                let key = self.current as u32;
                self.eat();

                self.nodes.push(Node::new(Token::Float, key, Interval::new(NONE, NONE)));
                (self.nodes.len() - 1) as u32
            },
            _ => panic!("[Line {:?}] Expected a expression", self.line())
        };


        loop{
            let op_tok = match self.peek(){
                Token::EOF | Token::Scolon | Token::Rparen |
                Token::Comma | Token::Arrow | Token::Vartical|
                Token::Rbracket => break,

                _true => self.peek()
            };


            if self.regular_op_sum > 1 && self.connect_op_sum > 1 || self.compare_op_sum > 1{
                panic!("[Line {:?}] Cannot using {:?} on bool ",self.line(), op_tok);
            }

            if self.is_regular_op(&op_tok){self.regular_op_sum += 1}
            else if self.is_compare_op(&op_tok){self.compare_op_sum += 1}
            else if self.is_connect_op(&op_tok){self.connect_op_sum += 1}


            let (l_bp, r_bp) = infix_binding_power(&op_tok);

            if l_bp < min_bp{
                break;
            }

            let op_key = self.current as u32;
            self.eat();

            let rhs_idx = self.expr(r_bp);

            let op_idxs = self.bt_push(lhs_idx, rhs_idx);

            self.nodes.push(Node::new(op_tok, op_key, op_idxs));

            lhs_idx = (self.nodes.len() - 1) as u32;
        }
        lhs_idx 
    }

    pub fn is_expr(&self, tok : Token) -> bool{
        match tok{
            Token::Int | Token::Ident | Token::Float |
            Token::Minus | Token::CmpNot => {
                true
            },
            _ => false
        }
    }
}


#[inline(always)]
fn infix_binding_power(op : &Token) -> (f32, f32){
    match op{
        Token::CmpAnd | Token::CmpOr     => (0.1, 0.2),
        Token::EqEq   | Token::NotEq     => (0.3, 0.3),
        Token::Grt    | Token::Les       => (0.5, 0.6),
        Token::Plus   | Token::Minus     => (1.0, 2.0),
        Token::Div    | Token::Mod       => (3.0, 3.1),
        Token::Mul                       => (4.0, 4.0),
        _         => panic!("Unknown Operator : {:?}", op)
    }
}