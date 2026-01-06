# ASTRIXA Language Server Protocol (LSP)

**World-Class IDE Support for ASTRIXA**

## 🎯 Why LSP Matters

Developers don't just choose languages - they choose **developer experience**.

Without LSP:
- ❌ Slow, frustrating coding
- ❌ Constant context switching to docs
- ❌ Runtime errors you could have caught earlier
- ❌ No adoption

With LSP:
- ✅ **Instant feedback** while typing
- ✅ **Autocomplete** that feels like magic
- ✅ **Documentation** at your fingertips
- ✅ **Go-to-definition** that just works
- ✅ **Professional** development experience

## ✨ Features

### 1. Real-Time Diagnostics ⚡

Errors appear as you type, with **human-friendly messages**:

```astrixa
fn add(a: number, b: string) {
    return a + b   // ❌ Cannot mix strings and numbers. Convert one to match the other
}
```

**Not scary, not cryptic - just helpful.**

### 2. Intelligent Autocomplete 💡

Type `ai.` and get:
- `model` - Load AI model
- `infer` - Run inference
- `embed` - Generate embeddings
- `tokenize` - Split text

Type `chain.` and get:
- `id` - Chain ID
- `name` - Chain name

Type `msg.` and get:
- `sender` - Caller address
- `value` - ETH sent
- `data` - Transaction data

**Context-aware completions that understand ASTRIXA.**

### 3. Rich Documentation on Hover 📖

Hover over any function to see:
- Function signature
- Parameter types
- Return type
- Usage examples
- Related functions

Example:
```astrixa
print("Hello")  // Hover shows: fn print(msg: any) -> ()
```

### 4. Go-to-Definition 🔗

Click on a function call, jump to its definition. Simple.

### 5. Document Outline 📋

See all functions, variables, and types in your file at a glance.

## 🚀 Installation

### Build the LSP Server

```bash
cd /workspaces/astrixa-lang/lsp
cargo build --release
```

Binary location: `lsp/target/release/astrixa-lsp`

### Install VS Code Extension

```bash
cd /workspaces/astrixa-lang/astrixa-vscode
npm install
npm run compile
code --install-extension .
```

Or manually:
```bash
cp -r astrixa-vscode ~/.vscode/extensions/
```

### Verify Installation

1. Create a `.ax` file
2. Open in VS Code
3. Check bottom-right corner for "ASTRIXA" status
4. Start typing and see autocomplete!

## 🏗️ Architecture

```
User types in VS Code
        ↓
VS Code Extension (TypeScript)
        ↓
LSP Server (Rust)
        ↓
ASTRIXA Compiler Frontend
   ├─ Lexer
   ├─ Parser
   └─ Type Checker
        ↓
Diagnostics, Completions, Hover, etc.
        ↓
Back to VS Code
```

**Key Design:** LSP reuses the same lexer/parser as the compiler. No duplication, no drift.

## 🧠 LSP Features in Detail

### Diagnostics Engine

**Human-Friendly Error Messages:**

| Bad | Good |
|-----|------|
| `TypeError at line 4` | `Cannot mix strings and numbers. Convert one to match the other` |
| `SyntaxError` | `Function needs parentheses. Try: fn name(...) { }` |
| `Undefined variable` | `Variable 'x' is not defined` |

**Three Severity Levels:**
- 🔴 **Error** - Code won't run
- ⚠️ **Warning** - Code works but could be better
- 💡 **Hint** - Style suggestions

### Completion Provider

**Context-Aware Completions:**

| Context | Completions |
|---------|-------------|
| After `ai.` | `model`, `infer`, `embed`, `tokenize` |
| After `chain.` | `id`, `name` |
| After `msg.` | `sender`, `value`, `data` |
| After `tx.` | `hash`, `timestamp` |
| Empty line | Keywords, functions, types |

**Smart Snippets:**
- Type `fn` → Get `fn name(params) { }`
- Type `if` → Get `if condition { }`
- Type `ai.infer` → Get `ai.infer(ai.model("$1"), "$2")`

### Hover Provider

**Rich Documentation:**
- Function signatures with types
- Parameter descriptions
- Return values
- Usage examples in ASTRIXA
- Links to related functions

### Symbol Provider

**Document Outline Shows:**
- Functions (with icon)
- Variables (with icon)
- Types/Structs (with icon)
- Contracts (with icon)

## 📝 Usage Examples

### Example 1: AI Operations

```astrixa
fn analyze_sentiment() {
    let model = ai.model("sentiment");  // Hover for docs
    let result = ai.infer(model, text); // Autocomplete suggests
    
    if result.label == "positive" {     // Type checking
        print("Great!");
    }
}
```

**LSP provides:**
- Autocomplete for `ai.` methods
- Hover docs for each function
- Type checking for `result.label`
- Diagnostics if you misuse APIs

### Example 2: Smart Contract

```astrixa
contract Token {
    state: ["balances"]
    
    fn transfer(to: address, amount: number) {
        if balances[msg.sender] < amount {  // msg.sender autocompletes
            panic("Insufficient balance");  // panic shows hover docs
        }
        
        emit("Transfer", {               // emit autocompletes
            from: msg.sender,
            to: to,
            amount: amount
        });
    }
}
```

**LSP provides:**
- `msg.sender` autocomplete
- `emit` documentation
- Type checking for `amount`
- Go-to-definition for functions

### Example 3: Type Safety

```astrixa
let name: string = 42;  // ❌ Cannot assign number to string. Remove ': string' or wrap in quotes

let x = "5" + 10;       // ❌ Cannot mix strings and numbers. Convert one to match the other

fn divide(a, b) {
    return a / 0;       // ⚠️ Division by zero will cause a runtime error
}
```

## 🛠️ Configuration

### VS Code Settings

```json
{
  "astrixa.lsp.path": "/path/to/astrixa-lsp",
  "astrixa.lsp.debug": false
}
```

### Enable Debug Logging

```json
{
  "astrixa.lsp.debug": true
}
```

Then check: `Output` → `ASTRIXA Language Server`

## 🔍 Troubleshooting

### LSP Not Starting

1. Check LSP binary exists:
   ```bash
   ls lsp/target/release/astrixa-lsp
   ```

2. Check VS Code extension is installed:
   ```bash
   code --list-extensions | grep astrixa
   ```

3. Check Output panel:
   `View` → `Output` → Select "ASTRIXA Language Server"

### Completions Not Working

1. Save the file (`.ax` extension)
2. Reload VS Code window
3. Check file is recognized as ASTRIXA (bottom-right corner)

### Hover Not Showing

- Ensure cursor is directly on a keyword/function name
- Try moving cursor off and back on

## 🎨 Future Enhancements

### Planned Features

- [ ] **Formatting** - Auto-format on save
- [ ] **Refactoring** - Rename symbol, extract function
- [ ] **Code Actions** - Quick fixes for common errors
- [ ] **Inlay Hints** - Show types inline
- [ ] **Semantic Highlighting** - Better syntax colors
- [ ] **Call Hierarchy** - See function call trees
- [ ] **Debugging** - Breakpoints and step-through

### Nice-to-Have

- [ ] **Snippets** - Template code blocks
- [ ] **Linting** - Code quality checks
- [ ] **Testing Integration** - Run tests from editor
- [ ] **Git Blame** - See commit info inline
- [ ] **Breadcrumbs** - Show current function in header

## 🏆 Why ASTRIXA LSP is Different

### 1. Reuses Compiler Frontend ✅

Most languages duplicate parsing logic between compiler and LSP. ASTRIXA doesn't.

**Benefit:** Zero drift between what compiler sees and what LSP sees.

### 2. Human-First Error Messages ✅

Errors are calm, clear, and actionable. No cryptic codes, no scary language.

**Example:**
- ❌ `Error E0308: mismatched types`
- ✅ `Cannot mix strings and numbers. Convert one to match the other`

### 3. AI & Web3 Aware ✅

LSP understands `ai.*` operations and `chain.*` / `msg.*` context natively.

**No other language has this built-in.**

### 4. Fast & Lightweight ✅

Written in Rust, uses Tower-LSP for performance. No Electron bloat.

## 📚 Learn More

- [AI Primitives](../AI_PRIMITIVES.md) - AI operations reference
- [Package Manager](../PACKAGE_MANAGER.md) - Using packages
- [README](../README.md) - Language overview
- [LSP Specification](https://microsoft.github.io/language-server-protocol/) - LSP protocol

## 🤝 Contributing

Want to improve the LSP?

1. **Add completions** - Edit `lsp/src/completion.rs`
2. **Improve diagnostics** - Edit `lsp/src/diagnostics.rs`
3. **Add hover docs** - Edit `lsp/src/hover.rs`
4. **Build and test**:
   ```bash
   cd lsp
   cargo build --release
   # Test in VS Code
   ```

## ✨ The Result

With LSP, coding in ASTRIXA feels like:
- ✅ **Professional** - Like TypeScript/Rust
- ✅ **Fast** - Instant feedback
- ✅ **Intuitive** - Suggestions make sense
- ✅ **Productive** - Less time debugging

**This is what makes developers choose a language.**

---

**ASTRIXA: AI-Native. Web3-First. Developer-Loved.**
