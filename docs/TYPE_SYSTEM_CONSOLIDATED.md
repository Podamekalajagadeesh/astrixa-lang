# ASTRIXA Type System — Complete Guide

> **Quick Navigation**: [Quick Reference](#quick-reference) | [Core Concepts](#core-concepts) | [Type Checking Rules](#type-checking-rules) | [Architecture](#architecture) | [Testing](#testing)

---

## Quick Reference

### Supported Types

| Type | Purpose | Example |
|------|---------|---------|
| `Int` | 64-bit signed integers | `let x = 42` |
| `String` | Unicode text | `let msg = "hello"` |
| `Bool` | Logical values (via Int) | `let x = 1` (true) |
| `Float` | Floating point numbers | `let pi = 3.14` |
| `Void` | No return value | Functions with no return |

### Type Inference Rules

Variables automatically infer their type from initialization:

```astrixa
let x = 10         // → Int
let s = "hello"    // → String
let f = 3.14       // → Float
let b = 1          // → Int (not Bool in V1)
```

---

## Core Concepts

### Type Safety Through Static Checking

ASTRIXA ensures type safety by checking all operations at **compile time** before execution:

```astrixa
let x = 10
x = "hello"        // ❌ Compile error: cannot assign String to Int
```

Error message:
```
Type mismatch: cannot assign String to variable of type Int
```

**Why this matters:** No silent type coercions. You know exactly what types you're working with.

### Type Inference

Variables automatically infer types from their initial values:

```astrixa
let count = 42         // Inferred: Int
let name = "ASTRIXA"   // Inferred: String
let ratio = 2.5        // Inferred: Float
```

---

## Type Checking Rules

### Variables

```
let var = expr        // var gets type of expr
var = new_expr        // new_expr type must match var type
```

### Arithmetic Operations

**Int-only (except Add):**

| Operation | Operands | Result |
|-----------|----------|--------|
| `a + b` | Both Int | Int ✅ |
| `a + b` | Both String | String ✅ |
| `a + b` | Mixed types | ❌ Error |
| `a - b` | Both Int | Int ✅ |
| `a - b` | Other types | ❌ Error |
| `a * b` | Both Int | Int ✅ |
| `a / b` | Both Int | Int ✅ |
| `a % b` | Both Int | Int ✅ |

### Comparison Operations

| Operation | Operands | Result |
|-----------|----------|--------|
| `a == b` | Both Int | Bool ✅ |
| `a != b` | Both Int | Bool ✅ |
| `a < b` | Both Int | Bool ✅ |
| `a <= b` | Both Int | Bool ✅ |
| `a > b` | Both Int | Bool ✅ |
| `a >= b` | Both Int | Bool ✅ |
| `a == b` | Other types | ❌ Error |

### Control Flow

```astrixa
if condition { ... }      // condition must be Int or Bool
while condition { ... }   // condition must be Int or Bool
```

### Functions

```astrixa
fn name(a b) {
  return expr           // expr type is return type
}

let result = name(5 10)  // Arguments validated against parameters
```

**Special case:** `panic()` requires String argument
```astrixa
panic("error message")   // ✅
panic(42)                // ❌ Error: requires String
```

---

## Common Errors & Solutions

### Error: Type Mismatch

```astrixa
let x = 10
x = "hello"              // ❌ Type mismatch
```

**Solution:** Ensure assigned value matches variable type
```astrixa
let x = 10
x = 20                   // ✅ Correct
```

### Error: Invalid Arithmetic

```astrixa
let result = 10 + "20"   // ❌ Cannot add Int and String
```

**Solution:** Ensure both operands are the same type
```astrixa
let result = 10 + 20     // ✅ Both Int
```

### Error: Function Argument Type

```astrixa
fn add(a b) { return a + b }
add(5 "10")              // ❌ Second argument is String
```

**Solution:** Pass arguments of the correct type
```astrixa
add(5 10)                // ✅ Both Int
```

### Error: Return Type Inconsistency

```astrixa
fn bad {
  if 1 {
    return 10            // Int
  } else {
    return "ten"         // String — ❌ Mismatch!
  }
}
```

**Solution:** Return the same type from all paths
```astrixa
fn good {
  if 1 {
    return 10
  } else {
    return 20
  }
}
```

---

## Architecture

### Compilation Pipeline

The type system operates during the **Type Checking phase**:

```
Source Code (.ax)
    ↓
[LEXER] → Tokenize
    ↓
[PARSER] → AST (Abstract Syntax Tree)
    ↓
[TYPE CHECKER] ← YOU ARE HERE
    ├─ Symbol Table (variable names → types)
    ├─ Type Inference (infer types from literals)
    └─ Type Validation (check all operations)
    ↓
[ERRORS?] → Report and stop
    ↓
[NO ERRORS]
    ↓
[CODEGEN] → Bytecode/WebAssembly
```

### Core Components

**Type Definitions** (compiler/src/types.rs):
```rust
pub enum Type {
    Int,      // 64-bit signed
    Float,    // 64-bit float
    Bool,     // Boolean
    String,   // Unicode string
    Void,     // No value
    Unknown,  // To be inferred
}
```

**Type Checker** (compiler/src/type_checker.rs):
- Maintains symbol table of variables → types
- Infers types from literals
- Validates operations
- Reports errors with line numbers

---

## Testing

### Running Type System Tests

The type system is tested with 15+ test cases covering:
- ✅ Variable type inference
- ✅ Arithmetic type checking
- ✅ Function call validation
- ✅ Return type consistency
- ✅ Error messages

Run tests:
```bash
cd compiler
cargo test type_checker
```

### Test Examples

**Test 1: Type Inference**
```astrixa
let x = 42      // Should infer Int
```
Expected: Passes type checking

**Test 2: Type Mismatch**
```astrixa
let x = 10
x = "hello"     // Should fail
```
Expected: Compiler error with clear message

**Test 3: Arithmetic Safety**
```astrixa
let result = 5 + 3  // ✅ Both Int
```
Expected: Passes

**Test 4: Function Type Checking**
```astrixa
fn add(a b) { return a + b }
add(5 10)     // ✅ Valid
add(5 "10")   // ❌ Invalid
```
Expected: First passes, second fails

---

## Future Enhancements

**v0.2.0:**
- Generic types: `array<T>`, `map<K, V>`
- Union types: `Int | String`
- Nullable types: `T | Void`

**v0.3.0:**
- Custom types: `type Point = {x: Int, y: Int}`
- Type bounds for functions: `fn max<T extends Comparable>(a: T, b: T) -> T`
- Struct and enum support

**v1.0.0:**
- Full algebraic data types
- Dependent types
- Formal verification integration

---

## References

- **Implementation Details:** [TYPE_SYSTEM_ARCHITECTURE.md](TYPE_SYSTEM_ARCHITECTURE.md)
- **Testing Guide:** [TYPE_SYSTEM_TESTING.md](TYPE_SYSTEM_TESTING.md)
- **Full Specification:** [docs/TYPE_SYSTEM.md](docs/TYPE_SYSTEM.md)
- **Main Language Reference:** [../README.md](../README.md)
