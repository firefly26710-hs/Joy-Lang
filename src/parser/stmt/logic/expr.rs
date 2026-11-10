use crate::lexer::diagnostic::source::issue::IssueType;
use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;

impl Parser{
    pub fn parse_expression(&mut self, min_bp : f32) -> Result<NodeIdx, IssueType>{



        self.stmt.prev_node_type = Type::void;
        self.stmt.is_prev_node_is_ident = false;
        self.stmt.has_ident_in_expr = false;
        self.stmt.prev_node_key = 0;
        self.stmt.expr_start_key = self.current as Key;

        self.expr(min_bp)
    }
    fn expr(&mut self, min_bp : f32) -> Result<NodeIdx, IssueType>{
        let mut lhs_idx = match self.peek(){
            Token::Int =>{
                let key = self.current as Key;
                self.stmt.prev_node_key = key;
                self.update_node_type(key)?;
                self.eat();
                self.stmt.is_prev_node_is_ident = false;
                self.push_leaf(Token::Int, key)
            }
            Token::Float =>{
                let key = self.current as Key;
                self.stmt.prev_node_key = key;
                self.update_node_type(key)?;
                self.eat();
                self.stmt.is_prev_node_is_ident = false;
                self.push_leaf(Token::Float, key)
            }
            Token::True =>{
                let key = self.current as Key;
                self.eat();
                self.push_leaf(Token::True, key)
            }
            Token::False=> {
                let key = self.current as Key;
                self.eat();
                self.push_leaf(Token::False, key)
            },
            Token::Sub => {
                let key = self.current as Key;
                self.eat();
                let expr_idx = self.parse_expression(1000.0)?;
                let start = self.ast.edge_start();
                self.ast.edges.push(NONE);
                self.ast.edges.push(expr_idx);
                let end = self.ast.edge_end();
                Ok(self.push_node(Token::Sub, key, start, end))
            },
            Token::Lparen => {
                self.eat();
                let expr_idx = self.parse_expression(0.0)?;
                self.matches_token(&Token::Rparen)?;
                Ok(expr_idx)
            }
            Token::CmpNot => {
                let key = self.current as Key;
                self.eat();
                self.matches_token(&Token::Lparen)?;
                let expr_idx = self.parse_expression(0.0)?;
                self.matches_token(&Token::Rparen)?;
                let start = self.ast.edge_start();
                self.ast.edges.push(NONE);
                self.ast.edges.push(expr_idx);
                let end = self.ast.edge_end();
                Ok(self.push_node(Token::CmpNot, key, start, end))
            }
            Token::Ident =>{
                if self.is_func_call(self.peek_next()){
                    self.expr_func_call()
                }else {
                    let ident_key = self.find_same_ident_key(self.current as Key);
                    self.var_not_in_scope(ident_key)?;
                    self.update_node_type(ident_key)?;
                    self.eat();
                    self.stmt.is_prev_node_is_ident = true;
                    self.stmt.has_ident_in_expr = true;
                    self.push_leaf(Token::Ident, ident_key)
                }
            }
            _ => Err(self.unexpected_expression_issue())
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

pub fn is_op(tok : &Token) -> bool{
    is_regular_op(tok) || is_bool_op(tok) || is_compare_op(tok)
}

fn is_regular_op(tok : &Token) -> bool{
    match tok{
        Token::Add | Token::Sub | Token::Mul |
        Token::Div  | Token::Mod => {
            true
        }
        _ => false
    }
}

fn is_bool_op(tok : &Token) -> bool{
    match tok{
        Token::Les | Token::LesEq |
        Token::Grt | Token::GrtEq |
        Token::NotEq | Token::EqEq => true,
        _ => false
    }
}


fn is_compare_op(tok : &Token) -> bool{
    match tok{
        Token::CmpAnd | Token::CmpOr => true,
        _ => false
    }
}



