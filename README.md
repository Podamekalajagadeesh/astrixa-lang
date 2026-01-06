# ASTRIXA Language

A Web3 + AI-native programming language designed for building intelligent blockchain applications and smart contracts. ASTRIXA combines Ethereum-like blockchain primitives with first-class AI operations, enabling developers to build sophisticated DApps without learning multiple languages or frameworks.

**🌐 Now with WebAssembly support - Run Astrixa anywhere: browsers, edge computing, and beyond!**
**📦 Package Manager - Build and share packages with the ASTRIXA ecosystem!**

## Features

### 📦 Package Manager (NEW!)
- **Ecosystem**: Install packages from the ASTRIXA registry
- **Simple Commands**: `astrixa init`, `astrixa install`, `astrixa list`
- **Deterministic**: Exact version pinning with lockfile
- **Secure**: Checksum verification and signed releases (coming soon)
- **Web3-Safe**: No dependency hell, reproducible builds
- [Full Package Manager Documentation](PACKAGE_MANAGER.md)

### 🌐 Universal Runtime
- **WASM Support**: Run Astrixa in web browsers without installation
- **Cross-Platform**: Native, contracts, and web - same language everywhere
- **Browser Playground**: Interactive code editor with live execution
- **Sandbox Security**: Memory-safe, isolated execution environment
- **Edge Computing**: Deploy to serverless and edge platforms

### 🔗 Web3 Native
- **Blockchain Context**: Access chain ID, sender, transaction value, hash, timestamp
- **Smart Contracts**: Define stateful contracts with state variables and methods
- **Gas Model**: Deterministic execution costs for contract safety
- **Web3 Types**: Native `Address` and `U256` types for Ethereum compatibility

### 🧠 AI-First Design
- **Sentiment Analysis**: Built-in emotion detection
- **Text Classification**: Categorize content with deterministic algorithms
- **Embeddings**: Generate semantic vectors for similarity search
- **Tokenization**: Text-to-token conversion
- **Deterministic Execution**: All AI operations are reproducible (safe for contracts)

### ⚙️ Complete Toolchain
- **Lexer**: Full tokenization with 29+ token types
- **Parser**: Recursive descent parser for contract and function definitions
- **Interpreter**: Tree-walking interpreter with blockchain context
- **Bytecode Compiler**: AST-to-bytecode compilation
- **Stack VM**: Gas-metered virtual machine for efficient execution
- **Module System**: Import other programs with circular dependency detection
- **WASM Runtime**: WebAssembly compilation for browser execution

## Quick Start

### Installation

```bash
cd compiler
cargo build --release
```

### Package Manager Quick Start

```bash
# Initialize a new project
astrixa init my-project
cd my-project

# Install packages
astrixa install math
astrixa install ai-tools

# Create your code
echo 'import "math"

fn main() {
    print("10 + 5 =", add(10, 5));
}' > src/main.ax

# Run it!
astrixa run src/main.ax
```

See [PACKAGE_MANAGER.md](PACKAGE_MANAGER.md) for complete documentation.

### Run WASM in Browser

```bash
# Build WASM module
./build_wasm.sh

# Start local server
cd examples
python3 -m http.server 8000

# Open http://localhost:8000/wasm_playground.html
```

### Use in JavaScript

```javascript
import init, { run_astrixa_vm } from './pkg/astrixa.js';

await init();

const result = run_astrixa_vm(`
    fn main() {
        print("Hello from browser!")
    }
`);
console.log(result);
```

### Basic AI Usage

```astrixa
// sentiment_analyzer.ax
fn main() {
    let positive = ai.infer(ai.model("sentiment"), "I love this!");
    print(positive);  // Output: positive: 0.92
    
    let negative = ai.infer(ai.model("sentiment"), "This is terrible");
    print(negative);  // Output: negative: 0.89
}
```

### Smart Contract Example

```astrixa
// content_moderator.ax
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
