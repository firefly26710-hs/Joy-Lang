use crate::lexer::center::lex::Lexer;
use crate::lexer::diagnostic::source::data::Error;
use crate::lexer::tokenizer::source::interval::Interval;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::stmt::data::stmt_data::Stmt;
use crate::parser::tree::ast::{AST, NONE};

pub struct Parser{
    pub input : String,
    pub ast : AST,
    pub intervals : Vec<Interval>,
    pub tokens : Vec<Token>,
    pub stmt  : Stmt,
    pub current : usize,

    pub error: Error

}

impl Parser{
    pub fn init(lexer : Lexer) -> Self{

        Self{
            input : lexer.tokenizer.input,
            tokens : lexer.tokenizer.tokens,
            intervals : lexer.tokenizer.intervals,
            ast   : AST::init(),
            stmt  : Stmt::init(),
            current : 0,

            error : lexer.error
        }
    }

    pub fn build_tree(&mut self){
        while self.tokens[self.current] != Token::EOF{
              match self.parse_function(){
                Ok(function_idx) =>  self.stmt.programs.push(function_idx),
                Err(_) => {
                    self.stmt.recorder_init();
                    while self.peek() != &Token::EOF{
                        if self.peek() == &Token::Fn{
                            break;
                        }
                        self.eat();
                    }
                }
            };

           
        }

        let start = self.ast.edge_start();
        self.ast.edges.extend(self.stmt.programs.drain(..));
        let end = self.ast.edge_end();
        if self.error.has_error {
            self.output_parser_error();
        }
       
        self.push_node(Token::Program, NONE, start, end);


    }

    



    
   





   


    



}




