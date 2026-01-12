/// ASTRIXA WASM Code Generator
/// 
/// Converts optimized IR to WebAssembly Text (WAT) format.
/// 
/// Design principles:
/// - Simple and correct (not optimized for size/speed yet)
/// - Stack-based (natural fit with WASM)
/// - Type-neutral (all values as i32 for now)
/// - Produces valid, verifiable WAT
/// 
/// IR → WASM Mapping:
/// LoadConstInt(n)  → i32.const n
/// LoadConstFloat(f)→ f32.const f
/// Add              → i32.add / f32.add
/// Sub              → i32.sub / f32.sub
/// Mul              → i32.mul / f32.mul
/// Div              → i32.div_s / f32.div
/// Mod              → i32.rem_s
/// Return           → return
/// etc.

use crate::ir::{IRInstr, IRModule};
use std::collections::{HashMap, HashSet};

/// Generate WASM module from IR
pub fn generate_wasm_module(module: &IRModule) -> String {
    let mut wasm = String::new();
    
    // Module header
    wasm.push_str("(module\n");
    
    // Collect all stdlib imports needed
    let imports = collect_stdlib_imports(module);
    
    // Generate imports
    for import in &imports {
        wasm.push_str(&generate_import(import));
    }
    
    if !imports.is_empty() {
        wasm.push_str("\n");
    }
    
    // Generate each function
    for func in &module.functions {
        wasm.push_str(&generate_function(func.name.as_str(), &func.instructions));
        wasm.push_str("\n");
    }
    
    // Module footer
    wasm.push_str(")\n");
    
    wasm
}

/// Collect all stdlib function calls from the module
fn collect_stdlib_imports(module: &IRModule) -> HashSet<String> {
    let mut imports = HashSet::new();
    
    for func in &module.functions {
        for instr in &func.instructions {
            if let IRInstr::CallStd(name) = instr {
                imports.insert(name.clone());
            }
        }
    }
    
    imports
}

/// Generate WASM import declaration for a stdlib function
fn generate_import(func_name: &str) -> String {
    // For now, all stdlib functions take one i32 parameter
    // This can be extended based on function signatures
    match func_name {
        "print" => {
            "  (import \"env\" \"print\" (func $print (param i32)))\n".to_string()
        }
        "println" => {
            "  (import \"env\" \"println\" (func $println (param i32)))\n".to_string()
        }
        _ => {
            // Default: function with one i32 parameter
            format!("  (import \"env\" \"{}\" (func ${} (param i32)))\n", func_name, func_name)
        }
    }
}

/// Generate a single function in WASM
pub fn generate_function(name: &str, instrs: &[IRInstr]) -> String {
    let mut func_def = String::new();
    
    // Function definition
    func_def.push_str(&format!("  (func ${} (result i32)\n", name));
    
    // Generate function body
    let body = generate_body(instrs);
    func_def.push_str(&body);
    
    // Function close
    func_def.push_str("  )\n");
    
    // Export function
    func_def.push_str(&format!("  (export \"{}\" (func ${}))\n", name, name));
    
    func_def
}

/// Generate function body from IR instructions
fn generate_body(instrs: &[IRInstr]) -> String {
    let mut body = String::new();
    
    for instr in instrs {
        match instr {
            // Constants
            IRInstr::LoadConstInt(n) => {
                body.push_str(&format!("    i32.const {}\n", n));
            }
            IRInstr::LoadConstFloat(f) => {
                body.push_str(&format!("    f32.const {}\n", f));
            }
            IRInstr::LoadConstBool(b) => {
                let value = if *b { 1 } else { 0 };
                body.push_str(&format!("    i32.const {}\n", value));
            }
            IRInstr::LoadConstString(s) => {
                // For now, strings are not supported in basic WASM
                // Store as local and reference
                body.push_str(&format!("    ;; string: {}\n", escape_string(s)));
            }
            
            // Variables
            IRInstr::LoadVar(name) => {
                body.push_str(&format!("    local.get ${}\n", name));
            }
            IRInstr::StoreVar(name) => {
                body.push_str(&format!("    local.set ${}\n", name));
            }
            
            // Arithmetic (i32)
            IRInstr::Add => {
                body.push_str("    i32.add\n");
            }
            IRInstr::Sub => {
                body.push_str("    i32.sub\n");
            }
            IRInstr::Mul => {
                body.push_str("    i32.mul\n");
            }
            IRInstr::Div => {
                body.push_str("    i32.div_s\n");
            }
            IRInstr::Mod => {
                body.push_str("    i32.rem_s\n");
            }
            
            // Comparison (i32)
            IRInstr::Eq => {
                body.push_str("    i32.eq\n");
            }
            IRInstr::Ne => {
                body.push_str("    i32.ne\n");
            }
            IRInstr::Lt => {
                body.push_str("    i32.lt_s\n");
            }
            IRInstr::Le => {
                body.push_str("    i32.le_s\n");
            }
            IRInstr::Gt => {
                body.push_str("    i32.gt_s\n");
            }
            IRInstr::Ge => {
                body.push_str("    i32.ge_s\n");
            }
            
            // Logical
            IRInstr::And => {
                body.push_str("    i32.and\n");
            }
            IRInstr::Or => {
                body.push_str("    i32.or\n");
            }
            IRInstr::Not => {
                // NOT is: xor with 1
                body.push_str("    i32.const 1\n");
                body.push_str("    i32.xor\n");
            }
            
            // Control flow
            IRInstr::Jump(target) => {
                body.push_str(&format!("    br {}\n", target));
            }
            IRInstr::JumpIfFalse(target) => {
                body.push_str("    i32.eqz\n");
                body.push_str(&format!("    br_if {}\n", target));
            }
            
            // Function calls
            IRInstr::Call(func_name, _arg_count) => {
                body.push_str(&format!("    call ${}\n", func_name));
            }
            
            // Stdlib calls
            IRInstr::CallStd(func_name) => {
                body.push_str(&format!("    call ${}\n", func_name));
            }
            
            // Return
            IRInstr::Return => {
                body.push_str("    return\n");
            }
            
            // Stack manipulation
            IRInstr::Pop => {
                body.push_str("    drop\n");
            }
            IRInstr::Dup => {
                body.push_str("    local.tee $__dup_temp\n");
                body.push_str("    local.get $__dup_temp\n");
            }
            
            // Special
            IRInstr::Nop => {
                body.push_str("    nop\n");
            }
        }
    }
    
    body
}

/// Escape strings for WAT comments
fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
}

/// Generate standalone WAT function (for testing)
pub fn generate_wat(function_name: &str, instrs: &[IRInstr]) -> String {
    let mut wat = String::new();
    
    wat.push_str("(module\n");
    wat.push_str(&format!("  (func ${} (result i32)\n", function_name));
    
    for instr in instrs {
        match instr {
            IRInstr::LoadConstInt(n) => {
                wat.push_str(&format!("    i32.const {}\n", n));
            }
            IRInstr::Add => {
                wat.push_str("    i32.add\n");
            }
            IRInstr::Sub => {
                wat.push_str("    i32.sub\n");
            }
            IRInstr::Mul => {
                wat.push_str("    i32.mul\n");
            }
            IRInstr::Div => {
                wat.push_str("    i32.div_s\n");
            }
            IRInstr::Mod => {
                wat.push_str("    i32.rem_s\n");
            }
            IRInstr::Eq => {
                wat.push_str("    i32.eq\n");
            }
            IRInstr::Ne => {
                wat.push_str("    i32.ne\n");
            }
            IRInstr::Lt => {
                wat.push_str("    i32.lt_s\n");
            }
            IRInstr::Le => {
                wat.push_str("    i32.le_s\n");
            }
            IRInstr::Gt => {
                wat.push_str("    i32.gt_s\n");
            }
            IRInstr::Ge => {
                wat.push_str("    i32.ge_s\n");
            }
            IRInstr::Return => {
                wat.push_str("    return\n");
            }
            _ => {}
        }
    }
    
    wat.push_str("  )\n");
    wat.push_str(&format!("  (export \"{}\" (func ${}))\n", function_name, function_name));
    wat.push_str(")\n");
    
    wat
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::IRInstr;

    #[test]
    fn test_generate_wat_simple_add() {
        let instrs = vec![
            IRInstr::LoadConstInt(5),
            IRInstr::LoadConstInt(3),
            IRInstr::Add,
            IRInstr::Return,
        ];

        let wat = generate_wat("add_test", &instrs);
        
        // Verify it contains expected WASM instructions
        assert!(wat.contains("i32.const 5"));
        assert!(wat.contains("i32.const 3"));
        assert!(wat.contains("i32.add"));
        assert!(wat.contains("return"));
        assert!(wat.contains("(export \"add_test\""));
    }

    #[test]
    fn test_generate_wat_multiplication() {
        let instrs = vec![
            IRInstr::LoadConstInt(4),
            IRInstr::LoadConstInt(5),
            IRInstr::Mul,
            IRInstr::Return,
        ];

        let wat = generate_wat("mul_test", &instrs);
        
        assert!(wat.contains("i32.const 4"));
        assert!(wat.contains("i32.const 5"));
        assert!(wat.contains("i32.mul"));
        assert!(wat.contains("(export \"mul_test\""));
    }

    #[test]
    fn test_generate_wat_comparison() {
        let instrs = vec![
            IRInstr::LoadConstInt(5),
            IRInstr::LoadConstInt(3),
            IRInstr::Lt,
            IRInstr::Return,
        ];

        let wat = generate_wat("cmp_test", &instrs);
        
        assert!(wat.contains("i32.lt_s"));
        assert!(wat.contains("(export \"cmp_test\""));
    }

    #[test]
    fn test_generate_wasm_module() {
        use crate::ir::{IRFunction, IRModule};
        
        let mut module = IRModule::new();
        let mut func = IRFunction::new("test".to_string());
        func.add_instruction(IRInstr::LoadConstInt(42));
        func.add_instruction(IRInstr::Return);
        module.add_function(func);
        
        let wasm = generate_wasm_module(&module);
        
        assert!(wasm.contains("(module"));
        assert!(wasm.contains("(func $test"));
        assert!(wasm.contains("i32.const 42"));
        assert!(wasm.contains("(export \"test\""));
        assert!(wasm.contains(")"));
    }
    
    #[test]
    fn test_stdlib_call() {
        use crate::ir::{IRFunction, IRModule};
        
        let mut module = IRModule::new();
        let mut func = IRFunction::new("main".to_string());
        
        // println(42)
        func.add_instruction(IRInstr::LoadConstInt(42));
        func.add_instruction(IRInstr::CallStd("println".to_string()));
        func.add_instruction(IRInstr::LoadConstInt(0));
        func.add_instruction(IRInstr::Return);
        
        module.add_function(func);
        
        let wasm = generate_wasm_module(&module);
        
        // Should have import for println
        assert!(wasm.contains("(import \"env\" \"println\""));
        assert!(wasm.contains("call $println"));
        assert!(wasm.contains("i32.const 42"));
    }
    
    #[test]
    fn test_multiple_stdlib_calls() {
        use crate::ir::{IRFunction, IRModule};
        
        let mut module = IRModule::new();
        let mut func = IRFunction::new("main".to_string());
        
        // print(10) println(20)
        func.add_instruction(IRInstr::LoadConstInt(10));
        func.add_instruction(IRInstr::CallStd("print".to_string()));
        func.add_instruction(IRInstr::LoadConstInt(20));
        func.add_instruction(IRInstr::CallStd("println".to_string()));
        func.add_instruction(IRInstr::LoadConstInt(0));
        func.add_instruction(IRInstr::Return);
        
        module.add_function(func);
        
        let wasm = generate_wasm_module(&module);
        
        // Should have imports for both print and println
        assert!(wasm.contains("(import \"env\" \"print\""));
        assert!(wasm.contains("(import \"env\" \"println\""));
    }

    #[test]
    fn test_escape_string() {
        let s = "Hello \"world\" with \\ backslash";
        let escaped = escape_string(s);
        
        assert!(escaped.contains("\\\""));
        assert!(escaped.contains("\\\\"));
    }
}
