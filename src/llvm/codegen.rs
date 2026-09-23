use std::usize;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{PointerValue, FunctionValue, BasicValueEnum};
use crate::lex::lexer::Lexer;
use crate::par::parser::{Parser};
use crate::lex::iterval::Interval;
use crate::par::node::Node;
use crate::par::ty::Type;


pub const NONE : usize = usize::MAX;


pub struct Codegen<'ctx>{
    pub context             : &'ctx Context,
    pub module              : Module<'ctx>,
    pub builder             : Builder<'ctx>,
    pub input               : String,
    pub intervals           : Vec<Interval>,
    pub key_checker         : Vec<u32>,
    pub types               : Vec<Type>,
    pub func_assigner       : Vec<u32>,
    pub param_type_idxs     : Vec<Interval>,
    pub ret_types           : Vec<Type>,
    pub functions           : Vec<FunctionValue<'ctx>>,
    pub variables           : Vec<PointerValue<'ctx>>,
    pub current_fn          : Option<FunctionValue<'ctx>>,
    pub ret_type            : Type,
    pub ctn_goto_block      : Vec<BasicBlock<'ctx>>,
    pub brk_goto_block      : Vec<BasicBlock<'ctx>>,
    pub edges               : Vec<u32>,
    pub nodes               : Vec<Node>,
}

impl<'ctx> Codegen<'ctx>{
    fn new(
        context : &'ctx Context,
        input            : String,
        intervals        : Vec<Interval>,
        key_checker      : Vec<u32>,
        types            : Vec<Type>,
        module_name      : &str,
        func_assigner    : Vec<u32>,
        param_type_idxs  : Vec<Interval>,
        ret_types        : Vec<Type>,
        edges            : Vec<u32>,
        nodes            : Vec<Node>,
    ) -> Self{
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        Codegen{
            context,
            module,
            builder,
            input,
            key_checker,
            intervals,
            types,
            func_assigner,
            param_type_idxs,
            ret_types,
            variables : Vec::new(),
            functions : Vec::new(),
            current_fn : None,
            ret_type   : Type::Void,
            ctn_goto_block : Vec::new(),
            brk_goto_block : Vec::new(),
            edges,
            nodes
        }
    }

    pub fn compile(&mut self){
        let program_node = self.nodes.last().unwrap();
        let l_edge_idx = program_node.l_edge_idx as usize;
        let r_edge_idx = program_node.r_edge_idx as usize;
        let func_idxs = self.edges[l_edge_idx..=r_edge_idx].to_vec();

        for func_idx in &func_idxs {
            self.build_function(*func_idx);
        }

        for func_idx in func_idxs{
            self.compile_function(func_idx);
        }

    }



    fn build_function(&mut self, func_idx : u32){
        let func_node = &self.nodes[func_idx as usize];

        let l_edge_idx = func_node.l_edge_idx as usize;
        let r_edge_idx = func_node.r_edge_idx as usize;

        //function name
        let name_idx = self.edges[l_edge_idx] as usize;
        let name_node = &self.nodes[name_idx];
        let name_key = name_node.key;
        let name_interval = &self.intervals[name_key as usize];
        let start = name_interval.start as usize;
        let end = name_interval.end as usize;
        let name_str = &self.input[start..end];

        //function param
        let mut current = 0;
        while name_key != self.func_assigner[current]{
            current += 1;

        }
        let param_idxs = &self.param_type_idxs[current];
        let l_type_idx = param_idxs.start as usize;
        let r_type_idx = param_idxs.end as usize;

        let has_param = l_edge_idx + 1 != r_edge_idx &&
            l_type_idx != NONE && r_type_idx != NONE;

        let param_types_vec : Vec<BasicMetadataTypeEnum> = if has_param{
            self.types[l_type_idx..=r_type_idx].iter().map(|t|
                match t{
                    Type::i32  => self.context.i32_type().into(),
                    Type::i64  => self.context.i64_type().into(),
                    Type::f32  => self.context.f32_type().into(),
                    Type::f64  => self.context.f64_type().into(),
                    Type::bool => self.context.bool_type().into(),
                    _ => unreachable!()
                }
            ).collect()
        }else {
            Vec::new()
        };

        //function return type
        let ret_type = &self.ret_types[current];

        //creat function
        let func  = match ret_type{
            Type::i32 =>{
                let ret_type = self.context.i32_type();
                let fn_type = ret_type.fn_type(&param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            },
            Type::i64 =>{
                let ret_type = self.context.i64_type();
                let fn_type = ret_type.fn_type(&param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            },
            Type::f32 =>{
                let ret_type = self.context.f32_type();
                let fn_type = ret_type.fn_type(&param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            },
            Type::f64 =>{
                let ret_type = self.context.f64_type();
                let fn_type = ret_type.fn_type(&param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            }
            Type::bool =>{
                let ret_type = self.context.bool_type();
                let fn_type = ret_type.fn_type(&param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            },
            Type::Void =>{
                let ret_type = self.context.void_type();
                let fn_type = ret_type.fn_type(&param_types_vec, false);
                self.module.add_function(name_str, fn_type, None)
            },
            _ => unreachable!()
        };

        if has_param{
            let param_idxs = self.edges[l_edge_idx + 1..r_edge_idx].to_vec();
            for (i, param_idx) in param_idxs.into_iter().enumerate(){
                let param_node = &self.nodes[param_idx as usize];
                let param_key = param_node.key;
                let interval = &self.intervals[param_key as usize];
                let start = interval.start as usize;
                let end = interval.end as usize;
                let param_name_str = &self.input[start..end];

                func.get_nth_param(i as u32).unwrap().set_name(param_name_str);

            }
        }
        self.functions.push(func);

    }



}
#[test]
pub fn small_test(){
    let input = "
   fn main(){
     let x = 5;
     let y = x;
     return;
   }



   ".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    lexer.print_output();
    let mut parser = Parser::init(lexer.input, lexer.tokens, lexer.lines, lexer.intervals);
    parser.build_tree();
    parser.print_node();
    parser.print_scope();


    let context = Context::create();
    let mut code_gen = Codegen::new(
        &context,
        parser.input,
        parser.intervals,
        parser.key_checker,
        parser.types,
        "joyc",
        parser.func_assign,
        parser.param_type_idxs,
        parser.ret_types,
        parser.edges,
        parser.nodes
    );

    code_gen.compile();
    code_gen.module.print_to_stderr();

    code_gen.module.print_to_file("test.ll").unwrap();
}

#[test]
pub fn llvm_test(){
    let input = "
fn fib(n : i32) -> i32{
     if(n < 2){
        return 0;
     }
     return fib(n - 1) + fib(n - 2);
}

fn main(){
    let n = 5;
    let sum = fib(n);
}
".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
   // lexer.print_output();
    let mut parser = Parser::init(lexer.input, lexer.tokens, lexer.lines, lexer.intervals);
    parser.build_tree();
    parser.print_node();


    let context = Context::create();
    let mut code_gen = Codegen::new(
        &context,
        parser.input,
        parser.intervals,
        parser.key_checker,
        parser.types,
        "joyc",
        parser.func_assign,
        parser.param_type_idxs,
        parser.ret_types,
        parser.edges,
        parser.nodes
    );

    code_gen.compile();
    code_gen.module.print_to_stderr();

    code_gen.module.print_to_file("output.ll").unwrap();


}
