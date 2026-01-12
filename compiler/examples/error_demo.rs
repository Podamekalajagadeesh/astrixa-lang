// STEP 36 ERROR DIAGNOSTICS DEMO
// This file demonstrates the human-friendly error system

use std::io::Write;

mod lexer;
mod parser;
mod token;
mod ast;
mod types;
mod error;
mod diagnostics;

use lexer::Lexer;
use parser::Parser;
use diagnostics::display_error;

fn main() {
    println!("🎯 STEP 36: Error Diagnostics Demo");
    println!("====================================\n");

    // Test Case 1: Valid code
    println!("Test 1: Valid Function");
    println!("----------------------");
    test_code(r#"
        fn greet {
        }
    "#);

    println!("\n");

    // Test Case 2: Missing function name
    println!("Test 2: Missing Function Name (Error)");
    println!("---------------------------------------");
    test_code(r#"
        fn {
        }
    "#);

    println!("\n");

    // Test Case 3: Multiple functions (one valid, one invalid)
    println!("Test 3: Mixed Valid/Invalid");
    println!("----------------------------");
    test_code(r#"
        fn hello {
        }
        fn {
        }
    "#);

    println!("\n🎉 Demo Complete!");
    println!("\n✅ Key Achievements:");
    println!("  • Clear error messages");
    println!("  • Precise location info (line/column)");
    println!("  • Helpful suggestions");
    println!("  • No panics or crashes");
    println!("  • Professional developer experience");
}

fn test_code(source: &str) {
    println!("Source:");
    for (i, line) in source.lines().enumerate() {
        println!("  {:2} | {}", i + 1, line);
    }
    println!();

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(ast) => {
            println!("✅ Parsing successful");
            if !ast.is_empty() {
                println!("   Functions parsed: {}", ast.len());
            }
        }
        Err(err) => {
            display_error(err);
        }
    }
}
