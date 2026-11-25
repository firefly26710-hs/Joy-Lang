use crate::semantic::diagnostic::data::scope_issue::ScopeIssue;

#[derive(Debug, Clone, PartialEq)]
pub struct ScopeMsgData{
    pub issue       : ScopeIssue,
    pub line        : usize,
    pub token_start : usize,
    pub token_end   : usize,
    pub span_id     : usize
}

impl ScopeMsgData{
    pub fn new(
        issue : ScopeIssue,
        line : usize,
        token_start : usize,
        token_end : usize,
        span_id   : usize
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