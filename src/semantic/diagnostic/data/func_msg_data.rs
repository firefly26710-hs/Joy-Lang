use crate::semantic::diagnostic::data::func_issue::FunctionIssue;

#[derive(Debug, Clone, PartialEq)]
pub struct FuncMsgData{
    pub issue       : FunctionIssue,
    pub line        : usize,
    pub token_start : usize,
    pub token_end   : usize,
    pub span_id     : usize
}

impl FuncMsgData {
    pub fn new(
        issue : FunctionIssue,
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