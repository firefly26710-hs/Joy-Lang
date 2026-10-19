
#[derive(Debug, Clone, PartialEq)]
pub enum Token{

    Int,
    Float,
    String,
    Ident,
    Type,

    //operator
    Plus,
    PlusEq,
    Minus,
    MinusEq,
    Mul,
    MulEq,
    Div,
    DivEq,
    Mod,
    ModEq,
    Equal,
    EqEq,
    NotEq,
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
    Val,
    Var,
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
    PrintStmt,
    
    
    Program,
    Print,
    EOF


}
