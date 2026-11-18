#[derive(Debug, Clone, PartialEq)]
pub enum ParserIssue{
    UnExpectedToken{wrong_string : String, correct_string : String},
    UnclosedDelimiter,
    UnExpectedExpr{wrong_string : String}
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

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionIssue{
    MismatchesRetTypeWithExpr {ret_type_string : String, actual_type_string : String},
    MismatchesRetTypeWithOutExpr{ret_type_string : String},
    NotRetVoidButNoReturn{ret_type_string : String}
}