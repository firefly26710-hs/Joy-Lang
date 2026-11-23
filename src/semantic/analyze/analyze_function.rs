use crate::semantic::center::sema::Semantic;
use crate::semantic::scope::data::scope_kind::ScopeKind;

impl Semantic{
    pub fn analyze_function(&mut self, func_idx : usize, depth : u8){
        let node = &self.nodes[func_idx];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let edges = &self.edges[start..=end];
        let has_param  = edges.len() > 3;
        let len = edges.len();

        let name_idx = edges[0] as usize;
        let name_key = self.nodes[name_idx].key;

        let ret_type_idx = edges[len - 2] as usize;
        let ret_type_key = self.nodes[ret_type_idx].key;
        let ret_type = self.type_mapping(ret_type_key);
        self.symbol_insert_function(name_key, ret_type);
        if has_param{
            let param_count = (len - 3) / 2;
            for i in 0..param_count{
                let param_idx = self.edges[start..=end][1 + 2 * i] as usize;
                let type_idx = self.edges[start..=end][2 + 2 * i] as usize;

                let param_key = self.nodes[param_idx].key;
                let type_key = self.nodes[type_idx].key;
                let param_type = self.type_mapping(type_key);

                self.symbol_insert_variable(param_key, param_type, ScopeKind::Fn, name_key, depth + 1)
            }
        }



    }
}