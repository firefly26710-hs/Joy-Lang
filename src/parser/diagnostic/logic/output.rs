use std::process::exit;
use colored::Colorize;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::msg_data::ParserMsgData;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::diagnostic::source::issue::{TokenIssue, ScopeIssue, ExprIssue, FunctionIssue};

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

        let mut no_color_notices = "^".to_string();
        for _ in 0..wrong_part.len() - 1 {
            no_color_notices.push_str("~");
        }
        let yellow_notices = no_color_notices.yellow();

        let info = match issue{
            ParserIssue::Token(
                TokenIssue::UnExpectedToken {
                    wrong_string, correct_string}
            ) => format!("expected {:?}, find {:?}", correct_string, wrong_string),


            ParserIssue::Token(TokenIssue::UnclosedDelimiter)
            => "unclosed delimiter".to_string(),

            ParserIssue::Scope(
                ScopeIssue::VariableNotInScope {unknown_var_string}
            ) => format!("cannot find value {unknown_var_string} in this scope"),

            ParserIssue::Scope(
                ScopeIssue::FunctionNotFound { unknown_fn_string }
            ) => format!("cannot find function {unknown_fn_string}"),

            ParserIssue::Scope(
                ScopeIssue::OutOfLoop {kw}
            ) => format!("{kw} outside of a loop"),

            ParserIssue::Expr(ExprIssue::UnExpectedExpression{wrong_string}
            ) => format!("expected expression, find {wrong_string}"),
            
            ParserIssue::Expr(
                ExprIssue::LiteralOutOfRange {expected_type_string}
            ) => format!("literal out of range for {expected_type_string}"),
            
            ParserIssue::Expr(
                ExprIssue::MismatchesExprType { left_expr_type_string: left_node_type, right_expr_type_string: right_node_type }
            ) => format!("cannot use {right_node_type} after {left_node_type}"),
            
            ParserIssue::Expr(
                ExprIssue::MismatchesType {expected_type, actual_type}
            ) => format!("expected {expected_type}, but found {actual_type}"),
            
            ParserIssue::Expr(ExprIssue::ComparisonOpChained) => {
                "comparison operator cannot be chained".to_string()
            }

            ParserIssue::Expr(
                ExprIssue::MismatchUnaryType {unary_op_string, right_expr_type_string}
            ) => format!("cannot apply unary operator {unary_op_string} to the type {right_expr_type_string}"),
            
            ParserIssue::Expr(
                ExprIssue::BinaryOpMismatches {expr_type_string}
            ) => format!("expected bool, find {expr_type_string}"),
            
            ParserIssue::Func(
                FunctionIssue::MismatchesRetTypeWithExpr {ret_type_string, actual_type_string}
            ) => format!("expected return {ret_type_string}, find {actual_type_string}"),
            
            ParserIssue::Func(
                FunctionIssue::MismatchesRetTypeWithOutExpr {ret_type_string}
            ) => format!("expected return {ret_type_string}, find void"),

            ParserIssue::Func(
                FunctionIssue::NotRetVoidButNoReturn {ret_type_string}
            ) => format!("expected return {ret_type_string}, but no any return value")
        };


        println!("{file_name}:{line}:{col}: {error}: {info} ");
        println!("    {line}|  {left_correct_part}{wrong_part}{right_correct_part}");
        println!("     |  {space}{yellow_notices}");



    }


}
