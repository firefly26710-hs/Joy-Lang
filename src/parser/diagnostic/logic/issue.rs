use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::diagnostic::source::msg_data::ParserMsgData;
use crate::parser::tree::node::Key;

impl Parser {
    pub fn error_unexpected_token(
        &mut self,
        correct_token: &Token,
        wrong_tok_key: usize,
    ) -> ParserIssue {
        self.error.has_error = true;

        let span_id = if wrong_tok_key >= self.error.span_ids.len() {
            self.error.span_ids[wrong_tok_key - 1] as usize
        } else {
            self.error.span_ids[wrong_tok_key] as usize
        };
        let line = span_id + 1;

        let interval = if wrong_tok_key >= self.intervals.len() {
            &self.intervals[wrong_tok_key - 1]
        } else {
            &self.intervals[wrong_tok_key]
        };
        let start = interval.start as usize;
        let end = interval.end as usize;

        let wrong_string = self.input[start..end].to_string();
        let correct_string = from_tok_to_string(correct_token).to_string();

        let issue = ParserIssue::UnExpectedToken {
            wrong_string,
            correct_string,
        };

        self.error
            .par_msg_data
            .push(ParserMsgData::new(issue.clone(), line, start, end, span_id));

        issue
    }

    pub fn error_unexpected_expression(&mut self, wrong_tok_key: usize) -> ParserIssue {
        self.error.has_error = true;
        let span_id = self.error.span_ids[wrong_tok_key] as usize;
        let line = span_id + 1;

        let interval = &self.intervals[wrong_tok_key];

        let start = interval.start as usize;
        let end = interval.end as usize;

        let wrong_string = self.input[start..end].to_string();
        let issue = ParserIssue::UnExpectedExpr { wrong_string };

        self.error
            .par_msg_data
            .push({ ParserMsgData::new(issue.clone(), line, start, end, span_id) });

        issue
    }

    pub fn error_unclosed_delimiter_token(&mut self, wrong_start_key: Key) -> ParserIssue {
        self.error.has_error = true;

        let span_id = if wrong_start_key as usize == self.error.span_ids.len() {
            self.error.span_ids[(wrong_start_key - 1) as usize] as usize
        } else {
            self.error.span_ids[wrong_start_key as usize] as usize
        };
        let line = span_id + 1;

        let interval = if wrong_start_key as usize == self.intervals.len() {
            &self.intervals[(wrong_start_key - 1) as usize]
        } else {
            &self.intervals[wrong_start_key as usize]
        };

        let start = interval.start as usize;
        let end = interval.end as usize;

        let issue = ParserIssue::UnclosedDelimiter;

        self.error
            .par_msg_data
            .push(ParserMsgData::new(issue.clone(), line, start, end, span_id));

        issue
    }

    pub fn error_comparison_op_chained(&mut self, op_key: Key) -> ParserIssue {
        self.error.has_error = true;
        let span_id = self.error.span_ids[op_key as usize] as usize;
        let line = span_id + 1;

        let interval = &self.intervals[op_key as usize];

        let start = interval.start as usize;
        let end = interval.end as usize;

        let issue = ParserIssue::ComparisonOpChained;

        self.error
            .par_msg_data
            .push({ ParserMsgData::new(issue.clone(), line, start, end, span_id) });

        issue
    }
}

fn from_tok_to_string(tok: &Token) -> &str {
    match tok {
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
        _ => panic!("Expected Token : {:?}", tok),
    }
}
