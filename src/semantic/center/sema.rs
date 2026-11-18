use crate::lexer::tokenizer::source::interval::Interval;
use crate::parser::center::par::Parser;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Node;
use crate::semantic::scope::data::scope_data::{ScopeData};
use crate::semantic::symbol::data::function_symbol::FunctionSymbol;
use crate::semantic::symbol::data::variable_symbol::VariableSymbol;

pub struct Semantic{
    pub input     : String,
    pub nodes     : Vec<Node>,
    pub edges     : Vec<NodeIdx>,
    pub intervals : Vec<Interval>,
    pub stmt_scope: Vec<ScopeData>,
    pub v_symbol  : VariableSymbol,
    pub f_symbol  : FunctionSymbol,
    pub cur_depth : u8,
    
}

impl Semantic{
    pub fn init(parser : Parser) -> Self{
        let alloc_size = parser.ast.nodes.len();
        
        Self{
            input     : parser.input,
            nodes     : parser.ast.nodes,
            edges     : parser.ast.edges,
            intervals: parser.intervals,
            stmt_scope: Vec::with_capacity(alloc_size),
            v_symbol  : VariableSymbol::init(),
            f_symbol  : FunctionSymbol::init(),
            cur_depth : 0
        }
        
    }
}