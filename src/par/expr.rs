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

                let idxs = self.bt_push(NONE, NONE);
                self.nodes.push(Node::new(Token::Int, key, idxs));
                (self.nodes.len() - 1) as u32
            },
            Token::Float => {
                self.flt_leaf_sum += 1;

                let key = self.current as u32;
                self.eat();

                let idxs = self.bt_push(NONE, NONE);
                self.nodes.push(Node::new(Token::Float, key, idxs));
                (self.nodes.len() - 1) as u32
            },
            _ => panic!("[Line {:?}] Expected a expression", self.lines[self.current ])
        };


        loop{
            let op_tok = match self.peek(){
                Token::EOF | Token::Scolon | Token::Rparen |
                Token::Comma | Token::Arrow | Token::Vartical|
                Token::Rbracket => break,

                _true => self.peek()
            };


            if self.regular_op_sum > 1 && self.connect_op_sum > 1 || self.compare_op_sum > 1{
                panic!("[Line {:?}] Cannot using {:?} on bool ",
                       self.lines[self.current ],
                       op_tok
                );
            }

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