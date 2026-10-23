use crate::lexer::diagnostic::source::issue::IssueType;

#[derive(Debug, Clone, PartialEq)]
pub struct MsgData {
    pub issue    : IssueType,
    pub line     : usize,
    pub rlt_col_start: usize,
    pub rlt_col_end  : usize,
    pub span_id         : usize
}

impl MsgData {
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