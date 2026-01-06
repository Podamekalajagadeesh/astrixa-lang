# RFC 0001: ASTRIXA Language Vision and Design Principles

- **Start Date:** 2024-12-15
- **Author:** Core Team
- **Status:** Accepted
- **Tracking Issue:** N/A (foundational)

## Summary

This RFC establishes the core vision and design principles for the ASTRIXA programming language. These principles guide all future design decisions and RFCs.

## Motivation

Without clear principles, languages drift. Features get added haphazardly, creating inconsistency. ASTRIXA needs a north star.

**Problems we're solving:**
- Web3 development requires multiple languages (Solidity, JavaScript, Python)
- AI integration is bolted on, not native
- Smart contracts have poor developer experience
- Full-stack dApps need complex toolchains
- Learning curve is steep for newcomers

## Vision

**ASTRIXA is the FIRST language designed for the decentralized web.**

**Core thesis:**
> One language for backend APIs, frontend UIs, smart contracts, Web3 clients, and AI agents.

**What makes ASTRIXA special:**
1. **Native Web3** - Blockchain operations built into the language
2. **Native AI** - Deterministic inference and training
3. **Multi-Target** - Compile to native, WASM, EVM bytecode
4. **Type Safe** - Catch errors at compile time
5. **Security First** - Language-level protection

## Design Principles

### 1. Stability Over Features

**Principle:** A working program today should work tomorrow.

**Means:**
- No breaking changes without major version bump
- Deprecation warnings before removal (6+ months)
- Automated migration tools
- RFC process for all changes
- Backward compatibility by default

**Example:**
```astrixa
// Code written in v0.5
let x = 42;
println(x);

// Still works in v0.9, v1.0, v2.0, etc.
```

**Anti-pattern:**
```
Python 2 vs Python 3 - Never again!
```

### 2. Explicit Over Implicit

**Principle:** Magic is bad. Clarity is good.

**Means:**
- Explicit imports (no global namespace pollution)
- Explicit types (inference where clear)
- Explicit mutability
- Explicit async (no hidden event loops)
- Explicit error handling

**Example:**
```astrixa
// ✅ GOOD - Explicit
import std::web3;
let mut balance: U256 = web3.balance(address);

// ❌ BAD - Implicit (we don't do this)
balance = getBalance(address)  // Where's getBalance from?
```

### 3. Safety Without Ceremony

**Principle:** Make correct code easy, incorrect code hard.

**Means:**
- Type system prevents errors
- Compiler enforces contracts
- Memory safe by default
- No null pointer exceptions
- No buffer overflows

**Example:**
```astrixa
// ✅ Compiler prevents mistakes
fn transfer(to: Address, amount: U256) {
    require(to != "0x0", "Invalid address");  // Enforced
    // Compiler ensures require() is present
}

// ❌ Won't compile
fn bad_transfer(to: Address, amount: U256) {
    state["balance"][to] += amount;  // Error: No validation!
}
```

### 4. Performance by Default

**Principle:** Fast code shouldn't require expert knowledge.

**Means:**
- Zero-cost abstractions
- Efficient bytecode generation
- Optimizing compiler
- Low gas costs (smart contracts)
- Fast RPC (Web3 operations)

**Example:**
```astrixa
// No performance penalty
for item in large_array {
    process(item);  // Compiled to efficient loop
}
```

### 5. Productive Developer Experience

**Principle:** Developer time is precious.

**Means:**
- Fast compile times (<5 seconds)
- Helpful error messages
- Great IDE support (LSP, autocomplete)
- Batteries included (stdlib)
- Minimal boilerplate

**Example:**
```astrixa
// ✅ Clear error message
Error: Cannot transfer to zero address
  --> contract.ax:42
   |
42 |     transfer("0x0", 100);
   |              ^^^^^ Invalid address
   |
   = help: Use a valid Ethereum address
```

### 6. Learn Once, Use Everywhere

**Principle:** Same language, different targets.

**Means:**
- One syntax for all use cases
- Shared standard library
- Consistent semantics
- Target-specific features clearly marked

**Example:**
```astrixa
// Same ASTRIXA code
fn calculate(x: i64) -> i64 {
    return x * 2;
}

// Compile to:
// - Native binary (--target=native)
// - WASM module (--target=wasm)
// - EVM bytecode (--target=contract)
```

### 7. Community Over Hype

**Principle:** Users > marketing.

**Means:**
- Listen to feedback
- RFC process for changes
- Public roadmap
- Transparent governance
- No surprise announcements

**Example:**
```
❌ "ASTRIXA 2.0 - Complete Rewrite!"
✅ "ASTRIXA v2.0 - 18 months of community RFCs"
```

### 8. Web3 First, Web3 Always

**Principle:** Blockchain is not a library, it's the language.

**Means:**
- `msg`, `tx`, `chain` available everywhere
- Native contract compilation
- Gas-aware execution
- Deterministic by default
- Multi-chain support

**Example:**
```astrixa
// No imports needed - Web3 is built-in
contract Token {
    fn transfer(to: Address, amount: U256) {
        // msg.sender is always available
        require(msg.sender != "0x0", "Invalid sender");
    }
}
```

### 9. AI as a Primitive

**Principle:** AI is not an addon, it's native.

**Means:**
- `ai.infer()` built into language
- Deterministic execution
- Gas metering for AI ops
- Model versioning
- On-chain and off-chain support

**Example:**
```astrixa
// AI is a language feature
let sentiment = ai.infer(ai.model("sentiment"), text);

// Works in contracts too (deterministic)
contract ContentDAO {
    fn moderate(content: string) {
        let result = ai.infer(ai.model("moderation"), content);
        // ...
    }
}
```

### 10. Boring Technology

**Principle:** Use proven concepts, innovate where it matters.

**Means:**
- Standard syntax (like Rust/TypeScript)
- Proven type system
- Established patterns
- Innovate on Web3/AI integration only

**Example:**
```astrixa
// Familiar syntax
let mut x = 5;
if x > 0 {
    println("positive");
}

// Innovation: Native Web3
let balance = web3.balance(address);
```

## Language Philosophy

### What ASTRIXA Is

- **Production-ready** - Stable, reliable, boring
- **Developer-focused** - DX first, always
- **Multi-paradigm** - Functional + imperative
- **Gradually typed** - Type inference where clear
- **Memory safe** - No segfaults, no UB
- **Concurrent** - Async/await built-in
- **Web3-native** - Blockchain is core, not addon

### What ASTRIXA Is NOT

- **Research language** - No unproven features
- **Minimalist** - Batteries included
- **Dynamically typed** - Type safety required
- **Object-oriented** - No classes (structs + traits)
- **Lowest-level** - Not a systems language
- **Solidity clone** - Better design, modern features

## Target Audience

### Primary Users

1. **Web3 Developers**
   - Building dApps
   - Writing smart contracts
   - Interacting with blockchain

2. **Full-Stack Developers**
   - Want one language for everything
   - Tired of context switching
   - Value productivity

3. **AI Engineers**
   - Need Web3 integration
   - Want deterministic inference
   - Building AI agents

### Secondary Users

4. **Solidity Developers**
   - Looking for better DX
   - Security concerns
   - Need more features

5. **Startups**
   - Small teams
   - Need to move fast
   - Want low maintenance

## Success Criteria

**ASTRIXA is successful when:**

1. **Adoption** - 10,000+ developers by 2027
2. **Production Use** - 100+ contracts on mainnet
3. **Ecosystem** - 1,000+ packages in registry
4. **Stability** - 99.9% backward compatibility
5. **Community** - Active, healthy, growing
6. **Performance** - 10% lower gas than Solidity
7. **Security** - Zero critical vulnerabilities
8. **Tooling** - Best-in-class IDE support

## Comparison to Other Languages

### vs Solidity

**ASTRIXA wins on:**
- Modern syntax and features
- Better type system
- Native Web3 (not just contracts)
- AI integration
- Developer experience

**Solidity wins on:**
- Maturity and ecosystem
- Auditor familiarity
- Tool support (for now)

### vs Rust

**ASTRIXA wins on:**
- Easier learning curve
- Native Web3
- Native AI
- Faster compile times
- Less complexity

**Rust wins on:**
- Lower level control
- Maximum performance
- Larger ecosystem

### vs TypeScript

**ASTRIXA wins on:**
- Type safety (no `any`)
- Smart contracts
- Native Web3 (no libraries)
- Deterministic execution

**TypeScript wins on:**
- Massive ecosystem
- More developers
- Browser native

## Future Evolution

### Version 0.x (2024-2025)

**Focus:** Core language + Web3
- Stable syntax
- Standard library
- Smart contracts
- LSP support
- Documentation

### Version 1.0 (2025)

**Focus:** Production ready
- Stability guarantees
- Performance optimizations
- Security audits
- Enterprise support

### Version 2.0+ (2026+)

**Focus:** Innovation
- Multi-chain (Solana, Move, Cosmos)
- Advanced AI features
- Formal verification
- Zero-knowledge proofs

## Non-Goals

**ASTRIXA will NOT:**
1. Replace every language (use right tool for job)
2. Support legacy code (clean slate)
3. Be all things to all people (Web3 focus)
4. Sacrifice stability for features (boring is good)
5. Fragment the ecosystem (one ASTRIXA)

## Alternatives Considered

### Alternative 1: Extend Existing Language

**Option:** Add Web3 to Python/JavaScript

**Pros:**
- Existing ecosystem
- Familiar syntax
- Large community

**Cons:**
- Baggage from past decisions
- Not Web3-first
- Poor type safety
- No smart contract target

**Decision:** New language allows clean slate

### Alternative 2: Domain-Specific Language

**Option:** ASTRIXA only for smart contracts

**Pros:**
- Narrower scope
- Easier to implement
- Clear use case

**Cons:**
- Still need other languages
- Misses full-stack opportunity
- Limited innovation

**Decision:** Full language more valuable

## Unresolved Questions

**To be determined in future RFCs:**
1. Exact syntax for certain features
2. Standard library API details
3. Package management specifics
4. Release cycle length
5. Formal specification format

## Implementation Plan

**This RFC is foundational - no implementation needed.**

**Instead, it guides:**
- All future RFCs
- Design decisions
- Community discussions
- Governance process

## Teaching

**This RFC should be:**
- Read by all contributors
- Referenced in other RFCs
- Linked from documentation
- Discussed in onboarding

## References

**Inspiration from:**
- Rust: Safety + performance
- Python: Readability + simplicity
- TypeScript: Gradual typing
- Solidity: Smart contracts
- Go: Simplicity + tooling

**Reading:**
- "The Design of Everyday Things" (Don Norman)
- "Programming Language Pragmatics" (Michael Scott)
- "Types and Programming Languages" (Benjamin Pierce)

---

**This RFC establishes the foundation. All future RFCs must align with these principles.**

---

**Status:** ✅ Accepted (Foundational)  
**Last Updated:** January 2025
