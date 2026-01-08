mod lexer;
mod parser;
mod token;
mod ast;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let source = r#"
        fn greet {
        }
    "#;

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse();
    println!("{:#?}", ast);
}
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
