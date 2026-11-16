use crate::parser::tree::node::Key;
use crate::parser::diagnostic::source::issue::FunctionIssue;
use crate::parser::diagnostic::source::msg_data::ParserMsgData;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::logic::error_handling::expr_issue::from_type_to_string;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::types::typ::Type;

impl Parser{
    pub fn mismatch_ret_type_with_expr_issue(
        &mut self,
        expr_idx : NodeIdx,
        expected_type : &Type,
        actual_type : &Type
    )-> ParserIssue{
        self.lexer.error.has_error = true;

        let (start_tok_key, end_tok_key)  = self.get_min_and_max_key(expr_idx as usize);

        let span_id = self.lexer.error.span_ids[start_tok_key] as usize;
        let line = span_id + 1;

        let start_interval = &self.lexer.tokenizer.intervals[start_tok_key];
        let start = start_interval.start as usize;

        let end_interval = &self.lexer.tokenizer.intervals[end_tok_key];
        let end = end_interval.end as usize;
        
        let ret_type_string = from_type_to_string(expected_type);
        let actual_type_string = from_type_to_string(actual_type);
        
        let issue = ParserIssue::Func(FunctionIssue::MismatchesRetTypeWithExpr {ret_type_string, actual_type_string});

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
    
    pub fn mismatch_ret_type_without_expr_issue(&mut self, tok_key: usize, ret_type : &Type) -> ParserIssue{
        self.lexer.error.has_error = true;
        
        let span_id = self.lexer.error.span_ids[tok_key] as usize;
        let interval = &self.lexer.tokenizer.intervals[tok_key];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let ret_type_string = from_type_to_string(ret_type);
        let issue = ParserIssue::Func(FunctionIssue::MismatchesRetTypeWithOutExpr {ret_type_string});

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

    pub fn no_return_void_but_no_return_issue(
        &mut self,
        func_name_key : Key,
        ret_type : &Type
    ) -> ParserIssue{
        self.lexer.error.has_error = true;

        let span_id = self.lexer.error.span_ids[func_name_key as usize] as usize;
        let interval = &self.lexer.tokenizer.intervals[func_name_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let ret_type_string = from_type_to_string(ret_type);
        let issue = ParserIssue::Func(FunctionIssue::NotRetVoidButNoReturn {ret_type_string});

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
}
