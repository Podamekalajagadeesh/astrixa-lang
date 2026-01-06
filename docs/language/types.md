# Type System

Understanding ASTRIXA's type system for safe and predictable code.

## Type Philosophy

ASTRIXA uses **static typing with type inference**:
- Types are checked at compile time
- You don't always need to write type annotations
- The compiler infers types when obvious

## Primitive Types

### Numbers

**Integer (`int`):**
```astrixa
let count: int = 42
let negative: int = -10
let large: int = 1000000
```

**Float (`float`):**
```astrixa
let price: float = 19.99
let ratio: float = 0.5
let scientific: float = 1.5e10
```

**Unsigned 256-bit (`U256`):** For blockchain applications
```astrixa
let tokenAmount: U256 = 1000000000000000000  // 1 ETH in wei
let balance: U256 = 500000
```

### Text

**String (`string`):**
```astrixa
let name: string = "Alice"
let message: string = 'Hello, World!'
let multiline: string = """
    This is a
    multi-line string
"""
```

### Boolean

**Bool (`bool`):**
```astrixa
let active: bool = true
let verified: bool = false
```

### Blockchain Types

**Address (`Address`):** Ethereum-compatible addresses
```astrixa
let userAddress: Address = 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb
let contractAddr: Address = 0x1234567890123456789012345678901234567890
```

## Collection Types

### Arrays

Fixed or dynamic sequences of the same type:

```astrixa
let numbers: [int] = [1, 2, 3, 4, 5]
let names: [string] = ["Alice", "Bob", "Charlie"]
let mixed = [1, "two", 3]  // Not allowed - must be same type
```

**Array operations:**
```astrixa
let first = numbers[0]         // Access element
let length = numbers.length    // Get length
numbers.push(6)               // Add element (if mutable)
```

### Maps

Key-value pairs with typed keys and values:

```astrixa
let scores: map<string, int> = {
    "Alice": 95,
    "Bob": 87
}

let balances: map<Address, U256> = {
    0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb: 1000000
}
```

**Map operations:**
```astrixa
let score = scores["Alice"]     // Get value
scores["Charlie"] = 92         // Set value
let exists = scores.has("Bob")  // Check key exists
```

### JSON Objects

Dynamic objects for web APIs:

```astrixa
let user = {
    name: "Alice",
    age: 30,
    email: "alice@example.com",
    settings: {
        theme: "dark",
        notifications: true
    }
}

// Access properties
let userName = user.name
let theme = user.settings.theme
```

## Type Annotations

### When to Use

**Explicit (required):**
```astrixa
// Function parameters
fn greet(name: string) -> string {
    return "Hello, " + name
}

// Contract state variables
contract Token {
    state balance: U256
    state owner: Address
}
```

**Inferred (optional):**
```astrixa
let count = 42              // int inferred
let price = 19.99           // float inferred
let name = "Alice"          // string inferred
let active = true           // bool inferred
```

**Ambiguous (needs annotation):**
```astrixa
let value: float = 42       // Without annotation, would be int
let empty: [string] = []    // Empty array needs type
```

## Type Conversion

### Explicit Casting

```astrixa
let x: int = 42
let y: float = float(x)     // int to float

let s: string = "123"
let n: int = int(s)         // string to int

let addr: string = string(0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb)
```

### Implicit Conversion

Some types convert automatically in safe contexts:

```astrixa
let x: int = 42
let y: float = x + 1.5      // int promoted to float
```

## Function Types

Functions are first-class values:

```astrixa
// Function type signature
let operation: fn(int, int) -> int

// Assign function
operation = fn(a: int, b: int) -> int {
    return a + b
}

let result = operation(5, 3)  // 8
```

### Higher-Order Functions

```astrixa
fn apply(f: fn(int) -> int, value: int) -> int {
    return f(value)
}

let double = fn(x: int) -> int { return x * 2 }
let result = apply(double, 5)  // 10
```

## Optional Types (coming soon)

Handle nullable values safely:

```astrixa
let name: string? = null    // Optional string

if name != null {
    print(name)
}

// Unwrap with default
let displayName = name ?? "Guest"
```

## Result Types (coming soon)

Explicit error handling:

```astrixa
fn divide(a: int, b: int) -> Result<int, string> {
    if b == 0 {
        return Err("Division by zero")
    }
    return Ok(a / b)
}

let result = divide(10, 2)
match result {
    Ok(value) => print(value),
    Err(error) => print("Error: " + error)
}
```

## Union Types (future)

Multiple possible types:

```astrixa
let value: int | string = 42
value = "hello"  // Also valid

fn process(input: int | string) {
    if typeof(input) == "int" {
        // Handle as int
    } else {
        // Handle as string
    }
}
```

## Type Aliases (future)

Create named types:

```astrixa
type UserId = int
type Email = string
type Balance = U256

let userId: UserId = 12345
let email: Email = "user@example.com"
```

## Struct Types (future)

Custom data structures:

```astrixa
struct User {
    id: int,
    name: string,
    email: string,
    verified: bool
}

let user = User {
    id: 1,
    name: "Alice",
    email: "alice@example.com",
    verified: true
}

print(user.name)  // Alice
```

## Type Checking

### Type Guards

```astrixa
let value: any = 42

if typeof(value) == "int" {
    // value is treated as int here
    let doubled = value * 2
}
```

### Type Assertions (future)

```astrixa
let value: any = "hello"
let text = value as string  // Assert value is string
```

## Generic Functions (future)

Write functions that work with multiple types:

```astrixa
fn first<T>(items: [T]) -> T {
    return items[0]
}

let num = first([1, 2, 3])        // T = int
let name = first(["a", "b", "c"]) // T = string
```

## Type Safety Best Practices

### ✅ Do: Use Explicit Types for Clarity

```astrixa
fn calculatePrice(quantity: int, unitPrice: float) -> float {
    return float(quantity) * unitPrice
}
```

### ✅ Do: Leverage Type Inference

```astrixa
let count = 0          // Clear from context
let name = "Alice"     // Obvious type
```

### ❌ Don't: Mix Types Unsafely

```astrixa
// ❌ Bad - type confusion
let value = "42"
let result = value + 10  // Error: can't add string and int

// ✅ Good - explicit conversion
let result = int(value) + 10
```

### ✅ Do: Use Blockchain Types Correctly

```astrixa
contract Token {
    state balances: map<Address, U256>  // Correct types for blockchain
    
    fn transfer(to: Address, amount: U256) {
        // Type-safe blockchain operations
    }
}
```

## Type Errors

Common type errors and solutions:

**Type mismatch:**
```astrixa
// ❌ Error
let x: int = "42"

// ✅ Fix
let x: int = int("42")
```

**Wrong function signature:**
```astrixa
// ❌ Error
fn greet(name: string) { }
let result: string = greet("Alice")  // Function returns void

// ✅ Fix
fn greet(name: string) -> string {
    return "Hello, " + name
}
```

**Array type mismatch:**
```astrixa
// ❌ Error
let numbers: [int] = [1, 2, "three"]

// ✅ Fix
let numbers: [int] = [1, 2, 3]
```

## Next Steps

- [Async Programming →](async.md)
- [Error Handling →](errors.md)
- [Standard Library →](../stdlib/web.md)

---

**Reference:** See the [Standard Library](../stdlib/) for built-in type utilities.
