use crate::lexer::Token;
use crate::ast::{Stmt, Expr, Function, Contract};

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

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while self.current().is_some() {
            match self.current() {
                Some(Token::Import) => {
                    statements.push(self.parse_import());
                }
                Some(Token::Contract) => {
                    statements.push(self.parse_contract());
                }
                Some(Token::Fn) => {
                    statements.push(self.parse_function());
                }
                _ => {
                    self.advance();
                }
            }
        }

        statements
    }

    fn parse_import(&mut self) -> Stmt {
        self.advance(); // import
        if let Some(Token::StringLiteral(path)) = self.current() {
            let module = path.clone();
            self.advance();
            Stmt::Import(module)
        } else {
            panic!("Expected module string");
        }
    }

    fn parse_contract(&mut self) -> Stmt {
        self.advance(); // contract
        let name = self.consume_identifier();

        self.advance(); // {

        let mut state = Vec::new();
        let mut functions = Vec::new();

        while let Some(token) = self.current() {
            if *token == Token::RightBrace {
                break;
            }

            match token {
                Token::Let => {
                    self.advance(); // let
                    let var_name = self.consume_identifier();
                    state.push(var_name);
                }
                Token::Fn => {
                    functions.push(match self.parse_function() {
                        Stmt::Function(f) => f,
                        _ => panic!("Expected function"),
                    });
                }
                _ => {
                    panic!("Invalid contract body element");
                }
            }
        }

        self.advance(); // }

        Stmt::Contract(Contract {
            name,
            state,
            functions,
        })
    }

    fn parse_import(&mut self) -> Stmt {
        self.advance(); // import
        if let Some(Token::StringLiteral(path)) = self.current() {
            let module = path.clone();
            self.advance();
            Stmt::Import(module)
        } else {
            panic!("Expected module string");
        }
    }

    fn parse_function(&mut self) -> Stmt {
        // expect 'fn'
        self.advance();

        let name = self.consume_identifier();

        self.advance(); // (

        let mut params = Vec::new();
        while let Some(Token::Identifier(p)) = self.current() {
            params.push(p.clone());
            self.advance();
            if let Some(Token::Comma) = self.current() {
                self.advance();
            }
        }

        self.advance(); // )
        self.advance(); // {

        let mut body = Vec::new();
        while let Some(token) = self.current() {
            if *token == Token::RightBrace {
                break;
            }
            body.push(self.parse_statement());
        }

        self.advance(); // }

        Stmt::Function(Function { name, params, body })
    }

    fn parse_statement(&mut self) -> Stmt {
        if let Some(Token::Identifier(name)) = self.current() {
            if let Some(Token::Assign) = self.peek() {
                let var_name = name.clone();
                self.advance(); // identifier
                self.advance(); // =
                let value = self.parse_expression();
                return Stmt::Assign {
                    name: var_name,
                    value,
                };
            }
        }

        match self.current() {
            Some(Token::Let) => self.parse_let(),
            Some(Token::If) => self.parse_if(),
            Some(Token::While) => self.parse_while(),
            Some(Token::Import) => self.parse_import(),
            Some(Token::Return) => {
                self.advance();
                Stmt::Return(self.parse_expression())
            }
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

    fn parse_while(&mut self) -> Stmt {
        self.advance(); // while

        let condition = self.parse_expression();

        self.advance(); // {

        let mut body = Vec::new();
        while let Some(token) = self.current() {
            if *token == Token::RightBrace {
                break;
            }
            body.push(self.parse_statement());
        }
        self.advance(); // }

        Stmt::While { condition, body }
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
                Token::Less => {
                    self.advance();
                    let right = self.parse_primary();
                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: "<".to_string(),
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
            Some(Token::True) => {
                self.advance();
                Expr::BoolLiteral(true)
            }
            Some(Token::False) => {
                self.advance();
                Expr::BoolLiteral(false)
            }
            Some(Token::LeftBracket) => {
                self.advance();
                let mut items = Vec::new();
                while let Some(token) = self.current() {
                    if *token == Token::RightBracket {
                        break;
                    }
                    items.push(self.parse_expression());
                    if let Some(Token::Comma) = self.current() {
                        self.advance();
                    }
                }
                self.advance(); // ]
                Expr::ArrayLiteral(items)
            }
            Some(Token::AI) => {
                self.advance(); // ai
                if let Some(Token::Dot) = self.current() {
                    self.advance(); // .
                    if let Some(Token::Identifier(method)) = self.current() {
                        let method = method.clone();
                        self.advance();
                        
                        if let Some(Token::LeftParen) = self.current() {
                            self.advance(); // (
                            let mut args = Vec::new();
                            while let Some(token) = self.current() {
                                if *token == Token::RightParen {
                                    break;
                                }
                                args.push(self.parse_expression());
                                if let Some(Token::Comma) = self.current() {
                                    self.advance();
                                }
                            }
                            self.advance(); // )
                            return Expr::AICall { method, args };
                        } else {
                            panic!("Expected '(' after ai.{}", method);
                        }
                    } else {
                        panic!("Expected method name after 'ai.'");
                    }
                } else {
                    panic!("Expected '.' after 'ai'");
                }
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();

                // Check for property access (e.g., msg.sender, chain.id)
                if let Some(Token::Dot) = self.current() {
                    self.advance(); // .
                    if let Some(Token::Identifier(property)) = self.current() {
                        let property = property.clone();
                        self.advance();
                        return Expr::Property {
                            object: name,
                            property,
                        };
                    } else {
                        panic!("Expected property name after '.'");
                    }
                }

                if let Some(Token::LeftParen) = self.current() {
                    self.advance(); // (
                    let mut args = Vec::new();
                    while let Some(token) = self.current() {
                        if *token == Token::RightParen {
                            break;
                        }
                        args.push(self.parse_expression());
                        if let Some(Token::Comma) = self.current() {
                            self.advance();
                        }
                    }
                    self.advance(); // )
                    Expr::Call { name, args }
                } else {
                    Expr::Identifier(name)
                }
            }
            _ => panic!("Unexpected token"),
        }
    }

    fn consume_identifier(&mut self) -> String {
        if let Some(Token::Identifier(n)) = self.current() {
            let name = n.clone();
            self.advance();
            name
        } else {
            panic!("Expected identifier");
        }
    }
}
