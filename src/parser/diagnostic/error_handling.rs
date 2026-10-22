use crate::lexer::diagnostic::logic::issue::IssueType;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;

impl Parser{
    pub fn unexpected_token(&mut self, expected_tok : &Token) -> Result<(), IssueType>{
        let current_tok = self.peek();

        if current_tok == expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.collect_unexpected_token_issue(&expected_tok))
        }
    }

    pub fn unclosed_delimiter(&mut self, expected_tok : &Token) -> Result<(), IssueType>{
        let current_tok = self.peek();

        if current_tok == expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.collect_unclosed_delimiter_token_issue())
        }
    }






}