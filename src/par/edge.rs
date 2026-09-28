use crate::lex::iterval::Interval;
use crate::par::parser::Parser;

impl Parser{
    #[inline(always)]
    pub fn bt_push(&mut self ,l_idx : u32, r_idx : u32) -> Interval{
        let l_edge_idx = self.edges.len() as u32;
        self.edges.push(l_idx);
        self.edges.push(r_idx);
        let r_edge_idx = (self.edges.len() - 1) as u32;
        Interval::new(l_edge_idx, r_edge_idx)
    }
}