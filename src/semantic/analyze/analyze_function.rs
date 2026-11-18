use crate::lexer::tokenizer::source::interval::{End, Interval, Start};
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::semantic::center::sema::Semantic;
use crate::semantic::scope::data::scope_kind::ScopeKind;

impl Semantic{
    pub fn analyze_function(&mut self, func_idx : NodeIdx){
        let node = &self.nodes[func_idx as usize];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let edges = &self.edges[start..=end];
        let last = edges.len() - 1;
        let scope_idx = edges[last];
        let name_key = self.nodes[edges[0] as usize].key;
        let ret_type_key = self.nodes[edges[last - 2] as usize].key;
        let has_param = edges.len() > 3;


        let param_start = self.v_symbol.var_types.len();

        if has_param{
            let param_num = (last - 2 ) / 2;
            self.cur_depth += 1;
            for i in 0..param_num{
                
                let arg_key = self.nodes[1 + 2 * i].key;
                let arg_type_key = self.nodes[2 + 2 * i].key;
                let ty = self.type_mapping(arg_type_key);
                self.symbol_insert_variable(arg_key, ty, ScopeKind::Fn);
            }
            self.cur_depth -= 1;
        }

        let param_end = self.v_symbol.var_types.len();

        let interval = if has_param{
            Interval::new(param_start as Start, param_end as End)
        }else{
            Interval::new(NONE, NONE)
        };

        self.symbol_insert_function(
            name_key, interval, self.type_mapping(ret_type_key)
        );

        self.analyze_scope(scope_idx, &ScopeKind::Fn);
    }
}