use inkwell::values::{BasicValueEnum, IntValue};
use crate::llvm::codegen::Codegen;
use crate::llvm::diagnostic::logic::helper::overflowed;
use crate::llvm::diagnostic::source::issue::GenIssue;
use crate::llvm::diagnostic::source::msg_data::GenMsgData;
use crate::parser::diagnostic::logic::error_handling::expr_issue::from_type_to_string;
use crate::parser::tree::ast::NodeIdx;
use crate::parser::types::typ::Type;

impl<'ctx> Codegen<'ctx>{

    pub fn check_expression_overflow(
        &mut self,
        func_name: &str,
        expr_idx : NodeIdx,
        expected_type : &Type,
        l_val : IntValue<'ctx>,
        r_val : IntValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, GenIssue>{
        let int_type = l_val.get_type();;

        let l_const = l_val.get_sign_extended_constant().unwrap() as i128;
        let r_const = r_val.get_sign_extended_constant().unwrap() as i128;

        let result_const = match func_name {
            "add" => l_const.overflowing_add(r_const).0,
            "sub" => l_const.overflowing_sub(r_const).0,
            "mul" => l_const.overflowing_mul(r_const).0,
            _ => unreachable!()
        };

        if overflowed(expected_type, result_const){
            return Err( self.expr_out_of_range(expr_idx, expected_type.clone()))
        }

        Ok(int_type.const_int(result_const as u64, false).into())




    }

    fn expr_out_of_range(&mut self, expr_idx : NodeIdx, expected_type : Type) -> GenIssue{
        self.parser.lexer.error.has_error = true;

        let (start_tok_key, end_tok_key)  = self.parser.get_min_and_max_key(expr_idx as usize);

        let span_id = self.parser.lexer.error.span_ids[start_tok_key] as usize;
        let line = span_id + 1;

        let start_interval = &self.parser.lexer.tokenizer.intervals[start_tok_key];
        let start = start_interval.start as usize;

        let end_interval = &self.parser.lexer.tokenizer.intervals[end_tok_key];
        let end = end_interval.end as usize;

        let expected_type_string = from_type_to_string(&expected_type);

        let issue = GenIssue::ExprOutOfRange {expected_type_string};

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



}

