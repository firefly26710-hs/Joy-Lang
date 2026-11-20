use crate::parser::tree::ast::NONE;
use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;

impl Semantic{
    pub fn get_min_and_max_key(&mut self, rhs : usize) -> (usize, usize){
        let mut max_key = Key::MIN;
        let mut min_key = Key::MAX;
        self.dfs(rhs, &mut max_key, &mut min_key);

        (min_key as usize, max_key as usize)
    }

    pub fn dfs(&mut self, expr_idx: usize, max_key : &mut Key, min_key : &mut Key){
        let node = &self.nodes[expr_idx];
        let mut key = node.key;
        let span = &node.edge_span;
        let span_start = span.start;
        let span_end = span.end;



        *max_key = *max_key.max(&mut key);
        *min_key = *min_key.min(&mut key);

        if span_start == NONE && span_end == NONE{
            return;
        }


        let lhs = self.edges[span_start as usize..=span_end as usize][0];
        let rhs = self.edges[span_start as usize..=span_end as usize][1];


        if lhs != NONE {
            self.dfs(lhs as usize, max_key, min_key);
        }
        self.dfs(rhs as usize, max_key, min_key);



    }

    pub fn from_op_to_string(&self, op_key : Key) -> String{
        let interval = &self.intervals[op_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let str = &self.input[start..end];
        match str{
            "+" => "add",
            "-" => "subtract",
            "*" => "multiply",
            "/" => "divide",
            "and" => "logic and",
            "or"  => "logic or",
            _ => unreachable!()
        }.to_string()
    }


}