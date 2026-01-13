# Astrixa Lang - Project Structure Reference

## Current Structure

```
astrixa-lang/
│
├── docs/                      # All documentation
│   ├── AI_PRIMITIVES.md              (when moved)
│   ├── CHANGELOG.md                  (when moved)
│   ├── CLI_REFERENCE.md              (when moved)
│   ├── CODE_OF_CONDUCT.md            (when moved)
│   ├── CONTRIBUTING.md               (when moved)
│   ├── DOCUMENTATION_INDEX.md         (when moved)
│   ├── GAS_MODEL.md                  (when moved)
│   ├── GOVERNANCE.md                 (when moved)
│   ├── LSP_QUICKSTART.md             (when moved)
│   ├── PACKAGE_MANAGER.md            (when moved)
│   ├── PACKAGE_MANAGER_TUTORIAL.md   (when moved)
│   ├── RELEASE_NOTES_v0.1.0.md       (when moved)
│   ├── ROADMAP.md                    (when moved)
│   ├── SECURITY.md                   (when moved)
│   ├── STDLIB_QUICKSTART.md          (when moved)
│   ├── WASM_RUNTIME.md
│   ├── installation.md
│   ├── intro.md
│   ├── principles.md
│   ├── vision.md
│   ├── language/               # Language documentation
│   └── stdlib/                 # Standard library docs
│
├── compiler/                   # Astrixa compiler (Rust)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs              # Module exports
│   │   ├── main.rs             # CLI entry point
│   │   ├── lexer.rs            # Lexical analysis
│   │   ├── parser.rs           # Syntax analysis
│   │   ├── ast.rs              # Abstract syntax tree
│   │   ├── types.rs            # Type system
│   │   ├── typechecker.rs      # Type checking
│   │   ├── error.rs            # Error handling
│   │   ├── diagnostics.rs      # Diagnostic output
│   │   ├── ir.rs               # Intermediate representation
│   │   ├── lowering.rs         # AST → IR lowering
│   │   ├── bytecode.rs         # Bytecode generation
│   │   ├── codegen/            # Code generation
│   │   │   └── wasm.rs         # WASM backend
│   │   ├── interpreter.rs      # Interpreter
│   │   ├── vm.rs               # Virtual machine
│   │   ├── ai_runtime.rs       # AI runtime support
│   │   ├── gas.rs              # Gas metering
│   │   ├── stdlib.rs           # Standard library
│   │   ├── loader.rs           # Module loader
│   │   ├── compiler.rs         # Main compiler logic
│   │   ├── package_manager.rs  # Package management
│   │   ├── wasm.rs             # WASM utilities
│   │   ├── opt/                # Optimization passes
│   │   └── bin/                # Additional binaries
│   └── examples/               # Compiler examples
│       ├── error_demo.rs
│       └── runtime_test.rs
│
├── runtime/                    # WASM runtime
│   ├── README.md
│   ├── run.js                  # JavaScript runtime
│   ├── wasm_env.js             (when created)
│   └── test_simple.wat         # Test WASM
│
├── lsp/                        # Language Server Protocol
│   ├── Cargo.toml
│   ├── README.md
│   ├── LSP_GUIDE.md
│   └── src/
│
├── astrixa-cli/                # CLI tool
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── templates.rs
│   │   └── commands/
│
├── astrixa-vscode/             # VS Code extension
│   ├── package.json
│   ├── tsconfig.json
│   ├── src/
│   │   └── extension.ts
│   └── syntaxes/
│       └── astrixa.tmLanguage.json
│
├── stdlib/                     # Standard library (Astrixa code)
│   ├── ai.ax
│   ├── async.ax
│   ├── crypto.ax
│   ├── fs.ax
│   ├── io.ax
│   ├── json.ax
│   ├── net.ax
│   ├── web.ax
│   └── web3.ax
│
├── examples/                   # Example programs
│   ├── hello_runtime.ax
│   ├── math.ax
│   ├── contract_with_ai_advanced.ax
│   ├── defi_portfolio_demo.ax
│   ├── smart_contract_token.ax
│   ├── wallet_contract.ax
│   ├── package_usage_example.ax
│   ├── ai-tools-package/       # Example package
│   ├── math-package/           # Example package
│   ├── playground.html
│   └── wasm_playground.html
│
├── tests/                      # Test files
│   ├── ai_test.ax
│   ├── gas_test.ax
│   ├── stdlib_test.ax
│   ├── web3_test.ax
│   ├── test_variables.ax
│   └── (15+ more test files)
│
├── scripts/                    # Build and utility scripts
│   ├── build_wasm.sh
│   ├── install.sh
│   ├── test_modules.sh
│   ├── cleanup_project.sh      (when moved)
│   └── organize_structure.sh   (this script)
│
├── design/                     # Design documents
│   ├── runtime.md
│   ├── syntax.md
│   └── types.md
│
├── rfcs/                       # Request for Comments (proposals)
│   ├── 0001-language-vision.md
│   ├── 0002-async-model.md
│   ├── 0003-smart-contract-subset.md
│   └── RFC_PROCESS.md
│
├── .github/                    # GitHub workflows
├── .git/                       # Git repository
├── README.md                   # Main project README
├── LICENSE                     # License file
├── .gitignore                  # Git ignore rules
├── organize.sh                 # (cleanup script - remove after running)
└── organize_structure.sh       # (cleanup script - remove after running)
```

## TODOS - Files to Move

Run `bash scripts/organize_structure.sh` to automatically:

- [ ] Move 15 markdown files from root to `docs/`
- [ ] Move `cleanup_project.sh` to `scripts/`
- [ ] Remove `.cleanup_backup/` directory

## What Makes This Structure Intentional

✅ **Clear Separation of Concerns**
- `compiler/` - Language implementation
- `runtime/` - Execution environment
- `examples/` - Usage demonstrations
- `tests/` - Quality assurance
- `docs/` - All documentation consolidated
- `stdlib/` - Language standard library
- `lsp/` - Developer tooling
- `scripts/` - Build and maintenance
- `rfcs/` & `design/` - Planning and design

✅ **No Experimental or Random Files**
- All markdown documentation organized in `docs/`
- All scripts organized in `scripts/`
- No personal or temporary files at root level

✅ **Professional Repository Appearance**
- Only essential files at root: README.md, LICENSE, .gitignore
- Clear naming: each directory has a specific purpose
- Modular organization supports growth

## Next Steps

1. Run `bash scripts/organize_structure.sh` to complete the reorganization
2. Delete `organize.sh` and `organize_structure.sh` from root after cleanup
3. Commit changes with: `git add -A && git commit -m "refactor: reorganize project structure"`
