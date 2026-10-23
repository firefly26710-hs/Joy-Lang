use crate::lexer::diagnostic::source::issue::IssueType;
use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;

impl Parser{
    pub fn parse_expression(&mut self) -> Result<NodeIdx, IssueType>{
        self.stmt.max_value.clear();
        self.stmt.max_value.push_str("0");

        self.stmt.int_node_sum = 0;
        self.stmt.flt_node_sum = 0;
        self.stmt.bool_node_sum = 0;
        self.stmt.regular_op_sum = 0;
        self.stmt.compare_op_sum = 0;
        self.stmt.is_string = false;

        self.expr(0.0)
    }
    fn expr(&mut self, min_bp : f32) -> Result<NodeIdx, IssueType>{
        let mut lhs_idx = match self.peek(){
            Token::Int =>{
                if self.stmt.flt_node_sum > 0{
                    return Err(self.collect_mismatch_node_type_issue(&Token::Float, &Token::Int))
                }

                self.stmt.int_node_sum += 1;
                let key = self.current as Key;
                self.update_int_max(key);
                self.eat();
                self.push_leaf(Token::Int, key)
            }
            Token::Float =>{
                if self.stmt.int_node_sum > 0{
                    return Err(self.collect_mismatch_node_type_issue(&Token::Int, &Token::Float))
                }
                self.stmt.flt_node_sum += 1;
                let key = self.current as Key;
                self.update_flt_max(key);
                self.eat();
                self.push_leaf(Token::Float, key)
            }
            Token::String => {
                if self.stmt.int_node_sum > 0{
                    return Err(self.collect_mismatch_node_type_issue(&Token::Int, &Token::String))
                }else if self.stmt.flt_node_sum > 0{
                    return Err(self.collect_mismatch_node_type_issue(&Token::Float, &Token::String))
                }

                self.stmt.is_string = true;
                let key = self.current as Key;
                self.eat();
                self.push_leaf(Token::String, key)
            }
            Token::True =>{
                self.stmt.bool_node_sum += 1;
                let key = self.current as Key;
                self.eat();
                self.push_leaf(Token::True, key)
            }
            Token::False=> {
                self.stmt.bool_node_sum += 1;
                let key = self.current as Key;
                self.eat();
                self.push_leaf(Token::False, key)
            }
            Token::Ident =>{
                if self.is_func_call(self.peek_next()){
                    self.expr_func_call()
                }else {
                    let ident_key = self.find_same_ident_key(self.current as Key);

                    self.var_not_in_scope(ident_key)?;
                    self.eat();
                    let ident_type = self.look_var_type(&ident_key);
                    self.matches_type_in_multi_var(&ident_type.clone())?;
                    match self.look_var_type(&ident_key) {
                        Type::i8 | Type::i16 | Type::i32 |
                        Type::i64 | Type::i128 => {
                            if self.stmt.flt_node_sum > 0{
                                return Err(self.collect_mismatch_node_type_issue(&Token::Float, &Token::Int))
                            }

                            self.stmt.int_node_sum += 1
                        },
                        Type::f32 | Type::f64 => {
                            if self.stmt.int_node_sum > 0{
                                return Err(self.collect_mismatch_node_type_issue(&Token::Int, &Token::Float))
                            }

                            self.stmt.flt_node_sum += 1
                        },
                        Type::bool => self.stmt.bool_node_sum += 1,
                        Type::String => {
                            if self.stmt.int_node_sum > 0{
                                return Err(self.collect_mismatch_node_type_issue(&Token::Int, &Token::String))
                            }else if self.stmt.flt_node_sum > 0{
                                return Err(self.collect_mismatch_node_type_issue(&Token::Float, &Token::String))
                            }
                            self.stmt.is_string = true

                        }
                        Type::void => panic!("[Line {:?}] Unknown Type ", self.current_line())
                    };

                    self.push_leaf(Token::Ident, ident_key)
                }
            }
            _ => panic!("[Line {:?}] Unknown Expression : {:?} ", self.current_line(), self.peek())
        };


        loop{
            let op_tok = match self.peek(){
                Token::EOF | Token::Scolon |Token::Lbrace|
                Token::Rparen | Token::Comma | Token::Arrow |
                Token::Vertical| Token::Rbracket | Token::Rbrace |
                Token::Range=> break,

                _true => self.peek()
            };

            let (l_bp, r_bp) = self.infix_binding_power(&op_tok);

            if l_bp < min_bp{
                break;
            }

            if is_regular_op(op_tok){self.stmt.regular_op_sum += 1}
            else if is_bool_op(op_tok){self.stmt.bool_node_sum += 1}
            else if is_compare_op(op_tok){self.stmt.compare_op_sum += 1}

            let op_key = self.current as Key;
            let op_tok = self.eat();

            let rhs_idx = self.expr(r_bp);

            let start : Start = self.ast.edges.len() as Start;
            self.ast.expr_recorder.push(lhs_idx?);
            self.ast.expr_recorder.push(rhs_idx?);
            self.ast.edges.extend(self.ast.expr_recorder.drain(..));
            let end : End = (self.ast.edges.len() - 1) as End;

            lhs_idx =Ok(self.push_node(op_tok, op_key , start, end))

        }
        lhs_idx
    }



    fn is_func_call(&self, peek : &Token) -> bool {
        matches!(peek, &Token::Lparen)
    }

    pub fn is_expr(&self, peek : &Token) -> bool{
        peek == &Token::Int
            || peek == &Token::Float
            || peek == &Token::Ident
            || peek == &Token::True
            || peek == &Token::Sub
            || peek == &Token::Lparen
            || peek == &Token::CmpNot
    }

    fn infix_binding_power(&self, op : &Token) -> (f32, f32){
        match op{
            Token::CmpAnd | Token::CmpOr     => (0.1, 0.2),
            Token::EqEq   | Token::NotEq     => (0.3, 0.3),
            Token::Grt    | Token::Les       => (0.5, 0.6),
            Token::GrtEq  | Token::LesEq     => (0.7, 0.8),
            Token::Add | Token::Sub => (1.0, 2.0),
            Token::Div    | Token::Mod       => (3.0, 3.1),
            Token::Mul                       => (4.0, 4.0),
            _         => panic!("[Line {:?}]Unknown Operator : {:?}",self.current_line(), op)
        }
    }



}



pub fn is_regular_op(tok : &Token) -> bool{
    match tok{
        Token::Add | Token::Sub | Token::Mul |
        Token::Div  | Token::Mod => {
            true
        }
        _ => false
    }
}

pub fn is_bool_op(tok : &Token) -> bool{
    match tok{
        Token::Les | Token::LesEq |
        Token::Grt | Token::GrtEq |
        Token::NotEq | Token::EqEq => true,
        _ => false
    }
}


pub fn is_compare_op(tok : &Token) -> bool{
    match tok{
        Token::CmpAnd | Token::CmpOr => true,
        _ => false
    }
}



