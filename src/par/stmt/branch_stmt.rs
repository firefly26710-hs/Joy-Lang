use crate::par::parser::parser::NONE;
use crate::lex::token::Token;
use crate::par::parser::parser::NodeIdx;
use crate::par::parser::parser::Parser;
use crate::par::parser::ty::Type;

impl Parser{
    pub fn parse_if(&mut self) -> NodeIdx{
        let b_recorder_start = self.branch_recorder.len();
        self.record_branch_idx(b_recorder_start);

        let start = self.ast.edge_start();
        self.ast.edges.extend(self.branch_recorder.drain(..));
        let end = self.ast.edge_end();

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
                let block_idx = self.parse_stmts();
                self.matches_token(&Token::Rbrace, "Expected }");
                self.scope.destroy_branch_scope();

                self.branch_recorder.push(expr_idx);
                self.branch_recorder.push(block_idx);
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
                let block_idx = self.parse_stmts();
                self.matches_token(&Token::Rbrace, "Expected }");
                self.scope.destroy_branch_scope();

                self.branch_recorder.push(expr_idx);
                self.branch_recorder.push(block_idx);
            },
            Token::Else =>{
                if self.branch_recorder[b_recorder_start..].len() < 2{
                panic!("[Line {:?}] Expected 'else' token", self.current_line())
                }

                self.eat();
                self.scope.build_branch_scope();
                self.matches_token(&Token::Lbrace,"Expected {");
                let block_idx = self.parse_stmts();
                self.matches_token(&Token::Rbrace, "Expected }");
                self.scope.destroy_branch_scope();

                self.branch_recorder.push(NONE);
                self.branch_recorder.push(block_idx);
            }
            _ => unreachable!()
        };

        match self.peek(){
            Token::Elif | Token::Else => self.record_branch_idx(b_recorder_start),
            _ => return
        }
    }



   
}