# ASTRIXA Compiler Test Guide - STEP 34

## Quick Start

```bash
cd compiler
cargo run
```

## Expected Output

When you run the compiler, you should see the Abstract Syntax Tree (AST) for the test program:

```
[
    Function {
        name: "greet",
        body: [],
    }
]
```

## What This Proves

### ✅ Lexer Works
The source code `fn greet {` was successfully tokenized into:
- `Token::Fn` (keyword)
- `Token::Identifier("greet")` (name)
- `Token::LBrace` (opening brace)
- `Token::RBrace` (closing brace)

### ✅ Parser Works
The tokens were successfully parsed into an AST representing:
- A function named "greet"
- With an empty body

### ✅ AST Works
The AST was formatted and printed with `println!("{:#?}", ast)`

## How to Test New Features

### 1. Test Multiple Functions
Replace the source in `main.rs`:
```rust
let source = r#"
    fn greet { }
    fn add { }
    fn multiply { }
"#;
```

Expected output: 3 functions in the AST

### 2. Test Different Keywords
```rust
let source = r#"
    fn calculate {
    }
    let x = 5
"#;
```

### 3. Test Identifiers and Numbers
```rust
let source = r#"fn fibonacci { }"#;
```

## Architecture Verification

Each component can be tested independently:

### Test Lexer Only
```rust
use crate::lexer::Lexer;

fn test_lexer() {
    let mut lexer = Lexer::new("fn greet");
    let tok1 = lexer.next_token();  // Token::Fn
    let tok2 = lexer.next_token();  // Token::Identifier("greet")
    let tok3 = lexer.next_token();  // Token::EOF
}
```

### Test Parser Only
```rust
use crate::lexer::Lexer;
use crate::parser::Parser;

fn test_parser() {
    let lexer = Lexer::new("fn greet { }");
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    assert_eq!(ast.len(), 1);
}
```

## Pipeline Visualization

```
Input: "fn greet { }"
    ↓
Lexer (src/lexer.rs)
    ↓
Tokens: [Fn, Identifier("greet"), LBrace, RBrace, EOF]
    ↓
Parser (src/parser.rs)
    ↓
AST: [Function { name: "greet", body: [] }]
    ↓
Output: Printed with println!("{:#?}", ast)
```

## Debugging Tips

### If it doesn't compile:
1. Check `main.rs` mod declarations match file names
2. Verify all imports are correct
3. Run `cargo check` for detailed errors

### If it panics:
1. The parser will panic on syntax errors
2. Check the source code in `main.rs` for invalid syntax
3. Add more error handling in `parser.rs` if needed

### If output looks wrong:
1. Check `#[derive(Debug)]` is on AST types
2. Verify `println!("{:#?}", ast)` is using pretty-print
3. Check parser is building the correct AST structure

## Next Steps After Verification

Once you can run and see the AST output, you're ready for:

1. **Type Checking Phase**
   - Validate variable types
   - Check function signatures
   - Create a symbol table

2. **Codegen Phase**
   - Convert AST to bytecode
   - Generate intermediate representation (IR)
   - Optimize code

3. **Runtime Phase**
   - Build a VM to execute bytecode
   - Implement built-in functions
   - Add memory management

## Success Criteria

You've completed STEP 34 when:

✅ `cargo run` compiles without errors  
✅ Output shows parsed AST  
✅ AST matches expected structure  
✅ You understand how data flows through: Source → Tokens → AST  

🎉 Congratulations! You have a working compiler foundation!
