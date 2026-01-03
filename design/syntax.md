# ASTRIXA Syntax Design

ASTRIXA syntax prioritizes:
- Readability like Python
- Structure like Rust
- Simplicity like Go

The syntax should feel familiar to developers,
but eliminate ambiguity and hidden behavior.

## Basic Program Structure

ASTRIXA files end with:
```
.ax
```

Example:
```ax
app main

fn main() {
    print("Hello, ASTRIXA")
}
```

Rules:
- `app` defines entry module
- `fn` defines functions
- `{}` used for clarity (no indentation bugs)

## Variables (simple + safe)

```ax
let name = "Astrixa"
let age: int = 1
let active: bool = true
```

Rules:
- `let` is immutable by default
- Type inference supported
- Explicit types optional but encouraged

Mutable variable:
```ax
var count = 0
count = count + 1
```

## Functions

```ax
fn add(a: int, b: int) -> int {
    return a + b
}
```

Short form:
```ax
fn square(x: int) => x * x
```

## Control Flow

### If
```ax
if age > 18 {
    print("Adult")
} else {
    print("Minor")
}
```

### Loop
```ax
for i in 0..10 {
    print(i)
}
```

### While
```ax
while active {
    run()
}
```

## Data Structures

### Struct
```ax
struct User {
    id: int
    name: string
    wallet: address
}
```

### Create instance
```ax
let user = User {
    id: 1,
    name: "Alice",
    wallet: "0xabc..."
}
```

## Error Handling (no silent failures)

```ax
fn divide(a: int, b: int) -> Result<int> {
    if b == 0 {
        return error("Division by zero")
    }
    return ok(a / b)
}
```

This avoids Python-style runtime surprises.

## Comments

```ax
// single line

/*
 multi-line
 */
```
