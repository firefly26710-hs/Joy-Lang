

#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    IllegalToken,
    MismatchString,
    UnExpectedToken{wrong_string : String, correct_string : String},
    UnclosedDelimiter,
    IllegalExprMatch{first : String, second : String },

    MismatchType{type_string : String},
    VariableNotInScope{unknown_var : String},
    NotDeclaredFunction,
}