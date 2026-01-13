// STEP 49: ASTRIXA Compiler Library
// Exports all compiler modules for use in binaries and tests

pub mod lexer;
pub mod parser;
pub mod token;
pub mod ast;
pub mod types;
pub mod typechecker;
pub mod error;
pub mod diagnostics;
pub mod ir;
pub mod lowering;
pub mod opt;
pub mod codegen {
    pub mod wasm;
}
pub mod stdlib;
pub mod loader;
pub mod compiler;
pub mod interpreter;
pub mod vm;
pub mod bytecode;
pub mod ai_runtime;
pub mod gas;
pub mod package_manager;
pub mod wasm;
