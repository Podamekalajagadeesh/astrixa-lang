# Installation

Get ASTRIXA running on your machine in 5-10 minutes.

## Quick Start Options

### ⚡ Option 1: Try in Browser (No Installation)

The fastest way to explore ASTRIXA:

```
Open: examples/playground.html in your browser
```

Write and run code instantly without installing anything.

---

## Build from Source (Recommended)

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Git** - [Install Git](https://git-scm.com/)
- **Node.js 16+** - [Install Node.js](https://nodejs.org/) (needed to run compiled WASM)

### Installation Steps

1. **Clone the repository:**
```bash
git clone https://github.com/Podamekalajagadeesh/astrixa-lang.git
cd astrixa-lang
```

2. **Build the compiler:**
```bash
cd compiler
cargo build --release
cd ..
```

⏱️ *This takes 3-5 minutes on first build.*

3. **Verify installation:**
```bash
./compiler/target/release/astrixa --version
# Output: astrixa 0.1.0
```

4. **Compile and run your first program:**
```bash
./compiler/target/release/astrixa examples/hello.ax -o examples/hello.wasm
node runtime/run.js examples/hello.wasm
```

**Expected Output:**
```
🚀 ASTRIXA Runtime - Executing WASM
Hello, ASTRIXA!
Welcome to Web3 Smart Contracts with AI
✅ Program completed (exit code: 0)
```

---

## IDE Support

### Visual Studio Code

Install the official ASTRIXA VS Code extension for syntax highlighting and code completion:

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "ASTRIXA"
4. Click Install

**Features:**
- Syntax highlighting
- Code completion  
- Error diagnostics
- Hover documentation

### Language Server Protocol (LSP)

For other LSP-compatible editors, build the language server:

```bash
cd lsp
cargo build --release
```

Then configure your editor to use the LSP binary at `lsp/target/release/astrixa-lsp`.

---

## Creating Your First Project

Initialize a new ASTRIXA project:

```bash
mkdir my-first-app
cd my-first-app
```

Create `main.ax`:
```astrixa
fn main {
  print("Hello from ASTRIXA!")
}
```

Compile and run:
```bash
../astrixa main.ax -o main.wasm
node ../runtime/run.js main.wasm
```
