use crate::lexer::diagnostic::source::data::Error;
use crate::lexer::tokenizer::source::data::Tokenizer;

pub struct Lexer{
    pub tokenizer: Tokenizer,
    pub error    : Error
}

impl Lexer{

    pub fn init(input : String, file_name : String) -> Self{
        let alloc_size = input.len();

        Lexer {
            tokenizer: Tokenizer::init(input, alloc_size),
            error    : Error::init(file_name, alloc_size)
        }
    }
}