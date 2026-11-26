use std::sync::atomic::ATOMIC_USIZE_INIT;

use crate::parser::tree::ast::NONE;
use crate::semantic::center::sema::Semantic;
use crate::semantic::symbol::data::stmt_kind::StmtKind;

impl Semantic {
    pub fn analyze(&mut self) {
        self.collect_control_flow();
        self.print_control_flow();
        self.analyze_flow_context();
        self.check_ret_pos();
        if self.error.has_error {
            self.output_semantic_error();
        }
    }

    pub fn check_ret_pos(&self) {
        let len = self.flow.stmt_datas.len();

        let mut cur_func_key = NONE;

        let mut l = 0;
        let mut r = 1;

        if len == 1 {
            let kw_idx = self.flow.kw_idxs[l];
            let node = &self.flow.kw_idxs[l];
            let span = &node.edge_span();
            let start = span.start as usize;
            let end = span.end as usize;
            let name_key = self.nodes[self.edges[start..=end][0] as usize].key;
            let ret_key = self.look_up_ret_type(name_key);
            if ret_key != Type::void {
                panic!("Mismatch Return");
            }
            return;
        }

        let mut branch_count = 0;
        let mut return_count = 0;

        while r < len && l < r {
            if &self.flow.stmt_datas[l].kind == &StmtKind::FuncStmt {
                let kw_idx = self.flow.kw_idxs[l];
                let node = &self.nodes[kw_idx as usize];
                let span = &node.edge_span;
                let start = span.start as usize;
                let end = span.end as usize;
                let cur_func_idx = self.edges[start..=end][0];
                cur_func_key = self.nodes[cur_func_idx as usize].key;
            }

            let mut r_dpeth = self.flow.stmt_datas[r].depth;
            while r < len && self.flow.stmt_datas[r].depth == r_depth {
                r += 1;
            }

            if self.flow.stmt_datas[r].depth > 1 {
                for _ in 0..(r - 1) {
                    l += 1;
                }

                if &self.flow.stmt_datas[l].kind == &StmtKind::IfStmt {}
            }
        }
    }
}
