

#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    IllegalToken,
    MismatchString,
    UnExpectedToken{wrong_string : String, correct_string : String},
    UnclosedDelimiter,
    MismatchType,
    MismatchExprType
}