use crate::llvm::codegen::Codegen;
use crate::lex::token::Token;

impl<'ctx> Codegen<'ctx>{
    pub fn compile_stmts(&mut self, stmt_idx : usize){
        let block_node = &self.nodes[stmt_idx];
        let l_edge_idx = block_node.l_edge_idx as usize;
        let r_edge_idx = block_node.r_edge_idx as usize;

        let stmt_idxs = &self.edges[l_edge_idx..=r_edge_idx].to_vec();

        for idx in stmt_idxs{
            let node = &self.nodes[*idx as usize];
            let tok = &node.token;
            let l_edge_idx = node.l_edge_idx as usize;
            let r_edge_idx = node.r_edge_idx as usize;

            match tok{
                Token::LetStmt => self.let_stmt(l_edge_idx, r_edge_idx),
                Token::ForStmt => self.for_stmt(l_edge_idx, r_edge_idx),
                Token::IfStmt => self.if_stmt(l_edge_idx, r_edge_idx),
                Token::WhileStmt => self.while_stmt(l_edge_idx, r_edge_idx),
                Token::BrkStmt => self.brk_stmt(l_edge_idx, r_edge_idx),
                Token::CtnStmt => self.ctn_stmt(l_edge_idx, r_edge_idx),
                Token::RetStmt => self.ret_stmt(l_edge_idx, r_edge_idx),
                Token::CallStmt => self.call_stmt(l_edge_idx, r_edge_idx),
                _ => unreachable!()
            }
        }
    }

    fn let_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){
        let idxs = &self.edges[l_edge_idx..=r_edge_idx];
    }
    fn for_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    fn if_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    fn while_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    fn brk_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    fn ctn_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    fn ret_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
    pub fn call_stmt(&mut self, l_edge_idx : usize, r_edge_idx : usize){}
}
