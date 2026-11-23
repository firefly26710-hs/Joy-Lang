use crate::parser::tree::ast::NodeIdx;
use crate::semantic::symbol::data::stmt_data::StmtData;

pub struct Flow{
    pub stmt_datas: Vec<StmtData>,
    pub kw_idxs   : Vec<NodeIdx>,
}

impl Flow{
    pub fn init(alloc_size : usize) -> Self{
        Self{
            stmt_datas: Vec::with_capacity(alloc_size),
            kw_idxs   : Vec::with_capacity(alloc_size),

        }
    }
}