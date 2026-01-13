# ASTRIXA Package Manager

The ASTRIXA package manager transforms ASTRIXA from a language into a **platform**. Just like Python has pip, Node has npm, and Rust has cargo, ASTRIXA has `astrixa`.

## 🧠 Core Philosophy

ASTRIXA packages are designed to be:
- **Simple**: Easy to create, install, and use
- **Secure**: Checksums and verified releases
- **Deterministic**: Exact versions, no surprises
- **Web3 & AI aware**: Built for the future

**No chaos. No dependency hell.**

## 🚀 Quick Start

### Initialize a New Project

```bash
astrixa init my-project
```

This creates:
- `astrixa.toml` - Project manifest
- `src/main.ax` - Entry point
- Project structure

### Install Packages

```bash
astrixa install math
astrixa install ai-tools
```

Packages are installed to `~/.astrixa/packages/`

### List Installed Packages

```bash
astrixa list
```

### Run Your Project

```bash
astrixa run src/main.ax
```

## 📦 Package Structure

Every ASTRIXA package follows this structure:

```
math/
├── astrixa.toml
└── src/
    └── index.ax
```

### astrixa.toml

```toml
name = "math"
version = "0.1.0"
description = "Basic math utilities"

[dependencies]
# Package dependencies go here
```

### src/index.ax

```ax
export fn add(a: int, b: int) -> int {
    return a + b;
}
```

## 🔧 Using Packages

Import packages in your ASTRIXA code:

```ax
import "math"

fn main() {
    let result = add(10, 5);
    print("10 + 5 =", result);
}
```

## 🔒 Dependency Resolution

ASTRIXA uses **deterministic builds** for safety and reliability:

- **Exact versions only**: No `^` or `~` version ranges
- **No silent upgrades**: Versions are locked
- **Lockfile generated**: `astrixa.lock` tracks all dependencies

### astrixa.lock

```toml
[packages]
math = "0.1.0"
ai-tools = "0.3.2"
```

This is **critical for Web3 safety** - smart contracts and blockchain apps need absolute reproducibility.

## 🌐 Package Registry

### Current (MVP)
- Git-based registry
- Public packages
- Manual installation

### Future
- `registry.astrixa.org` - Central package registry
- Signed releases with cryptographic verification
- Private packages
- Package search and discovery
- Automated security scanning

## 📚 CLI Reference

### `astrixa init [project-name]`
Initialize a new ASTRIXA project with standard structure.

**Example:**
```bash
astrixa init my-web3-app
```

### `astrixa install <package> [version]`
Install a package from the registry.

**Examples:**
```bash
astrixa install math
astrixa install ai-tools 0.3.2
```

### `astrixa list`
List all installed packages with versions and descriptions.

### `astrixa run <file.ax> [--vm|--interp]`
Run an ASTRIXA file.

**Options:**
- `--vm` - Use bytecode VM (default, faster)
- `--interp` - Use interpreter (slower, better debugging)

**Example:**
```bash
astrixa run src/main.ax --vm
```

## 🎯 Creating Your Own Package

1. **Create package structure:**
   ```bash
   mkdir my-package
   cd my-package
   mkdir src
   ```

2. **Create astrixa.toml:**
   ```toml
   name = "my-package"
   version = "0.1.0"
   description = "My awesome package"
   ```

3. **Create src/index.ax:**
   ```ax
   export fn hello() -> string {
       return "Hello from my-package!";
   }
   ```

4. **Test locally:**
   ```bash
   # Copy to packages directory
   mkdir -p ~/.astrixa/packages/my-package
   cp -r . ~/.astrixa/packages/my-package
   ```

5. **Use in another project:**
   ```ax
   import "my-package"
   
   fn main() {
       print(hello());
   }
   ```

## 🔐 Security Best Practices

1. **Always review package code** before installation
2. **Pin exact versions** in production
3. **Verify checksums** (automatic in future)
4. **Use lockfile** in version control
5. **Audit dependencies** regularly

## 🌟 Package Ecosystem

The package manager enables:

✅ **Sharing code** across projects
✅ **Growing community** of developers
✅ **Supporting frameworks** and tools
✅ **Building AI + Web3 tooling**
✅ **Scaling adoption** of ASTRIXA

### Popular Package Categories (Coming Soon)

- **ai-tools** - AI/ML utilities
- **web3-sdk** - Blockchain interactions
- **math** - Mathematical functions
- **crypto** - Cryptography
- **http** - HTTP client/server
- **json** - JSON parsing
- **testing** - Test frameworks
- **cli** - CLI utilities

## 💡 Strategic Insight

> Languages don't win by syntax. They win by **ecosystem**.

The package manager is the foundation for:
- **AI libraries** - Integrate cutting-edge AI
- **Web3 SDKs** - Connect to blockchains
- **Frameworks** - Build full applications
- **Tooling** - Developer productivity
- **Companies** - Production-ready solutions

## 🛣️ Roadmap

### Phase 1: MVP (Current)
- ✅ Basic package structure
- ✅ CLI commands (init, install, list, run)
- ✅ Local package storage
- ✅ Import resolution
- ✅ Lockfile generation

### Phase 2: Registry
- 🔲 Central package registry
- 🔲 Web interface
- 🔲 Package search
- 🔲 User accounts
- 🔲 Publishing workflow

### Phase 3: Security
- 🔲 Cryptographic signatures
- 🔲 Checksum verification
- 🔲 Security scanning
- 🔲 Vulnerability database
- 🔲 Audit logs

### Phase 4: Advanced Features
- 🔲 Semantic versioning
- 🔲 Dependency graphs
- 🔲 Private packages
- 🔲 Organization packages
- 🔲 Package analytics

## 📖 Examples

See the `examples/` directory for complete examples:
- `examples/math-package/` - Math utilities package
- `examples/ai-tools-package/` - AI tools package
- `examples/package_usage_example.ax` - Using packages

## 🤝 Contributing

To contribute a package to the ASTRIXA ecosystem:

1. Follow the package structure guidelines
2. Add comprehensive tests
3. Document all exported functions
4. Submit to registry (coming soon)
5. Join the ASTRIXA community

## ❓ FAQ

**Q: Where are packages stored?**
A: `~/.astrixa/packages/`

**Q: Can I use relative imports?**
A: Yes! Use `./module` for local files, `module` for installed packages.

**Q: How do I update a package?**
A: Currently, reinstall with `astrixa install <package>`. Version management coming soon.

**Q: Can I create private packages?**
A: Yes, for local use. Private registry packages coming in Phase 3.

**Q: What about transitive dependencies?**
A: Full dependency resolution coming in Phase 2.

---

**Built with ❤️ for the ASTRIXA community**

For more information, visit the [ASTRIXA documentation](README.md).
