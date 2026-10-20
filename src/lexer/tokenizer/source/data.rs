use crate::lexer::tokenizer::source::interval::Interval;
use crate::lexer::tokenizer::source::token::Token;

#[derive(Debug, Clone, PartialEq)]

pub struct Tokenizer {
	pub input    : String,
    pub intervals: Vec<Interval>,
    pub tokens   : Vec<Token>,
    pub lines    : Vec<u32>,
    pub start    : u32,
    pub current  : u32,
    pub line     : u32,
}

impl Tokenizer {
	pub fn init(input : String,alloc_size : usize) -> Self{

		Tokenizer {
			input,
			intervals: Vec::with_capacity(alloc_size),
			tokens   : Vec::with_capacity(alloc_size),
			lines     : Vec::with_capacity(alloc_size),
			start    : 0,
			current  : 0,
			line     : 0

		}
	}


    
} 








