use crate::lex::iterval::{End, Start};
use crate::lex::token::Token;
use crate::par::ast::node::{Key};
use crate::par::parser::{NodeIdx, Parser};
impl Parser{
    pub fn parse_expression(&mut self) -> NodeIdx{
        self.expr(0.0)
    }
    fn expr(&mut self, min_bp : f32) -> NodeIdx{
        let lhs_idx = match self.peek(){
            Token::Int =>{
                self.int_node_sum += 1;
                let key = self.current as Key;
                self.eat();
                self.push_leaf(Token::Int, key)
            }
            Token::Float =>{
                self.flt_node_sum += 1;
                let key = self.current as Key;
                self.eat();
                self.push_leaf(Token::Int, key)
            }
            Token::String => {
                self.is_string = true;
                let key = self.current as Key;
                self.eat();
                self.push_leaf(Token::Int, key)
            }
            Token::True =>{
                self.bool_node_sum += 1;
                let key = self.current as Key;
                self.eat();
                self.push_leaf(Token::True, key)
            }
            Token::False=> {
                self.bool_node_sum += 1;
                let key = self.current as Key;
                self.eat();
                self.push_leaf(Token::False, key)
            }
            Token::Ident =>{
                if self.is_func_call(self.peek_next()){}
                let key = self.find_same_ident_key(self.current as Key);
                self.push_leaf(Token::Ident, key)
            }
            _ => panic!("[Line {:?}] Unknown Expression", self.current_line())
        };

        loop{
            let op_tok = match self.peek(){
                Token::EOF | Token::Scolon |Token::Lbrace|
                Token::Rparen | Token::Comma | Token::Arrow |
                Token::Vertical| Token::Rbracket | Token::Rbrace => break,

                _true => self.peek()
            };

            let (l_bp, r_bp) = infix_binding_power(&op_tok);

            if l_bp < min_bp{
                break;
            }

            if is_regular_op(op_tok){self.regular_op_sum += 1}
            else if is_connect_op(op_tok){self.bool_node_sum += 1}
            else if is_compare_op(op_tok){self.compare_op_sum += 1}

            let op_key = self.current as Key;
            let op_tok = self.eat();

            let rhs_idx = self.expr(0.0);

            let start : Start = self.ast.childs.len() as Start;
            self.ast.expr_recorder.push(lhs_idx);
            self.ast.expr_recorder.push(rhs_idx);
            self.ast.childs.extend(self.ast.expr_recorder.drain(..));
            let end : End = (self.ast.childs.len() - 1) as End;

             _ =self.push_node(op_tok, op_key , start, end)

        }
        lhs_idx
    }



    fn is_func_call(&self, peek_next : &Token) -> bool{
        peek_next == &Token::Lparen
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

#[inline(always)]
pub fn is_regular_op(tok : &Token) -> bool{
    match tok{
        Token::Plus | Token::Minus | Token::Mul |
        Token::Div  | Token::Mod => {
            true
        }
        _ => false
    }
}

#[inline(always)]
pub fn is_compare_op(tok : &Token) -> bool{
    match tok{
        Token::Les | Token::LesEq |
        Token::Grt | Token::GrtEq |
        Token::NotEq | Token::EqEq => true,
        _ => false
    }
}


#[inline(always)]
pub fn is_connect_op(tok : &Token) -> bool{
    match tok{
        Token::CmpAnd | Token::CmpOr => true,
        _ => false
    }
}

