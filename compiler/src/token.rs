#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Fn,
    Let,
    Return,
    If,
    Else,
    While,
    Panic,
    Import,
    Export,
    True,
    False,

    Identifier(String),
    Number(i64),
    Float(f64),
    String(String),

    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Comma,
    Arrow,
    Dot,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    EqualEqual,
    NotEqual,

    EOF,
}
