mod lexer;
mod parser;
mod ast;
mod interpreter;
mod bytecode;
mod compiler;
mod vm;
mod gas;
mod ai_runtime;
mod package_manager;

// WASM module (only compiled for wasm32 target)
#[cfg(target_arch = "wasm32")]
mod wasm;

use std::env;
use std::fs;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use compiler::Compiler;
use vm::VM;
use package_manager::PackageManager;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "run" => {
            if args.len() < 3 {
                println!("Usage: astrixa run <file.ax> [--vm|--interp]");
                return;
            }
            let filename = &args[2];
            let mode = args.get(3).map(|s| s.as_str()).unwrap_or("--vm");
            
            let source = fs::read_to_string(filename).expect("Failed to read file");
            
            if let Err(e) = run_astrixa(source, mode) {
                println!("{}", e);
            }
        }
        "build" => {
            if args.len() < 3 {
                println!("Usage: astrixa build <file.ax> [--target=<target>] [--output=<file>]");
                println!("Targets: contract, wasm, web, native (default)");
                return;
            }
            let filename = &args[2];
            let mut target = "native";
            let mut output = None;

            // Parse flags
            for arg in args.iter().skip(3) {
                if arg.starts_with("--target=") {
                    target = arg.strip_prefix("--target=").unwrap();
                } else if arg.starts_with("--output=") {
                    output = Some(arg.strip_prefix("--output=").unwrap());
                }
            }

            let source = fs::read_to_string(filename).expect("Failed to read file");
            
            if let Err(e) = build_astrixa(source, target, output) {
                eprintln!("Build failed: {}", e);
                std::process::exit(1);
            }
        }
        "init" => {
            let project_name = args.get(2).map(|s| s.as_str()).unwrap_or("my-project");
            if let Err(e) = PackageManager::init(project_name) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        "install" => {
            if args.len() < 3 {
                println!("Usage: astrixa install <package>");
                return;
            }
            let package_name = &args[2];
            let version = args.get(3).map(|s| s.as_str());
            
            match PackageManager::new() {
                Ok(pm) => {
                    if let Err(e) = pm.install(package_name, version) {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "list" => {
            match PackageManager::new() {
                Ok(pm) => {
                    if let Err(e) = pm.list() {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("ASTRIXA - AI-Native Web3 Language");
    println!("\nUsage:");
    println!("  astrixa init [project-name]        Initialize a new ASTRIXA project");
    println!("  astrixa install <package>          Install a package from registry");
    println!("  astrixa list                       List installed packages");
    println!("  astrixa run <file.ax> [mode]       Run an ASTRIXA file");
    println!("  astrixa build <file.ax> [options]  Build for different targets");
    println!("\nRun Modes:");
    println!("  --vm       Use bytecode VM (default)");
    println!("  --interp   Use interpreter");
    println!("\nBuild Targets:");
    println!("  --target=native     Native executable (default)");
    println!("  --target=contract   Smart contract bytecode (EVM)");
    println!("  --target=wasm       WebAssembly module");
    println!("  --target=web        Web server binary");
    println!("\nBuild Options:");
    println!("  --output=<file>     Output file path");
    println!("\nExamples:");
    println!("  astrixa build token.ax --target=contract --output=token.bin");
    println!("  astrixa build api.ax --target=web");
    println!("  astrixa build ui.ax --target=wasm");
}

fn run_astrixa(source: String, mode: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    match mode {
        "--interp" => {
            // Use interpreter
            let mut interpreter = Interpreter::new();
            interpreter.run(ast)
        }
        "--vm" | _ => {
            // Use bytecode VM
            let mut compiler = Compiler::new();
            let bytecode = compiler.compile(ast)?;

            let mut vm = VM::new();
            vm.run(bytecode)?;
            Ok(())
        }
    }
}

fn build_astrixa(source: String, target: &str, output: Option<&str>) -> Result<(), String> {
    println!("Building ASTRIXA program...");
    println!("Target: {}", target);
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    match target {
        "contract" => {
            // Compile to smart contract bytecode
            println!("Validating contract restrictions...");
            validate_contract_ast(&ast)?;
            
            let mut compiler = Compiler::new();
            compiler.set_mode("contract");
            let bytecode = compiler.compile(ast)?;
            
            let output_file = output.unwrap_or("output.contract");
            fs::write(output_file, bytecode).map_err(|e| e.to_string())?;
            
            println!("✅ Contract compiled successfully: {}", output_file);
            println!("   Deploy with: astrixa deploy {}", output_file);
        }
        "wasm" => {
            // Compile to WebAssembly
            println!("Compiling to WebAssembly...");
            
            let mut compiler = Compiler::new();
            compiler.set_mode("wasm");
            let bytecode = compiler.compile(ast)?;
            
            let output_file = output.unwrap_or("output.wasm");
            fs::write(output_file, bytecode).map_err(|e| e.to_string())?;
            
            println!("✅ WASM module compiled: {}", output_file);
            println!("   Use in browser or Node.js");
        }
        "web" => {
            // Compile web server
            println!("Building web server...");
            
            let mut compiler = Compiler::new();
            compiler.set_mode("web");
            let bytecode = compiler.compile(ast)?;
            
            let output_file = output.unwrap_or("output.web");
            fs::write(output_file, bytecode).map_err(|e| e.to_string())?;
            
            println!("✅ Web server compiled: {}", output_file);
            println!("   Run with: ./{}", output_file);
        }
        "native" | _ => {
            // Default: compile to native executable
            println!("Building native executable...");
            
            let mut compiler = Compiler::new();
            let bytecode = compiler.compile(ast)?;
            
            let output_file = output.unwrap_or("output");
            fs::write(output_file, bytecode).map_err(|e| e.to_string())?;
            
            println!("✅ Built successfully: {}", output_file);
        }
    }
    
    Ok(())
}

fn validate_contract_ast(ast: &[ast::Statement]) -> Result<(), String> {
    // Validate smart contract restrictions
    // This ensures deterministic, secure contract execution
    
    for stmt in ast {
        validate_contract_statement(stmt)?;
    }
    
    Ok(())
}

fn validate_contract_statement(stmt: &ast::Statement) -> Result<(), String> {
    use ast::Statement;
    
    match stmt {
        // Check for forbidden operations
        Statement::Expression(expr) => validate_contract_expression(expr)?,
        Statement::Let { value, .. } => validate_contract_expression(value)?,
        Statement::If { condition, then_block, else_block } => {
            validate_contract_expression(condition)?;
            for s in then_block {
                validate_contract_statement(s)?;
            }
            if let Some(else_blk) = else_block {
                for s in else_blk {
                    validate_contract_statement(s)?;
                }
            }
        }
        Statement::Function { body, .. } => {
            for s in body {
                validate_contract_statement(s)?;
            }
        }
        _ => {}
    }
    
    Ok(())
}

fn validate_contract_expression(expr: &ast::Expression) -> Result<(), String> {
    use ast::Expression;
    
    match expr {
        Expression::FunctionCall { name, args } => {
            // Check for forbidden function calls
            let forbidden = vec![
                "read", "write", "open", "close",  // File I/O
                "http_get", "http_post", "fetch",  // Network
                "random", "rand",                   // Non-deterministic
                "sleep", "delay",                   // Timing
                "malloc", "alloc",                  // Dynamic allocation
            ];
            
            if forbidden.contains(&name.as_str()) {
                return Err(format!(
                    "❌ Contract validation failed: '{}' is not allowed in smart contracts\n   \
                     Smart contracts must be deterministic and cannot use:\n   \
                     • File system operations\n   \
                     • Network requests\n   \
                     • Random number generation\n   \
                     • Dynamic memory allocation\n   \
                     Use contract-safe alternatives instead.",
                    name
                ));
            }
            
            // Recursively check arguments
            for arg in args {
                validate_contract_expression(arg)?;
            }
        }
        Expression::Binary { left, right, .. } => {
            validate_contract_expression(left)?;
            validate_contract_expression(right)?;
        }
        _ => {}
    }
    
    Ok(())
}
