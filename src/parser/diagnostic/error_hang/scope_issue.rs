use crate::lexer::diagnostic::source::issue::LexerIssueType;
use crate::parser::tree::node::Key;
use crate::parser::center::par::Parser;
use crate::parser::diagnostic::source::issue::ParserIssue;

impl Parser{

    pub fn var_not_in_scope(&mut self, current_key: Key) -> Result<(), ParserIssue>{
        let prev_key = self.current as Key;

        if prev_key != current_key {
            Ok(())
        }else{
            Err(self.variable_not_in_scope_issue(current_key))
        }
    }


    pub fn has_out_of_loop(&mut self) -> Result<(), ParserIssue>{
        if !self.in_loop(){
            return Err(self.outside_of_a_loop_issue());
        }

        Ok(())
    }

    pub fn fn_not_exist(&mut self, current_key : Key) -> Result<(), ParserIssue>{
        let prev_key = self.current as Key;

        if prev_key != current_key{
            Ok(())
        }else{
            Err(self.cannot_found_function_issue(current_key))
        }
    }
    
    
}