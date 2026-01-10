# STEP 36: Visual Architecture & Diagrams

## 🏗️ System Architecture

### Before (❌ Panic-Based)
```
┌─────────────────────────────────────────────┐
│         Source Code                         │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│    Lexer                                    │
│  (No position tracking)                     │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│    Parser                                   │
│  • Reads tokens                             │
│  • Parses syntax                            │
│  • panic! on error ❌                        │
└──────────────────┬──────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │
        ▼                     ▼
    🔥 PANIC          (if successful)
    • Thread crash       │
    • Stack trace        ▼
    • User fear       Type Check
                      ✅ Continue
```

### After (✅ Result-Based)
```
┌─────────────────────────────────────────────┐
│         Source Code                         │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│    Lexer                                    │
│  • Tracks line/column ✅                    │
│  • Position awareness ✅                    │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│    Parser                                   │
│  • Reads tokens with position              │
│  • Parses syntax                            │
│  • Result<Stmt, Error> ✅                   │
└──────────────────┬──────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │
        ▼                     ▼
      Err              Ok(AST)
        │                │
        ▼                ▼
    Error Handler    Type Checker
        │                │
        ▼                ▼
   display_error()   ✅ Continue
    • Clear message
    • Line & column
    • Help text
    😊 User happy
```

---

## 📊 Data Flow

### Error Processing Flow

```
┌─────────────────────────────────────────┐
│         Parser encounters error         │
│         (e.g., missing identifier)      │
└────────────────────┬────────────────────┘
                     │
        ┌────────────▼────────────┐
        │                         │
        ▼                         ▼
    Create Error            Populate Fields
    CompileError::new       • message: "Expected..."
                            • line: 2
                            • column: 8
                            
                                 │
                                 ▼
                            Add Help Text
                            .help("Function names...")
                            
                                 │
                                 ▼
                            Return Error
                            Err(err)
                            
                                 │
                                 ▼
                            Error Propagates
                            (via ?)
                            
                                 │
                                 ▼
                            Match in Main
                            
                        ┌───────┴────────┐
                        │                │
                        ▼                ▼
                   display_error    (other handling)
                        │
                    Output:
                    Error: Expected...
                     → line 2, column 8
                     Help: Function...
```

---

## 🔄 Lexer Position Tracking

### Character-by-Character Tracking

```
Input: "fn\ngreet {"
        ┌─────────────────┬──────────┬──────────┐
        │ Character       │ Position │ Line:Col │
        ├─────────────────┼──────────┼──────────┤
        │ 'f'             │ 0        │ 1:1      │
        │ 'n'             │ 1        │ 1:2      │
        │ '\n'            │ 2        │ 1:3 →    │ (→ resets to 2:1)
        │ 'g'             │ 3        │ 2:1      │
        │ 'r'             │ 4        │ 2:2      │
        │ 'e'             │ 5        │ 2:3      │
        │ 'e'             │ 6        │ 2:4      │
        │ 't'             │ 7        │ 2:5      │
        │ ' '             │ 8        │ 2:6      │
        │ '{'             │ 9        │ 2:7      │
        └─────────────────┴──────────┴──────────┘

advance() method:
┌────────────────────────────────────────┐
│ fn advance(&mut self)                  │
├────────────────────────────────────────┤
│ if current_char == '\n'                │
│     line += 1                          │
│     column = 1                         │
│ else                                   │
│     column += 1                        │
│ position += 1                          │
└────────────────────────────────────────┘
```

---

## 🌳 Error Type Hierarchy (Future)

```
CompileError (Current)
├── Struct
├── Fields: message, line, column, help
└── Methods: new(), help(), Display

Future Error Types:
├── SyntaxError
│   ├── UnexpectedToken
│   ├── MissingToken
│   └── InvalidIdentifier
├── TypeError
│   ├── TypeMismatch
│   ├── UndefinedType
│   └── IncompatibleTypes
├── ScopeError
│   ├── UndefinedVariable
│   ├── DuplicateDeclaration
│   └── OutOfScope
└── SemanticError
    ├── InvalidOperation
    ├── InvalidContext
    └── OtherSemanticIssue
```

---

## 💬 Error Message Anatomy

### Template
```
Error: <SPECIFIC_PROBLEM>
 → line <N>, column <M>
 Help: <ACTIONABLE_ADVICE>
```

### Example 1: Missing Function Name
```
Input:
    fn {
    }

Output:
Error: Expected function name
 → line 1, column 4
 Help: Function names must be valid identifiers
```

### Example 2: Invalid Type (Future)
```
Input:
    let x: InvalidType;

Output:
Error: Unknown type 'InvalidType'
 → line 1, column 11
 Help: Did you mean one of: int, float, bool, string?
```

### Example 3: Duplicate Variable (Future)
```
Input:
    let x = 5;
    let x = 10;

Output:
Error: Variable 'x' already declared
 → line 2, column 5
 Help: Use a different name or reassign with 'x = value'
```

---

## 📈 Compilation Pipeline

### Full Compilation Flow

```
Source Code (.ax file)
        │
        ▼
┌──────────────────────┐
│ Lexer                │
│ • Reads characters   │
│ • Tracks line/col    │
│ • Produces tokens    │
└─────────┬────────────┘
          │ Result<Vec<Token>, Error>
          │
      ┌───┴────┐
      │         │
      ▼ Err     │ Ok
    Display  Tokens
    Error     │
      │       ▼
      │   ┌──────────────────────┐
      │   │ Parser               │
      │   │ • Reads tokens       │
      │   │ • Builds AST         │
      │   │ • Checks syntax      │
      │   └─────────┬────────────┘
      │             │ Result<Vec<Stmt>, Error>
      │             │
      │         ┌───┴────┐
      │         │         │
      │         ▼ Err     │ Ok
      │       Display  AST
      │       Error     │
      │         │       ▼
      │         │   ┌──────────────────────┐
      │         │   │ Type Checker         │
      │         │   │ • Validates types    │
      │         │   │ • Checks operations  │
      │         │   └─────────┬────────────┘
      │         │             │ Result<(), Vec<Error>>
      │         │             │
      │         │         ┌───┴────┐
      │         │         │         │
      │         ▼ Err     │ Ok      │
      │       Display     │         │
      │       Errors      │         ▼
      │         │         │     ✅ Compilation
      │         │         │        Success
      └─────────┴─────────┘
              │
              ▼
        🎉 Complete
```

---

## 🔧 Parser State Machine (Simplified)

```
              ┌─────────────────┐
              │   PARSE START   │
              └────────┬────────┘
                       │
                       ▼
              ┌─────────────────┐
              │ Read Token      │
              └────────┬────────┘
                       │
        ┌──────────┬───┴───┬──────────┐
        │          │       │          │
        ▼          ▼       ▼          ▼
      Fn        Let   Other        EOF
        │          │       │          │
        ▼          ▼       ▼          ▼
  Parse Fn   Parse Let  Skip    End Parse
        │          │       │          │
        ├──────────┴───┬───┴──────────┤
        │              │              │
        ▼              ▼              ▼
   Success        Continue         Done
        │              │              │
        └──────────┬───┴──────────────┘
                   │
                   ▼
          ┌─────────────────┐
          │ Error Handling  │
          │ (if any)        │
          └────────┬────────┘
                   │
                   ▼
        ┌─────────────────┐
        │  Return Result  │
        │ Ok(stmts) or    │
        │ Err(error)      │
        └─────────────────┘
```

---

## 📝 File Relationships

```
┌──────────────────────────────────────────────────────┐
│                   main.rs                            │
│  • Orchestrates compilation                         │
│  • Handles errors from parser                       │
│  • Calls display_error()                            │
└──────────────────────────────────────────────────────┘
        │                    │
        ▼                    ▼
┌────────────────┐  ┌─────────────────────┐
│   parser.rs    │  │ diagnostics.rs      │
│ • parse()      │  │ • display_error()   │
│ • parse_fn()   │  │ • display_errors()  │
│ Returns Err    │  │ • Formats messages  │
└────────────────┘  └─────────────────────┘
        │                    ▲
        │ needs              │ formats
        │ location info      │
        ▼                    │
┌────────────────┐      ┌─────────────────────┐
│   lexer.rs     │      │ error.rs            │
│ • next_token() │      │ • CompileError      │
│ • line/column  │      │ • Display impl      │
│ • advance()    │      │ • Builder pattern   │
└────────────────┘      └─────────────────────┘
```

---

## ✨ Quality Transformation

### Crash to Clarity

```
❌ BEFORE:
thread 'main' panicked at 'Expected function name'
[scary internal details]
[user is confused]

                    ▼ STEP 36 ▼

✅ AFTER:
Error: Expected function name
 → line 2, column 8
 Help: Function names must be valid identifiers
[user understands]
[user knows how to fix]
```

---

## 🎯 Design Principles Map

```
┌─────────────────────────────────────────────────┐
│     ASTRIXA Error Design Principles             │
├─────────────────────────────────────────────────┤
│                                                 │
│  1. Never Blame the User                        │
│     "Expected identifier"  ✓ Good              │
│     "You forgot the name"  ✗ Blame             │
│                                                 │
│  2. Always Explain the Fix                      │
│     With .help() text ✓                        │
│     Actionable advice ✓                        │
│                                                 │
│  3. Never Dump Internals                        │
│     No stack trace ✓                           │
│     No Rust details ✓                          │
│     No jargon ✓                                │
│                                                 │
│  4. Be Precise                                  │
│     Exact line ✓                               │
│     Exact column ✓                             │
│     Specific issue ✓                           │
│                                                 │
│  5. Graceful Failure                            │
│     No panics ✓                                │
│     Return Result ✓                            │
│     Let caller handle ✓                        │
│                                                 │
└─────────────────────────────────────────────────┘
```

---

## 📊 Comparison Matrix

```
Feature              │  Before  │  After
─────────────────────┼──────────┼─────────
Error Type           │ panic!   │ Result
Message Quality      │ cryptic  │ clear
Location Info        │ none     │ line:col
Help Text            │ none     │ helpful
User Experience      │ scary    │ friendly
Crash Risk           │ high     │ none
Professional Grade   │ no       │ yes
Extension Ready      │ no       │ yes
IDE Compatible       │ no       │ yes
Multiple Errors      │ n/a      │ ready
Error Recovery       │ n/a      │ ready
```

---

## 🚀 Evolution Timeline

```
STEP 35: Type Checker
        ✓ Basic type checking
        │
        ▼
STEP 36: Error Diagnostics ✅ (YOU ARE HERE)
        ✓ Clear error messages
        ✓ Position tracking
        ✓ Professional quality
        │
        ▼
STEP 37: Expanded Parser
        • More syntax support
        • Additional error types
        
        ▼
STEP 38: Error Recovery
        • Multiple errors
        • Continue parsing
        
        ▼
STEP 39: Multi-pass Compilation
        • Better error context
        • Forward references
        
        ▼
STEP 40+: Advanced Features
        • IDE Integration
        • Suggestions
        • Error codes
```

---

*STEP 36: Visual Architecture Complete*
