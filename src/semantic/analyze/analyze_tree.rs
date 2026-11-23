use crate::semantic::center::sema::Semantic;
use crate::semantic::control_flow::data::stmt_kind::StmtKind;

impl Semantic{
    pub fn analyze_flow_context(&mut self){
        let len = self.flow.stmt_datas.len();

        for i in 0..len{
            let stmt_kind = &self.flow.stmt_datas[i].kind;
            match stmt_kind{
                StmtKind::VarStmt => todo!(),
                StmtKind::IfStmt  => todo!(),
                StmtKind::ForStmt => todo!(),
                StmtKind::WhileStmt=> todo!(),
                StmtKind::RetStmt => todo!(),
                StmtKind::CtnStmt => todo!(),
                StmtKind::BrkStmt => todo!(),
                StmtKind::FuncStmt=> todo!(),
                StmtKind::CallStmt => todo!(),
                StmtKind::PrintStmt => todo!(),
                StmtKind::UpdateStmt => todo!()
            }
        }
    }
}