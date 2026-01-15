
# ASTRIXA Language

**One language for Web, Web3, and AI.**

ASTRIXA is a modern programming language designed for building web applications, smart contracts, and AI-powered features. Write everything in one language with familiar syntax, strong types, and built-in support for blockchain and AI operations.

**🎉 v0.1.0 Released!** | **[📚 Documentation](docs/intro.md)** | **[🎮 Try Online](playground.html)** | **[⬇️ Install](#installation)**

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![GitHub Stars](https://img.shields.io/github/stars/Podamekalajagadeesh/astrixa-lang?style=social)](https://github.com/Podamekalajagadeesh/astrixa-lang)

## ✨ Why ASTRIXA?

```astrixa
use std::web

server {
    route GET "/" {
        return json({ hello: "ASTRIXA" })
    }
}

server.run(8080)
```

**That's a complete web server.** No boilerplate. No configuration. Just code.

### Key Features

🌐 **Full-Stack Development**  
Build APIs, smart contracts, and AI features in one language

⛓️ **Web3 Native**  
First-class blockchain support with gas-efficient execution

🤖 **AI Operations**  
Built-in sentiment analysis, classification, and embeddings

📦 **Package Manager**  
Modern dependency management with `astrixa.toml`

🚀 **WebAssembly**  
Run in browsers, edge computing, anywhere

🛠️ **Complete Tooling**  
LSP, VS Code extension, online playground

## Quick Examples

### Web API
```astrixa
use std::web

let users = []

server {
    route GET "/api/users" {
        return json(users)
    }
    
    route POST "/api/users" {
        users.push(request.body)
        return json({ success: true }, 201)
    }
}

server.run(8080)
```

### Smart Contract
```astrixa
contract Token {
    state balances: map<Address, U256>
    
    fn transfer(to: Address, amount: U256) {
        require(balances[tx.sender] >= amount, "Insufficient balance")
        balances[tx.sender] -= amount
        balances[to] += amount
    }
    
    fn balanceOf(account: Address) -> U256 {
        return balances[account]
    }
}
```

### AI Analysis
```astrixa
use std::ai

let text = "This product is amazing!"
let sentiment = ai.sentiment(text)  // "positive"

let category = ai.classify("Reset password help", [
    "technical_support",
    "billing",
    "general"
])  // "technical_support"
```

## Installation

### One-Command Install

```bash
curl -fsSL https://astrixa.org/install | sh
```

### Build from Source

```bash
git clone https://github.com/Podamekalajagadeesh/astrixa-lang.git
cd astrixa-lang/compiler
cargo build --release
sudo cp target/release/astrixa /usr/local/bin/
```

### Verify Installation

```bash
astrixa --version
echo 'print("Hello, ASTRIXA!")' > hello.ax
astrixa hello.ax
```

## Getting Started

### Your First Program

```bash
# Create a file
echo 'print("Hello, ASTRIXA!")' > hello.ax

# Run it
astrixa hello.ax
```

### Create a Project

```bash
# Initialize project
astrixa init my-project
cd my-project

# Install packages
astrixa install <package-name>

# Edit main.ax and run
astrixa run main.ax
```

### Try the Playground

**No installation needed!** Try ASTRIXA in your browser:  
👉 **[playground.astrixa.org](playground.html)**

## Documentation

📚 **[Introduction](docs/intro.md)** - Learn ASTRIXA in 10 minutes  
📥 **[Installation](docs/installation.md)** - Detailed setup guide  
📖 **[Language Syntax](docs/language/syntax.md)** - Complete syntax reference  
🎯 **[Standard Library](docs/stdlib/web.md)** - API documentation  
💡 **[Examples](examples/)** - Real-world code samples  

### Language Guide
- [Syntax](docs/language/syntax.md) - Variables, functions, control flow
- [Types](docs/language/types.md) - Type system and collections
- [Async](docs/language/async.md) - Asynchronous programming
- [Errors](docs/language/errors.md) - Error handling patterns
- [Modules](docs/language/modules.md) - Code organization

### Standard Library
- [Web](docs/stdlib/web.md) - HTTP server and routing
- [Web3](docs/stdlib/web3.md) - Blockchain operations
- [AI](docs/stdlib/ai.md) - AI and ML operations

## IDE Support

### VS Code Extension

Install the official ASTRIXA extension from the marketplace:

1. Open VS Code
2. Search for "ASTRIXA" in Extensions
3. Click Install

**Features:**
- Syntax highlighting
- Code completion
- Error diagnostics
- Hover documentation
- Go to definition

### Language Server

ASTRIXA includes an LSP server for any LSP-compatible editor:

```bash
cd lsp
cargo build --release
# Configure your editor to use target/release/astrixa-lsp
```

## Package Manager

```bash
# Initialize new project
astrixa init my-project

# Install packages
astrixa install <package-name>

# List installed packages
astrixa list

# Update dependencies
astrixa update
```

See [Package Manager Documentation](PACKAGE_MANAGER.md) for details.

## WebAssembly

Run ASTRIXA in web browsers:

```bash
# Build WASM module
./build_wasm.sh

# Use in JavaScript
import init, { run_astrixa_vm } from './pkg/astrixa.js';
await init();
const result = run_astrixa_vm('print("Hello from browser!")');
```

## Roadmap

### v0.1.0 (✅ Released)
- Core language features
- Standard library (web, web3, ai)
- Package manager
- LSP and VS Code extension
- WebAssembly support
- Documentation and playground

### v0.2.0 (Q1 2026)
- Async/await implementation
- Try/catch error handling
- Optional and Result types
- Pattern matching
- Enhanced error messages

### v0.3.0 (Q2 2026)
- Remote package registry
- Multi-chain support (Solana, Cosmos)
- Advanced LSP features
- Performance optimizations
- Production-ready tooling

See [ROADMAP.md](ROADMAP.md) for the complete roadmap.

## Contributing

We welcome contributions! Please see:

- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) - Community standards
- [GitHub Issues](https://github.com/Podamekalajagadeesh/astrixa-lang/issues) - Report bugs or request features
- [GitHub Discussions](https://github.com/Podamekalajagadeesh/astrixa-lang/discussions) - Ask questions

## Community

- **GitHub**: [github.com/Podamekalajagadeesh/astrixa-lang](https://github.com/Podamekalajagadeesh/astrixa-lang)
- **Discussions**: [Community Forum](https://github.com/Podamekalajagadeesh/astrixa-lang/discussions)
- **Issues**: [Bug Reports & Features](https://github.com/Podamekalajagadeesh/astrixa-lang/issues)

## License

ASTRIXA is dual-licensed under:
- MIT License ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## Acknowledgments

Special thanks to all contributors and the Rust community for making ASTRIXA possible.

---

**Built with ❤️ for developers who want one language for everything.**

*"ASTRIXA: One language for Web, Web3, and AI"*
contract ContentModerator {
    state: ["moderation_log"]
    
    fn moderate(content: string) {
        let result = ai.infer(ai.model("sentiment"), content);
        
        if result.label == "negative" && result.score > 0.7 {
            panic("Toxic content detected");
        }
        
        emit("ContentApproved", result);
    }
}
```

## Language Syntax

### Variables and Functions

```astrixa
let x = 42;
let name = "ASTRIXA";

fn add(a: number, b: number) {
    return a + b;
}
```

### Control Flow

```astrixa
if x > 10 {
    print("Large");
} else {
    print("Small");
}

while x > 0 {
    x = x - 1;
}
```

### Arrays and Objects

```astrixa
let arr = [1, 2, 3, 4, 5];
print(len(arr));        // 5
print(arr[0]);          // 1

let tokens = ai.tokenize("Hello world");
print(tokens);          // ["hello", "world"]
```

### Blockchain Access

```astrixa
// Access blockchain context
print(chain.id);        // Chain ID (e.g., 1 for Ethereum mainnet)
print(chain.name);      // Chain name (e.g., "mainnet")

print(msg.sender);      // Current caller address
print(msg.value);       // ETH value sent with transaction
print(msg.data);        // Transaction calldata

print(tx.hash);         // Transaction hash
print(tx.timestamp);    // Block timestamp
```

### Modules and Imports

```astrixa
import "math.ax";

fn main() {
    let result = math.add(5, 3);
    print(result);
}
```

## Type System

- **Primitives**: `number`, `string`, `bool`, `null`
- **Collections**: `array`
- **Web3 Types**: `address`, `u256`
- **AI Types**: `ai_result` (with `label` and `score` fields)

Type introspection:
```astrixa
print(type(42));        // "number"
print(type("hello"));   // "string"
let r = ai.infer(ai.model("sentiment"), "great");
print(type(r));         // "ai_result"
```

## Execution Modes

### Tree-Walking Interpreter (Default)
```bash
./astrixa program.ax --interp
```
Good for development and debugging.

### Bytecode VM
```bash
./astrixa program.ax --vm
```
Faster execution with gas metering.

## AI Methods Reference

| Method | Input | Output | Use Case |
|--------|-------|--------|----------|
| `ai.model(name)` | Model name (string) | Model identifier | Load an AI model |
| `ai.infer(model, text)` | Model + text | AIResult | Classification & sentiment |
| `ai.embed(text)` | Text string | Array of numbers | Semantic similarity |
| `ai.tokenize(text)` | Text string | Array of strings | Text preprocessing |

## Gas Costs (VM Mode)

All operations have deterministic gas costs:

```
LoadConst:   1 gas
LoadVar:     2 gas
StoreVar:    2 gas
Add/Sub:     3 gas
Mul:         5 gas
Div:         8 gas
Call:        10 gas
AI.infer:    50 gas
AI.embed:    100 gas
AI.tokenize: 30 gas
```

## Documentation

- [AI Primitives Guide](AI_PRIMITIVES.md) - Complete AI API reference
- [Gas Model](GAS_MODEL.md) - Gas costs and limits
- [Design Principles](docs/principles.md) - Language design philosophy
- [Vision](docs/vision.md) - Roadmap and future features

## Examples

See the [examples/](examples/) directory for complete programs:

- `sentiment_analyzer.ax` - Analyze text emotion
- `content_classifier.ax` - Classify content by topic
- `embedding_search.ax` - Find similar documents
- `contract_with_ai.ax` - Smart contract using AI

## Project Structure

```
astrixa-lang/
├── compiler/              # Rust compiler implementation
│   ├── src/
│   │   ├── main.rs       # CLI entry point
│   │   ├── lexer.rs      # Tokenization
│   │   ├── parser.rs     # AST generation
│   │   ├── interpreter.rs # Tree-walking executor
│   │   ├── ast.rs        # AST node definitions
│   │   ├── bytecode.rs   # Bytecode instruction set
│   │   ├── compiler.rs   # AST-to-bytecode compiler
│   │   ├── vm.rs         # Stack-based VM
│   │   ├── gas.rs        # Gas model implementation
│   │   └── ai_runtime.rs # AI operation runtime
│   └── Cargo.toml
├── examples/              # Example programs
├── stdlib/                # Standard library definitions
├── design/                # Design documentation
├── docs/                  # Additional documentation
└── README.md

```

## Design Philosophy

1. **Web3-First**: Blockchain operations are built into the language, not layered on top
2. **AI-Native**: AI is a first-class feature, not a library or afterthought
3. **Deterministic**: All operations produce reproducible results (safe for blockchain)
4. **Single Language**: No need to learn Solidity, Python, and JavaScript - just ASTRIXA
5. **Developer-Friendly**: Simple syntax, clear semantics, excellent error messages

## Roadmap

- [x] Lexer and parser
- [x] Interpreter (tree-walking)
- [x] Module/import system
- [x] Bytecode compiler
- [x] Stack VM with gas metering
- [x] Web3 primitives (blockchain context)
- [x] Smart contracts with state
- [x] AI-native operations
- [ ] Standard library (collections, utilities)
- [ ] Package manager
- [ ] IDE integration (VS Code extension)
- [ ] Formal verification support
- [ ] GPU-accelerated VM

## Governance

ASTRIXA is community-driven with clear processes for evolution:

- **[Governance Model](GOVERNANCE.md)**: Three-phase evolution from BDFL to Foundation
- **[RFC Process](rfcs/RFC_PROCESS.md)**: How language changes are proposed and decided
- **[Roadmap](ROADMAP.md)**: Versioned milestones from v0.1 to v2.0+
- **[Code of Conduct](CODE_OF_CONDUCT.md)**: Community standards and behavior

## Contributing

We welcome contributions! ASTRIXA is built by the community, for the community.

**Get Started:**
- Read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines
- Check [ROADMAP.md](ROADMAP.md) for priorities
- Browse [open issues](https://github.com/astrixa-lang/astrixa/issues)
- Join our [Discord](#) for discussions

**Ways to Contribute:**
- 🐛 Report bugs and request features
- 📝 Improve documentation
- 💻 Submit code and fix issues
- 🗳️ Participate in RFC discussions
- 🎓 Help others in the community

## Security

Found a security vulnerability? Please report responsibly:
- **Email**: security@astrixa.dev
- **Do NOT** create public issues for security bugs
- See [SECURITY.md](SECURITY.md) for full policy

## License

Apache 2.0 - See [LICENSE](LICENSE) for details.

**This is open-source software, free to use and modify. Patent protection included.**

## Resources

### Documentation
- [START_HERE.md](START_HERE.md) - New user guide
- [INDEX.md](INDEX.md) - Complete documentation index
- [STDLIB_COMPLETE_REFERENCE.md](STDLIB_COMPLETE_REFERENCE.md) - Standard library
- [PACKAGE_MANAGER.md](PACKAGE_MANAGER.md) - Package management
- [LSP_INDEX.md](LSP_INDEX.md) - IDE support

### Community
- **Discord**: [Join](#) (coming soon)
- **Forum**: [Discuss](#) (coming soon)
- **Twitter**: [@astrixalang](#) (coming soon)
- **GitHub**: [astrixa-lang/astrixa](https://github.com/astrixa-lang/astrixa)

### Inspiration
- **Ethereum JSON-RPC**: Blockchain context compatibility
- **Solidity**: Contract syntax
- **Python**: Clean syntax
- **Rust**: Memory safety and tooling
- **Lua**: VM design

---

Built with ❤️ by the ASTRIXA community for the future of Web3 × AI
