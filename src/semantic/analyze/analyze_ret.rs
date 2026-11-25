use crate::semantic::symbol::data::ty::Type;
use crate::parser::tree::ast::NONE;
use crate::semantic::center::sema::Semantic;

impl Semantic{
    pub fn analyze_return(&mut self, ret_idx : usize, depth : u8){
        let node = &self.nodes[ret_idx];
        let span = &node.edge_span;
        let start = span.start as usize;
        let end = span.end as usize;
        let edge = &self.edges[start..=end];
        let func_name_key = self.nodes[edge[0] as usize].key;
        let ret_type = self.look_up_ret_type(func_name_key);

        let expr_idx = edge[1];



        match (ret_type, expr_idx){
            (Type::void, NONE) => {
                return;
            },
            (Type::void, expr_idx) if expr_idx == NONE => {
                panic!("Expected Return Void but find expr");
            },
            (any, expr_idx) if any != &Type::void && expr_idx == NONE => {
                panic!("Mismatch return type")
            },
            (any, expr_idx) => {
                self.type_check(&any.clone(), expr_idx, depth);
            }
        }




    }
}