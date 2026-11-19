#[derive(Debug, Clone, PartialEq)]
pub enum TypeIssue{
    LiteralOutOfRange{ expected_type_string: String},
    MismatchType {expected_type : String,  actual_type : String},
    UnaryMismatchType {unary_op_string : String, right_expr_type_string : String},
    DifferenceConstLiteral { left_literal_type: String, right_literal_type: String},
    DifferenceIdentifierLiteral{left_literal_type : String, right_literal_type : String},
    ComparisonOpChained,
    BinaryOpMismatches{expr_type_string : String}
}