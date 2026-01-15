use crate::types::Type;

// STEP 49: Module structure
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub imports: Vec<String>,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Identifier(String),
    Call(String, Vec<Expr>), // Function call: name, arguments
    ModuleCall(String, String, Vec<Expr>), // STEP 49: module.function(args)
    
    // Binary operators (STEP 43)
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    
    // Comparison operators (STEP 43)
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Import(String),    // STEP 49: import module_name
    Function {
        name: String,
        params: Vec<String>,  // STEP 46: Function parameters
        return_type: Type,
        body: Vec<Stmt>,
        exported: bool,      // STEP 49: export fn
    },
    Expression(Expr),  // Expression statement
    Let {              // Variable declaration
        name: String,
        value: Expr,
    },
    Assign {           // Variable assignment (NEW)
        name: String,
        value: Expr,
    },
    If {               // If/Else statement
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    While {            // While loop (NEW)
        condition: Expr,
        body: Vec<Stmt>,
    },
    Return(Expr),      // STEP 46: Return statement
    Panic(Expr),       // STEP 48: Panic statement - explicit failure
}
