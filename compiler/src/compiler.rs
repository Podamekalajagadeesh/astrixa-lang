// ASTRIXA Bytecode Compiler: AST → Bytecode

use crate::ast::{Expr, Stmt};
use crate::bytecode::{OpCode, Instruction};
use std::collections::HashMap;

pub struct Compiler {
    pub instructions: Vec<Instruction>,
    functions: HashMap<String, Vec<Instruction>>,
    locals: Vec<HashMap<String, usize>>,
    mode: String,  // "native", "contract", "wasm", "web"
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
            functions: HashMap::new(),
            locals: vec![HashMap::new()],
            mode: "native".to_string(),
        }
    }
    
    pub fn set_mode(&mut self, mode: &str) {
        self.mode = mode.to_string();
    }
    
    pub fn get_mode(&self) -> &str {
        &self.mode
    }

    pub fn compile(&mut self, program: Vec<Stmt>) -> Result<Vec<Instruction>, String> {
        // First pass: collect all functions
        for stmt in &program {
            match stmt {
                Stmt::Function { name, params, body, .. } => {
                    self.compile_function(name.clone(), params.clone(), body.clone())?;
                }
                _ => {}
            }
        }

        // Second pass: compile main and imports
        for stmt in program {
            match stmt {
                Stmt::Function { .. } => {} // Already compiled
                Stmt::Import(_module) => {
                    // Imports are handled at runtime
                }
                _ => {
                    self.compile_stmt(stmt)?;
                }
            }
        }

        Ok(self.instructions.clone())
    }

    fn compile_function(&mut self, name: String, params: Vec<String>, body: Vec<Stmt>) -> Result<(), String> {
        let mut func_instructions = Vec::new();
        
        // Save current state
        let saved_instructions = self.instructions.clone();
        self.instructions = Vec::new();
        self.locals.push(HashMap::new());

        // Add parameters to local scope
        for (i, param) in params.iter().enumerate() {
            self.locals.last_mut().unwrap().insert(param.clone(), i);
        }

        // Compile function body
        for stmt in body {
            self.compile_stmt(stmt)?;
        }

        // Add implicit return null if no return statement
        if self.instructions.last().map(|i| i.opcode != OpCode::Return).unwrap_or(true) {
            self.emit(OpCode::LoadConst, Some("null".to_string()));
            self.emit(OpCode::Return, None);
        }

        func_instructions = self.instructions.clone();
        self.locals.pop();
        self.instructions = saved_instructions;

        self.functions.insert(name, func_instructions);
        Ok(())
    }

    fn compile_stmt(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, value } => {
                self.compile_expr(value)?;
                self.emit(OpCode::StoreVar, Some(name));
                self.emit(OpCode::Pop, None);
                Ok(())
            }
            Stmt::Expression(expr) => {
                self.compile_expr(expr)?;
                self.emit(OpCode::Pop, None);
                Ok(())
            }
            Stmt::Assign { name, value } => {
                self.compile_expr(value)?;
                self.emit(OpCode::StoreVar, Some(name));
                self.emit(OpCode::Pop, None);
                Ok(())
            }
            Stmt::Return(expr) => {
                self.compile_expr(expr)?;
                self.emit(OpCode::Return, None);
                Ok(())
            }
            Stmt::If { condition, then_body, else_body } => {
                self.compile_expr(condition)?;
                let jump_if_false_addr = self.instructions.len();
                self.emit(OpCode::JumpIfFalse, Some("0".to_string())); // Patched to real address below

                // Compile then branch
                for stmt in then_body {
                    self.compile_stmt(stmt)?;
                }

                let jump_addr = self.instructions.len();
                self.emit(OpCode::Jump, Some("0".to_string())); // Patched to real address below

                // Patch jump_if_false
                self.instructions[jump_if_false_addr].operand = Some(self.instructions.len().to_string());

                // Compile else branch
                if let Some(else_b) = else_body {
                    for stmt in else_b {
                        self.compile_stmt(stmt)?;
                    }
                }

                // Patch jump
                self.instructions[jump_addr].operand = Some(self.instructions.len().to_string());

                Ok(())
            }
            Stmt::While { condition, body } => {
                let loop_start = self.instructions.len();

                self.compile_expr(condition)?;
                let jump_if_false_addr = self.instructions.len();
                self.emit(OpCode::JumpIfFalse, Some("0".to_string())); // Patched to exit address below

                for stmt in body {
                    self.compile_stmt(stmt)?;
                }

                self.emit(OpCode::Jump, Some(loop_start.to_string()));

                // Patch jump_if_false
                self.instructions[jump_if_false_addr].operand = Some(self.instructions.len().to_string());

                Ok(())
            }
            Stmt::Function { .. } => {
                // Functions are compiled separately
                Ok(())
            }
            Stmt::Import(_) => {
                // Imports are handled at runtime
                Ok(())
            }
            Stmt::Panic(_) => {
                // Panic statements are handled at runtime
                Ok(())
            }
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> Result<(), String> {
        match expr {
            Expr::Number(n) => {
                self.emit(OpCode::LoadConst, Some(n.to_string()));
                Ok(())
            }
            Expr::Float(f) => {
                self.emit(OpCode::LoadConst, Some(f.to_string()));
                Ok(())
            }
            Expr::String(s) => {
                self.emit(OpCode::LoadConst, Some(format!("\"{}\"", s)));
                Ok(())
            }
            Expr::Bool(b) => {
                self.emit(OpCode::LoadConst, Some(b.to_string()));
                Ok(())
            }
            Expr::Identifier(name) => {
                self.emit(OpCode::LoadVar, Some(name));
                Ok(())
            }
            Expr::Add(left, right) | Expr::Sub(left, right) | 
            Expr::Mul(left, right) | Expr::Div(left, right) | Expr::Mod(left, right) => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;

                let opcode = match &expr {
                    Expr::Add(_, _) => OpCode::Add,
                    Expr::Sub(_, _) => OpCode::Sub,
                    Expr::Mul(_, _) => OpCode::Mul,
                    Expr::Div(_, _) => OpCode::Div,
                    Expr::Mod(_, _) => OpCode::Mod,
                    _ => unreachable!(),
                };

                self.emit(opcode, None);
                Ok(())
            }
            Expr::Eq(left, right) | Expr::Ne(left, right) |
            Expr::Lt(left, right) | Expr::Le(left, right) |
            Expr::Gt(left, right) | Expr::Ge(left, right) => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;

                let opcode = match &expr {
                    Expr::Eq(_, _) => OpCode::Equal,
                    Expr::Ne(_, _) => OpCode::NotEqual,
                    Expr::Lt(_, _) => OpCode::Less,
                    Expr::Le(_, _) => OpCode::LessEqual,
                    Expr::Gt(_, _) => OpCode::Greater,
                    Expr::Ge(_, _) => OpCode::GreaterEqual,
                    _ => unreachable!(),
                };

                self.emit(opcode, None);
                Ok(())
            }
            Expr::Call(name, args) => {
                // Push arguments
                for arg in args {
                    self.compile_expr(arg)?;
                }
                self.emit(OpCode::Call, Some(name));
                Ok(())
            }
            Expr::ModuleCall(module, func, args) => {
                // Push arguments
                for arg in args {
                    self.compile_expr(arg)?;
                }
                self.emit(OpCode::Call, Some(format!("{}.{}", module, func)));
                Ok(())
            }
        }
    }

    fn emit(&mut self, opcode: OpCode, operand: Option<String>) {
        self.instructions.push(Instruction::new(opcode, operand));
    }

    pub fn get_function_code(&self, name: &str) -> Option<Vec<Instruction>> {
        self.functions.get(name).cloned()
    }
}
