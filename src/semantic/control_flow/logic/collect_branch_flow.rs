use crate::lexer::tokenizer::source::token::Token;
use crate::semantic::symbol::data::stmt_kind::StmtKind;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::semantic::center::sema::Semantic;

impl Semantic{
    pub fn collect_branch_scope(&mut self, kw_idx: NodeIdx, depth : u8){
        let node = &self.nodes[kw_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let len = self.edges[start..=end].len();


        let branch_count = len / 2;

        for i in 0..branch_count{
            let expr_idx = self.edges[start..=end][2 * i];
            let stmt_idx = self.edges[start..=end][1 + 2 * i];
            if i > 0 && expr_idx != NONE{ self.collect_branch_stmt_data(StmtKind::ElifStmt, NONE, depth - 1); }
            else if expr_idx == NONE{self.collect_branch_stmt_data(StmtKind::ElseStmt, NONE, depth - 1);}

            let stmt_interval = &self.nodes[stmt_idx as usize].edge_span;
            let start = stmt_interval.start as usize;
            let end = stmt_interval.end as usize;
            let stmt_count = self.edges[start..=end].len();
            for j in 0..stmt_count{
                let kw_idx = self.edges[start..=end][j];
                if kw_idx == NONE{continue};
                self.collect_stmt_data(&self.nodes[self.edges[start..=end][j] as usize].token.clone(), kw_idx, depth);
                match &self.nodes[self.edges[start..=end][j] as usize].token{
                    Token::VarKw   => continue,
                    Token::IfKw    => self.collect_scope(kw_idx, depth + 1),
                    Token::ForKw   => self.collect_scope(kw_idx, depth + 1),
                    Token::WhileKw => self.collect_scope(kw_idx, depth + 1),
                    Token::RetKw   => continue,
                    Token::CtnKw   => continue,
                    Token::BrkKw   => continue,
                    Token::CallKw  => continue,
                    Token::PrintKw => continue,
                    Token::UpdateKw=> continue,
                    //Token::Fn      => todo!(),
                    _ => unreachable!()
                }
            }

        }

    }




}