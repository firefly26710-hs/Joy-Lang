#[derive(Debug, Clone, PartialEq)]
pub enum TypeIssue{
    MismatchBetweenIdentifierLiteral { left_literal_type: String, right_literal_type: String},
    MismatchBetweenOpLiteral {op_string : String, correct_literal_type : String, error_literal_type : String},
    MismatchBetweenOpAndTwoLiterals { op_string : String, left_literal_type: String, right_literal_type: String},
    LiteralOutOfRange{ expected_type_string: String},
    MismatchType {expected_type : String,  actual_type : String},
    UnaryMismatchType {unary_op : String, right_type : String},
    
}