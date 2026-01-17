/// AST to IR Lowering
/// 
/// This module converts the Abstract Syntax Tree (AST) into the
/// Intermediate Representation (IR).
/// 
/// The lowering process:
/// 1. Traverses the AST
/// 2. Generates linear instruction sequences
/// 3. Resolves control flow to jump targets
/// 4. Produces optimizable IR
/// 5. Maps stdlib calls to CallStd instructions

use crate::ast::{Expr, Stmt};
use crate::ir::{IRFunction, IRInstr, IRModule};
use std::collections::HashMap;

/// Context for lowering - tracks variables and their stack slots
#[derive(Debug, Clone)]
pub struct LowerCtx {
    locals: HashMap<String, u32>,
    next_slot: u32,
}

impl LowerCtx {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
            next_slot: 0,
        }
    }
    
    /// Allocate a new local variable slot
    pub fn alloc(&mut self, name: String) -> u32 {
        let slot = self.next_slot;
        self.locals.insert(name, slot);
        self.next_slot += 1;
        slot
    }
    
    /// Get the slot for a variable
    pub fn get(&self, name: &str) -> Option<u32> {
        self.locals.get(name).copied()
    }
    
    /// Get the number of local slots used
    pub fn num_locals(&self) -> u32 {
        self.next_slot
    }
}

/// Lower an AST into IR
pub fn lower(stmts: &[Stmt]) -> IRModule {
    let mut module = IRModule::new();

    for stmt in stmts {
        match stmt {
            Stmt::Import(_) => {
                // Imports are handled at compilation level, not lowered to IR
            }
            Stmt::Function { name, params, body, exported: _, .. } => {
                let function = lower_function(name, params, body);
                module.add_function(function);
                // Note: `exported` flag is tracked in AST but doesn't affect IR
            }
            _ => {
                // Other statements not allowed at module level
            }
        }
    }

    module
}

/// Lower a single function to IR
fn lower_function(name: &str, params: &[String], body: &[Stmt]) -> IRFunction {
    let mut function = IRFunction::new(name.to_string());
    let mut ctx = LowerCtx::new();
    
    // STEP 46: Allocate slots for parameters first
    for param in params {
        ctx.alloc(param.clone());
    }
    function.param_count = params.len();
    
    // Lower function body
    for stmt in body {
        lower_statement(stmt, &mut function, &mut ctx);
    }
    
    // Ensure function ends with return
    if function.instructions.is_empty() 
        || !matches!(function.instructions.last(), Some(IRInstr::Return)) {
        function.add_instruction(IRInstr::LoadConstInt(0));
        function.add_instruction(IRInstr::Return);
    }
    
    // Store the number of locals in the function
    function.local_count = ctx.num_locals() as usize;
    
    function
}

/// Lower a single statement
fn lower_statement(stmt: &Stmt, function: &mut IRFunction, ctx: &mut LowerCtx) {
    match stmt {
        Stmt::Import(_) => {
            // Imports are handled at module level, not lowered to IR
        }
        Stmt::Expression(expr) => {
            lower_expression(expr, function, ctx);
            // Don't emit Pop - let the WASM codegen decide whether to drop based on context
            // In most cases, expression statements don't have their result used
        }
        Stmt::Let { name, value } => {
            // Allocate a new local variable slot
            let slot = ctx.alloc(name.clone());
            
            // Lower the initializer expression
            lower_expression(value, function, ctx);
            
            // Store to the local slot
            function.add_instruction(IRInstr::StoreLocal(slot));
        }
        Stmt::Function { .. } => {
            // Nested functions not supported yet
        }
        Stmt::If { condition, then_body, else_body } => {
            // Lower the condition expression
            lower_expression(condition, function, ctx);
            
            // Add JumpIfFalse - target will be patched in a later step
            let jump_if_false_index = function.instructions.len();
            function.add_instruction(IRInstr::JumpIfFalse(0));
            
            // Lower then body
            for stmt in then_body {
                lower_statement(stmt, function, ctx);
            }
            
            // Check if we have an else body
            if let Some(else_body) = else_body {
                // Add Jump to skip else body - target will be patched later
                let jump_index = function.instructions.len();
                function.add_instruction(IRInstr::Jump(0));
                
                // Patch JumpIfFalse to jump here (to else body)
                let else_start = function.instructions.len();
                if let IRInstr::JumpIfFalse(ref mut target) = &mut function.instructions[jump_if_false_index] {
                    *target = else_start;
                }
                
                // Lower else body
                for stmt in else_body {
                    lower_statement(stmt, function, ctx);
                }
                
                // Patch Jump to jump here (after else body)
                let end = function.instructions.len();
                if let IRInstr::Jump(ref mut target) = &mut function.instructions[jump_index] {
                    *target = end;
                }
            } else {
                // No else body - JumpIfFalse jumps to after then body
                let end = function.instructions.len();
                if let IRInstr::JumpIfFalse(ref mut target) = &mut function.instructions[jump_if_false_index] {
                    *target = end;
                }
            }
        }
        Stmt::Assign { name, value } => {
            // Get the slot for the variable (must be already allocated)
            if let Some(slot) = ctx.get(name) {
                // Lower the right-hand side expression
                lower_expression(value, function, ctx);
                
                // Store to the local slot
                function.add_instruction(IRInstr::StoreLocal(slot));
            }
            // If variable not found, the type checker should have caught this
        }
        Stmt::While { condition, body } => {
            // Mark the start of the loop
            let loop_start = function.instructions.len();
            
            // Lower the condition expression
            lower_expression(condition, function, ctx);
            
            // Add JumpIfFalse to exit the loop - target will be patched later
            let jump_if_false_index = function.instructions.len();
            function.add_instruction(IRInstr::JumpIfFalse(0));
            
            // Lower loop body
            for stmt in body {
                lower_statement(stmt, function, ctx);
            }
            
            // Add Jump back to loop start
            function.add_instruction(IRInstr::Jump(loop_start));
            
            // Patch JumpIfFalse to jump here (exit point)
            let loop_end = function.instructions.len();
            if let IRInstr::JumpIfFalse(ref mut target) = &mut function.instructions[jump_if_false_index] {
                *target = loop_end;
            }
        }
        Stmt::Return(expr) => {
            // STEP 46: Lower return statement
            lower_expression(expr, function, ctx);
            function.add_instruction(IRInstr::Return);
        }
        Stmt::Panic(expr) => {
            // STEP 48: Lower panic statement
            // Evaluate the error message expression
            lower_expression(expr, function, ctx);
            // Emit panic instruction (message is on stack)
            function.add_instruction(IRInstr::Panic);
        }
    }
}

/// Lower an expression
fn lower_expression(expr: &Expr, function: &mut IRFunction, ctx: &LowerCtx) {
    match expr {
        Expr::Number(n) => {
            function.add_instruction(IRInstr::LoadConstInt(*n));
        }
        Expr::Float(f) => {
            function.add_instruction(IRInstr::LoadConstFloat(*f));
        }
        Expr::Bool(b) => {
            function.add_instruction(IRInstr::LoadConstBool(*b));
        }
        Expr::String(s) => {
            function.add_instruction(IRInstr::LoadConstString(s.clone()));
        }
        Expr::Identifier(name) => {
            // Check if it's a local variable first
            if let Some(slot) = ctx.get(name) {
                function.add_instruction(IRInstr::LoadLocal(slot));
            } else {
                // Fallback to named variable (for backward compatibility)
                function.add_instruction(IRInstr::LoadVar(name.clone()));
            }
        }
        Expr::Call(name, args) => {
            // Lower arguments first (left to right)
            for arg in args {
                lower_expression(arg, function, ctx);
            }
            
            // STEP 53: Check if this is a Web3 function
            if is_web3_function(name) {
                function.add_instruction(IRInstr::CallWeb3(name.clone()));
            }
            // STEP 52: Check if this is an AI function
            else if is_ai_function(name) {
                function.add_instruction(IRInstr::CallAI(name.clone()));
            }
            // Check if this is a stdlib function
            else if is_stdlib_function(name) {
                function.add_instruction(IRInstr::CallStd(name.clone()));
            } else {
                function.add_instruction(IRInstr::Call(name.clone(), args.len()));
            }
        }
        
        // STEP 49: Module-qualified function call: module.function(args)
        Expr::ModuleCall(module_name, func_name, args) => {
            // Lower arguments first (left to right)
            for arg in args {
                lower_expression(arg, function, ctx);
            }
            
            // Generate fully qualified function name: module.func
            let qualified_name = format!("{}.{}", module_name, func_name);
            
            // STEP 52: Check if this is an AI function
            if is_ai_function(&qualified_name) {
                function.add_instruction(IRInstr::CallAI(qualified_name));
            }
            // STEP 53: Check if this is a Web3 function
            else if is_web3_function(&qualified_name) {
                function.add_instruction(IRInstr::CallWeb3(qualified_name));
            }
            // STEP 54: Check if this is a file system function
            else if is_fs_function(&qualified_name) {
                function.add_instruction(IRInstr::CallFS(qualified_name));
            } else {
                function.add_instruction(IRInstr::Call(qualified_name, args.len()));
            }
        }
        
        // Binary arithmetic operators (STEP 43)
        Expr::Add(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Add);
        }
        Expr::Sub(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Sub);
        }
        Expr::Mul(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Mul);
        }
        Expr::Div(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Div);
        }
        Expr::Mod(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Mod);
        }
        
        // Binary comparison operators (STEP 43)
        Expr::Eq(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Eq);
        }
        Expr::Ne(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Ne);
        }
        Expr::Lt(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Lt);
        }
        Expr::Le(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Le);
        }
        Expr::Gt(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Gt);
        }
        Expr::Ge(left, right) => {
            lower_expression(left, function, ctx);
            lower_expression(right, function, ctx);
            function.add_instruction(IRInstr::Ge);
        }
    }
}

/// Check if a function is a standard library function
fn is_stdlib_function(name: &str) -> bool {
    crate::stdlib::is_stdlib(name)
}

/// Check if a function is an AI function (STEP 52)
fn is_ai_function(name: &str) -> bool {
    crate::stdlib::is_ai(name)
}

/// Check if a function is a Web3 function (STEP 53)
fn is_web3_function(name: &str) -> bool {
    crate::stdlib::is_web3(name)
}

/// Check if a function is a file system function (STEP 54)
fn is_fs_function(name: &str) -> bool {
    crate::stdlib::is_fs_function(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Type;
    
    #[test]
    fn test_lower_empty_function() {
        let stmts = vec![Stmt::Function {
            name: "test".to_string(),
            params: vec![],
            return_type: Type::Void,
            body: vec![],
            exported: false,
        }];
        
        let module = lower(&stmts);
        assert_eq!(module.functions.len(), 1);
        assert_eq!(module.functions[0].name, "test");
        assert_eq!(module.functions[0].instructions.len(), 2); // LoadConstInt(0) + Return
        assert!(matches!(
            module.functions[0].instructions[1],
            IRInstr::Return
        ));
    }
    
    #[test]
    fn test_lower_multiple_functions() {
        let stmts = vec![
            Stmt::Function {
                name: "foo".to_string(),
                params: vec![],
                return_type: Type::Void,
                body: vec![],
                exported: false,
            },
            Stmt::Function {
                name: "bar".to_string(),
                params: vec![],
                return_type: Type::Void,
                body: vec![],
                exported: false,
            },
        ];
        
        let module = lower(&stmts);
        assert_eq!(module.functions.len(), 2);
        assert_eq!(module.functions[0].name, "foo");
        assert_eq!(module.functions[1].name, "bar");
    }
}
