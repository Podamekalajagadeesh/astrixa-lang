use crate::ast::{Expr, Stmt};
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
            match &self.current {
                Token::Import => {
                    stmts.push(self.parse_import()?);
                }
                Token::Export => {
                    stmts.push(self.parse_export_function()?);
                }
                Token::Fn => {
                    stmts.push(self.parse_function(false)?);
                }
                _ => {
                    self.advance();
                }
            }
        }

        Ok(stmts)
    }
    
    fn parse_import(&mut self) -> Result<Stmt, CompileError> {
        self.advance();
        
        let module_name = match &self.current {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(CompileError::new(
                    "Expected module name after 'import'",
                    self.lexer.line,
                    self.lexer.column,
                ).help("Example: import math"));
            }
        };
        
        self.advance();
        
        Ok(Stmt::Import(module_name))
    }
    
    fn parse_export_function(&mut self) -> Result<Stmt, CompileError> {
        self.advance();
        
        if self.current != Token::Fn {
            return Err(CompileError::new(
                "Expected 'fn' after 'export'",
                self.lexer.line,
                self.lexer.column,
            ).help("Only functions can be exported. Example: export fn add(a, b) { }"));
        }
        
        self.parse_function(true)
    }

    fn parse_function(&mut self, exported: bool) -> Result<Stmt, CompileError> {
        self.advance();

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

        self.advance();

        let mut params = Vec::new();
        if let Token::LParen = self.current {
            self.advance();
            
            while self.current != Token::RParen && self.current != Token::EOF {
                if let Token::Identifier(param) = &self.current {
                    params.push(param.clone());
                    self.advance();
                    
                    if let Token::Comma = self.current {
                        self.advance();
                    }
                } else {
                    return Err(CompileError::new(
                        "Expected parameter name",
                        self.lexer.line,
                        self.lexer.column,
                    ));
                }
            }
            
            if let Token::RParen = self.current {
                self.advance();
            }
        }

        let return_type = Type::Void;
        
        let body = if let Token::LBrace = self.current {
            self.advance();
            let body = self.parse_block()?;
            if let Token::RBrace = self.current {
                self.advance();
            }
            body
        } else {
            vec![]
        };

        Ok(Stmt::Function {
            name,
            params,
            return_type,
            body,
            exported,
        })
    }
    
    fn parse_block(&mut self) -> Result<Vec<Stmt>, CompileError> {
        let mut stmts = Vec::new();
        
        while self.current != Token::RBrace && self.current != Token::EOF {
            stmts.push(self.parse_statement()?);
        }
        
        Ok(stmts)
    }
    
    fn parse_statement(&mut self) -> Result<Stmt, CompileError> {
        match &self.current {
            Token::Let => {
                self.advance();
                
                let name = match &self.current {
                    Token::Identifier(n) => n.clone(),
                    _ => {
                        return Err(CompileError::new(
                            "Expected variable name after 'let'",
                            self.lexer.line,
                            self.lexer.column,
                        ));
                    }
                };
                
                self.advance();
                
                if let Token::Assign = self.current {
                    self.advance();
                } else {
                    return Err(CompileError::new(
                        "Expected '=' after variable name",
                        self.lexer.line,
                        self.lexer.column,
                    ));
                }
                
                let value = self.parse_expression()?;
                
                Ok(Stmt::Let { name, value })
            }
            Token::While => {
                self.parse_while()
            }
            Token::If => {
                self.advance();
                
                let condition = self.parse_expression()?;
                
                if self.current != Token::LBrace {
                    return Err(CompileError::new(
                        "Expected '{' after if condition",
                        self.lexer.line,
                        self.lexer.column,
                    ));
                }
                self.advance();
                
                let then_body = self.parse_block()?;
                
                if self.current != Token::RBrace {
                    return Err(CompileError::new(
                        "Expected '}' after if body",
                        self.lexer.line,
                        self.lexer.column,
                    ));
                }
                self.advance();
                
                let else_body = if self.current == Token::Else {
                    self.advance();
                    
                    if self.current != Token::LBrace {
                        return Err(CompileError::new(
                            "Expected '{' after else",
                            self.lexer.line,
                            self.lexer.column,
                        ));
                    }
                    self.advance();
                    
                    let body = self.parse_block()?;
                    
                    if self.current != Token::RBrace {
                        return Err(CompileError::new(
                            "Expected '}' after else body",
                            self.lexer.line,
                            self.lexer.column,
                        ));
                    }
                    self.advance();
                    
                    Some(body)
                } else {
                    None
                };
                
                Ok(Stmt::If { condition, then_body, else_body })
            }
            Token::Return => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Stmt::Return(expr))
            }
            Token::Panic => {
                // STEP 48: Parse panic statement: panic("message")
                self.advance(); // consume 'panic'
                
                // Expect (
                if self.current != Token::LParen {
                    return Err(CompileError::new(
                        "Expected '(' after 'panic'",
                        self.lexer.line,
                        self.lexer.column,
                    ));
                }
                self.advance(); // consume '('
                
                // Parse the error message expression
                let expr = self.parse_expression()?;
                
                // Expect )
                if self.current != Token::RParen {
                    return Err(CompileError::new(
                        "Expected ')' after panic message",
                        self.lexer.line,
                        self.lexer.column,
                    ));
                }
                self.advance(); // consume ')'
                
                Ok(Stmt::Panic(expr))
            }
            Token::Identifier(_) => {
                // Could be assignment: name = value, or just an expression (function call, etc.)
                // We need to peek ahead to distinguish
                // For now, parse as expression and it will handle function calls
                let expr = self.parse_expression()?;
                
                // Check if it's an assignment we just parsed
                if let Expr::Identifier(name) = &expr {
                    if let Token::Assign = self.current {
                        self.advance(); // consume '='
                        let value = self.parse_expression()?;
                        return Ok(Stmt::Assign { name: name.clone(), value });
                    }
                }
                
                Ok(Stmt::Expression(expr))
            }
            _ => {
                let expr = self.parse_expression()?;
                Ok(Stmt::Expression(expr))
            }
        }
    }
    
    fn parse_while(&mut self) -> Result<Stmt, CompileError> {
        self.advance(); // consume 'while'
        
        let condition = self.parse_expression()?;
        
        if self.current != Token::LBrace {
            return Err(CompileError::new(
                "Expected '{' after while condition",
                self.lexer.line,
                self.lexer.column,
            ));
        }
        self.advance(); // consume '{'
        
        let body = self.parse_block()?;
        
        if self.current != Token::RBrace {
            return Err(CompileError::new(
                "Expected '}' after while body",
                self.lexer.line,
                self.lexer.column,
            ));
        }
        self.advance(); // consume '}'
        
        Ok(Stmt::While { condition, body })
    }
    
    fn parse_expression(&mut self) -> Result<Expr, CompileError> {
        self.parse_comparison()
    }
    
    /// Parse comparison expressions: a > b, a < b, etc.
    fn parse_comparison(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.parse_additive()?;
        
        while matches!(self.current, 
            Token::Greater | Token::Less | Token::GreaterEqual | Token::LessEqual | 
            Token::EqualEqual | Token::NotEqual) {
            let op = self.current.clone();
            self.advance();
            let right = self.parse_additive()?;
            
            left = match op {
                Token::Greater => Expr::Gt(Box::new(left), Box::new(right)),
                Token::Less => Expr::Lt(Box::new(left), Box::new(right)),
                Token::GreaterEqual => Expr::Ge(Box::new(left), Box::new(right)),
                Token::LessEqual => Expr::Le(Box::new(left), Box::new(right)),
                Token::EqualEqual => Expr::Eq(Box::new(left), Box::new(right)),
                Token::NotEqual => Expr::Ne(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        
        Ok(left)
    }
    
    /// Parse additive expressions (lowest precedence): a + b - c
    fn parse_additive(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.parse_multiplicative()?;
        
        while matches!(self.current, Token::Plus | Token::Minus) {
            let op = self.current.clone();
            self.advance();
            let right = self.parse_multiplicative()?;
            
            left = match op {
                Token::Plus => Expr::Add(Box::new(left), Box::new(right)),
                Token::Minus => Expr::Sub(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        
        Ok(left)
    }
    
    /// Parse multiplicative expressions (higher precedence): a * b / c % d
    fn parse_multiplicative(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.parse_call()?;
        
        while matches!(self.current, Token::Star | Token::Slash | Token::Percent) {
            let op = self.current.clone();
            self.advance();
            let right = self.parse_call()?;
            
            left = match op {
                Token::Star => Expr::Mul(Box::new(left), Box::new(right)),
                Token::Slash => Expr::Div(Box::new(left), Box::new(right)),
                Token::Percent => Expr::Mod(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        
        Ok(left)
    }
    
    fn parse_call(&mut self) -> Result<Expr, CompileError> {
        let expr = self.parse_primary()?;
        
        // Check for module-qualified call: module.function(args)
        if let Expr::Identifier(ref name) = expr {
            // Check for dot notation (module access)
            if let Token::Dot = self.current {
                let module_name = name.clone();
                self.advance(); // consume '.'
                
                let function_name = match &self.current {
                    Token::Identifier(func) => func.clone(),
                    _ => {
                        return Err(CompileError::new(
                            "Expected function name after '.'",
                            self.lexer.line,
                            self.lexer.column,
                        ));
                    }
                };
                self.advance(); // consume function name
                
                // Parse arguments
                if let Token::LParen = self.current {
                    self.advance(); // consume (
                    let mut args = Vec::new();
                    
                    while self.current != Token::RParen && self.current != Token::EOF {
                        args.push(self.parse_expression()?);
                        if let Token::Comma = self.current {
                            self.advance(); // consume ,
                        }
                    }
                    
                    if let Token::RParen = self.current {
                        self.advance(); // consume )
                    }
                    
                    return Ok(Expr::ModuleCall(module_name, function_name, args));
                } else {
                    return Err(CompileError::new(
                        "Expected '(' after module function name",
                        self.lexer.line,
                        self.lexer.column,
                    ));
                }
            }
            
            // Regular function call
            if let Token::LParen = self.current {
                let name_clone = name.clone();
                self.advance(); // consume (
                let mut args = Vec::new();
                
                while self.current != Token::RParen && self.current != Token::EOF {
                    args.push(self.parse_expression()?);
                    if let Token::Comma = self.current {
                        self.advance(); // consume ,
                    }
                }
                
                if let Token::RParen = self.current {
                    self.advance(); // consume )
                }
                
                return Ok(Expr::Call(name_clone, args));
            }
        }
        
        Ok(expr)
    }
    
    fn parse_primary(&mut self) -> Result<Expr, CompileError> {
        let expr = match &self.current.clone() {
            Token::Number(n) => {
                let n = *n;
                self.advance();
                Expr::Number(n)
            }
            Token::Float(f) => {
                let f = *f;
                self.advance();
                Expr::Float(f)
            }
            Token::True => {
                self.advance();
                Expr::Bool(true)
            }
            Token::False => {
                self.advance();
                Expr::Bool(false)
            }
            Token::String(s) => {
                let s = s.clone();
                self.advance();
                Expr::String(s)
            }
            Token::Identifier(id) => {
                let id = id.clone();
                self.advance();
                Expr::Identifier(id)
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                if let Token::RParen = self.current {
                    self.advance();
                }
                expr
            }
            _ => {
                return Err(CompileError::new(
                    "Unexpected token in expression",
                    self.lexer.line,
                    self.lexer.column,
                ));
            }
        };
        
        Ok(expr)
    }
}
