use crate::parser::tree::ast::NONE;
use crate::semantic::center::sema::Semantic;
use crate::semantic::scope::data::scope_kind::ScopeKind;
use crate::semantic::symbol::data::stmt_kind::StmtKind;
use crate::semantic::symbol::data::ty::Type;
impl Semantic {
    pub fn analyze_for(&mut self, stmt_idx: usize, depth: u8) {
        let node = &self.nodes[stmt_idx];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let name_node_idx = self.edges[start..=end][0] as usize;
        let var_key = self.nodes[name_node_idx].key;
        self.symbol_insert_variable(var_key, Type::i128, ScopeKind::Local, NONE, depth + 1);

        let start_expr_idx = self.edges[start..=end][1];
        self.type_check(&Type::i128, start_expr_idx, depth);

        let end_expr_idx = self.edges[start..=end][2];
        self.type_check(&Type::i128, end_expr_idx, depth);

        let step_expr_idx = self.edges[start..=end][3];
        self.type_check(&Type::i128, step_expr_idx, depth);
    }


    pub fn analyze_while(&mut self, kw_idx: usize, depth: u8) {
        let node = &self.nodes[kw_idx];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;

        let expr_idx = self.edges[start..=end][0];
        self.type_check(&Type::bool, expr_idx, depth);
    }

    pub fn analyze_ctn(&self, stmt_data_idx : usize) {
        let ctn_stmt_data = &self.flow.stmt_datas[stmt_data_idx];
        let ctn_stmt_depth = ctn_stmt_data.depth;

        for i in (0..stmt_data_idx).rev(){
            let cur_stmt_data = &self.flow.stmt_datas[i];
            let cur_stmt_kind = &cur_stmt_data.kind;
            let cur_stmt_depth = cur_stmt_data.depth;

            if matches!(cur_stmt_kind, StmtKind::ForStmt | StmtKind::WhileStmt) && ctn_stmt_depth > cur_stmt_depth{
                return;
            }

        }

        panic!("`continue` outside of a loop");
    }
    pub fn analyze_brk(&self, stmt_data_idx : usize) {
        let brk_stmt_data = &self.flow.stmt_datas[stmt_data_idx];
        let brk_stmt_depth = brk_stmt_data.depth;

        for i in (0..stmt_data_idx).rev(){
            let cur_stmt_data = &self.flow.stmt_datas[i];
            let cur_stmt_kind = &cur_stmt_data.kind;
            let cur_stmt_depth = cur_stmt_data.depth;

            if matches!(cur_stmt_kind, StmtKind::ForStmt | StmtKind::WhileStmt) && brk_stmt_depth > cur_stmt_depth{
                return;
            }

        }

        panic!("`break` outside of a loop");
    }
}
