use crate::lex::iterval::Interval;
use crate::par::parser::{Parser, NONE};
use crate::lex::token::Token;
use crate::par::node::Node;
use crate::par::ty::Type;

impl Parser{
    pub fn expr(&mut self, min_bp : f32) -> u32{
        let mut lhs_idx = match self.peek(){
            Token::Ident => {
                //Just a Literal
                if self.literal(self.peek_next()){
                    let ident_var_key = self.current as u32;
                    self.eat();
                    let same_key = self.find_same_var_key(ident_var_key);
                    match self.table.get_var_type(same_key){
                        Type::i32 | Type::i64 => self.int_leaf_sum += 1,
                        Type::bool            => self.bool_type_node_sum += 1,
                        Type::f32 | Type::f64 => self.flt_leaf_sum += 1,
                        _ => unreachable!()
                    };

                    self.nodes.push(
                        Node::new(Token::Ident, same_key, Interval::new(NONE, NONE))
                    );

                    (self.nodes.len() - 1) as u32

                    //Function call
                }else if self.is_function_call(self.peek_next()){
                    let idx =  self.func_call();
                    let func_node = self.nodes.last().unwrap();
                    let interval = &func_node.idxs;
                    let start = interval.start as usize;
                    let end = interval.end as usize;
                    let range = &self.edges[start..=end];
                    let name_node = &self.nodes[range[0] as usize];
                    let name_key = name_node.key;
                    match self.table.get_ret_type(name_key){
                        Type::i32 | Type::i64 => self.int_leaf_sum += 1,
                        Type::bool            => self.bool_type_node_sum += 1,
                        Type::f32 | Type::f64 => self.flt_leaf_sum += 1,
                        Type::void            => {},
                        _ => unreachable!()
                    };
                    idx

                }else{
                    panic!("[Line {:?}] Expected Identifier", self.line())
                }
            },
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
            Token::String => {
                panic!("[Line {:?}] Expression can't include string", self.line())
            },
            _ => panic!("[Line {:?}] Expected a expression", self.line())
        };


        loop{
            let op_tok = match self.peek(){
                Token::EOF | Token::Scolon |Token::Lbrace|
                Token::Rparen | Token::Comma | Token::Arrow |
                Token::Vartical| Token::Rbracket | Token::Rbrace => break,

                _true => self.peek()
            };




            let (l_bp, r_bp) = infix_binding_power(&op_tok);

            if l_bp < min_bp{
                break;
            }

            if self.is_regular_op(&op_tok){self.regular_op_sum += 1}
            else if self.is_compare_op(&op_tok){self.bool_type_node_sum += 1}
            else if self.is_connect_op(&op_tok){self.connect_op_sum += 1}

            let op_key = self.current as u32;
            let op_tok =  self.eat();

            let rhs_idx = self.expr(r_bp);

            let op_idxs = self.bt_push(lhs_idx, rhs_idx);

            self.nodes.push(Node::new(op_tok, op_key, op_idxs));

            lhs_idx = (self.nodes.len() - 1) as u32;
        }
        lhs_idx 
    }

    pub fn literal(&self ,peeker : &Token) -> bool{
        self.is_op(peeker) ||
            matches!(peeker, &Token::Scolon | &Token::Lbrace | &Token::Rbrace | &Token::Rparen)
    }

    pub fn is_function_call(&self, peeker : &Token) -> bool{
        matches!(self.peek_next(), &Token::Lparen | &Token::Comma | &Token::Rparen | &Token::Lbrace)
    }


    #[inline(always)]
    pub fn is_expr(&self, tok : &Token) -> bool{
        self.is_number(tok) || self.is_identifier(tok) || self.is_paren(tok)
        || self.peek() == &Token::Minus || self.peek() == &Token::CmpNot
    }

    pub fn is_paren(&self, tok : &Token) -> bool{
        match tok{
            Token::Lparen | Token::Rparen => true,
            _ => false
        }
    }

    pub fn is_brace(&self, tok : &Token) -> bool{
        match tok{
            Token::Lbrace | Token::Rbrace => true,
            _ => false
        }
    }

    #[inline(always)]
    pub fn is_identifier(&self, tok : &Token) -> bool{
        match tok{
            Token::Ident => true,
            _ => false
        }
    }

    #[inline(always)]
    pub fn is_number(&self, tok : &Token) -> bool{
        match tok{
            Token::Int | Token::Float => true,
            _ => false
        }
    }



    #[inline(always)]
    pub fn is_op(&self, tok : &Token) -> bool{
         match tok{
            Token::Minus | Token::CmpNot |
            Token::Plus  | Token::Mul| Token::Div |
            Token::EqEq   | Token::NotEq |
            Token::Grt    | Token::Les |
            Token::GrtEq  | Token::LesEq |
            Token::Mod => true,
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
        Token::GrtEq  | Token::LesEq     => (0.7, 0.8),
        Token::Plus   | Token::Minus     => (1.0, 2.0),
        Token::Div    | Token::Mod       => (3.0, 3.1),
        Token::Mul                       => (4.0, 4.0),
        _         => panic!("Unknown Operator : {:?}", op)
    }
}