use crate::lexer::tokenizer::source::token::Token;
use crate::parser::diagnostic::source::issue::{ExprIssue, ParserIssue};
use crate::parser::diagnostic::source::msg_data::ParserMsgData;
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;
use crate::parser::center::par::Parser;

impl Parser{
    pub fn unexpected_expression_issue(&mut self) -> ParserIssue {
        self.lexer.error.has_error = true;

        let wrong_tok_key = self.current;

        let span_id = self.lexer.error.span_ids[wrong_tok_key] as usize;
        let interval = &self.lexer.tokenizer.intervals[wrong_tok_key];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let wrong_string = self.lexer.tokenizer.input[start..end].to_string();

        let issue = ParserIssue::Expr(ExprIssue::UnExpectedExpression{wrong_string});

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                span_id + 1,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn mismatch_type_issue(
        &mut self,
        expr_start_key : Key,
        expected_type : &Type,
        actual_type : &Type
    ) -> ParserIssue {
        self.lexer.error.has_error = true;

        let actual_type_key = expr_start_key as usize;

        let span_id = self.lexer.error.span_ids[actual_type_key] as usize;
        let line = span_id + 1;

        let interval = &self.lexer.tokenizer.intervals[actual_type_key];
        let start =interval.start as usize;
        let end = interval.end as usize;

        let expected_type = from_type_to_string(expected_type).to_string();
        let actual_type =  from_type_to_string(actual_type).to_string();

        let issue = ParserIssue::Expr(ExprIssue::MismatchesType {expected_type, actual_type});

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                span_id + 1,
                start,
                end,
                span_id
            )
        );

        issue
    }



    pub fn type_out_of_range(
        &mut self,
        key : Key,
        expected_type : &Type,
    ) -> ParserIssue {
        self.lexer.error.has_error = true;

        let wrong_literal_key = key as usize;
        let span_id = self.lexer.error.span_ids[wrong_literal_key] as usize;
        let line = span_id + 1;
        let interval = &self.lexer.tokenizer.intervals[wrong_literal_key];
        let start = interval.start as usize;
        let end = interval.end as usize;


        let expected_type_string = from_type_to_string(&expected_type).to_string();
        let issue = ParserIssue::Expr(ExprIssue::LiteralOutOfRange { expected_type_string});

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn comparison_op_chained_issue(&mut self, key : Key) -> ParserIssue {
        self.lexer.error.has_error = true;

        dbg!(key);
        let second_comp_op_key = key as usize;

        let span_id = self.lexer.error.span_ids[second_comp_op_key] as usize;
        let line = span_id + 1;
        let interval = &self.lexer.tokenizer.intervals[second_comp_op_key];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let issue = ParserIssue::Expr(ExprIssue::ComparisonOpChained);

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn mismatch_node_type_issue(
        &mut self,
        left_type: &Type,
        right_type: &Type,
        right_expr_key: usize
    ) -> ParserIssue {

        self.lexer.error.has_error = true;
        let right_expr_key = right_expr_key;

        let span_id = self.lexer.error.span_ids[right_expr_key] as usize;
        let line = span_id + 1;
        let interval = &self.lexer.tokenizer.intervals[right_expr_key];
        let start = interval.start as usize;
        let end = interval.end as usize;



        let left_node_type = match left_type {
            Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128
            => "integer",
            Type::f32 | Type::f64 => "float",
            Type::bool => "bool",
            _ => panic!("unknown node ty")
        }.to_string();

        let right_node_type = match right_type {
            Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128
            => "integer",
            Type::f32 | Type::f64 => "float",
            Type::bool => "bool",
            _ => panic!("unknown node ty")
        }.to_string();

        let issue = ParserIssue::Expr(ExprIssue::MismatchesNodeType {
            left_node_type, right_node_type
        });

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue

    }
    
    pub fn mismatch_expr_type_in_unary_issue(&mut self, unary_key : Key, child_expr_key : Key, child_expr_type : Type) -> ParserIssue{
        self.lexer.error.has_error = true;
        
        let unary_key = unary_key as usize;
        let child_expr_key = child_expr_key as usize;
        
        let span_id = self.lexer.error.span_ids[child_expr_key] as usize;
        let line = span_id + 1;
        
        let interval = &self.lexer.tokenizer.intervals[child_expr_key];
        let start = interval.start as usize;
        let end = interval.end as usize;
        
        let unary_op_string = match &self.lexer.tokenizer.tokens[unary_key] {
            Token::Sub => "-",
            Token::CmpNot => "not",
            _ => unreachable!()
        }.to_string();
        
        let right_expr_type_string = from_type_to_string(&child_expr_type);
        
        let issue = ParserIssue::Expr(ExprIssue::MismatchUnaryType {
            unary_op_string, right_expr_type_string
        });

        self.lexer.error.par_msg_data.push(
            ParserMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );
        
        
        issue
        
        
    }







    pub fn out_of_this_stmt(&mut self){
        self.eat();
        while !is_stmt(self.peek()){
            if matches!(self.peek(), Token::Rbrace) | matches!(self.peek(), Token::Scolon){
                self.eat();
                break;
            }

            self.eat();
        }


    }
    
    
}

fn is_stmt(tok : &Token) -> bool{
    matches!(tok,
        Token::EOF|
        Token::Rbrace|
        Token::Val|
        Token::Var |
        Token::If  |
        Token::Elif|
        Token::Else|
        Token::For |
        Token::While|
        Token::Ctn |
        Token::Brk |
        Token::Ret |
        Token::Print
    )
}

pub fn from_type_to_string(ty : &Type) -> String{
    let str = match ty {
        Type::i8  => "i8",
        Type::i16 => "i16",
        Type::i32 => "i32",
        Type::i64 => "i64",
        Type::i128=> "i128",
        Type::f32 => "f32",
        Type::f64 => "f64",
        Type::bool => "bool",
        Type::void => "void"
    };
    
    str.to_string()
}