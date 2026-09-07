#[derive(Debug, Clone, PartialEq)]
pub struct Iterval{
    start : u32,
    end   : u32
}

impl Iterval{
    pub fn init(start : u32, end : u32) -> Self{
        Iterval{
            start,
            end
        }
    }

}

