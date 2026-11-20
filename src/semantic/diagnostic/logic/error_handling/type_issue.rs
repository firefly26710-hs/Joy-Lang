use crate::parser::tree::ast::NodeIdx;
use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;
use crate::semantic::diagnostic::data::issue::TypeIssue;
use crate::semantic::diagnostic::data::type_msg_data::TypeMsgData;
use crate::semantic::symbol::data::ty::Type;

impl Semantic{
    pub fn error_between_op_literal_literal(
        &mut self,
        op_key      : Key,
        expr_idx    : NodeIdx,
        l_node_type : Type,
        r_node_type : Type
    ) -> TypeIssue{
        let (start_tok_key, end_tok_key)  = self.get_min_and_max_key(expr_idx as usize);

        let span_id = self.error.span_ids[start_tok_key] as usize;
        let line = span_id + 1;

        let start_interval = &self.intervals[start_tok_key];
        let start = start_interval.start as usize;

        let end_interval = &self.intervals[end_tok_key];
        let end = end_interval.end as usize;

        let op_string = self.from_op_to_string(op_key);
        let left_literal_type = from_type_state_to_string(&l_node_type);
        let right_literal_type = from_type_state_to_string(&r_node_type);

        let issue = TypeIssue::MismatchBetweenOpAndTwoLiterals {op_string, left_literal_type, right_literal_type};
        self.error.type_msg_data.push(
            TypeMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue

    }

    pub fn error_between_op_literal(
        &mut self,
        op_key              : Key,
        error_literal_key   : Key,
        corr_literal_type   : Type,
        error_literal_type  : Type,
    ) -> TypeIssue{

        self.error.has_error = true;

        let span_id = self.error.span_ids[error_literal_key as usize] as usize;
        let line = span_id + 1;

        let interval = &self.intervals[error_literal_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let interval = &self.intervals[op_key as usize];
        let op_start = interval.start as usize;
        let op_end = interval.end as usize;
        let op_string = self.input[op_start..op_end].to_string();
        let correct_literal_type = from_type_state_to_string(&corr_literal_type);
        let error_literal_type = from_type_state_to_string(&error_literal_type);
        let issue = TypeIssue::MismatchBetweenOpLiteral {op_string, correct_literal_type, error_literal_type};
        self.error.type_msg_data.push(
            TypeMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }


    pub fn error_between_literal_and_literal(
        &mut self,
        error_literal_key: Key,
        l_node_type      : &Type,
        r_node_type      : &Type
    ) -> TypeIssue {
        self.error.has_error = true;

        let span_id = self.error.span_ids[error_literal_key as usize] as usize;
        let line = span_id + 1;

        let interval = &self.intervals[error_literal_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let left_literal_type = from_type_state_to_string(l_node_type).to_string();
        let right_literal_type =  from_type_state_to_string(r_node_type).to_string();

        let issue = TypeIssue::MismatchBetweenLiteralLiteral { left_literal_type, right_literal_type };

        self.error.type_msg_data.push(
            TypeMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }
    pub fn error_between_identifier_literal(
        &mut self,
        op_key: Key,
        l_node_type      : &Type,
        r_node_type      : &Type
    ) -> TypeIssue {
        self.error.has_error = true;


        let span_id = self.error.span_ids[op_key as usize] as usize;
        let line = span_id + 1;

        let interval = &self.intervals[op_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let left_literal_type = from_type_state_to_string(l_node_type).to_string();
        let right_literal_type =  from_type_state_to_string(r_node_type).to_string();

        let issue = TypeIssue::MismatchBetweenLiteralLiteral { left_literal_type,  right_literal_type };

        self.error.type_msg_data.push(
            TypeMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }


    pub fn error_literal_out_of_range(
        &mut self,
        expr_idx : NodeIdx,
        expected_type : &Type,
    ) -> TypeIssue {
        self.error.has_error = true;

        let (start_tok_key, end_tok_key)  = self.get_min_and_max_key(expr_idx as usize);

        let span_id = self.error.span_ids[start_tok_key] as usize;
        let line = span_id + 1;

        let start_interval = &self.intervals[start_tok_key];
        let start = start_interval.start as usize;

        let end_interval = &self.intervals[end_tok_key];
        let end = end_interval.end as usize;


        let expected_type_string = from_type_to_string(&expected_type).to_string();
        let issue = TypeIssue::LiteralOutOfRange {expected_type_string};

        self.error.type_msg_data.push(
            TypeMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn error_mismatch_type_between_literal_literal(
        &mut self,
        error_literal_key    : Key,
        correct_literal_type : &Type,
        error_literal_type: &Type
    ) -> TypeIssue{
        self.error.has_error = true;


        let span_id = self.error.span_ids[error_literal_key as usize] as usize;
        let line = span_id + 1;

        let interval = &self.intervals[error_literal_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;

        let expected_type = from_type_state_to_string(correct_literal_type);
        let actual_type = from_type_state_to_string(error_literal_type);
        let issue = TypeIssue::MismatchType {expected_type, actual_type};

        self.error.type_msg_data.push(
            TypeMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue

    }

    pub fn error_mismatch_type_with_expression(
        &mut self,
        expr_idx      : NodeIdx,
        declared_type : &Type,
        expr_type     : &Type,
    ) -> TypeIssue{
        self.error.has_error = true;

        let (start_tok_key, end_tok_key)  = self.get_min_and_max_key(expr_idx as usize);

        let span_id = self.error.span_ids[start_tok_key] as usize;
        let line = span_id + 1;

        let start_interval = &self.intervals[start_tok_key];
        let start = start_interval.start as usize;

        let end_interval = &self.intervals[end_tok_key];
        let end = end_interval.end as usize;

        let expected_type = from_type_to_string(declared_type).to_string();
        let actual_type =  from_type_to_string(expr_type).to_string();


        let issue = TypeIssue::MismatchType {expected_type, actual_type};
        self.error.type_msg_data.push(
            TypeMsgData::new(
                issue.clone(),
                line,
                start,
                end,
                span_id
            )
        );

        issue
    }

    pub fn error_unary_mismatch_type(
        &mut self,
        op_key : Key,
        error_expr_idx : NodeIdx,
        error_expr_type :Type
    ) -> TypeIssue{
        self.error.has_error = true;

        let (start_tok_key, end_tok_key)  = self.get_min_and_max_key(error_expr_idx as usize);

        let span_id = self.error.span_ids[start_tok_key] as usize;
        let line = span_id + 1;

        let start_interval = &self.intervals[start_tok_key];
        let start = start_interval.start as usize;

        let end_interval = &self.intervals[end_tok_key];
        let end = end_interval.end as usize;

        let interval = &self.intervals[op_key as usize];
        let op_start = interval.start as usize;
        let op_end = interval.end as usize;
        
        let unary_op = self.input[op_start..op_end].to_string();
        let right_type = from_type_state_to_string(&error_expr_type);
        let issue = TypeIssue::UnaryMismatchType {unary_op, right_type};

        self.error.type_msg_data.push(
            TypeMsgData::new(
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



fn from_type_to_string(ty : &Type) -> String{
    match ty{
        Type::i8   => "i8",
        Type::i16  => "i16",
        Type::i32  => "i32",
        Type::i64  => "i64",
        Type::i128 => "i128",
        Type::f32  => "f32",
        Type::f64  => "f64",
        Type::bool => "bool",
        _ => unreachable!()
    }.to_string()
}

fn from_type_state_to_string(ty : &Type) -> String{
    match ty{
        Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128 => {
            "{integer}"
        },
        Type::f32 | Type::f64 => {
            "{float}"
        },
        Type::bool => {
           "bool"
        }
        _ => unreachable!()
    }.to_string()
}
