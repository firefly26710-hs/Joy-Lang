
#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum Token{

    //type
    Int,
    Float,
    Bool,
    Ident,

    //operator
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Equel,
    NotEq,
    EqEq,
    Grt,
    GrtEq,
    Les,
    LesEq,
    CmpAnd,
    CmpOr,
    CmpNot,
    Bang,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Arrow,
    Vartical,
    Colon,
    Comma,
    Dot,
    Scolon,
    

    //keyword
    Let,
    Func,
    For,
    In,
    While,
    If,
    Elif,
    Else,
    True,
    False,
    alse,
    Ret,
    Brk,
    Ctn,
   
    //Statment Kind
    LetStmt,
    IfStmt,
    ForStmt,
    WhileStmt,
    BrkStmt,
    CtnStmt,
    FuncStmt,
    RetStmt,
    BlockStmt,
    CallStmt,
    
    
    Program,
    EOF


}
