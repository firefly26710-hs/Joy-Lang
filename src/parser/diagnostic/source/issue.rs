pub enum ParserIssue{
    Token(TokenIssue),
    Expr(ExprIssue),
    Scope(ScopeIssue),

    
    
    
    
}

pub enum TokenIssue{
    UnExpectedToken{wrong_string : String, correct_string : String},
    UnclosedDelimiter,
}

pub enum ExprIssue{
    IllegalExprMatch{first : String, second : String },
    LiteralOutOfRange{expected_type : String},
    MismatchesType{expected_type : String,  actual_type : String},
    MismatchesNodeType{ left_node_type: String, right_node_type: String},
    ComparisonChained
}

pub enum ScopeIssue{
    VariableNotInScope{unknown_var : String},
    FunctionNotFound{unknown_fn : String},
    OutOfLoop{kw : String},
}