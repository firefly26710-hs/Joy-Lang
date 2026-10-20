use std::process::exit;
use crate::lexer::center::lex::Lexer;
use colored::Colorize;
use crate::lexer::diagnostic::logic::issue::IssueType;
use crate::lexer::diagnostic::source::msg_data::MsgData;

impl Lexer{

    pub fn output_error(&self){
        for msg_data in &self.error.msg_datas{
            let data = msg_data;
            self.build_message(data.clone());
        }
        exit(1);
    }
    pub fn collection_illegal_issue(&mut self){
        self.error.has_error = true;

        let rlt_col_start = self.tokenizer.current - self.error.span_start - 1;
        let rlt_col_end =  self.tokenizer.current - self.error.span_start;

        let line = self.error.line;

        self.error.msg_datas.push(
            MsgData::new(
                IssueType::IllegalToken,
                line as usize,
                rlt_col_start as usize,
                rlt_col_end as usize,
                self.error.span_id as usize
            )
        );


    }

    pub fn build_message(&self, msg_data: MsgData){

        let issue = msg_data.issue;
        let span_id = msg_data.span_id;
        let span = self.error.spans[span_id];
        let span_start = span.0;
        let span_end = span.1;
        let line = msg_data.line;
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
        let col = rlt_col_start + 1;


        let mut no_color_notices = String::new();
        for _ in 0..wrong_part.len() {
            no_color_notices.push_str("^");
        }
        let yellow_notices = no_color_notices.yellow();

        let info = match issue{
            IssueType::IllegalToken => {
                let mut info = "illegal token: ".to_string();
                info.push_str(wrong_part);
                info
            },
            _ => unreachable!()
        };


        println!("{file_name}:{line}:{col}: {error}: {info} ");
        println!("    {left_correct_part}{wrong_part}{right_correct_part}");
        println!("    {space}{yellow_notices}");





    }
}


#[test]
fn test_error(){
    let input =
        "fn main() @ = void{\n    print(String, \"Hello World\");\n}
    ".to_string();
    let input = input + "\0\0\0";
    let mut lexer = Lexer::init(input, "main.joy".to_string());
    lexer.tokenize();
}

