pub mod lex;
pub mod par;
pub mod llvm;

use crate::lex::lexer::Lexer;
use crate::par::parser::Parser;



fn main() {
    let input = "
    fn fib(a : i32){
           return;
    } 

    fn main(){
         let n = 5;
         fib(n);
    }



".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    let mut parser = Parser::init(lexer.input, lexer.tokens, lexer.lines, lexer.intervals);
    parser.build_tree();
}
