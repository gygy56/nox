#[derive(Debug, Clone, PartialEq)]

pub enum Token {
    // Keywords 

    Fn,
    Stck,
    Const,
    Return,
    
    If,
    Else,
    ElseIf,

    While,
    For,
    Loop,

    Break,
    Continue,

    // Literals
    
    True,
    False,
    Nil,

    Int(i64),
    Float(f64),
    Char(char),
    String(String),

    // Types

    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,

    F32,
    F64,

    Bool,
    CharType,
    Str,
    Void,

    // Identifiers
    
    Ident(String),

    // Arith Operators

    Plus,
    Minus,
    Star,
    Slash,
    IntegerDivision,
    Percent,

    // Compar Operators

    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Bitwise Operators
    Ampersand,
    Caret,
    Pipe,
    ShiftLeft,
    ShiftRight,

    // Logical operators

    And,
    Or,
    Not,

    // Symbols

    Question,

    // Delimiters

    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,

    Comma,
    Colon,
    Dot,

    // End Of File
    
    Eof,
}