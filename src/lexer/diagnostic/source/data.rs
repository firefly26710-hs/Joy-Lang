pub type SpanStart = u32;
pub type SpanLength = u32;
pub type ColStart = u32;
pub type ColEnd = u32;


#[derive(Debug, Clone, PartialEq)]

pub struct Error{
    file_name   : String,
    pub spans   : Vec<(SpanStart, SpanLength)>,
	pub lines   : Vec<u32>,
    pub span_ids: Vec<u32>,

	pub spans_start : u32,
	pub span_id     : u32,
	pub line        : u32,
}

impl Error{
    pub fn init(file_name : String, alloc_size: usize) -> Self{
	
		Self{
	    	file_name,
	    	spans   : Vec::with_capacity(alloc_size),
	    	span_ids: Vec::with_capacity(alloc_size),
			lines   : Vec::with_capacity(alloc_size),

			spans_start: 0,
			span_id    : 0,
			line       : 1
		} 
	
	}


    
}




















    
