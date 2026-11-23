use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::tree::node::Key;
use crate::semantic::center::sema::Semantic;
use crate::semantic::diagnostic::data::issue::TypeIssue;
use crate::semantic::scope::data::scope_kind::ScopeKind;
use crate::semantic::symbol::data::ty::Type;

impl Semantic{
    pub fn analyze_var(&mut self, var_idx : usize, depth : u8){
        let node = &self.nodes[var_idx];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let edges = &self.edges[start..=end];
        
        //name
        let var_name_key = self.nodes[edges[0] as usize].key;
        
        //type
        let type_key = self.nodes[edges[1] as usize].key;
        let declared_ty = if type_key == NONE{
            &Type::void
        }else{
            &self.type_mapping(type_key)
        };

        //type_check
       let actual_ty =  self.type_check(declared_ty, edges[2], depth);
       self.symbol_insert_variable(var_name_key, actual_ty, ScopeKind::Local, NONE, depth);
    }

    pub fn analyze_update(&mut self, var_idx : NodeIdx){
    }

   
}