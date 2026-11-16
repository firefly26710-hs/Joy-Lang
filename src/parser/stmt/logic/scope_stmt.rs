use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::center::par::Parser;

impl Parser{
    pub fn parse_scope(&mut self) -> Result<NodeIdx, ParserIssue>{
        self.matches_token(&Token::Lbrace)?;
        let start_idx = self.stmt.func_recorder.len();
        while self.peek() != &Token::EOF{
            match self.matches_stmts(){
                Ok(stmt_idx) => {
                    self.stmt.func_recorder.push(stmt_idx);
                    if matches!(self.peek(), Token::Rbrace){
                        break
                    }
                }
                Err(_) => {
                    if matches!(self.peek(), Token::EOF){
                        break;
                    }
                    self.stmt.func_recorder.push(67);
                    self.out_of_this_stmt();

                }
            };
        }
        self.unclosed_delimiter(&Token::Rbrace)?;

        let edge_start : Start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.func_recorder.drain(start_idx..));
        let edge_end : End = self.ast.edge_end();

        Ok( self.push_node(Token::BlockStmt, NONE, edge_start, edge_end))
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
            Token::Rbrace=> Ok(NONE),
            _ => panic!("[Line {:?}] Find not exist statement {:?}", self.current_line(), self.peek())
        }
    }
}