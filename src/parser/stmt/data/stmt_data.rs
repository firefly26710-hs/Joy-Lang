use crate::parser::tree::ast::NodeIdx;
use crate::parser::types::typ::Type;

const RECORDER_SIZE : usize = 256;

pub struct Stmt{
    pub programs     : Vec<NodeIdx>,
    pub func_recorder: Vec<NodeIdx>,
    pub branch_recorder:Vec<NodeIdx>,
    pub loop_recorder : Vec<NodeIdx>,
    pub func_call_recorder : Vec<NodeIdx>,

    pub int_node_sum   : u8,
    pub flt_node_sum   : u8,
    pub bool_node_sum  : u8,
    pub regular_op_sum : u8,
    pub compare_op_sum : u8,
    pub is_string      : bool,

    pub is_prev_node_is_ident : bool,
    pub has_ident_in_expr     : bool,
    pub expr_start_key : u32,
    pub prev_node_key  : u32,
    pub prev_node_type : Type,


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

            int_node_sum : 0,
            flt_node_sum : 0,
            bool_node_sum : 0,
            regular_op_sum : 0,
            compare_op_sum : 0,
            is_string : false,
            expr_start_key : 0,
            prev_node_key : 0,
            prev_node_type: Type::void,

            is_prev_node_is_ident : false,


            ret_times_in_branch: 0,
            real_times_in_branch: 0,
            is_have_else : false,
            already_full_ret_in_if: false
        }



    }
}