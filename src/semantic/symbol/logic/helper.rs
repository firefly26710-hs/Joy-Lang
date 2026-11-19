use std::ops::Deref;
use crate::lexer::tokenizer::source::interval::{End, Interval, Start};
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::tree::ast::NONE;
use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;
use crate::semantic::scope::data::scope_data::ScopeData;
use crate::semantic::scope::data::scope_kind::ScopeKind;
use crate::semantic::symbol::data::ty::Type;

impl Semantic{
    pub fn symbol_insert_function(
        &mut self,
        func_key : Key,
        param_interval : Interval,
        ret_type       : Type
    ){
        self.f_symbol.func_keys.push(func_key);
        self.f_symbol.param_intervals.push(param_interval);
        self.f_symbol.ret_types.push(ret_type);

    }

    pub fn symbol_insert_variable(
        &mut self,
        var_key : Key,
        var_ty : Type,
        scope_kind : ScopeKind,
        scope_kind_name_key : Key
    ){
        self.v_symbol.var_keys.push(var_key);
        self.v_symbol.var_types.push(var_ty);
        self.v_symbol.var_scope.push(
            ScopeData::new(scope_kind, self.cur_depth)
        );

        let name_interval = if scope_kind_name_key != NONE {
            let interval = &self.intervals[scope_kind_name_key as usize];
            let start = interval.start as usize;
            let end = interval.end as usize;
            let bytes = self.input[start..end].as_bytes();
            let byte_start = self.v_symbol.name_buf.len();
            self.v_symbol.name_buf.extend_from_slice(bytes);
            let byte_end = self.v_symbol.name_buf.len() - 1;
            Interval::new(byte_start as Start, byte_end as End)
        }else{
            Interval::new(NONE, NONE)
        };

        self.v_symbol.scope_name_intervals.push(name_interval);



    }

    pub fn type_mapping(&self, key : Key) -> Type{
        let interval = &self.intervals[key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let str = &self.input[start..end];
        match str{
            "i8"  => Type::i8,
            "i16" => Type::i16,
            "i32" => Type::i32,
            "i64" => Type::i64,
            "i128"=> Type::i128,
            "f32" => Type::f32,
            "f64" => Type::f64,
            "bool"=> Type::bool,
            "void"=> Type::void,
            _ => unreachable!()
        }
    }
}

