use wasm_bindgen::prelude::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::vm::{VM, Value as VMValue};
use crate::compiler::Compiler;
use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator for smaller WASM binary size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Initialize panic hook for better error messages in browser console
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Run Astrixa source code in interpreter mode
/// Returns the result as a JSON string or error message
#[wasm_bindgen]
pub fn run_astrixa(source: &str) -> Result<String, String> {
    // Lexical analysis
    let mut lexer = Lexer::new(source.to_string());
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => return Err(format!("Lexer error: {}", e)),
    };

    // Parsing
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => return Err(format!("Parser error: {:?}", e)),
    };

    // Interpretation
    let mut interpreter = Interpreter::new();
    match interpreter.interpret(program) {
        Ok(_) => Ok("Execution completed successfully".to_string()),
        Err(e) => Err(format!("Runtime error: {}", e)),
    }
}

/// Run Astrixa source code in VM (bytecode) mode
/// Returns the result as a JSON string or error message
#[wasm_bindgen]
pub fn run_astrixa_vm(source: &str) -> Result<String, String> {
    // Lexical analysis
    let mut lexer = Lexer::new(source.to_string());
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => return Err(format!("Lexer error: {}", e)),
    };

    // Parsing
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => return Err(format!("Parser error: {:?}", e)),
    };

    // Compilation to bytecode
    let mut compiler = Compiler::new();
    let bytecode = match compiler.compile(program) {
        Ok(bc) => bc,
        Err(e) => return Err(format!("Compiler error: {}", e)),
    };

    // VM execution
    let mut vm = VM::new(1_000_000); // 1M gas limit for browser safety
    match vm.execute(&bytecode) {
        Ok(result) => Ok(vm_value_to_string(&result)),
        Err(e) => Err(format!("VM error: {}", e)),
    }
}

/// Compile Astrixa source to bytecode (returns JSON representation)
#[wasm_bindgen]
pub fn compile_astrixa(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => return Err(format!("Lexer error: {}", e)),
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => return Err(format!("Parser error: {:?}", e)),
    };

    let mut compiler = Compiler::new();
    let bytecode = match compiler.compile(program) {
        Ok(bc) => bc,
        Err(e) => return Err(format!("Compiler error: {}", e)),
    };

    Ok(format!("{:?}", bytecode))
}

/// Validate Astrixa source code syntax
#[wasm_bindgen]
pub fn validate_astrixa(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => return Err(format!("Lexer error: {}", e)),
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(_) => Ok("Syntax valid".to_string()),
        Err(e) => Err(format!("Parser error: {:?}", e)),
    }
}

/// Helper function to convert VM values to string representation
fn vm_value_to_string(value: &VMValue) -> String {
    match value {
        VMValue::Number(n) => n.to_string(),
        VMValue::Bool(b) => b.to_string(),
        VMValue::String(s) => s.clone(),
        VMValue::Array(arr) => {
            let items: Vec<String> = arr.iter().map(vm_value_to_string).collect();
            format!("[{}]", items.join(", "))
        }
        VMValue::Object(map) => {
            let items: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("{}: {}", k, vm_value_to_string(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
        VMValue::Null => "null".to_string(),
        VMValue::Function(_) => "<function>".to_string(),
    }
}

/// Get version information
#[wasm_bindgen]
pub fn get_version() -> String {
    "Astrixa WASM v0.1.0".to_string()
}

/// Check if WASM environment is ready
#[wasm_bindgen]
pub fn is_ready() -> bool {
    true
}
