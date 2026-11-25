use crate::parser::tree::ast::NONE;
use crate::semantic::center::sema::Semantic;
use crate::semantic::symbol::data::ty::Type;

impl Semantic {
    pub fn analyze(&mut self) {
        self.collect_control_flow();
        if self.error.has_error { self.output_semantic_error(); }
        self.print_control_flow();
        self.analyze_flow_context();
        self.check_return_pos();
        if self.error.has_error { self.output_semantic_error(); }
    }


    pub fn check_return_pos(&self){
        let len = self.flow.stmt_datas.len();
        let mut cur_func_key= NONE;

        let mut branch_count = 0;
        let mut return_count = 1;

        let mut l = 0;
        let mut r = 1;

        if len == 1{
            let ret_type = self.f_symbol.ret_types.last().unwrap();
            if ret_type != &Type::void{
                panic!("unexpected return type");
            }
        }


        while r < len && l < r{
        }



    }


}
