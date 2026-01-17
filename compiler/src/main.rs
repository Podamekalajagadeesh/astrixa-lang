use std::env;
use std::fs;

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
mod opt;
mod codegen;
mod stdlib;
mod loader;  // STEP 49: Module loader

use lexer::Lexer;
use parser::Parser;
use typechecker::TypeChecker;
use diagnostics::display_error;
use lowering::lower;
use opt::optimize_module;
use codegen::wasm;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let (source, input_file) = if args.len() > 1 {
        // Read from file if provided
        match fs::read_to_string(&args[1]) {
            Ok(content) => (content, Some(args[1].clone())),
            Err(e) => {
                eprintln!("Error reading file '{}': {}", args[1], e);
                std::process::exit(1);
            }
        }
    } else {
        // Default hardcoded example
        (r#"
        fn greet {
        }
    "#.to_string(), None)
    };

    let lexer = Lexer::new(&source);
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
                    let optimized_ir = optimize_module(&ir);
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
                    
                    // Save output file if input was provided
                    if let Some(input) = input_file {
                        let output = input.replace(".ax", ".wat");
                        match fs::write(&output, &wasm_module) {
                            Ok(_) => {
                                println!("\n💾 WASM saved to: {}", output);
                            }
                            Err(e) => {
                                eprintln!("Error writing output file: {}", e);
                            }
                        }
                    }
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
