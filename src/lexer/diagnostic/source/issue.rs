use crate::lexer::tokenizer::source::token::Token;
use crate::parser::types::typ::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    IllegalToken,
    MismatchString,
    UnExpectedToken{wrong_string : String, correct_string : String},
    UnclosedDelimiter,
    IllegalExprMatch{first : String, second : String },

    //MismatchType{type_string : String},
    LiteralOutOfRange{expected_type : String},
    MismatchesType{expected_type : String,  actual_type : String},
    MismatchesNodeType{prev_node_type : String, curr_node_type : String},
    VariableNotInScope{unknown_var : String},
    FunctionNotFound{unknown_fn : String},
}