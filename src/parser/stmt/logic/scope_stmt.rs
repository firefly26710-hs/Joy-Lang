use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::diagnostic::source::issue::{ParserIssue};
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::center::par::Parser;
use crate::parser::tree::node::Key;

impl Parser{
    pub fn parse_scope(&mut self) -> Result<NodeIdx, ParserIssue>{
        let current_scope_len = self.stmt.scope_recorder.len();
        self.matches_token(&Token::Lbrace)?;
         'outer : while !matches!(self.peek(), Token::Rbrace){
            match self.matches_stmts(){
                Ok(stmt_idx) => {
                    self.stmt.scope_recorder.push(stmt_idx);
                }
                Err(_) => {
                    self.peek();
                    while !is_stmt(self.peek()) {
                        if matches!(self.peek(), Token::Lbrace){ self.stmt.lbrace_stack.push(self.current as u32) }
                        if matches!(self.peek(), Token::Rbrace) && !self.stmt.lbrace_stack.is_empty(){
                            self.stmt.lbrace_stack.pop();
                        }
                        if matches!(self.peek(), Token::EOF | Token::Fn){
                            break 'outer
                        }
                        self.eat();
                    }
                }
            };
        }
        if self.peek() == &Token::Rbrace{
            self.stmt.scope_recorder.push(NONE);
        }
       self.unclosed_delimiter()?;
        let edge_start : Start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.scope_recorder.drain(current_scope_len..));
        let edge_end : End = self.ast.edge_end();

        Ok( self.push_node(Token::Scope, NONE, edge_start, edge_end))
    }

    fn matches_stmts(&mut self) -> Result<NodeIdx, ParserIssue>{
        match self.peek(){
            Token::Val | Token::Var => self.parse_var(),
            Token::If     => self.parse_if(),
            Token::For    => self.parse_for(),
            Token::While  => self.parse_while(),
            Token::Ident  => self.ident_branch(),
            Token::Ret    => self.parse_ret(),
            Token::Brk   => self.parse_break(),
            Token::Ctn   => self.parse_ctn(),
            Token::Print => self.parse_print(),
            Token::Lbrace=> self.parse_scope(),
            Token::Fn    => self.parse_function(),
            Token::EOF => Err(ParserIssue::UnclosedDelimiter),
            _ => panic!("[Line {:?}] Find not exist statement {:?}", self.current_line(), self.peek())
        }
    }
}
fn is_stmt(peek : &Token) -> bool{
    matches!(peek,
        Token::Var|
        Token::If |
        Token::For|
        Token::While|
        Token::Ident|
        Token::Elif|
        Token::Else
    )
}