use crate::par::ty::Type;
use crate::lex::token::Token;
use crate::lex::iterval::Interval;
use crate::par::node::Node;
use crate::par::parser::{Parser, NONE, MAGIC};
use crate::par::expr::is_expr;

impl Parser{
    
    pub fn func_stmt(&mut self) -> u32{
	    let mut func_node_idx_recorder = Vec::new();
	    let mut param_type_recorder = Vec::new();
        
	    let func_key = self.current;
	    self.eat();
        
	    let func_name_key = self.current;
	    let func_name_tok = self.eat();
        
	    let func_name_l_edge_idx = self.edges.len() as u32;
	    self.edges.push(NONE);
	    self.edges.push(NONE);
	    let func_name_r_edge_idx = (self.edges.len() - 1) as u32;
	    
	    let func_name_node = Node::init(
	        func_name_tok, func_name_key,
	        func_name_l_edge_idx, func_name_r_edge_idx
	    );
	    self.nodes.push(func_name_node);
        
	    let func_name_node_idx = self.nodes.len() - 1;
	    func_node_idx_recorder.push(func_name_node_idx as u32);
        
	    
	    self.func_assign.push(func_name_key.clone() );
	    
	    match self.peek(){
	        Token::Lparen => self.eat(),
	        _ => panic!("[Line {:?}] Expected a (", self.lines[self.current as usize])
	    };
	    
	    let mut param_count  = 0;
	    loop{
	        match self.peek(){
	            Token::Ident =>{
		            let func_ident_key = self.current;
		            let func_ident_tok = self.eat();
		            
		            self.key_checker.push(func_ident_key);
		            self.var_scope_stack.push(func_ident_key);
		            
		            let func_ident_l_edge_idx = self.edges.len() as u32;
		            self.edges.push(NONE);
		            self.edges.push(NONE);
		            let func_ident_r_edge_idx = (self.edges.len() - 1) as u32;
		            
		            let func_ident_node = Node::init(
		                func_ident_tok.clone(), func_ident_key,
		                func_ident_l_edge_idx, func_ident_r_edge_idx
		            );
		            self.nodes.push(func_ident_node);
		            
		            let func_ident_node_idx = self.nodes.len() - 1;
		            func_node_idx_recorder.push(func_ident_node_idx as u32);
		            
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
		            param_type_recorder.push(ty);
		        }
		        _ => unreachable!()
	        };

            param_count += 1;
            
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


        let ret_ty = if self.peek() == Token::Arrow && self.peek_next() == Token::Ident{
            self.eat();
            let type_key = self.current;
            self.eat();

            let ret_ty = self.type_mapping(type_key);
            self.ret_type = ret_ty.clone();
            ret_ty
        }else if self.peek() == Token::Lbrace{
            Type::Void
        } else{
            panic!("[Line {:?}] Mismatch return type", self.lines[self.current as usize])
        };

        self.ret_types.push(ret_ty);
	    
	    
	    let l_type_idx = if param_count != 0{self.types.len() as u32}else{NONE};
	    self.types.extend(param_type_recorder);	
	    let r_type_idx = if param_count != 0{(self.types.len() - 1) as u32} else {NONE};
        
	    self.param_type_idxs.push(Interval::init(l_type_idx, r_type_idx));
	    
	    let block_idx = self.block_stmt();
	    func_node_idx_recorder.push(block_idx);
	    
	    let l_edge_idx = self.edges.len() as u32;
	    self.edges.extend(func_node_idx_recorder);
	    let r_edge_idx = (self.edges.len() - 1) as u32;
        

	    
	    let func_node = Node::init(Token::FuncStmt, func_key, l_edge_idx, r_edge_idx);
	    self.nodes.push(func_node);
        
	    (self.nodes.len() - 1) as u32
    }
    
    pub fn match_stmt(&mut self) -> u32{
	    dbg!(self.current);
        match self.peek(){
            Token::Let => self.let_stmt(),
            Token::For => self.for_stmt(),
            Token::While => self.while_stmt(),
            Token::If    => self.if_stmt(),
            Token::Brk   => self.brk_stmt(),
            Token::Ctn   => self.ctn_stmt(),
            Token::Ret   => self.ret_stmt(),
            Token::Ident => self.call_stmt(),
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
    
    
    pub fn let_stmt(&mut self) -> u32{
	    let let_key = self.current;
        self.eat();
        
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
                    Token::Ident, var_name_key.clone(), l_edge_idx, r_edge_idx
                );
                
                self.nodes.push(name_node);
                (self.nodes.len() - 1) as u32
            }
            
            t  =>{
                panic!(	"[Line {:?}] Invaild Variable Name : {:?}",
                          self.lines[self.current as usize], t)
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
        
        let start_number_idx = match self.peek(){
            Token::Int => self.expr(0.0),
            Token::Ident | Token::Minus => {
                let expr_idx = self.expr(0.0);
                if !self.is_integer(expr_idx){panic!("Expected Integer")}
                expr_idx
            }
            _ => {
                panic!("[Line {:?}] Start Number Expected Number", self.lines[self.current as usize])
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
                               self.lines[self.current as usize]
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
        self.loop_counter += 1;

        let while_key = self.current;
        self.eat();

        match self.peek(){
            Token::Lparen =>{
                self.eat();
            },
            _ => {
                panic!("[Line {}] Expected '('", self.lines[self.current as usize])
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

        self.loop_counter -= 1;
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
            panic!("[Line {:?}] continue can't not in loop", self.lines[self.current as usize])
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

    

    pub fn call_stmt(&mut self)->u32{
        let idx =self.func_call();

        match self.peek(){
            Token::Scolon => self.eat(),
            _ => panic!("[Line {:?}] Expected ';'", self.lines[self.current as usize])
        };

        idx

    }
    
    pub fn func_call(&mut self) -> u32{
        self.recorder.clear();
        
        let func_name_key = self.current;
        let func_name_tok = self.eat();
        
	    let same_key = self.find_same_func_key(func_name_key);
	    dbg!(same_key);
	
	    let l_edge_idx = self.edges.len() as u32;
	    self.edges.push(NONE);
	    self.edges.push(NONE);
	    let r_edge_idx = (self.edges.len() - 1) as u32;
        
	    let func_name_node = Node::init(func_name_tok, same_key, l_edge_idx, r_edge_idx);
	    self.nodes.push(func_name_node);
        
	    let idx = self.nodes.len() - 1;
	    self.recorder.push(idx as u32);
        
        
        match self.peek(){
            Token::Lparen => {
                self.eat()
            },
            _ => {
                panic!("[Line {:?}] Expected '(' ", self.lines[self.current as usize])
            }
        };
        
	if self.peek() == Token::Int ||
        self.peek() == Token::Float ||
        self.peek() == Token::Ident ||
         self.peek() == Token::Lparen ||
        self.peek() == Token::Minus{
	    loop {
		    let expr_idx = self.expr(0.0);
		    self.recorder.push(expr_idx);
            match self.peek() {
                Token::Rparen => break,
                Token::Comma => self.eat(),
                _ => break,
		    };
	    }
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
            Token::CallStmt, same_key, l_edge_idx, r_edge_idx
        );
        
        self.nodes.push(func_call_node);
        
	    let idx = (self.nodes.len() - 1) as u32;
        (self.nodes.len() - 1) as u32
    }
    
    pub fn ret_stmt(&mut self) -> u32{
        
        let ret_key = self.current;
        let ret_tok = self.eat();
        
        
        if self.ret_type != Type::Void && self.peek() == Token::Scolon{
            panic!("[Line {:?}]Mismatch return type", self.lines[self.current as usize]);
        }

        let r_node = if self.ret_type != Type::Void{
        self.expr(0.0)
        } else{
          NONE
        };

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
        self.edges.push(r_node);
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
                        panic!("[Line {}] Expected '('", self.lines[self.current as usize])
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
                        panic!("[Line {}] Expected '(' ", self.lines[self.current as usize])
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
                        panic!("[Line {}] Expected ')' ", self.lines[self.current as usize])
                    }
                };
                
                let block_idx = self.block_stmt();
                
                self.recorder.clear();
                branch_idx_recorder.push(cond_idx);
                branch_idx_recorder.push(block_idx);
            },
            Token::Else => {
                if self.recorder.len() < 1{
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

	    self.var_scope_stack.push(MAGIC);

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
        
	    let mut i = self.var_scope_stack.len() - 1;
	    while self.var_scope_stack[i] != MAGIC{
	        self.var_scope_stack.pop();
	        i -= 1;
	    }
	    self.var_scope_stack.pop();
        
        let l_edge_idx = self.edges.len() as u32;
        self.edges.extend(self.recorder.clone());
        let r_edge_idx = (self.edges.len() - 1) as u32;
	
        self.nodes.push(Node::init(Token::BlockStmt, block_key, l_edge_idx, r_edge_idx));
        (self.nodes.len() - 1) as u32
    }
}
