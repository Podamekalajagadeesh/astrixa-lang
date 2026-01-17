# Contributing to ASTRIXA

**First off, thank you for considering contributing to ASTRIXA!**

ASTRIXA is a community-driven project, and we welcome contributions of all kinds: code, documentation, examples, bug reports, feature requests, and more.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [How Can I Contribute?](#how-can-i-contribute)
3. [Getting Started](#getting-started)
4. [Development Workflow](#development-workflow)
5. [Pull Request Process](#pull-request-process)
6. [Style Guidelines](#style-guidelines)
7. [Community](#community)

---

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to conduct@astrixa.dev.

---

## How Can I Contribute?

### 🐛 Reporting Bugs

**Before submitting a bug report:**
- Check if the bug has already been reported in [Issues](https://github.com/astrixa-lang/astrixa/issues)
- Make sure you're using the latest version

**How to submit a bug report:**
1. Use the bug report template
2. Provide a clear title and description
3. Include steps to reproduce
4. Add code samples if applicable
5. Describe expected vs actual behavior
6. Include version information

**Example:**
```markdown
### Bug Description
Compiler crashes when using nested async functions

### Steps to Reproduce
1. Create file with nested async
2. Run `astrixa build test.ax`
3. Observe crash

### Expected Behavior
Should compile successfully

### Actual Behavior
Segmentation fault

### Version
ASTRIXA v0.1.0, rustc 1.70.0
```

### 💡 Suggesting Features

**Before suggesting a feature:**
- Check if it's already been suggested
- Read the [RFC process](rfcs/RFC_PROCESS.md)
- Consider if it aligns with [language vision](rfcs/0001-language-vision.md)

**How to suggest a feature:**
1. Open a discussion (not an issue) on GitHub
2. Describe the problem you're trying to solve
3. Explain why existing solutions aren't adequate
4. Propose your solution
5. Consider alternatives

**For significant features, write an RFC:**
See [RFC_PROCESS.md](rfcs/RFC_PROCESS.md) for details.

### 📝 Improving Documentation

Documentation is crucial! Help us by:
- Fixing typos and errors
- Adding examples
- Improving clarity
- Writing tutorials

**Documentation lives in:**
- `/docs/` - Long-form documentation
- `/examples/` - Example programs
- `/rfcs/` - Design documents
- Inline comments in code

### 💻 Contributing Code

**Areas where we need help:**
- Compiler optimizations
- Standard library functions
- LSP features
- VS Code extension improvements
- Test coverage
- Bug fixes

**Good first issues:**
Look for issues tagged [`good-first-issue`](https://github.com/astrixa-lang/astrixa/labels/good-first-issue)

---

## Getting Started

### Prerequisites

**Required:**
- Rust 1.75+ (for compiler and LSP)
- Node.js 18+ (for VS Code extension)
- Git

**Optional:**
- VS Code (for extension development)
- Docker (for testing)

### Fork and Clone

```bash
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/astrixa.git
cd astrixa

# Add upstream remote
git remote add upstream https://github.com/astrixa-lang/astrixa.git
```

### Build from Source

**Compiler:**
```bash
cd compiler
cargo build --release
cargo test
```

**LSP:**
```bash
cd lsp
cargo build --release
cargo test
```

**VS Code Extension:**
```bash
cd astrixa-vscode
npm install
npm run compile
```

### Run Tests

```bash
# All tests
cargo test --all

# Specific component
cd compiler && cargo test
cd lsp && cargo test

# With output
cargo test -- --nocapture
```

---

## Development Workflow

### 1. Create a Branch

```bash
# Update main
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/bug-description
```

**Branch naming:**
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation
- `refactor/` - Code refactoring
- `test/` - Test improvements

### 2. Make Changes

**Best practices:**
- Make small, focused commits
- Write clear commit messages
- Add tests for new features
- Update documentation
- Run tests before committing

**Commit message format:**
```
<type>: <subject>

<body>

<footer>
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `style` - Formatting
- `refactor` - Code restructuring
- `test` - Tests
- `chore` - Maintenance

**Example:**
```
feat: add async/await syntax

Implements RFC 0002 for async/await support.

- Added async keyword to lexer
- Implemented Promise type in compiler
- Added tests for async functions

Closes #123
```

### 3. Test Your Changes

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Build documentation
cargo doc --no-deps --open
```

### 4. Keep Branch Updated

```bash
# Fetch latest changes
git fetch upstream

# Rebase your branch
git rebase upstream/main

# Resolve conflicts if any
# Then push (force push if rebased)
git push origin feature/your-feature-name --force-with-lease
```

---

## Pull Request Process

### 1. Prepare Your PR

**Checklist:**
- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No linter warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] CHANGELOG updated (if applicable)
- [ ] Commits are clean and logical

### 2. Create Pull Request

**On GitHub:**
1. Go to your fork
2. Click "New Pull Request"
3. Select base: `main`, compare: `your-branch`
4. Fill in PR template

**PR Title Format:**
```
<type>: <short description>
```

**Example:**
```
feat: implement async/await (RFC 0002)
```

### 3. PR Description

Include:
- **What:** Brief description of changes
- **Why:** Motivation and context
- **How:** Technical approach
- **Testing:** How you tested
- **Closes:** Link to issues (if applicable)

**Template:**
```markdown
## What
Implements async/await syntax according to RFC 0002.

## Why
Enables non-blocking I/O for web servers and Web3 operations.

## How
- Added async keyword to lexer
- Implemented Promise type in compiler
- Created event loop runtime

## Testing
- Added 50+ unit tests
- Tested with async web server example
- All existing tests pass

## Closes
Closes #123, closes #456
```

### 4. Code Review

**Expect:**
- Feedback and suggestions
- Requests for changes
- Discussion about approach

**Be:**
- Patient and respectful
- Open to feedback
- Responsive to comments

**Reviewers will check:**
- Code quality and correctness
- Test coverage
- Documentation
- Performance impact
- Backward compatibility

### 5. Make Requested Changes

```bash
# Make changes in your branch
git add .
git commit -m "address review feedback"
git push origin feature/your-feature-name
```

**For small changes:**
- Amend commits: `git commit --amend`
- Force push: `git push --force-with-lease`

**For larger changes:**
- Add new commits
- Squash later before merge

### 6. Merging

**When approved:**
- Maintainer will merge your PR
- Squash and merge (default)
- Your changes are now in main! 🎉

**After merge:**
```bash
# Update your fork
git checkout main
git pull upstream main
git push origin main

# Delete feature branch
git branch -d feature/your-feature-name
git push origin --delete feature/your-feature-name
```

---

## Style Guidelines

### Rust Code Style

**Follow Rust conventions:**
```rust
// ✅ Good
fn calculate_balance(address: &Address) -> U256 {
    let balance = state.get(address).unwrap_or(0);
    balance + pending_balance
}

// ❌ Bad
fn CalculateBalance(address:&Address)->U256{
    let balance=state.get(address).unwrap_or(0);
    return balance+pending_balance;
}
```

**Use `cargo fmt`:**
```bash
cargo fmt
```

**Follow `clippy` suggestions:**
```bash
cargo clippy -- -D warnings
```

### ASTRIXA Code Style

**Standard library:**
```astrixa
// ✅ Good
fn transfer(to: Address, amount: U256) -> bool {
    require(to != "0x0", "Invalid address");
    require(amount > 0, "Invalid amount");
    
    update_balance(to, amount);
    return true;
}

// ❌ Bad
fn transfer(to: Address,amount: U256)->bool{
    if to=="0x0"{panic("Invalid");}
    if amount<=0{panic("Invalid");}
    update_balance(to,amount);
    true
}
```

### Documentation Style

**Use Markdown:**
- Clear headings
- Code blocks with language
- Examples for complex concepts
- Links to related docs

**Code comments:**
```rust
// ✅ Good: Explains WHY
// Use saturating add to prevent overflow in gas calculations
let total_gas = base_gas.saturating_add(compute_gas);

// ❌ Bad: Explains WHAT (code already shows this)
// Add base_gas and compute_gas
let total_gas = base_gas + compute_gas;
```

### Commit Message Style

**Format:**
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Example:**
```
feat(compiler): add async/await syntax

Implements RFC 0002 for async programming support.

The compiler now recognizes async/await keywords and
generates appropriate bytecode for Promise-based execution.

Closes #123
BREAKING CHANGE: async is now a reserved keyword
```

---

## Community

### Communication Channels

- **GitHub Issues:** Bug reports and feature requests
- **GitHub Discussions:** General questions and ideas
- **Pull Requests:** Code contributions and reviews

### Getting Help

**Stuck? Ask for help!**
- Comment on your PR or issue
- Start a discussion on GitHub Discussions
- Tag maintainers for urgent matters

**Don't know where to start?**
- Look for `good-first-issue` labels
- Check [ROADMAP.md](ROADMAP.md) for ideas
- Browse existing issues and PRs

### Recognition

**Contributors get:**
- Listed in CONTRIBUTORS.md
- Credit in release notes
- Maintainer role (after sustained contributions)
- Swag (stickers, t-shirts)
- Eternal gratitude! 🙏

### Core Team

**Interested in joining the Core Team?**
- Make consistent, high-quality contributions
- Participate in RFC discussions
- Help review PRs
- Support community members

**Contact:** core@astrixa.dev

---

## Legal

### Contributor License Agreement (CLA)

**By contributing, you agree that:**
1. You own the copyright to your contribution
2. You grant ASTRIXA project a perpetual, worldwide license to use your contribution
3. Your contribution is provided under MIT license

**No separate CLA required** - submitting a PR indicates acceptance.

### Copyright

**Add copyright notice to new files:**
```rust
// Copyright 2024-2026 ASTRIXA Contributors
// SPDX-License-Identifier: MIT
```

---

## Questions?

**Have questions about contributing?**
- Read [GOVERNANCE.md](GOVERNANCE.md)
- Check [RFC_PROCESS.md](rfcs/RFC_PROCESS.md)
- Ask in Discord #contributors
- Email: contributors@astrixa.dev

---

**Thank you for contributing to ASTRIXA! Together, we're building the future of Web3 development. 🚀**

---

**Last Updated:** January 2025
