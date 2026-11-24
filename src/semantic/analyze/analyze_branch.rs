use crate::{semantic::center::sema::Semantic, semantic::symbol::data::ty::Type};

impl Semantic {
    pub fn analyze_if(&mut self, kw_idx: usize, depth: u8) {
        let node = &self.nodes[kw_idx];
        let span = &node.edge_span;

        let start = span.start as usize;
        let end = span.end as usize;

        let len = self.edges[start..end].len();
        for i in 0..(len / 2) {
            let expr_idx = self.edges[start..=end][2 * i];
            self.type_check(&Type::bool, expr_idx, depth);
        }
    }

   
}
