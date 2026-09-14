use crate::par::ty::Type;
use crate::lex::token::Token;
use crate::par::node::Node;
use crate::par::parser::{Parser, NONE};

impl Parser{
    
    //var_scope_stack, func_scope_stack
    pub fn func_stmt(&mut self) -> u32{
    let mut func_recorder = Vec::new();

    let func_key = self.current;
    let func_tok = self.eat();


    let func_name_key = self.current;
    let func_name_tok = self.eat();

    let func_name_l_edge_idx = self.edges.len() as u32;
    self.edges.push(NONE);
    self.edges.push(NONE);
    let func_name_r_edge_idx = self.edges.len() as u32;

    let func_name_node = Node::init(
	func_name_tok.clone(), func_name_key,
	func_name_l_edge_idx, func_name_r_edge_idx
    );

    self.nodes.push(func_name_node);

    let func_name_node_idx = self.nodes.len() - 1;
    func_recorder.push(func_name_node_idx as u32);

    match self.peek(){
	Token::Lparen => self.eat(),
	_ => panic!("[Line {:?}]", self.lines[self.current as usize])
    };


    loop{
	match self.peek(){
	    Token::Ident =>{
		    let func_ident_key = self.current;
		    let func_ident_tok = self.eat();

		    self.key_checker.push(func_ident_key);


		    let func_ident_l_edge_idx = self.edges.len() as u32;
		    self.edges.push(NONE);
		    self.edges.push(NONE);
		    let func_ident_r_edge_idx = self.edges.len() as u32;

		    let func_ident_node = Node::init(
		        func_ident_tok.clone(), func_ident_key,
		        func_ident_l_edge_idx, func_ident_r_edge_idx
		    );
		    self.nodes.push(func_ident_node);

		    let func_ident_node_idx = self.nodes.len() - 1;
		    func_recorder.push(func_ident_node_idx as u32);

		    func_ident_key
	    },
        Token::Rparen => break,
	    _ => {
            panic!("[Line {:?}] Expected an Identifier", self.lines[self.current as usize])
	    }
	};

	match self.peek(){
	    Token::Colon => self.eat(),
	    _ => panic!("[Line {}] Expected ':'", self.lines[self.current as usize])
	};

	match self.peek(){
	    Token::Ident => {
		let type_idx = self.current;
		self.eat();

		let ty = self.type_mapping(type_idx);
		self.types.push(ty);

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

	match self.peek(){
	    Token::Arrow => {
		self.eat();
		match self.peek(){
		    Token::Ident =>{
			let type_idx = self.current;
			self.eat();
			
			let ty = self.type_mapping(type_idx);
			self.ret_type = ty.clone();
			
			self.key_checker.push(func_name_key);
			self.types.push(ty);
		    }
		    _ => unreachable!()
		}
	    },
	    
	    Token::Lbrace => {},
	    _ => panic!("[Line {:?}] Expected '->'", self.lines[self.current as usize])
    }
	
	
	
	
	let block_idx = self.block_stmt();
	func_recorder.push(block_idx);
	
	let l_edge_idx = self.edges.len() as u32;
	self.edges.extend(func_recorder);
	let r_edge_idx = (self.edges.len() - 1) as u32;
	
	let func_node = Node::init(Token::FuncStmt, func_name_key, l_edge_idx, r_edge_idx);
	self.nodes.push(func_node);
	
	(self.nodes.len() - 1) as u32
    }
    
    pub fn match_stmt(&mut self) -> u32{
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

    
    //todo! : var stack part and the same primary key mapping
    pub fn let_stmt(&mut self) -> u32{
        let let_key = self.current;
        let let_tok = self.eat();
	
	

        let name_idx = match self.peek(){
            Token::Ident =>{
                let var_name_key = self.current;
                self.key_checker.push(var_name_key);
                self.var_scope_stack.push(var_name_key);
		
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

        let node = Node::init(Token::LetStmt , let_key, l_edge_idx, r_edge_idx);
        self.nodes.push(node);
	
        (self.nodes.len() - 1) as u32
    }
    
    
    
    
    pub fn for_stmt(&mut self)->u32{
        self.loop_counter += 1;
	
	
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

        // todo! : function call support(using integer)
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

        // todo! : function call support(using integer)
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


        self.loop_counter -= 1;
        (self.nodes.len() - 1) as u32
    }



    pub fn while_stmt(&mut self)->u32{
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

        let cond_idx = self.expr(0.0);

        if !self.is_bool(cond_idx){
            panic!("[Line {:?} Ask for bool expr", self.lines[self.current as usize])
        }

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

    pub fn brk_stmt(&mut self) -> u32{

        if self.loop_counter <= 0{
            panic!("[Line {:?}] Not in for stmt", self.lines[self.current as usize])
        }

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


    pub fn ctn_stmt(&mut self) -> u32{

        if self.loop_counter <= 0{
            panic!("[Line {:?}] Not in for stmt", self.lines[self.current as usize])
        }

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


    // todo! : func stack check and, and the waiting stack 
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

    pub fn ret_stmt(&mut self) -> u32{

        let ret_key = self.current;
        let ret_tok = self.eat();

        if self.ret_type == Type::Void && self.peek() != Token::Scolon{
            panic!("[Line {:?}]Mismatch return type", self.lines[self.current as usize]);
        }

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

    pub fn if_stmt(&mut self) -> u32{
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


    pub fn record_branch_idx(&mut self, branch_idx_recorder : &mut Vec<u32>){
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

                let cond_idx = self.expr(0.0);
                if !self.is_bool(cond_idx){
                    panic!("[Line {:?} Ask for bool expr", self.lines[self.current as usize])
                }

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

                let cond_idx = self.expr(0.0);
                if !self.is_bool(cond_idx){
                    panic!("[Line {:?} Ask for bool expr", self.lines[self.current as usize])
                }

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



    pub fn block_stmt(&mut self)->u32{
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
    
    
}
