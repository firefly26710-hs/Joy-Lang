use crate::lex::iterval::Interval;
use crate::lex::token::Token;
use crate::llvm::codegen::Codegen;

impl<'ctx> Codegen<'ctx>{
    pub fn build_block(&self, stmts_idx : u32){
        let block_node = &self.parser.nodes[stmts_idx as usize];
		let idxs = &block_node.idxs;
		let start = idxs.start as usize;
		let end = idxs.end as usize;
		let childs = &self.parser.edges[start..=end];
		let len = childs.len();
		let ptr = childs.as_ptr();
		for i in 0..len{
			let stmt_node = &self.parser.nodes[unsafe{ptr.add(i)} as usize];
			let tok = &stmt_node.tok;
			let idxs = &stmt_node.idxs;

			match tok{
				Token::LetStmt => self.build_let_stmt(idxs),
				Token::IfStmt => self.build_if_stmt(idxs),
				Token::RetStmt => self.build_ret_stmt(idxs),
				Token::CallStmt => self.build_call_stmt(idxs),
				Token::ForStmt => self.build_for_stmt(idxs),
				Token::WhileStmt => self.build_while_stmt(idxs),
				Token::BrkStmt => self.build_brk_stmt(),
				Token::CtnStmt => self.build_ctn_stmt(),

				_ => unreachable!()
			}
		}
	
    }

	fn build_let_stmt(&self, idxs : &Interval){
		todo!()
	}

	fn build_if_stmt(&self, idxs : &Interval){
		todo!()
	}

	fn build_ret_stmt(&self, idxs : &Interval){
		todo!()
	}

	fn build_call_stmt(&self, idxs : &Interval){
		todo!()
	}
	fn build_for_stmt(&self, idxs : &Interval){
		todo!()
	}
	fn build_while_stmt(&self, idxs : &Interval){
		todo!()
	}
	fn build_brk_stmt(&self){
		todo!()
	}
	fn build_ctn_stmt(&self){
		todo!()
	}
}
