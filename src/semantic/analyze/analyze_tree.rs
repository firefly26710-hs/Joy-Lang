use crate::semantic::center::sema::Semantic;
use crate::semantic::diagnostic::data::issue::TypeIssue;

impl Semantic {
    pub fn analyze_tree(&mut self){
        let node = self.nodes.last().unwrap();
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let len = self.edges[start..=end].len();

        for i in 0..len {
            let stmt_idx = self.edges[start..=end][i];
            match self.analyze_function(stmt_idx){
                Ok(_) => {},
                Err(_) => self.output_type_error()
            };
        }
    }
}
