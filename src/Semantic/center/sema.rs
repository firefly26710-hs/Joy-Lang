use crate::parser::center::par::Parser;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Node;
use crate::Semantic::scope::data::scopestruct::ScopeData;
use crate::Semantic::symbol::data::data::Symbol;

struct Semantic{
    nodes : Vec<Node>,
    edges : Vec<NodeIdx>,
    scope : Vec<ScopeData>,
    symbol: Symbol,
}

impl Semantic{
    fn init(parser : Parser) -> Self{
        let alloc_size = parser.ast.nodes.len();
        
        Self{
            nodes : parser.ast.nodes,
            edges : parser.ast.edges,
            scope : Vec::with_capacity(alloc_size),
            symbol: Symbol::init(),
        }
        
    }
}