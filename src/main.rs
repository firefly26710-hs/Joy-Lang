pub mod lex;
pub mod par;
pub mod llvm;

use std::env;
use std::fs::File;
use std::io::{ Read};
use std::path::Path;
use std::process::Command;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use crate::lex::lexer::Lexer;
use crate::llvm::codegen::Codegen;
use crate::par::parser::parser::Parser;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("No file in {:?}", args[0]);
    }


    let mut file =File::open(&args[1]).expect("Can't find file");
    let file_size = file.metadata().map(|m| m.len() as usize).unwrap_or(0);
    let mut input = String::with_capacity(file_size + 3);
    file.read_to_string(&mut input).expect("Failed Read to String");
    input.push_str("\0\0\0");


    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    let mut parser = Parser::init(lexer);
    parser.build_tree();

    let context = Context::create();
    let mut llvm = Codegen::init(&context, parser);
    llvm.compile();


    let engine = llvm.module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();

    unsafe {
        let main_fn: inkwell::execution_engine::JitFunction<unsafe extern "C" fn()> =
            engine.get_function("main").unwrap();

        main_fn.call();
    }

}

#[test]
fn test(){
    let input =
        "fn main() = void{
            print(String,\"Hello World\n\");
        }
    ".to_string();
    let input = input + "\0\0\0";
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    lexer.print_output();

    let mut parser = Parser::init(lexer);
    parser.build_tree();
    parser.print_node();
    dbg!(&parser.table.var_keys);
    dbg!(&parser.table.func_keys);
    dbg!(&parser.table.param_idxs);
    dbg!(&parser.table.var_types);
    let context = Context::create();
    let mut llvm = Codegen::init(&context, parser);
    llvm.compile();


    llvm.module.print_to_stderr();




}

