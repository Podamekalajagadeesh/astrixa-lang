// STEP 50: Build Command

use colored::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use crate::config::{Config, find_project_root};

pub fn build_project(release: bool, output: Option<&String>) -> Result<(), String> {
    let root = find_project_root()?;
    let config = Config::load(root.join("astrixa.toml"))?;
    
    let mode = if release { "release" } else { "debug" };
    println!("{} {} {} ({})", 
        "Compiling".green().bold(),
        config.package.name,
        config.package.version.dimmed(),
        mode.cyan()
    );
    
    // Find main source file
    let main_file = root.join("src/main.ax");
    if !main_file.exists() {
        return Err("src/main.ax not found".to_string());
    }
    
    // Create build directory
    let build_dir = root.join("build");
    fs::create_dir_all(&build_dir)
        .map_err(|e| format!("Failed to create build directory: {}", e))?;
    
    // Determine output path
    let output_path = if let Some(out) = output {
        PathBuf::from(out)
    } else {
        build_dir.join(format!("{}.wat", config.package.name))
    };
    
    println!("   {} src/main.ax", "Parsing".cyan());
    
    // Call the ASTRIXA compiler
    let compiler_result = compile_file(&main_file, &output_path, release)?;
    
    println!("   {} {} ({} functions)", 
        "Compiled".green(),
        output_path.display(),
        compiler_result.function_count
    );
    
    if release {
        println!("   {} Applied optimizations", "Optimized".yellow());
    }
    
    println!();
    println!("{} {} in {:.2}s", 
        "Finished".green().bold(),
        mode,
        compiler_result.duration
    );
    
    Ok(())
}

pub fn check_project() -> Result<(), String> {
    let root = find_project_root()?;
    let config = Config::load(root.join("astrixa.toml"))?;
    
    println!("{} {}", "Checking".green().bold(), config.package.name);
    
    let main_file = root.join("src/main.ax");
    if !main_file.exists() {
        return Err("src/main.ax not found".to_string());
    }
    
    // Parse and type-check without building
    check_file(&main_file)?;
    
    println!();
    println!("{}", "✅ No errors found".green().bold());
    
    Ok(())
}

pub fn clean_project() -> Result<(), String> {
    let root = find_project_root()?;
    let build_dir = root.join("build");
    
    if build_dir.exists() {
        println!("{} build artifacts", "Removing".yellow());
        fs::remove_dir_all(&build_dir)
            .map_err(|e| format!("Failed to remove build directory: {}", e))?;
        println!("{}", "✅ Build artifacts removed".green());
    } else {
        println!("{}", "Nothing to clean".dimmed());
    }
    
    Ok(())
}

struct CompileResult {
    function_count: usize,
    duration: f64,
}

fn compile_file(input: &PathBuf, output: &PathBuf, optimize: bool) -> Result<CompileResult, String> {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Read source file
    let source = fs::read_to_string(input)
        .map_err(|e| format!("Failed to read source file: {}", e))?;
    
    // Parse
    let lexer = astrixa::lexer::Lexer::new(&source);
    let mut parser = astrixa::parser::Parser::new(lexer);
    let ast = parser.parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;
    
    // Type check
    let mut checker = astrixa::typechecker::TypeChecker::new();
    checker.check(&ast)
        .map_err(|e| format!("Type error: {:?}", e))?;
    
    // Lower to IR
    let mut ir = astrixa::lowering::lower(&ast);
    
    // Optimize if in release mode
    if optimize {
        ir = astrixa::opt::optimize_module(&ir);
    }
    
    let function_count = ir.functions.len();
    
    // Generate WASM
    let wasm = astrixa::codegen::wasm::generate_wasm_module(&ir);
    
    // Write output
    fs::write(output, wasm)
        .map_err(|e| format!("Failed to write output file: {}", e))?;
    
    let duration = start.elapsed().as_secs_f64();
    
    Ok(CompileResult {
        function_count,
        duration,
    })
}

fn check_file(input: &PathBuf) -> Result<(), String> {
    // Read source file
    let source = fs::read_to_string(input)
        .map_err(|e| format!("Failed to read source file: {}", e))?;
    
    // Parse
    let lexer = astrixa::lexer::Lexer::new(&source);
    let mut parser = astrixa::parser::Parser::new(lexer);
    let ast = parser.parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;
    
    // Type check
    let mut checker = astrixa::typechecker::TypeChecker::new();
    checker.check(&ast)
        .map_err(|e| format!("Type error: {:?}", e))?;
    
    println!("   {} Syntax and types", "Checked".green());
    
    Ok(())
}
