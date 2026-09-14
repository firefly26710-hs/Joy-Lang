use crate::lex::token::Token;
use crate::par::node::Node;
use crate::par::parser::{Parser, NONE};

impl Parser{
    pub fn expr(&mut self, min_bp : f32) -> u32{
        let mut lhs_idx = match self.eat(){
            Token::Int   =>{
                let l_edge_idx = self.edges.len() as u32;

                self.edges.push(NONE);
                self.edges.push(NONE);

                let r_edge_idx = (self.edges.len() - 1) as u32;
                let node = Node::init(Token::Int, self.current - 1, l_edge_idx, r_edge_idx);
                self.nodes.push(node);
                self.nodes.len() - 1
            },
            Token::Float =>{
                let l_edge_idx = self.edges.len() as u32;

                self.edges.push(NONE);
                self.edges.push(NONE);

                let r_edge_idx = (self.edges.len() - 1) as u32;
                let node = Node::init(Token::Float, self.current - 1, l_edge_idx, r_edge_idx);
                self.nodes.push(node);
                self.nodes.len() - 1
            },

	    // todo! : var_scope_stack same pk algorithm and the waiting algorithm
            Token::Ident =>{
                let l_edge_idx = self.edges.len() as u32;

                self.edges.push(NONE);
                self.edges.push(NONE);

                let r_edge_idx = (self.edges.len() - 1) as u32;
                let node = Node::init(Token::Int, self.current - 1, l_edge_idx, r_edge_idx);
                self.nodes.push(node);
                self.nodes.len() - 1
            }
            Token::Minus => {
                let unary_key = self.current - 1;

                let expr_idx = self.expr(1000.0);
                self.edges.push(NONE);
                let l_edge_idx = (self.edges.len() - 1) as u32;
                self.edges.push(expr_idx);
                let r_edge_idx = (self.edges.len() - 1) as u32;

                let node = Node::init(Token::Minus, unary_key, l_edge_idx, r_edge_idx);
                self.nodes.push(node);
                self.nodes.len() - 1
            },
            Token::Lparen => {
                let lhs_idx = self.expr(0.0);
                match self.peek(){
                    Token::Rparen => self.eat(),
                    _ => panic!("[Line {}] Expected  ')' ", self.lines[self.current as usize])
                };
                lhs_idx as usize
            },
            Token::CmpNot => {
                let cmp_not_key = self.current - 1;

                let expr_idx = match self.peek(){
                    Token::Lparen =>{
                        let idx = self.expr(1000.0);
                        idx as u32
                    },
                    _ => {
                        panic!(
                            "[Line {}] 'not' must combine with '(' ",
                            self.lines[self.current as usize]
                        )
                    }
                };

                self.edges.push(NONE);
                let l_edge_idx = (self.edges.len() - 1) as u32;
                self.edges.push(expr_idx);
                let r_edge_idx = (self.edges.len() - 1) as u32;
                let node = Node::init(Token::CmpNot, cmp_not_key, l_edge_idx, r_edge_idx);

                self.nodes.push(node);
                self.nodes.len() - 1
            }
            _ => panic!("[Line {:?}] Expected a expression", self.lines[self.current as usize])
        };

        loop{
            let op_token = match self.peek(){
                Token::EOF | Token::Scolon | Token::Rparen |
                Token::Comma | Token::Arrow | Token::Vartical|
                Token::Rbracket => break,

                _true => self.peek().clone()
            };

            let (l_bp, r_bp) = infix_binding_power(&op_token);

            if l_bp < min_bp{
                break;
            }

            let op_key = self.current;
            self.eat();

            let rhs_idx = self.expr(r_bp);

            let l_edge_idx = self.edges.len() as u32;
            self.edges.push(lhs_idx as u32);

            self.edges.push(rhs_idx);

            let r_edge_idx = (self.edges.len() - 1) as u32;


            self.nodes.push(
                Node::init(
                    op_token,
                    op_key,
                    l_edge_idx,
                    r_edge_idx
                )
            );

            lhs_idx = self.nodes.len() - 1;
        }

        lhs_idx as u32

    }

}

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
