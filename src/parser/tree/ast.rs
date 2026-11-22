use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::parser::tree::node::Node;
pub const NONE : u32 = u32::MAX;
pub const MAX_SIZE : usize = 1024;


pub type NodeIdx = u32;
pub struct AST{
    pub nodes : Vec<Node>,
    pub edges: Vec<NodeIdx>,
    pub expr_recorder: Vec<NodeIdx>,

   
}

impl AST{
    pub fn init() -> Self{
        Self{
            nodes : Vec::with_capacity(MAX_SIZE),
            edges : Vec::with_capacity(MAX_SIZE),
            expr_recorder : Vec::with_capacity(MAX_SIZE),

            
        }
    }
    
    pub fn edge_push(&mut self, idx : NodeIdx){
        self.edges.push(idx);
    }
    
    pub fn edge_start(&self) -> Start{
        self.edges.len() as Start
    }
    
    pub fn edge_end(&self) -> End{
        if self.edges.len() == 0{
            return 0;
        }
        (self.edges.len() - 1) as End
    }



}
