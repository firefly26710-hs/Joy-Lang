use crate::semantic::center::sema::Semantic;
use crate::semantic::symbol::data::stmt_kind::StmtKind;

impl Semantic {
    pub fn analyze_flow_context(&mut self) {
        let len = self.flow.stmt_datas.len();

        for i in 0..len {
            let stmt_kind = &self.flow.stmt_datas[i].kind;
            let depth = self.flow.stmt_datas[i].depth;
            let stmt_idx = self.flow.kw_idxs[i] as usize;
            match stmt_kind {
                StmtKind::VarStmt    => self.analyze_var(stmt_idx, depth),
                StmtKind::IfStmt     => self.analyze_if(stmt_idx, depth),
                StmtKind::ForStmt    => self.analyze_for(stmt_idx, depth),
                StmtKind::WhileStmt  => self.analyze_while(stmt_idx, depth),
                StmtKind::RetStmt    => self.analyze_return(),
                StmtKind::CtnStmt    => self.analyze_ctn(i),
                StmtKind::BrkStmt    => self.analyze_brk(i),
                StmtKind::FuncStmt   => self.analyze_function(stmt_idx, depth),
                StmtKind::CallStmt   => self.analyze_call(stmt_idx, depth),
                StmtKind::PrintStmt  => todo!(),
                StmtKind::UpdateStmt => todo!(),
            }
        }
    }
}
