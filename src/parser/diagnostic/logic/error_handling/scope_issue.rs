use crate::parser::diagnostic::source::issue::ScopeIssue;
use crate::parser::diagnostic::source::msg_data::ParserMsgData;
use crate::parser::tree::node::Key;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;

impl Parser{

    pub fn var_not_in_scope(&mut self, current_key: Key) -> Result<(), ParserIssue>{
        let prev_key = self.current as Key;

        if prev_key != current_key {
            Ok(())
        }else{
            Err(self.variable_not_in_scope_issue(current_key))
        }
    }


    pub fn has_out_of_loop(&mut self) -> Result<(), ParserIssue>{
        if !self.in_loop(){
            return Err(self.outside_of_a_loop_issue());
        }

        Ok(())
    }

    pub fn fn_not_exist(&mut self, current_key : Key) -> Result<(), ParserIssue>{
        let prev_key = self.current as Key;

        if prev_key != current_key{
            Ok(())
        }else{
            Err(self.cannot_found_function_issue(current_key))
        }
    }


    pub fn variable_not_in_scope_issue(&mut self, wrong_tok_key: Key ) -> ParserIssue {
        self.lexer.error.has_error = true;
        let wrong_tok_key = wrong_tok_key as usize;

        let span_id = self.lexer.error.span_ids[wrong_tok_key] as usize;
        let line = span_id + 1;

        let interval = &self.lexer.tokenizer.intervals[wrong_tok_key];
        let start =interval.start as usize;
        let end = interval.end as usize;

        let unknown_var_string = self.lexer.tokenizer.input[start..end].to_string();

        let issue = ParserIssue::Scope(ScopeIssue::VariableNotInScope { unknown_var_string });

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

    pub fn cannot_found_function_issue(&mut self, key: Key) -> ParserIssue {
        self.lexer.error.has_error = true;
        let unknown_func_key = key as usize;

        let span_id = self.lexer.error.span_ids[unknown_func_key] as usize;
        let line = span_id + 1;
        let interval = &self.lexer.tokenizer.intervals[unknown_func_key];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let unknown_fn = self.lexer.tokenizer.input[start..end].to_string();

        let issue = ParserIssue::Scope(ScopeIssue::FunctionNotFound { unknown_fn_string: unknown_fn });

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


    pub fn outside_of_a_loop_issue(&mut self) -> ParserIssue {
        self.lexer.error.has_error = true;

        let kw_key = self.current;

        let span_id = self.lexer.error.span_ids[kw_key] as usize;
        let line = span_id + 1;
        let interval = &self.lexer.tokenizer.intervals[kw_key];
        let start = interval.start as usize;
        let end = interval.end as usize;


        let kw = self.lexer.tokenizer.input[start..end].to_string();

        let issue = ParserIssue::Scope(ScopeIssue::OutOfLoop {kw});

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
    
    
}