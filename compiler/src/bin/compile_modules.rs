// STEP 49: Module compilation entry point
// This tool compiles ASTRIXA programs with module support

use std::env;
use std::fs;
use std::path::PathBuf;

use astrixa::loader::ModuleLoader;
use astrixa::lexer::Lexer;
use astrixa::parser::Parser;
use astrixa::lowering::lower;
use astrixa::opt::optimize_module;
use astrixa::codegen::wasm::generate_wasm_module;
use astrixa::diagnostics::display_error;
use astrixa::ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <input.ax> [output.wat]", args[0]);
        eprintln!("\nCompiles ASTRIXA source with module support");
        std::process::exit(1);
    }
    
    let input_file = &args[1];
    let output_file = if args.len() >= 3 {
        args[2].clone()
    } else {
        // Replace .ax with .wat
        let path = PathBuf::from(input_file);
        path.with_extension("wat").to_string_lossy().to_string()
    };
    
    println!("🚀 ASTRIXA Compiler (STEP 49 - Module System)");
    println!("📄 Input: {}", input_file);
    println!("📦 Output: {}", output_file);
    println!();
    
    match compile_with_modules(input_file, &output_file) {
        Ok(()) => {
            println!("✅ Compilation successful!");
            println!("💾 WASM module written to: {}", output_file);
        }
        Err(e) => {
            eprintln!("❌ Compilation failed:");
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn compile_with_modules(input_file: &str, output_file: &str) -> Result<(), String> {
    // Initialize module loader
    let mut loader = ModuleLoader::new();
    
    // Add current directory to search paths
    let input_path = PathBuf::from(input_file);
    if let Some(parent) = input_path.parent() {
        loader.add_search_path(parent.to_path_buf());
    }
    
    // Load the main module
    let source = fs::read_to_string(input_file)
        .map_err(|e| format!("Failed to read input file: {}", e))?;
    
    println!("📖 Parsing main module...");
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    
    let main_ast = parser.parse()
        .map_err(|e| {
            display_error(e);
            "Parse error".to_string()
        })?;
    
    println!("✅ Main module parsed ({} top-level items)", main_ast.len());
    
    // Extract imports and load imported modules
    let mut all_modules = vec![main_ast.clone()];
    let mut imported_modules = Vec::new();
    
    for stmt in &main_ast {
        if let ast::Stmt::Import(module_name) = stmt {
            println!("📦 Loading module: {}", module_name);
            
            let module = loader.load_module(module_name)
                .map_err(|e| {
                    display_error(e.clone());
                    format!("Failed to load module '{}'", module_name)
                })?;
            
            println!("  ✅ Module '{}' loaded ({} items)", module_name, module.statements.len());
            
            // Store imported module's statements
            imported_modules.push(module.statements.clone());
        }
    }
    
    // Combine all modules into one compilation unit
    // First, add all imported modules (so their functions are defined first)
    for module_stmts in &imported_modules {
        all_modules.push(module_stmts.clone());
    }
    
    // Flatten all statements from all modules
    let mut combined_ast = Vec::new();
    for module_stmts in all_modules {
        for stmt in module_stmts {
            // Skip import statements (already processed)
            if !matches!(stmt, ast::Stmt::Import(_)) {
                combined_ast.push(stmt);
            }
        }
    }
    
    println!();
    println!("🔄 Lowering to IR...");
    let ir_module = lower(&combined_ast);
    println!("  ✅ {} functions in IR", ir_module.functions.len());
    for func in &ir_module.functions {
        println!("    - {} ({} instructions)", func.name, func.instructions.len());
    }
    
    println!();
    println!("⚡ Optimizing IR...");
    let optimized = optimize_module(&ir_module);
    
    // Count optimizations
    let original_instrs: usize = ir_module.functions.iter().map(|f| f.instructions.len()).sum();
    let optimized_instrs: usize = optimized.functions.iter().map(|f| f.instructions.len()).sum();
    
    if original_instrs > optimized_instrs {
        println!("  ✅ Removed {} instructions", original_instrs - optimized_instrs);
    } else {
        println!("  ✅ No optimizations applied");
    }
    
    println!();
    println!("🎯 Generating WASM...");
    let wasm_code = generate_wasm_module(&optimized);
    
    fs::write(output_file, wasm_code)
        .map_err(|e| format!("Failed to write output file: {}", e))?;
    
    Ok(())
}
