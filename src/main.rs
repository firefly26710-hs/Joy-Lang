pub mod lexer;
pub mod parser;
pub mod semantic;

//use crate::llvm::codegen::Codegen;
use crate::lexer::center::lex::Lexer;
use crate::parser::center::par::Parser;
use crate::semantic::center::sema::Semantic;
//use inkwell::OptimizationLevel;
//use inkwell::context::Context;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No file in {:?}", args[0]);
    }

    let mut file = File::open(&args[1]).expect("Can't find file");
    let file_size = file.metadata().map(|m| m.len() as usize).unwrap_or(0);
    let mut input = String::with_capacity(file_size + 3);
    file.read_to_string(&mut input)
        .expect("Failed Read to String");
    input.push_str("\n\0\0\0");

    let mut lexer = Lexer::init(input, args[1].to_string());
    lexer.tokenize();
    // lexer.print_output();
    let mut parser = Parser::init(lexer);
    parser.build_tree();

    let mut semantic = Semantic::init(parser);
    semantic.analyze()
    // let context = Context::create();
    //let mut llvm = Codegen::init(&context, parser);
    // llvm.compile();
    //
    //
    // let engine = llvm.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    //
    // unsafe {
    //     let main_fn: inkwell::execution_engine::JitFunction<unsafe extern "C" fn()> =
    //         engine.get_function("main").unwrap();
    //
    //     main_fn.call();
    // }
}

#[test]
fn diag_test() {
    let input = "
    fn main() = void{
        var x = 5;
        var x = 6;
        if x == 5{
            return;
        }   
    }
    "
    .to_string();
    let input = input + "\n\0\0\0";
    let mut lexer = Lexer::init(input, "main.joy".to_string());
    lexer.tokenize();
    // lexer.print_output();
    let mut parser = Parser::init(lexer);
    parser.build_tree();
    //parser.print_node();

    let mut semantic = Semantic::init(parser);
    semantic.analyze();
    //semantic.print_control_flow();
    //semantic.print_variable_symbol();
    //  let context = Context::create();
    //  let mut llvm = Codegen::init(&context, parser);
    //  llvm.compile();
    //  llvm.module.print_to_stderr();
}
