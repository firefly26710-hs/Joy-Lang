pub mod lex;
pub mod par;

use std::{env, fs};
use inkwell::context::Context;
use crate::lex::lexer::Lexer;



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


}
