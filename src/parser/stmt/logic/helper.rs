use crate::parser::diagnostic::source::issue::{ParserIssue};
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;

impl Parser{
    pub fn matches_token(&mut self, expected_tok : &Token) -> Result<(), ParserIssue>{
        let current_tok = self.peek().clone();
        let current_key = self.current as Key;
        if current_tok == Token::Lbrace{
            self.lexer.error.lbrace_stack.push(current_key);
        }
        if current_tok == *expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.unexpected_token_issue(&expected_tok))
        }
    }

    pub fn unclosed_delimiter(&mut self) -> Result<(), ParserIssue>{
        let current_tok = self.peek();
        if self.lexer.error.lbrace_stack.is_empty() {
            Ok(())
        }else{
            if current_tok == Token::Rbrace{
                self.eat();
                self.lexer.error.lbrace_stack.pop();
                Ok(())
            }else{
                Err(self.unclosed_delimiter_token_issue(
                    *self.lexer.error.lbrace_stack.last().unwrap()
                ))
            }
        }
    }


    pub fn peek(&self) -> Token {
        self.lexer.tokenizer.tokens[self.current].clone()

    }

    pub fn peek_next(&self) -> Token{
        self.lexer.tokenizer.tokens[self.current + 1].clone()

    }

    pub fn eat(&mut self) -> Token{
        let tok = self.lexer.tokenizer.tokens[self.current].clone();
        self.current += 1;
        tok
    }


    pub fn type_mapping(&self, input_key: Key) -> Type{
        let interval = &self.lexer.tokenizer.intervals[input_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let type_str = &self.lexer.tokenizer.input[start..end];

        match type_str {
            "i8"     => Type::i8,
            "i16"    => Type::i16,
            "i32"    => Type::i32,
            "i64"    => Type::i64,
            "i128"   => Type::i128,
            "f32"    => Type::f32,
            "f64"    => Type::f64,
            "bool"   => Type::bool,
            "void"   => Type::void,
            _ => panic!("[Line {:?}] Unknown Type : {:?}",self.current_line(), type_str)
        }
    }

    pub fn current_line(&self) -> u32{
        self.lexer.error.span_ids[self.current] + 1
    }

}