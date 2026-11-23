use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;

impl Semantic{
    pub fn matches_str(&self, key_a : Key, key_b: Key) -> bool{
        let interval_a = &self.intervals[key_a as usize];
        let start_a = interval_a.start as usize;
        let end_a = interval_a.end as usize;
        let str_a = &self.input[start_a..end_a];
        
        let interval_b = &self.intervals[key_b as usize];
        let start_b = interval_b.start as usize;
        let end_b =interval_b.end as usize;
        let str_b = &self.input[start_b..end_b];
        
        str_a == str_b
        
    }
}