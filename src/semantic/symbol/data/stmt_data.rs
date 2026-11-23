use crate::semantic::symbol::data::stmt_kind::StmtKind;

pub struct StmtData{
    pub kind : StmtKind,
    pub depth: u8
}

impl StmtData {
    pub fn new(kind : StmtKind, depth : u8) -> Self{
        Self{
            kind,
            depth
        }
    }

}