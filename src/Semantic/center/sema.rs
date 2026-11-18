use crate::parser::tree::ast::NodeIdx;

struct Semantic{
    nodes : Vec<NodeIdx>,
    edges : Vec<NodeIdx>,
    symbol: Vec<Scope>,
}