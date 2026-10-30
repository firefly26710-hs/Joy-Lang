use crate::parser::types::logic::{is_bool_type, is_flt_type, is_int_type, is_string_type, is_two_types_in_same_set, left_bigger_than_right};
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
            Err(self.unexpected_token_issue(&expected_tok))
        }
    }

    pub fn unclosed_delimiter(&mut self, expected_tok : &Token) -> Result<(), IssueType>{
        let current_tok = self.peek();


        if current_tok == expected_tok{
            self.eat();
            Ok(())
        }else{
            Err(self.unclosed_delimiter_token_issue())
        }
    }

    pub fn var_not_in_scope(&mut self, current_key: Key) -> Result<(), IssueType>{
        let prev_key = self.current as Key;

        if prev_key != current_key {
            Ok(())
        }else{
            Err(self.variable_not_in_scope_issue(current_key))
        }
    }
    
    
    pub fn has_out_of_loop(&mut self) -> Result<(), IssueType>{
        if !self.is_in_loop(){
            return Err(self.outside_of_a_loop_issue());
        }
        
        Ok(())
    }
    
    pub fn fn_not_exist(&mut self, current_key : Key) -> Result<(), IssueType>{
        let prev_key = self.current as Key;
        
        if prev_key != current_key{
            Ok(())
        }else{
            Err(self.cannot_found_function_issue(current_key))
        }
    }

    pub fn matches_type(&mut self, expected_type : &Type) -> Result<Type, IssueType>{
        let expr_type = &self.get_expr_type();
        if !is_two_types_in_same_set(expr_type, expected_type){
           return Err( self.mismatch_type_issue(self.stmt.expr_start_key, expected_type, expr_type));
        }
        
        if matches!(expected_type, Type::void){
            if self.stmt.has_ident_in_expr{
                Ok(expr_type.clone())
            }else{
                match expr_type{
                    Type::i8 | Type::i16 | Type::i32 => Ok(Type::i32),
                    Type::i64 => Ok(Type::i64),
                    Type::i128 => Ok(Type::i128),
                    Type::f32 | Type::f64 => Ok(Type::f64),
                    _ => panic!("[Line {:?}]Todo this type : {:?}", self.current_line(), expr_type)
                }
            }
        }else{
            if self.stmt.has_ident_in_expr{
                if left_bigger_than_right(&expected_type, &expr_type){
                    Err(self.mismatch_type_issue(self.stmt.expr_start_key, &expected_type, &expr_type))
                }else{
                    Ok(expr_type.clone())
                }
            }else{
                if left_bigger_than_right(&expected_type, expr_type) || expected_type == expr_type{
                    Ok(expected_type.clone())
                }else{
                    Err(self.type_out_of_range(self.stmt.expr_start_key, &expr_type))
                }
            }
        }

    }








}





