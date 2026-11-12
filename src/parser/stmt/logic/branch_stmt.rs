use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::tree::ast::{NONE, NodeIdx};
use crate::parser::types::typ::Type;

impl Parser {
    pub fn parse_if(&mut self) -> Result<NodeIdx, ParserIssue>{
       
        
        let b_recorder_start = self.stmt.branch_recorder.len();
        self.record_branch_idx(b_recorder_start)?;

        dbg!(&self.stmt.branch_recorder);
        let start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.branch_recorder.drain(..));
        let end = self.ast.edge_end();



        if !self.stmt.already_full_ret_in_if{
            self.match_counter_number();
        }

       Ok( self.push_node(Token::IfStmt, NONE, start, end))
    }

    fn record_branch_idx(&mut self, b_recorder_start : usize) -> Result<(), ParserIssue>{
        match self.peek(){
            Token::If => {
                self.eat();
                let expr_idx = self.parse_expression()?;
                self.matches_type(&Type::bool, expr_idx)?;
                self.build_scope();
                let stmts_idx = self.parse_scope()?;
                self.destroy_scope();

                if !self.stmt.already_full_ret_in_if{
                    self.check_has_ret_in_end(stmts_idx, false);
                }

                self.stmt.branch_recorder.push(expr_idx);
                self.stmt.branch_recorder.push(stmts_idx);
            },
            Token::Elif => {
                if self.stmt.branch_recorder[b_recorder_start..].len() < 2{
                    panic!("[Line {:?}] Expected 'elif' token, but found 'if'", self.current_line())
                }
                self.eat();
                let expr_idx = self.parse_expression()?;
                self.matches_type(&Type::bool, expr_idx)?;
                self.build_scope();
                let stmts_idx = self.parse_scope()?;
                self.destroy_scope();
                if !self.stmt.already_full_ret_in_if{
                    self.check_has_ret_in_end(stmts_idx, false);
                }

                self.stmt.branch_recorder.push(expr_idx);
                self.stmt.branch_recorder.push(stmts_idx);
            },
            Token::Else =>{
                if self.stmt.branch_recorder[b_recorder_start..].len() < 2{
                panic!("[Line {:?}] Expected 'else' token", self.current_line())
                }

                self.eat();
                self.build_scope();
                let stmts_idx = self.parse_scope()?;
                self.destroy_scope();

                if !self.stmt.already_full_ret_in_if{
                    self.check_has_ret_in_end(stmts_idx, true);
                }

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
        self.has_out_of_loop()?;

        self.eat();

        self.matches_token(&Token::Scolon)?;

        self.push_leaf(Token::BrkStmt, NONE)
    }

    pub fn parse_ctn(&mut self) -> Result<NodeIdx, ParserIssue>{
        self.has_out_of_loop()?;

        self.eat();

        self.matches_token(&Token::Scolon)?;

        self.push_leaf(Token::CtnStmt, NONE)

    }






    pub fn check_has_ret_in_end(&mut self, stmt_idx : NodeIdx, is_else : bool){
        if is_else{self.stmt.is_have_else = true}

        let stmt_node_span = &self.ast.nodes[stmt_idx as usize].edge_span;
        let start = stmt_node_span.start as usize;
        let end = stmt_node_span.end as usize;

        let stmt_node_idx = *self.ast.edges[start..=end].last().unwrap() as usize;
        let stmt_token = &self.ast.nodes[stmt_node_idx].token;

        if matches!(stmt_token, Token::RetStmt){
            self.stmt.ret_times_in_branch += 1;
        }

        self.stmt.real_times_in_branch += 1;
    }


    fn match_counter_number(&mut self){
        if self.stmt.ret_times_in_branch == self.stmt.real_times_in_branch && self.stmt.is_have_else{
            self.stmt.already_full_ret_in_if = true;
            return;
        }else{
            self.stmt.ret_times_in_branch = 0;
            self.stmt.real_times_in_branch = 0;
        }
    }



   
}