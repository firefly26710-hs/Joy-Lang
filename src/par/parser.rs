use crate::lex::token::Token;
use crate::lex::lexer::Lexer;
use crate::par::node::Node;
use crate::par::ty::Type;

pub const NONE : u32 = u32::MAX;

pub struct Parser{
    pub source         : String,
    pub tokens         : Vec<Token>,
    pub lines          : Vec<u32>,
    pub itervals       : Vec<(u32, u32)>,
    pub types          : Vec<Type>,
    pub edges          : Vec<u32>,

    pub nodes          : Vec<Node>, // token, key, l_idx, r_idx
    pub key_checker    : Vec<u32>,
    pub l_type_idx     : Vec<u32>,
    pub r_type_idx     : Vec<u32>,

    var_scope_stack    : Vec<u32>,
    func_scope_stack   : Vec<u32>,

    loop_counter       : u32,
    current            : u32
    
}

impl Parser{
    pub fn init(
	source    : String,
	tokens    : Vec<Token>,
	lines     : Vec<u32>,
	itervals  : Vec<(u32, u32)>
    ) -> Self{
	Parser{
	    source,
	    tokens,
	    lines,
	    itervals,
	    types              : Vec::new(),
	    edges              : Vec::new(),
	    nodes              : Vec::new(),
            key_checker        : Vec::new(),
            l_type_idx         : Vec::new(),
            r_type_idx         : Vec::new(),

             var_scope_stack    : Vec::new(),
            func_scope_stack   : Vec::new(),

            loop_counter       : 0,
            current            : 0 
	}
    }

    pub fn expression(&mut self, min_bp : f32) -> u32{
	let mut lhs_idx = match self.eat(){
	    Token::Int   =>{
		let int_node = Node::init(Token::Int, self.current - 1, NONE, NONE);
		self.nodes.push(int_node);
		self.edges.push(as_u32(self.nodes.len() - 1));
		self.nodes.len() - 1
	    },
	    Token::Float =>{
		let float_node = Node::init(Token::Float ,self.current - 1, NONE, NONE);
		self.nodes.push(float_node);
		self.edges.push(as_u32(self.nodes.len() - 1));
		self.nodes.len() - 1
	    },
	    Token::Minus => {
		let rhs_idx = self.expression(1000.0);
		let unary_node = Node::init(Token::Minus, self.current - 2, NONE, rhs_idx);
		self.nodes.push(unary_node);
		self.edges.push(as_u32(self.nodes.len() - 1));
		self.nodes.len() - 1
	    }
	    _ => panic!("wth")
	};

	loop{
	    let op_token = match self.peek(){
		Token::EOF | Token::Scolon => break,
		op_token => self.peek().clone()
	    };
	    
	    let (l_bp, r_bp) = infix_binding_power(&op_token);
	    
	    if l_bp < min_bp{
		break;
	    }

	    self.eat();

	    let op_idx = self.current - 1;

	    let rhs_idx = self.expression(r_bp);
	    
	    

	    self.nodes.push(
		Node::init(
		    op_token,
		    op_idx,
		    as_u32(lhs_idx),
		    rhs_idx
		)
	    );

	    self.edges.push(as_u32(self.nodes.len() - 1));
	    lhs_idx = self.nodes.len() - 1;
	}
	
	as_u32(lhs_idx)
	
    }

    fn eat(&mut self) -> &Token{
	let curr = as_usize(self.current);
	self.current += 1;
	&self.tokens[curr]
    }

    fn peek(&mut self) -> &Token{
	&self.tokens[as_usize(self.current)]
    }

    fn peek_next(&mut self) -> &Token{
	&self.tokens[as_usize(self.current + 1)]
    }

    pub fn print_output(&mut self){
	for (i, node) in self.nodes.iter().enumerate(){
	    println!("Node : {i}, {:?} ", node);

	    let l_idx = node.l_edge_idx;
	    let r_idx = node.r_edge_idx;

	    if matches!(l_idx, NONE) && matches!(r_idx, NONE){continue;}
	    
	}
	   
    }
	
}



fn infix_binding_power(op : &Token) -> (f32, f32){
    match op{
	Token::CmpAnd | Token::CmpOr     => (0.1, 0.2),
        Token::CmpAnd | Token::CmpOr     => (0.3, 0.3),
	Token::Grt    | Token::Les       => (0.5, 0.6),
        Token::Plus   | Token::Minus     => (1.0, 2.0),
        Token::Div    | Token::Mod       => (3.0, 3.1),
	Token::Mul                       => (4.0, 4.0),
        _         => panic!("Unknown Operator{:?}", op)
    }
}



fn as_usize(idx : u32) -> usize{
    idx as usize
}

fn as_u32(idx : usize) -> u32{
    idx as u32
}

#[test]
fn parser_testing() {
    let input = "1 + 1 - 5".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    lexer.print_output();
    let mut parser = Parser::init(lexer.input, lexer.tokens, lexer.lines, lexer.itervals);
    parser.expression(0.0);
    parser.print_output()
}
