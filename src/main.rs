pub mod lex;
pub mod par;

pub mod llvm;

use std::env;
use std::fs::File;
use std::io::{ Read};
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use crate::par::table::Table;
use crate::par::scope::Scope;
use crate::par::parser::Parser;
use crate::lex::lexer::Lexer;
use crate::llvm::codegen::Codegen;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("No file in {:?}", args[0]);
    }


    let mut file =File::open(&args[1]).expect("Can't find file");
    let file_size = file.metadata().map(|m| m.len() as usize).unwrap_or(0);
    let mut input = String::with_capacity(file_size);
    file.read_to_string(&mut input).expect("Failed Read to String");


    let mut lexer = Lexer::init(input);
    lexer.tokenize();

    let mut parser = Parser::init(lexer, Table::init(), Scope::init());
    parser.build_tree();

    let context = Context::create();
    let mut llvm = Codegen::init(&context, parser);
    llvm.compile();

    let engine = llvm.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

    unsafe {
        let main_fn: inkwell::execution_engine::JitFunction<unsafe extern "C" fn()> =
            engine.get_function("main").unwrap();

        main_fn.call();
    }



}

#[test]
fn str(){
    let input = "
    fn fib(n : i32) -> i32{
        if n < 2{
            return 1;
        }
        return fib(n - 1) + fib(n - 2);
    }
    fn main(){
        let a = \"Hello World!\";
        print(a);
        return;
    }
    ";
    let mut lexer = Lexer::init(input.to_string());
    lexer.tokenize();

    let mut parser = Parser::init(lexer, Table::init(), Scope::init());
    parser.build_tree();

    let context = Context::create();
    let mut llvm = Codegen::init(&context, parser);
    llvm.compile();
    llvm.module.print_to_stderr();
    let engine = llvm.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

    unsafe {
        let main_fn: inkwell::execution_engine::JitFunction<unsafe extern "C" fn()> =
            engine.get_function("main").unwrap();

        main_fn.call();
    }

}

#[test]
fn compile_test(){
    let input = "
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }

    fn check_range(val: i32) -> i32 {
        if val < 10 {
            return 1;
        } elif val < 50 {
            return 2;
        } else {
            return 3;
        }
        return 4;
    }

    fn factorial(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        return n * factorial(n - 1);
    }

    fn main() {
        let x = add(15, 35);
        let status = check_range(x);
        let fact = factorial(5);

        let total = 0;
        for i in [1 -> 10 | 1]{
            break;
        }

        return;
    }
    ".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
   // lexer.print_output();

    let mut parser = Parser::init(lexer, Table::init(), Scope::init());
    parser.build_tree();
    //parser.print_node();

    let context = Context::create();
    let mut llvm = Codegen::init(&context, parser);
    llvm.compile();

    llvm.module.print_to_stderr();
    llvm.module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();

}
