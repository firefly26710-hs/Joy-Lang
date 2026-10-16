pub mod lex;
pub mod par;


use std::env;
use std::fs::File;
use std::io::{ Read};
use crate::par::parser::Parser;
use crate::lex::lexer::Lexer;

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
    let mut patser = Parser::init(lexer);



}

#[test]
fn test(){
    let input = "fn main() -> i32{}".to_string();
    let input = input + "\0\0\0";
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    lexer.print_output();

    let mut parser = Parser::init(lexer);
    parser.build_tree();
    parser.print_node();
}

