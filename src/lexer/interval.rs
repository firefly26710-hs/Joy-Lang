pub type Start = u32;
pub type End = u32;
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub start : Start,
    pub end   : End
}

impl Interval {
    pub fn new(start : Start, end : End) -> Self{
	Interval { start, end }
    }
    
    
}
