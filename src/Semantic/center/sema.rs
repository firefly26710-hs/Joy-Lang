use crate::parser::tree::ast::NodeIdx;
use crate::Semantic::symbol::data::data::Symbol;

struct Semantic{
    nodes : Vec<NodeIdx>,
    edges : Vec<NodeIdx>,
    symbol: Vec<Symbol>,
}