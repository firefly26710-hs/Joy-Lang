use crate::parser::tree::ast::NodeIdx;
use crate::semantic::center::sema::Semantic;

impl Semantic{
    pub fn analyze_function(&self, func_idx : NodeIdx){
        let node = &self.nodes[func_idx as usize];
        let edges = self.get_edges(node);
        let len = edges.len();
        
    }
}