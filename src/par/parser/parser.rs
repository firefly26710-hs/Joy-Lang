use crate::lex::iterval::{End, Interval, Start};
use crate::par::ast::node::Node;
use crate::lex::token::Token;
use crate::par::ast::node::Key;
use crate::lex::lexer::Lexer;
use crate::par::ast::ast::AST;
use crate::par::parser::ty::Type;
use crate::par::scope::scope::Scope;
use crate::par::table::table::Table;

pub const NONE : u32 = u32::MAX;
const RECORDER_SIZE : usize = 256;

pub type NodeIdx = u32;

pub struct Parser{


    //data
    pub lexer : Lexer,
    pub ast : AST,
    pub table : Table,
    pub scope : Scope,

    //logic helper
    pub programs     : Vec<NodeIdx>,
    pub func_recorder: Vec<NodeIdx>,
    pub branch_recorder:Vec<NodeIdx>,
    pub loop_recorder : Vec<NodeIdx>,
    pub func_call_recorder : Vec<NodeIdx>,

    pub current : usize,

    pub ret_times_in_branch: u8,
    pub real_times_in_branch: u8,
    pub is_have_else       : bool,
    pub already_full_ret_in_if: bool


}

impl Parser{
    pub fn init(lexer : Lexer) -> Self{
        Self{
            lexer,
            ast   : AST::init(),
            table : Table::init(),
            scope : Scope::init(),


            programs : Vec::with_capacity(RECORDER_SIZE),
            func_recorder: Vec::with_capacity(RECORDER_SIZE),
            branch_recorder : Vec::with_capacity(RECORDER_SIZE),
            loop_recorder   : Vec::with_capacity(RECORDER_SIZE),
            func_call_recorder : Vec::with_capacity(RECORDER_SIZE),

            current : 0,

            ret_times_in_branch: 0,
            real_times_in_branch: 0,
            is_have_else : false,
            already_full_ret_in_if: false
        }
    }

    pub fn build_tree(&mut self){
        while self.lexer.tokens[self.current] != Token::EOF{
            let func_idx = self.parse_function();
            self.programs.push(func_idx)
        }

        let start = self.ast.edge_start();
        self.ast.edges.extend(self.programs.drain(..));
        let end = self.ast.edge_end();

        self.push_node(Token::Program, NONE, start, end);
    }

    pub fn find_same_ident_key(&self, input_key : Key) -> Key{
        let mut target_key : Option<Key> = None;
        for key in self.scope.var_scope_stack.iter().rev(){
            if self.is_matches_string(input_key, *key){
                target_key = Some(*key);
            }
        }

        match target_key{
            Some(key) => key,
            None => {
                panic!(
                    "[Line {:?}] Can't find the value in this scope",
                    self.current_line(),
                )
            }
        }
    }

    pub fn find_same_func_key(&self, input_key : Key) -> Key{
        let mut target_key : Option<Key> = None;
        for key in self.scope.func_assigner.iter(){
            if self.is_matches_string(input_key, *key){
                target_key = Some(*key);
            }
        }

        match target_key{
            Some(key) => key,
            None => {
                panic!(
                    "[Line {:?}] Can't find the function",
                    self.current_line()
                )
            }
        }
    }


    pub fn push_node(
        &mut self,
        tok : Token,
        key : Key,
        start: Start,
        end: End
    ) -> NodeIdx{
        self.ast.nodes.push(
            Node::new(tok, key, Interval::new(start, end))
        );

        (self.ast.nodes.len() - 1) as NodeIdx
    }

    pub fn push_leaf(
        &mut self,
        tok : Token,
        key : Key
    ) ->NodeIdx{
        self.push_node(tok, key, NONE, NONE)
    }




    pub fn current_line(&self) -> u32{
        self.lexer.lines[self.current]
    }



    pub fn peek(&self) -> &Token {
        &self.lexer.tokens[self.current]

    }

    pub fn peek_next(&self) -> &Token{
        &self.lexer.tokens[self.current + 1]

    }

    #[inline(always)]
    pub fn eat(&mut self) -> Token{
        let tok = self.lexer.tokens[self.current].clone();
        self.current += 1;
        tok
    }
    pub fn is_matches_string(&self, key1 : Key, key2 : Key) ->bool{
        self.look_string(key1) == self.look_string(key2)
    }

    pub fn look_string(&self, key : Key) -> &str{
        let interval = &self.lexer.intervals[key as usize];
        let slice_start = interval.start as usize;
        let slice_end = (interval.end - 1) as usize;

        &self.lexer.input[slice_start..=slice_end]
    }





    pub fn type_mapping(&self, input_key: Key) -> Type{
        let interval = &self.lexer.intervals[input_key as usize];
        let start = interval.start as usize;
        let end = interval.end as usize;
        let type_str = &self.lexer.input[start..end];

        match type_str {
            "i8"     => Type::i8,
            "i16"    => Type::i16,
            "i32"    => Type::i32,
            "i64"    => Type::i64,
            "i128"   => Type::i128,
            "f32"    => Type::f32,
            "f64"    => Type::f64,
            "bool"   => Type::bool,
            "String" => Type::String,
            "void"   => Type::void,
            _ => panic!("[Line {:?}] Unknown Type : {:?}",self.current_line(), type_str)
        }
    }


    pub fn print_node(&mut self){
        for (i, node) in self.ast.nodes.iter().enumerate() {
            println!("Number : {i}");
            println!("Node : {:?}", node);

            let idxs = &node.edge_span;
            let l_idx = idxs.start;
            let r_idx = idxs.end;

            if l_idx != NONE && r_idx != NONE {
                println!("Edge : {:?}", &self.ast.edges[l_idx as usize..=r_idx as usize]);
            }

            let key = node.key;
            if key == NONE{continue}
            let interval = &self.lexer.intervals[key as usize];
            let start = interval.start as usize;
            let end   = interval.end   as usize;
            println!("String : {:?}\n", &self.lexer.input[start..end]);
        }
    }



}




