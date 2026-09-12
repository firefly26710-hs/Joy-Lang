#![allow(warnings)]
#[derive(Debug, Clone, PartialEq)]
pub enum Type{
    i32,
    i64,
    f32,
    f64,
    bool,
    Void,

    MAGIC
}
