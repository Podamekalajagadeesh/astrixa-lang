# Astrixa Lang - Project Structure

## Quick Navigation

```
astrixa-lang/
│
├── README.md                     🎯 START HERE
├── EXAMPLES.md                   📝 Code examples
│
├── compiler/                     ⚙️  Compiler (Rust)
├── runtime/                      🚀 Runtime (JavaScript)
├── lsp/                          💻 Language Server
├── astrixa-vscode/               🎨 VS Code Extension
├── stdlib/                       📦 Standard Library
├── examples/                     📚 Example Programs
├── tests/                        ✅ Tests
├── docs/                         📖 Documentation
├── scripts/                      🛠️  Scripts
└── design/                       🏗️  Design Docs
```

---

## How to Navigate

### 🚀 Getting Started
1. Read [README.md](../README.md) - 5 minutes
2. Try [browser playground](../examples/playground.html) - 2 minutes  
3. Follow [installation.md](installation.md) - 10 minutes
4. Run examples from [EXAMPLES.md](../EXAMPLES.md) - 5 minutes

### 📚 Learning the Language
- **Basics**: [intro.md](intro.md)
- **Features**: [language/syntax.md](language/syntax.md)
- **Types**: [language/types.md](language/types.md)
- **Advanced**: [../TYPE_SYSTEM.md](../TYPE_SYSTEM.md)

### ⛓️ Web3 & Smart Contracts
- **Examples**: [../examples/smart_contract_token.ax](../examples/smart_contract_token.ax)
- **Reference**: [stdlib/web3.md](stdlib/web3.md)

### 🤖 AI Features
- **Guide**: [AI_PRIMITIVES.md](AI_PRIMITIVES.md)
- **Reference**: [stdlib/ai.md](stdlib/ai.md)

### 💻 Contributing
- **Guidelines**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Setup**: [installation.md](installation.md)

---

## Component Status

| Component | Status | Purpose |
|-----------|--------|---------|
| Compiler | ✅ Working | Astrixa → WebAssembly |
| Runtime | ✅ Working | Execute WASM in Node.js |
| Stdlib | ✅ Working | AI, Web3, Web, Crypto, JSON |
| Type System | ✅ Working | Type safety & contracts |
| LSP | ✅ Working | VS Code support |
| Gas Metering | ✅ Working | Smart contract costs |
| Examples | ✅ Working | Hello world, contracts, AI |
| Playground | ✅ Working | Browser-based IDE |

---

## Directory Details

**`/compiler`** - Core language implementation (Rust)
- Lexer, Parser, Type Checker, WASM Codegen

**`/runtime`** - Execution environment (JavaScript)  
- Runs compiled WASM programs
- Command: `node runtime/run.js program.wasm`

**`/stdlib`** - Standard library (Astrixa source)
- AI, Web3, Web, Crypto, JSON, etc.
- Imported: `use std::MODULE_NAME`

**`/docs`** - All documentation  
- Language guides, stdlib refs, examples
- Organized by topic, fully cross-referenced

**`/examples`** - Runnable example programs
- Hello world, smart contracts, AI operations

**`/lsp` & `/astrixa-vscode`** - Editor support
- Language Server Protocol
- VS Code extension with syntax highlighting

**`/tests`** - Test programs  
- Verify language features, stdlib, compiler

**`/scripts`** - Build utilities  
- Compilation, testing, installation helpers

---

## For Contributors

### Adding a Feature
1. Modify: `/compiler/src/`
2. Test: `/tests/`
3. Document: `/docs/`
4. Example: `/examples/`

### Reporting Issues
Include:
- `astrixa --version`
- Error message + stack trace
- Minimal reproducible example

### Full Contribution Guide
See [CONTRIBUTING.md](CONTRIBUTING.md)
