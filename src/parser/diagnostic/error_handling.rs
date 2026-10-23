use crate::parser::types::logic::{is_bool_type, is_flt_type, is_int_type, is_string_type};
use crate::lexer::diagnostic::source::issue::IssueType;
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::center::par::Parser;
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;

impl Parser{
    pub fn unexpected_token(&mut self, expected_tok : &Token) -> Result<(), IssueType>{
        let current_tok = self.peek();

        if current_tok == expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.collect_unexpected_token_issue(&expected_tok))
        }
    }

    pub fn unclosed_delimiter(&mut self, expected_tok : &Token) -> Result<(), IssueType>{
        let current_tok = self.peek();


        if current_tok == expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.collect_unclosed_delimiter_token_issue())
        }
    }

    pub fn var_not_in_scope(&mut self, current_key: Key) -> Result<(), IssueType>{
        let prev_key = self.current as Key;

        if prev_key != current_key {
            Ok(())
        }else{
            Err(self.collect_variable_not_in_scope_issue(current_key))
        }
    }
    
    pub fn fn_not_exist(&mut self, current_key : Key) -> Result<(), IssueType>{
        let prev_key = self.current as Key;
        
        if prev_key != current_key{
            Ok(())
        }else{
            Err(self.collect_cannot_found_function_issue(current_key))
        }
    }

    pub fn mismatch_prev_node_type(
        &mut self,
        prev_node_type : &Token,
        curr_node_type : &Token
    ) -> Result<(), IssueType>{
        Err(self.collect_mismatch_node_type_issue(prev_node_type, curr_node_type))
    }

    pub fn matches_type_in_multi_var(&mut self, expected_type : &Type) -> Result<(), IssueType>{
        let actual_type = &self.stmt.called_var_types;

        if actual_type == &Type::void{
            self.stmt.called_var_types = expected_type.clone();
            Ok(())
        }else if expected_type != actual_type{
            Err(self.collect_mismatch_type_issue(expected_type, &actual_type.clone()))
        }else{
            return Ok(())
        }

    }



    pub fn matches_type(&mut self, expected_type : &Type) -> Result<Type, IssueType>{

        let expr_type = self.get_expr_type();


       let issue = match (expected_type, expr_type){
           (Type::void, Type::i8 | Type::i16 | Type::i32 )   => Ok(Type::i32),
           (Type::void, Type::i64 ) => Ok(Type::i64),
           (Type::void, Type::i128) => Ok(Type::i128),
           (Type::void, Type::f32 | Type::f64) => Ok(Type::f64),
           (Type::void, Type::bool) => Ok(Type::bool),
           (Type::void, Type::String) => Ok(Type::String),

           (Type::i8, Type::i8) => Ok(Type::i8),
           (Type::i8, Type::i16 | Type::i32 | Type::i64 | Type::i128)
           => Err(self.collect_type_out_of_range(self.stmt.max_value_key, &expected_type)),

           (Type::i16, Type::i8 | Type::i16) => Ok(Type::i16),
           (Type::i16, Type::i32 | Type::i64 | Type::i128)
           => Err(self.collect_type_out_of_range(self.stmt.max_value_key, &expected_type)),


           (Type::i32, Type::i8 | Type::i16 | Type::i32) => Ok(Type::i32),
           (Type::i32, Type::i64 | Type::i128)
           => Err(self.collect_type_out_of_range(self.stmt.max_value_key, &expected_type)),


           (Type::i64, Type::i8 | Type::i16 | Type::i32 | Type::i64) => Ok(Type::i64),
           (Type::i64, Type::i128)
           => Err(self.collect_type_out_of_range(self.stmt.max_value_key, &expected_type)),


           (Type::i128, Type::i8 | Type::i16 | Type::i32 | Type::i64 | Type::i128)
           => Ok(Type::i128),

           (Type::f32, Type::f32) => Ok(Type::f32),
           (Type::f32, Type::f64)
           => Err(self.collect_type_out_of_range(self.stmt.max_value_key, &expected_type)),


           (Type::f64, Type::f32 | Type::f64) => Ok(Type::f64),

           (Type::bool, Type::bool) => Ok(Type::bool),
           (Type::String, Type::String) => Ok(Type::bool),

           (expect, expr)
           if is_int_type(expect) && !is_int_type(&expr) ||
               is_flt_type(expect) && !is_flt_type(&expr) ||
               is_bool_type(expect) && !is_bool_type(&expr) ||
               is_string_type(expect) && !is_string_type(&expr)
           => Err(self.collect_mismatch_type_issue(expect, &expr)),



           (_, _) => todo!(),
       };

        issue
    }





}





