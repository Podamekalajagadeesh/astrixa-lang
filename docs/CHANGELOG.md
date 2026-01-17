# Changelog

All notable changes to ASTRIXA will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-06

### 🎉 Initial Release

The first public release of ASTRIXA - a modern programming language for Web, Web3, and AI.

### Added

#### Core Language
- **Lexer**: Complete tokenization with 29+ token types
- **Parser**: Recursive descent parser for contracts, functions, and expressions
- **Interpreter**: Tree-walking interpreter with full expression evaluation
- **Bytecode Compiler**: AST-to-bytecode compilation
- **Stack VM**: Gas-metered virtual machine for deterministic execution
- **Module System**: Import other programs with circular dependency detection

#### Type System
- Primitive types: `int`, `float`, `string`, `bool`
- Blockchain types: `Address`, `U256`
- Collections: Arrays, Maps
- JSON object support
- Type inference for variable declarations

#### Control Flow
- `if`/`else if`/`else` statements
- `while` loops
- `for` loops with range syntax
- `break` and `continue` statements
- Function definitions with return values

#### Smart Contracts
- `contract` keyword for blockchain contracts
- `state` variables for persistent storage
- Gas model for deterministic execution
- Built-in blockchain context (`tx.sender`, `tx.value`, `block.number`, etc.)
- Transaction support with gas limits

#### Standard Library
- **web**: HTTP server framework with routing, middleware, JSON/HTML responses
- **web3**: Wallet management, contract interaction, blockchain queries
- **ai**: Sentiment analysis, text classification, entity extraction (deterministic)
- **json**: JSON parsing and serialization
- **crypto**: Cryptographic hash functions
- **fs**: File system operations
- **net**: HTTP client for making requests
- **io**: Input/output operations

#### Tooling
- **CLI**: Command-line interface for running ASTRIXA programs
- **LSP**: Language Server Protocol implementation for IDE support
  - Syntax highlighting
  - Code completion
  - Error diagnostics
  - Hover documentation
  - Go to definition
- **VS Code Extension**: Full IDE integration
  - Syntax highlighting
  - IntelliSense
  - Real-time error checking
  - Symbol navigation

#### Package Manager
- `astrixa init`: Initialize new projects
- `astrixa install`: Install packages
- `astrixa list`: List installed packages
- `astrixa.toml`: Package manifest
- `astrixa.lock`: Dependency lockfile
- Version resolution and conflict detection

#### WebAssembly Support
- WASM compilation for browser execution
- Sandboxed runtime for security
- Cross-platform compatibility
- Playground for browser-based development

#### Documentation
- **Getting Started**: `docs/intro.md`
- **Installation Guide**: `docs/installation.md`
- **Language Reference**: 
  - `docs/language/syntax.md`
  - `docs/language/types.md`
  - `docs/language/async.md`
  - `docs/language/errors.md`
  - `docs/language/modules.md`
- **Standard Library**:
  - `docs/stdlib/web.md`
  - `docs/stdlib/web3.md`
  - `docs/stdlib/ai.md`
- **Examples**: Token contracts, web servers, AI applications

#### Examples
- `examples/smart_contract_token.ax`: ERC-20 style token
- `examples/package_usage_example.ax`: Package system demo
- Web server examples
- AI sentiment analysis demos
- Math and crypto packages

### Technical Details

- **Language**: Rust (compiler, VM, LSP)
- **License**: MIT
- **Platforms**: Linux, macOS, Windows
- **Architecture**: x86_64, aarch64

### Known Limitations

- Async/await is planned but not yet implemented
- No floating point operations in smart contracts (by design)
- WASM execution is basic (no full stdlib support yet)
- Package registry is local-only (no remote registry yet)
- Error messages could be more helpful

### Breaking Changes

None (initial release)

### Security

- All smart contract operations are gas-metered
- WASM sandbox prevents unsafe operations
- Input validation on all stdlib functions

---

## [Unreleased]

### Planned for v0.2.0
- Async/await implementation
- Error handling with try/catch
- Optional types (`T?`)
- Result types (`Result<T, E>`)
- Pattern matching
- Multi-chain support (Solana, Cosmos)
- Remote package registry
- Improved error messages
- More stdlib functions

---

**Note**: This is the first public release. Future versions will maintain backward compatibility where possible, but breaking changes may be necessary for v0.x releases.

For the full roadmap, see [ROADMAP.md](ROADMAP.md).

[0.1.0]: https://github.com/Podamekalajagadeesh/astrixa-lang/releases/tag/v0.1.0
