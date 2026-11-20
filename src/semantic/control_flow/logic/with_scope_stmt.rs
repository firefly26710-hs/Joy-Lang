use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::semantic::center::sema::Semantic;
use crate::semantic::scope::data::scope_kind::ScopeKind;
use crate::semantic::symbol::data::ty::Type;

impl Semantic{
    pub fn collect_for_kw_scope_data(&mut self, for_idx : NodeIdx){
        let node = &self.nodes[for_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let edge = &self.edges[start..=end];
        let len = edge.len();
        let last = len - 1;

        let ident_key = self.nodes[edge[0] as usize].key;
        self.symbol_insert_variable(
            ident_key, Type::i128, ScopeKind::Local, NONE
        );

        self.collect_scope(self.edges[start..=end][last]);

    }

    pub fn collect_while_kw_data(&mut self, while_idx : NodeIdx){
        let node = &self.nodes[while_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let edge = &self.edges[start..=end];
        let len = edge.len();
        let last = len - 1;
        self.collect_scope(self.edges[start..=end][last]);
    }

    pub fn collect_if_kw_data(&mut self, if_idx : NodeIdx){
        let node = &self.nodes[if_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let edge = &self.edges[start..=end];
        let len = edge.len();
        for i in 0..len{
            self.collect_scope(self.edges[start..=end][2 * i + 1])
        }
    }
}