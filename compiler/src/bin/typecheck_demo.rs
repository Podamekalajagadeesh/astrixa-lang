// Minimal TypeChecker demo for assignment mismatch

use astrixa::lexer::Lexer;
use astrixa::parser::Parser;
use astrixa::typechecker::TypeChecker;
use astrixa::diagnostics::display_error;

fn main() {
    let source = r#"
        fn demo {
            let x = 10
            x = "hello"   // should error
        }
    "#;

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(ast) => {
            println!("✅ Parsing successful");
            let mut checker = TypeChecker::new();
            match checker.check(&ast) {
                Ok(()) => {
                    println!("✅ Type check passed (unexpected)");
                }
                Err(errors) => {
                    println!("❌ Type check failed:");
                    for error in errors {
                        println!("  - {}", error);
                    }
                }
            }
        }
        Err(err) => {
            display_error(err);
        }
    }
}
