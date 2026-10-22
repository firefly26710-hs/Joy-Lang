use crate::lexer::center::lex::Lexer;
use crate::lexer::diagnostic::logic::issue::IssueType;
use crate::lexer::diagnostic::source::msg_data::MsgData;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;

impl Parser{
    pub fn collect_unexpected_token_issue(
        &mut self,
        correct_token : &Token,
    ) -> IssueType{
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        //get range of sentence which has it
        let span_id = self.lexer.error.span_ids[wrong_tok_key] as usize;
        let span_range = self.lexer.error.spans[span_id];
        let line = span_id + 1;

        //alt means absolute
        let alt_span_start = span_range.0 as usize;
        let alt_span_end = span_range.1 as usize;

        //get itself Absolute column
        let interval = &self.lexer.tokenizer.intervals[wrong_tok_key];
        let alt_col_start = interval.start as usize;
        let alt_col_end = interval.end as usize;

        let wrong_string = self.lexer.tokenizer.input[alt_col_start..alt_col_end].to_string();

        let rlt_col_start = alt_col_start - alt_span_start;
        let rlt_col_end = alt_col_end - alt_span_start;

        let correct_string = from_tok_to_string(correct_token).to_string();

        let issue =  IssueType::UnExpectedToken{wrong_string, correct_string};

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        self.out_of_this_stmt();
        
        issue


    }

    pub fn out_of_this_stmt(&mut self){
        while !is_stmt(self.peek()) && !matches!(self.peek(), Token::EOF){
            self.eat();
        }


    }

}

fn is_stmt(tok : &Token) -> bool{
    matches!(tok,
        Token::Val|
        Token::Var |
        Token::If  |
        Token::Elif|
        Token::Else|
        Token::For |
        Token::While|
        Token::Ctn |
        Token::Brk |
        Token::Ret |
        Token::Print
    )


}


fn from_tok_to_string(tok : &Token) -> &str{
    match tok{
        Token::Val => "val",
        Token::Var => "var",
        Token::For => "for",
        Token::While => "while",
        Token::In => "in",
        Token::Ctn => "continue",
        Token::Brk => "break",
        Token::If => "if",
        Token::Elif => "elif",
        Token::Else => "else",
        Token::CmpNot => "not",
        Token::CmpAnd => "and",
        Token::CmpOr => "or",
        Token::Fn => "fn",
        Token::Ret => "return",
        Token::True => "true",
        Token::False => "false",
        Token::Print => "print",

        Token::Add => "+",
        Token::AddEq => "+=",
        Token::Sub => "-",
        Token::SubEq => "-=",
        Token::Mul => "*",
        Token::MulEq => "*=",
        Token::Div => "/",
        Token::DivEq => "/=",
        Token::Mod => "%",
        Token::ModEq => "%=",
        Token::Eq => "=",
        Token::EqEq => "==",
        Token::Lparen => "(",
        Token::Rparen => ")",
        Token::Lbrace => "{",
        Token::Rbrace => "}",
        Token::Lbracket => "[",
        Token::Rbracket => "]",
        Token::Colon => ":",
        Token::Scolon => ";",
        Token::Comma => ",",
        Token::Vertical => "|",
        Token::Ident => "an identifier",
        Token::Type => "a Type",
        _ => panic!("Expected Token : {:?}", tok)
    }
}


#[test]
fn test_par_error(){
    let input =
        "var a = 5;".to_string();
    let input = input + "\n\0\0\0";
    let mut lexer = Lexer::init(input, "main.joy".to_string());
    lexer.tokenize();

    let mut parser = Parser::init(lexer);
    parser.parse_var();

    if parser.lexer.error.has_error{
        parser.lexer.output_error();
    }

}



