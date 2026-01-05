# ASTRIXA Package Manager - Usage Guide

## Table of Contents
- [Installation](#installation)
- [Getting Started](#getting-started)
- [Creating Packages](#creating-packages)
- [Publishing Packages](#publishing-packages)
- [Advanced Topics](#advanced-topics)
- [Best Practices](#best-practices)

## Installation

### Building from Source

```bash
cd compiler
cargo build --release

# Add to PATH
export PATH="$PWD/target/release:$PATH"

# Verify installation
astrixa --help
```

## Getting Started

### 1. Initialize Your First Project

```bash
# Create a new project
astrixa init hello-world
cd hello-world

# Your project structure:
# hello-world/
# ├── astrixa.toml
# └── src/
#     └── main.ax
```

The generated `astrixa.toml`:
```toml
name = "hello-world"
version = "0.1.0"
description = "A new ASTRIXA project"

[dependencies]
```

### 2. Install Your First Package

```bash
# Install the math utilities package
astrixa install math
```

Output:
```
📦 Installing math@latest...
✓ Installed math@0.1.0
```

This:
- Downloads the package
- Stores it in `~/.astrixa/packages/math`
- Creates/updates `astrixa.lock`

### 3. Use the Package

Edit `src/main.ax`:
```ax
import "math"

fn main() {
    let sum = add(10, 5);
    let product = multiply(10, 5);
    let power = power(2, 8);
    
    print("10 + 5 =", sum);
    print("10 * 5 =", product);
    print("2^8 =", power);
}
```

### 4. Run Your Code

```bash
astrixa run src/main.ax
```

Output:
```
10 + 5 = 15
10 * 5 = 50
2^8 = 256
```

## Creating Packages

### Step-by-Step Package Creation

#### 1. Create Package Directory

```bash
mkdir my-utils
cd my-utils
mkdir src
```

#### 2. Create astrixa.toml

```toml
name = "my-utils"
version = "0.1.0"
description = "My utility functions"

[dependencies]
# Add dependencies here
```

#### 3. Create src/index.ax

The `index.ax` file is the entry point. Use `export` to make functions public:

```ax
// String utilities
export fn uppercase(text: string) -> string {
    // Implementation
    return text;
}

export fn lowercase(text: string) -> string {
    // Implementation
    return text;
}

export fn reverse(text: string) -> string {
    let result = "";
    let i = len(text) - 1;
    while i >= 0 {
        result = result + text[i];
        i = i - 1;
    }
    return result;
}

// Private helper (not exported)
fn helper_function() -> int {
    return 42;
}
```

#### 4. Test Locally

```bash
# Copy to packages directory
mkdir -p ~/.astrixa/packages/my-utils
cp -r . ~/.astrixa/packages/my-utils

# Create test file
echo 'import "my-utils"

fn main() {
    print(reverse("hello"));
}' > test.ax

# Run test
astrixa run test.ax
```

## Publishing Packages

### Current (MVP) - Manual Publishing

1. **Prepare your package:**
   ```bash
   # Ensure all files are ready
   ls -la
   # astrixa.toml
   # src/index.ax
   ```

2. **Create a git repository:**
   ```bash
   git init
   git add .
   git commit -m "Initial release"
   git tag v0.1.0
   ```

3. **Share the repository:**
   ```bash
   # Push to GitHub/GitLab
   git remote add origin <your-repo-url>
   git push origin main
   git push origin v0.1.0
   ```

4. **Users can install:**
   ```bash
   # From git (coming soon)
   astrixa install my-utils --git https://github.com/user/my-utils
   ```

### Future - Registry Publishing

```bash
# Login to registry
astrixa login

# Publish package
astrixa publish

# Version bump
astrixa version patch  # 0.1.0 -> 0.1.1
astrixa version minor  # 0.1.1 -> 0.2.0
astrixa version major  # 0.2.0 -> 1.0.0
```

## Advanced Topics

### Package Dependencies

Add dependencies to `astrixa.toml`:

```toml
name = "my-app"
version = "1.0.0"

[dependencies]
math = "0.1.0"
ai-tools = "0.3.2"
```

Install all dependencies:
```bash
astrixa install
```

### Lockfile Management

The `astrixa.lock` file ensures deterministic builds:

```toml
[packages]
math = "0.1.0"
ai-tools = "0.3.2"
```

**Always commit `astrixa.lock` to version control!**

### Import Resolution Order

1. Check if import starts with `./` or `../` → Local file
2. Otherwise → Package from `~/.astrixa/packages/`

```ax
import "math"           // ~/.astrixa/packages/math/src/index.ax
import "./local"        // ./local.ax
import "../utils"       // ../utils.ax
```

### Module Organization

For larger packages:

```
my-package/
├── astrixa.toml
└── src/
    ├── index.ax        # Main entry point
    ├── utils.ax        # Internal module
    └── helpers.ax      # Internal module
```

`index.ax` can import internal modules:
```ax
import "./utils"
import "./helpers"

export fn main_function() {
    // Use internal modules
}
```

## Best Practices

### 1. Versioning

Follow semantic versioning:
- **Patch** (0.1.0 → 0.1.1): Bug fixes
- **Minor** (0.1.0 → 0.2.0): New features, backward compatible
- **Major** (0.1.0 → 1.0.0): Breaking changes

### 2. Documentation

Document your exports:

```ax
// Calculate the factorial of a number
// Returns: factorial value
// Example: factorial(5) = 120
export fn factorial(n: int) -> int {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
```

### 3. Testing

Create a `tests/` directory:

```
my-package/
├── astrixa.toml
├── src/
│   └── index.ax
└── tests/
    ├── test_math.ax
    └── test_string.ax
```

### 4. Error Handling

Use descriptive error messages:

```ax
export fn divide(a: int, b: int) -> int {
    if b == 0 {
        panic("divide: division by zero");
    }
    return a / b;
}
```

### 5. Naming Conventions

- **Packages**: lowercase, hyphen-separated: `my-package`
- **Functions**: snake_case: `calculate_sum`
- **Constants**: UPPER_CASE: `MAX_SIZE`

### 6. Security

For Web3/smart contract packages:
- Avoid randomness (non-deterministic)
- Minimize gas usage
- Validate all inputs
- Use exact version pinning

```toml
[dependencies]
crypto = "1.0.0"  # Exact version, not "^1.0.0"
```

### 7. Performance

- Minimize dependencies
- Use efficient algorithms
- Cache expensive computations

## Common Workflows

### Creating a Library Package

```bash
# 1. Create package
mkdir math-advanced
cd math-advanced

# 2. Setup structure
cat > astrixa.toml << 'EOF'
name = "math-advanced"
version = "0.1.0"
description = "Advanced mathematical functions"
EOF

mkdir src
cat > src/index.ax << 'EOF'
export fn sqrt(n: int) -> int {
    // Newton's method
    let guess = n / 2;
    let i = 0;
    while i < 10 {
        guess = (guess + n / guess) / 2;
        i = i + 1;
    }
    return guess;
}
EOF

# 3. Test locally
mkdir -p ~/.astrixa/packages/math-advanced
cp -r . ~/.astrixa/packages/math-advanced

# 4. Create test
echo 'import "math-advanced"
fn main() {
    print("sqrt(16) =", sqrt(16));
}' > test.ax

astrixa run test.ax
```

### Creating a Web3 Package

```bash
mkdir web3-utils
cd web3-utils

cat > astrixa.toml << 'EOF'
name = "web3-utils"
version = "0.1.0"
description = "Web3 utility functions"
EOF

mkdir src
cat > src/index.ax << 'EOF'
export fn get_sender() -> string {
    return ctx.sender;
}

export fn get_chain_id() -> int {
    return ctx.chain_id;
}

export fn get_block_time() -> int {
    return ctx.tx_timestamp;
}
EOF
```

### Creating an AI Package

```bash
mkdir ai-helpers
cd ai-helpers

cat > astrixa.toml << 'EOF'
name = "ai-helpers"
version = "0.1.0"
description = "AI helper functions"
EOF

mkdir src
cat > src/index.ax << 'EOF'
export fn is_positive_sentiment(text: string) -> bool {
    let result = ai.infer(ai.model("sentiment"), text);
    return result.label == "positive" && result.score > 0.7;
}

export fn categorize(text: string, categories: array) -> string {
    let result = ai.infer(ai.model("classify"), text);
    return result.label;
}
EOF
```

## Troubleshooting

### Package Not Found

```
Error: Cannot find module 'math'
```

**Solution:**
```bash
astrixa install math
```

### Version Conflict

```
Error: Package 'utils' requires 'math' version 0.2.0, but 0.1.0 is installed
```

**Solution:** Update dependencies:
```bash
astrixa install math 0.2.0
```

### Import Path Issues

```
Error: Cannot find module './utils'
```

**Check:**
1. File exists: `ls utils.ax`
2. Correct path: use `./` for current directory
3. Correct extension: `.ax` files

### Clear Package Cache

```bash
rm -rf ~/.astrixa/packages
astrixa install  # Reinstall all
```

## Next Steps

- [Read the full documentation](PACKAGE_MANAGER.md)
- [View examples](examples/)
- [Join the community](#)
- [Contribute packages](#)

---

**Happy coding with ASTRIXA! 🚀**
