mod lexer;
mod parser;
mod token;
mod ast;
mod types;
mod typechecker;
mod error;
mod diagnostics;

use lexer::Lexer;
use parser::Parser;
use typechecker::TypeChecker;
use diagnostics::display_error;

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
                Ok(()) => println!("✅ Type check passed"),
                Err(errors) => {
                    println!("❌ Type check failed:");
                    for error in errors {
                        println!("  - {}", error);
                    }
                }
            }
        }
        Err(err) => {
            println!("❌ Parsing failed:");
            display_error(&err);
        }
    }
}
