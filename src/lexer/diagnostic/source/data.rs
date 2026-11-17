use crate::lexer::diagnostic::source::msg_data::{LexerMsgData};
use crate::parser::diagnostic::source::msg_data::ParserMsgData;
pub type SpanStart = usize;
pub type SpanLength = usize;
pub type ColStart = u32;
pub type ColEnd = u32;


#[derive(Debug, Clone, PartialEq)]

pub struct Error{
	pub lex_msg_data : Vec<LexerMsgData>,
	pub par_msg_data : Vec<ParserMsgData>,
    pub file_name   : String,
    pub spans       : Vec<(SpanStart, SpanLength)>,
    pub span_ids    : Vec<u32>,
	pub lbrace_stack : Vec<u32>,
	pub span_start: u32,
	pub span_id     : u32,
	pub line        : u32,
	pub has_error   : bool
}

impl Error{
    pub fn init(file_name : String, alloc_size: usize) -> Self{
	
		Self{
			lex_msg_data: Vec::with_capacity(alloc_size),
			par_msg_data : Vec::with_capacity(alloc_size),
			lbrace_stack : Vec::with_capacity(64),
	    	file_name,
	    	spans   : Vec::with_capacity(alloc_size),
	    	span_ids: Vec::with_capacity(alloc_size),

			span_start: 0,
			span_id    : 0,
			line       : 1,
			has_error  : false
		} 
	
	}


    
}




















    
