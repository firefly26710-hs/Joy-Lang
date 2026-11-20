use crate::lexer::tokenizer::source::interval::{End, Interval, Start};
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::semantic::center::sema::Semantic;
use crate::semantic::control_flow::data::stmt_kind::StmtKind;
use crate::semantic::diagnostic::data::issue::TypeIssue;
use crate::semantic::scope::data::scope_kind::ScopeKind;

impl Semantic{
    pub fn analyze_function(&mut self, func_idx : NodeIdx) -> Result<(), TypeIssue>{
        let node = &self.nodes[func_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let edges = &self.edges[start..=end];
        let last = edges.len() - 1;
        let scope_idx = edges[last];

        self.analyze_scope(scope_idx)?;
        Ok(())
    }
}