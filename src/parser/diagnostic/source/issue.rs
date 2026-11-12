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
    MismatchUnaryType{unary_op_string : String, right_expr_type_string : String},
    MismatchesExprType { left_expr_type_string: String, right_expr_type_string: String},
    ComparisonOpChained,
    BinaryOpMismatches{expr_type_string : String}
}
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeIssue{
    VariableNotInScope{ unknown_var_string: String},
    FunctionNotFound{ unknown_fn_string: String},
    OutOfLoop{kw : String},
}