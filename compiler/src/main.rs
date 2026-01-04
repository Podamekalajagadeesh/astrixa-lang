mod lexer;
mod parser;
mod ast;
mod interpreter;
mod bytecode;
mod compiler;
mod vm;
mod gas;
mod ai_runtime;

use std::env;
use std::fs;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use compiler::Compiler;
use vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: astrixa run <file.ax> [--vm|--interp]");
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let mode = args.get(3).map(|s| s.as_str()).unwrap_or("--vm");

    if command != "run" {
        println!("Unknown command");
        return;
    }

    let source = fs::read_to_string(filename).expect("Failed to read file");

    if let Err(e) = run_astrixa(source, mode) {
        println!("{}", e);
    }
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
