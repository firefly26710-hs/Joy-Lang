use crate::llvm::diagnostic::source::issue::GenIssue;

#[derive(Debug, Clone, PartialEq)]
pub struct GenMsgData{
    pub issue    : GenIssue,
    pub line     : usize,
    pub token_start: usize,
    pub token_end: usize,
    pub span_id      : usize
}

impl GenMsgData{
    pub fn new(
        issue: GenIssue,
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