type IntervalStart = u32;
type IntervalEnd = u32;
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub start : IntervalStart,
    pub end   : IntervalEnd
}

impl Interval {
    pub fn new(start : IntervalStart, end : IntervalEnd) -> Self{
	Interval { start, end }
    }
    
    
}
