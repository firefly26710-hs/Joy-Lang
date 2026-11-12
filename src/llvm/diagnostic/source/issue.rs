#[derive(Debug, Clone, PartialEq)]
pub enum GenIssue{
    DivideByZero,
    ExprOutOfRange {expected_type_string : String}
}