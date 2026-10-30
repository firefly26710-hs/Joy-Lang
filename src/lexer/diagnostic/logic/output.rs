use crate::lexer::center::lex::Lexer;
use crate::lexer::diagnostic::source::issue::IssueType;
use crate::lexer::diagnostic::source::msg_data::MsgData;
use colored::Colorize;
use std::process::exit;

impl Lexer{
    pub fn output_error(&self){
        for msg_data in &self.error.msg_datas{
            let data = msg_data;
            self.build_message(data.clone());
        }
        exit(1);
    }

    pub fn build_message(&self, msg_data: MsgData){

        let issue = msg_data.issue;
        let span_id = msg_data.span_id;
        let span = self.error.spans[span_id];
        let span_start = span.0;
        let span_end = span.1;
        let line = msg_data.line.to_string().blue().bold();
        let rlt_col_start = msg_data.rlt_col_start;
        let rlt_col_end = msg_data.rlt_col_end;



        let sentence =  &self.tokenizer.input[span_start..span_end];
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

        let file_name = &self.error.file_name;
        let error = "error".red();
        let col = (rlt_col_start + 1).to_string().blue().bold();


        let mut no_color_notices = String::new();
        for _ in 0..wrong_part.len() {
            no_color_notices.push_str("^");
        }
        let yellow_notices = no_color_notices.yellow();

        let info = match issue{
            IssueType::IllegalToken => {
                let mut info = "illegal token: ".to_string();
                info.push_str(&wrong_part);
                info
            },
            IssueType::MismatchString => {
                "unterminated double quote string".to_string()
            },
            IssueType::UnExpectedToken{wrong_string, correct_string} => {
                let mut info = "expected ".to_string();
                info.push_str(&correct_string);
                info.push_str(" , find ");
                info.push_str(&wrong_string);
                info
            },
            IssueType::UnclosedDelimiter => {
                "unclosed delimiter".to_string()
            },
            IssueType::VariableNotInScope{unknown_var}=>{
                let mut info = "cannot find value ".to_string();
                info.push_str(&unknown_var);
                info.push_str(" in this scope");
                info
            },
            IssueType::LiteralOutOfRange {expected_type} =>{
                let mut info = "literal out of range for ".to_string();
                info.push_str("'");
                info.push_str(expected_type.as_str());
                info.push_str("'");
                info

            },
            IssueType::FunctionNotFound {unknown_fn} => {
                let mut info = "cannot find function ".to_string();
                info.push_str(&unknown_fn);
                info
            },
            IssueType::MismatchesNodeType {prev_node_type,curr_node_type} => {
                let mut info = "cannot use ".to_string();
                info.push_str(&curr_node_type);
                info.push_str(" after ");
                info.push_str(&prev_node_type);
                info
            }
            IssueType::MismatchesType {expected_type, actual_type} =>{
                let mut info = "expected ".to_string();
                info.push_str(&expected_type);
                info.push_str(" but found ");
                info.push_str(&actual_type);
                info
            }
            IssueType::OutOfLoop {kw} =>{
                let mut info = "'".to_string();
                info.push_str(kw.as_str());
                info.push_str("' outside of a loop");
                info
            }
            _ => unreachable!()
        };


        println!("{file_name}:{line}:{col}: {error}: {info} ");
        println!("    {left_correct_part}{wrong_part}{right_correct_part}");
        println!("    {space}{yellow_notices}");





    }
    
    
}