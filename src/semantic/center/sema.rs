use crate::parser::center::par::Parser;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Node;
use crate::semantic::scope::data::scopestruct::{Scope};
use crate::semantic::symbol::data::data::Symbol;

pub struct Semantic{
    pub nodes : Vec<Node>,
    pub edges : Vec<NodeIdx>,
    pub scope : Scope,
    pub symbol: Symbol,
}

impl Semantic{
    pub fn init(parser : Parser) -> Self{
        let alloc_size = parser.ast.nodes.len();
        
        Self{
            nodes : parser.ast.nodes,
            edges : parser.ast.edges,
            scope : Scope::init(alloc_size),
            symbol: Symbol::init(),
        }
        
    }
}