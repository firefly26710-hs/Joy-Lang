#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind{
    Fn,
    Local,
    Global,
    //todo ? struct enum
}

pub fn from_scope_kind_to_string(kind : &ScopeKind) -> String{
    match kind {
        ScopeKind::Fn => "Fn",
        ScopeKind::Local => "Local",
        ScopeKind::Global => "Global",
    }.to_string()
}