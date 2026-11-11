use crate::lexer::center::lex::Lexer;
use crate::lexer::diagnostic::source::data::{SpanLength, SpanStart};
use crate::lexer::diagnostic::source::issue::LexerIssueType;
use crate::lexer::diagnostic::source::msg_data::LexerMsgData;

impl Lexer {
    pub fn collection_illegal_issue(&mut self){
        self.error.has_error = true;

        let rlt_col_start = self.tokenizer.current - self.error.span_start - 1;
        let rlt_col_end = self.tokenizer.current - self.error.span_start;

        let line = self.error.line;

        self.error.lex_msg_data.push(
            LexerMsgData::new(
                LexerIssueType::IllegalToken,
                line as usize,
                rlt_col_start as usize,
                rlt_col_end as usize,
                self.error.span_id as usize
            )
        );
    }

    pub fn collect_span(&mut self){
        self.error.spans.push(
            (self.error.span_start as SpanStart
             , self.tokenizer.current as SpanLength
            )
        );
        self.error.span_start = self.tokenizer.current;
        self.error.span_id += 1;
        self.error.line += 1;
    }

    




    
}


#[test]
fn test_error(){
    let input =
        "fn main() = void{\n \"Hello World\n}
    ".to_string();
    let input = input + "\0\0\0";
    let mut lexer = Lexer::init(input, "main.joy".to_string());
    lexer.tokenize();
}

