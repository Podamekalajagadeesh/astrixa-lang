# 🛠️ ASTRIXA CLI Reference

Complete command reference for the ASTRIXA package manager and runtime.

---

## Table of Contents

1. [Project Commands](#project-commands)
2. [Package Commands](#package-commands)
3. [Runtime Commands](#runtime-commands)
4. [Advanced Usage](#advanced-usage)
5. [Configuration](#configuration)
6. [Troubleshooting](#troubleshooting)

---

## Project Commands

### `astrixa init`

Initialize a new ASTRIXA project.

**Usage:**
```bash
astrixa init [project-name]
```

**Arguments:**
- `project-name` (optional): Name of your project. Default: `my-project`

**What it does:**
- Creates directory structure
- Generates `astrixa.toml` manifest
- Creates `src/main.ax` template
- Initializes empty dependencies

**Examples:**
```bash
# Create a project named "my-app"
astrixa init my-app

# Create with default name
astrixa init

# Create in current directory
astrixa init .
```

**Output:**
```
✓ Initialized ASTRIXA project: my-app
✓ Created astrixa.toml
✓ Created src/main.ax
```

**Generated Files:**
```
astrixa.toml
├── name = "my-app"
├── version = "0.1.0"
├── description = "A new ASTRIXA project"
└── [dependencies]

src/
└── main.ax
    └── fn main() { print("Hello, ASTRIXA!"); }
```

---

## Package Commands

### `astrixa install`

Install a package from the registry.

**Usage:**
```bash
astrixa install <package> [version]
```

**Arguments:**
- `package` (required): Package name to install
- `version` (optional): Specific version. Default: `latest`

**What it does:**
1. Fetches package from registry
2. Verifies checksums
3. Stores in `~/.astrixa/packages/<package>/`
4. Updates `astrixa.lock`

**Examples:**
```bash
# Install latest version
astrixa install math

# Install specific version
astrixa install math 0.1.0

# Install multiple packages
astrixa install math
astrixa install ai-tools
astrixa install crypto-utils
```

**Output:**
```
📦 Installing math@latest...
✓ Installed math@latest
```

**Updates:**
```toml
# astrixa.lock is created/updated:
[packages]
math = "0.1.0"
```

**Environment:**
- Packages stored in: `~/.astrixa/packages/`
- Lockfile created: `./astrixa.lock`
- Manifest read: `astrixa.toml`

---

### `astrixa list`

List all installed packages.

**Usage:**
```bash
astrixa list
```

**What it does:**
- Reads `~/.astrixa/packages/`
- Shows package name, version, description
- Shows total count

**Examples:**
```bash
# List installed packages
astrixa list

# Output with formatting:
astrixa list --verbose

# JSON output:
astrixa list --json
```

**Output:**
```
📦 Installed ASTRIXA packages:

  math v0.1.0
    Basic math utilities for ASTRIXA

  ai-tools v0.1.0
    AI utilities for ASTRIXA

  crypto-utils v0.1.0
    Cryptocurrency utility functions

Total: 3 package(s)
```

**Options (Future):**
- `--verbose` - Show more details
- `--json` - Output as JSON
- `--sort` - Sort by name/version/date

---

### `astrixa remove`

Uninstall a package. (Planned feature)

**Usage:**
```bash
astrixa remove <package>
```

**Examples:**
```bash
# Remove a package
astrixa remove math

# Remove with confirmation
astrixa remove math --force
```

---

### `astrixa search`

Search for packages. (Planned feature)

**Usage:**
```bash
astrixa search <query>
```

**Examples:**
```bash
# Search for math packages
astrixa search math

# Search with filters
astrixa search --category defi
astrixa search --author official
```

---

## Runtime Commands

### `astrixa run`

Execute an ASTRIXA program.

**Usage:**
```bash
astrixa run <file.ax> [mode]
```

**Arguments:**
- `file.ax` (required): ASTRIXA source file to run
- `mode` (optional): Execution mode. Default: `--vm`

**Modes:**
- `--vm` - Use bytecode Virtual Machine (faster, default)
- `--interp` - Use interpreter (slower, debugging)

**What it does:**
1. Reads the source file
2. Tokenizes (lexical analysis)
3. Parses (syntax analysis)
4. Compiles to bytecode or interprets
5. Executes program

**Examples:**
```bash
# Run with default VM mode
astrixa run src/main.ax

# Run with interpreter
astrixa run src/main.ax --interp

# Run specific file
astrixa run examples/defi_portfolio_demo.ax

# Run tests
astrixa run tests/math_test.ax
```

**Output:**
Program output is printed directly:
```
Hello, ASTRIXA!
```

**Performance:**
```bash
# VM mode (default) - recommended for production
astrixa run app.ax              # ~0.1ms

# Interpreter mode - useful for debugging
astrixa run app.ax --interp     # ~1ms
```

**Error Handling:**
```bash
# Compilation error
$ astrixa run bad_syntax.ax
Error: Expected identifier at line 5
Exit code: 1

# Runtime error
$ astrixa run divide_zero.ax
Error: Division by zero
Exit code: 1
```

---

## Advanced Usage

### Multi-File Projects

**Structure:**
```
my-project/
├── astrixa.toml
├── src/
│   ├── main.ax
│   ├── utils.ax
│   └── helpers.ax
└── tests/
    └── test.ax
```

**Main file with imports:**
```javascript
import "math"
import "utils"

fn main() {
    let result = add(10, 20);
    print(result);
}
```

**Run main:**
```bash
astrixa run src/main.ax
```

---

### Using Multiple Packages

**astrixa.toml:**
```toml
name = "my-defi-app"
version = "1.0.0"
description = "My DeFi application"

[dependencies]
math = "0.1.0"
ai-tools = "0.1.0"
crypto-utils = "0.1.0"
```

**Installation:**
```bash
astrixa install math
astrixa install ai-tools
astrixa install crypto-utils
```

**Usage in code:**
```javascript
import "math"
import "ai-tools"
import "crypto-utils"

fn main() {
    let sum = add(5, 3);
    let sentiment = analyze_sentiment("bullish");
    let gas = calculate_gas(50, 10);
}
```

---

### Reproducible Builds

**First time:**
```bash
astrixa init my-app
astrixa install math       # Creates astrixa.lock with math = "0.1.0"
astrixa run src/main.ax    # Result A
```

**Later (different machine):**
```bash
# astrixa.lock already exists
git clone <repo>
astrixa run src/main.ax    # Result A (same!)
```

**astrixa.lock ensures:**
- Same packages across machines
- Same versions every time
- Reproducible builds
- Team consistency

---

## Configuration

### astrixa.toml Structure

```toml
# Project metadata
name = "my-project"
version = "0.1.0"
description = "My awesome project"
author = "Your Name"
license = "MIT"

# Dependencies (exact versions only)
[dependencies]
math = "0.1.0"
ai-tools = "0.1.0"

# Build settings (future)
[build]
target = "vm"              # vm or wasm
optimize = false
```

### .astrixa Directory Structure

```
~/.astrixa/
├── packages/              # Installed packages
│   ├── math/
│   │   ├── astrixa.toml
│   │   └── src/
│   │       └── index.ax
│   └── ai-tools/
│       ├── astrixa.toml
│       └── src/
│           └── index.ax
└── config.toml           # Global config (future)
```

### Environment Variables

```bash
# Set packages directory
export ASTRIXA_PACKAGES=~/.astrixa/packages

# Set registry URL
export ASTRIXA_REGISTRY=https://registry.astrixa.org

# Set execution mode
export ASTRIXA_MODE=vm
```

---

## Troubleshooting

### Problem: "Command not found: astrixa"

**Solution 1: Build from source**
```bash
cd compiler
cargo build --release
cp target/release/astrixa /usr/local/bin/
```

**Solution 2: Add to PATH**
```bash
export PATH="$PATH:/workspaces/astrixa-lang/compiler/target/debug"
```

---

### Problem: "Cannot find module 'math'"

**Cause:** Package not installed

**Solution:**
```bash
# Install the package
astrixa install math

# Verify installation
astrixa list

# Check location
ls ~/.astrixa/packages/math/
```

---

### Problem: "Division by zero" panic

**Cause:** Code bug

**Solution:**
```javascript
// Add validation
fn safe_divide(a: int, b: int) -> int {
    if b == 0 {
        print("Error: Cannot divide by zero");
        return 0;
    }
    return divide(a, b);
}
```

---

### Problem: Import fails with "local file"

**Cause:** Trying to use non-existent local file

**Solution:**
```javascript
// Instead of:
import "utils"           // Won't work if not in packages

// Use package:
import "utils"           // Install: astrixa install utils

// Or create local file:
// utils.ax
// then: import "utils"
```

---

### Problem: Lockfile conflicts in git

**Prevention:**
```bash
# Commit lockfile with code
git add astrixa.lock
git commit -m "Update dependencies"

# Or treat as local config
echo "astrixa.lock" >> .gitignore
```

---

### Problem: "Different versions on different machines"

**Cause:** No lockfile or old lockfile

**Solution:**
```bash
# Ensure astrixa.lock is in git
git add astrixa.lock

# Update if needed
astrixa install math 0.2.0    # Updates astrixa.lock
git add astrixa.lock
git commit -m "Update math to 0.2.0"

# Other machines automatically get same versions
git pull
astrixa run src/main.ax       # Uses versions from astrixa.lock
```

---

## Command Cheatsheet

```bash
# Initialize project
astrixa init my-app

# Install packages
astrixa install math
astrixa install ai-tools

# List packages
astrixa list

# Run program
astrixa run src/main.ax
astrixa run src/main.ax --interp

# Check package info
astrixa list | grep math

# View dependencies
cat astrixa.toml
cat astrixa.lock

# Check installed location
ls ~/.astrixa/packages/math/
cat ~/.astrixa/packages/math/astrixa.toml
```

---

## Performance Tips

### Use VM mode for production
```bash
# Fast (default)
astrixa run app.ax

# Slow (for debugging)
astrixa run app.ax --interp
```

### Optimize package imports
```javascript
// ✓ Good - Import once
import "math"

let a = add(1, 2);
let b = multiply(3, 4);
let c = divide(a, b);

// ✗ Inefficient - Multiple imports (if supported)
import "math"
import "math"
import "math"
```

### Cache packages locally
```bash
# First run (downloads)
astrixa install math       # ~1s

# Subsequent runs (cached)
astrixa run app.ax         # ~0.1s
```

---

## Future Features

```bash
# Planned commands (not yet available)

# Publish your package
astrixa publish

# Search for packages
astrixa search defi

# View package info
astrixa info math

# Update packages
astrixa update math

# Run tests
astrixa test

# Build for WASM
astrixa build --target wasm

# Lint code
astrixa lint

# Format code
astrixa fmt
```

---

## Getting Help

```bash
# Show help
astrixa --help
astrixa -h

# Show version
astrixa --version
astrixa -v

# Report issues
# GitHub: https://github.com/astrixa/astrixa-lang/issues
```

---

## Exit Codes

```
0    - Success
1    - General error
2    - Compilation error
3    - Runtime error
4    - Package not found
5    - Invalid arguments
```

---

**Last Updated:** January 4, 2026
**ASTRIXA Version:** 0.1.0
