# ASTRIXA DOCUMENTATION INDEX

## 🚀 Getting Started

1. **[../README.md](../README.md)** - Main repository introduction
2. **[intro.md](intro.md)** - Introduction to ASTRIXA (10-minute primer)
3. **[installation.md](installation.md)** - Installation and setup instructions
4. **[../EXAMPLES.md](../EXAMPLES.md)** - Code examples with outputs

---

## 📚 Core Documentation

### ⭐ New: Feature Status & Type System (Start Here!)
- **[STATUS_REFERENCE.md](STATUS_REFERENCE.md)** - **What works, what doesn't** (feature status checklist)
- **[TYPE_SYSTEM_CONSOLIDATED.md](TYPE_SYSTEM_CONSOLIDATED.md)** - **Complete type system guide** (replaces fragmented docs)

### Type System (Detailed References)
- **[../README_TYPE_SYSTEM.md](../README_TYPE_SYSTEM.md)** - Type system documentation guide
- **[../TYPE_SYSTEM_REFERENCE.md](../TYPE_SYSTEM_REFERENCE.md)** - Quick lookup of type rules
- **[../TYPE_SYSTEM_ARCHITECTURE.md](../TYPE_SYSTEM_ARCHITECTURE.md)** - Implementation internals
- **[../TYPE_SYSTEM_TESTING.md](../TYPE_SYSTEM_TESTING.md)** - Type system test guide

### Web3 & Blockchain

### AI Features
- **[AI_PRIMITIVES.md](AI_PRIMITIVES.md)** - AI primitive operations
- **[../docs/stdlib/ai.md](stdlib/ai.md)** - AI standard library reference

### Language Features
- **[../TYPE_SYSTEM.md](../TYPE_SYSTEM.md)** - Complete type system
- **[language/syntax.md](language/syntax.md)** - Language syntax reference
- **[language/types.md](language/types.md)** - Type declarations and usage
- **[language/async.md](language/async.md)** - Asynchronous programming
- **[language/modules.md](language/modules.md)** - Module system
- **[language/errors.md](language/errors.md)** - Error handling

### Language Server Protocol (LSP)
- **[LSP_QUICKSTART.md](LSP_QUICKSTART.md)** - Quick start with LSP
- **[../lsp/LSP_GUIDE.md](../lsp/LSP_GUIDE.md)** - Comprehensive LSP guide
- **[../lsp/README.md](../lsp/README.md)** - LSP implementation details

### Standard Library
- **[stdlib/ai.md](stdlib/ai.md)** - AI operations
- **[stdlib/web3.md](stdlib/web3.md)** - Web3 operations
- **[stdlib/web.md](stdlib/web.md)** - Web server framework
- **[STDLIB_QUICKSTART.md](STDLIB_QUICKSTART.md)** - Quick start guide
- **[../stdlib/](../stdlib/)** - Standard library source code

### Package Manager
- **[PACKAGE_MANAGER.md](PACKAGE_MANAGER.md)** - Package manager guide
- **[PACKAGE_MANAGER_TUTORIAL.md](PACKAGE_MANAGER_TUTORIAL.md)** - Step-by-step tutorial

### CLI Reference
- **[CLI_REFERENCE.md](CLI_REFERENCE.md)** - Command-line interface reference

---

## 💻 Examples

### Smart Contracts
- **[examples/smart_contract_token.ax](examples/smart_contract_token.ax)** - Complete smart contract examples (NEW - 350+ lines)
  - ERC20 Token implementation
  - AMM DEX contract
  - AI + Web3 DAO contract

### Web3 Examples
- **[wallet_contract.ax](wallet_contract.ax)** - Wallet contract example
- **[contract_with_ai_advanced.ax](contract_with_ai_advanced.ax)** - AI + blockchain integration
- **[defi_portfolio_demo.ax](defi_portfolio_demo.ax)** - DeFi portfolio management
- **[web3_test.ax](web3_test.ax)** - Web3 tests

### AI Examples
- **[ai_test.ax](ai_test.ax)** - AI operation tests

### Standard Library Examples
- **[stdlib_test.ax](stdlib_test.ax)** - Standard library tests
- **[std_test.ax](std_test.ax)** - More stdlib tests

### Package Examples
- **[examples/package_usage_example.ax](examples/package_usage_example.ax)** - Package usage
- **[examples/math-package/](examples/math-package/)** - Math package
- **[examples/ai-tools-package/](examples/ai-tools-package/)** - AI tools package

### Other Examples
- **[main.ax](main.ax)** - Main example
- **[math.ax](math.ax)** - Math operations
- **[gas_test.ax](gas_test.ax)** - Gas model testing

---

## 🏗️ Architecture & Design

### Design Documents
- **[design/syntax.md](design/syntax.md)** - Language syntax
- **[design/types.md](design/types.md)** - Type system
- **[design/runtime.md](design/runtime.md)** - Runtime architecture

### Vision & Principles
- **[docs/vision.md](docs/vision.md)** - Project vision
- **[docs/principles.md](docs/principles.md)** - Design principles
- **[docs/WASM_RUNTIME.md](docs/WASM_RUNTIME.md)** - WebAssembly runtime

### Technical Specifications
- **[GAS_MODEL.md](GAS_MODEL.md)** - Gas metering model
- **[FILE_MANIFEST.md](FILE_MANIFEST.md)** - File structure manifest

---

## 📋 Status & Progress

### Completion Reports
- **[COMPLETION_REPORT.md](COMPLETION_REPORT.md)** - Overall completion report
- **[COMPLETION_SUMMARY.md](COMPLETION_SUMMARY.md)** - Summary of completed work
- **[IMPLEMENTATION_CHECKLIST.md](IMPLEMENTATION_CHECKLIST.md)** - Implementation checklist

### Strategy
- **[ECOSYSTEM_STRATEGY.md](ECOSYSTEM_STRATEGY.md)** - Ecosystem strategy

---

## 🏛️ Governance & Community

### Governance Documents
- **[GOVERNANCE.md](GOVERNANCE.md)** - Complete governance structure (NEW - 800 lines)
  - Three-phase evolution (BDFL → Core Team → Foundation)
  - Decision-making processes
  - Release procedures
  - Code of conduct framework
  - Security policy
  - Conflict resolution

- **[rfcs/RFC_PROCESS.md](rfcs/RFC_PROCESS.md)** - RFC system (NEW - 600 lines)
  - RFC lifecycle (Draft → Review → Comment → FCP → Decision → Implementation)
  - Templates and guidelines
  - Numbering and types
  - Best practices

- **[ROADMAP.md](ROADMAP.md)** - Product roadmap (NEW - 600 lines)
  - Versioned milestones (v0.1 → v2.0+)
  - Release schedule
  - Feature timeline
  - Success metrics

### Active RFCs
- **[rfcs/0001-language-vision.md](rfcs/0001-language-vision.md)** - Design principles (Accepted)
  - 10 core principles
  - Stability over features
  - Explicit over implicit
  - Safety without ceremony

- **[rfcs/0002-async-model.md](rfcs/0002-async-model.md)** - Async/await proposal (Draft)
  - async fn syntax
  - Promise type
  - Event loop runtime
  - Target: v0.5.0

- **[rfcs/0003-smart-contract-subset.md](rfcs/0003-smart-contract-subset.md)** - Contract restrictions (Accepted)
  - Security restrictions
  - Forbidden operations
  - Deterministic execution
  - Implemented: v0.5.0

### Community Standards
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guide (NEW - 500 lines)
  - Bug reports
  - Feature requests
  - Code contributions
  - Development setup
  - Pull request process

- **[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)** - Community standards (NEW - 400 lines)
  - Expected behavior
  - Unacceptable behavior
  - Enforcement process
  - Reporting violations

- **[SECURITY.md](SECURITY.md)** - Security policy (NEW - 500 lines)
  - Vulnerability reporting
  - Response timeline
  - Disclosure policy
  - Bug bounty (coming v1.0)

### Legal
- **[LICENSE](LICENSE)** - MIT license
  - Business-friendly
  - Patent protection
  - Contributor rights

### GitHub Templates
- **[.github/ISSUE_TEMPLATE/rfc.md](.github/ISSUE_TEMPLATE/rfc.md)** - RFC submission template
- **[.github/ISSUE_TEMPLATE/bug_report.md](.github/ISSUE_TEMPLATE/bug_report.md)** - Bug report template
- **[.github/ISSUE_TEMPLATE/feature_request.md](.github/ISSUE_TEMPLATE/feature_request.md)** - Feature request template

---

## 🔧 Development

### Compiler
- **[compiler/](compiler/)** - Compiler source code
  - `src/main.rs` - CLI entry point (MODIFIED - build command)
  - `src/compiler.rs` - Bytecode compiler (MODIFIED - multi-target)
  - `src/lexer.rs` - Lexical analyzer (ENHANCED - line/column tracking)
  - `src/parser.rs` - Parser (ENHANCED - Result-based errors)
  - `src/error.rs` - Error type definition (NEW - Step 36)
  - `src/diagnostics.rs` - Error pretty-printing (NEW - Step 36)
  - `src/ast.rs` - Abstract syntax tree
  - `src/interpreter.rs` - Interpreter
  - `src/vm.rs` - Virtual machine
  - `src/bytecode.rs` - Bytecode definitions
  - `src/gas.rs` - Gas metering
  - `src/ai_runtime.rs` - AI runtime
  - `src/package_manager.rs` - Package manager
  - `src/wasm.rs` - WebAssembly support

#### Step 36: Error Diagnostics (COMPLETE ✅)
- **[STEP_36_COMPLETION_README.md](STEP_36_COMPLETION_README.md)** - **START HERE** - Overview and index
- **[STEP_36_ERROR_DIAGNOSTICS_COMPLETE.md](STEP_36_ERROR_DIAGNOSTICS_COMPLETE.md)** - Full implementation guide
- **[STEP_36_TRANSFORMATION_VISUAL.md](STEP_36_TRANSFORMATION_VISUAL.md)** - Before/after comparison
- **[STEP_36_TESTING_GUIDE.md](STEP_36_TESTING_GUIDE.md)** - Testing strategies
- **[STEP_36_DELIVERY_COMPLETE.md](STEP_36_DELIVERY_COMPLETE.md)** - Delivery summary
- **[STEP_36_VISUAL_SUMMARY.md](STEP_36_VISUAL_SUMMARY.md)** - Visual showcase
- **[STEP_36_QUICK_REFERENCE.md](STEP_36_QUICK_REFERENCE.md)** - Quick lookup guide
- **[compiler/examples/error_demo.rs](compiler/examples/error_demo.rs)** - Live demo
- **[test_step36_errors.sh](test_step36_errors.sh)** - Test script

#### Step 37: Intermediate Representation (COMPLETE ✅)
- **[STEP_37_SUMMARY.md](STEP_37_SUMMARY.md)** - **START HERE** - Complete summary
- **[STEP_37_IR_COMPLETE.md](STEP_37_IR_COMPLETE.md)** - Full implementation guide
- **[STEP_37_VISUAL_ARCHITECTURE.md](STEP_37_VISUAL_ARCHITECTURE.md)** - Visual architecture diagrams
- **[STEP_37_QUICK_REFERENCE.md](STEP_37_QUICK_REFERENCE.md)** - Quick lookup guide
- **[STEP_37_BANNER.md](STEP_37_BANNER.md)** - Visual banner
- **[compiler/src/ir.rs](compiler/src/ir.rs)** - IR type definitions
- **[compiler/src/lowering.rs](compiler/src/lowering.rs)** - AST to IR lowering

### Language Server
- **[lsp/](lsp/)** - LSP implementation
  - `src/main.rs` - LSP server
  - `src/diagnostics.rs` - Error diagnostics (ENHANCED)
  - `src/completion.rs` - Code completion (REWRITTEN)
  - `src/hover.rs` - Hover documentation (REWRITTEN)
  - `src/symbols.rs` - Symbol provider

### VS Code Extension
- **[astrixa-vscode/](astrixa-vscode/)** - VS Code extension
  - `src/extension.ts` - Extension entry point (ENHANCED)
  - `package.json` - Extension manifest (UPDATED)
  - `syntaxes/astrixa.tmLanguage.json` - Syntax highlighting
  - `language-configuration.json` - Language configuration

### Build Scripts
- **[build_wasm.sh](build_wasm.sh)** - Build WebAssembly

---

## 📖 Reading Order

### For Beginners
1. Start with [WHY_ASTRIXA.md](WHY_ASTRIXA.md) - Understand why ASTRIXA exists
2. Read [START_HERE.md](START_HERE.md) - Get overview
3. Follow [WEB3_QUICK_REFERENCE.md](WEB3_QUICK_REFERENCE.md) - Quick start
4. Try [examples/smart_contract_token.ax](examples/smart_contract_token.ax) - See it in action

### For Web3 Developers
1. [WHY_ASTRIXA.md](WHY_ASTRIXA.md) - See comparison with Solidity/JS
2. [WEB3_COMPLETE_GUIDE.md](WEB3_COMPLETE_GUIDE.md) - Learn everything
3. [examples/smart_contract_token.ax](examples/smart_contract_token.ax) - Study examples
4. [WEB3_QUICK_REFERENCE.md](WEB3_QUICK_REFERENCE.md) - Keep as reference

### For AI/ML Engineers
1. [AI_COMPLETE_GUIDE.md](AI_COMPLETE_GUIDE.md) - AI features
2. [contract_with_ai_advanced.ax](contract_with_ai_advanced.ax) - AI + Web3 integration
3. [AI_PRIMITIVES.md](AI_PRIMITIVES.md) - Deep dive into AI

### For Tool Developers
1. [LSP_COMPLETE.md](LSP_COMPLETE.md) - LSP implementation
2. [lsp/LSP_GUIDE.md](lsp/LSP_GUIDE.md) - Detailed guide
3. [PACKAGE_MANAGER_COMPLETE.md](PACKAGE_MANAGER_COMPLETE.md) - Package system

### For Language Designers
1. [design/syntax.md](design/syntax.md) - Language syntax
2. [design/types.md](design/types.md) - Type system
3. [design/runtime.md](design/runtime.md) - Runtime
4. [GAS_MODEL.md](GAS_MODEL.md) - Gas model

### For Contributors
1. [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute
2. [ROADMAP.md](ROADMAP.md) - What's planned
3. [rfcs/RFC_PROCESS.md](rfcs/RFC_PROCESS.md) - Propose changes
4. [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) - Community standards

---

## 🎯 Quick Reference by Feature

### Web3 Operations
- **Guide:** [WEB3_COMPLETE_GUIDE.md](WEB3_COMPLETE_GUIDE.md)
- **Quick Ref:** [WEB3_QUICK_REFERENCE.md](WEB3_QUICK_REFERENCE.md)
- **Implementation:** [stdlib/web3.ax](stdlib/web3.ax)
- **Examples:** [examples/smart_contract_token.ax](examples/smart_contract_token.ax)

### Smart Contracts
- **Guide:** [WEB3_COMPLETE_GUIDE.md](WEB3_COMPLETE_GUIDE.md) (Smart Contracts section)
- **Examples:** [examples/smart_contract_token.ax](examples/smart_contract_token.ax)
- **Context:** [WEB3_QUICK_REFERENCE.md](WEB3_QUICK_REFERENCE.md) (Blockchain Context)

### AI Integration
- **Guide:** [AI_COMPLETE_GUIDE.md](AI_COMPLETE_GUIDE.md)
- **Primitives:** [AI_PRIMITIVES.md](AI_PRIMITIVES.md)
- **Implementation:** [stdlib/ai.ax](stdlib/ai.ax)
- **Examples:** [contract_with_ai_advanced.ax](contract_with_ai_advanced.ax)

### IDE Support
- **Guide:** [LSP_COMPLETE.md](LSP_COMPLETE.md)
- **Quick Start:** [LSP_QUICKSTART.md](LSP_QUICKSTART.md)
- **Detailed:** [lsp/LSP_GUIDE.md](lsp/LSP_GUIDE.md)

### Package Management
- **Complete:** [PACKAGE_MANAGER_COMPLETE.md](PACKAGE_MANAGER_COMPLETE.md)
- **Tutorial:** [PACKAGE_MANAGER_TUTORIAL.md](PACKAGE_MANAGER_TUTORIAL.md)
- **Usage:** [PACKAGE_MANAGER_USAGE.md](PACKAGE_MANAGER_USAGE.md)

### Standard Library
- **Reference:** [STDLIB_COMPLETE_REFERENCE.md](STDLIB_COMPLETE_REFERENCE.md)
- **Quick Start:** [STDLIB_QUICKSTART.md](STDLIB_QUICKSTART.md)
- **Source:** [stdlib/](stdlib/)

### Governance & Community
- **Governance:** [GOVERNANCE.md](GOVERNANCE.md)
- **RFC Process:** [rfcs/RFC_PROCESS.md](rfcs/RFC_PROCESS.md)
- **Roadmap:** [ROADMAP.md](ROADMAP.md)
- **Contributing:** [CONTRIBUTING.md](CONTRIBUTING.md)
- **Code of Conduct:** [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- **Security:** [SECURITY.md](SECURITY.md)
- **License:** [LICENSE](LICENSE)

---

## 📊 Statistics

### Documentation
- **Total Files:** 60+
- **Total Lines:** 20,000+
- **Languages:** ASTRIXA, Rust, TypeScript, Markdown
- **Examples:** 15+

### Code
- **Compiler:** 3,000+ lines (Rust)
- **LSP:** 2,000+ lines (Rust)
- **VS Code Extension:** 500+ lines (TypeScript)
- **Standard Library:** 2,000+ lines (ASTRIXA)

### New in This Update
- **stdlib/web3.ax:** 500+ lines
- **WEB3_COMPLETE_GUIDE.md:** 2,000+ lines
- **WEB3_QUICK_REFERENCE.md:** 500+ lines
- **examples/smart_contract_token.ax:** 350+ lines
- **WHY_ASTRIXA.md:** 800+ lines
- **GOVERNANCE.md:** 800+ lines (NEW)
- **RFC system:** 1,500+ lines (NEW)
- **Community docs:** 1,400+ lines (NEW)
- **Compiler enhancements:** Multi-target support
- **Total New Content:** 7,850+ lines

---

## 🌟 Key Features

### ✅ Implemented
- Native Web3 support
- Smart contract compilation
- AI integration
- LSP with autocomplete, diagnostics, hover
- VS Code extension
- Package manager
- Multi-target compilation
- Type system
- Gas metering
- WASM support (partial)
- Comprehensive documentation

### 🔄 In Progress
- Multi-chain support (Solana, Aptos)
- Enhanced WASM frontend
- Package registry
- Formal verification
- Security auditing tools
- Foundation establishment (v1.0+)

### 🎯 Planned
- Account abstraction (EIP-4337)
- Built-in indexer
- IPFS integration
- Cross-chain messaging
- Enterprise features

---

## 🔗 External Links

- **Website:** https://astrixa.dev (coming soon)
- **GitHub:** https://github.com/astrixa-lang/astrixa
- **Discord:** https://discord.gg/astrixa (coming soon)
- **Twitter:** @astrixalang (coming soon)
- **Documentation:** https://docs.astrixa.dev (coming soon)

---

## 💡 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) (to be created) for:
- Code style guidelines
- Pull request process
- Issue reporting
- Development setup

---

## 📄 License

See [LICENSE](LICENSE) for licensing information.

---

## 🎉 Conclusion

ASTRIXA is the **FIRST programming language** with:
- ✅ Native Web3 support
- ✅ Native AI integration
- ✅ One language for everything (contracts, backend, frontend, AI)
- ✅ Type-safe blockchain operations
- ✅ Security enforced at language level
- ✅ World-class developer experience

**The future of Web3 development is here.**

---

## Quick Navigation

| I want to... | Go to... |
|-------------|----------|
| Understand why ASTRIXA | [WHY_ASTRIXA.md](WHY_ASTRIXA.md) |
| Get started quickly | [START_HERE.md](START_HERE.md) |
| Learn Web3 features | [WEB3_COMPLETE_GUIDE.md](WEB3_COMPLETE_GUIDE.md) |
| Quick Web3 reference | [WEB3_QUICK_REFERENCE.md](WEB3_QUICK_REFERENCE.md) |
| Write smart contracts | [examples/smart_contract_token.ax](examples/smart_contract_token.ax) |
| Learn AI features | [AI_COMPLETE_GUIDE.md](AI_COMPLETE_GUIDE.md) |
| Use the LSP | [LSP_COMPLETE.md](LSP_COMPLETE.md) |
| Manage packages | [PACKAGE_MANAGER_COMPLETE.md](PACKAGE_MANAGER_COMPLETE.md) |
| See all stdlib functions | [STDLIB_COMPLETE_REFERENCE.md](STDLIB_COMPLETE_REFERENCE.md) |
| Check implementation status | [COMPLETION_REPORT.md](COMPLETION_REPORT.md) |
| Learn about error diagnostics | [STEP_36_ERROR_DIAGNOSTICS_COMPLETE.md](STEP_36_ERROR_DIAGNOSTICS_COMPLETE.md) |
| Learn about IR | [STEP_37_SUMMARY.md](STEP_37_SUMMARY.md) |
| Contribute to ASTRIXA | [CONTRIBUTING.md](CONTRIBUTING.md) |
| Propose language changes | [rfcs/RFC_PROCESS.md](rfcs/RFC_PROCESS.md) |
| Report security issue | [SECURITY.md](SECURITY.md) |
| See roadmap | [ROADMAP.md](ROADMAP.md) |

---

**Last Updated:** January 2025  
**Version:** 0.4.0 (Governance Phase)

---

**Built with ❤️ by the ASTRIXA team.**
