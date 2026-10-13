use crate::lex::iterval::Interval;
use crate::lex::token::Token;
use crate::llvm::codegen::Codegen;

impl<'ctx> Codegen<'ctx>{
    pub fn build_block(&mut self, stmts_idx : u32){
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
            let start = idxs.start as usize;
            let end = idxs.end as usize;

			      match tok{
				        Token::LetStmt => self.build_let_stmt(start, end),
				        Token::IfStmt => self.build_if_stmt(start, end),
				        Token::RetStmt => self.build_ret_stmt(start, end),
				        Token::CallStmt => self.build_call_stmt(start, end),
				        Token::ForStmt => self.build_for_stmt(start, end),
				        Token::WhileStmt => self.build_while_stmt(start, end),
				        Token::BrkStmt => self.build_brk_stmt(),
				        Token::CtnStmt => self.build_ctn_stmt(),

				        _ => unreachable!()
			      }
		    }
	
    }

	fn build_let_stmt(&mut self, start : usize, end : usize){
        let childs = &self.parser.edges[start..=end];
        let name_node = &self.parser.nodes[childs[0] as usize];
        let name_key = name_node.key;

        //build expression 
        let declared_type = self.parser.table.get_var_type(name_key);
        let expr = self.build_expr(childs[1] as usize, declared_type);


	}

	fn build_if_stmt(&self, start : usize, end : usize){

    } 
	fn build_ret_stmt(&self, start : usize, end : usize){
		todo!()
	}

	pub fn build_call_stmt(&self, start : usize, end : usize){
		todo!()
	}
	fn build_for_stmt(&self, start : usize, end : usize){
		todo!()
	}
	fn build_while_stmt(&self, start : usize, end : usize){
		todo!()
	}
	fn build_brk_stmt(&self){
		todo!()
	}
	fn build_ctn_stmt(&self){
		todo!()
	}
}
