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
