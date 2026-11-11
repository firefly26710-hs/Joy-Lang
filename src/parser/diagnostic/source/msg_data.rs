use crate::lexer::diagnostic::source::data::{SpanLength, SpanStart};
use crate::parser::diagnostic::source::issue::ParserIssue;

#[derive(Debug, Clone, PartialEq)]
pub struct ParserMsgData{
    pub issue    : ParserIssue,
    pub line     : usize,
    pub token_start: usize,
    pub token_end: usize,
    pub span_id      : usize
}

impl ParserMsgData{
    pub fn new(
        issue: ParserIssue,
        line : usize,
        token_start: usize,
        token_end: usize,
        span_id      : usize
    ) -> Self{
        Self{
            issue,
            line,
            token_start,
            token_end,
            span_id
        }
    }
}