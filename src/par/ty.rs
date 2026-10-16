#![allow(warnings)]

pub mod table;
pub mod tys;

#[derive(Debug, Clone, PartialEq)]
pub enum Type{
    i32,
    i64,
    f32,
    f64,
    bool,
    String,
    void,

}