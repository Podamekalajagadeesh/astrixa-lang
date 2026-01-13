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
            ':' => self.simple(Token::Colon),
            ',' => self.simple(Token::Comma),
            '.' => self.simple(Token::Dot),  // STEP 49: Module access
            '=' => self.peek_two_char_op(),
            '!' => self.peek_not_equal(),
            '<' => self.peek_less(),
            '>' => self.peek_greater(),
            '"' => self.read_string(),
            _ if ch.is_numeric() => self.read_number(),
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
            && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_')
        {
            self.advance();
        }

        let text: String = self.input[start..self.position].iter().collect();

        match text.as_str() {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "panic" => Token::Panic,  // STEP 48: Panic keyword
            "import" => Token::Import, // STEP 49: Import keyword
            "export" => Token::Export, // STEP 49: Export keyword
            _ => Token::Identifier(text),
        }
    }

    fn peek_two_char_op(&mut self) -> Token {
        if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
            self.advance();
            self.advance();
            Token::EqualEqual
        } else {
            self.simple(Token::Assign)
        }
    }

    fn peek_not_equal(&mut self) -> Token {
        if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
            self.advance();
            self.advance();
            Token::NotEqual
        } else {
            self.advance();
            Token::Identifier("!".to_string())
        }
    }

    fn peek_less(&mut self) -> Token {
        if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
            self.advance();
            self.advance();
            Token::LessEqual
        } else {
            self.simple(Token::Less)
        }
    }

    fn peek_greater(&mut self) -> Token {
        if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
            self.advance();
            self.advance();
            Token::GreaterEqual
        } else {
            self.simple(Token::Greater)
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;

        while self.position < self.input.len() && self.input[self.position].is_numeric() {
            self.advance();
        }

        let text: String = self.input[start..self.position].iter().collect();
        
        if let Ok(num) = text.parse::<i64>() {
            Token::Number(num)
        } else {
            Token::Identifier(text)
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // consume opening quote
        let mut result = String::new();

        while self.position < self.input.len() && self.input[self.position] != '"' {
            if self.input[self.position] == '\\' && self.position + 1 < self.input.len() {
                self.advance();
                match self.input[self.position] {
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    _ => {
                        result.push('\\');
                        result.push(self.input[self.position]);
                    }
                }
            } else {
                result.push(self.input[self.position]);
            }
            self.advance();
        }

        if self.position < self.input.len() && self.input[self.position] == '"' {
            self.advance(); // consume closing quote
        }

        Token::String(result)
    }
}
