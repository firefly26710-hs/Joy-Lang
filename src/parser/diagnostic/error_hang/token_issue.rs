use crate::lexer::diagnostic::source::issue::LexerIssueType;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;

impl Parser{
    pub fn matches_token(&mut self, expected_tok : &Token) -> Result<(), ParserIssue>{
        let current_tok = self.peek();

        if current_tok == expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.unexpected_token_issue(&expected_tok))
        }
    }

    pub fn unclosed_delimiter(&mut self, expected_tok : &Token) -> Result<(), ParserIssue>{
        let current_tok = self.peek();


        if current_tok == expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.unclosed_delimiter_token_issue())
        }
    }
    
    
    
}