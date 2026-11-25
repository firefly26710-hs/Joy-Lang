use crate::lexer::diagnostic::source::data::Error;
use crate::lexer::tokenizer::source::interval::Interval;
use crate::parser::center::par::Parser;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Node;
use crate::semantic::control_flow::data::control_flow::Flow;
use crate::semantic::symbol::data::function_symbol::FunctionSymbol;
use crate::semantic::symbol::data::variable_symbol::VariableSymbol;

pub struct Semantic{
    pub input     : String,
    pub nodes     : Vec<Node>,
    pub edges     : Vec<NodeIdx>,
    pub intervals : Vec<Interval>,
    pub v_symbol  : VariableSymbol,
    pub f_symbol  : FunctionSymbol,
    pub flow      : Flow,
    pub error     : Error,
    pub cur_depth : u8,

}

impl Semantic{
    pub fn init(parser : Parser) -> Self{
        let alloc_size = parser.ast.nodes.len();
        
        Self{
            input     : parser.input,
            nodes     : parser.ast.nodes,
            edges     : parser.ast.edges,
            intervals : parser.intervals, 
            flow      : Flow::init(alloc_size / 2),
            v_symbol  : VariableSymbol::init(),
            f_symbol  : FunctionSymbol::init(),
            error     : parser.error,
            cur_depth : 0,
        }
        
    }
}