/// ASTRIXA Optimization Passes
/// 
/// All optimizations happen on IR — never on AST.
/// 
/// Optimization Passes:
/// 1. Constant Folding: Evaluate constant expressions at compile time
/// 2. Dead Code Elimination: Remove unreachable instructions after returns
/// 
/// These optimizations:
/// - Improve performance
/// - Reduce unnecessary instructions
/// - Prepare code for WASM / native / contract targets

use crate::ir::{IRInstr, IRModule};

/// Optimize an entire IR module
pub fn optimize_module(module: IRModule) -> IRModule {
    let mut optimized_module = IRModule::new();
    
    for mut function in module.functions {
        // Apply all optimization passes to each function
        let optimized_instructions = optimize(&function.instructions);
        let cleaned_instructions = remove_dead_code(&optimized_instructions);
        
        function.instructions = cleaned_instructions;
        optimized_module.add_function(function);
    }
    
    optimized_module
}

/// Constant Folding Optimization
/// 
/// Evaluates constant expressions at compile time.
/// Example: 2 + 3 becomes 5
/// 
/// How it works:
/// 1. Track a stack of constant values
/// 2. When we see LoadConstInt, push it to the stack
/// 3. When we see Add, if both operands are on the stack, fold them
/// 4. If we can't fold, emit the instructions as-is
pub fn optimize(instrs: &[IRInstr]) -> Vec<IRInstr> {
    let mut stack: Vec<i64> = Vec::new();
    let mut optimized = Vec::new();

    for instr in instrs {
        match instr {
            // For integer constants, try to fold
            IRInstr::LoadConstInt(n) => {
                stack.push(*n);
                // Don't emit yet - wait to see if we can fold
            }
            
            // Arithmetic operations - try to fold constants
            IRInstr::Add => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    // Constant folding: a + b
                    stack.push(a + b);
                } else {
                    // Can't fold, emit the instructions
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Add);
                }
            }
            
            IRInstr::Sub => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    // Constant folding: a - b
                    stack.push(a - b);
                } else {
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Sub);
                }
            }
            
            IRInstr::Mul => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    // Constant folding: a * b
                    stack.push(a * b);
                } else {
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Mul);
                }
            }
            
            IRInstr::Div => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    // Constant folding: a / b (avoid division by zero at compile time)
                    if b != 0 {
                        stack.push(a / b);
                    } else {
                        emit_stack_to_ir(&mut stack, &mut optimized);
                        optimized.push(IRInstr::Div);
                    }
                } else {
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Div);
                }
            }
            
            IRInstr::Mod => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    // Constant folding: a % b
                    if b != 0 {
                        stack.push(a % b);
                    } else {
                        emit_stack_to_ir(&mut stack, &mut optimized);
                        optimized.push(IRInstr::Mod);
                    }
                } else {
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Mod);
                }
            }
            
            // For comparison operations - fold if possible
            IRInstr::Eq => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    // Comparison folding: replace with constant
                    stack.push(if a == b { 1 } else { 0 });
                } else {
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Eq);
                }
            }
            
            IRInstr::Ne => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a != b { 1 } else { 0 });
                } else {
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Ne);
                }
            }
            
            IRInstr::Lt => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a < b { 1 } else { 0 });
                } else {
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Lt);
                }
            }
            
            IRInstr::Le => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a <= b { 1 } else { 0 });
                } else {
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Le);
                }
            }
            
            IRInstr::Gt => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a > b { 1 } else { 0 });
                } else {
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Gt);
                }
            }
            
            IRInstr::Ge => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(if a >= b { 1 } else { 0 });
                } else {
                    emit_stack_to_ir(&mut stack, &mut optimized);
                    optimized.push(IRInstr::Ge);
                }
            }
            
            // For non-arithmetic operations, emit stack and pass through
            _ => {
                emit_stack_to_ir(&mut stack, &mut optimized);
                optimized.push(instr.clone());
            }
        }
    }

    // Emit any remaining constants in the stack
    emit_stack_to_ir(&mut stack, &mut optimized);

    optimized
}

/// Helper: Emit all stacked constants as LoadConstInt instructions
fn emit_stack_to_ir(stack: &mut Vec<i64>, optimized: &mut Vec<IRInstr>) {
    while let Some(val) = stack.pop() {
        optimized.insert(optimized.len() - (stack.len()), IRInstr::LoadConstInt(val));
    }
}

/// Dead Code Elimination
/// 
/// Removes unreachable code after control flow instructions.
/// Example:
/// ```
/// fn test {
///     return
///     print("never runs")  // <- Dead code
/// }
/// ```
/// 
/// Simple rule: Once Return is seen, stop adding instructions.
pub fn remove_dead_code(instrs: &[IRInstr]) -> Vec<IRInstr> {
    let mut result = Vec::new();

    for instr in instrs {
        result.push(instr.clone());
        
        // Stop after control flow instructions that don't return to next instruction
        match instr {
            IRInstr::Return => {
                break;
            }
            IRInstr::Jump(_) => {
                // Jump always transfers control elsewhere, stop here
                break;
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_folding_addition() {
        let instrs = vec![
            IRInstr::LoadConstInt(2),
            IRInstr::LoadConstInt(3),
            IRInstr::Add,
            IRInstr::Return,
        ];

        let optimized = optimize(&instrs);
        
        // Should have: LoadConstInt(5), Return
        assert!(optimized.len() <= instrs.len());
        assert!(optimized.iter().any(|i| matches!(i, IRInstr::LoadConstInt(5))));
    }

    #[test]
    fn test_constant_folding_multiplication() {
        let instrs = vec![
            IRInstr::LoadConstInt(4),
            IRInstr::LoadConstInt(5),
            IRInstr::Mul,
            IRInstr::Return,
        ];

        let optimized = optimize(&instrs);
        
        // Should fold 4 * 5 = 20
        assert!(optimized.iter().any(|i| matches!(i, IRInstr::LoadConstInt(20))));
    }

    #[test]
    fn test_dead_code_after_return() {
        let instrs = vec![
            IRInstr::LoadConstInt(42),
            IRInstr::Return,
            IRInstr::LoadConstInt(99),  // Dead code
            IRInstr::Add,               // Dead code
        ];

        let cleaned = remove_dead_code(&instrs);
        
        // Should have: LoadConstInt(42), Return
        assert_eq!(cleaned.len(), 2);
        assert!(matches!(cleaned[1], IRInstr::Return));
    }

    #[test]
    fn test_no_dead_code_before_return() {
        let instrs = vec![
            IRInstr::LoadConstInt(42),
            IRInstr::LoadConstInt(99),
            IRInstr::Add,
            IRInstr::Return,
        ];

        let cleaned = remove_dead_code(&instrs);
        
        // All instructions should remain
        assert_eq!(cleaned.len(), instrs.len());
    }

    #[test]
    fn test_dead_code_after_jump() {
        let instrs = vec![
            IRInstr::Jump(0),
            IRInstr::LoadConstInt(42),  // Dead code
            IRInstr::Return,
        ];

        let cleaned = remove_dead_code(&instrs);
        
        // Should have: Jump(0)
        assert_eq!(cleaned.len(), 1);
        assert!(matches!(cleaned[0], IRInstr::Jump(0)));
    }
}
