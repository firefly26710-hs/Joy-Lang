use crate::semantic::center::sema::Semantic;

impl Semantic {
    pub fn analyze_tree(&self) {
        let program_node = self.nodes.last().unwrap();
        let edges = self.get_edges(program_node);
        let len = edges.len();

        for i in 0..len {
            let stmt_idx = edges[i];
            self.analyze_function(stmt_idx);
        }
    }
}
