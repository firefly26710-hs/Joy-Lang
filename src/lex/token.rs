#[derive(Debug, Clone, PartialEq)]
pub enum Token{

    //type
    Int,
    Float,
    Ident,

    //operator
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Equel,
    PlusEq,
    MinusEq,
    MulEq,
    DivEq,
    ModEq,
    EqEq,
    Grt,
    GrtEq,
    Les,
    LesEq,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Arrow,
    Vartical,
    Colon,
    Comma,
    Dot,

    //keyword
    Let,
    Func,
    For,
    In,
    While,
    If,
    Elif,
    Else,
    CmpNot,
    CmpAnd,
    CmpOr,
    True,
    False,
    Ret,
    Brk,
    Ctn,

    
    EOF


}
