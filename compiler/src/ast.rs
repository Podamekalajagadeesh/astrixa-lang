use crate::types::Type;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Text(String),
    Identifier(String),
    Call(String, Vec<Expr>), // Function call: name, arguments
}

#[derive(Debug)]
pub enum Stmt {
    Function {
        name: String,
        return_type: Type,
        body: Vec<Stmt>,
    },
    Expression(Expr), // Expression statement
}
