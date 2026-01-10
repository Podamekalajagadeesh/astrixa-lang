mod lexer;
mod parser;
mod token;
mod ast;
mod types;
mod typechecker;

use lexer::Lexer;
use parser::Parser;
use typechecker::TypeChecker;
use types::Type;

fn main() {
    let source = r#"
        fn greet {
        }
    "#;

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse();
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
