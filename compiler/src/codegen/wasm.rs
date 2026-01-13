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

/// Memory allocator for strings (simple linear allocator)
pub struct MemoryAllocator {
    offset: usize,
    strings: HashMap<String, (usize, usize)>, // string -> (ptr, len)
}

impl MemoryAllocator {
    fn new() -> Self {
        Self {
            offset: 0,
            strings: HashMap::new(),
        }
    }

    fn allocate_string(&mut self, s: &str) -> (usize, usize) {
        if let Some(&entry) = self.strings.get(s) {
            return entry;
        }

        let ptr = self.offset;
        let len = s.len();
        self.strings.insert(s.to_string(), (ptr, len));
        self.offset += len;
        (ptr, len)
    }

    fn get_data_section(&self) -> String {
        let mut data = String::new();

        for (s, (ptr, _len)) in &self.strings {
            // Escape string for WAT data section
            let escaped = escape_wat_string(s);
            data.push_str(&format!("  (data (i32.const {}) \"{}\")\n", ptr, escaped));
        }

        data
    }
}

/// Generate WASM module from IR
pub fn generate_wasm_module(module: &IRModule) -> String {
    let mut wasm = String::new();
    let mut allocator = MemoryAllocator::new();
    
    // First pass: collect all strings
    for func in &module.functions {
        for instr in &func.instructions {
            if let IRInstr::LoadConstString(s) = instr {
                allocator.allocate_string(s);
            }
        }
    }
    
    // Module header
    wasm.push_str("(module\n");
    
    // Export memory (1 page = 64KB, should be enough for now)
    wasm.push_str("  (memory (export \"memory\") 1)\n\n");
    
    // Add data section if there are strings
    let data_section = allocator.get_data_section();
    if !data_section.is_empty() {
        wasm.push_str(&data_section);
        wasm.push_str("\n");
    }
    
    // Collect all stdlib imports needed
    let imports = collect_stdlib_imports(module);
    
    // Generate imports
    for import in &imports {
        wasm.push_str(&generate_import(import));
    }
    
    if !imports.is_empty() {
        wasm.push_str("\n");
    }
    
    // Generate each function with string allocator
    for func in &module.functions {
        let mut func_allocator = MemoryAllocator::new();
        
        // Collect strings for this function
        for instr in &func.instructions {
            if let IRInstr::LoadConstString(s) = instr {
                func_allocator.allocate_string(s);
            }
        }
        
        wasm.push_str(&generate_function(
            func.name.as_str(),
            &func.instructions,
            &func_allocator,
            func.param_count,  // STEP 46: Pass param count
            func.local_count,
        ));
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
            match instr {
                IRInstr::CallStd(name) => {
                    imports.insert(name.clone());
                }
                IRInstr::CallAI(name) => {
                    // STEP 52: Collect AI function imports
                    imports.insert(name.clone());
                }
                IRInstr::CallWeb3(name) => {
                    // STEP 53: Collect Web3 function imports
                    imports.insert(name.clone());
                }
                IRInstr::Panic => {
                    // STEP 48: Include panic in imports
                    imports.insert("panic".to_string());
                }
                _ => {}
            }
        }
    }
    
    imports
}

/// Generate WASM import declaration for a stdlib function
fn generate_import(func_name: &str) -> String {
    // Handle different stdlib functions with their signatures
    match func_name {
        // Core I/O
        "print" => {
            "  (import \"env\" \"print_str\" (func $print (param i32 i32)))\n".to_string()
        }
        "println" => {
            "  (import \"env\" \"println_str\" (func $println (param i32 i32)))\n".to_string()
        }
        "input" => {
            "  (import \"env\" \"input\" (func $input (result i32)))\n".to_string()
        }
        "len" => {
            "  (import \"env\" \"len\" (func $len (param i32) (result i32)))\n".to_string()
        }
        "exit" => {
            "  (import \"env\" \"exit\" (func $exit (param i32)))\n".to_string()
        }
        "panic" => {
            // STEP 48: Panic import - takes ptr and len for error message
            "  (import \"env\" \"panic\" (func $panic (param i32 i32)))\n".to_string()
        }
        
        // Math functions
        "abs" => {
            "  (import \"env\" \"abs\" (func $abs (param i32) (result i32)))\n".to_string()
        }
        "pow" => {
            "  (import \"env\" \"pow\" (func $pow (param i32 i32) (result i32)))\n".to_string()
        }
        "sqrt" => {
            "  (import \"env\" \"sqrt\" (func $sqrt (param i32) (result i32)))\n".to_string()
        }
        "min" => {
            "  (import \"env\" \"min\" (func $min (param i32 i32) (result i32)))\n".to_string()
        }
        "max" => {
            "  (import \"env\" \"max\" (func $max (param i32 i32) (result i32)))\n".to_string()
        }
        "rand" => {
            "  (import \"env\" \"rand\" (func $rand (param i32) (result i32)))\n".to_string()
        }
        
        // Time functions
        "time" => {
            "  (import \"env\" \"time\" (func $time (result i32)))\n".to_string()
        }
        "sleep" => {
            "  (import \"env\" \"sleep\" (func $sleep (param i32)))\n".to_string()
        }
        
        // Crypto functions
        "hash" => {
            "  (import \"env\" \"hash\" (func $hash (param i32 i32) (result i32)))\n".to_string()
        }
        "keccak" => {
            "  (import \"env\" \"keccak\" (func $keccak (param i32 i32) (result i32)))\n".to_string()
        }
        "sha256" => {
            "  (import \"env\" \"sha256\" (func $sha256 (param i32 i32) (result i32)))\n".to_string()
        }
        
        // STEP 52: AI functions
        "ai.generate" => {
            "  (import \"env\" \"ai_generate\" (func $ai_generate (param i32 i32) (result i32)))\n".to_string()
        }
        "ai.embed" => {
            "  (import \"env\" \"ai_embed\" (func $ai_embed (param i32 i32) (result i32)))\n".to_string()
        }
        "ai.classify" => {
            "  (import \"env\" \"ai_classify\" (func $ai_classify (param i32 i32) (result i32)))\n".to_string()
        }
        
        // STEP 53: Web3 functions
        "web3.wallet" => {
            "  (import \"env\" \"web3_wallet\" (func $web3_wallet (result i32)))\n".to_string()
        }
        "web3.sign" => {
            "  (import \"env\" \"web3_sign\" (func $web3_sign (param i32 i32) (result i32)))\n".to_string()
        }
        "web3.verify" => {
            "  (import \"env\" \"web3_verify\" (func $web3_verify (param i32 i32 i32 i32) (result i32)))\n".to_string()
        }
        "web3.keccak" => {
            "  (import \"env\" \"web3_keccak\" (func $web3_keccak (param i32 i32) (result i32)))\n".to_string()
        }
        "web3.balance" => {
            "  (import \"env\" \"web3_balance\" (func $web3_balance (param i32) (result i32)))\n".to_string()
        }
        "web3.send" => {
            "  (import \"env\" \"web3_send\" (func $web3_send (param i32 i32) (result i32)))\n".to_string()
        }
        
        _ => {
            // Default: function with one i32 parameter
            format!("  (import \"env\" \"{}\" (func ${} (param i32)))\n", func_name, func_name)
        }
    }
}

/// Generate a single function in WASM
pub fn generate_function(
    name: &str,
    instrs: &[IRInstr],
    allocator: &MemoryAllocator,
    param_count: usize,   // STEP 46: Number of parameters
    local_count: usize,
) -> String {
    let mut func_def = String::new();
    
    // STEP 49: Sanitize function names for WASM (replace dots with underscores)
    let wasm_func_name = name.replace('.', "_");
    
    // STEP 46: Function definition with parameters and result
    // Parameters are the first `param_count` locals
    func_def.push_str(&format!("  (func ${}", wasm_func_name));
    
    // Add parameters (all as i32 for V1)
    for _ in 0..param_count {
        func_def.push_str(" (param i32)");
    }
    
    // Add result type
    func_def.push_str(" (result i32)\n");
    
    // STEP 46: Declare non-parameter local variables
    // In WASM, parameters are already declared, so we only need to declare
    // locals that are NOT parameters
    let non_param_locals = if local_count > param_count {
        local_count - param_count
    } else {
        0
    };
    
    if non_param_locals > 0 {
        func_def.push_str("    (local");
        for _ in 0..non_param_locals {
            func_def.push_str(" i32");
        }
        func_def.push_str(")\n");
    }
    
    // Generate function body
    let body = generate_body(instrs, allocator);
    func_def.push_str(&body);
    
    // Function close
    func_def.push_str("  )\n");
    
    // Export function (use original name for export)
    func_def.push_str(&format!("  (export \"{}\" (func ${}))\n", name, wasm_func_name));
    
    func_def
}

/// Generate function body from IR instructions
fn generate_body(instrs: &[IRInstr], allocator: &MemoryAllocator) -> String {
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
                // Load string as (ptr, len) for print_str
                if let Some(&(ptr, len)) = allocator.strings.get(s) {
                    body.push_str(&format!("    i32.const {}  ;; ptr to \"{}\"\n", ptr, escape_string(s)));
                    body.push_str(&format!("    i32.const {}  ;; len\n", len));
                } else {
                    // Fallback: string not in allocator (shouldn't happen)
                    body.push_str(&format!("    i32.const 0  ;; string not found: {}\n", escape_string(s)));
                    body.push_str("    i32.const 0\n");
                }
            }
            
            // Variables
            IRInstr::LoadVar(name) => {
                body.push_str(&format!("    local.get ${}\n", name));
            }
            IRInstr::StoreVar(name) => {
                body.push_str(&format!("    local.set ${}\n", name));
            }
            IRInstr::LoadLocal(slot) => {
                body.push_str(&format!("    local.get {}  ;; load from slot {}\n", slot, slot));
            }
            IRInstr::StoreLocal(slot) => {
                body.push_str(&format!("    local.set {}  ;; store to slot {}\n", slot, slot));
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
                // STEP 49: Sanitize function names for WASM (replace dots with underscores)
                let wasm_func_name = func_name.replace('.', "_");
                body.push_str(&format!("    call ${}\n", wasm_func_name));
            }
            
            // Stdlib calls
            IRInstr::CallStd(func_name) => {
                body.push_str(&format!("    call ${}\n", func_name));
            }
            
            // STEP 52: AI calls
            IRInstr::CallAI(func_name) => {
                // Sanitize AI function names (ai.generate -> ai_generate)
                let wasm_func_name = func_name.replace('.', "_");
                body.push_str(&format!("    call ${}\n", wasm_func_name));
            }
            
            // STEP 53: Web3 calls
            IRInstr::CallWeb3(func_name) => {
                // Sanitize Web3 function names (web3.wallet -> web3_wallet)
                let wasm_func_name = func_name.replace('.', "_");
                body.push_str(&format!("    call ${}\n", wasm_func_name));
            }
            
            // Return
            IRInstr::Return => {
                body.push_str("    return\n");
            }
            
            // STEP 48: Panic
            IRInstr::Panic => {
                // Stack has (ptr, len) from LoadConstString
                // Call the panic function which will abort execution
                body.push_str("    call $panic\n");
                // Panic never returns, but WASM requires unreachable after a call that doesn't return
                body.push_str("    unreachable\n");
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

/// Escape strings for WAT data section
fn escape_wat_string(s: &str) -> String {
    let mut result = String::new();
    
    for byte in s.bytes() {
        match byte {
            b'\"' => result.push_str("\\\""),
            b'\\' => result.push_str("\\\\"),
            b'\n' => result.push_str("\\n"),
            b'\r' => result.push_str("\\r"),
            b'\t' => result.push_str("\\t"),
            32..=126 => result.push(byte as char),
            _ => result.push_str(&format!("\\{:02x}", byte)),
        }
    }
    
    result
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
        assert!(wasm.contains("(memory (export \"memory\")"));
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
        
        // print(42)
        func.add_instruction(IRInstr::LoadConstInt(42));
        func.add_instruction(IRInstr::CallStd("print".to_string()));
        func.add_instruction(IRInstr::LoadConstInt(0));
        func.add_instruction(IRInstr::Return);
        
        module.add_function(func);
        
        let wasm = generate_wasm_module(&module);
        
        // Should have import for print_str (not print anymore)
        assert!(wasm.contains("(import \"env\" \"print_str\""));
        assert!(wasm.contains("call $print"));
        assert!(wasm.contains("(memory (export \"memory\")"));
    }
    
    #[test]
    fn test_string_constant() {
        use crate::ir::{IRFunction, IRModule};
        
        let mut module = IRModule::new();
        let mut func = IRFunction::new("main".to_string());
        
        // print("Hello ASTRIXA")
        func.add_instruction(IRInstr::LoadConstString("Hello ASTRIXA".to_string()));
        func.add_instruction(IRInstr::CallStd("print".to_string()));
        func.add_instruction(IRInstr::LoadConstInt(0));
        func.add_instruction(IRInstr::Return);
        
        module.add_function(func);
        
        let wasm = generate_wasm_module(&module);
        
        // Should have data section with the string
        assert!(wasm.contains("(data"));
        assert!(wasm.contains("Hello ASTRIXA"));
        // Should have memory export
        assert!(wasm.contains("(memory (export \"memory\")"));
        // Should have print_str import
        assert!(wasm.contains("(import \"env\" \"print_str\""));
    }
    
    #[test]
    fn test_multiple_stdlib_calls() {
        use crate::ir::{IRFunction, IRModule};
        
        let mut module = IRModule::new();
        let mut func = IRFunction::new("main".to_string());
        
        // print("Hello") print("World")
        func.add_instruction(IRInstr::LoadConstString("Hello".to_string()));
        func.add_instruction(IRInstr::CallStd("print".to_string()));
        func.add_instruction(IRInstr::LoadConstString("World".to_string()));
        func.add_instruction(IRInstr::CallStd("print".to_string()));
        func.add_instruction(IRInstr::LoadConstInt(0));
        func.add_instruction(IRInstr::Return);
        
        module.add_function(func);
        
        let wasm = generate_wasm_module(&module);
        
        // Should have imports for print_str
        assert!(wasm.contains("(import \"env\" \"print_str\""));
        // Should have data for both strings
        assert!(wasm.contains("Hello"));
        assert!(wasm.contains("World"));
    }

    #[test]
    fn test_escape_string() {
        let s = "Hello \"world\" with \\ backslash";
        let escaped = escape_string(s);
        
        assert!(escaped.contains("\\\""));
        assert!(escaped.contains("\\\\"));
    }
    
    #[test]
    fn test_escape_wat_string() {
        let s = "Hello\nWorld";
        let escaped = escape_wat_string(s);
        
        assert!(escaped.contains("\\n"));
    }
}
