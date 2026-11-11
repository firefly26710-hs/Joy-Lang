use crate::lexer::diagnostic::source::issue::LexerIssueType;
use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::ast::{NONE, NodeIdx};
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;

impl Parser{
    pub fn parse_stmts(&mut self) -> Result<NodeIdx, LexerIssueType>{
        
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

        let edge_start : Start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.func_recorder.drain(start_idx..));
        let edge_end : End = self.ast.edge_end();

       Ok( self.push_node(Token::BlockStmt, NONE, edge_start, edge_end))
    }

    fn matches_stmts(&mut self) -> Result<NodeIdx, LexerIssueType>{
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
            Token::Rbrace=> Ok(NONE),
            _ => panic!("[Line {:?}] Find not exist statement {:?}", self.current_line(), self.peek())
        }
    }


    pub fn parse_var(&mut self) -> Result<NodeIdx, LexerIssueType>{
        self.eat();


        //var name
        let var_name_key = self.current as Key;
        let var_name_idx = self.get_tok_node_idx(&Token::Ident);


        self.matches_token(&Token::Ident)?;

        //type
        let declared_ty = if !matches!(self.peek(), Token::Eq){
            self.matches_token(&Token::Colon)?;
            self.matches_token(&Token::Type)?;
            self.type_mapping((self.current - 1) as Key)
        }else{
            Type::void
        };

        self.matches_token(&Token::Eq)?;

        let expr_idx = self.parse_expression(0.0)?;
        let actually_ty = self.matches_type(&declared_ty, expr_idx)?;


        self.insert_var_type(var_name_key, actually_ty);

        self.matches_token(&Token::Scolon)?;

        let edge_start = self.ast.edge_start();
        self.ast.edges.push(var_name_idx);
        self.ast.edges.push(expr_idx);
        let edge_end = self.ast.edge_end();

        self.sign_var(var_name_key);
        Ok(self.push_node(Token::LetStmt, NONE, edge_start, edge_end))
    }


    fn ident_branch(&mut self) -> Result<NodeIdx, LexerIssueType>{
        match self.peek_next(){
            Token::Lparen => self.parse_func_call(),
            Token::Eq |Token::AddEq | Token::SubEq | Token::MulEq | Token::DivEq
            |Token::ModEq | Token::Scolon => self.parse_var_update(),
            _ => panic!("[Line {:?}] Expected ; ", self.current_line())
        }
    }

    fn parse_var_update(&mut self) -> Result<NodeIdx, LexerIssueType>{
        let key = self.find_same_ident_key(self.current as Key);

        let ident_name_idx = self.push_leaf(Token::Ident, key)?;
        self.matches_token(&Token::Ident)?;

        let op_tok_key = self.current as Key;
        let op_tok = self.eat();

        let expr_idx = self.parse_expression(0.0)?;
        let ident_type = self.look_var_type(&key);
        self.matches_type(&ident_type.clone(), expr_idx)?;


       self.matches_token(&Token::Scolon)?;

        let start = self.ast.edge_start();
        self.ast.edges.push(ident_name_idx);
        self.ast.edges.push(expr_idx);
        let end = self.ast.edge_end();


       Ok( self.push_node(op_tok, op_tok_key, start, end))
    }



}