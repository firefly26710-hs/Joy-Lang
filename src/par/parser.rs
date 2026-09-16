use crate::lex::token::Token;
use crate::lex::iterval::Iterval;
use crate::par::node::Node;
use crate::par::ty::Type;
use crate::lex::lexer::Lexer;

pub const NONE : u32 = u32::MAX;
pub const MAGIC: u32 = u32::MAX - 5;


pub struct Parser{
    pub input               : String,
    pub tokens              : Vec<Token>,
    pub lines               : Vec<u32>,
    pub intervals           : Vec<Iterval>,
    pub types               : Vec<Type>,
    pub edges               : Vec<u32>,
    pub nodes               : Vec<Node>,
    pub current             : u32,
    pub loop_counter        : u32,

    //scope
    pub var_scope_stack     : Vec<u32>,
    pub key_checker         : Vec<u32>,
    
    pub func_assign         : Vec<u32>, 
    pub param_type_idxs     : Vec<Iterval>,
    pub ret_types           : Vec<Type>,

    pub waiting             : Vec<u32>,
    


    //edge
    pub recorder            : Vec<u32>,

    //stmt
    pub ret_type            : Type,
    
    
}

impl Parser{
    pub fn init(
        input     : String,
        tokens    : Vec<Token>,
        lines     : Vec<u32>,
        intervals: Vec<Iterval>
    ) -> Self{
	Parser{
	    input,
	    tokens,
	    lines,
        intervals,
	    types               : Vec::new(),
	    edges               : Vec::new(),
	    nodes               : Vec::new(),
	    
            key_checker         : Vec::new(),
            var_scope_stack     : Vec::new(),
	    param_type_idxs     : Vec::new(),
	    func_assign         : Vec::new(),
	    ret_types           : Vec::new(),
	    recorder            : Vec::new(),
            waiting             : Vec::new(),
	    ret_type            : Type::Void,
	    loop_counter        : 0,
	    current             : 0 
	}
    }
	
    

    pub fn eat(&mut self) -> Token{ 
        let tok = self.tokens[self.current as usize].clone();
        self.current += 1;
	    tok
    }

    pub fn peek(&mut self) -> Token{
        if (self.current as usize) > self.tokens.len(){
            return Token::EOF
        }
	    self.tokens[self.current as usize].clone()
    }

    pub fn peek_next(&mut self) -> Token{
	if (self.current + 1) as usize > self.tokens.len(){
		return Token::EOF
	    }
	self.tokens[(self.current + 1) as usize].clone()
    }


    pub fn print_node(&mut self){
	for (i, node) in self.nodes.iter().enumerate(){
	    println!("Number : {i}");
	    println!("Node : {:?}", node);
	    let l_idx = (node.l_edge_idx) as usize;
	    let r_idx = (node.r_edge_idx) as usize;
	    
	    println!("Edge : {:?}", &self.edges[l_idx..=r_idx ]);
	    
	    let key = node.key;
	    let interval = &self.intervals[key as usize];
	    let start = interval.start as usize;
	    let end   = interval.end   as usize;
	    println!("String : {:?}\n", &self.input[start..end]);
	}

	    
	
    }

    pub fn print_scope(&mut self){
	print!("Key Checker : {:?}\n", self.key_checker);
	
	print!("Type Number : ");
	print!("[");
	for (_, ty) in self.types.iter().enumerate(){
	    print!("Types : {:?} ", ty);
	}
	println!("]");

	println!("Var Scope Stack : {:?}", self.var_scope_stack);
	println!("Func assign : {:?}", self.func_assign);
	    
	println!("Param type idxs : {:?}", self.param_type_idxs);
	println!("Ret type : {:?}", self.ret_types);
	
    }

    
    
}







#[test]
fn parser_testing() {
    let input = "
      fn add(a : i32, b : i32) -> i32{
            let x
            = 
            add(
            6
            , 
            7
            )
            ;
      }
".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    //lexer.print_output();
    let mut parser = Parser::init(lexer.input, lexer.tokens, lexer.lines, lexer.itervals);
    parser.func_stmt();

    parser.print_node();
    parser.print_scope();
}
