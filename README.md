# ASTRIXA

An experimental programming language designed for Web, Web3, and AI-native development.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![GitHub Stars](https://img.shields.io/github/stars/Podamekalajagadeesh/astrixa-lang?style=social)](https://github.com/Podamekalajagadeesh/astrixa-lang)

---

## Why ASTRIXA Exists

Modern developers building AI + Web + Blockchain apps face serious pain points:

- **Too much boilerplate** — JavaScript/Python for web, Solidity for smart contracts, Python/TensorFlow for AI
- **AI not first-class** — External APIs, complex integrations, inconsistent patterns
- **Web3 too complex** — Gas costs hidden, security vulnerabilities, steep learning curve
- **Unsafe runtimes** — No memory safety guarantees, runtime errors in production
- **Multiple deployment pipelines** — Different type systems, incompatible toolchains

ASTRIXA solves this: **One language, one type system, one deployment target → WebAssembly**

---

## Example

```astrixa
fn main {
  // AI is first-class
  let text = ai.generate("Explain ASTRIXA")
  print(text)
  
  // Web3 is native
  let sentiment = ai.classify("Great product!")
  if sentiment == "positive" {
    print("Transfer approved!")
  }
}
```

---

## Features

✅ **AI-native** — Built-in sentiment analysis, embeddings, classification (deterministic & safe)  
✅ **Web3-native** — Smart contracts, gas metering, blockchain primitives included  
✅ **WASM-based** — Compiles to WebAssembly, runs anywhere (browsers, servers, edge)  
✅ **Capability-secured** — Fine-grained permissions for filesystem, network, AI operations  
✅ **Memory-safe** — Type safety at compile time, no runtime errors  
✅ **Fast compilation** — Rust-powered compiler with optimized output

---

## Status

⚠️ **ASTRIXA is in early alpha (v0.1.0). APIs may change. Not recommended for production use yet.**

| Component | Status |
|-----------|--------|
| Core Language | ✅ Working |
| WebAssembly Compiler | ✅ Working |
| Standard Library | ✅ Working |
| Smart Contracts | ✅ Proof-of-Concept (not audited) |
| Real LLM APIs | 🚧 Planned |

---

## How to Run

### Prerequisites
- **Rust 1.70+** — [Install Rust](https://rustup.rs/)
- **Node.js 16+** — [Install Node.js](https://nodejs.org/)
- **Git** — [Install Git](https://git-scm.com/)

### Build and Run (Local)

```bash
# 1. Clone repository
git clone https://github.com/Podamekalajagadeesh/astrixa-lang.git
cd astrixa-lang

# 2. Build compiler (one-time, 3-5 minutes)
cd compiler
cargo build --release
cd ..

# 3. Verify installation
./compiler/target/release/astrixa --version

# 4. Run an example
node runtime/run.js examples/hello.wasm
```

**Expected output:**
```
🚀 ASTRIXA Runtime - Executing WASM
Hello, ASTRIXA!
Welcome to Web3 Smart Contracts with AI
✅ Program completed (exit code: 0)
```

### Quick Start (Browser Playground)

No installation needed — open `examples/playground.html` in your browser, write code, and click "Run" for instant feedback.

---

## More Examples

### AI Sentiment Analysis
```astrixa
fn main {
  print("[AI] Analyzing sentiment...")
  ai.generate("What is blockchain?")
  ai.classify("This is a positive review!")
}
```

### Smart Contract with AI
```astrixa
contract SmartTransfer {
  state balances: map<Address, U256>
  
  fn transfer(to: Address, amount: U256, reason: string) {
    // AI: Analyze transaction reason
    let sentiment = ai.sentiment(reason)
    
    // Smart contract: Check balance
    if balances[tx.sender] < amount {
      panic("Insufficient balance")
    }
    
    // Only proceed if reason has positive sentiment
    if sentiment == "positive" {
      balances[tx.sender] -= amount
      balances[to] += amount
      print("Transfer approved and recorded!")
    }
  }
}
```

---

## Documentation

- [Introduction](docs/intro.md) — Language overview
- [Syntax Guide](design/syntax.md) — Language syntax
- [Type System](docs/TYPE_SYSTEM.md) — Type safety
- [Standard Library](docs/STDLIB_QUICKSTART.md) — Built-in functions
- [Examples](EXAMPLES.md) — More examples
- [Documentation Index](docs/DOCUMENTATION_INDEX.md) — All documentation

---

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

---

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

*ASTRIXA: One language for Web, Web3, and AI* | v0.1.0 | [GitHub](https://github.com/Podamekalajagadeesh/astrixa-lang)

