use std::ptr::addr_of_mut;
use crate::par::node::Node;
use crate::lex::token::Token;
use crate::par::parser::{Parser, NONE};
use crate::par::ty::Type;

impl Parser{

    pub fn match_stmt(&mut self ) -> u32{

        match self.eat(){
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

        let name_key = self.current as u32;
        let name_idx = if self.peek() == Token::Ident{
            self.eat();
            let idxs = self.bt_push(NONE, NONE);
            self.nodes.push(
                Node::new(Token::Ident, name_key.clone(), idxs)
            );

            (self.nodes.len() - 1) as u32

        }else{
            panic!("[Line {:?}] Invalid Variable Name ",self.line())};

        let has_declared_type
            = self.peek() == Token::Colon && self.peek_next() == Token::Ident;

        let only_declare_colon
            = self.peek() == Token::Colon && self.peek_next() != Token::Ident;
        

        let declared_ty = if !has_declared_type{
            Type::void
        }else if has_declared_type{
            self.eat();//eat ':'

            let declared_type_key = self.current;
            self.eat();
            self.type_mapping(declared_type_key)
        }else if only_declare_colon{
            panic!("[Line {:?}] Declare a Type", self.line())
        }else{
            panic!("[Line {:?}] Misses the ':'", self.line())
        };

        match self.peek(){
            Token::Equel => self.eat(),
            _ => {
                panic!("[Line {:?}] expectdd '=' ", self.line())
            }
        };

        let expr_idx = self.expr(0.0);

        let let_idxs = self.bt_push(name_idx, expr_idx);
        self.nodes.push(Node::new(Token::LetStmt, NONE, let_idxs));
        
        self.table.var_insert(name_key, declared_ty);
        self.scope.var_scope_assign(name_key);
        
        (self.nodes.len() - 1) as u32
    }

    pub fn call_stmt(&mut self) -> u32{todo!()}


    pub fn if_stmt(&mut self) -> u32{todo!()}

    pub fn ret_stmt(&mut self) -> u32 { todo!() }
    pub fn for_stmt(&mut self) -> u32 { todo!() }
    pub fn while_stmt(&mut self) -> u32 { todo!()}
    pub fn brk_stmt(&mut self) -> u32 {todo!()}

    pub fn ctn_stmt(&mut self) -> u32 {todo!()}



}