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
    fn init(file_name : String, input_len : usize) -> Self{
	let alloc_size = input_len;
	
	Self{
	    file_name,
	    spans : Vec::with_capacity(alloc_size),
	    span_id : Vec::with_capacity(alloc_size),
	    cols    : Vec::with_capacity(alloc_size)
	}
    }


    
}




















    
