// ASTRIXA Abstract Syntax Tree (AST)

#[derive(Debug, Clone)]
pub enum Expr {
    StringLiteral(String),
    NumberLiteral(i64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Function {
        name: String,
        body: Vec<Stmt>,
    },
    Let {
        name: String,
        value: Expr,
    },
    Expression(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Vec<Stmt>,
    },
}
