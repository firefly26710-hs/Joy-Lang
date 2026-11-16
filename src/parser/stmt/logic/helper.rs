use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;

impl Parser{
    pub fn peek(&self) -> &Token {
        &self.lexer.tokenizer.tokens[self.current]

    }

    pub fn peek_next(&self) -> &Token{
        &self.lexer.tokenizer.tokens[self.current + 1]

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
        self.lexer.error.span_ids[self.current]
    }

}