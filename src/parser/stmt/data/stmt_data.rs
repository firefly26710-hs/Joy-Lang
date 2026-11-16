use inkwell::llvm_sys::lto::thinlto_codegen_set_final_cache_size_relative_to_available_space;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::tree::node::Key;
use crate::parser::types::typ::Type;

const RECORDER_SIZE : usize = 256;

pub struct Stmt{
    pub programs     : Vec<NodeIdx>,
    pub func_recorder: Vec<NodeIdx>,
    pub branch_recorder:Vec<NodeIdx>,
    pub loop_recorder : Vec<NodeIdx>,
    pub func_call_recorder : Vec<NodeIdx>,
    

    pub expr_start_key : u32,
    pub already_has_comparison_op: bool,
    pub has_ident_or_func_in_expr: bool,
    pub unary_sub_counter: u16,
    pub first_unary_expr_idx : Key,


    pub ret_times_in_branch: u8,
    pub real_times_in_branch: u8,
    pub is_have_else       : bool,
    pub already_full_ret_in_if: bool
}

impl Stmt{
    pub fn init() -> Self{
        Self{
            programs : Vec::with_capacity(RECORDER_SIZE),
            func_recorder: Vec::with_capacity(RECORDER_SIZE),
            branch_recorder : Vec::with_capacity(RECORDER_SIZE),
            loop_recorder   : Vec::with_capacity(RECORDER_SIZE),
            func_call_recorder : Vec::with_capacity(RECORDER_SIZE),

            expr_start_key : 0,
            already_has_comparison_op: false,
            has_ident_or_func_in_expr: false,
            unary_sub_counter: 0,
            first_unary_expr_idx : 0,

            ret_times_in_branch: 0,
            real_times_in_branch: 0,
            is_have_else : false,
            already_full_ret_in_if: false
        }



    }
}