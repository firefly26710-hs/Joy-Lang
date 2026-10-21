

#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    IllegalToken,
    MismatchString,
    MismatchToken,
    MismatchType,
    MismatchExprType
}