use crate::lexer::center::lex::Lexer;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::scope::data::Scope;
use crate::parser::stmt::data::stmt_data::Stmt;
use crate::parser::tree::ast::{AST, NONE, NodeIdx};
use crate::parser::types::data::Table;


pub struct Parser{

    //source
    pub ast : AST,
    pub lexer : Lexer,

    pub table : Table,
    pub scope : Scope,
    pub stmt  : Stmt,
    

    pub current : usize,

    

}

impl Parser{
    pub fn init(lexer : Lexer) -> Self{

        Self{
            lexer,
            ast   : AST::init(),
            table : Table::init(),
            scope : Scope::init(),
            stmt  : Stmt::init(),

            current : 0,

        }
    }

    pub fn build_tree(&mut self) -> Result<NodeIdx, ParserIssue>{
        while self.lexer.tokenizer.tokens[self.current] != Token::EOF{
            let function_idx =  self.parse_function()?;
            self.stmt.programs.push(function_idx);
           
        }

        let start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.programs.drain(..));
        let end = self.ast.edge_end();

       
        self.push_node(Token::Program, NONE, start, end);

        
        Ok(0)
    }

    



    
   





   


    



}




