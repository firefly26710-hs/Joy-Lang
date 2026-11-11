use crate::parser::center::par::Parser;
use crate::parser::tree::ast::{NONE};
use crate::parser::tree::node::Key;
impl Parser{
    pub fn get_min_and_max_key(&mut self, rhs : usize) -> (usize, usize){
        let mut max_key = Key::MIN;
        let mut min_key = Key::MAX;

         self.dfs(rhs, &mut max_key, &mut min_key);

        dbg!(max_key);
        dbg!(min_key);
        (min_key as usize, max_key as usize)
    }

    pub fn dfs(&mut self, expr_idx: usize, max_key : &mut Key, min_key : &mut Key){

        let node = &self.ast.nodes[expr_idx];
        let mut key = node.key;
        dbg!(key);
        let span = &node.edge_span;
        let span_start = span.start;
        let span_end = span.end;



            *max_key = *max_key.max(&mut key);
            *min_key = *min_key.min(&mut key);

        if span_start == NONE && span_end == NONE{
            return;
        }

        let lhs = self.ast.edges[span_start as usize..=span_end as usize][0] as usize;
        let rhs = self.ast.edges[span_start as usize..=span_end as usize][1] as usize;


        self.dfs(lhs, max_key, min_key);
        self.dfs(rhs, max_key, min_key);



    }

}

