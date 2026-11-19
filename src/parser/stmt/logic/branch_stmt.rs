use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::tree::ast::{NONE, NodeIdx};

impl Parser {
    pub fn parse_if(&mut self) -> Result<NodeIdx, ParserIssue>{
        
        let b_recorder_start = self.stmt.branch_recorder.len();
        self.record_branch_idx(b_recorder_start)?;

        let start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.branch_recorder.drain(..));
        let end = self.ast.edge_end();

       Ok( self.push_node(Token::IfKw, NONE, start, end))
    }

    fn record_branch_idx(&mut self, b_recorder_start : usize) -> Result<(), ParserIssue>{
        match self.peek(){
            Token::If => {
                self.eat();
                let expr_idx = self.parse_expression()?;
                let stmts_idx = self.parse_scope()?;

                self.stmt.branch_recorder.push(expr_idx);
                self.stmt.branch_recorder.push(stmts_idx);
            },
            Token::Elif => {
                if self.stmt.branch_recorder[b_recorder_start..].len() < 2{
                    panic!("[Line {:?}] Expected 'elif' token, but found 'if'", self.current_line())
                }
                self.eat();
                let expr_idx = self.parse_expression()?;
                let stmts_idx = self.parse_scope()?;


                self.stmt.branch_recorder.push(expr_idx);
                self.stmt.branch_recorder.push(stmts_idx);
            },
            Token::Else =>{
                if self.stmt.branch_recorder[b_recorder_start..].len() < 2{
                panic!("[Line {:?}] Expected 'else' token", self.current_line())
                }

                self.eat();
                let stmts_idx = self.parse_scope()?;


                self.stmt.branch_recorder.push(NONE);
                self.stmt.branch_recorder.push(stmts_idx);
            }
            _ => unreachable!()
        };

        match self.peek(){
            Token::Elif | Token::Else => self.record_branch_idx(b_recorder_start),
            _ => Ok(())
        }
    }

    pub fn parse_break(&mut self) -> Result<NodeIdx, ParserIssue>{

        self.eat();

        self.matches_token(&Token::Scolon)?;

        self.push_leaf(Token::BrkKw, NONE)
    }

    pub fn parse_ctn(&mut self) -> Result<NodeIdx, ParserIssue>{

        self.eat();

        self.matches_token(&Token::Scolon)?;

        self.push_leaf(Token::CtnKw, NONE)

    }






    



   
}