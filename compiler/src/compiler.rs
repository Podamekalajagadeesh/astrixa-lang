// ASTRIXA Bytecode Compiler: AST → Bytecode

use crate::ast::{Expr, Stmt, Function, Contract};
use crate::bytecode::{OpCode, Instruction};
use std::collections::HashMap;

pub struct Compiler {
    pub instructions: Vec<Instruction>,
    functions: HashMap<String, Vec<Instruction>>,
    contracts: HashMap<String, Contract>,
    locals: Vec<HashMap<String, usize>>,
    mode: String,  // "native", "contract", "wasm", "web"
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
            functions: HashMap::new(),
            contracts: HashMap::new(),
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
        // First pass: collect all functions and contracts
        for stmt in &program {
            match stmt {
                Stmt::Function(func) => {
                    self.compile_function(func.clone())?;
                }
                Stmt::Contract(contract) => {
                    self.contracts.insert(contract.name.clone(), contract.clone());
                    // Compile contract functions
                    for func in &contract.functions {
                        self.compile_function(func.clone())?;
                    }
                }
                _ => {}
            }
        }

        // Second pass: compile main and imports
        for stmt in program {
            match stmt {
                Stmt::Function(_) => {} // Already compiled
                Stmt::Contract(_) => {} // Already compiled
                Stmt::Import(module) => {
                    // Imports are handled at runtime
                }
                _ => {
                    self.compile_stmt(stmt)?;
                }
            }
        }

        Ok(self.instructions.clone())
    }

    fn compile_function(&mut self, func: Function) -> Result<(), String> {
        let mut func_instructions = Vec::new();
        
        // Save current state
        let saved_instructions = self.instructions.clone();
        self.instructions = Vec::new();
        self.locals.push(HashMap::new());

        // Add parameters to local scope
        for (i, param) in func.params.iter().enumerate() {
            self.locals.last_mut().unwrap().insert(param.clone(), i);
        }

        // Compile function body
        for stmt in func.body {
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

        self.functions.insert(func.name, func_instructions);
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
            Stmt::If { condition, then_branch, else_branch } => {
                self.compile_expr(condition)?;
                let jump_if_false_addr = self.instructions.len();
                self.emit(OpCode::JumpIfFalse, Some("0".to_string())); // Placeholder

                // Compile then branch
                for stmt in then_branch {
                    self.compile_stmt(stmt)?;
                }

                let jump_addr = self.instructions.len();
                self.emit(OpCode::Jump, Some("0".to_string())); // Placeholder

                // Patch jump_if_false
                self.instructions[jump_if_false_addr].operand = Some(self.instructions.len().to_string());

                // Compile else branch
                for stmt in else_branch {
                    self.compile_stmt(stmt)?;
                }

                // Patch jump
                self.instructions[jump_addr].operand = Some(self.instructions.len().to_string());

                Ok(())
            }
            Stmt::While { condition, body } => {
                let loop_start = self.instructions.len();

                self.compile_expr(condition)?;
                let jump_if_false_addr = self.instructions.len();
                self.emit(OpCode::JumpIfFalse, Some("0".to_string())); // Placeholder

                for stmt in body {
                    self.compile_stmt(stmt)?;
                }

                self.emit(OpCode::Jump, Some(loop_start.to_string()));

                // Patch jump_if_false
                self.instructions[jump_if_false_addr].operand = Some(self.instructions.len().to_string());

                Ok(())
            }
            Stmt::Function(_) => {
                // Functions are compiled separately
                Ok(())
            }
            Stmt::Contract(_) => {
                // Contracts are compiled separately
                Ok(())
            }
            Stmt::Import(_) => {
                // Imports are handled at runtime
                Ok(())
            }
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> Result<(), String> {
        match expr {
            Expr::NumberLiteral(n) => {
                self.emit(OpCode::LoadConst, Some(n.to_string()));
                Ok(())
            }
            Expr::StringLiteral(s) => {
                self.emit(OpCode::LoadConst, Some(format!("\"{}\"", s)));
                Ok(())
            }
            Expr::BoolLiteral(b) => {
                self.emit(OpCode::LoadConst, Some(b.to_string()));
                Ok(())
            }
            Expr::Identifier(name) => {
                self.emit(OpCode::LoadVar, Some(name));
                Ok(())
            }
            Expr::Property { object, property } => {
                // Emit a special load that references Web3 properties
                self.emit(OpCode::LoadVar, Some(format!("{}.{}", object, property)));
                Ok(())
            }
            Expr::AICall { method, args } => {
                // Push arguments for AI call
                for arg in args {
                    self.compile_expr(arg)?;
                }
                // Emit a special AI call opcode with method name and arg count
                self.emit(OpCode::Call, Some(format!("ai.{}", method)));
                Ok(())
            }
            Expr::ArrayLiteral(items) => {
                for item in &items {
                    self.compile_expr(item.clone())?;
                }
                self.emit(OpCode::Array, Some(items.len().to_string()));
                Ok(())
            }
            Expr::Binary { left, operator, right } => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;

                let opcode = match operator.as_str() {
                    "+" => OpCode::Add,
                    "-" => OpCode::Sub,
                    "*" => OpCode::Mul,
                    "/" => OpCode::Div,
                    ">" => OpCode::Greater,
                    "<" => OpCode::Less,
                    _ => return Err(format!("Unknown operator: {}", operator)),
                };

                self.emit(opcode, None);
                Ok(())
            }
            Expr::Call { name, args } => {
                // Push arguments
                for arg in args {
                    self.compile_expr(arg)?;
                }
                self.emit(OpCode::Call, Some(name));
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
