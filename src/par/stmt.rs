use crate::lex::iterval::Interval;
use crate::par::node::Node;
use crate::lex::token::Token;
use crate::par::parser::{Parser, NONE};
use crate::par::ty::Type;

impl Parser{
     pub fn func_stmt(&mut self) -> u32{

         match self.peek(){
             Token::Func => self.eat(),
             _ => {
                 panic!("[Line {:?}], Can't find function", self.line())
             }
         };

         let func_name_key = self.current as u32;
         let func_name_node_idx = if self.peek() == &Token::Ident{
             self.eat();
             self.nodes.push(
                 Node::new(Token::Ident, func_name_key, Interval::new(NONE, NONE))
             );
             (self.nodes.len() - 1) as u32
         }else{
             panic!("[Line {:?}] Expected a function name", self.line())
         };

         match self.peek(){
             Token::Lparen => self.eat(),
             _ => {
                 panic!("[Line {:?}] Expected a (", self.line())
             }
         };

         let has_declared_params = self.is_expr(self.peek());
         let no_declared_params = self.peek() == &Token::Rparen;
         let error_expr = !self.is_expr(self.peek()) && self.peek() !=&Token::Rparen;
         let no_params_and_no_rparen = !self.is_expr(self.peek()) || self.peek() ==&Token::Rparen;

         self.scope.build_func_scope();
         let params_start = if has_declared_params{self.table.var_types.len() as u32}else{NONE};
         if has_declared_params{
             loop {
                 let param_key = if self.peek() == &Token::Ident && self.peek_next() == &Token::Colon{
                     let key = self.current as u32;
                     self.eat();
                     self.eat();
                     self.nodes.push(
                       Node::new(Token::Ident, key, Interval::new(NONE, NONE))
                     );
                     self.func_recorder.push(key);
                     self.current as u32
                 }else{
                     panic!("[Line {:?}] wtf", self.line())
                 };

                 let parm_type = if self.peek() == &Token::Ident{
                     let type_key = self.current;
                     self.eat();
                     self.type_mapping(type_key)
                 }else{
                     panic!("[Line {:?}] Expected a param type", self.line())
                 };
                 self.table.var_insert(param_key, parm_type);

                 match self.peek(){
                     Token::Rparen => break,
                     Token::Comma => self.eat(),
                     _ => break,
                 };
             }

             match self.peek(){
                 Token::Rparen => self.eat(),
                 _ => {
                     panic!("[Line {:?}] Expected ')'", self.line())
                 }
             };
         }else if no_declared_params{
             self.eat();
         }else if error_expr{
             todo!()
         }else if no_params_and_no_rparen{
             todo!()
         }
         let params_end = if has_declared_params{(self.table.var_types.len() - 1) as u32}else{NONE};

         let has_declared_ret_type =
             self.peek() == &Token::Arrow && self.peek_next() == &Token::Ident;

         let no_declared_ret_type = self.peek() == &Token::Lbrace;

         let only_declare_arrow
             = self.peek() == &Token::Arrow && self.peek_next() != &Token::Ident;


         let ret_ty = if has_declared_ret_type{
             self.eat();
             let type_key = self.current;
             self.type_mapping(type_key)
         }else if no_declared_ret_type{
             Type::void
         }else if only_declare_arrow{
             todo!()
         }else{ // only_declare_type
             todo!()
         };

         self.table.func_insert(func_name_key, params_start, params_end, ret_ty);

         let stmt_idx = self.block_stmt();
         self.scope.destroy_func_scope();


         let idxs_start = self.edges.len() as u32;
         self.edges.push(func_name_node_idx);
         self.edges.extend(self.scope.func_assigner.drain(..));
         self.edges.push(stmt_idx);
         let idxs_end = (self.edges.len() - 1) as u32;
         self.nodes.push(
             Node::new(Token::FuncStmt, NONE, Interval::new(idxs_start, idxs_end))
         );

         (self.nodes.len() - 1) as u32
     }

    pub fn match_stmt(&mut self) -> u32{

        match self.peek(){
            Token::Let    => self.let_stmt(),
            Token::Ident  => self.call_stmt(),
            Token::If     => self.if_stmt(),
            Token::Ret    => self.ret_stmt(),
            Token::For    => self.for_stmt(),
            Token::While  => self.while_stmt(),
            Token::Brk    => self.brk_stmt(),
            Token::Ctn    => self.ctn_stmt(),
            Token::Func   =>{
                panic!(
                    "[Line {:?}] Function shouldn't on other function", self.line()
                )
            },
            _ => {
                panic!(
                    "[Line {:?}] This Statement is not exist", self.line()
                )
            }
        }

    }
    pub fn let_stmt(&mut self) -> u32{
        self.eat();

        let name_key = self.current as u32;
        let name_idx = if self.peek() == &Token::Ident{
            self.eat();
            self.nodes.push(
                Node::new(Token::Ident, name_key, Interval::new(NONE, NONE))
            );

            (self.nodes.len() - 1) as u32
        }else{
            panic!("[Line {:?}] Invalid Variable Name ",self.line())
        };

        let no_declared_type =
            self.peek() != &Token::Colon && self.peek_next() != &Token::Ident;
        let has_declared_type =
            self.peek() == &Token::Colon && self.peek_next() == &Token::Ident;
        let only_declare_colon
            =  self.peek() == &Token::Colon && self.peek_next() != &Token::Ident;


        //The most frequency condition is on first,
        // So do not change the position
        let declared_ty = if no_declared_type{
            Type::void
        }else if has_declared_type{
            self.eat();//eat ':'
            let declared_type_key = self.current;
            self.eat();
            self.type_mapping(declared_type_key)
        } else if only_declare_colon{
                panic!("[Line {:?}] Declare a Type", self.line())
        }else{
            panic!("[Line {:?}] Misses the ':'", self.line())
        };

        match self.peek(){
            Token::Equel => self.eat(),
            _ => {
                panic!("[Line {:?}] expected '=' ", self.line())
            }
        };

        let expr_idx = if self.is_expr(self.peek()){
            self.expr(0.0)
        }else{
            panic!("[Line {:?}] Expected a Expression", self.line())
        };

        let expr_type = self.get_expr_token_type();

        match self.peek(){
            Token::Scolon => self.eat(),
            _ => {
                panic!("[Line {:?}] expected ';' ", self.line())
            }
        };


        let let_idxs = self.bt_push(name_idx, expr_idx);
        self.nodes.push(Node::new(Token::LetStmt, NONE, let_idxs));


        self.table.var_insert(name_key, self.type_check(&declared_ty, expr_type));

        self.scope.var_scope_stack.push(name_key);

        (self.nodes.len() - 1) as u32
    }


    pub fn call_stmt(&mut self) -> u32{
        let idx = self.func_call();

        match self.peek(){
            Token::Scolon => self.eat(),
            _ => {
                panic!("[Line {:?}] Expected ';'", self.line())
            }
        };

        idx
    }

    pub fn func_call(&mut self) -> u32{
        let unknown_name_key = self.current as u32;
        self.eat();
        let func_name_key = self.find_prev_func_key(unknown_name_key);


        let idxs_start = self.edges.len() as u32;

        let func_name_node_idx = {
            self.nodes.push(
                Node::new(Token::Ident, func_name_key, Interval::new(NONE, NONE))
            );

            (self.nodes.len() - 1) as u32
        };
        self.edges.push(func_name_node_idx);

        match self.peek(){
            Token::Lparen => {
                self.eat()
            },
            _ => {
                panic!("[Line {:?}] Expected '(' ", self.line())
            }
        };

        match self.peek(){
            Token::Scolon => self.eat(),
            _ => {
                panic!("[Line {:?}] Expected ';'", self.line())
            }
        };

        if self.is_expr(self.peek()){
            loop {
                let expr_idx = self.expr(0.0);
                self.edges.push(expr_idx);
                match self.peek() {
                    Token::Comma => self.eat(),
                    Token::Rparen => break,
                    _ => break,
                };
            }
        }else{
            panic!("[Line {:?}] Expected a Expression", self.line())
        }

        match self.peek(){
            Token::Rparen => self.eat(),
            _ =>{
                panic!("[Line {:?}] Expected ')' ", self.line())
            }
        };



        let idxs_end = (self.edges.len() - 1) as u32;

        self.nodes.push(
            Node::new(Token::CallStmt, NONE, Interval::new(idxs_start, idxs_end))
        );

        (self.nodes.len() - 1 ) as u32


    }






    pub fn if_stmt(&mut self) -> u32{
        let start_idx = self.branch_recorder.len();
        self.record_branch_idx();

        let idxs_start = self.edges.len() as u32;
        self.edges.extend(self.branch_recorder.drain(start_idx..));
        let idxs_end = (self.edges.len() - 1) as u32;
        self.nodes.push(
            Node::new(Token::IfStmt, NONE, Interval::new(idxs_start, idxs_end))
        );

        (self.nodes.len() - 1) as u32

    }

    fn record_branch_idx(&mut self){
        match self.peek(){
            Token::Elif => {  //most freq
                if self.branch_recorder.len() >= 2{
                    self.eat();
                    let expr_idx = if self.is_expr(self.peek()){
                        let bool_expr = self.expr(0.0);
                        let expr_type = self.get_expr_token_type();
                        if self.is_bool(expr_type){
                            bool_expr
                        } else{
                            panic!("[Line {:?}] Expected Bool in Condition", self.line())
                        }
                    }else{
                        panic!("[Line {:?}] Expected a Expression", self.line())
                    };


                    self.scope.build_branch_scope();
                    let block_idx = self.block_stmt();
                    self.scope.destroy_branch_scope();


                    self.branch_recorder.push(expr_idx);
                    self.branch_recorder.push(block_idx);
                }else{
                    panic!("[Line {:?}] Expected 'if' token, but found 'elif'", self.line() )
                }
            },
            Token::If   => { // less_freq
                self.eat();
                let expr_idx = if self.is_expr(self.peek()){
                    let bool_expr = self.expr(0.0);
                    let expr_type = self.get_expr_token_type();
                    if self.is_bool(expr_type){bool_expr}
                    else{panic!("[Line {:?}] Expected Bool in Condition", self.line())}
                }else{
                    panic!("[Line {:?}] Expected a Expression", self.line())
                };


                self.scope.build_branch_scope();
                let block_idx = self.block_stmt();
                self.scope.destroy_branch_scope();


                self.branch_recorder.push(expr_idx);
                self.branch_recorder.push(block_idx);
            },
            Token::Else => { //less_freq
                self.eat();


                self.scope.build_branch_scope();
                let block_idx = self.block_stmt();
                self.scope.destroy_branch_scope();


                self.branch_recorder.push(NONE);
                self.branch_recorder.push(block_idx);
            },
            _ => unreachable!()
        };

        match self.peek(){
            Token::Elif | Token::Else => self.record_branch_idx(),
            _ => return
        }
    }

    pub fn ret_stmt(&mut self) -> u32{
        if self.table.ret_types.last().unwrap() != &Type::void{
            self.eat();
            let expr_idx = if self.is_expr(self.peek()){
                let expr = self.expr(0.0);
                let expr_type = self.get_expr_token_type();
                let ret_type = self.table.ret_types.last().unwrap();
                _ = self.type_check(ret_type, expr_type);
                expr
            }else{
                panic!("[Line {:?}] Expected a Expression", self.line())
            };

            match self.peek(){
                Token::Scolon => self.eat(),
                _ => {
                    panic!("[Line {:?}] Expected ';'", self.line())
                }
            };

            let idxs = self.bt_push(NONE, expr_idx);

            self.nodes.push(
                Node::new(Token::RetStmt, NONE, idxs)
            );

            return (self.nodes.len() - 1) as u32;
        }

        self.eat();

        match self.peek(){
            Token::Scolon => self.eat(),
            _ => {
                panic!("[Line {:?}] Expected ';'", self.line())
            }
        };

        self.nodes.push(
            Node::new(Token::RetStmt, NONE, Interval::new(NONE, NONE))
        );

        (self.nodes.len() - 1) as u32

    }
    pub fn for_stmt(&mut self) -> u32 {
        self.eat();

        self.scope.build_loop_scope();

        let ident_key = self.current as u32;
        let loop_ident_idx = if self.peek() == &Token::Ident{

             self.eat();
             self.nodes.push(
                Node::new(Token::Ident, ident_key, Interval::new(NONE, NONE))
            );

            (self.nodes.len() - 1) as u32
        }else{
            panic!("[Line {:?}] Expected an Identifier", self.line())
        };

        match self.peek(){
            Token::In => self.eat(),
            _ => {
                panic!("[Line {:?}] Expected 'In' ", self.line())
            }
        };

        match self.peek(){
            Token::Lbracket => self.eat(),
            _ => {
                panic!("[Line {:?}] Expected '[' ", self.line())
            }
        };

        let start_num_idx = if self.is_expr(self.peek()){
            let int_expr = self.expr(0.0);
            let expr_type = self.get_expr_token_type();
            if self.is_int(expr_type){int_expr}
            else { panic!("[Line {:?}] Expected Integer in start", self.line()) }
        }else{
            panic!("[Line {:?}] Expected Expression in start", self.line())
        };

        match self.peek(){
            Token::Arrow => self.eat(),
            _ => {
                panic!("[Line {:?}] Expected '->'", self.line())
            }
        };

        let end_num_idx = if self.is_expr(self.peek()){
            let int_expr = self.expr(0.0);
            let expr_type = self.get_expr_token_type();
            if self.is_int(expr_type){int_expr}
            else { panic!("[Line {:?}] Expected Integer in end", self.line()) }
        }else{
            panic!("[Line {:?}] Expected Expression in end", self.line())
        };

        let has_declared_step_num
            = self.peek() == &Token::Vartical && self.is_expr(self.peek_next());
        let step_number_idx = if has_declared_step_num{
            self.eat();
            let int_expr = self.expr(0.0);
            let expr_type = self.get_expr_token_type();
            if self.is_int(expr_type){int_expr}
            else{panic!("[Line {:?}] Expected Integer in step", self.line())}
        }else{
            panic!("[Line {:?}] Expected Expression in step", self.line())
        };

        match self.peek(){
            Token::Rbracket => self.eat(),
            _ => {
                panic!("[Line {:?}] Expected ']' ", self.line())
            }
        };

        self.scope.var_scope_stack.push(loop_ident_idx);


        let block_idx = self.block_stmt();

        let idxs_start = self.edges.len() as u32;
        self.edges.push(loop_ident_idx);
        self.edges.push(start_num_idx);
        self.edges.push(end_num_idx);
        self.edges.push(step_number_idx);
        self.edges.push(block_idx);
        let idxs_end = (self.edges.len() - 1) as u32;

        self.nodes.push(
            Node::new(Token::ForStmt, NONE, Interval::new(idxs_start, idxs_end))
        );


       self.scope.destroy_loop_scope();

        (self.nodes.len() - 1) as u32
    }

    
    pub fn while_stmt(&mut self) -> u32 {
        self.eat();

        let cond_expr_idx = if self.is_expr(self.peek()){
            let bool_expr = self.expr(0.0);
            let expr_type = self.get_expr_token_type();
            if self.is_bool(expr_type){bool_expr}
            else{panic!("[Line {:?}] Expected Bool in Condition", self.line())}
        }else{
            panic!("[Line {:?}] Expected Expression in Condition", self.line())
        };

        self.scope.build_loop_scope();
        let block_idx = self.block_stmt();
        self.scope.destroy_loop_scope();

        let idxs_interval = self.bt_push(cond_expr_idx, block_idx);
        self.nodes.push(
            Node::new(Token::WhileStmt, NONE, idxs_interval)
        );

        (self.nodes.len() - 1) as u32

    }
    pub fn brk_stmt(&mut self) -> u32 {
        if self.scope.is_in_loop_scope(){


            let brk_key = self.current as u32;
            let brk_idx = {
                self.nodes.push(
                    Node::new(Token::Brk, brk_key, Interval::new(NONE, NONE))
                );
                (self.nodes.len() - 1) as u32
            };

            self.eat();


            match self.peek(){
                Token::Scolon => self.eat(),
                _ => {
                    panic!("[Line {:?}], Expected ;", self.line())
                }
            };

            let idxs_interval = self.bt_push(NONE, brk_idx);
            self.nodes.push(
                Node::new(Token::BrkStmt, NONE, idxs_interval)
            );


        }else{
            panic!("[Line {:?}] 'break' can't not in loop", self.line());
        }

        (self.nodes.len() - 1) as u32
    }

    pub fn ctn_stmt(&mut self) -> u32 {
        if self.scope.is_in_loop_scope(){

            let ctn_key = self.current as u32;
            let ctn_idx = {
                self.nodes.push(
                    Node::new(Token::Ctn, ctn_key, Interval::new(NONE, NONE))
                );
                (self.nodes.len() - 1) as u32
            };

            self.eat();


            match self.peek(){
                Token::Scolon => self.eat(),
                _ => {
                    panic!("[Line {:?}], Expected ;", self.line())
                }
            };

            let idxs_interval = self.bt_push(NONE, ctn_idx);

            self.nodes.push(
                Node::new(Token::CtnStmt, NONE, idxs_interval)
            );
        }else{
            panic!("[Line {:?}] 'continue' can't not in loop", self.line());
        }

        (self.nodes.len() - 1) as u32


    }

    pub fn block_stmt(&mut self) -> u32{
        match self.peek(){
            Token::Lbrace => self.eat(),
            _ => {
		     panic!("[Line {:?}] Expected {:?}", self.line(), '{')
	        }
        };


        let start_idx = self.block_recorder.len();
        while self.peek() != &Token::Rbrace{
            let stmt_idx = self.match_stmt();
            self.block_recorder.push(stmt_idx);
        }

        match self.peek(){
            Token::Rbrace => self.eat(),
            _ => {
                panic!("[Line {:?}] Expected {:?}", self.line(), '}')
            }
        };



        let idxs_start = self.edges.len() as u32;
        self.edges.extend(self.block_recorder.drain(start_idx..));
        let idxs_end = (self.edges.len() - 1) as u32;
        self.nodes.push(
            Node::new(Token::BlockStmt, NONE, Interval::new(idxs_start, idxs_end))
        );

        if !self.scope.is_in_func{
            (self.nodes.len() - 1) as u32
        }else{
            let idx = *self.edges.last().unwrap();
            let stmt_node = &self.nodes[idx as usize];
            let stmt_tok = &stmt_node.tok;
            match stmt_tok{
                Token::RetStmt => (self.nodes.len() - 1) as u32,
                _ => panic!("[Line {:?}] Expected last statement is return"
                            , self.lexer.lines[self.current - 2]
                )
            }
        }

    }

    pub fn find_prev_func_key(&self, input_key : u32) -> u32{
        let mut found_key = None;

        for key in &self.scope.func_assigner{
            if !self.matches_string(input_key, *key){
                continue;
            }else{
                found_key = Some(key);
                break;
            }
        }

        match found_key{
            Some(the_key) => *the_key,
            None => {
                panic!("[Line {:?}] Can't find this function", self.line())
            }
        }
    }

    fn matches_string(&self ,input_key : u32, key : u32) -> bool{
        let input_key_interval = &self.lexer.intervals[input_key as usize];
        let target_key_interval = &self.lexer.intervals[key as usize];

        let i_start = input_key_interval.start as usize;
        let i_end   = input_key_interval.end as usize;
        let t_start = target_key_interval.start as usize;
        let t_end   = target_key_interval.end as usize;

        &self.lexer.input[i_start..i_end] == &self.lexer.input[t_start..t_end]
    }



}
