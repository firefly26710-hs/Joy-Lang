#[derive(Debug, Clone, PartialEq)]

pub enum StmtKind {
    VarStmt,
    IfStmt ,
    ElifStmt,
    ElseStmt,
    ForStmt ,
    WhileStmt,
    RetStmt,
    CtnStmt,
    BrkStmt ,
    FuncStmt,
    CallStmt,
    PrintStmt,
    UpdateStmt
}

pub fn from_stmt_kind_to_string(kind : &StmtKind) -> String{
    match kind{
        StmtKind::VarStmt => "VarStmt",
        StmtKind::IfStmt  => "IfStmt",
        StmtKind::ForStmt => "ForStmt",
        StmtKind::WhileStmt=> "WhileStmt",
        StmtKind::RetStmt => "RetStmt",
        StmtKind::CtnStmt => "CtnStmt",
        StmtKind::BrkStmt => "BrkStmt",
        StmtKind::FuncStmt=> "FuncStmt",
        StmtKind::CallStmt => "CallStmt",
        StmtKind::PrintStmt => "PrintStmt",
        StmtKind::UpdateStmt => "UpdateStmt",
        StmtKind::ElifStmt  => "ElifStmt",
        StmtKind::ElseStmt  => "ElseStmt"
    }.to_string()
}