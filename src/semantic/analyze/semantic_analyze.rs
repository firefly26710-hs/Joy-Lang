use crate::parser::tree::ast::NONE;
use crate::semantic::center::sema::Semantic;
use crate::semantic::symbol::data::stmt_kind::StmtKind;
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


        while r < len && l < r {
            if self.flow.stmt_datas[l].kind == StmtKind::FuncStmt{
                let
            }


            if r == len{

            }


            let l_depth = self.flow.stmt_datas[l].depth;
            let r_depth = self.flow.stmt_datas[r].depth;
            while r < len && self.flow.stmt_datas[r].depth == r_depth {
                r += 1;

            }



        }


    }
}
