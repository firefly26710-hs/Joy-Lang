pub type SpanStart = u32;
pub type SpanLength = u32;
pub type ColStart = u32;
pub type ColEnd = u32;


#[derive(Debug, Clone, PartialEq)]

pub struct Error{
    file_name : String,
    pub spans     : Vec<(SpanStart, SpanLength)>,
    pub span_ids: Vec<usize>,
    cols      : Vec<(ColStart, ColEnd)>,
	
	pub span_id: usize,
	pub spans_start : u32
}

impl Error{
    pub fn init(file_name : String, alloc : usize) -> Self{
	
		Self{
	    	file_name,
	    	spans   : Vec::with_capacity(alloc),
	    	span_ids: Vec::with_capacity(alloc),
	    	cols    : Vec::with_capacity(alloc),
			span_id: 0,
			spans_start : 0
		} 
	
	}


    
}




















    
