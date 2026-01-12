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

/// Lower an AST into IR
pub fn lower(stmts: &[Stmt]) -> IRModule {
    let mut module = IRModule::new();

    for stmt in stmts {
        if let Stmt::Function { name, body, .. } = stmt {
            let function = lower_function(name, body);
            module.add_function(function);
        }
    }

    module
}

/// Lower a single function to IR
fn lower_function(name: &str, body: &[Stmt]) -> IRFunction {
    let mut function = IRFunction::new(name.to_string());
    
    // Lower function body
    for stmt in body {
        lower_statement(stmt, &mut function);
    }
    
    // Ensure function ends with return
    if function.instructions.is_empty() 
        || !matches!(function.instructions.last(), Some(IRInstr::Return)) {
        function.add_instruction(IRInstr::LoadConstInt(0));
        function.add_instruction(IRInstr::Return);
    }
    
    function
}

/// Lower a single statement
fn lower_statement(stmt: &Stmt, function: &mut IRFunction) {
    match stmt {
        Stmt::Expression(expr) => {
            lower_expression(expr, function);
            // Pop the result if it's not used
            function.add_instruction(IRInstr::Pop);
        }
        Stmt::Function { .. } => {
            // Nested functions not supported yet
        }
    }
}

/// Lower an expression
fn lower_expression(expr: &Expr, function: &mut IRFunction) {
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
        Expr::Text(t) => {
            function.add_instruction(IRInstr::LoadConstString(t.clone()));
        }
        Expr::Identifier(name) => {
            function.add_instruction(IRInstr::LoadVar(name.clone()));
        }
        Expr::Call(name, args) => {
            // Lower arguments first (left to right)
            for arg in args {
                lower_expression(arg, function);
            }
            
            // Check if this is a stdlib function
            if is_stdlib_function(name) {
                function.add_instruction(IRInstr::CallStd(name.clone()));
            } else {
                function.add_instruction(IRInstr::Call(name.clone(), args.len()));
            }
        }
    }
}

/// Check if a function is a standard library function
fn is_stdlib_function(name: &str) -> bool {
    matches!(name, "print" | "println")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Type;
    
    #[test]
    fn test_lower_empty_function() {
        let stmts = vec![Stmt::Function {
            name: "test".to_string(),
            return_type: Type::Void,
            body: vec![],
        }];
        
        let module = lower(&stmts);
        assert_eq!(module.functions.len(), 1);
        assert_eq!(module.functions[0].name, "test");
        assert_eq!(module.functions[0].instructions.len(), 1);
        assert!(matches!(
            module.functions[0].instructions[0],
            IRInstr::Return
        ));
    }
    
    #[test]
    fn test_lower_multiple_functions() {
        let stmts = vec![
            Stmt::Function {
                name: "foo".to_string(),
                return_type: Type::Void,
                body: vec![],
            },
            Stmt::Function {
                name: "bar".to_string(),
                return_type: Type::Void,
                body: vec![],
            },
        ];
        
        let module = lower(&stmts);
        assert_eq!(module.functions.len(), 2);
        assert_eq!(module.functions[0].name, "foo");
        assert_eq!(module.functions[1].name, "bar");
    }
}
