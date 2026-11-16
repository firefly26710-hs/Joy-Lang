use crate::lexer::tokenizer::source::token::Token;
use crate::llvm::expr::{is_binary_op, is_comparison_op};
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;

impl Parser{
    pub fn matches_type(&mut self, expected_type : &Type, expr_idx : u32) -> Result<Type, ParserIssue>{
        let expr_type = &self.parse_expr_type(expr_idx, expected_type)?;
        if matches!(expected_type, Type::void){

            if self.stmt.has_ident_or_func_in_expr {
                Ok(expr_type.clone())
            }else{
                match expr_type{
                    Type::i8 | Type::i16 | Type::i32 => Ok(Type::i32),
                    Type::i64 => Ok(Type::i64),
                    Type::i128 => Ok(Type::i128),
                    Type::bool => Ok(Type::bool),
                    Type::f32 | Type::f64 => Ok(Type::f64),
                    _ => panic!("[Line {:?}]Todo this type : {:?}", self.current_line(), expr_type)
                }
            }
        }else{
            if self.stmt.has_ident_or_func_in_expr {
                if !is_two_types_in_same_set(expected_type, expr_type){
                    return Err(self.mismatch_type_issue(expr_idx, &expected_type, &expr_type))
                }

                if left_bigger_than_right(&expected_type, &expr_type){
                    Err(self.literal_out_of_range(expr_idx, &expected_type))
                }else{
                    Ok(expr_type.clone())
                }
            }else{
                if !is_two_types_in_same_set(expected_type, expr_type){
                    return Err(self.mismatch_type_issue(expr_idx, &expected_type, &expr_type))
                }

                if left_bigger_equal_than_right(&expected_type, expr_type){
                    Ok(expected_type.clone())
                }else{
                    Err(self.literal_out_of_range(expr_idx, &expected_type))
                }
            }
        }

    }

    pub fn parse_expr_type(&mut self, expr_idx : NodeIdx, expected_type : &Type) -> Result<Type, ParserIssue>{
        self.stmt.already_has_comparison_op = false;
        self.stmt.unary_sub_counter = 0;
        self.get_expr_type(expr_idx, expected_type)
    }

    pub fn matches_ret_type(&mut self, expr_idx : Option<NodeIdx>, ret_type : &Type, tok_pos : usize) -> Result<(), ParserIssue>{

        match expr_idx{
            Some(idx) => {
                if matches!(ret_type, Type::void){
                    let expr_type = self.parse_expr_type(idx, ret_type)?;
                    Err(self.mismatch_ret_type_with_expr_issue(idx, ret_type, &expr_type))
                }else{
                    Ok(())
                }
            },
            None => {
                if !matches!(ret_type, Type::void){
                    Err(self.mismatch_ret_type_without_expr_issue(tok_pos, ret_type))
                }else{
                    Ok(())
                }
            }
        }
    }

    fn get_expr_type(&mut self, expr_idx : u32, expected_type : &Type) -> Result<Type, ParserIssue> {
        let node = &self.ast.nodes[expr_idx as usize];
        let tok = &node.token;
        let key = node.key;
        let edge_span = &node.edge_span;
        let left_span = edge_span.start;
        let right_span = edge_span.end;
        if tok == &Token::CallStmt{
            let name_node_idx = self.ast.edges[left_span as usize..=right_span as usize][0];
            let key = self.ast.nodes[name_node_idx as usize].key;
            return Ok(self.parse_func_call_type(key)?)
        }
        match(left_span, right_span){
            (NONE, NONE) => match tok{
                Token::Int => Ok(self.parse_int_type(key, expected_type, expr_idx)?),
                Token::Float => Ok(self.parse_flt_type(key, expected_type, expr_idx)?),
                Token::Ident => Ok(self.parse_ident_type(key)?),
                Token::True | Token::False => Ok(Type::bool),
                _ => panic!("unknown Leaf : {:?}", tok)
            }

            (left_span, right_span) => {
                let left_idx = self.ast.edges[left_span as usize..=right_span as usize][0];
                let right_idx = self.ast.edges[left_span as usize..=right_span as usize][1];

                match (left_idx, right_idx){
                    (NONE, rhs) => match tok{
                        Token::Sub => {
                            if self.stmt.unary_sub_counter == 0{ self.stmt.first_unary_expr_idx = expr_idx; }
                            self.stmt.unary_sub_counter += 1;
                            let expr_type = self.parse_expr_type(rhs, expected_type)?;
                            self.stmt.unary_sub_counter  -= 1;
                            if expr_type == Type::bool{
                                return Err(self.mismatch_expr_type_in_unary_issue(key, rhs,Type::bool));
                            }

                            Ok(expr_type)

                        }

                        Token::LogicNot => {
                            let expr_type = self.parse_expr_type(rhs, expected_type)?;
                            if expr_type != Type::bool{
                                return Err(self.mismatch_expr_type_in_unary_issue(key, rhs, expr_type));
                            }

                            Ok(expr_type)
                        }
                        _ => unreachable!()
                    },
                    (lhs, rhs) =>{
                        let tok = tok.clone();

                        let l_ty = self.parse_expr_type(lhs, expected_type)?;
                        let r_ty = self.parse_expr_type(rhs, expected_type)?;

                        if is_binary_op(&tok){
                            if l_ty != Type::bool{ return Err(self.mismatch_binary_op_type(lhs, l_ty)) }
                            if r_ty != Type::bool{ return Err(self.mismatch_binary_op_type(rhs, r_ty))}
                        }

                        if is_comparison_op(&tok) && self.stmt.already_has_comparison_op{
                            return Err(self.comparison_op_chained_issue(key));
                        }
                        if is_comparison_op(&tok){
                            self.stmt.already_has_comparison_op = true;
                        }
                        
                        
                        
                        if !is_two_types_in_same_set(&r_ty, &l_ty){
                            return Err(self.mismatches_expression_type(&l_ty, &r_ty, rhs as usize));
                        }

                        

                        if self.is_ident(lhs) && left_bigger_than_right(&r_ty, &l_ty){
                            Err(self.literal_out_of_range(lhs, expected_type))
                        }else if self.is_ident(rhs) && left_bigger_than_right(&l_ty, &r_ty){
                            Err(self.literal_out_of_range(rhs, expected_type))
                        }else{
                            if left_bigger_equal_than_right(&l_ty, &r_ty){
                                if self.stmt.already_has_comparison_op{
                                    return Ok(Type::bool);
                                }
                                if !left_bigger_equal_than_right(expected_type, &l_ty){
                                    return Err(self.literal_out_of_range(lhs, expected_type));
                                }

                                if !left_bigger_equal_than_right(expected_type, &r_ty){
                                    return Err(self.literal_out_of_range(rhs, expected_type))
                                }

                                Ok(l_ty)
                            } else{
                                if self.stmt.already_has_comparison_op{
                                    return Ok(Type::bool);
                                }
                                Ok(r_ty)
                            }
                        }

                    }
                }


            }
        }
    }



    fn parse_int_type(&mut self, key : Key, expected_type : &Type, expr_idx : NodeIdx) -> Result<Type, ParserIssue>{
        let interval = &self.lexer.tokenizer.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let int_val: u128 = match self.lexer.tokenizer.input[start..end].parse() {
            Ok(val) => val,
            Err(val) => {
                panic!("[Line {:?}] Expected Int but find : {:?}", self.current_line(), val)
            }
        };

        if self.stmt.unary_sub_counter == 0 {
            match int_val {
                0..=127 => Ok(Type::i8),
                0..=32767 => Ok(Type::i16),
                0..=2147483647 => Ok(Type::i32),
                0..=9223372036854775807 => Ok(Type::i64),
                0..=170141183460469231731687303715884105727 => Ok(Type::i128),
                _ =>{
                    let expected_type = if expected_type == &Type::void{
                        &Type::i32
                    }else {
                        expected_type
                    };
                    Err(self.literal_out_of_range(expr_idx, expected_type ))
                }
            }
        }else{
            if self.stmt.unary_sub_counter % 2  == 0{
                let expected_type = if expected_type == &Type::void{
                    &Type::i32
                }else{
                    &expected_type
                };

                return Err(self.literal_out_of_range(self.stmt.first_unary_expr_idx, expected_type))
            }


            let val_ty = match int_val {
                0..=128 => Type::i8,
                0..=32768 => Type::i16,
                0..=2147483648 => Type::i32,
                0..=9223372036854775808 => Type::i64,
                0..=170141183460469231731687303715884105728 => Type::i128,
                 _ => {
                     let expected_type = if expected_type == &Type::void {
                         &Type::i32
                     } else {
                         expected_type
                     };
                     Err(self.literal_out_of_range(expr_idx, expected_type))
                 }?
            };

            if !left_bigger_equal_than_right(&expected_type, &val_ty) {
                 return Err(self.literal_out_of_range(self.stmt.first_unary_expr_idx, expected_type));
            }

            Ok(val_ty)
        }


    }

    fn parse_flt_type(&mut self, key: Key, expected_type : &Type, expr_idx : NodeIdx) -> Result<Type, ParserIssue>{
        let interval = &self.lexer.tokenizer.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let flt_val: f64 = match self.lexer.tokenizer.input[start..end].parse() {
            Ok(val) => val,
            _ => panic!("[Line {:?}]Parse Float Error", self.current_line())
        };



        match flt_val{
            0.0..=3.40282347e+38 => Ok(Type::f32),
            0.0..=1.7976931348623157e+308=> Ok(Type::f64),
            _ => {
                let expected_type = if expected_type == &Type::void{
                    &Type::f64
                }else {
                    expected_type
                };
                Err(self.literal_out_of_range(expr_idx, expected_type ))
            }
        }
    }

    fn parse_ident_type(&mut self, key : Key) -> Result<Type, ParserIssue>{
        self.stmt.has_ident_or_func_in_expr = true;
        Ok(self.look_var_type(&key).clone())
    }

    fn parse_func_call_type(&mut self, key: Key) -> Result<Type, ParserIssue>{
        self.stmt.has_ident_or_func_in_expr = true;
        Ok(self.look_ret_type(key).clone())
    }

    fn is_ident(&self ,idx : u32) -> bool{
        self.ast.nodes[idx as usize].token == Token::Ident
    }



}

pub fn is_two_types_in_same_set(type_a : &Type, type_b : &Type) -> bool{
    let a_num = weight(type_a);
    let b_num = weight(type_b);

    if a_num == i32::MAX || b_num == i32::MAX{
        return true;
    }

    //integer type set
    if a_num > 0 && a_num < 10 && b_num > 0 && b_num < 10 {
        true

    //float type set
    }else if a_num > 10 && a_num < 20 && b_num > 10 && b_num < 20{
        true
    }else if a_num == 0 && b_num == 0 {
        true
    }else{
        false
    }

}

pub fn left_bigger_equal_than_right(type_a : &Type, type_b : &Type) -> bool{

    let weight_a = weight(type_a);
    let weight_b = weight(type_b);

    if weight_a >= weight_b {
        true
    }else{
        false
    }
}

pub fn left_bigger_than_right(type_a : &Type, type_b : &Type) -> bool{

    let weight_a = weight(type_a);
    let weight_b = weight(type_b);

    if weight_a > weight_b {
        true
    }else{
        false
    }
}


fn weight(ty : &Type) -> i32{
    match ty {
        Type::i8 => 1,
        Type::i16 => 2,
        Type::i32 => 3,
        Type::i64 => 4,
        Type::i128 => 5,
        Type::f32 => 11,
        Type::f64 => 12,
        Type::bool => 0,
        Type::void => i32::MAX,
    }
}





