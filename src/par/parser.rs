use crate::lex::token::Token;
use crate::lex::lexer::Lexer;
use crate::lex::iterval::Iterval;
use crate::par::node::Node;
use crate::par::ty::Type;

pub const NONE : u32 = u32::MAX;

pub struct Parser{
    pub input           : String,
    pub tokens          : Vec<Token>,
    pub lines           : Vec<u32>,
    pub itervals        : Vec<Iterval>,
    pub types           : Vec<Type>,
    pub edges           : Vec<u32>,
    pub nodes           : Vec<Node>, 
    pub key_checker     : Vec<u32>,
    pub l_type_idxs     : Vec<u32>,
    pub r_type_idxs     : Vec<u32>,

    var_scope_stack     : Vec<u32>,
    func_scope_stack    : Vec<u32>,

    recorder            : Vec<u32>,
    ret_type            : Type,
    loop_counter        : u32,
    current             : u32
    
}

impl Parser{
    pub fn init(
	input     : String,
	tokens    : Vec<Token>,
	lines     : Vec<u32>,
	itervals  : Vec<Iterval>
    ) -> Self{
	Parser{
	    input,
	    tokens,
	    lines,
	    itervals,
	    types              : Vec::new(),
	    edges              : Vec::new(),
	    nodes              : Vec::new(),
	    key_checker        : Vec::new(),
	    l_type_idxs        : Vec::new(),
	    r_type_idxs        : Vec::new(),
	    
	    var_scope_stack    : Vec::new(),
	    func_scope_stack   : Vec::new(),

	    recorder           : Vec::new(),
	    ret_type           : Type::Void,
	    loop_counter       : 0,
	    current            : 0 
	}
    }

    //var_scope_stack, func_scope_stack and types
    fn func_stmt(&mut self) -> u32{
	let mut func_recorder = Vec::new();
	
	let func_tok = self.eat();
	
	let func_name_key = self.current;
	let func_name_tok = self.eat();

	


	match self.peek(){
	    Token::Lparen => self.eat(),
	    _ => panic!("[Line {:?}]", self.lines[self.current as usize])
	};

	
	loop{
	    match self.peek(){
		Token::Ident =>{
		    let func_ident_key = self.current;
		    let func_ident_tok = self.eat();

		    let func_ident_l_edge_idx = self.edges.len() as u32;
		    self.edges.push(NONE);
		    self.edges.push(NONE);
		    let func_ident_r_edge_idx = self.edges.len() as u32;

		    let func_ident_node = Node::init(
			func_ident_tok.clone(), func_ident_key,
			func_ident_l_edge_idx, func_ident_r_edge_idx
		    );
		    self.nodes.push(func_ident_node);

		    let func_name_node_idx = self.nodes.len() - 1;
		    func_recorder.push(func_name_node_idx as u32);
		}
		_ => {
		    panic!("[Line {:?}] Expected an Identifier", self.lines[self.current as usize])
		}
	    };

	    match self.peek(){
		Token::Colon => self.eat(),
		_ => panic!("[Line {}] Expected ':'", self.lines[self.current as usize])
	    };

	    // todo! : type mapping and record
	    match self.peek(){
		Token::Ident => {
		    self.eat()
		}
		_ => unreachable!()
	    };

	    match self.peek(){
        Token::Rparen => break,
		Token::Comma => self.eat(),
		_ => break,
	    };
	    
	}
	match self.peek(){
	    Token::Rparen => self.eat(),
	    _ => {
		panic!("[Line {:?}] Expected ')' ", self.lines[self.current as usize])
	    }
	};
	let block_idx = self.block_stmt();
	func_recorder.push(block_idx);

	let l_edge_idx = self.edges.len() as u32;
	self.edges.extend(func_recorder);
	let r_edge_idx = (self.edges.len() - 1) as u32;

	let func_node = Node::init(Token::FuncStmt, func_name_key, l_edge_idx, r_edge_idx);
	self.nodes.push(func_node);

	(self.nodes.len() - 1) as u32

	

    }
    
    fn match_stmt(&mut self) -> u32{
	match self.peek(){
	    Token::Let => self.let_stmt(),
	    Token::For => self.for_stmt(),
	    Token::While => self.while_stmt(),
	    Token::If    => self.if_stmt(),
	    Token::Brk   => self.brk_stmt(),
	    Token::Ctn   => self.ctn_stmt(),
	    Token::Ret   => self.ret_stmt(),
	    Token::Ident => self.func_call_stmt(),
	    Token::Func  =>{
		panic!(
		    "[Line {:?}] Function shouldn't on other function",
		    self.lines[self.current as usize]
		)
	    } 
	    _ => panic!(
		"[Line {:?}] This Statment token isn't exist",
		self.lines[self.current as usize]
	    )
	};
	
	(self.nodes.len() - 1) as u32
	    
	
    }

    
    //todo! : var stack part
    fn let_stmt(&mut self) -> u32{
	let let_key = self.current;
	let let_tok = self.eat();


	let var_name_key = self.current;

	let name_idx = match self.peek(){
	    Token::Ident =>{
		self.eat();
		let l_edge_idx = self.edges.len() as u32;
		self.edges.push(NONE);
		self.edges.push(NONE);
		let r_edge_idx = (self.edges.len() - 1) as u32;
		let name_node = Node::init(
		    Token::Ident, var_name_key.clone() , l_edge_idx , r_edge_idx
		);
		
		self.nodes.push(name_node);
		(self.nodes.len() - 1) as u32
	    }
	    
	    t  =>{
		panic!(	"[Line {:?}] Invaild Variable Name : {:?}",
			  self.lines[(self.current) as usize], t)
	    }
	    
	};

	let expected_type = match self.peek(){
	    Token::Colon =>{
		self.eat();
		match self.peek(){
                    Token::Ident =>{
			let type_ident_idx = self.current;
			
			self.key_checker.push(var_name_key);
			
			self.eat();
			
			self.type_mapping(type_ident_idx)
			
                    }
                    t => {
			panic!("[Line {:?}] Unexpected type, {:?}", 
                               self.lines[self.current as usize],t)
                    }
		}
		
        },

        Token::Equel => Type::Void, 
        t  => {
            panic!("[Line {:?}] Unknown token, {:?}", self.lines[self.current as usize], t)
        }
    };

	match self.peek(){
	    Token::Equel => {
		self.eat()
	    },
	    _            => {
		panic!("[Line {:?}] expectdd '=' ", self.lines[self.current as usize])
	    }
	};

	let expr_idx = self.expr(0.0);
	
	let l_edge_idx = self.edges.len() as u32;
	self.edges.push(name_idx);
	self.edges.push(expr_idx);
	let r_edge_idx = (self.edges.len() - 1) as u32;
	
	self.type_check(expected_type, expr_idx);
	match self.peek(){
	    Token::Scolon => {
		self.eat()
	    },
	    _             => {
	    panic!("[Line {:?}] expected ';' ", self.lines[self.current as usize])
	    }
 	};

	self.key_checker.push(var_name_key);

	let node = Node::init(Token::LetStmt , let_key, l_edge_idx, r_edge_idx);
	self.nodes.push(node);
	
	(self.nodes.len() - 1) as u32
    }




    // todo! :  loop_counter for brk_stmt and ctn_stmt
    fn for_stmt(&mut self)->u32{
	let for_key = self.current;

	self.eat();

	let ident_idx = match self.peek(){
	    Token::Ident => {
		let ident_key = self.current;
		let ident_tok = self.eat();
		
		let l_edge_idx = self.edges.len() as u32;
		self.edges.push(NONE);
		self.edges.push(NONE);
		let r_edge_idx = (self.edges.len() - 1) as u32;
		
                let ident_node = Node::init(
 		    ident_tok, ident_key, l_edge_idx, r_edge_idx
		);
		
                self.nodes.push(ident_node);
                (self.nodes.len() - 1) as u32
	    },
	    _            => {
		panic!("[Line {:?}] Expected identifier", self.lines[self.current as usize])
	    }  
	};

        match self.peek(){
	    Token::In => {
		self.eat();
	    },
	    _         => {
                panic!("[Line {:?}] Expected 'in' ",self.lines[self.current as usize])
	    }
	};

	match self.peek(){
	    Token::Lbracket => {
		self.eat();
	    },
	    _               => {
		panic!("[Line {:?}] Expected '[' ", self.lines[self.current as usize])
	    }
	};

	// todo! : function call support(using integer)
	let start_number_idx = match self.peek(){
	    Token::Int => self.expr(0.0),
	    Token::Ident | Token::Minus => {
		let expr_idx = self.expr(0.0);
		if !self.is_integer(expr_idx){panic!("Expected Integer")}
		expr_idx
	    }
	    _ => {
		panic!("[Line {:?}] Start Number Expected Number", self.lines[(self.current) as usize])
	    }
	};

	match self.peek(){
	    Token::Arrow => {
		self.eat();
	    },
	    _            => {
		panic!("[Line {:?}] Expected '->' ", self.lines[(self.current) as usize])
	    }
	}

	// todo! : Add Expr, type checker, function call support(using integer)
	let end_number_idx = match self.peek(){
	    Token::Int => self.expr(0.0),
	    Token::Ident | Token::Minus => {
		let expr_idx = self.expr(0.0);
		if !self.is_integer(expr_idx){panic!("Expected Integer")}
		expr_idx
	    }
	    _ => {
		panic!("[Line {:?}] End Number Expected Number", self.lines[(self.current) as usize])
	    }
	};

	// todo! : Add Expr, type checker, function call support(using integer)
	let step_number_idx = match self.peek(){
	    Token::Vartical =>{
		self.eat();
		match self.peek(){
		    Token::Int => self.expr(0.0),
		    Token::Ident | Token::Minus => {
			let expr_idx = self.expr(0.0);
			if !self.is_integer(expr_idx){panic!("Expected Integer")}
			expr_idx
		    }
		    _ => {
			panic!("[Line {:?}] Step Number Expected Number",
			       self.lines[(self.current) as usize]
			)
		    }
		}
	    },
	    
	    _ => {
		panic!(" [Line {:?}] Step part expected an '|' "
		       ,self.lines[(self.current) as usize])
	    }
	};

	match self.peek(){
	    Token::Rbracket => {
		self.eat();
	    },
	    _               => {
		panic!("[Line {:?}] Expected ']' ", self.lines[(self.current) as usize])
	    }
	};

	let block_idx = self.block_stmt();
	let l_edge_idx = self.edges.len() as u32;
	
	self.edges.push(ident_idx);
	self.edges.push(start_number_idx);
	self.edges.push(end_number_idx);
	self.edges.push(step_number_idx);
	self.edges.push(block_idx);
	
	let r_edge_idx = (self.edges.len() - 1) as u32;
	let node = Node::init(Token::ForStmt, for_key, l_edge_idx, r_edge_idx);
	
        self.nodes.push(node);
	(self.nodes.len() - 1) as u32
    }

    

    fn while_stmt(&mut self)->u32{
	let while_key = self.current;
	self.eat();

	match self.peek(){
	    Token::Lparen =>{
		self.eat();
	    },
	    _ => {
		panic!("[Line {}] Expected '('", self.lines[(self.current) as usize])
	    }
	};

	let cond_idx = self.condition();

	match self.peek(){
	    Token::Rparen =>{
		self.eat();
	    },
	    _ => {
		panic!("[Line {}] Expected ')'", self.lines[self.current as usize])
	    }
	};

	let block_idx = self.block_stmt();

	let l_edge_idx = self.edges.len() as u32;
	self.edges.push(cond_idx);
	self.edges.push(block_idx);
	let r_edge_idx = (self.edges.len() - 1) as u32;
	self.nodes.push(Node::init(Token::WhileStmt, while_key, l_edge_idx, r_edge_idx));
	(self.nodes.len() - 1) as u32
    }

    // add loop checker
    fn brk_stmt(&mut self) -> u32{
	
	let brk_key = self.current;
	let brk_tok = self.eat();

	match self.peek(){
	    Token::Scolon =>{
		self.eat()
	    },
	    _ => {
		panic!("[Line {:?}] expected ';' ", self.lines[self.current as usize])
	    }
	};
	
	let l_edge_idx = self.edges.len() as u32;
	self.edges.push(NONE);
	self.edges.push(NONE);
	let r_edge_idx = (self.edges.len() - 1) as u32;
	
	let brk_node = Node::init(brk_tok, brk_key, l_edge_idx, r_edge_idx);
	self.nodes.push(brk_node);
	
	(self.nodes.len() - 1) as u32
    }


    // add loop checker
    fn ctn_stmt(&mut self) -> u32{
	
	let ctn_key = self.current;
	let ctn_tok = self.eat();

	match self.peek(){
	    Token::Scolon =>{
		self.eat()
	    },
	    _ => {
		panic!("[Line {:?}] expected ';' ", self.lines[self.current as usize])
	    }
	};
	
	let l_edge_idx = self.edges.len() as u32;
	self.edges.push(NONE);
	self.edges.push(NONE);
	let r_edge_idx = (self.edges.len() - 1) as u32;
	
	let ctn_node = Node::init(ctn_tok, ctn_key, l_edge_idx, r_edge_idx);
	self.nodes.push(ctn_node);
	
	(self.nodes.len() - 1) as u32
    }


    // todo! : func stack check and, record to types
    fn func_call_stmt(&mut self) -> u32{
	self.recorder.clear();

	let func_name_key = self.current;
    self.eat();

	
	match self.peek(){
	    Token::Lparen => {
		self.eat()
	    },
	    _ => {
		panic!("[Line {:?}] Expected '(' ", self.lines[self.current as usize])
	    }
	};

	
	loop{
	    let expr_idx = self.expr(0.0);
	    self.recorder.push(expr_idx);

	    match self.peek(){
		Token::Rparen => break,
		Token::Comma  => self.eat(),
		_ => break,
	    };
	    
	}

	match self.peek(){
	    Token::Rparen => self.eat(),
	    _ =>{
		panic!("[Line {:?}] Expected ')' ", self.lines[self.current as usize])
	    }
	};

	let l_edge_idx = self.edges.len() as u32;
	self.edges.extend(self.recorder.clone());
	let r_edge_idx = (self.edges.len() - 1) as u32;
	
	let func_call_node = Node::init(
        Token::CallStmt, func_name_key, l_edge_idx, r_edge_idx
    );

	self.nodes.push(func_call_node);
	
	(self.nodes.len() - 1) as u32
    }

    //todo! : add expr type check, also void means don't need any return expr
    fn ret_stmt(&mut self) -> u32{
	
	let ret_key = self.current;
	let ret_tok = self.eat();
	let expr_idx = self.expr(0.0);
	
	match self.peek(){
	    Token::Scolon => {
		self.eat()
	    },
	    _ => {
		panic!("[Line {:?}] Expected ';'", self.lines[self.current as usize])
	    }
	};
	
	let l_edge_idx = self.edges.len() as u32;
	self.edges.push(NONE);
	self.edges.push(expr_idx);
	let r_edge_idx = (self.edges.len() - 1) as u32;

	let ret_node = Node::init(ret_tok, ret_key, l_edge_idx, r_edge_idx);
	self.nodes.push(ret_node);

	
	(self.nodes.len() - 1) as u32
    }

    fn if_stmt(&mut self) -> u32{
	let if_key = self.current;
	let mut branch_idx_recorder = Vec::new();
	self.record_branch_idx(&mut branch_idx_recorder);
	
	let l_edge_idx = self.edges.len() as u32;
	self.edges.extend(self.recorder.clone());
	let r_edge_idx = (self.edges.len() - 1) as u32;

	let if_node = Node::init(Token::IfStmt, if_key, l_edge_idx, r_edge_idx);
	self.nodes.push(if_node);

	
	(self.nodes.len() - 1) as u32
    }
 
    
    fn record_branch_idx(&mut self, branch_idx_recorder : &mut Vec<u32>){
	match self.peek(){
	    Token::If   => {
		self.eat();
		match self.peek(){
		    Token::Lparen =>{
			self.eat();
		    },
		    _ => {
			panic!("[Line {}] Expected '('", self.lines[(self.current) as usize])
		    }
		};
		
		let cond_idx = self.condition();
		
		match self.peek(){
		    Token::Rparen =>{
			self.eat();
		    },
		    _ => {
			panic!("[Line {}] Expected ')'", self.lines[(self.current) as usize])
		    }
		};

		self.recorder.clear();
		let block_idx = self.block_stmt();
		branch_idx_recorder.push(cond_idx);
		branch_idx_recorder.push(block_idx);
	    },
	    Token::Elif => {
		if self.recorder.len() < 2{
		    panic!(
			"[Line {:?}] Expected 'if' token, but found 'elif'",
			self.lines[self.current as usize]
		    )
		}

		self.eat();

		match self.peek(){
		    Token::Lparen =>{
			self.eat();
		    },
		    _ => {
			panic!("[Line {}] Expected '(' ", self.lines[(self.current) as usize])
		    }
		};

		let cond_idx = self.condition();

		match self.peek(){
		    Token::Rparen =>{
			self.eat();
		    },
		    _ => {
			panic!("[Line {}] Expected ')' ", self.lines[(self.current) as usize])
		    }
		};

		let block_idx = self.block_stmt();

		self.recorder.clear();
		branch_idx_recorder.push(cond_idx);
		branch_idx_recorder.push(block_idx);
	    },
	    Token::Else => {
		if self.recorder.len() < 2{
		    panic!(
			"[Line {:?}] Expected 'if' token, but found 'else'",
			self.lines[self.current as usize]
		    )
		}
		
		self.eat();
		let block_idx = self.block_stmt();

		self.recorder.clear();
		branch_idx_recorder.push(NONE);
		branch_idx_recorder.push(block_idx);
	    }
	    _ => unreachable!()
	};

	match self.peek(){
	    Token::Elif | Token::Else => self.record_branch_idx(branch_idx_recorder),
	    _ => return
	}
    }

    

    fn block_stmt(&mut self)->u32{
	let block_key = self.current;
	
	match self.peek(){
	    Token::Lbrace => {
		self.eat()
	    },
	    
	    _ => {
		panic!("[Line {}] Expected '{}' ", self.lines[self.current as usize], '{')
	    }
	};

	while self.peek() != Token::Rbrace{
	    let stmt_idx = self.match_stmt();
	    self.recorder.push(stmt_idx);
	}
	


	match self.peek(){
	    Token::Rbrace => {
		self.eat()
	    },
	    _ => {
		panic!("[Line {}] Expected {} ", self.lines[self.current as usize], '}')
	    }
	};

	let l_edge_idx = (self.edges.len()) as u32;
	self.edges.extend(self.recorder.clone());
	let r_edge_idx = (self.edges.len() - 1) as u32;
	
	self.nodes.push(Node::init(Token::BlockStmt, block_key, l_edge_idx, r_edge_idx));
	(self.nodes.len() - 1) as u32
    }



    

    fn expr(&mut self, min_bp : f32) -> u32{
	let mut lhs_idx = match self.eat(){
	    Token::Int   =>{
		let l_edge_idx = self.edges.len() as u32;
		
		self.edges.push(NONE);
		self.edges.push(NONE);
		
		let r_edge_idx = (self.edges.len() - 1) as u32;
		let node = Node::init(Token::Int, self.current - 1, l_edge_idx, r_edge_idx);
		self.nodes.push(node);
		self.nodes.len() - 1
	    },
	    Token::Float =>{
		let l_edge_idx = self.edges.len() as u32;
		
		self.edges.push(NONE);
		self.edges.push(NONE);
		
		let r_edge_idx = (self.edges.len() - 1) as u32;
		let node = Node::init(Token::Int, self.current - 1, l_edge_idx, r_edge_idx);
		self.nodes.push(node);
		self.nodes.len() - 1
	    },
	    Token::Ident =>{
		let l_edge_idx = self.edges.len() as u32;
		
		self.edges.push(NONE);
		self.edges.push(NONE);
		
		let r_edge_idx = (self.edges.len() - 1) as u32;
		let node = Node::init(Token::Int, self.current - 1, l_edge_idx, r_edge_idx);
		self.nodes.push(node);
		self.nodes.len() - 1
	    }
	    Token::Minus => {
		let unary_key = self.current - 1;
		
		let expr_idx = self.expr(1000.0);
		self.edges.push(NONE);
		let l_edge_idx = (self.edges.len() - 1) as u32;
		self.edges.push(expr_idx);
		let r_edge_idx = (self.edges.len() - 1) as u32;
		
		let node = Node::init(Token::Minus, unary_key, l_edge_idx, r_edge_idx);
		self.nodes.push(node);
		self.nodes.len() - 1
	    },
	    Token::Lparen => {
		let lhs_idx = self.expr(0.0);
		match self.peek(){
		    Token::Rparen => self.eat(),
		    _ => panic!("[Line {}] Expected  ')' ", self.lines[self.current as usize])
		};
		lhs_idx as usize
	    },
	    Token::CmpNot => {
		let cmp_not_key = self.current - 1;
		
		let expr_idx = match self.peek(){
		    Token::Lparen =>{
			let idx = self.expr(1000.0);
			idx as u32
		    },
		    _ => {
			panic!(
			    "[Line {}] 'not' must combine with '(' ",
			    self.lines[self.current as usize]
			)
		    }
		};
		
		self.edges.push(NONE);
		let l_edge_idx = (self.edges.len() - 1) as u32;
		self.edges.push(expr_idx);
		let r_edge_idx = (self.edges.len() - 1) as u32;
		let node = Node::init(Token::CmpNot, cmp_not_key, l_edge_idx, r_edge_idx);
		
		self.nodes.push(node);
		self.nodes.len() - 1
	    }
	    _ => panic!("[Line {:?}] Expected a expression", self.lines[self.current as usize])
	};

	loop{
	    let op_token = match self.peek(){
		Token::EOF | Token::Scolon | Token::Rparen |
		Token::Comma | Token::Arrow | Token::Vartical|
		Token::Rbracket => break,
		
		_true => self.peek().clone()
	    };
	    
	    let (l_bp, r_bp) = infix_binding_power(&op_token);
	    
	    if l_bp < min_bp{
		break;
	    }

	    let op_key = self.current;
	    self.eat();
	    
	    let rhs_idx = self.expr(r_bp);

            let l_edge_idx = self.edges.len() as u32;
            self.edges.push(lhs_idx as u32);

	    self.edges.push(rhs_idx);

	    let r_edge_idx = (self.edges.len() - 1) as u32;


	    self.nodes.push(
		Node::init(
		    op_token,
		    op_key,
		    l_edge_idx,
		    r_edge_idx
		)
	    );

	    lhs_idx = self.nodes.len() - 1;
	}
	lhs_idx as u32
	
    }

    fn type_mapping(&mut self, type_ident_idx : u32)-> Type{
        let iterval = self.itervals[type_ident_idx as usize].clone();
        let start = iterval.start as usize;
	let end   = iterval.end as usize;
        let type_str_slice = &self.input[start..end];
        
        match type_str_slice{
            "i32"  => Type::i32,
            "i64"  => Type::i64,
            "f32"  => Type::f32,
            "f64"  => Type::f64,
            "bool" => Type::bool,
            t => panic!("[Line {:?}] Unknown Type {:?}]", self.lines[self.current as usize], t)
        }
    }

    //y1 + y2 = x + 1
    fn type_check(&mut self, expected_type : Type, expr_idx : u32){
	let mut y1 = 0; // y1
	let mut y2 = 0; // y2
	let mut x = 0;  // x

	// y = y1 + y2 = x + 1

	
	let (result_token, y1, y2,x) = self.expr_type(expr_idx, y1, y2, x);

    dbg!(y1);
    dbg!(y2);
    dbg!(x);


	
	let l_type_idx = self.types.len() as u32;
	
	match (&expected_type, &result_token){
	    (Type::i32, Token::Int)   => self.types.push(Type::i32),
	    (Type::i64, Token::Int)   => self.types.push(Type::i32),
	    
	    (Type::f32, Token::Float) => self.types.push(Type::f32),
	    (Type::f64, Token::Float) => self.types.push(Type::f32),

	    (Type::bool, Token::Bool) => self.types.push(Type::bool),

	    
	    (Type::Void,Token::Int)   => self.types.push(Type::i32),
	    (Type::Void,Token::Float) => self.types.push(Type::f64),
	    (Type::Void, Token::Bool) => self.types.push(Type::bool),
	    
	    
	    _ =>{
		panic!("[Line {:?}] Expected {:?}, but found {:?}",
		       self.lines[self.current as usize],
		       expected_type,
		       result_token
		)
	    }
	    
	};

	let r_type_idx = (self.types.len() - 1) as u32;

	self.l_type_idxs.push(l_type_idx);
	self.r_type_idxs.push(r_type_idx);
    }

    // todo! : add identitier part, 
    fn expr_type(
        &mut self, expr_idx : u32,
        mut y1 : u16,
        mut y2 : u16,
        mut x  : u16,
        ) -> (Token, u16, u16, u16) {
	let node = &self.nodes[expr_idx as usize];
	
	let tok = node.token.clone();
	
	let l_edge_idx = node.l_edge_idx as usize;
	let r_edge_idx = node.r_edge_idx as usize;
	
	let l_idx = self.edges[l_edge_idx];
	let r_idx = self.edges[r_edge_idx];

	match (l_idx, r_idx){
	    (NONE, NONE) => {
		match tok{
		    Token::Int   => return (Token::Int, y1, y2, x),
		    Token::Float => return (Token::Float, y1, y2, x),
		    Token::Ident => todo!(),
		    _ => unreachable!()
		}
	    },
	    (NONE, r_idx) => {
		return self.expr_type(r_idx, y1, y2, x);
	    }

	    (l_idx, r_idx) => {
		//todo y1 & y2 == y1 or y2
		let (l_type, _, _, _) = self.expr_type(l_idx, y1, y2, x);
		let (r_type, _, _, _) = self.expr_type(r_idx, y1, y2, x);
		
		if l_type != r_type{
		    panic!("[Line {:?}] Mismatch!!!!", self.lines[self.current as usize]);
		}

	
		return (l_type, y1, y2, x);
	    }
	}
    }



    fn is_bool_expr(node : Node) -> bool{
        let op_tok = &node.token;

        match op_tok{
            Token::Grt | Token::Les | Token::GrtEq | Token::LesEq | Token::EqEq => return true,
            _ => return false
        }

    }

    fn is_cmp_op(tok : Token) -> bool{
        match tok{
            Token::CmpAnd | Token::CmpOr => return true,
            _ => return false

        }   

    }
 
    
    fn is_integer(&mut self, expr_idx : u32) -> bool{
	let(ty,_,_,_ ) =self.expr_type(expr_idx, 0,  0, 0);
    ty== Token::Int
    }

    // todo! : Add is bool type check
    fn condition(&mut self) -> u32{
	self.expr(0.0)
    }

    fn eat(&mut self) -> Token{
	let tok = self.tokens[self.current as usize].clone();
	self.current += 1;
	tok
    }

    fn peek(&mut self) -> Token{
	self.tokens[self.current as usize].clone()
    }


    pub fn print_output(&mut self){
	for (i, node) in self.nodes.iter().enumerate(){
	    println!("Number : {i}");
	    println!("Node : {:?}", node);
	    let l_idx = (node.l_edge_idx) as usize;
	    let r_idx = (node.r_edge_idx) as usize;
	    
	    println!("Edge : {:?}", &self.edges[l_idx..=r_idx ]);
	    
	    let key = node.key;
	    let iterval = &self.itervals[key as usize];
	    let start = iterval.start as usize;
	    let end   = iterval.end   as usize;
	    println!("String : {:?}\n", &self.input[start..end])
	}

	
    }

    pub fn print(&mut self){
	print!(" Key Checker : {:?}\n", self.key_checker);
	print!(" l_type_idxs : {:?}\n", self.l_type_idxs);
	print!(" r_type_idxs : {:?}\n", self.r_type_idxs);
	for (i, ty) in self.types.iter().enumerate(){
	    print!(" Number : {i}, {:?}, ", ty);
	}
	println!("\n");
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
        _         => panic!("Unknown Operator : {:?}", op)
    }
}





#[test]
fn parser_testing() {
    let input = "
    let x = (5 < 1) and(5 > 1);".to_string();
    let mut lexer = Lexer::init(input);
    lexer.tokenize();
    //lexer.print_output();
    let mut parser = Parser::init(lexer.input, lexer.tokens, lexer.lines, lexer.itervals);
    parser.match_stmt();
    parser.print_output();
    parser.print();
}
