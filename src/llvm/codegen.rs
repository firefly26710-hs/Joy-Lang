use crate::lexer::tokenizer::source::interval::{End, Start};
use crate::parser::center::par::Parser;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::{Key, Node};
use crate::parser::types::logic::Type;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, PointerValue};

pub struct Codegen<'ctx> {
    pub context             : &'ctx Context,
    pub module              : Module<'ctx>,
    pub builder             : Builder<'ctx>,
    pub parser              : Parser,
    pub functions           : Vec<FunctionValue<'ctx>>,
    pub variables           : Vec<PointerValue<'ctx>>,

    pub param_types         : Vec<BasicMetadataTypeEnum<'ctx>>,
    pub call_param_types    : Vec<BasicMetadataValueEnum<'ctx>>,


    pub ctn_goto_block      : Vec<BasicBlock<'ctx>>,
    pub brk_goto_block      : Vec<BasicBlock<'ctx>>,
    pub cod_blocks          : Vec<BasicBlock<'ctx>>,
    pub body_blocks         : Vec<BasicBlock<'ctx>>,


    pub current_fn          : Option<FunctionValue<'ctx>>,
    pub ret_type            : Type
}

impl<'ctx> Codegen<'ctx>{
    pub fn init(context: &'ctx Context, parser: Parser) -> Self{
        let module = context.create_module("Joy IR");
        let builder        = context.create_builder();

        Self{
            context,
            module,
            builder,
            parser,
            functions : Vec::with_capacity(512),
            variables : Vec::with_capacity(64),

            ctn_goto_block : Vec::with_capacity(8),
            brk_goto_block : Vec::with_capacity(8),
            cod_blocks     : Vec::with_capacity(64),
            body_blocks    : Vec::with_capacity(64),


            param_types: Vec::with_capacity(16),
            call_param_types : Vec::with_capacity(16),
            current_fn : None,
            ret_type   : Type::void
        }
    }

    pub fn compile(&mut self){
        let programs_node = self.parser.ast.nodes.last().unwrap();
        let interval = &programs_node.edge_span;
        let start = interval.start as usize;
        let end = interval.end as usize;

        let len = self.parser.ast.edges[start..=end].len();
        for i in 0..len{
            self.build_function(self.parser.ast.edges[start..=end][i]);
        }

        for i in 0..len{
            self.compile_function(self.parser.ast.edges[start..=end][i]);
        }

    }

    pub fn get_edge_slice(&self, start : Start, end : End) -> &[NodeIdx]{
        &self.parser.ast.edges[start as usize..=end as usize]
    }
    
    
    pub fn get_str_interval(&self, key : Key) -> (Start, End){
        let interval = &self.parser.lexer.tokenizer.intervals[key as usize];
        (interval.start, interval.end)
    }

    pub fn get_node(&self, idx : NodeIdx) -> &Node{
        &self.parser.ast.nodes[idx as usize]
    }






}


