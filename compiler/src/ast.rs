// ASTRIXA Abstract Syntax Tree (AST)

#[derive(Debug, Clone)]
pub enum Expr {
    StringLiteral(String),
    NumberLiteral(i64),
    BoolLiteral(bool),
    ArrayLiteral(Vec<Expr>),
    Identifier(String),
    Property {
        object: String,
        property: String,
    },
    AICall {
        method: String,
        args: Vec<Expr>,
    },
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

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Contract {
    pub name: String,
    pub state: Vec<String>,        // State variable names
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Function(Function),
    Contract(Contract),
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
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Assign {
        name: String,
        value: Expr,
    },
    Return(Expr),
    Import(String),
}
