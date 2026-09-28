use crate::par::node::Node;
use crate::lex::token::Token;
use crate::par::parser::{Parser, NONE};
use crate::par::ty::Type;

impl Parser{
    pub fn let_stmt(&mut self) -> u32{
        let let_key = self.current as u32;
        self.eat();

        let name_key = self.current as u32;
        let name_idx = if self.peek() == Token::Ident{
            self.eat();
            let idxs = self.bt_push(NONE, NONE);
            self.nodes.push(
                Node::new(Token::Ident, name_key.clone(), idxs)
            );

            (self.nodes.len() - 1) as u32

        }else{
            panic!(	"[Line {:?}] Invalid Variable Name ",
                       self.lines[self.current])
        };

        let has_declared_type
            = self.peek() == Token::Colon && self.peek_next() == Token::Ident;

        let only_colon 
            = self.peek() == Token::Colon && self.peek_next() != Token::Ident;
        

        let declared_ty = if !has_declared_type{
            Type::void
        }else if has_declared_type{
            self.eat(); //eat ':'
            let declared_type_key = self.current;
            self.eat();
            self.type_mapping(declared_type_key)
        }else if only_colon{
            panic!("[Line {:?}] Declare a Type", self.lines[self.current])
        }else{
            panic!("[Line {:?}] Misses the ':'", self.lines[self.current])
        };

        match self.peek(){
            Token::Equel => self.eat(),
            _ => {
                panic!("[Line {:?}] expectdd '=' ", self.lines[self.current ])
            }
        };

        let expr_idx = self.expr(0.0);

        let let_idxs = self.bt_push(name_idx, expr_idx);
        self.nodes.push(Node::new(Token::IfStmt, let_key, let_idxs));
        
        self.table.var_insert(name_key, declared_ty);
        self.scope.var_scope_assign(name_key);
        
        (self.nodes.len() - 1) as u32


    }
}