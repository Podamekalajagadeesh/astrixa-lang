use std::collections::HashMap;
use crate::types::Type;
use crate::ast::{Stmt, Expr};

// STEP 46: Function signature for type checking
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub params: Vec<Type>,
    pub return_type: Type,
}

pub struct TypeChecker {
    symbols: HashMap<String, Type>,
    functions: HashMap<String, FunctionSignature>,  // STEP 46: Function signatures
    errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            functions: HashMap::new(),  // STEP 46: Initialize function table
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
            Stmt::Function { name, params, return_type, body } => {
                // STEP 46: Register function signature
                let param_types = vec![Type::Int; params.len()]; // For V1, all params are Int
                let sig = FunctionSignature {
                    params: param_types.clone(),
                    return_type: return_type.clone(),
                };
                self.functions.insert(name.clone(), sig);
                self.symbols.insert(name.clone(), return_type.clone());

                // STEP 46: Register parameters as local variables
                for param in params {
                    self.symbols.insert(param.clone(), Type::Int);
                }

                // Check function body
                for inner_stmt in body {
                    self.check_stmt(inner_stmt);
                }
                
                // STEP 46: Clean up parameter symbols after function
                for param in params {
                    self.symbols.remove(param);
                }
            }
            Stmt::Expression(_expr) => {
                // Check expression statement
                // For now, just validate the expression exists
            }
            Stmt::Let { name, value } => {
                // Check the value expression
                let _value_type = self.check_expr(value);
                // In a full type system, we'd infer the type of name from value
                // For now, store it as Unknown
                self.symbols.insert(name.clone(), Type::Unknown);
            }
            Stmt::If { condition, then_body, else_body } => {
                // Check condition expression must be Int (boolean)
                let cond_type = self.check_expr(condition);
                if cond_type != Type::Int && cond_type != Type::Bool && cond_type != Type::Unknown {
                    self.errors.push(format!(
                        "Type error: if condition must be Int, got {:?}",
                        cond_type
                    ));
                }
                
                // Check then body
                for stmt in then_body {
                    self.check_stmt(stmt);
                }
                
                // Check else body if present
                if let Some(body) = else_body {
                    for stmt in body {
                        self.check_stmt(stmt);
                    }
                }
            }
            Stmt::Assign { name, value } => {
                // Check that variable is already defined
                if !self.symbols.contains_key(name) {
                    self.errors.push(format!(
                        "Type error: variable '{}' not defined",
                        name
                    ));
                }
                
                // Check the value expression type is compatible
                let value_type = self.check_expr(value);
                let var_type = self.symbols.get(name).cloned().unwrap_or(Type::Unknown);
                
                if var_type != Type::Unknown && value_type != Type::Unknown && var_type != value_type {
                    self.errors.push(format!(
                        "Type error: cannot assign {:?} to variable of type {:?}",
                        value_type, var_type
                    ));
                }
            }
            Stmt::While { condition, body } => {
                // Check condition expression must be Int (boolean)
                let cond_type = self.check_expr(condition);
                if cond_type != Type::Int && cond_type != Type::Bool && cond_type != Type::Unknown {
                    self.errors.push(format!(
                        "Type error: while condition must be Int, got {:?}",
                        cond_type
                    ));
                }
                
                // Check loop body
                for stmt in body {
                    self.check_stmt(stmt);
                }
            }
            Stmt::Return(expr) => {
                // STEP 46: Check return expression type
                let _return_type = self.check_expr(expr);
                // In V1, we accept any return type
            }
            Stmt::Panic(expr) => {
                // STEP 48: Check panic expression must be a string
                let expr_type = self.check_expr(expr);
                if expr_type != Type::String && expr_type != Type::Text && expr_type != Type::Unknown {
                    self.errors.push(format!(
                        "Type error: panic() requires a string message, got {:?}",
                        expr_type
                    ));
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
            Expr::Call(name, args) => {
                // STEP 46: Check function call arguments
                // Clone the signature to avoid borrowing issues
                let sig = self.functions.get(name).cloned();
                
                if let Some(sig) = sig {
                    // Check argument count
                    if args.len() != sig.params.len() {
                        self.errors.push(format!(
                            "Type error: function '{}' expects {} arguments, got {}",
                            name, sig.params.len(), args.len()
                        ));
                    }
                    
                    // Check argument types
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = self.check_expr(arg);
                        if i < sig.params.len() {
                            let expected_type = &sig.params[i];
                            if arg_type != *expected_type && arg_type != Type::Unknown {
                                self.errors.push(format!(
                                    "Type error: argument {} of function '{}' expects {:?}, got {:?}",
                                    i, name, expected_type, arg_type
                                ));
                            }
                        }
                    }
                    
                    sig.return_type.clone()
                } else {
                    // For stdlib or unknown functions, assume Int return type
                    Type::Int
                }
            }
            
            // Arithmetic operators: require both operands to be Int
            Expr::Add(left, right) | Expr::Sub(left, right) | 
            Expr::Mul(left, right) | Expr::Div(left, right) | Expr::Mod(left, right) => {
                let left_type = self.check_expr(left);
                let right_type = self.check_expr(right);
                
                if left_type == Type::Int && right_type == Type::Int {
                    Type::Int
                } else {
                    self.errors.push(format!(
                        "Type error: arithmetic operators require Int operands, got {:?} and {:?}",
                        left_type, right_type
                    ));
                    Type::Unknown
                }
            }
            
            // Comparison operators: require Int operands, return Bool
            Expr::Eq(left, right) | Expr::Ne(left, right) |
            Expr::Lt(left, right) | Expr::Le(left, right) |
            Expr::Gt(left, right) | Expr::Ge(left, right) => {
                let left_type = self.check_expr(left);
                let right_type = self.check_expr(right);
                
                if left_type == Type::Int && right_type == Type::Int {
                    Type::Bool
                } else {
                    self.errors.push(format!(
                        "Type error: comparison operators require Int operands, got {:?} and {:?}",
                        left_type, right_type
                    ));
                    Type::Bool
                }
            }
        }
    }

    pub fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}
