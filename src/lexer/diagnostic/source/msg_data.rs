use crate::lexer::diagnostic::source::data::{SpanLength, SpanStart};
use crate::lexer::diagnostic::source::issue::IssueType;

#[derive(Debug, Clone, PartialEq)]
pub struct LexerMsgData {
    pub issue    : IssueType,
    pub line     : usize,
    pub rlt_col_start: usize,
    pub rlt_col_end  : usize,
    pub span_id         : usize
}

impl LexerMsgData {
    pub fn new(
        issue: IssueType,
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
#[derive(Debug, Clone, PartialEq)]
pub struct ParserMsgData{
    pub issue    : IssueType,
    pub line     : usize,
    pub rlt_col_start: usize,
    pub rlt_col_end  : usize,
    pub span_start   : SpanStart,
    pub span_length  : SpanLength
}

impl ParserMsgData{
    fn new(
        issue: IssueType,
        line : usize,
        rlt_col_start: usize,
        rlt_col_end  : usize,
        span_start   : SpanStart,
        span_length  : SpanLength
    )  -> Self{
        Self{
            issue,
            line,
            rlt_col_start,
            rlt_col_end,
            span_start,
            span_length
        }
    }
}

