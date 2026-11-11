#[derive(Debug, Clone, PartialEq)]
pub enum GenIssue{
    DivideByZero,
    LiteralOutOfRange{expected_type_string : String}
}