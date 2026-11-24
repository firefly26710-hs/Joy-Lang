use crate::parser::tree::ast::{NodeIdx};
use crate::parser::tree::node::Key;

const RECORDER_SIZE : usize = 256;

pub struct Stmt{
    pub programs           : Vec<NodeIdx>,
    pub func_recorder      : Vec<NodeIdx>,
    pub branch_recorder    :Vec<NodeIdx>,
    pub loop_recorder      : Vec<NodeIdx>,
    pub func_call_recorder : Vec<NodeIdx>,
    pub scope_recorder     : Vec<NodeIdx>,
    pub lbrace_stack       : Vec<u32>,
    pub comparison_op_counter: u8,

}

impl Stmt{
    pub fn init() -> Self{
        Self{
            programs : Vec::with_capacity(RECORDER_SIZE),
            func_recorder: Vec::with_capacity(RECORDER_SIZE),
            branch_recorder : Vec::with_capacity(RECORDER_SIZE),
            loop_recorder   : Vec::with_capacity(RECORDER_SIZE),
            func_call_recorder : Vec::with_capacity(RECORDER_SIZE),
            scope_recorder : Vec::with_capacity(RECORDER_SIZE),
            lbrace_stack : Vec::with_capacity(32),
            comparison_op_counter: 0,
        }
    }

    pub fn recorder_init(&mut self){
        self.func_recorder.clear();
        self.branch_recorder.clear();
        self.loop_recorder.clear();
        self.func_call_recorder.clear();
        self.scope_recorder.clear();
    }
}