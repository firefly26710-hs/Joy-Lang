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

    fn match_stmt(&mut self) -> u32{
        match self.peek(){
            Token::Let   => self.let_stmt(),
            Token::For   => self.for_stmt(),
            Token::While => self.while_stmt(),
        }




    }

    fn let_stmt(&mut self) -> u32{
        let current = self.current;

	    let name_idx = match self.peek(){
	        Token::Ident =>{
		    self.eat();
		    let node = Node::init(Token::Ident, current , NONE, NONE);
		    self.nodes.push(node);
		    self.edges.push(as_u32(self.nodes.len() - 1));
		    self.nodes.len() - 1
	        }
	    
	        t            =>panic!(
                "[Line {:?}] Find the Invaild Variable Name : {:?}", self.lines[as_usize(self.current)], t
                )
	    };

	    let _eq_tok = match self.peek(){
	        Token::Equel => self.eat(),
	        _            => panic!("[Line {:?}] expectdd '=' ", self.lines[as_usize(self.current)])
	    };

	    let expr_idx = self.expr(0.0);

	    let _scolon_tok = match self.peek(){
	        Token::Scolon => self.eat(),
	        _             => panic!("[Line {:?}] expected ';' ", self.lines[as_usize(self.current)])
    	};

	    let node = Node::init(Token::LetStmt , current, as_u32(name_idx), expr_idx);
	    self.nodes.push(node);
	    self.edges.push(as_u32(self.nodes.len() - 1));
	    as_u32(self.nodes.len() - 1)

        }

        fn for_stmt(&mut self)->u32{
            self.eat();
            let l_idx = self.current - 1;

            //i
            self.peek(){
                Token::Ident => {
                    self.eat();
                    let node = Node::init(Token::Ident, self.current, NONE, NONE);
                    self.nodes.push(node);
                    self.edges.push(as_u32(self.nodes.len() - 1));
                },
                _            => {
                    panic!("[Line {:?}] Expected identifier", self.lines[self.current])
                }
            };

            //In 
            self.peek(){
                Token::In => {
                    self.eat();
                },
                _         => {
                    panic!("[Line {:?}] Expected 'in' ",self.lines[self.current])
                }
            };

            //Left Bracket
            self.peek(){
                Token::Lbracket => {
                    self.eat();
                },
                _              => {
                    panic!("[Line {:?}] Expected '[' ", self.lines[self.current])
                }
            };

            //start
            self.peek(){
                Token::Int => {
                    self.eat();
                    let node = Node::init(Token::Int, self.current, NONE, NONE);
                    self.nodes.push(node);
                    self.edges.push(as_u32(self.nodes.len() - 1));
                },
                Token::Ident => {
                    todo!()
                },
                _ => {
                    panic!("[Line {:?}] Expected an integer", self.lines[self.current])
                }
            };

            //Arrow
            self.peek(){
                Token::Arrow => {
                    self.eat();
                },
                _ => {
                    panic!("[Line {:?}] Expected '->'", self.lines[self.current])
                }
            };

            //end
            self.peek(){
                Token::Int => {
                    self.eat();
                    let node = Node::init(Token::Int, self.current, NONE, NONE);
                    self.nodes.push(node);
                    self.edges.push(as_u32(self.nodes.len() - 1));
                },
                _ => {
                    panic!("[Line {:?}] Expected an integer", self.lines[self.current])
                }
            };

            //Right 
            self.peek(){
                Token::Rbracket => {
                    self.eat();
                },
                _ => {

                }
            }

    }

    fn while_stmt(&mut self)->u32{todo!()}

    fn block_stmt(&mut self)->u32{todo!()}



    

    fn expr(&mut self, min_bp : f32) -> u32{
	let mut lhs_idx = match self.eat(){
	    Token::Int   =>{
		let node = Node::init(Token::Int, self.current - 1, NONE, NONE);
		self.nodes.push(node);
		self.edges.push(as_u32(self.nodes.len() - 1));
		self.nodes.len() - 1
	    },
	    Token::Float =>{
		let node = Node::init(Token::Float ,self.current - 1, NONE, NONE);
		self.nodes.push(node);
		self.edges.push(as_u32(self.nodes.len() - 1));
		self.nodes.len() - 1
	    },
	    Token::Ident =>{
		let node = Node::init(Token::Ident ,self.current - 1, NONE, NONE);
		self.nodes.push(node);
		self.edges.push(as_u32(self.nodes.len() - 1));
		self.nodes.len() - 1
	    }
	    Token::Minus => {
		let rhs_idx = self.expr(1000.0);
		let node = Node::init(Token::Minus, self.current - 2, NONE, rhs_idx);
		self.nodes.push(node);
		self.edges.push(as_u32(self.nodes.len() - 1));
		self.nodes.len() - 1
	    },
	    Token::Lparen => {
		let lhs_idx = self.expr(0.0);
		match self.peek(){
		    Token::Rparen => self.eat(),
		    _ => panic!("[Line {}] Expected  ')' ", self.lines[as_usize(self.current)])
		};
		as_usize(lhs_idx)
	    },
	    Token::CmpNot => {
		let current = self.current - 1;
		let rhs_idx = match self.peek(){
		    Token::Lparen => self.expr(1000.0),
		    _ => panic!(
			"[Line {}] 'not' must combine with '(' ", self.lines[as_usize(self.current)]
		    )
		};
		let node = Node::init(Token::CmpNot, current, NONE, rhs_idx);
		
		self.nodes.push(node);
		self.edges.push(as_u32(self.nodes.len() - 1));
		self.nodes.len() - 1
	    }
	    _ => panic!("wth")
	};

	loop{
	    let op_token = match self.peek(){
		Token::EOF | Token::Scolon | Token::Rparen => break,
		op_token => self.peek().clone()
	    };
	    
	    let (l_bp, r_bp) = infix_binding_power(&op_token);
	    
	    if l_bp < min_bp{
		break;
	    }

	    self.eat();

	    let op_idx = self.current - 1;

	    let rhs_idx = self.expr(r_bp);
	    
	    

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

    fn eat(&mut self) -> Token{
	let curr = as_usize(self.current);
	self.current += 1;
	self.tokens[curr].clone()
    }

    fn peek(&mut self) -> Token{
	self.tokens[as_usize(self.current)].clone()
    }

    fn peek_next(&mut self) -> Token{
	self.tokens[as_usize(self.current + 1)].clone()
    }

    pub fn print_output(&mut self){
	for (i, node) in self.nodes.iter().enumerate(){
	    println!("Node : {i}, {:?} ", node);
	}
	
	for (i, edge) in self.edges.iter().enumerate(){
	    println!("edge : {i} {:?}", edge);
	}
    }

    
}



fn infix_binding_power(op : &Token) -> (f32, f32){
    match op{
        Token::CmpAnd | Token::CmpOr     => (0.1, 0.2),
        Token::EqEq   | Token::NotEq     => (0.3, 0.3),
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
    let input = "for i in [0  -> 3]{}".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    lexer.print_output();
    let mut parser = Parser::init(lexer.input, lexer.tokens, lexer.lines, lexer.itervals);
    parser.let_stmt();
    parser.print_output();
}
