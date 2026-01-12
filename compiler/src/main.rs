mod lexer;
mod parser;
mod token;
mod ast;
mod types;
mod typechecker;
mod error;
mod diagnostics;
mod ir;
mod lowering;
mod optimize;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use typechecker::TypeChecker;
use diagnostics::display_error;
use lowering::lower;
use optimize::optimize_module;
use codegen::wasm;

fn main() {
    let source = r#"
        fn greet {
        }
    "#;

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(ast) => {
            println!("✅ Parsing successful");
            println!("AST: {:#?}", ast);

            let mut checker = TypeChecker::new();
            match checker.check(&ast) {
                Ok(()) => {
                    println!("✅ Type check passed");
                    
                    // Lowering phase: AST → IR
                    let ir = lower(&ast);
                    println!("\n📊 IR Module (before optimization):");
                    println!("  Functions: {}", ir.functions.len());
                    for func in &ir.functions {
                        println!("  - {} ({} instructions)", func.name, func.instructions.len());
                    }
                    
                    // Optimization phase: Apply optimization passes
                    let optimized_ir = optimize_module(ir);
                    println!("\n🚀 IR Module (after optimization):");
                    println!("  Functions: {}", optimized_ir.functions.len());
                    for func in &optimized_ir.functions {
                        println!("  - {} ({} instructions)", func.name, func.instructions.len());
                    }
                    
                    // Code generation phase: IR → WASM
                    println!("\n🧬 WASM Code Generation:");
                    let wasm_module = wasm::generate_wasm_module(&optimized_ir);
                    println!("  Generated WebAssembly (WAT format):\n");
                    println!("{}", wasm_module);
                }
                Err(errors) => {
                    println!("❌ Type check failed:");
                    for error in errors {
                        println!("  - {}", error);
                    }
                }
            }
        }
        Err(err) => {
            display_error(err);
        }
    }
}
