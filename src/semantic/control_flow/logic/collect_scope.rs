use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::semantic::center::sema::Semantic;

impl Semantic{
    pub fn collect_scope(&mut self, scope_idx : NodeIdx){
        self.cur_depth += 1;
        let node = &self.nodes[scope_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let len = self.edges[start..=end].len();


        for i in 0..len{
            let kw_idx = self.edges[start..=end][i];
            if kw_idx == NONE {
                return;
            }
            self.collect_data(&self.nodes[kw_idx as usize].token.clone(), kw_idx, self.cur_depth);
            let stmt_tok = &self.nodes[kw_idx as usize].token.clone();
            match stmt_tok{
                Token::VarKw   => continue,
                Token::IfKw    => self.collect_if_kw_data(kw_idx),
                Token::ForKw   => self.collect_for_kw_scope_data(kw_idx),
                Token::WhileKw => self.collect_while_kw_data(kw_idx),
                Token::RetKw   => continue,
                Token::CtnKw   => continue,
                Token::BrkKw   => continue,
                Token::CallKw  => continue,
                Token::PrintKw => continue,
                Token::UpdateKw=> continue,
                //Token::Fn      => todo!(),
                _ => unreachable!()
            }
            self.cur_depth -= 1;
        }

    }
}