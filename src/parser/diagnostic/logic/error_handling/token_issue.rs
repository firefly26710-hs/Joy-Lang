use crate::parser::diagnostic::source::issue::TokenIssue;
use crate::parser::diagnostic::source::msg_data::ParserMsgData;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;

impl Parser{
    pub fn matches_token(&mut self, expected_tok : &Token) -> Result<(), ParserIssue>{
        let current_tok = self.peek();
        if current_tok == &Token::EOF{
            //dummy value
            return Err(ParserIssue::Token(TokenIssue::UnclosedDelimiter))
        }
        if current_tok == expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.unexpected_token_issue(&expected_tok))
        }
    }

    pub fn unclosed_delimiter(&mut self, expected_tok : &Token) -> Result<(), ParserIssue>{
        let current_tok = self.peek();
        if current_tok == &Token::EOF{
            //dummy value
            return Err(ParserIssue::Token(TokenIssue::UnclosedDelimiter))
        }
        if current_tok == expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.unclosed_delimiter_token_issue())
        }
    }

    pub fn unexpected_token_issue(
        &mut self,
        correct_token : &Token,
    ) -> ParserIssue {
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        let span_id = if wrong_tok_key >= self.lexer.error.span_ids.len(){
            self.lexer.error.span_ids[wrong_tok_key - 1] as usize
        }else{
            self.lexer.error.span_ids[wrong_tok_key] as usize
        };
        let line = span_id + 1;

        let interval = if wrong_tok_key >= self.lexer.tokenizer.intervals.len(){
            &self.lexer.tokenizer.intervals[wrong_tok_key - 1]
        } else{
            &self.lexer.tokenizer.intervals[wrong_tok_key]
        };
        let start =interval.start as usize;
        let end = interval.end as usize;

        let wrong_string = self.lexer.tokenizer.input[start..end].to_string();
        let correct_string = from_tok_to_string(correct_token).to_string();

        let issue = ParserIssue::Token(TokenIssue::UnExpectedToken {wrong_string, correct_string});

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn unclosed_delimiter_token_issue(&mut self) -> ParserIssue {
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        let span_id = if wrong_tok_key >= self.lexer.error.span_ids.len(){
            self.lexer.error.span_ids[wrong_tok_key - 1] as usize
        }else{
            self.lexer.error.span_ids[wrong_tok_key] as usize
        };
        let line = span_id + 1;

        let interval = if wrong_tok_key >= self.lexer.tokenizer.intervals.len(){
            &self.lexer.tokenizer.intervals[wrong_tok_key - 1]
        } else{
            &self.lexer.tokenizer.intervals[wrong_tok_key]
        };
        let start =interval.start as usize;
        let end = interval.end as usize;

        let issue = ParserIssue::Token(TokenIssue::UnclosedDelimiter);

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }
    
    
    
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
        Token::LogicNot => "not",
        Token::LogicAnd => "and",
        Token::LogicOr => "or",
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