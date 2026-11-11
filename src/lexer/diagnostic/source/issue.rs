
#[derive(Debug, Clone, PartialEq)]
pub enum LexerIssueType {
    IllegalToken,
    
    
    
    UnExpectedToken{wrong_string : String, correct_string : String},
    UnclosedDelimiter,
    IllegalExprMatch{first : String, second : String },

    
    LiteralOutOfRange{expected_type : String},
    MismatchesType{expected_type : String,  actual_type : String},
    MismatchesNodeType{ left_node_type: String, right_node_type: String},
    VariableNotInScope{unknown_var : String},
    FunctionNotFound{unknown_fn : String},
    OutOfLoop{kw : String},
    
    ComparisonChained
}