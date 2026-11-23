use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::NodeIdx;
use crate::semantic::center::sema::Semantic;
use crate::semantic::scope::data::scope_kind::from_scope_kind_to_string;
use crate::semantic::symbol::data::stmt_data::StmtData;
use crate::semantic::symbol::data::stmt_kind::{from_stmt_kind_to_string, StmtKind};

impl Semantic{
    pub fn collect_stmt_data(&mut self, kw_tok : &Token, kw_idx : NodeIdx, depth : u8){
        let stmt_kind = kw_to_stmt_ty(kw_tok);
        let new_stmt_data = StmtData::new(stmt_kind, depth);
        self.flow.stmt_datas.push(new_stmt_data);
        self.flow.kw_idxs.push(kw_idx);
    }

    pub fn print_control_flow(&self){
        println!("=============== Control Flow ===============");
        println!("{:>11} | {:>11} | {:>11}", "StmtKind", "ScopeDepth", "NodeIdx");

        let len = self.flow.stmt_datas.len();
        for i in 0..len{
            let stmt_data = &self.flow.stmt_datas[i];
            let stmt_kind = from_stmt_kind_to_string(&stmt_data.kind);
            let depth = stmt_data.depth;
            let node_idx = self.flow.kw_idxs[i];
            println!("{:>10}{:>9}{:>17}", stmt_kind, depth, node_idx);
        }
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
        Token::FuncKw  => StmtKind::FuncStmt,
        Token::CallKw  => StmtKind::CallStmt,
        Token::PrintKw => StmtKind::PrintStmt,
        Token::UpdateKw=> StmtKind::UpdateStmt,
        _ => unreachable!()
    }
}