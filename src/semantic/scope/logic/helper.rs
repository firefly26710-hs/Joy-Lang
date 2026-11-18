use crate::lexer::tokenizer::source::token::Token;
use crate::semantic::scope::data::scope_kind::ScopeKind;

pub fn get_scope_notice(tok : &Token) -> ScopeKind{
    match tok{
        Token::VarStmt => ScopeKind::Var,
        Token::IfStmt  => ScopeKind::If,
        Token::ForStmt => ScopeKind::For,
        Token::WhileStmt=>ScopeKind::While,
        Token::RetStmt  => ScopeKind::Ret,
        Token::CtnStmt  => ScopeKind::Ctn,
        Token::BrkStmt  => ScopeKind::Brk,
        Token::CallStmt => ScopeKind::Ctn,
        Token::Fn       => ScopeKind::Fn,
        Token::PrintStmt=> ScopeKind::Print,
        _ => unreachable!()
    }
}