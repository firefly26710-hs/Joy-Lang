use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, PointerValue};
use crate::par::parser::Parser;
use crate::par::ty::Type;

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
        let programs_node = self.parser.nodes.last().unwrap();
        let interval = &programs_node.idxs;
        let start = interval.start as usize;
        let end = interval.end as usize;

        let func_idxs = &self.parser.edges[start..=end];
        let len = func_idxs.len();
        let ptr = func_idxs.as_ptr();

        for i in 0..len{
            self.build_function(unsafe{*ptr.add(i)});
        }

        for i in 0..len{
            self.compile_function(unsafe{*ptr.add(i)});
        }


    }








}


