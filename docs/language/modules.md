# Modules

Organize and reuse code with ASTRIXA's module system.

## Module Basics

### What is a Module?

A module is a single ASTRIXA file (`.ax`) that contains related code. Each file is automatically a module.

**File: math.ax**
```astrixa
fn add(a: int, b: int) -> int {
    return a + b
}

fn multiply(a: int, b: int) -> int {
    return a * b
}

const PI = 3.14159
```

### Importing Modules

Use the `use` keyword to import:

**File: main.ax**
```astrixa
use math

let sum = math.add(5, 3)
let product = math.multiply(5, 3)
print(math.PI)
```

## Import Syntax

### Basic Import

```astrixa
use math              // Import entire module
use web               // Import standard library module
use crypto            // Another stdlib module
```

### Specific Imports

Import only what you need:

```astrixa
use math::{add, multiply}  // Import specific functions

let result = add(5, 3)     // No module prefix needed
```

### Wildcard Imports

Import everything (use sparingly):

```astrixa
use math::*

let sum = add(5, 3)        // All functions available directly
let area = PI * radius * radius
```

### Aliasing

Rename modules or functions:

```astrixa
use math as m
let result = m.add(5, 3)

use crypto::{hash as cryptoHash}
let digest = cryptoHash("data")
```

## Standard Library Modules

### Available Modules

```astrixa
use std::io       // Input/output operations
use std::fs       // File system operations
use std::net      // Networking and HTTP
use std::web      // Web server framework
use std::json     // JSON parsing and serialization
use std::crypto   // Cryptographic functions
use std::ai       // AI operations
use std::web3     // Blockchain operations
use std::async    // Async utilities
```

### Using Standard Library

```astrixa
use std::web
use std::json

server {
    route GET "/api/data" {
        let data = { message: "Hello" }
        return json(data)
    }
}

server.run(8080)
```

## Creating Your Own Modules

### Simple Module

**File: utils.ax**
```astrixa
fn greet(name: string) -> string {
    return "Hello, " + name + "!"
}

fn farewell(name: string) -> string {
    return "Goodbye, " + name + "!"
}

const APP_NAME = "MyApp"
const VERSION = "1.0.0"
```

**File: main.ax**
```astrixa
use utils

print(utils.greet("Alice"))
print(utils.APP_NAME + " v" + utils.VERSION)
```

### Module with State

**File: counter.ax**
```astrixa
let count = 0  // Module-level state

fn increment() {
    count = count + 1
}

fn decrement() {
    count = count - 1
}

fn getCount() -> int {
    return count
}
```

**File: main.ax**
```astrixa
use counter

counter.increment()
counter.increment()
print(counter.getCount())  // 2
```

## Module Organization

### Directory Structure

```
my-project/
├── main.ax
├── lib/
│   ├── math.ax
│   ├── string.ax
│   └── validation.ax
└── models/
    ├── user.ax
    └── product.ax
```

### Importing from Subdirectories

```astrixa
use lib::math
use lib::string
use models::user

let result = math.add(5, 3)
let uppercased = string.toUpper("hello")
let newUser = user.create("Alice")
```

## Package System

### Creating a Package

Initialize with `astrixa.toml`:

```toml
[package]
name = "my-math-lib"
version = "1.0.0"
author = "Your Name"

[dependencies]
# Add dependencies here
```

**Package structure:**
```
my-math-lib/
├── astrixa.toml
└── src/
    ├── main.ax
    ├── basic.ax
    └── advanced.ax
```

### Publishing Packages (coming soon)

```bash
astrixa publish
```

### Installing Packages

```bash
astrixa install my-math-lib
```

**Usage in code:**
```astrixa
use my-math-lib

let result = my-math-lib.calculate(42)
```

## Module Visibility (future)

### Public vs Private

```astrixa
// Public (exported)
pub fn publicFunction() {
    // Available to importers
}

// Private (not exported)
fn privateHelper() {
    // Only used internally
}

pub const PUBLIC_CONSTANT = 42
let privateVariable = "internal"
```

### Selective Exports

```astrixa
// Only export specific items
pub use {publicFunction, PUBLIC_CONSTANT}
```

## Circular Dependencies

ASTRIXA detects and prevents circular imports:

**File: a.ax**
```astrixa
use b  // ❌ Error: Circular dependency (a -> b -> a)

fn functionA() {
    b.functionB()
}
```

**File: b.ax**
```astrixa
use a

fn functionB() {
    a.functionA()
}
```

**Solution:** Restructure to avoid cycles:
```
shared.ax  <-- Common code
  ↑    ↑
  a.ax  b.ax
```

## Module Best Practices

### ✅ Do: One Concern Per Module

```astrixa
// ✅ Good - focused modules
use auth      // Authentication only
use database  // Database operations only
use validation // Input validation only
```

### ✅ Do: Use Clear Names

```astrixa
// ✅ Good - descriptive names
use userManagement
use orderProcessing
use emailNotifications

// ❌ Bad - vague names
use utils
use helpers
use misc
```

### ✅ Do: Import at Top

```astrixa
// ✅ Good - all imports at top
use std::web
use std::json
use database

fn handler() {
    // Function code
}
```

### ❌ Don't: Import Everything

```astrixa
// ❌ Bad - namespace pollution
use std::*
use math::*
use crypto::*

// ✅ Good - explicit imports
use std::web
use math::{add, multiply}
use crypto::hash
```

### ✅ Do: Group Related Imports

```astrixa
// ✅ Good - organized imports
// Standard library
use std::web
use std::json

// Third-party packages
use external-api
use database-lib

// Local modules
use models::user
use lib::validation
```

## Advanced Module Patterns

### Re-exporting

**File: lib/index.ax**
```astrixa
use lib::math
use lib::string
use lib::validation

// Re-export for convenient access
pub use math
pub use string
pub use validation
```

**Usage:**
```astrixa
use lib  // Gets math, string, validation

lib.math.add(5, 3)
lib.string.toUpper("hello")
```

### Module Initialization

Execute code when module is imported:

```astrixa
// Module-level initialization
let config = loadConfig()
let database = connectDB()

fn useDatabase() {
    database.query("SELECT * FROM users")
}
```

### Lazy Loading (future)

```astrixa
lazy use heavyModule  // Only loads when first used

fn sometimes() {
    if needHeavyFeature {
        heavyModule.process()  // Loaded here
    }
}
```

## Module Examples

### Web Server Module

**File: server.ax**
```astrixa
use std::web

fn start(port: int) {
    server {
        route GET "/" {
            return json({ status: "ok" })
        }
        
        route GET "/health" {
            return json({ healthy: true })
        }
    }
    
    server.run(port)
}
```

**File: main.ax**
```astrixa
use server

server.start(8080)
```

### Database Module

**File: database.ax**
```astrixa
let connection = null

fn connect(url: string) {
    connection = db.connect(url)
}

async fn query(sql: string) -> [map] {
    if connection == null {
        panic("Database not connected")
    }
    return await connection.execute(sql)
}

fn close() {
    if connection != null {
        connection.close()
        connection = null
    }
}
```

**File: main.ax**
```astrixa
use database

database.connect("postgresql://localhost/mydb")
let users = await database.query("SELECT * FROM users")
database.close()
```

### Validation Module

**File: validation.ax**
```astrixa
fn isEmail(email: string) -> bool {
    return email.matches("^[^@]+@[^@]+\\.[^@]+$")
}

fn isPhoneNumber(phone: string) -> bool {
    return phone.matches("^\\+?[0-9]{10,15}$")
}

fn isStrongPassword(password: string) -> bool {
    return password.length >= 8 &&
           password.matches("[A-Z]") &&
           password.matches("[a-z]") &&
           password.matches("[0-9]")
}

fn validateUser(user: map) -> Result<map, string> {
    if !isEmail(user.email) {
        return Err("Invalid email")
    }
    
    if !isStrongPassword(user.password) {
        return Err("Weak password")
    }
    
    return Ok(user)
}
```

## Module Testing

```astrixa
// File: math_test.ax
use math
use std::testing

fn testAdd() {
    assert(math.add(2, 3) == 5)
    assert(math.add(-1, 1) == 0)
}

fn testMultiply() {
    assert(math.multiply(2, 3) == 6)
    assert(math.multiply(0, 100) == 0)
}
```

## Module Documentation (future)

```astrixa
/// Calculates the sum of two numbers
///
/// # Arguments
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
/// The sum of a and b
///
/// # Example
/// ```
/// let result = add(5, 3)
/// assert(result == 8)
/// ```
pub fn add(a: int, b: int) -> int {
    return a + b
}
```

## Next Steps

- [Standard Library: web →](../stdlib/web.md)
- [Standard Library: web3 →](../stdlib/web3.md)
- [Standard Library: ai →](../stdlib/ai.md)
- [Package Manager →](../../PACKAGE_MANAGER.md)

---

**Philosophy:** Modules should be cohesive, loosely coupled, and easy to understand in isolation.
