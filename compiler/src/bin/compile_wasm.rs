/// ASTRIXA WASM Compiler - Compiles Astrixa source to WebAssembly

use std::env;
use std::fs;
use std::process;

// We'll include these modules to build a standalone compiler
use astrixa::lexer::Lexer;
use astrixa::parser::Parser;
use astrixa::typechecker::TypeChecker;
use astrixa::lowering::lower;
use astrixa::opt::optimize_module;
use astrixa::codegen::wasm;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <source_file> [--output output.wat]", args[0]);
        process::exit(1);
    }

    let source_file = &args[1];
    let mut output_file = source_file.replace(".ax", ".wat");

    // Parse --output flag
    for i in 2..args.len() {
        if args[i] == "--output" && i + 1 < args.len() {
            output_file = args[i + 1].clone();
        }
    }

    // Read source file
    let source = match fs::read_to_string(source_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file '{}': {}", source_file, e);
            process::exit(1);
        }
    };

    // Parsing (which includes lexing)
    println!("📝 Lexing and parsing...");
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let ast = match parser.parse() {
        Ok(a) => {
            println!("✅ Parsing successful");
            a
        }
        Err(e) => {
            eprintln!("❌ Parser error: {:?}", e);
            process::exit(1);
        }
    };

    // Type checking
    println!("🔎 Type checking...");
    let mut checker = TypeChecker::new();
    match checker.check(&ast) {
        Ok(()) => {
            println!("✅ Type check passed");
        }
        Err(errors) => {
            eprintln!("❌ Type check failed:");
            for error in errors {
                eprintln!("  - {}", error);
            }
            process::exit(1);
        }
    }

    // Lowering (AST → IR)
    println!("📊 Lowering AST to IR...");
    let ir = lower(&ast);
    println!("✅ Lowering successful ({} functions)", ir.functions.len());
    for func in &ir.functions {
        println!("  - {} ({} instructions, {} params, {} locals)", 
                 func.name, func.instructions.len(), func.param_count, func.local_count);
    }

    // Optimization
    println!("🚀 Optimizing IR...");
    let optimized_ir = optimize_module(&ir);
    println!("✅ Optimization complete");
    for func in &optimized_ir.functions {
        println!("  - {} ({} instructions)", func.name, func.instructions.len());
    }

    // Code generation (IR → WASM)
    println!("🧬 Generating WASM...");
    let wasm_module = wasm::generate_wasm_module(&optimized_ir);
    println!("✅ WASM generation successful");

    // Write output file
    match fs::write(&output_file, &wasm_module) {
        Ok(_) => {
            println!("💾 Written to: {}", output_file);
            println!("\n🎉 Compilation successful!");
        }
        Err(e) => {
            eprintln!("❌ Error writing output file '{}': {}", output_file, e);
            process::exit(1);
        }
    }

    // Print module info
    println!("\n📋 Module Info:");
    println!("  Size: {} bytes", wasm_module.len());
    println!("  Functions: {}", optimized_ir.functions.len());
    for func in &optimized_ir.functions {
        println!("    - {}: {} instructions", func.name, func.instructions.len());
    }
}
