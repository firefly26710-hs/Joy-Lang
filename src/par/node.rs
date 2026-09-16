use crate::lex::token::Token;


#[derive(Debug, Clone, PartialEq)]
pub struct Node{
    pub token : Token,
    pub key : u32,
    pub l_edge_idx : u32,
    pub r_edge_idx : u32
}

impl Node{
    pub fn init(token : Token, key : u32, l_edge_idx : u32, r_edge_idx : u32) -> Self{
	    Node{
	        token,
	        key,
	        l_edge_idx,
	        r_edge_idx
	    }
    }
    
    
}
