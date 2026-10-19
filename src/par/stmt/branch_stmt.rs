use crate::lex::tokenize::token::Token;
use crate::par::build_tree::parser::Parser;
use crate::par::build_tree::ty::Type;
use crate::par::tree::ast::{NodeIdx, NONE};

impl Parser{
    pub fn parse_if(&mut self) -> NodeIdx{
        let b_recorder_start = self.stmt.branch_recorder.len();
        self.record_branch_idx(b_recorder_start);

        let start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.branch_recorder.drain(..));
        let end = self.ast.edge_end();

        if !self.stmt.already_full_ret_in_if{
            self.match_counter_number();
        }

        self.push_node(Token::IfStmt, NONE, start, end)
    }

    fn record_branch_idx(&mut self, b_recorder_start : usize){
        match self.peek(){
            Token::If => {


                self.eat();
                let expr_idx = self.parse_expression();
                if !self.is_matches_type(Type::bool) {
                    panic!("[Line {:?}] Expected bool type", self.current_line())
                }
                self.scope.build_branch_scope();
                self.matches_token(&Token::Lbrace,"Expected {");
                let stmts_idx = self.parse_stmts();
                self.matches_token(&Token::Rbrace, "Expected }");
                self.scope.destroy_branch_scope();

                if !self.already_full_ret_in_if{
                    self.check_has_ret_in_end(stmts_idx, false);
                }

                self.branch_recorder.push(expr_idx);
                self.branch_recorder.push(stmts_idx);
            },
            Token::Elif => {
                if self.branch_recorder[b_recorder_start..].len() < 2{
                    panic!("[Line {:?}] Expected 'elif' token, but found 'if'", self.current_line())
                }
                self.eat();
                let expr_idx = self.parse_expression();
                if !self.is_matches_type(Type::bool){
                    panic!("[Line {:?}] Expected bool type", self.current_line())
                }
                self.scope.build_branch_scope();
                self.matches_token(&Token::Lbrace,"Expected {");
                let stmts_idx = self.parse_stmts();
                self.matches_token(&Token::Rbrace, "Expected }");
                self.scope.destroy_branch_scope();

                if !self.already_full_ret_in_if{
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
                self.scope.build_branch_scope();
                self.matches_token(&Token::Lbrace,"Expected {");
                let stmts_idx = self.parse_stmts();
                self.matches_token(&Token::Rbrace, "Expected }");
                self.scope.destroy_branch_scope();

                if !self.already_full_ret_in_if{
                    self.check_has_ret_in_end(stmts_idx, true);
                }

                self.stmt.branch_recorder.push(NONE);
                self.stmt.branch_recorder.push(stmts_idx);
            }
            _ => unreachable!()
        };

        match self.peek(){
            Token::Elif | Token::Else => self.record_branch_idx(b_recorder_start),
            _ => return
        }
    }



    pub fn check_has_ret_in_end(&mut self, stmt_idx : NodeIdx, is_else : bool){
        if is_else{self.is_have_else = true}

        let stmt_node_span = &self.ast.nodes[stmt_idx as usize].edge_span;
        let start = stmt_node_span.start as usize;
        let end = stmt_node_span.end as usize;

        let stmt_node_idx = *self.ast.edges[start..=end].last().unwrap() as usize;
        let stmt_token = &self.ast.nodes[stmt_node_idx].token;

        if matches!(stmt_token, Token::RetStmt){
            self.ret_times_in_branch += 1;
        }

        self.real_times_in_branch += 1;
    }


    fn match_counter_number(&mut self){
        if self.ret_times_in_branch == self.real_times_in_branch && self.is_have_else{
            self.already_full_ret_in_if = true;
            return;
        }else{
            self.ret_times_in_branch = 0;
            self.real_times_in_branch = 0;
        }
    }



   
}