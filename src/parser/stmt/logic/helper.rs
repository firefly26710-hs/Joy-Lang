use crate::parser::diagnostic::source::issue::{ParserIssue};
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::node::Key;
impl Parser{
    pub fn matches_token(&mut self, expected_tok : &Token) -> Result<(), ParserIssue>{
        let current_tok = self.peek().clone();
        let current_key = self.current as Key;
        
        if current_tok == *expected_tok{
            if current_tok == Token::Lbrace{
                self.stmt.lbrace_stack.push(current_key);
            }
            self.eat();
            Ok(())
        }else{
            Err(self.error_unexpected_token(&expected_tok, self.current))
        }
    }

    pub fn unclosed_delimiter(&mut self) -> Result<(), ParserIssue>{
        let current_tok = self.peek();
        if self.stmt.lbrace_stack.is_empty() {
            Ok(())
        }else{
            if current_tok == &Token::Rbrace{
                self.eat();
                self.stmt.lbrace_stack.pop();
                Ok(())
            }else{
                Err(self.error_unclosed_delimiter_token(
                    *self.stmt.lbrace_stack.last().unwrap()
                ))
            }
        }
    }

    pub fn out_this_wrong_stmt(&mut self){

    }


    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn peek_next(&self) -> &Token{
        &self.tokens[self.current + 1]

    }

    pub fn eat(&mut self) -> Token{
        let tok = self.tokens[self.current].clone();
        self.current += 1;
        tok
    }


    pub fn current_line(&self) -> u32{
        self.error.span_ids[self.current] + 1
    }

}

