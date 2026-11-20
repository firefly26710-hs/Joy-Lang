use crate::parser::tree::node::Key;
use crate::semantic::symbol::data::ty::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeState{
    Stable(Type),
    Integer(Key, bool),
    Float(Key),
    OmegaBool(Key),
}