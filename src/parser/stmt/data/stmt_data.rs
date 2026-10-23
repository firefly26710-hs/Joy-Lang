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

    pub max_value_key  : u32,
    pub max_value      : String,
    pub called_var_types :Type,


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
            called_var_types : Type::void,

            max_value_key : 0,
            max_value : String::with_capacity(1024),



            ret_times_in_branch: 0,
            real_times_in_branch: 0,
            is_have_else : false,
            already_full_ret_in_if: false
        }



    }
}