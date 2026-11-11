use std::process::exit;
use colored::Colorize;
use crate::lexer::center::lex::Lexer;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::msg_data::ParserMsgData;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::diagnostic::source::issue::{TokenIssue, ScopeIssue, ExprIssue};

impl Parser{
    pub fn output_parser_error(&self){
        for msg_data in &self.lexer.error.par_msg_data {
            let data = msg_data;
            self.build_message(data);
        }
        exit(1);
    }

    pub fn build_message(&self, msg_data : &ParserMsgData){
        let issue = &msg_data.issue;
        let line = msg_data.line.to_string().blue().bold();
        let wrong_tok_start = msg_data.token_start;
        let wrong_tok_end = msg_data.token_end;

        let span_id =msg_data.span_id;
        let span = self.lexer.error.spans[span_id];
        let span_start = span.0 as usize;
        let span_length = span.1 as usize;
        let mut sentence = &self.lexer.tokenizer.input[span_start..span_length];

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

        let file_name = &self.lexer.error.file_name;
        let error = "error".red();
        let col = (rlt_start + 1).to_string().blue().bold();


        let mut no_color_notices = String::new();
        for _ in 0..wrong_part.len() {
            no_color_notices.push_str("^");
        }
        let yellow_notices = no_color_notices.yellow();

        let info = match issue{
            ParserIssue::Token(
                TokenIssue::UnExpectedToken {
                    wrong_string, correct_string}) =>
                {
                    let mut info = "expected ".to_string();
                    info.push_str(&correct_string);
                    info.push_str(", find ");
                    info.push_str(&wrong_string);
                    info
                },

            ParserIssue::Token(TokenIssue::UnclosedDelimiter) =>
                {
                    "unclosed delimiter".to_string()
                },

            ParserIssue::Scope(
                ScopeIssue::VariableNotInScope {unknown_var_string}) =>
                {
                    let mut info = "cannot find value ".to_string();
                    info.push_str(&unknown_var_string);
                    info.push_str(" in this scope");
                    info
                },

            ParserIssue::Scope(
                ScopeIssue::FunctionNotFound { unknown_fn_string }) =>
                {
                    let mut info = "cannot find function ".to_string();
                    info.push_str(&unknown_fn_string);
                    info
                }

            ParserIssue::Scope(
                ScopeIssue::OutOfLoop {kw}) =>
                {
                    let mut info = "'".to_string();
                    info.push_str(kw.as_str());
                    info.push_str("' outside of a loop");
                    info

                }

            ParserIssue::Expr(ExprIssue::UnExpectedExpression{wrong_string}) =>
                {
                    let mut info = "expected expression, find ".to_string();
                    info.push_str(wrong_string);
                    info
                }

            ParserIssue::Expr(
                ExprIssue::LiteralOutOfRange {expected_type_string}) =>
                {
                    let mut info = "literal out of range for ".to_string();
                    info.push_str("'");
                    info.push_str(expected_type_string.as_str());
                    info.push_str("'");
                    info
                }

            ParserIssue::Expr(
                ExprIssue::MismatchesNodeType {left_node_type, right_node_type}) =>
                {
                    let mut info = "cannot use ".to_string();
                    info.push_str(&right_node_type);
                    info.push_str(" after ");
                    info.push_str(&left_node_type);
                    info
                }

            ParserIssue::Expr(
                ExprIssue::MismatchesType {expected_type, actual_type}) =>
                {
                    let mut info = "expected ".to_string();
                    info.push_str(&expected_type);
                    info.push_str(" but found ");
                    info.push_str(&actual_type);
                    info
                }

            ParserIssue::Expr(ExprIssue::ComparisonOpChained) => {
                "comparison operator cannot be chained".to_string()
            }

            ParserIssue::Expr(
                ExprIssue::MismatchUnaryType {unary_op_string, right_expr_type_string}) =>
                {
                    let mut info = "cannot apply unary operator '".to_string();
                    info.push_str(unary_op_string.as_str());
                    info.push_str("' to the type '");
                    info.push_str(right_expr_type_string);
                    info.push_str("'");
                    info
                }

        };


        println!("{file_name}:{line}:{col}: {error}: {info} ");
        println!("    {left_correct_part}{wrong_part}{right_correct_part}");
        println!("    {space}{yellow_notices}");


    }


}

#[test]
fn test_par_error(){
    let input =
        "fn main() = void {\n var x = not(1 + 1 - 1);}  ".to_string();
    let input = input + "\n\0\0\0";
    let mut lexer = Lexer::init(input, "main.joy".to_string());
    lexer.tokenize();
    // lexer.print_output();

    let mut parser = Parser::init(lexer);
    _= parser.build_tree();
    dbg!(parser.scope.var_scope_stack);
    dbg!(parser.table.var_keys);
    dbg!(parser.table.var_types);

}