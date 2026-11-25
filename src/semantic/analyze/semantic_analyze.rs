use crate::parser::tree::ast::NONE;
use crate::semantic::center::sema::Semantic;
use crate::semantic::symbol::data::stmt_kind::StmtKind;

impl Semantic {
    pub fn analyze(&mut self) {
        self.collect_control_flow();
        // self.print_control_flow();
        self.analyze_flow_context();
        self.check_ret_pos();
        if self.error.has_error {
            self.output_semantic_error();
        }
    }

    pub fn check_ret_pos(&self){
        let len = self.flow.stmt_datas.len();

        let mut cur_func_key = NONE;

        let mut l = 0;
        let mut r = 0;

        let mut branch_count = 0;
        let mut return_count = 0;

        while l < r{
            if &self.flow.stmt_datas[l].kind == &StmtKind::FuncStmt{
                let kw_idx = self.flow.kw_idxs[l];
                let node = &self.nodes[kw_idx as usize];
                let span = &node.edge_span;
                let start = span.start as usize;
                let end = span.end as usize;
                cur_func_key = self.edges[start..=end][0];
            }

        }


    }
}
