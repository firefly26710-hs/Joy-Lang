use crate::semantic::diagnostic::data::type_issue::TypeIssue;
#[derive(Debug, Clone, PartialEq)]
pub struct TypeMsgData{
    pub issue       : TypeIssue,
    pub line        : usize,
    pub token_start : usize,
    pub token_end   : usize,
    pub span_id     : usize
}

impl TypeMsgData{
    pub fn new(
        issue : TypeIssue,
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