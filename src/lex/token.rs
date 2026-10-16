
#[derive(Debug, Clone, PartialEq)]
pub enum Token{

    Int,
    Float,
    String,
    Ident,
    Type,

    //operator
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Equal,
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
    Vertical,
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
    Ret,
    Brk,
    Ctn,

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
    Print,
    EOF


}
