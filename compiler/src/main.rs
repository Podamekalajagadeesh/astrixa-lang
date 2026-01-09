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
