use crate::lexer::diagnostic::logic::issue::IssueType;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;

impl Parser{
    pub fn expect(&mut self, expected_tok : &Token) -> Result<(), IssueType>{
        if self.peek() == expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.collect_unexpected_token_issue(&expected_tok))
        }
    }



}