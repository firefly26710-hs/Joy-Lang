#[derive(Debug, Clone, PartialEq)]

pub enum FunctionIssue{
    MismatchesRetTypeWithExpr {ret_type_string : String, actual_type_string : String},
    MismatchesRetTypeWithOutExpr{ret_type_string : String},
    NotRetVoidButNoReturn{ret_type_string : String}
}