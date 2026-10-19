use crate::par::tree::ast::NodeIdx;
const RECORDER_SIZE : usize = 256;

pub struct Stmt{
    //logic helper
    pub programs     : Vec<NodeIdx>,
    pub func_recorder: Vec<NodeIdx>,
    pub branch_recorder:Vec<NodeIdx>,
    pub loop_recorder : Vec<NodeIdx>,
    pub func_call_recorder : Vec<NodeIdx>,

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

            ret_times_in_branch: 0,
            real_times_in_branch: 0,
            is_have_else : false,
            already_full_ret_in_if: false
        }



    }
}