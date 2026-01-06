# Language Syntax

Complete guide to ASTRIXA syntax and semantics.

## Basic Syntax

### Comments

```astrixa
// Single-line comment

/*
   Multi-line comment
   Can span multiple lines
*/
```

### Variables

Variables are declared with `let`:

```astrixa
let name = "Alice"              // Type inferred as string
let age: int = 30               // Explicit type annotation
let price: float = 99.99
let active: bool = true
```

**Rules:**
- Variable names must start with a letter or underscore
- Variables are immutable by default (use `mut` for mutable variables - coming soon)
- Type inference works for most cases

### Constants

Constants are compile-time values:

```astrixa
const MAX_SIZE = 1000
const PI = 3.14159
const APP_NAME = "ASTRIXA"
```

## Data Types

### Primitive Types

| Type | Description | Example |
|------|-------------|---------|
| `int` | Integer number | `42`, `-10` |
| `float` | Floating point | `3.14`, `-0.5` |
| `string` | Text | `"hello"`, `'world'` |
| `bool` | Boolean | `true`, `false` |
| `Address` | Blockchain address | `0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb` |
| `U256` | 256-bit unsigned integer | `1000000000000000000` |

### Collections

**Arrays:**
```astrixa
let numbers = [1, 2, 3, 4, 5]
let names = ["Alice", "Bob", "Charlie"]

// Access elements
let first = numbers[0]  // 1
let last = numbers[4]   // 5
```

**Maps (Dictionaries):**
```astrixa
let scores = {
    "Alice": 95,
    "Bob": 87,
    "Charlie": 92
}

// Access values
let aliceScore = scores["Alice"]  // 95
```

**JSON Objects:**
```astrixa
let user = {
    name: "Alice",
    age: 30,
    active: true
}

// Nested objects
let config = {
    server: {
        port: 8080,
        host: "localhost"
    }
}
```

## Operators

### Arithmetic
```astrixa
let sum = 10 + 5        // 15
let diff = 10 - 5       // 5
let product = 10 * 5    // 50
let quotient = 10 / 5   // 2
let remainder = 10 % 3  // 1
```

### Comparison
```astrixa
10 == 10    // true (equal)
10 != 5     // true (not equal)
10 > 5      // true (greater than)
10 < 20     // true (less than)
10 >= 10    // true (greater or equal)
10 <= 5     // false (less or equal)
```

### Logical
```astrixa
true && false   // false (AND)
true || false   // true (OR)
!true          // false (NOT)
```

### String Operations
```astrixa
let greeting = "Hello" + " " + "World"  // Concatenation
let name = "Alice"
let message = "Welcome, " + name        // "Welcome, Alice"
```

## Control Flow

### If Statements

```astrixa
let score = 85

if score >= 90 {
    print("A grade")
} else if score >= 80 {
    print("B grade")
} else {
    print("C grade")
}
```

**Ternary-style (coming soon):**
```astrixa
let status = score >= 60 ? "Pass" : "Fail"
```

### While Loops

```astrixa
let count = 0

while count < 5 {
    print(count)
    count = count + 1
}
```

### For Loops

```astrixa
// Range-based
for i in 0..10 {
    print(i)
}

// Array iteration
let names = ["Alice", "Bob", "Charlie"]
for name in names {
    print(name)
}
```

### Break and Continue

```astrixa
while true {
    if condition {
        break  // Exit loop
    }
    if other_condition {
        continue  // Skip to next iteration
    }
}
```

## Functions

### Basic Functions

```astrixa
fn greet(name: string) -> string {
    return "Hello, " + name
}

let message = greet("Alice")  // "Hello, Alice"
```

### Multiple Parameters

```astrixa
fn add(a: int, b: int) -> int {
    return a + b
}

let sum = add(5, 3)  // 8
```

### No Return Value

```astrixa
fn printMessage(msg: string) {
    print(msg)
}
```

### Default Parameters (coming soon)

```astrixa
fn greet(name: string = "Guest") -> string {
    return "Hello, " + name
}
```

### Anonymous Functions (Lambdas)

```astrixa
let double = fn(x: int) -> int {
    return x * 2
}

let result = double(5)  // 10
```

## Modules

### Importing

```astrixa
use std::web
use std::ai
use std::json

// Import specific functions
use std::math::{sqrt, pow}

// Import everything
use std::crypto::*
```

### Creating Modules

**File: math.ax**
```astrixa
fn add(a: int, b: int) -> int {
    return a + b
}

fn multiply(a: int, b: int) -> int {
    return a * b
}
```

**File: main.ax**
```astrixa
use math

let sum = math.add(5, 3)
let product = math.multiply(5, 3)
```

## Error Handling

### Try-Catch (coming soon)

```astrixa
try {
    let result = riskyOperation()
    print(result)
} catch error {
    print("Error: " + error.message)
}
```

### Panic

```astrixa
if value < 0 {
    panic("Value must be positive")
}
```

## Pattern Matching (coming soon)

```astrixa
match value {
    0 => print("Zero"),
    1..10 => print("Small"),
    _ => print("Large")
}
```

## Generics (future)

```astrixa
fn first<T>(items: [T]) -> T {
    return items[0]
}

let num = first([1, 2, 3])      // int
let name = first(["a", "b"])    // string
```

## Best Practices

### Naming Conventions

```astrixa
// Variables and functions: snake_case
let user_name = "Alice"
fn get_user() { }

// Constants: UPPER_SNAKE_CASE
const MAX_CONNECTIONS = 100

// Types (future): PascalCase
type UserProfile = { }
```

### Code Style

```astrixa
// ✅ Good - Clear and readable
fn calculateTotal(price: float, quantity: int) -> float {
    return price * quantity
}

// ❌ Avoid - Unclear names
fn calc(p: float, q: int) -> float {
    return p * q
}
```

### Error Messages

```astrixa
// ✅ Good - Descriptive
if age < 0 {
    panic("Age cannot be negative: " + age)
}

// ❌ Avoid - Vague
if age < 0 {
    panic("Invalid")
}
```

## Next Topics

- [Types →](types.md)
- [Async Programming →](async.md)
- [Errors →](errors.md)
- [Modules →](modules.md)

---

**Quick Reference:** See [examples/](../../examples/) for real-world code samples.
