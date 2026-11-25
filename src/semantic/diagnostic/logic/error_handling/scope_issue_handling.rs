use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;
use crate::semantic::diagnostic::data::scope_issue::ScopeIssue;
use crate::semantic::diagnostic::data::scope_msg_data::ScopeMsgData;

impl Semantic{
    pub fn error_variable_not_in_this_scope(&mut self,name_key : Key){
        self.error.has_error = true;

        let span_id = self.error.span_ids[name_key as usize] as usize;
        let line = span_id + 1;

        let interval = &self.intervals[name_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let var_string = self.input[start..end].to_string();
        let issue = ScopeIssue::VariableNotInScope { var_string };

        self.error.scope_msg_data.push(
            ScopeMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );
    }

    pub fn error_function_not_found(&mut self, name_key : Key){
        self.error.has_error = true;

        let span_id = self.error.span_ids[name_key as usize] as usize;
        let line = span_id + 1;

        let interval = &self.intervals[name_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let unknown_fn_string = self.input[start..end].to_string();
        let issue = ScopeIssue::FunctionNotFound { unknown_fn_string };

        self.error.scope_msg_data.push(
            ScopeMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );
    }

    pub fn error_out_of_loop(&mut self, kw_node_idx : NodeIdx){
        self.error.has_error = true;

        let key = self.nodes[kw_node_idx as usize].key;

        let interval = &self.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let span_id = self.error.span_ids[key as usize] as usize;
        let line = span_id + 1;

        let kw = self.input[start..end].to_string();
        let issue = ScopeIssue::OutOfLoop {kw};

        self.error.scope_msg_data.push(
            ScopeMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

    }
}