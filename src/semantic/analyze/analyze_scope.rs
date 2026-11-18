use crate::semantic::scope::data::scope_data::ScopeData;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::NodeIdx;
use crate::semantic::center::sema::Semantic;
use crate::semantic::scope::data::scope_kind::ScopeKind;
use crate::semantic::scope::logic::helper::get_scope_notice;

impl Semantic{
    pub fn analyze_scope(&mut self, scope_idx : NodeIdx, parent_scope_kind : &ScopeKind){
        self.cur_depth += 1;
        let node = &self.nodes[scope_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;


        let len = self.edges[start..=end].len();


        for i in 0..len{
            let edges = &self.edges[start..=end];
            let stmt_token = &self.nodes[edges[i] as usize].token;
            self.stmt_scope.push(ScopeData::new(
                get_scope_notice(stmt_token),
                self.cur_depth
            ));
        }

        for i in 0..len{
            let edges = &self.edges[start..=end];
            let stmt_token = &self.nodes[edges[i] as usize].token;

            match stmt_token{
                Token::VarStmt   => self.analyze_var(edges[i], parent_scope_kind),
                Token::IfStmt    => todo!(),
                Token::ForStmt   => todo!(),
                Token::WhileStmt => todo!(),
                Token::BrkStmt   => todo!(),
                Token::CtnStmt   => todo!(),
                Token::RetStmt   => todo!(),
                Token::CallStmt  => todo!(),
                Token::PrintStmt => todo!(),
                Token::Scope     => todo!(),
                _ => unreachable!()
            }

        }

    }
}