use crate::lexer::center::lex::Lexer;

impl Lexer{
    pub fn collect_diagnostic_data(&mut self) {
        self.error.span_ids.push(self.error.span_id);
    }
    
    
    
}

