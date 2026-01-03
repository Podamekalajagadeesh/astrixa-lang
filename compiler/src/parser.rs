use crate::lexer::Token;
use crate::ast::{Stmt, Expr};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> Stmt {
        self.parse_function()
    }

    fn parse_function(&mut self) -> Stmt {
        // expect 'fn'
        self.advance();

        // function name
        let name = if let Some(Token::Identifier(n)) = self.current() {
            n.clone()
        } else {
            panic!("Expected function name");
        };
        self.advance();

        // skip ()
        self.advance(); // (
        self.advance(); // )

        // skip {
        self.advance();

        let mut body = Vec::new();

        while let Some(token) = self.current() {
            if *token == Token::RightBrace {
                break;
            }
            body.push(self.parse_statement());
        }

        self.advance(); // }

        Stmt::Function { name, body }
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.current() {
            Some(Token::Let) => self.parse_let(),
            Some(Token::If) => self.parse_if(),
            _ => Stmt::Expression(self.parse_expression()),
        }
    }

    fn parse_let(&mut self) -> Stmt {
        self.advance(); // let

        let name = if let Some(Token::Identifier(n)) = self.current() {
            n.clone()
        } else {
            panic!("Expected variable name");
        };
        self.advance();

        self.advance(); // =

        let value = self.parse_expression();

        Stmt::Let { name, value }
    }

    fn parse_if(&mut self) -> Stmt {
        self.advance(); // if

        let condition = self.parse_expression();

        self.advance(); // {

        let mut then_branch = Vec::new();
        while let Some(token) = self.current() {
            if *token == Token::RightBrace {
                break;
            }
            then_branch.push(self.parse_statement());
        }
        self.advance(); // }

        let mut else_branch = Vec::new();

        if let Some(Token::Else) = self.current() {
            self.advance(); // else
            self.advance(); // {

            while let Some(token) = self.current() {
                if *token == Token::RightBrace {
                    break;
                }
                else_branch.push(self.parse_statement());
            }
            self.advance(); // }
        }

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_primary();

        while let Some(token) = self.current() {
            match token {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_primary();
                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: "+".to_string(),
                        right: Box::new(right),
                    };
                }
                Token::Greater => {
                    self.advance();
                    let right = self.parse_primary();
                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: ">".to_string(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current() {
            Some(Token::NumberLiteral(n)) => {
                let value = *n;
                self.advance();
                Expr::NumberLiteral(value)
            }
            Some(Token::StringLiteral(s)) => {
                let value = s.clone();
                self.advance();
                Expr::StringLiteral(value)
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();

                if let Some(Token::LeftParen) = self.current() {
                    self.advance(); // (
                    let arg = self.parse_expression();
                    self.advance(); // )
                    Expr::Call {
                        name,
                        args: vec![arg],
                    }
                } else {
                    Expr::Identifier(name)
                }
            }
            _ => panic!("Unexpected token"),
        }
    }
}
