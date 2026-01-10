use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    pub line: usize,
    pub column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.position];

        match ch {
            '(' => self.simple(Token::LParen),
            ')' => self.simple(Token::RParen),
            '{' => self.simple(Token::LBrace),
            '}' => self.simple(Token::RBrace),
            '+' => self.simple(Token::Plus),
            '-' => self.simple(Token::Minus),
            '*' => self.simple(Token::Star),
            '/' => self.simple(Token::Slash),
            '=' => self.simple(Token::Equal),
            ':' => self.simple(Token::Colon),
            ',' => self.simple(Token::Comma),
            _ => self.read_identifier(),
        }
    }

    fn simple(&mut self, tok: Token) -> Token {
        self.advance();
        tok
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            if self.input[self.position] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.input[self.position].is_whitespace()
        {
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;

        while self.position < self.input.len()
            && self.input[self.position].is_alphanumeric()
        {
            self.advance();
        }

        let text: String = self.input[start..self.position].iter().collect();

        match text.as_str() {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "return" => Token::Return,
            _ => Token::Identifier(text),
        }
    }
}
