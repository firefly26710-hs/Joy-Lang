use crate::lex::iterval::Start;
use crate::lex::iterval::End;
use crate::par::ast::node::{Key, Node};
pub const NONE : u32 = u32::MAX;
pub const MAX_SIZE : usize = 1024;

type NodeIdx = u32;
pub struct AST{
    pub nodes : Vec<Node>,
    pub childs: Vec<NodeIdx>,
    pub expr_recorder: Vec<NodeIdx>
}

impl AST{
    pub fn init() -> Self{
        Self{
            nodes   : Vec::with_capacity(MAX_SIZE),
            childs  : Vec::with_capacity(MAX_SIZE),
            expr_recorder: Vec::with_capacity(MAX_SIZE)
        }
    }
    
    pub fn childs_start(&self) -> Start{
        self.childs.len() as Start
    }
    
    pub fn childs_end(&self) -> End{
        (self.childs.len() - 1) as End
    }



}
