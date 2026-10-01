use crate::lex::iterval::Interval;
use crate::par::node::Node;
use crate::lex::token::Token;
use crate::par::parser::{Parser, NONE};
use crate::par::ty::Type;

impl Parser{
    // pub fn func_stmt(&mut self) -> u32{
    //     match self.peek(){
    //         Token::Func => self.eat(),
    //         _ => {
    //             panic!("[Line {:?}], Can't find function", self.line())
    //         }
    //     };
    //
    //     let func_name_key = self.current as u32;
    //     let func_name_node_idx = if self.peek() == &Token::Ident{
    //         self.eat();
    //         self.nodes.push(
    //             Node::new(Token::Ident, func_name_key, Interval::new(NONE, NONE))
    //         );
    //         (self.nodes.len() - 1) as u32
    //     }else{
    //         panic!("[Line {:?}] Expected a function name", self.line())
    //     };
    //
    //     match self.peek(){
    //         Token::Lparen => self.eat(),
    //         _ => {
    //             panic!("[Line {:?}] Expected a (", self.line())
    //         }
    //     };
    //
    //     let has_declared_params = self.is_expr(self.peek());
    //     let no_declared_params = self.peek() == &Token::Rparen;
    //     let error_expr = !self.is_expr(self.peek()) && self.peek() !=&Token::Rparen;
    //     let no_params_and_no_rparen = !self.is_expr(self.peek()) || self.peek() ==&Token::Rparen;
    //
    //     if has_declared_params{
    //
    //     }else if no_declared_params{
    //         self.eat();
    //     }else if error_expr{
    //         todo!()
    //     }else if no_params_and_no_rparen{
    //         todo!()
    //     }
    //
    //     let has_declared_ret_type =
    //         self.peek() == &Token::Arrow && self.peek_next() == &Token::Ident;
    //
    //     let no_declared_ret_type =
    //         self.peek() != &Token::Arrow && self.peek_next() == &Token::Ident;
    //
    //     let only_declare_arrow
    //         = self.peek() == &Token::Arrow && self.peek_next() != &Token::Ident;
    //
    //
    //     let ret_ty = if has_declared_ret_type{
    //         self.eat();
    //         let type_key = self.current;
    //         self.type_mapping(type_key)
    //     }else if no_declared_ret_type{
    //         Type::void
    //     }else if only_declare_arrow{
    //         todo!()
    //     }else{ // only_declare_type
    //         todo!()
    //     };
    //
    //     
    // }

    pub fn match_stmt(&mut self) -> u32{

        match self.peek(){
            Token::Let    => self.let_stmt(),
            Token::Ident  => self.call_stmt(),
            Token::If     => self.if_stmt(),
            Token::Ret    => self.ret_stmt(),
            Token::For    => self.for_stmt(),
            Token::While  => self.while_stmt(),
            Token::Brk   => self.brk_stmt(),
            Token::Ctn   => self.ctn_stmt(),
            Token::Func  =>{
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
                Node::new(Token::Ident, name_key.clone(), Interval::new(NONE, NONE))
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

        let expr_type = self.get_expr_nodes_type();

        match self.peek(){
            Token::Scolon => self.eat(),
            _ => {
                panic!("[Line {:?}] expected ';' ", self.line())
            }
        };


        let let_idxs = self.bt_push(name_idx, expr_idx);
        self.nodes.push(Node::new(Token::LetStmt, NONE, let_idxs));


        self.table.var_insert(name_key, self.type_check(declared_ty, expr_type));

        self.scope.var_scope_assign(name_key);
        
        (self.nodes.len() - 1) as u32
    }

    pub fn call_stmt(&mut self) -> u32{todo!()}


    pub fn if_stmt(&mut self) -> u32{
        let start_idx = self.scope.branch_recorder.len();
        self.record_branch_idx();

        let idxs_start = self.edges.len() as u32;
        self.edges.extend(self.scope.branch_recorder.drain(start_idx..));
        let idxs_end = (self.edges.len() - 1) as u32;
        self.nodes.push(
            Node::new(Token::IfStmt, NONE, Interval::new(idxs_start, idxs_end))
        );

        (self.nodes.len() - 1) as u32

    }

    fn record_branch_idx(&mut self){
        match self.peek(){
            Token::Elif => {  //most freq
                if self.scope.branch_recorder.len() >= 2{
                    self.eat();
                    let expr_idx = if self.is_expr(self.peek()){
                        let bool_expr = self.expr(0.0);
                        let expr_type = self.get_expr_nodes_type();
                        if self.is_bool(expr_type){bool_expr}
                        else{panic!("[Line {:?}] Expected Bool in Condition", self.line())}
                    }else{
                        panic!("[Line {:?}] Expected a Expression", self.line())
                    };

                    let block_idx = self.block_stmt(false);
                    self.scope.branch_recorder.push(expr_idx);
                    self.scope.branch_recorder.push(block_idx);
                }else{
                    panic!("[Line {:?}] Expected 'if' token, but found 'elif'", self.line() )
                }
            },
            Token::If   => { // less_freq
                self.eat();
                let expr_idx = if self.is_expr(self.peek()){
                    let bool_expr = self.expr(0.0);
                    let expr_type = self.get_expr_nodes_type();
                    if self.is_bool(expr_type){bool_expr}
                    else{panic!("[Line {:?}] Expected Bool in Condition", self.line())}
                }else{
                    panic!("[Line {:?}] Expected a Expression", self.line())
                };

                let block_idx = self.block_stmt(false);
                self.scope.branch_recorder.push(expr_idx);
                self.scope.branch_recorder.push(block_idx);
            },
            Token::Else => { //less_freq
                self.eat();
                let block_idx = self.block_stmt(false);
                self.scope.branch_recorder.push(NONE);
                self.scope.branch_recorder.push(block_idx);
            },
            _ => unreachable!()
        };

        match self.peek(){
            Token::Elif | Token::Else => self.record_branch_idx(),
            _ => return
        }
    }

    pub fn ret_stmt(&mut self) -> u32 { todo!() }
    pub fn for_stmt(&mut self) -> u32 {
        self.eat();

        self.scope.build_func_scope(true);

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
            let expr_type = self.get_expr_nodes_type();
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
            let expr_type = self.get_expr_nodes_type();
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
            let expr_type = self.get_expr_nodes_type();
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

        self.scope.var_scope_assign(loop_ident_idx);


        let block_idx = self.block_stmt(false);

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


        self.scope.destroy_scope(true);

        (self.nodes.len() - 1) as u32
    }

    
    pub fn while_stmt(&mut self) -> u32 {
        self.eat();

        let cond_expr_idx = if self.is_expr(self.peek()){
            let bool_expr = self.expr(0.0);
            let expr_type = self.get_expr_nodes_type();
            if self.is_bool(expr_type){bool_expr}
            else{panic!("[Line {:?}] Expected Bool in Condition", self.line())}
        }else{
            panic!("[Line {:?}] Expected Expression in Condition", self.line())
        };

        self.scope.build_func_scope(true);
        let block_idx = self.block_stmt(false);
        self.scope.destroy_scope(true);

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

    pub fn block_stmt(&mut self, is_on_func_stmt : bool) -> u32{
        match self.peek(){
            Token::Lbrace => self.eat(),
            _ => {
		     panic!("[Line {:?}] Expected {:?}", self.line(), '{')
	        }
        };


        let start_idx = self.scope.block_recorder.len();
        while self.peek() != &Token::Rbrace{
            let stmt_idx = self.match_stmt();
            self.scope.block_recorder.push(stmt_idx);
        }

        match self.peek(){
            Token::Rbrace => self.eat(),
            _ => {
                panic!("[Line {:?}] Expected {:?}", self.line(), '}')
            }
        };



        let idxs_start = self.edges.len() as u32;
        self.edges.extend(self.scope.block_recorder.drain(start_idx..));
        let idxs_end = (self.edges.len() - 1) as u32;
        self.nodes.push(
            Node::new(Token::BlockStmt, NONE, Interval::new(idxs_start, idxs_end))
        );


        (self.nodes.len() - 1) as u32

    }



}
