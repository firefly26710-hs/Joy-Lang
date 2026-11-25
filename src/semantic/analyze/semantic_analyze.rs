use crate::parser::tree::ast::NONE;
use crate::semantic::center::sema::Semantic;
use crate::semantic::symbol::data::stmt_kind::StmtKind;
use crate::semantic::symbol::data::ty::Type;

impl Semantic {
    pub fn analyze(&mut self) {
        self.collect_control_flow();
        if self.error.has_error { self.output_semantic_error(); }
        self.analyze_flow_context();
        self.print_control_flow();
        self.check_return_pos();
        if self.error.has_error { self.output_semantic_error(); }
    }


    pub fn check_return_pos(&self){
        let len = self.flow.stmt_datas.len();
        let mut cur_func_key= NONE;
        for i in 0..len{
            let kind = &self.flow.stmt_datas[i].kind;
            let depth = self.flow.stmt_datas[i].depth;
            if matches!(kind, StmtKind::FuncStmt){
                let node_idx = self.flow.kw_idxs[0] as usize;
                let node = &self.nodes[node_idx];
                let span = &node.edge_span;
                let start = span.start as usize;
                let end = span.end as usize;
                cur_func_key = self.nodes[self.edges[start..=end][0] as usize].key;
            }

            if matches!(kind, StmtKind::IfStmt){

            }

            if matches!(kind, StmtKind::RetStmt) && depth == 1{
                if i + 1 == len{continue}

                if i + 1 < len && &self.flow.stmt_datas[i + 1].kind == &StmtKind::FuncStmt{
                    continue;
                }else{
                    panic!("Mismatch Return");
                }
            }
        }
    }


}
