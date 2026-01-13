#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn,
    Let,
    Return,
    If,
    Else,
    While,
    Panic,  // STEP 48: Panic keyword
    Import, // STEP 49: Module import
    Export, // STEP 49: Module export

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
    Dot,    // STEP 49: Module access (math.add)

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Assign,  // =
    
    // Comparison operators
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    EqualEqual,
    NotEqual,

    EOF,
}
