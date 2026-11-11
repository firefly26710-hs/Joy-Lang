use std::process::exit;
use colored::Colorize;
use crate::llvm::codegen::Codegen;
use crate::llvm::diagnostic::source::issue::GenIssue;
use crate::llvm::diagnostic::source::msg_data::GenMsgData;

impl<'ctx> Codegen<'ctx>{
    pub fn output_error(&self){
        for msg_data in &self.parser.lexer.error.gen_msg_data {
            let data = msg_data;
            self.build_message(data);
        }
        exit(1);
    }

    pub fn build_message(&self, msg_data : &GenMsgData){
        let issue = &msg_data.issue;
        let span_id = msg_data.span_id;
        let span = self.parser.lexer.error.spans[span_id];
        let span_start = span.0;
        let span_end = span.1;
        let line = msg_data.line.to_string().blue().bold();
        let rlt_col_start = msg_data.token_start;
        let rlt_col_end = msg_data.token_end;



        let sentence =  &self.parser.lexer.tokenizer.input[span_start..span_end];
        let sentence_end = sentence.len();


        let left_correct_part = &sentence[0..rlt_col_start];
        let wrong_part = &sentence[rlt_col_start..rlt_col_end].red();
        let right_correct_part =  if sentence.ends_with("\n"){
            &sentence[rlt_col_end..sentence_end - 1]
        }else{
            &sentence[rlt_col_end..sentence_end]
        };

        let mut space = String::new();
        for _ in 0..left_correct_part.len(){
            space.push_str(" ");
        }

        let file_name = &self.parser.lexer.error.file_name;
        let error = "error".red();
        let col = (rlt_col_start + 1).to_string().blue().bold();

        let mut no_color_notices = String::new();
        for _ in 0..wrong_part.len() {
            no_color_notices.push_str("^");
        }
        let yellow_notices = no_color_notices.yellow();

        let info = match issue{
            GenIssue::DivideByZero => {
                "cannot divide zero".to_string()
            }
            GenIssue::LiteralOutOfRange {expected_type_string} => {
                let mut info = "literal out of range for ".to_string();
                info.push_str("'");
                info.push_str(expected_type_string.as_str());
                info.push_str("'");
                info
            }
        };


        println!("{file_name}:{line}:{col}: {error}: {info} ");
        println!("    {left_correct_part}{wrong_part}{right_correct_part}");
        println!("    {space}{yellow_notices}");
    }




}