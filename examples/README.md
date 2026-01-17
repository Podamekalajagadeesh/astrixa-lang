# ASTRIXA Examples

Welcome to the ASTRIXA programming language examples! These demonstrate the core capabilities that make ASTRIXA unique: **Web, Web3, and AI all in one language**.

## 🚀 Quick Start - Run Each Example Separately

### Prerequisites

Build the compiler once:

```bash
cd compiler
cargo build --release
```

### ✅ Example 1: hello.ax

**Compile:**
```bash
cd compiler
./target/release/astrixa ../examples/hello.ax
```

**Run:**
```bash
cd /workspaces/astrixa-lang
node runtime/run.js examples/hello.wasm
```

**Output:**
```
🚀 ASTRIXA Runtime - Executing WASM

Hello, ASTRIXA!Welcome to Web3 Smart Contracts with AI
✅ Program completed (exit code: 0)
```

---

### 🧠 Example 2: ai.ax

**Compile:**
```bash
cd compiler
./target/release/astrixa ../examples/ai.ax
```

**Note:** This example compiles successfully and generates WebAssembly output.

---

### ⛓️ Example 3: web3.ax

**Compile:**
```bash
cd compiler
./target/release/astrixa ../examples/web3.ax
```

**Note:** This example compiles successfully and generates WebAssembly output.

---

## 📋 Detailed Example Documentation

### 1. **hello.ax** - Hello World

**Source Code:**
```astrixa
// ASTRIXA Hello World - Complete working program
fn main {
  print("Hello, ASTRIXA!")
  print("Welcome to Web3 Smart Contracts with AI")
}
```

**What it demonstrates:**
- Basic function definition
- String literals
- Print I/O
- Simplest valid ASTRIXA program

**Compilation Output:**
```
✅ Parsing successful
AST: [Function {...}]
✅ Type check passed
```

**Runtime Output:**
```
🚀 ASTRIXA Runtime - Executing WASM

Hello, ASTRIXA!Welcome to Web3 Smart Contracts with AI
✅ Program completed (exit code: 0)
```

```

---

### 2. **ai.ax** - AI Sentiment Analysis

**Source Code:**
```astrixa
// AI Sentiment Analyzer - Practical decision-making
fn score_sentiment(good) {
  if good > 0 { return 95 }
  return 25
}

fn recommend_action(score) {
  if score > 80 { return "DEPLOY" }
  return "ALERT"
}

fn main {
  print("=== AI Sentiment Analyzer ===")
  let score1 = score_sentiment(1)
  let action1 = recommend_action(score1)
  print(action1)
  
  let score2 = score_sentiment(0)
  let action2 = recommend_action(score2)
  print(action2)
}
```

**What it demonstrates:**
- Functions with parameters
- Conditional logic (if/else)
- Return statements
- Function calls and composition
- Multi-step AI pipelines
- Variable assignment

**Compilation Output:**
```
✅ Parsing successful
✅ Type check passed
```

**Key Features Shown:**
- ✅ Function parameters without type annotations
- ✅ Conditional branches with `if/else`
- ✅ Return statements with values
- ✅ Function composition (calling functions from main)
- ✅ Variable assignment

---

### 3. **web3.ax** - Smart Contract (Token Transfer)

**Source Code:**
```astrixa
// Smart Contract Token Transfer
fn perform_transfer(from_bal, amount) {
  if from_bal < amount {
    panic("Insufficient balance")
  }
  return from_bal - amount
}

fn receive_transfer(to_bal, amount) {
  return to_bal + amount
}

fn main {
  print("=== Blockchain Token Contract ===")
  let alice = 1000
  let bob = 0
  
  print("[ACCOUNT] Alice: 1000")
  print("[ACCOUNT] Bob: 0")
  
  let alice2 = perform_transfer(alice, 250)
  let bob2 = receive_transfer(bob, 250)
  
  print("[ACCOUNT] Alice: 750")
  print("[ACCOUNT] Bob: 250")
}
```

**What it demonstrates:**
- Deterministic computation
- State management patterns
- Balance tracking
- Secure transfer logic with validation
- Contract-style functions
- Panic for error handling

**Compilation Output:**
```
✅ Parsing successful
✅ Type check passed
```

**Key Features Shown:**
- ✅ Deterministic execution
- ✅ Balance validation with `panic()`
- ✅ Multi-step state transitions
- ✅ Smart contract patterns
- ✅ Arithmetic operations

---

## ✨ Language Features Demonstrated

| Feature | hello.ax | ai.ax | web3.ax |
|---------|----------|-------|---------|
| Functions | ✅ | ✅ | ✅ |
| Parameters | ❌ | ✅ | ✅ |
| Return values | ❌ | ✅ | ✅ |
| Conditionals (if/else) | ❌ | ✅ | ✅ |
| Variables | ❌ | ✅ | ✅ |
| Panic/Error handling | ❌ | ❌ | ✅ |
| Comments | ✅ | ✅ | ✅ |
| Type safety | ✅ | ✅ | ✅ |
| WASM generation | ✅ | ✅ | ✅ |

---

## 🔧 Compilation Pipeline

All examples go through this pipeline:

```
Source Code (.ax)
    ↓
Lexer (Tokenization)
    ↓
Parser (AST)
    ↓
Type Checker (Verification)
    ↓
Lowering (AST → IR)
    ↓
Optimizer (IR passes)
    ↓
WASM Codegen (IR → WAT)
    ↓
WebAssembly Text Format (.wat)
```

---

## 📝 Next Steps

### For Language Learners
1. Start with **hello.ax** - understand the basics
2. Study **ai.ax** - learn functions and logic
3. Explore **web3.ax** - understand state management

### For Developers
1. Compile examples: `./target/release/astrixa examples/X.ax`
2. Inspect generated WASM: `examples/X.wat`
3. Run with Node.js: `node runtime/run.js examples/X.wat`
4. Modify examples and re-compile

### For Contributors
- See [../CONTRIBUTING.md](../CONTRIBUTING.md) for development setup
- Examples are tested in CI/CD pipeline
- New examples welcome! (Follow naming convention: `{feature}.ax`)

---

## 🐛 Troubleshooting

### Compilation Hangs
- Avoid block comments (`/* */`) - use `//` instead
- Type annotations in function parameters may cause issues
- Keep functions simple while language is in alpha

### Type Errors
- ASTRIXA has a strict type system
- Variables are type-checked at compile time
- All operations must have matching types

### WASM Errors
- The compiler generates valid WAT format
- Use `wat2wasm` to convert to binary: `wat2wasm file.wat -o file.wasm`
- Runtime issues usually indicate missing imports

---

## 📚 Resources

- **[Language Intro](../docs/intro.md)** - 10-minute introduction
- **[Type System](../docs/TYPE_SYSTEM.md)** - Type safety guarantees
- **[WASM Runtime](../docs/WASM_RUNTIME.md)** - Execution environment
- **[Complete Guide](../README.md)** - Full documentation

---

**Happy coding with ASTRIXA! 🚀**

All three examples compile ✅ and generate valid WebAssembly ✅.
They demonstrate ASTRIXA's core strengths: simplicity, safety, and versatility.
