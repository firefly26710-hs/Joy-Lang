use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::NodeIdx;
use crate::semantic::center::sema::Semantic;
use crate::semantic::control_flow::data::stmt_data::StmtData;
use crate::semantic::control_flow::data::stmt_kind::StmtKind;

impl Semantic{
    pub fn collect_data(&mut self, kw_tok : &Token, kw_idx : NodeIdx){
        let stmt_kind = kw_to_stmt_ty(kw_tok);
        let new_stmt_data = StmtData::new(stmt_kind, self.cur_depth);
        self.flow.stmt_datas.push(new_stmt_data);
        self.flow.kw_idxs.push(kw_idx);
    }
}

fn kw_to_stmt_ty(kw_tok : &Token) -> StmtKind{
    match kw_tok{
        Token::VarKw   => StmtKind::VarStmt,
        Token::IfKw    => StmtKind::IfStmt,
        Token::ForKw   => StmtKind::ForStmt,
        Token::WhileKw => StmtKind::WhileStmt,
        Token::RetKw   => StmtKind::RetStmt,
        Token::CtnKw   => StmtKind::CtnStmt,
        Token::BrkKw   => StmtKind::BrkStmt,
        Token::Fn      => StmtKind::FuncStmt,
        Token::CallKw  => StmtKind::CallStmt,
        Token::PrintKw => StmtKind::PrintStmt,
        Token::UpdateKw=> StmtKind::UpdateStmt,
        _ => unreachable!()
    }
}