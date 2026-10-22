use std::process::exit;
use crate::lexer::center::lex::Lexer;

impl Lexer{
    pub fn output_error(&self){
        for msg_data in &self.error.msg_datas{
            let data = msg_data;
            self.build_message(data.clone());
        }
        exit(1);
    }
}