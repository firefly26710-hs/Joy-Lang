use crate::lex::token::Token;
use crate::lex::lexer::Lexer;
use crate::par::node::Node;
use crate::par::scope::Scope;
use crate::par::table::Table;

pub const NONE : u32 = u32::MAX;
const RECORDER_SIZE : usize = 256;

pub type NodeIdx = u32;

pub struct Parser{
    pub lexer               : Lexer,
    

    pub edges               : Vec<NodeIdx>,
    pub nodes               : Vec<Node>,


    pub table               : Table,
    pub scope               : Scope,


    pub int_leaf_sum        : u8,
    pub flt_leaf_sum        : u8,
    pub regular_op_sum      : u8,
    pub compare_op_sum      : u8,
    pub connect_op_sum      : u8,


    pub func_recorder       : Vec<u32>,
    pub branch_recorder     : Vec<u32>,
    pub block_recorder      : Vec<u32>,

   
    pub current             : usize,



}

impl Parser{
    pub fn init(lexer : Lexer, table: Table, scope: Scope) -> Self{
        let alloc_size = lexer.tokens.len();

        Self{
            lexer,

            edges               : Vec::with_capacity(alloc_size),
            nodes               : Vec::with_capacity(alloc_size),


            table,
            scope,


            int_leaf_sum        : 0,
            flt_leaf_sum        : 0,
            regular_op_sum      : 0,
            compare_op_sum      : 0,
            connect_op_sum      : 0,


            func_recorder        : Vec::with_capacity(RECORDER_SIZE),
            branch_recorder      : Vec::with_capacity(RECORDER_SIZE),
            block_recorder       : Vec::with_capacity(RECORDER_SIZE),

            current             : 0,
        }
    }

    pub fn build_tree(&mut self){todo!()}


    //More Frequency in first cond
    //Set N is the length of tokens
    //So We only Branch Hazard in once
    #[inline(always)]
    pub fn peek(&self) -> &Token {
        if self.current < self.lexer.tokens.len(){
            return &self.lexer.tokens[self.current]
        }

        &Token::EOF
    }

    //More Frequency in first cond
    //Set N is the length of tokens
    //So We only Branch Hazard in once
    #[inline(always)]
    pub fn peek_next(&self) -> &Token{
        if self.current + 1 < self.lexer.tokens.len(){
            return &self.lexer.tokens[self.current + 1]
        }

        &Token::EOF // less_freq
    }
    #[inline(always)]
    pub fn eat(&mut self) -> Token{
        let tok = self.lexer.tokens[self.current].clone();
        self.current += 1;
        tok
    }



    #[inline(always)]
    pub fn line(& self) -> u32{
        self.lexer.lines[self.current]
    }

    pub fn print_node(&mut self){
        for (i, node) in self.nodes.iter().enumerate() {
            println!("Number : {i}");
            println!("Node : {:?}", node);

            let idxs = &node.idxs;
            let l_idx = idxs.start;
            let r_idx = idxs.end;

            if l_idx != NONE && r_idx != NONE {
                println!("Edge : {:?}", &self.edges[l_idx as usize..=r_idx as usize]);
            }

            let key = node.key;
            if key == NONE{continue}
            let interval = &self.lexer.intervals[key as usize];
            let start = interval.start as usize;
            let end   = interval.end   as usize;
            println!("String : {:?}\n", &self.lexer.input[start..end]);
        }
    }
    


    pub fn print_table(&mut self){
        for (i, ty) in self.table.var_types.iter().enumerate(){
            println!("Var Type {i} : {:?} ", ty);
        }


        for (i, idx) in self.table.param_idxs.iter().enumerate(){
            println!("param idx {i} : {:?} ", idx);
        }

        for (i, ty) in self.table.ret_types.iter().enumerate(){
            println!("Ret Type {i} : {:?} ", ty);
        }

    }
    pub fn print_scope(&mut self){todo!()}


}




#[test]
fn parser_test(){
    let input = "
    fn main(a : i32){
        let c = 5;
    }
    ".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    lexer.print_output();

    let mut parser = Parser::init(lexer, Table::init(), Scope::init());
    parser.func_stmt();
    parser.print_node();
    parser.print_table();

}