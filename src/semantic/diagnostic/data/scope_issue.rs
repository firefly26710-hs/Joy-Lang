#[derive(Debug, Clone, PartialEq)]
pub enum ScopeIssue{
    VariableNotInScope{ var_string: String},
    FunctionNotFound{ unknown_fn_string: String},
    OutOfLoop{kw : String},
}

