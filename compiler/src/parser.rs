use crate::ast::Stmt;
use crate::error::CompileError;
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, CompileError> {
        let mut stmts = Vec::new();

        while self.current != Token::EOF {
            if let Token::Fn = self.current {
                stmts.push(self.parse_function()?);
            } else {
                self.advance();
            }
        }

        Ok(stmts)
    }

    fn parse_function(&mut self) -> Result<Stmt, CompileError> {
        self.advance(); // consume fn

        let name = match &self.current {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(
                    CompileError::new(
                        "Expected function name",
                        self.lexer.line,
                        self.lexer.column,
                    )
                    .help("Function names must be valid identifiers"),
                );
            }
        };

        self.advance(); // consume name

        // Default return type to Void for now
        let return_type = Type::Void;

        Ok(Stmt::Function {
            name,
            return_type,
            body: vec![],
        })
    }
}
