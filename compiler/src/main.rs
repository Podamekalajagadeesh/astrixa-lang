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
    println!("  astrixa init [project-name]     Initialize a new ASTRIXA project");
    println!("  astrixa install <package>       Install a package from registry");
    println!("  astrixa list                    List installed packages");
    println!("  astrixa run <file.ax> [mode]    Run an ASTRIXA file");
    println!("\nModes:");
    println!("  --vm       Use bytecode VM (default)");
    println!("  --interp   Use interpreter");
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
