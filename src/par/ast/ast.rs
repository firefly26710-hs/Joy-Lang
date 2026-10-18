use crate::lex::iterval::Start;
use crate::lex::iterval::End;
use crate::par::ast::node::{Node};
pub const NONE : u32 = u32::MAX;
pub const MAX_SIZE : usize = 1024;

type NodeIdx = u32;
pub struct AST{
    pub nodes : Vec<Node>,
    pub edges: Vec<NodeIdx>,
    pub expr_recorder: Vec<NodeIdx>,

    pub int_node_sum   : u8,
    pub flt_node_sum   : u8,
    pub bool_node_sum  : u8,
    pub regular_op_sum : u8,
    pub compare_op_sum : u8,
    pub is_string      : bool,

    pub max_value      : String
}

impl AST{
    pub fn init() -> Self{
        Self{
            nodes   : Vec::with_capacity(MAX_SIZE),
            edges: Vec::with_capacity(MAX_SIZE),
            expr_recorder: Vec::with_capacity(MAX_SIZE),

            int_node_sum : 0,
            flt_node_sum : 0,
            bool_node_sum : 0,
            regular_op_sum : 0,
            compare_op_sum : 0,
            is_string : false,

            max_value : String::with_capacity(1024)
        }
    }
    
    pub fn edge_push(&mut self, idx : NodeIdx){
        self.edges.push(idx);
    }
    
    pub fn edge_start(&self) -> Start{
        self.edges.len() as Start
    }
    
    pub fn edge_end(&self) -> End{
        (self.edges.len() - 1) as End
    }



}
