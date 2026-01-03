use std::collections::HashMap;
use crate::ast::{Expr, Stmt};

#[derive(Clone)]
enum Value {
    String(String),
    Number(i64),
    Bool(bool),
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Function { name, body } => {
                if name == "main" {
                    for s in body {
                        self.execute(s);
                    }
                }
            }
            _ => {}
        }
    }

    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                let val = self.eval_expr(value);
                self.variables.insert(name, val);
            }
            Stmt::Expression(expr) => {
                self.eval_expr(expr);
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let cond = self.eval_expr(condition);

                match cond {
                    Value::Bool(true) => {
                        for stmt in then_branch {
                            self.execute(stmt);
                        }
                    }
                    Value::Bool(false) => {
                        for stmt in else_branch {
                            self.execute(stmt);
                        }
                    }
                    _ => panic!("Condition must be boolean"),
                }
            }
            _ => {}
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::StringLiteral(v) => Value::String(v),
            Expr::NumberLiteral(n) => Value::Number(n),
            Expr::Identifier(name) => self.variables.get(&name).unwrap().clone(),
            Expr::Binary { left, operator, right } => {
                let l = self.eval_expr(*left);
                let r = self.eval_expr(*right);

                match (l, r, operator.as_str()) {
                    (Value::Number(a), Value::Number(b), "+") => Value::Number(a + b),
                    (Value::Number(a), Value::Number(b), ">") => Value::Bool(a > b),
                    _ => panic!("Invalid operation"),
                }
            }
            Expr::Call { name, args } => {
                if name == "print" {
                    let val = self.eval_expr(args[0].clone());
                    match val {
                        Value::String(s) => println!("{}", s),
                        Value::Number(n) => println!("{}", n),
                        Value::Bool(b) => println!("{}", b),
                    }
                    Value::Number(0)
                } else {
                    panic!("Unknown function");
                }
            }
        }
    }
}
