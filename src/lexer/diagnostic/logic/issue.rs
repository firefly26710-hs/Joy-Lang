

#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    IllegalToken,
    MismatchToken,
    MismatchType,
    MismatchExprType
}