use crate::ast::Stmt;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::types::Type;

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while self.current != Token::EOF {
            if let Token::Fn = self.current {
                stmts.push(self.parse_function());
            } else {
                self.advance();
            }
        }

        stmts
    }

    fn parse_function(&mut self) -> Stmt {
        self.advance(); // consume fn

        let name = if let Token::Identifier(name) = &self.current {
            name.clone()
        } else {
            panic!("Expected function name");
        };

        self.advance(); // consume name

        // Default return type to Void for now
        let return_type = Type::Void;

        Stmt::Function {
            name,
            return_type,
            body: vec![],
        }
    }
}
