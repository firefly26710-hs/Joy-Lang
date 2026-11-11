use crate::llvm::codegen::Codegen;
use crate::llvm::diagnostic::source::issue::GenIssue;
use crate::llvm::diagnostic::source::msg_data::GenMsgData;
use crate::parser::diagnostic::logic::error_handling::expr_issue::from_type_to_string;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::types::typ::Type;

impl<'ctx> Codegen<'ctx>{


    pub fn divided_by_zero_issue(&mut self, expr_idx : NodeIdx) -> GenIssue{
        self.parser.lexer.error.has_error = true;

        let (start_tok_key, end_tok_key)  = self.parser.get_min_and_max_key(expr_idx as usize);

        let span_id = self.parser.lexer.error.span_ids[start_tok_key] as usize;
        let line = span_id + 1;

        let start_interval = &self.parser.lexer.tokenizer.intervals[start_tok_key];
        let start = start_interval.start as usize;

        let end_interval = &self.parser.lexer.tokenizer.intervals[end_tok_key];
        let end = end_interval.end as usize;



        let issue = GenIssue::DivideByZero;

        self.parser.lexer.error.gen_msg_data.push(
            GenMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn literal_out_of_range(&mut self, expr_idx : NodeIdx, expected_type : Type) -> GenIssue{
        self.parser.lexer.error.has_error = true;

        let key = self.parser.ast.nodes[expr_idx as usize].key as usize;

        let span_id = self.parser.lexer.error.span_ids[key] as usize;
        let line = span_id + 1;

        let interval = &self.parser.lexer.tokenizer.intervals[key];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let expected_type_string = from_type_to_string(&expected_type);

        let issue = GenIssue::LiteralOutOfRange {expected_type_string};

        self.parser.lexer.error.gen_msg_data.push(
            GenMsgData::new(
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

