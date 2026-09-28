pub mod lex;
pub mod par;
pub mod llvm;

use std::{env, fs};
use inkwell::context::Context;
use crate::llvm::codegen::Codegen;
use crate::lex::lexer::Lexer;
use crate::par::parser::Parser;



fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("No file in {:?}", args[0]);
    }

    let file_path = &args[1];

    let input = match fs::read_to_string(file_path){
        Ok(file_path) => file_path,
        Err(_) => panic!("Error : Failed Read the file")
    };



    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    let mut parser = Parser::init(lexer.input, lexer.tokens, lexer.lines, lexer.intervals);
    parser.build_tree();

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
