pub mod lex;
pub mod par;

use std::{env, fs};
use std::fs::File;
use std::io::{ Read};
use std::os::unix::fs::MetadataExt;
use crate::par::table::Table;
use crate::par::scope::Scope;
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

    let mut parser = Parser::init(lexer, Table::init(), Scope::init());


}
