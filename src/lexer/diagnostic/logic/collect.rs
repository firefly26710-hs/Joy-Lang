use crate::lexer::center::lex::Lexer;

impl Lexer{
    pub fn collect_diagnostic_data(&mut self){
        self.error.lines.push(self.error.line);
        self.error.span_ids.push(self.error.span_id);
        self.error.rlt_start_cols.push(self.tokenizer.current);
    }
    
    
    
}

