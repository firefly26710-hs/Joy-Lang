use crate::lex::iterval::{End, Interval, Start};
use crate::par::ast::node::Node;
use crate::lex::token::Token;
use crate::par::ast::node::Key;
use crate::lex::lexer::Lexer;
use crate::par::ast::ast::AST;
use crate::par::Scope::scope::Scope;
use crate::par::ty::table::Table;
use crate::par::ty::tys::Type;

pub const NONE : u32 = u32::MAX;
const RECORDER_SIZE : usize = 256;

pub type NodeIdx = u32;

pub struct Parser{
    lexer : Lexer,
    pub ast : AST,
    pub table : Table,
    pub scope : Scope,

    pub int_node_sum   : u8,
    pub flt_node_sum   : u8,
    pub bool_node_sum  : u8,
    pub regular_op_sum : u8,
    pub compare_op_sum : u8,
    pub is_string      : bool,

    pub stmt_recorder: Vec<NodeIdx>,
    pub prev_end      : usize,

    pub current : usize
}

impl Parser{
    pub fn init(lexer : Lexer) -> Self{
        Self{
            lexer,
            ast   : AST::init(),
            table : Table::init(),
            scope : Scope::init(),

            int_node_sum : 0,
            flt_node_sum : 0,
            bool_node_sum : 0,
            regular_op_sum : 0,
            compare_op_sum : 0,
            is_string : false,

            stmt_recorder: Vec::with_capacity(64),
            prev_end     : 0,

            current : 0
        }
    }

    pub fn build_tree(&mut self){
        while self.lexer.tokens[self.current] != Token::EOF{
            self.parse_function();
        }
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

        self.record_node(tok, key, start, end)

    }
    pub fn push_leaf(
        &mut self,
        tok : Token,
        key : Key
    ) ->NodeIdx{
        self.record_node(tok, key, NONE, NONE)
    }
    fn record_node(
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



    #[inline(always)]
    pub fn current_line(&self) -> u32{
        self.lexer.lines[self.current]
    }



    #[inline(always)]
    pub fn peek(&self) -> &Token {
        &self.lexer.tokens[self.current]

    }

    #[inline(always)]
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
            "i32"    => Type::i32,
            "bool"   => Type::bool,
            "f32"    => Type::f32,
            "i64"    => Type::i64,
            "f64"    => Type::f64,
            "String" => Type::String,
            _ => panic!("UnExpected str {:?}", type_str)
        }
    }

    pub fn print_node(&mut self){
        for (i, node) in self.ast.nodes.iter().enumerate() {
            println!("Number : {i}");
            println!("Node : {:?}", node);

            let idxs = &node.childs;
            let l_idx = idxs.start;
            let r_idx = idxs.end;

            if l_idx != NONE && r_idx != NONE {
                println!("Edge : {:?}", &self.ast.childs[l_idx as usize..=r_idx as usize]);
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




