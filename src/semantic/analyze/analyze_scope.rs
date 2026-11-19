use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::semantic::center::sema::Semantic;
use crate::semantic::diagnostic::data::issue::TypeIssue;

impl Semantic{
    pub fn analyze_scope(&mut self, scope_idx : NodeIdx) -> Result<(), TypeIssue>{
        self.cur_depth += 1;

        let node = &self.nodes[scope_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let len = self.edges[start..=end].len();



        for i in 0..len{
            let edges = &self.edges[start..=end];
            if edges[i] == NONE{
                continue
            }
            
            let stmt_token = &self.nodes[edges[i] as usize].token;
            match stmt_token{
                Token::VarKw => self.analyze_var(edges[i])?,
                Token::UpdateKw=> self.analyze_update(edges[i])?,
                Token::IfKw => todo!(),
                Token::ForKw => todo!(),
                Token::WhileKw => todo!(),
                Token::BrkKw => continue,
                Token::CtnKw => continue,
                Token::RetKw => todo!(),
                Token::CallKw => todo!(),
                Token::PrintKw => todo!(),
                Token::Scope     => self.analyze_scope(edges[i])?,
                _ => unreachable!()
            }

        }
        self.cur_depth -= 1;
        Ok(())
    }
}