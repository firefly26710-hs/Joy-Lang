use crate::lex::token::Token;
use crate::lex::lexer::Lexer;
use crate::par::node::Node;
use crate::par::scope::Scope;
use crate::par::table::Table;

pub const NONE : u32 = u32::MAX;
pub struct Parser{
    pub lexer               : Lexer,
    

    pub edges               : Vec<u32>,
    pub nodes               : Vec<Node>,


    pub table               : Table,
    pub scope               : Scope,


    pub int_leaf_sum        : u8,
    pub flt_leaf_sum        : u8,
    pub regular_op_sum      : u8,
    pub compare_op_sum      : u8,
    pub connect_op_sum      : u8,


    pub current             : usize,

}

impl Parser{
    fn init(lexer : Lexer, table: Table, scope: Scope) -> Self{
        Self{
            lexer,

            edges               : Vec::new(),
            nodes               : Vec::new(),


            table,
            scope,


            int_leaf_sum        : 0,
            flt_leaf_sum        : 0,
            regular_op_sum      : 0,
            compare_op_sum      : 0,
            connect_op_sum      : 0,


            current             : 0,
        }
    }

    #[inline(always)]
    pub fn peek(&self) -> Token {
        if !self.current > self.lexer.tokens.len(){
            return self.lexer.tokens[self.current].clone()
        }

        Token::EOF
    }
    #[inline(always)]
    pub fn peek_next(&self) -> Token{
        if !self.current + 1 > self.lexer.tokens.len(){
            return self.lexer.tokens[self.current + 1].clone()
        }

        Token::EOF


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
    


    pub fn print_table(&mut self){todo!()}
    pub fn print_scope(&mut self){todo!()}


}


#[test]
fn parser_test(){
    let input = "let x  = 1 + 2 * 3;".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
     lexer.print_output();
    let mut parser = Parser::init(lexer, Table::init(), Scope::init());
    parser.match_stmt();
    parser.print_node();

}