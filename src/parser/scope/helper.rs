use crate::parser::tree::node::Key;
use crate::parser::center::par::Parser;

impl Parser{
    pub fn is_matches_string(&self, key1 : Key, key2 : Key) ->bool{
        self.look_string(key1) == self.look_string(key2)
    }

    pub fn look_string(&self, key : Key) -> &str{
        let interval = &self.lexer.tokenizer.intervals[key as usize];
        let slice_start = interval.start as usize;
        let slice_end = (interval.end - 1) as usize;

        &self.lexer.tokenizer.input[slice_start..=slice_end]
    }
    
    
    
    
}