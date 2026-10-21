use crate::lexer::center::lex::Lexer;
use crate::lexer::tokenizer::source::interval::Interval;
use crate::lexer::tokenizer::source::token::Token;


impl Lexer{
    pub fn collect_tokenizer_data(&mut self ,token : Token){
        self.tokenizer.tokens.push(token);
        self.error.span_ids.push(self.error.span_id);
        self.tokenizer.intervals.push(Interval::new(self.tokenizer.start, self.tokenizer.current));
    }
    
    
    
    
}