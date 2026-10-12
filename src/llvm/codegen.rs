use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{FunctionValue, PointerValue};
use crate::lex::lexer::Lexer;
use crate::par::scope::Scope;
use crate::par::table::Table;
use crate::par::parser::Parser;
use crate::par::ty::Type;

pub struct Codegen<'ctx> {
    pub context             : &'ctx Context,
    pub module              : Module<'ctx>,
    pub builder             : Builder<'ctx>,
    pub parser              : Parser,
    pub functions           : Vec<FunctionValue<'ctx>>,
    pub variables           : Vec<PointerValue<'ctx>>,

    pub param_types_vec     : Vec<BasicMetadataTypeEnum<'ctx>>,
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

            param_types_vec : Vec::with_capacity(8),
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


#[test]
fn compile_test(){
    let input = "
    fn add(a : i32, b: i32) -> i32{
        return a + b;
    }


    fn main(){
        let c = add(6, 7);
        return;
    }
    ".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();

    let mut parser = Parser::init(lexer, Table::init(), Scope::init());
    parser.build_tree();
   // parser.print_node();

    let context = Context::create();
    let mut llvm = Codegen::init(&context,parser);
    llvm.compile();
    llvm.module.print_to_stderr();

}