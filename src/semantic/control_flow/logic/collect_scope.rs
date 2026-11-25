use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::semantic::center::sema::Semantic;

impl Semantic{
    pub fn collect_scope(&mut self, kw_idx: NodeIdx, depth : u8){
        let node = &self.nodes[kw_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let len = self.edges[start..=end].len();
        let last = len - 1;

        let scope_node = &self.nodes[self.edges[start..=end][last] as usize];
        let scope_span =  &scope_node.edge_span;
        let s_start = scope_span.start as usize;
        let s_end = scope_span.end as usize;

        let stmt_count = self.edges[s_start..=s_end].len();



        for i in 0..stmt_count{
            let kw_idx = self.edges[s_start..=s_end][i];
            if kw_idx == NONE {
                return;
            }
            self.collect_stmt_data(&self.nodes[kw_idx as usize].token.clone(), kw_idx, depth);
            let stmt_tok = &self.nodes[kw_idx as usize].token.clone();
            match stmt_tok{
                Token::VarKw   => continue,
                Token::IfKw    => self.collect_branch_scope(kw_idx, depth + 1),
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