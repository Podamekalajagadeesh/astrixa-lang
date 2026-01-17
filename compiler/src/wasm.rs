use wasm_bindgen::prelude::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::{Interpreter, Value};
use crate::vm::VM;
use crate::compiler::Compiler;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Run Astrixa source code in interpreter mode
#[wasm_bindgen]
pub fn run_astrixa(source: &str) -> Result<String, String> {
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

    let mut interpreter = Interpreter::new();
    match interpreter.interpret(program) {
        Ok(_) => Ok("Execution completed successfully".to_string()),
        Err(e) => Err(format!("Runtime error: {}", e)),
    }
}

/// Run Astrixa source code in VM (bytecode) mode
#[wasm_bindgen]
pub fn run_astrixa_vm(source: &str) -> Result<String, String> {
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

    let mut vm = VM::new(1_000_000);
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
fn vm_value_to_string(value: &Value) -> String {
    match value {
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(vm_value_to_string).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Address(addr) => addr.clone(),
        Value::U256(num) => num.to_string(),
        Value::AIResult { label, score } => {
            format!("AIResult {{ label: {}, score: {} }}", label, score)
        }
        Value::Null => "null".to_string(),
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
