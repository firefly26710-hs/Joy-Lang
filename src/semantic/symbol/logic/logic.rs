use crate::lexer::tokenizer::source::interval::{End, Interval, Start};
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;
use crate::semantic::diagnostic::logic::error_handling::type_issue::from_type_to_string;
use crate::semantic::scope::data::scope_data::ScopeData;
use crate::semantic::scope::data::scope_kind::{from_scope_kind_to_string, ScopeKind};
use crate::semantic::symbol::data::ty::Type;
use crate::semantic::type_check::data::type_state::TypeState;

impl Semantic{


    pub fn assign_function(&mut self, func_idx: NodeIdx){
        let node = &self.nodes[func_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let name_node = self.edges[start..=end][0];
        let name_key = self.nodes[name_node as usize].key;
        self.f_symbol.func_keys.push(name_key);
    }
    pub fn symbol_insert_function(
        &mut self,
        func_key : Key,
        ret_type : Type
    ){
        for i in 0..self.f_symbol.func_keys.len(){
            if self.f_symbol.func_keys[i] == func_key{
                self.f_symbol.ret_types.push(ret_type.clone());
                return;
            }
        }
        panic!("No find this function");
    }

    pub fn look_up_var_type(&self, var_key : Key, var_depth : u8) -> TypeState{
        dbg!(var_key);
        let len = self.v_symbol.var_keys.len();
        for i in (0..len).rev(){
            let cur_depth = self.v_symbol.var_scopes[i].depth;
            if self.matches_str(var_key, self.v_symbol.var_keys[i]) && var_depth == cur_depth{
                let ty = &self.v_symbol.var_types[i];
                return TypeState::Stable(ty.clone())
            }
        }
        
        TypeState::Phi(var_key)
    }

    pub fn symbol_insert_variable(
        &mut self,
        var_key : Key,
        var_ty : Type,
        scope_kind : ScopeKind,
        scope_name_key: Key,
        cur_depth     : u8
    ){
        let len = self.v_symbol.var_keys.len();
        for i in 0..len{
            if self.matches_str(var_key, self.v_symbol.var_keys[i]){
                panic!("Duplicate variable name");
            }
        }
        self.v_symbol.var_keys.push(var_key);
        self.v_symbol.var_types.push(var_ty);
        self.v_symbol.var_scopes.push(
            ScopeData::new(scope_kind, cur_depth)
        );

        let name_interval = if scope_name_key != NONE {
            let interval = &self.intervals[scope_name_key as usize];
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

    pub fn print_variable_symbol(&self){
        println!("=========================Variable Symbol=========================");
        println!("Variable Name   | Scope Name   |   Type    | Depth   | ScopeKind ");
        for (i, _)in self.v_symbol.var_keys.iter().enumerate(){
            let scope_name_interval = &self.v_symbol.scope_name_intervals[i];
            let name_start = scope_name_interval.start;
            let name_end = scope_name_interval.end;
            let scope_name = if name_start == NONE && name_end == NONE{
                "No Name"
            }else{
                let bytes = &self.v_symbol.name_buf[name_start as usize..=name_end as usize];
                str::from_utf8(bytes).unwrap()
            };

            let var_name_interval = &self.intervals[self.v_symbol.var_keys[i] as usize];
            let name_start =  var_name_interval.start as usize;
            let name_end = var_name_interval.end as usize;
            let var_name = &self.input[name_start..name_end];
            let var_type = from_type_to_string(&self.v_symbol.var_types[i]);
            let scope_data = &self.v_symbol.var_scopes[i];
            let depth = scope_data.depth;
            let scope_kind = from_scope_kind_to_string(&scope_data.kind);
            println!("{:>8}{:>19}{:>12}{:>8}{:>15}",var_name,scope_name,var_type, depth,scope_kind);

        }
        println!("=================================================================");
    }
}

