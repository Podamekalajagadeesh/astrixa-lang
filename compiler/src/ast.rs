#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Identifier(String),
}

#[derive(Debug)]
pub enum Stmt {
    Function {
        name: String,
        body: Vec<Stmt>,
    },
}
