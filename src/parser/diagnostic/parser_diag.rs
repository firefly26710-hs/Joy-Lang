use crate::lexer::center::lex::Lexer;
use crate::lexer::diagnostic::logic::issue::IssueType;
use crate::lexer::diagnostic::source::msg_data::MsgData;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::node::Key;

impl Parser{
    pub fn collect_unexpected_token_issue(
        &mut self,
        correct_token : &Token,
    ) -> IssueType{
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        let(line,
            rlt_col_start,
            rlt_col_end,
            alt_col_start,
            alt_col_end,
            span_id)
        = self.get_data(wrong_tok_key);

        let wrong_string = self.lexer.tokenizer.input[alt_col_start..alt_col_end].to_string();
        let correct_string = from_tok_to_string(correct_token).to_string();

        let issue = IssueType::UnExpectedToken{wrong_string, correct_string};

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        issue
    }

    pub fn collect_unclosed_delimiter_token_issue(&mut self) -> IssueType{
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        let(line,
            rlt_col_start,
            rlt_col_end,
            _alt_col_start,
            _alt_col_end,
            span_id)
            = self.get_data(wrong_tok_key);


        let issue = IssueType::UnclosedDelimiter;

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        issue
    }

    pub fn collect_variable_not_in_scope_issue(&mut self, key: Key ) -> IssueType{
        self.lexer.error.has_error = true;

        let(line,
            rlt_col_start,
            rlt_col_end,
            _alt_col_start,
            _alt_col_end,
            span_id)
            = self.get_data(key as usize);

        let interval = &self.lexer.tokenizer.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let unknown_var = self.lexer.tokenizer.input[start..end].to_string();

        let issue = IssueType::VariableNotInScope{unknown_var};

        self.lexer.error.msg_datas.push(
            MsgData::new(
                issue.clone(),
                line,
                rlt_col_start,
                rlt_col_end,
                span_id
            )
        );

        issue



    }


    pub fn get_data(&self, wrong_tok_key : usize) -> (usize, usize, usize, usize, usize, usize){

        let already_look_last_tok = self.current >= self.lexer.error.span_ids.len();
        //get range of sentence which has it
        let span_id = if already_look_last_tok{
            self.lexer.error.span_ids[wrong_tok_key - 1] as usize

        }else{
            self.lexer.error.span_ids[wrong_tok_key] as usize
        };
        let span_range = self.lexer.error.spans[span_id];
        let line = span_id + 1;

        //alt means absolute
        let alt_span_start = span_range.0 as usize;

        //get itself Absolute column
        let interval = if already_look_last_tok{
            &self.lexer.tokenizer.intervals[wrong_tok_key - 1]
        }else{
            &self.lexer.tokenizer.intervals[wrong_tok_key]
        };

        let alt_col_start = interval.start as usize;
        let alt_col_end = interval.end as usize;



        let rlt_col_start = alt_col_start - alt_span_start;
        let rlt_col_end = alt_col_end - alt_span_start;

        (line,  rlt_col_start, rlt_col_end, alt_col_start, alt_col_end, span_id)


    }

    pub fn out_of_this_stmt(&mut self){
        while !is_stmt(self.peek()){
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
        Token::Print|
        Token::Rbrace|
        Token::EOF
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
        "fn main() = void{\n var x = x;".to_string();
    let input = input + "\n\0\0\0";
    let mut lexer = Lexer::init(input, "main.joy".to_string());
    lexer.tokenize();
    lexer.print_output();

    let mut parser = Parser::init(lexer);
    _ =parser.parse_function();

    if parser.lexer.error.has_error{
        parser.lexer.output_error();
    }

}



