use crate::parser::tree::ast::NodeIdx;
use crate::semantic::control_flow::data::stmt_data::StmtData;

pub struct Flow{
    pub stmts  : Vec<StmtData>,
    pub kw_idx : Vec<NodeIdx>
}

impl Flow{
    pub fn init(alloc_size : usize) -> Self{
        Self{
            stmts : Vec::with_capacity(alloc_size),
            kw_idx: Vec::with_capacity(alloc_size),
        }
    }
}