use crate::lexer::tokenizer::source::interval::{End, Interval, Start};
use crate::lexer::tokenizer::source::token::Token;
use crate::parser::diagnostic::source::issue::ParserIssue;
use crate::parser::tree::ast::{NodeIdx, NONE};
use crate::parser::tree::node::{Key, Node};
use crate::parser::center::par::Parser;

impl Parser{
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
    ) ->Result<NodeIdx, ParserIssue>{
        Ok(self.push_node(tok, key, NONE, NONE))
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
            let interval = &self.lexer.tokenizer.intervals[key as usize];
            let start = interval.start as usize;
            let end   = interval.end   as usize;
            println!("String : {:?}\n", &self.lexer.tokenizer.input[start..end]);
        }
    }

}