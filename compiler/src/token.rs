#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Fn,
    Let,
    Return,

    Identifier(String),
    Number(i64),
    String(String),

    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Comma,
    Arrow,

    Plus,
    Minus,
    Star,
    Slash,
    Equal,

    EOF,
}
