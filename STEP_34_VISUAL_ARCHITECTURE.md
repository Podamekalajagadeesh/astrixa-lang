# STEP 34 - Visual Architecture & Data Flow

## 🏗️ Compiler Architecture

### High-Level Pipeline

```
┌─────────────────────────────────────────────────────────────┐
│                    ASTRIXA COMPILER                         │
└─────────────────────────────────────────────────────────────┘

Step 1: Source Code (input)
┌─────────────────────────────────────────────────────────────┐
│                                                               │
│  fn greet {                                                   │
│  }                                                            │
│                                                               │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
Step 2: Lexer (text → tokens)
┌─────────────────────────────────────────────────────────────┐
│  Lexer::new() → next_token() → Token                         │
│                                                               │
│  Input:  "fn greet {}"                                       │
│  Output: [Fn, Identifier("greet"), LBrace, RBrace, EOF]     │
│                                                               │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
Step 3: Parser (tokens → AST)
┌─────────────────────────────────────────────────────────────┐
│  Parser::new() → parse() → Vec<Stmt>                        │
│                                                               │
│  Input:  Tokens stream                                       │
│  Output: [Function { name: "greet", body: [] }]             │
│                                                               │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
Step 4: AST Visualization (output)
┌─────────────────────────────────────────────────────────────┐
│                                                               │
│  [                                                            │
│      Function {                                               │
│          name: "greet",                                       │
│          body: [],                                            │
│      }                                                        │
│  ]                                                            │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## 📦 Component Relationships

### Direct Dependencies

```
main.rs
  ├─> uses: Lexer, Parser
  ├─> produces: AST output
  │
├─> Lexer (lexer.rs)
│   ├─> uses: Token enum
│   └─> produces: Token stream
│
├─> Parser (parser.rs)
│   ├─> uses: Lexer, Token, Stmt
│   └─> produces: Vec<Stmt>
│
├─> Token (token.rs)
│   └─> defines: All token types
│
└─> AST (ast.rs)
    ├─> defines: Expr enum
    └─> defines: Stmt enum
```

---

## 🔄 Data Transformations

### Transformation 1: Lexing

```
Input String
"fn greet {"

    ↓ (character by character)

Lexer
• Check if whitespace → skip
• Check if '(' or ')' or '{' or '}' → single token
• Check if alphabetic → read_identifier()
  • Accumulate alphanumeric chars
  • Check if matches keyword → keyword token
  • Otherwise → Identifier token

    ↓

Output Tokens
[
  Token::Fn,
  Token::Identifier("greet"),
  Token::LBrace,
  Token::RBrace,
  Token::EOF
]
```

### Transformation 2: Parsing

```
Input Token Stream
[Fn, Identifier("greet"), LBrace, RBrace, EOF]

    ↓ (token by token)

Parser
Loop: while current != EOF
  if current == Fn:
    parse_function()
      • advance() - consume Fn
      • extract name from Identifier
      • advance() - consume name
      • create Stmt::Function
  else:
    advance()

    ↓

Output AST
[
  Function {
    name: "greet",
    body: []
  }
]
```

### Transformation 3: Visualization

```
Input AST
[Function { name: "greet", body: [] }]

    ↓

Format
println!("{:#?}", ast)
Uses Debug derive macro
Pretty-prints with indentation

    ↓

Output
[
    Function {
        name: "greet",
        body: [],
    }
]
```

---

## 🗂️ Module Structure

```
compiler/
│
├── src/
│   │
│   ├── main.rs ✅
│   │   ├── mod lexer
│   │   ├── mod parser
│   │   ├── mod token
│   │   ├── mod ast
│   │   │
│   │   └── fn main()
│   │       ├── Create Lexer
│   │       ├── Create Parser
│   │       ├── Call parse()
│   │       └── Print AST
│   │
│   ├── token.rs ✅
│   │   └── pub enum Token
│   │       ├── Keywords: Fn, Let, Return
│   │       ├── Literals: Identifier, Number, String
│   │       ├── Symbols: LParen, RParen, etc.
│   │       ├── Operators: Plus, Minus, etc.
│   │       └── Special: EOF
│   │
│   ├── lexer.rs ✅
│   │   ├── pub struct Lexer
│   │   ├── pub fn new()
│   │   ├── pub fn next_token()
│   │   ├── fn simple()
│   │   ├── fn skip_whitespace()
│   │   └── fn read_identifier()
│   │
│   ├── parser.rs ✅
│   │   ├── pub struct Parser
│   │   ├── pub fn new()
│   │   ├── pub fn parse()
│   │   ├── fn advance()
│   │   └── fn parse_function()
│   │
│   └── ast.rs ✅
│       ├── pub enum Expr
│       │   ├── Number(i64)
│       │   └── Identifier(String)
│       │
│       └── pub enum Stmt
│           └── Function { name, body }
│
└── Cargo.toml ✅
    ├── name = "astrixa"
    └── dependencies = [...]
```

---

## 🔀 Control Flow

### Main Execution Flow

```
                    ┌─────────────┐
                    │   START     │
                    └──────┬──────┘
                           │
                           ▼
            ┌──────────────────────────────┐
            │ Define source code string    │
            │ r#"fn greet { }"#            │
            └──────────────┬───────────────┘
                           │
                           ▼
            ┌──────────────────────────────┐
            │ Create Lexer                 │
            │ Lexer::new(source)           │
            └──────────────┬───────────────┘
                           │
                           ▼
            ┌──────────────────────────────┐
            │ Create Parser                │
            │ Parser::new(lexer)           │
            │ • read first token           │
            └──────────────┬───────────────┘
                           │
                           ▼
            ┌──────────────────────────────┐
            │ Parse Program                │
            │ parser.parse()               │
            │ • loop through tokens        │
            │ • build AST                  │
            │ • return Vec<Stmt>          │
            └──────────────┬───────────────┘
                           │
                           ▼
            ┌──────────────────────────────┐
            │ Format and Print             │
            │ println!("{:#?}", ast)       │
            └──────────────┬───────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │     END     │
                    └─────────────┘
```

### Lexer State Machine

```
                    ┌──────────┐
                    │   START  │
                    └────┬─────┘
                         │
           ┌─────────────┘└─────────────┐
           │                             │
           ▼                             ▼
    ┌──────────────┐         ┌──────────────────┐
    │ Skip space   │         │ Is EOF?          │
    │ Advance pos  │         │ Return EOF token │
    └──────┬───────┘         └──────────────────┘
           │
           ▼
    ┌──────────────────────┐
    │ Match current char   │
    └──────┬───────────────┘
           │
    ┌──────┴──────┬──────────┬──────────┐
    │             │          │          │
    ▼             ▼          ▼          ▼
  ( ) { }     + - * /      :  ,      Other
   │  │  │ │    │ │ │ │    │  │       │
   ▼  ▼  ▼ ▼    ▼ ▼ ▼ ▼    ▼  ▼       ▼
  LParen RParen Plus Minus Colon Comma read_identifier()
  RBrace RBrace Star Slash         │
                                    ▼
                              Collect alphanumeric
                                    │
                              Check if keyword
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
                   "fn"           "let"          other
                    │               │               │
                    ▼               ▼               ▼
                  Token::Fn    Token::Let    Token::Identifier
```

---

## 📊 Data Structure Diagram

### Token Enum

```
┌─────────────────────────────────────────────┐
│             Token Enum                      │
├─────────────────────────────────────────────┤
│                                             │
│  Keywords (Unit variants)                   │
│  ├─ Fn                                      │
│  ├─ Let                                     │
│  └─ Return                                  │
│                                             │
│  Data variants (with values)                │
│  ├─ Identifier(String)                      │
│  ├─ Number(i64)                             │
│  └─ String(String)                          │
│                                             │
│  Punctuation (Unit variants)                │
│  ├─ LParen, RParen                          │
│  ├─ LBrace, RBrace                          │
│  ├─ Colon, Comma, Arrow                     │
│                                             │
│  Operators (Unit variants)                  │
│  ├─ Plus, Minus                             │
│  ├─ Star, Slash                             │
│  └─ Equal                                   │
│                                             │
│  Special                                    │
│  └─ EOF                                     │
│                                             │
└─────────────────────────────────────────────┘
```

### AST Structure

```
┌─────────────────────────────────────────────┐
│             Stmt Enum (Statement)           │
├─────────────────────────────────────────────┤
│                                             │
│  Function {                                 │
│    name: String,                            │
│    body: Vec<Stmt>                          │
│  }                                          │
│                                             │
│  (more variants to come in future steps)   │
│                                             │
└─────────────────────────────────────────────┘

┌─────────────────────────────────────────────┐
│             Expr Enum (Expression)          │
├─────────────────────────────────────────────┤
│                                             │
│  Number(i64)                                │
│  Identifier(String)                         │
│                                             │
│  (more variants to come in future steps)   │
│                                             │
└─────────────────────────────────────────────┘
```

---

## 🔀 Lexer State Transitions

```
Initial: position = 0, input = ['f', 'n', ' ', 'g', 'r', 'e', 'e', 't', ...]

next_token():
  │
  ├─> skip_whitespace()
  │   position = 0 (not whitespace, no change)
  │
  ├─> ch = input[0] = 'f'
  │
  └─> Match 'f':
      matches "_" branch
      → read_identifier()
         position_start = 0
         Loop: while position < len && is_alphanumeric
           position becomes 1, 2 (accumulate "fn")
           position = 2, input[2] = ' ' (not alphanumeric, stop)
         text = "fn"
         Match "fn":
           → return Token::Fn, position = 2

Next: position = 2, input = [' ', 'g', 'r', ...]

next_token():
  │
  ├─> skip_whitespace()
  │   position = 2 → 3 (skip space)
  │
  ├─> ch = input[3] = 'g'
  │
  └─> Match 'g':
      matches "_" branch
      → read_identifier()
         ... (accumulate "greet")
         → return Token::Identifier("greet"), position = 8
```

---

## 🎯 Pipeline Example Walkthrough

### Input: `fn greet { }`

**Step 1: Create Lexer**
```
Lexer {
  input: ['f', 'n', ' ', 'g', 'r', 'e', 'e', 't', ' ', '{', ' ', '}'],
  position: 0
}
```

**Step 2: Create Parser & Read First Token**
```
position = 0
next_token() → Token::Fn
Parser {
  lexer: Lexer { ... },
  current: Token::Fn
}
```

**Step 3: Parse Loop**
```
Iteration 1:
  current = Token::Fn (matches!)
  parse_function()
    advance() → current = Token::Identifier("greet")
    extract name = "greet"
    advance() → current = Token::LBrace
    create Stmt::Function { name: "greet", body: [] }
    return Stmt::Function

Iteration 2:
  current = Token::LBrace (not Fn)
  advance() → current = Token::RBrace

Iteration 3:
  current = Token::RBrace (not Fn)
  advance() → current = Token::EOF

Loop ends: current == EOF
return Vec containing Function statement
```

**Step 4: Output**
```
[
    Function {
        name: "greet",
        body: [],
    }
]
```

---

## 📈 Complexity Analysis

### Lexer
- **Time:** O(n) where n = number of characters
- **Space:** O(n) for storing characters vector
- **Reason:** Each character processed once

### Parser
- **Time:** O(m) where m = number of tokens
- **Space:** O(h) where h = AST height
- **Reason:** Each token processed once, AST size depends on nesting

### Overall Pipeline
- **Time:** O(n) dominated by lexer
- **Space:** O(n + h) for lexer input + AST

---

## 🚀 Extension Points

```
Current: Text → Lexer → Parser → AST → Output

Future extensions:

Option 1: Add TypeChecker
Text → Lexer → Parser → TypeChecker → AST with types → Output

Option 2: Add Codegen
Text → Lexer → Parser → Codegen → Bytecode → Output

Option 3: Add Interpreter
Text → Lexer → Parser → Interpreter → Results → Output

All build on this foundation!
```

---

**Status:** STEP 34 ✅ COMPLETE  
**Date:** January 9, 2026  
**Next:** Enhance AST, add expression parsing, implement type checking
