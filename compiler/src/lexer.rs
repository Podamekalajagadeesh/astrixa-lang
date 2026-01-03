#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Fn,
    Let,
    If,
    Else,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(i64),
    Equal,
    Plus,
    Greater,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current_char() {
            match c {
                ' ' | '\n' | '\t' | '\r' => {
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.advance();
                }
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                '=' => {
                    tokens.push(Token::Equal);
                    self.advance();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '>' => {
                    tokens.push(Token::Greater);
                    self.advance();
                }
                '"' => {
                    tokens.push(self.read_string());
                }
                c if c.is_digit(10) => {
                    tokens.push(self.read_number());
                }
                c if c.is_alphabetic() => {
                    tokens.push(self.read_identifier());
                }
                _ => {
                    self.advance(); // skip unknown for now
                }
            }
        }

        tokens
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // skip opening quote
        let mut value = String::new();

        while let Some(c) = self.current_char() {
            if c == '"' {
                break;
            }
            value.push(c);
            self.advance();
        }

        self.advance(); // skip closing quote
        Token::StringLiteral(value)
    }

    fn read_identifier(&mut self) -> Token {
        let mut value = String::new();

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match value.as_str() {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Identifier(value),
        }
    }

    fn read_number(&mut self) -> Token {
        let mut value = String::new();

        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token::NumberLiteral(value.parse::<i64>().unwrap())
    }
}
