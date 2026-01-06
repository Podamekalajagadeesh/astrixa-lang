# ASTRIXA ROADMAP

**Vision: The first language for AI, Web3, and the decentralized web.**

---

## Release Philosophy

- **Stability over features** - Working code today works tomorrow
- **Predictable releases** - Minor version every 6 weeks
- **Community-driven** - RFCs shape the language
- **No surprises** - All changes documented and telegraphed

---

## Version 0.1 - Foundation (Q4 2024) ✅

**Status:** COMPLETE

### Core Language
- ✅ Lexer and parser
- ✅ AST representation
- ✅ Type system (basic)
- ✅ Interpreter
- ✅ Bytecode compiler
- ✅ Virtual machine
- ✅ Gas metering model

### Standard Library
- ✅ Basic I/O (`io.ax`)
- ✅ File system (`fs.ax`)
- ✅ JSON handling (`json.ax`)
- ✅ Networking (`net.ax`)
- ✅ Cryptography (`crypto.ax`)

### CLI
- ✅ `astrixa run` - Execute programs
- ✅ `astrixa init` - Initialize projects
- ✅ Package manager basics

---

## Version 0.2 - Developer Experience (Q1 2025) ✅

**Status:** COMPLETE

### IDE Support
- ✅ Language Server Protocol (LSP)
- ✅ VS Code extension
- ✅ Syntax highlighting
- ✅ Autocomplete (300+ completions)
- ✅ Hover documentation (80+ docs)
- ✅ Error diagnostics
- ✅ Go-to-definition
- ✅ Find references

### Package Manager
- ✅ Package registry structure
- ✅ `astrixa install` command
- ✅ Dependency resolution
- ✅ `astrixa.toml` manifest

### Documentation
- ✅ Language reference
- ✅ Standard library docs
- ✅ Tutorial and examples

---

## Version 0.3 - Web & Web3 (Q1 2025) ✅

**Status:** COMPLETE

### Web3 Support
- ✅ Native Web3 standard library (`web3.ax`)
- ✅ Wallet management
- ✅ ETH operations (balance, send, transactions)
- ✅ Smart contract interaction
- ✅ ENS support
- ✅ Event listening
- ✅ Multi-chain support (Ethereum, Polygon, Arbitrum)

### Smart Contracts
- ✅ Contract syntax and compilation
- ✅ EVM bytecode generation
- ✅ Blockchain context (`msg`, `tx`, `chain`)
- ✅ Security restrictions (compile-time enforced)
- ✅ Gas-aware execution

### Web Server
- ✅ HTTP server (`web.ax`)
- ✅ Routing (GET, POST, PUT, DELETE)
- ✅ Middleware support
- ✅ JSON responses
- ✅ CORS and compression

### AI Integration
- ✅ AI runtime (`ai.ax`)
- ✅ Model loading and inference
- ✅ Deterministic AI for contracts
- ✅ AI + Web3 examples

### Compilation Targets
- ✅ Native binary (`--target=native`)
- ✅ Smart contract (`--target=contract`)
- ✅ WebAssembly (`--target=wasm`) - partial
- ✅ Web server (`--target=web`)

---

## Version 0.4 - Governance & Stability (Q2 2025) 🔄

**Status:** IN PROGRESS

### Governance
- ✅ GOVERNANCE.md - governance structure
- ✅ RFC process established
- ✅ Foundational RFCs (0001-0003)
- 🔄 Core team formation
- 🔄 Community channels (Discord, forum)

### Stability
- 🔄 Comprehensive test suite (>80% coverage)
- 🔄 Benchmark suite
- 🔄 Performance regression testing
- 🔄 Security audit (first pass)

### Language Features
- 🔄 Error handling (try/catch)
- 🔄 Pattern matching
- 🔄 Destructuring
- 🔄 Traits/interfaces

### Tooling
- 🔄 Debugger support
- 🔄 Profiler
- 🔄 Code formatter
- 🔄 Linter

---

## Version 0.5 - Async & Concurrency (Q2 2025) 📝

**Status:** PLANNED

### Async/Await
- 📝 Async function syntax
- 📝 Promise/Future type
- 📝 Event loop runtime
- 📝 Async standard library

### Concurrency
- 📝 Channels for message passing
- 📝 Structured concurrency
- 📝 Timeout and cancellation

### Web Enhancements
- 📝 WebSocket support
- 📝 Server-sent events (SSE)
- 📝 HTTP/2 support

### Smart Contract Enhancements
- 📝 Upgradeable contracts pattern
- 📝 Multi-signature support
- 📝 Formal verification (basic)

---

## Version 0.6 - Multi-Chain (Q3 2025) 📝

**Status:** PLANNED

### Blockchain Support
- 📝 Solana support
- 📝 Aptos/Move support
- 📝 Cosmos SDK support
- 📝 Cross-chain messaging

### Chain Abstraction
- 📝 Universal wallet
- 📝 Cross-chain token transfers
- 📝 Multi-chain contract deployment

### DeFi Primitives
- 📝 AMM library
- 📝 Lending protocol library
- 📝 Oracle integration (Chainlink, Band)

---

## Version 0.7 - Advanced AI (Q3 2025) 📝

**Status:** PLANNED

### AI Features
- 📝 Fine-tuning support
- 📝 Custom model loading
- 📝 Multi-modal AI (text, image, audio)
- 📝 Streaming inference

### AI + Web3
- 📝 On-chain AI verification
- 📝 Decentralized AI training
- 📝 AI agent framework

---

## Version 0.8 - WASM & Frontend (Q4 2025) 📝

**Status:** PLANNED

### WASM
- 📝 Complete WASM compilation
- 📝 DOM bindings
- 📝 Component model

### Frontend Framework
- 📝 Reactive UI components
- 📝 State management
- 📝 Router
- 📝 Virtual DOM

### Full-Stack dApps
- 📝 Shared types across stack
- 📝 End-to-end type safety
- 📝 Hot reload

---

## Version 0.9 - Production Hardening (Q4 2025) 📝

**Status:** PLANNED

### Performance
- 📝 JIT compiler
- 📝 Ahead-of-time optimization
- 📝 Binary size reduction
- 📝 Startup time optimization

### Security
- 📝 Full security audit
- 📝 Fuzzing infrastructure
- 📝 Static analysis tools
- 📝 Dependency scanning

### Enterprise Features
- 📝 LTS support
- 📝 Commercial support option
- 📝 SLA guarantees

---

## Version 1.0 - Stable Release (Q1 2026) 🎯

**Status:** TARGET

### Stability Guarantees
- 🎯 No breaking changes (backward compatible)
- 🎯 6-month deprecation warnings
- 🎯 Automated migration tools
- 🎯 LTS releases (18 months support)

### Production Ready
- 🎯 1000+ test cases
- 🎯 <1% known bugs
- 🎯 Security audit passed
- 🎯 Performance benchmarks met

### Ecosystem
- 🎯 100+ packages in registry
- 🎯 1000+ active developers
- 🎯 10+ production contracts on mainnet
- 🎯 100+ stars on GitHub

### Documentation
- 🎯 Complete language reference
- 🎯 Comprehensive tutorials
- 🎯 Video courses
- 🎯 Books/guides

---

## Version 2.0+ - Future (2026+) 💭

**Status:** VISION

### Advanced Features
- 💭 Formal verification (complete)
- 💭 Zero-knowledge proofs
- 💭 Homomorphic encryption
- 💭 Quantum-resistant crypto

### Decentralized Infrastructure
- 💭 Decentralized package registry
- 💭 On-chain code repository
- 💭 Decentralized build system

### AI Evolution
- 💭 On-chain model training
- 💭 Federated learning
- 💭 AI DAOs

### Developer Tools
- 💭 Visual IDE
- 💭 No-code contract builder
- 💭 AI-assisted coding

---

## Release Schedule

### Minor Releases
- **Frequency:** Every 6 weeks
- **Scope:** New features, improvements
- **Breaking changes:** No

### Patch Releases
- **Frequency:** As needed
- **Scope:** Bug fixes only
- **Critical bugs:** Within 48 hours

### Major Releases
- **Frequency:** 12-18 months
- **Scope:** Breaking changes (rare)
- **Notice:** 6+ months in advance

---

## Deprecation Policy

### Timeline
1. **Announcement** - Feature marked deprecated (release N)
2. **Warning Period** - 6 months minimum (releases N+1, N+2, N+3)
3. **Removal** - Feature removed (release N+4 or next major)

### Procedure
1. RFC proposing deprecation
2. Community feedback (2 weeks)
3. Deprecation warnings in compiler
4. Migration guide published
5. Removal in major version

---

## Community Milestones

### Q1 2025
- ✅ 10 GitHub stars
- ✅ 5 contributors
- ✅ Documentation complete

### Q2 2025
- 🔄 100 GitHub stars
- 🔄 20 contributors
- 🔄 Discord community (100 members)

### Q3 2025
- 📝 500 GitHub stars
- 📝 50 contributors
- 📝 First community conference

### Q4 2025
- 📝 1000 GitHub stars
- 📝 100 contributors
- 📝 50 packages in registry

### 2026
- 🎯 5000 GitHub stars
- 🎯 500 contributors
- 🎯 1000 packages in registry
- 🎯 ASTRIXA Foundation established

---

## Success Metrics

### Technical
- [ ] 99.9% uptime in production
- [ ] <100ms compile time (small programs)
- [ ] 10% lower gas costs than Solidity
- [ ] Zero critical security vulnerabilities

### Adoption
- [ ] 10,000 downloads
- [ ] 1,000 active developers
- [ ] 100 production contracts
- [ ] 10 companies using ASTRIXA

### Community
- [ ] Healthy Discord community
- [ ] Active RFC participation
- [ ] Regular contributor meetings
- [ ] Annual conference

---

## Long-Term Vision (5 Years)

**By 2030, ASTRIXA should be:**
1. **The default language** for Web3 development
2. **Top 20** programming language (TIOBE Index)
3. **$1B+** value secured by ASTRIXA contracts
4. **10,000+** active developers
5. **ASTRIXA Foundation** self-sustaining

---

## Contributing to Roadmap

**Want to influence ASTRIXA's future?**
1. Submit RFCs for new features
2. Vote on priorities in Discord
3. Implement features from roadmap
4. Sponsor development (GitHub Sponsors)

---

## Changelog

- **2025-01-06:** Added v0.3 (Web & Web3) as complete
- **2025-01-06:** Added governance (v0.4) in progress
- **2024-12-15:** Initial roadmap published

---

## Questions?

- **Roadmap discussion:** Discord #roadmap
- **RFC proposals:** See [RFC_PROCESS.md](rfcs/RFC_PROCESS.md)
- **Feature requests:** GitHub Issues

---

**Legend:**
- ✅ Complete
- 🔄 In Progress
- 📝 Planned
- 🎯 Target
- 💭 Vision

---

**This roadmap is a living document. Priorities may shift based on community feedback.**

**Last Updated:** January 2025
