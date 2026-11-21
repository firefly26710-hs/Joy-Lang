use crate::lexer::tokenizer::source::token::Token;
use crate::semantic::center::sema::Semantic;

impl Semantic{
    pub fn collect_symbol_and_scope_info(&mut self){
        let node = self.nodes.last().unwrap();
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let len = self.edges[start..=end].len();
        for i in 0..len{
            self.assign_function(self.edges[start..=end][i]);
            self.collect_data(&Token::Fn,self.edges[start..=end][i], self.cur_depth );
            self.function_flow(self.edges[start..=end][i]);
        }



    }
}
