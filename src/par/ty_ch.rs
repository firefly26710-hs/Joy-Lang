use crate::lex::token::Token;
use crate::par::node::Node;
use crate::par::parser::{Parser, NONE};
use crate::par::ty::Type;

impl Parser{
    pub fn type_mapping(&mut self, type_ident_idx : u32)-> Type{
        let interval = self.intervals[type_ident_idx as usize].clone();
        let start = interval.start as usize;
        let end   = interval.end as usize;
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

    pub fn type_check(&mut self, expected_type : Type, expr_idx : u32){

        let (expr_token) = self.expr_type(expr_idx);

        let result_token = expr_token;


        match (&expected_type, &result_token){
            (Type::i32, Token::Int)   => self.types.push(Type::i32),
            (Type::i64, Token::Int)   => self.types.push(Type::i32),

            (Type::f32, Token::Float) => self.types.push(Type::f32),
            (Type::f64, Token::Float) => self.types.push(Type::f64),

            (Type::bool, Token::Bool) => self.types.push(Type::bool),
            (Type::bool, Token::True) => self.types.push(Type::bool),
            (Type::bool, Token::False)=> self.types.push(Type::bool),
            
            (Type::Void, Token::Int)   => self.types.push(Type::i32),
            (Type::Void, Token::Float) => self.types.push(Type::f64),
            (Type::Void, Token::True) => self.types.push(Type::bool),
            (Type::Void, Token::False)=> self.types.push(Type::bool),
            (Type::Void, Token::Bool) => self.types.push(Type::bool),
            
            
            _ =>{
                panic!("[Line {:?}] Expected {:?}, but found {:?}",
                       self.lines[self.current as usize],
                       expected_type,
                       result_token
                )
            }
            
        };
        
        
    }

    pub fn expr_type(
        &mut self, expr_idx : u32,
    ) -> (Token) {
        let node = &self.nodes[expr_idx as usize];
        let tok = node.token.clone();

        let l_edge_idx = node.l_edge_idx as usize;
        let r_edge_idx = node.r_edge_idx as usize;

        let l_idx = self.edges[l_edge_idx];
        let r_idx = self.edges[r_edge_idx];



        if tok == Token::CallStmt{
            let key = node.key;
            let ty = self.find_func_type(key);
            self.param_type_check(l_edge_idx, r_edge_idx);
            ty
        }else{
            match (l_idx, r_idx){
                (NONE, NONE) => {
                    match tok{
                        Token::Int   => (Token::Int),
                        Token::Float => (Token::Float),
                        Token::Ident => {
                            let key = node.key;
                            let ty = self.find_ident_type(key);
                            return ty
                        },
                        Token::True =>  return (Token::True),
                        Token::False => return (Token::False),
                        _ => unreachable!()
                    }
                },
                (NONE, r_idx) => {
                    return self.expr_type(r_idx);
                }

                (l_idx, r_idx) => {
                    if self.is_bool_op(tok.clone()){self.x += 1}
                    let l_type = self.expr_type(l_idx);
                    let r_type = self.expr_type(r_idx);

                    if l_type != r_type{
                        panic!("[Line {:?}] Mismatch!!!!", self.lines[self.current as usize]);
                    }

                    if self.is_bool_op(tok) {
                        Token::Bool
                    }else{
                        l_type
                    }

                }
            }
        }
    }
    
    
    pub fn param_type_check(&mut self, l_edge_idx : usize, r_edge_idx : usize){
	    if r_edge_idx == l_edge_idx {return}
	    
	    let start = (l_edge_idx + 1) as usize;
	    let end = r_edge_idx as usize;
	    
	    let params_idx  = self.edges[start..=end].to_vec();
	    let name_idx = &self.edges[(start - 1) as usize];
        
	    let name_node = &self.nodes[name_idx.clone() as usize];
        
	    let key = name_node.key;
        
	    let mut current = 0;
	    while key != self.func_assign[current]{
	        current += 1;
	    }
        
	    let param_types_interval = self.param_type_idxs[current].clone();
        
	    let l_idx = param_types_interval.start as usize;
	    let r_idx = param_types_interval.end as usize;
        
	    let params_type = self.types[l_idx..=r_idx].to_vec();

	    for (i, _) in (&params_idx).iter().enumerate(){
	        let expr_token = self.expr_type(params_idx[i]);

            let result_token = expr_token;

            let ty = &params_type[i];
            
	        match (result_token, ty){
		        (Token::Int, Type::i32) => {},
		        (Token::Int, Type::i64) => {},
                
		        (Token::Float, Type::f32) => {},
		        (Token::Float, Type::f64) => {},
                
		        (Token::Bool, Type::bool) => {},
		        _ =>{
		            panic!("[Line {:?}] Expected Param Type in {i}th", self.lines[self.current as usize])
		        } 
		        
	        }	    
	        
	    }
	    
        
        
	    
    }
    

    
    pub fn is_bool_op(&self, tok : Token) -> bool{
        match tok{
            Token::CmpAnd | Token::CmpOr | Token::Grt | Token::Les | Token::GrtEq | Token::LesEq | Token::EqEq => return true,
            _ => return false
                
        }
        
    }

    pub fn find_ident_type(&mut self, input : u32) -> Token{
	    let mut current = 0;
	    while input != self.key_checker[current as usize]{
	        current += 1;
	    }
        
        
	    let ty = &self.types[current as usize];
	    match ty{
	        Type::i32 | Type::i64 => return Token::Int,
	        Type::f32 | Type::f64 => return Token::Float,
	        Type::bool            => return Token::Bool,
	        Type::Void            => {
		        panic!("[Line {:?}] Can't include a void function in expr",
		               self.lines[self.current as usize]
		        )
	        },
	        _ => {
		        panic!("[Line {:?}] Mismatch variable type", self.lines[self.current as usize])
	        }
	    };
    }
    
    pub fn find_func_type(&mut self, input : u32) -> Token{
        let mut current = 0;
        while input != self.func_assign[current]{
            current  += 1;
        }
        
        let ty = &self.ret_types[current];

        match ty{
            Type::i32 | Type::i64 =>  Token::Int,
            Type::f32 | Type::f64 =>  Token::Float,
            Type::bool            =>  Token::Bool,
            _ => panic!("[Line {:?}] Mismatch function type", self.lines[self.current as usize]) 
        }

    }

    
 
    pub fn match_ret_type(&mut self, expr_idx : u32, expected_type : Type) -> bool{
        let expr_token = self.expr_type(expr_idx);

        let result_token = expr_token;

        match(result_token, expected_type){
            (Token::Int, Type::i32)   => return true,
            (Token::Int, Type::i64)   => return true,
            (Token::Float, Type::f32) => return true,
            (Token::Float, Type::f64) => return true,
            _ => panic!("[Line {:?}] Mismatch return type", self.lines[self.current as usize]),
        }



    }


    pub fn is_integer(&mut self, expr_idx : u32) -> bool{
        let ty= self.expr_type(expr_idx);

        ty == Token::Int
    }

    pub fn is_bool(&mut self, expr_idx : u32) -> bool{
        let ty = self.expr_type(expr_idx);

        ty == Token::Bool
    }

    pub fn is_float(&mut self, expr_idx : u32) -> bool{
        let ty = self.expr_type(expr_idx);

        ty == Token::Float
    }
}
