use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::NONE;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;

use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::stmt::logic::expr::is_expr_val;

impl Parser{
    pub fn parse_function(&mut self) -> Result<NodeIdx, ParserIssue>{

        self.matches_token(&Token::Fn)?;
        let func_name_key = self.current as Key;
        let func_name_node_idx = self.get_tok_node_idx(&Token::Ident);
        self.stmt.func_recorder.push(func_name_node_idx);
        self.matches_token(&Token::Ident)?;

        self.matches_token(&Token::Lparen)?;

        let has_declared_arg = self.peek() == &Token::Ident;

        if has_declared_arg{
            loop{
                let arg_name_key = self.current as Key;
                let arg_idx =self.get_tok_node_idx(&Token::Ident);

                self.matches_token(&Token::Ident)?;
                self.matches_token(&Token::Colon)?;
                self.matches_token(&Token::Type)?;
                self.stmt.func_recorder.push(arg_idx);


                match self.peek(){
                    Token::Rparen => break,
                    Token::Comma => self.eat(),
                    _ => break,
                };
            }
        }


        self.matches_token(&Token::Rparen)?;
        self.matches_token(&Token::Eq)?;
        self.matches_token(&Token::Type)?;



        let stmt_idx = self.parse_scope()?;

        let start = self.ast.edge_start();
        self.stmt.func_recorder.push(stmt_idx);
        self.ast.edges.extend(self.stmt.func_recorder.drain(..));

        let end = self.ast.edge_end();

        Ok( self.push_node(Token::FuncStmt, NONE, start, end))


    }



    pub fn parse_ret(&mut self) -> Result<NodeIdx, ParserIssue>{
        self.eat();
        let expr_idx : Option<NodeIdx> = if is_expr_val(&self.peek()){
            Some(self.parse_expression()?)
        }else{
            None
        };
        self.matches_token(&Token::Scolon)?;

        let start = self.ast.edge_start();
        self.ast.edges.push(NONE);
        if expr_idx != None{
            self.ast.edges.push(expr_idx.unwrap())
        }else{
            self.ast.edges.push(NONE);
        }
        let end = self.ast.edge_end();

       Ok( self.push_node(Token::RetStmt, NONE, start, end))
    }

    pub fn parse_func_call(&mut self) -> Result<NodeIdx, ParserIssue>{
        let expr_func_idx = self.expr_func_call()?;

       self.matches_token(&Token::Scolon)?;

        Ok(expr_func_idx)
    }

    pub fn expr_func_call(&mut self) -> Result<NodeIdx, ParserIssue> {
        let func_name_key = self.current as Key;

        let func_name_idx = {
            self.eat();
            self.push_leaf(Token::Ident, func_name_key)
        }?;

        self.stmt.func_call_recorder.push(func_name_idx);
        self.matches_token(&Token::Lparen)?;


        let has_param = self.peek() != &Token::Rparen;


        if has_param {
            loop{
                let expr_idx = self.parse_expression()?;
                self.stmt.func_call_recorder.push(expr_idx);
                match self.peek(){
                    Token::Comma => self.eat(),
                    Token::Rparen => break,
                    _ => break
                };
            }
        }
        self.matches_token(&Token::Rparen)?;

        let start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.func_call_recorder.drain(..));
        let end = self.ast.edge_end();

        Ok(self.push_node(Token::CallStmt, NONE, start, end))
    }


    pub fn parse_print(&mut self) -> Result<NodeIdx, ParserIssue>{
        self.eat();

       self.matches_token(&Token::Lparen)?;
        let type_idx = self.get_tok_node_idx(&Token::Type);
        self.matches_token(&Token::Type)?;

        self.matches_token(&Token::Comma)?;
        let expr_idx = self.parse_expression()?;
        
        self.matches_token(&Token::Rparen)?;
        self.matches_token(&Token::Scolon)?;

        let edges_start = self.ast.edge_start();
        self.ast.edges.push(type_idx);
        self.ast.edges.push(expr_idx);
        let edges_end = self.ast.edge_end();

        Ok(self.push_node(Token::PrintStmt, NONE, edges_start, edges_end))


    }




}