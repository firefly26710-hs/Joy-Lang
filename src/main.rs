pub mod lex;
pub mod par;

use crate::lex::lexer::Lexer;
use crate::par::parser::Parser;



fn main() {
    let input = "1 + 2 + 3".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    let mut parser = Parser::init(lexer.input, lexer.tokens, lexer.lines, lexer.itervals);
    parser.expression(0.0);
    parser.print_output()
}
