use crate::lexer::diagnostic::source::data::{SpanLength, SpanStart};
use crate::lexer::diagnostic::source::issue::LexerIssueType;

#[derive(Debug, Clone, PartialEq)]
pub struct LexerMsgData {
    pub issue    : LexerIssueType,
    pub line     : usize,
    pub rlt_col_start: usize,
    pub rlt_col_end  : usize,
    pub span_id         : usize
}

impl LexerMsgData {
    pub fn new(
        issue: LexerIssueType,
        line : usize,
        rlt_col_start : usize,
        rlt_col_end : usize,
        span_id : usize
    ) -> Self{
        Self{
            issue,
            line,
            rlt_col_start,
            rlt_col_end,
            span_id
        }
        
    }
}


