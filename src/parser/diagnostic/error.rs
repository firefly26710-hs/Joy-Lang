pub type SpanStart = u32;
pub type SpanLength = u32;
pub type ColStart = u32;
pub type ColEnd = u32;



pub struct Error{
    file_name : String,
    spans     : Vec<(SpanStart, SpanLength)>,
    span_id   : Vec<usize>,
    cols      : Vec<(ColStart, ColEnd)>,
}

impl Error{
    pub fn init(file_name : String, alloc : usize) -> Self{
	
		Self{
	    	file_name,
	    	spans   : Vec::with_capacity(alloc),
	    	span_id : Vec::with_capacity(alloc),
	    	cols    : Vec::with_capacity(alloc)
		} 
	
	}


    
}




















    
