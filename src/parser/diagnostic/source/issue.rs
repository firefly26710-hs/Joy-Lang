#[derive(Debug, Clone, PartialEq)]
pub enum ParserIssue{
    Token(TokenIssue),
    Expr(ExprIssue),
    Scope(ScopeIssue),
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenIssue{
    UnExpectedToken{wrong_string : String, correct_string : String},
    UnclosedDelimiter,
}
#[derive(Debug, Clone, PartialEq)]
pub enum ExprIssue{
    UnExpectedExpression{wrong_string : String},
    LiteralOutOfRange{ expected_type_string: String},
    MismatchesType{expected_type : String,  actual_type : String},
    MismatchesNodeType{ left_node_type: String, right_node_type: String},
    ComparisonOpChained
}
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeIssue{
    VariableNotInScope{ unknown_var_string: String},
    FunctionNotFound{ unknown_fn_string: String},
    OutOfLoop{kw : String},
}