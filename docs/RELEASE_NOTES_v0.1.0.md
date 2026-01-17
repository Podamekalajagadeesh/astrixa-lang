# ASTRIXA v0.1.0 Release Notes

## 🎉 First Public Release

We're thrilled to announce the first public release of **ASTRIXA** - a modern programming language designed for Web, Web3, and AI development.

### What is ASTRIXA?

ASTRIXA is a programming language that lets you build:
- **Web applications** with a built-in HTTP framework
- **Smart contracts** for blockchain platforms
- **AI-powered features** with deterministic operations

All in one language, with familiar syntax and modern features.

## ✨ Highlights

### One Language, Full Stack

```astrixa
use std::web
use std::ai

server {
    route POST "/analyze" {
        let text = request.body.text
        let sentiment = ai.sentiment(text)
        return json({ sentiment: sentiment })
    }
}

server.run(8080)
```

### Smart Contract Support

```astrixa
contract Token {
    state balances: map<Address, U256>
    
    fn transfer(to: Address, amount: U256) {
        require(balances[tx.sender] >= amount, "Insufficient balance")
        balances[tx.sender] -= amount
        balances[to] += amount
    }
}
```

### Built-in AI Operations

```astrixa
use std::ai

let sentiment = ai.sentiment("ASTRIXA is amazing!")  // "positive"
let category = ai.classify(text, ["tech", "business", "sports"])
```

## 📦 What's Included

### Core Features
✅ Complete lexer, parser, and interpreter  
✅ Bytecode compiler with stack VM  
✅ Gas-metered execution for smart contracts  
✅ Module system with dependency management  
✅ Package manager (`astrixa init`, `install`, `list`)  
✅ WebAssembly compilation for browser execution  

### Standard Library
✅ `std::web` - HTTP server framework  
✅ `std::web3` - Blockchain operations  
✅ `std::ai` - AI operations (sentiment, classification)  
✅ `std::json` - JSON parsing  
✅ `std::crypto` - Cryptographic functions  
✅ `std::fs` - File system operations  

### Developer Tools
✅ VS Code extension with syntax highlighting  
✅ Language Server Protocol (LSP) for IDE support  
✅ Online playground for browser-based experimentation  
✅ Comprehensive documentation  
✅ Example programs and tutorials  

## 🚀 Getting Started

### Install ASTRIXA

**One-command install:**
```bash
curl -fsSL https://astrixa.org/install | sh
```

**Or build from source:**
```bash
git clone https://github.com/Podamekalajagadeesh/astrixa-lang.git
cd astrixa-lang/compiler
cargo build --release
```

### Your First Program

```bash
echo 'print("Hello, ASTRIXA!")' > hello.ax
astrixa hello.ax
```

### Create a Project

```bash
astrixa init my-project
cd my-project
# Edit main.ax
astrixa run main.ax
```

## 📚 Documentation

- **[Introduction](docs/intro.md)** - Learn ASTRIXA in 10 minutes
- **[Installation Guide](docs/installation.md)** - Detailed setup instructions
- **[Language Syntax](docs/language/syntax.md)** - Complete syntax reference
- **[Standard Library](docs/stdlib/web.md)** - API documentation
- **[Examples](examples/)** - Real-world code samples

## 🎮 Try the Playground

Experiment with ASTRIXA in your browser:  
👉 **[playground.astrixa.org](playground.html)**

No installation required!

## 🧪 Example: REST API in 5 Minutes

```astrixa
use std::web

let users = []

server {
    route GET "/api/users" {
        return json(users)
    }
    
    route POST "/api/users" {
        let user = request.body
        users.push(user)
        return json(user, 201)
    }
}

server.run(8080)
```

## 🌟 What Makes ASTRIXA Different?

| Feature | ASTRIXA | Traditional Approach |
|---------|---------|---------------------|
| **Full Stack** | ✅ One language | ❌ Multiple languages |
| **Web + Web3 + AI** | ✅ Native support | ❌ Need SDKs/libraries |
| **Smart Contracts** | ✅ Built-in | ❌ Separate language |
| **Type Safety** | ✅ Static types | ⚠️ Varies |
| **Gas Efficiency** | ✅ Optimized | ⚠️ Runtime overhead |

## 🎯 Use Cases

### Web Developers
Build APIs and web services with minimal boilerplate.

### Blockchain Developers
Write smart contracts without learning Solidity.

### Full-Stack Engineers
Use one language from frontend to blockchain.

### AI Researchers
Integrate deterministic AI operations in contracts.

## ⚠️ Known Limitations

This is an initial release (v0.1.0). Some features are planned but not yet implemented:

- ❌ Async/await (coming in v0.2)
- ❌ Try/catch error handling (coming in v0.2)
- ❌ Optional types (`T?`) (coming in v0.2)
- ❌ Remote package registry (coming soon)
- ❌ Multi-chain support beyond EVM (roadmap)

See [CHANGELOG.md](CHANGELOG.md) for full details.

## 🛠️ Technical Details

- **Written in**: Rust
- **License**: MIT
- **Platforms**: Linux, macOS, Windows
- **Architecture**: x86_64, aarch64

## 🤝 Contributing

We welcome contributions! See:
- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) - Community guidelines
- [GitHub Issues](https://github.com/Podamekalajagadeesh/astrixa-lang/issues) - Report bugs

## 🗺️ Roadmap

### v0.2.0 (Q1 2026)
- Async/await
- Error handling (try/catch)
- Optional and Result types
- Pattern matching

### v0.3.0 (Q2 2026)
- Remote package registry
- Multi-chain support (Solana)
- Enhanced LSP features
- Performance optimizations

See [ROADMAP.md](ROADMAP.md) for the complete roadmap.

## 💬 Community

- **GitHub Discussions**: [Join the conversation](https://github.com/Podamekalajagadeesh/astrixa-lang/discussions)
- **Issues**: [Report bugs](https://github.com/Podamekalajagadeesh/astrixa-lang/issues)
- **Documentation**: [Read the docs](docs/intro.md)

## 📢 Spread the Word

If you like ASTRIXA, please:
- ⭐ Star the repo on GitHub
- 🐦 Share on Twitter/X
- 📝 Write a blog post
- 💬 Tell your developer friends

## 🙏 Acknowledgments

Thank you to everyone who provided feedback, tested early versions, and contributed to making ASTRIXA possible.

---

**Get started now:**

```bash
curl -fsSL https://astrixa.org/install | sh
```

**Questions?** Open an issue or start a discussion on GitHub.

**Happy coding with ASTRIXA! 🚀**

---

*"One language for Web, Web3, and AI"*
