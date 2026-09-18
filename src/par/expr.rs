use crate::lex::token::Token;
use crate::par::node::Node;
use crate::par::parser::{Parser, NONE, MAGIC};

impl Parser{
    pub fn expr(&mut self, min_bp : f32) -> u32{
        let lhs_tok = self.peek();
        
        let mut lhs_idx = match self.peek(){
            Token::Int =>{
		        let key = self.current;
		        self.eat();
		        
                let l_edge_idx = self.edges.len() as u32;
                
                self.edges.push(NONE);
                self.edges.push(NONE);
                
                let r_edge_idx = (self.edges.len() - 1) as u32;
                let node = Node::init(Token::Int, key, l_edge_idx, r_edge_idx);
                self.nodes.push(node);
                self.nodes.len() - 1
            },
            Token::Float =>{
		        let key = self.current;
		        self.eat();
		        
                let l_edge_idx = self.edges.len() as u32;
                
                self.edges.push(NONE);
                self.edges.push(NONE);
                
                let r_edge_idx = (self.edges.len() - 1) as u32;
                let node = Node::init(Token::Float, key, l_edge_idx, r_edge_idx);
                self.nodes.push(node);
                self.nodes.len() - 1
            },
            Token::True =>{
                let key = self.current;
		        self.eat();
		        
                let l_edge_idx = self.edges.len() as u32;
                
                self.edges.push(NONE);
                self.edges.push(NONE);
                
                let r_edge_idx = (self.edges.len() - 1) as u32;
                let node = Node::init(Token::True, key, l_edge_idx, r_edge_idx);
                self.nodes.push(node);
                self.nodes.len() - 1
            },
            Token::False =>{
                let key = self.current;
		        self.eat();
		        
                let l_edge_idx = self.edges.len() as u32;
                
                self.edges.push(NONE);
                self.edges.push(NONE);
                
                let r_edge_idx = (self.edges.len() - 1) as u32;
                let node = Node::init(Token::False, key, l_edge_idx, r_edge_idx);
                self.nodes.push(node);
                self.nodes.len() - 1
            },
            Token::Ident =>{
		        if self.peek_next() == Token::Lparen{
		             self.func_call() as usize
		        }else if is_expr(self.peek_next()) ||
                    self.peek_next() == Token::Scolon ||
                    self.peek_next() == Token::Rparen{
                        
		            let ident_var_key = self.current;
		            self.eat();
		            let same_key = self.find_the_same_var_key(ident_var_key);
		            let same_key_type = self.find_ident_type(same_key);
		            
		            let l_edge_idx = self.edges.len() as u32;
		            
                    self.edges.push(NONE);
                    self.edges.push(NONE);
                    
                    let r_edge_idx = (self.edges.len() - 1) as u32;
                    let node = Node::init(same_key_type, same_key, l_edge_idx, r_edge_idx);
                    self.nodes.push(node);
                    self.nodes.len() - 1
		        }else{
                    panic!("[Line {:?}] Expected Identifier", self.lines[self.current as usize])
		        }

            }
            Token::Minus => {
                let unary_key = self.current;
                self.eat();
                
                let expr_idx = self.expr(1000.0);
                let l_edge_idx = self.edges.len() as u32;
                self.edges.push(NONE);
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

            if (lhs_tok == Token::True || lhs_tok == Token::False) &&
                (op_token == Token::Plus || op_token == Token::Minus ||
                 op_token == Token::Mul || op_token == Token::Div || op_token == Token::Mod ){
                    panic!("[Line {:?}] Cannot using {:?} on bool ",
                           self.lines[self.current as usize],
                           op_token
                    );
                }

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
    
    fn find_the_same_var_key(&mut self, input_key : u32) -> u32{
	    let mut current = self.var_scope_stack.len() - 1;
	    
	    loop {
	        let current_key = self.var_scope_stack[current];
            
	        if current_key != MAGIC && self.is_match_str(input_key, current_key) {
		        return current_key;
		        
            }

	        if current == 0 {break;}
            
        current -= 1;
	    }
        
	    panic!("[Line {:?}] the variable not exist", self.lines[self.current as usize]);
	    
    }
    
    pub fn find_same_func_key(&mut self, input_key : u32) -> u32{
	    let mut current = 0;
	    
	    loop {
	        let current_key = self.func_assign[current];
            
	        if self.is_match_str(input_key, current_key) {
		        return current_key;
		        
            }
            
	        if current == 0 {break;}
            
            current += 1;
	}
        
	    panic!("[Line {:?}] Can't find this function", self.lines[self.current as usize]);
	    
    }
    
    
    
    
    
    pub fn is_match_str(&self, input_key : u32, host_key : u32) -> bool{
	    let input_interval = &self.intervals[input_key as usize];
	    let host_interval = &self.intervals[host_key as usize];
        
	    let input_start = input_interval.start as usize;
	    let input_end   = input_interval.end as usize;
        
	    let host_start  = host_interval.start as usize;
	    let host_end    = host_interval.end as usize;
        
	    let input_str = &self.input[input_start..input_end];
	    let host_str  = &self.input[host_start..host_end];
        
        
	    input_str == host_str
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

pub fn is_expr(peek : Token) -> bool{
    match peek{
	    Token::Int | Token::Float | Token::Ident | Token::Lparen | Token::Plus |
        Token::Minus | Token::Mul | Token::Div | Token::Mod | Token::Grt | Token::GrtEq |
        Token::Les | Token::LesEq | Token::EqEq
        => true,
	    _ => false
    }
}
