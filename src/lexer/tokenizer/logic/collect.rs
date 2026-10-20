use crate::lexer::center::lex::Lexer;
use crate::lexer::tokenizer::source::interval::Interval;
use crate::lexer::tokenizer::source::token::Token;


impl Lexer{
    pub fn collect_tokenizer_data(&mut self ,token : Token){
        self.tokenizer.tokens.push(token);
        self.tokenizer.intervals.push(Interval::new(self.tokenizer.start, self.tokenizer.current));
        self.error.lines.push(self.error.line);
    }
    
    
    
    
}