use std::process::exit;
use colored::Colorize;
use crate::semantic::center::sema::Semantic;
use crate::semantic::diagnostic::data::issue::TypeIssue;
use crate::semantic::diagnostic::data::type_msg_data::TypeMsgData;

impl Semantic{
    pub fn output_type_error(&self){
        for msg_data in &self.error.type_msg_data {
            let data = msg_data;
            self.build_message(data);
        }
        exit(1);
    }

    pub fn build_message(&self, msg_data : &TypeMsgData){
        let issue = &msg_data.issue;
        let line = msg_data.line.to_string().blue().bold();
        let wrong_tok_start = msg_data.token_start;
        let wrong_tok_end = msg_data.token_end;

        let span_id =msg_data.span_id;
        let span = self.error.spans[span_id];
        let span_start = span.0 as usize;
        let span_length = span.1 as usize;
        let mut sentence = &self.input[span_start..span_length];

        if sentence.contains("\n"){
            let length = sentence.len();
            sentence = &sentence[..length - 1];
        }

        let rlt_start = wrong_tok_start - span_start;
        let rlt_end = wrong_tok_end - span_start;


        let left_correct_part = &sentence[0..rlt_start];
        let wrong_part = &sentence[rlt_start..rlt_end].red();
        let right_correct_part = &sentence[rlt_end..];

        let mut space = String::new();
        for _ in 0..left_correct_part.len(){
            space.push_str(" ");
        }

        let file_name = &self
            .error.file_name;
        let error = "error".red();
        let col = (rlt_start + 1).to_string().blue().bold();

        let mut no_color_notices = "^".to_string();
        for _ in 0..wrong_part.len() - 1 {
            no_color_notices.push_str("~");
        }
        let yellow_notices = no_color_notices.yellow();

        let info = match issue{
            TypeIssue::DifferenceConstLiteral {
                left_literal_type, right_literal_type} => {
                format!("cannot use {right_literal_type} after {left_literal_type}")
            },
            TypeIssue::LiteralOutOfRange {expected_type_string} => {
                format!("literal out of range for {expected_type_string}")
            },
            TypeIssue::MismatchType {
                expected_type, actual_type} => {
                format!("expected {expected_type}, but found {actual_type}")
            },
            _ => todo!()

        };


        println!("{file_name}:{line}:{col}: {error}: {info} ");
        println!("    {line}|  {left_correct_part}{wrong_part}{right_correct_part}");
        println!("     |  {space}{yellow_notices}");



    }
}