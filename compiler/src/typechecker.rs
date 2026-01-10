use std::collections::HashMap;
use crate::types::Type;
use crate::ast::{Stmt, Expr};

pub struct TypeChecker {
    symbols: HashMap<String, Type>,
    errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn check(&mut self, stmts: &[Stmt]) -> Result<(), Vec<String>> {
        for stmt in stmts {
            self.check_stmt(stmt);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Function { name, return_type, body } => {
                // Register function in symbol table
                self.symbols.insert(name.clone(), return_type.clone());

                // Check function body
                for inner_stmt in body {
                    self.check_stmt(inner_stmt);
                }
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Number(_) => Type::Int,
            Expr::Float(_) => Type::Float,
            Expr::Bool(_) => Type::Bool,
            Expr::String(_) => Type::String,
            Expr::Text(_) => Type::Text,
            Expr::Identifier(name) => {
                self.symbols.get(name).cloned().unwrap_or(Type::Unknown)
            }
        }
    }

    pub fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}
