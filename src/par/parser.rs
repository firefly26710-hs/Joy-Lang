use crate::lex::token::Token;
use crate::lex::iterval::Interval;
use crate::lex::lexer::Lexer;
use crate::par::node::Node;
use crate::par::scope::Scope;
use crate::par::table::Table;
use crate::par::ty::Type;

pub const NONE : u32 = u32::MAX;
pub struct Parser{
    pub input               : String,
    pub tokens              : Vec<Token>,
    pub lines               : Vec<u32>,
    pub str_intervals: Vec<Interval>,

    pub edges               : Vec<u32>,
    pub nodes               : Vec<Node>,

    pub table               : Table,
    pub scope               : Scope,



    pub int_leaf_sum        : u8,
    pub flt_leaf_sum        : u8,
    pub regular_op_sum      : u8,
    pub compare_op_sum      : u8,
    pub connect_op_sum      : u8,

    pub ret_type            : Type,

    pub current             : usize,

}

impl Parser{
    fn init(lexer : Lexer, table: Table, scope: Scope) -> Self{
        Self{
            input     : lexer.input,
            tokens    : lexer.tokens,
            lines     : lexer.lines,
            str_intervals: lexer.intervals,

            edges               : Vec::new(),
            nodes               : Vec::new(),


            table,
            scope,


            int_leaf_sum        : 0,
            flt_leaf_sum        : 0,
            regular_op_sum      : 0,
            compare_op_sum      : 0,
            connect_op_sum      : 0,

            ret_type            : Type::void,
            current             : 0,
        }
    }

    #[inline(always)]
    pub fn peek(&self) -> Token {
        if self.current > self.tokens.len(){
            return Token::EOF;
        }

        self.tokens[self.current].clone()
    }
    #[inline(always)]
    pub fn peek_next(&self) -> Token{
        if self.current + 1 > self.tokens.len(){
            return Token::EOF;
        }

        self.tokens[self.current + 1].clone()
    }
    #[inline(always)]
    pub fn eat(&mut self) -> Token{
        let tok = self.tokens[self.current].clone();
        self.current += 1;
        tok
    }

    pub fn print_node(&mut self){
        for (i, node) in self.nodes.iter().enumerate(){
            println!("Number : {i}");
            println!("Node : {:?}", node);
            let idxs = &node.idxs;
            let l_idx = idxs.start as usize;
            let r_idx = idxs.end as usize;

            println!("Edge : {:?}", &self.edges[l_idx..=r_idx ]);

            let key = node.key;
            if key == NONE{continue}
            let interval = &self.str_intervals[key as usize];
            let start = interval.start as usize;
            let end   = interval.end   as usize;
            println!("String : {:?}\n", &self.input[start..end]);
        }
    }

    pub fn print_table(&mut self){todo!()}
    pub fn print_scope(&mut self){todo!()}


}


#[test]
fn parser_test(){
    let input = "let x = 1 + 2 * 3".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    let mut parser = Parser::init(lexer, Table::init(), Scope::init());
    parser.let_stmt();
    parser.print_node();

}