
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub start : u32,
    pub end   : u32
}

impl Interval {
    pub fn new(start : u32, end : u32) -> Self{
	Interval {
	    start,
	    end 
	}
    }
}
