# Error Handling

Build robust applications with proper error handling in ASTRIXA.

## Error Philosophy

ASTRIXA follows these principles:
1. **Explicit errors** - Errors are visible in function signatures
2. **Fail fast** - Catch errors early, handle them properly
3. **Recovery** - Give programs a chance to recover gracefully

## Panic

### When to Use Panic

`panic` stops execution immediately. Use it for:
- Programming errors (bugs)
- Invalid state that should never happen
- Unrecoverable errors

```astrixa
fn divide(a: int, b: int) -> int {
    if b == 0 {
        panic("Division by zero")
    }
    return a / b
}
```

### Panic with Context

Provide helpful error messages:

```astrixa
fn processUser(id: int) {
    if id < 0 {
        panic("User ID cannot be negative: " + id)
    }
    
    let user = db.getUser(id)
    
    if user == null {
        panic("User not found: " + id)
    }
    
    // Process user...
}
```

## Try-Catch (coming soon)

### Basic Try-Catch

```astrixa
try {
    let data = riskyOperation()
    print("Success: " + data)
} catch error {
    print("Error: " + error.message)
}
```

### Specific Error Types

```astrixa
try {
    let file = fs.readFile("config.json")
    let config = json.parse(file)
} catch error {
    if error.type == "FileNotFound" {
        print("Config file missing")
    } else if error.type == "ParseError" {
        print("Invalid JSON in config")
    } else {
        throw error  // Re-throw unknown errors
    }
}
```

### Finally Block

```astrixa
let file = null

try {
    file = fs.open("data.txt")
    let content = file.read()
    processContent(content)
} catch error {
    print("Error reading file: " + error.message)
} finally {
    if file != null {
        file.close()  // Always cleanup
    }
}
```

## Result Type (coming soon)

### Explicit Error Returns

Instead of exceptions, use `Result<T, E>`:

```astrixa
fn divide(a: int, b: int) -> Result<int, string> {
    if b == 0 {
        return Err("Division by zero")
    }
    return Ok(a / b)
}

// Usage
let result = divide(10, 2)

match result {
    Ok(value) => print("Result: " + value),
    Err(error) => print("Error: " + error)
}
```

### Chaining Results

```astrixa
fn processData() -> Result<string, string> {
    let file = fs.readFile("data.txt")?  // ? propagates errors
    let parsed = json.parse(file)?
    let validated = validate(parsed)?
    return Ok(validated)
}
```

### Result Helpers

```astrixa
// Unwrap with default
let value = result.unwrapOr(42)

// Unwrap or panic
let value = result.unwrap()

// Transform success value
let doubled = result.map(fn(x) { return x * 2 })

// Transform error
let friendlyError = result.mapErr(fn(e) { 
    return "Friendly: " + e 
})
```

## Option Type (coming soon)

### Handling Nullable Values

```astrixa
fn findUser(id: int) -> Option<User> {
    let user = db.query("SELECT * FROM users WHERE id = " + id)
    
    if user == null {
        return None
    }
    return Some(user)
}

// Usage
let userOption = findUser(123)

match userOption {
    Some(user) => print("Found: " + user.name),
    None => print("User not found")
}
```

### Option Helpers

```astrixa
// Unwrap with default
let user = userOption.unwrapOr(defaultUser)

// Check if has value
if userOption.isSome() {
    let user = userOption.unwrap()
}

// Transform value if present
let name = userOption.map(fn(u) { return u.name })
```

## Error Types

### Built-in Error Types

```astrixa
// File system errors
FileNotFound
PermissionDenied
InvalidPath

// Network errors
ConnectionFailed
Timeout
InvalidResponse

// Parse errors
ParseError
InvalidJSON
InvalidNumber

// Blockchain errors
InsufficientFunds
GasExhausted
RevertError
```

### Custom Error Types (future)

```astrixa
enum ValidationError {
    TooShort(int),
    TooLong(int),
    InvalidFormat(string)
}

fn validatePassword(pwd: string) -> Result<string, ValidationError> {
    if pwd.length < 8 {
        return Err(ValidationError.TooShort(pwd.length))
    }
    
    if pwd.length > 128 {
        return Err(ValidationError.TooLong(pwd.length))
    }
    
    if !pwd.matches("[0-9]") {
        return Err(ValidationError.InvalidFormat("Must contain numbers"))
    }
    
    return Ok(pwd)
}
```

## Error Propagation

### The ? Operator (coming soon)

Automatically propagate errors up the call stack:

```astrixa
fn loadConfig() -> Result<Config, string> {
    let file = fs.readFile("config.json")?  // Returns early if error
    let parsed = json.parse(file)?
    let validated = validate(parsed)?
    return Ok(validated)
}

// Equivalent to:
fn loadConfigVerbose() -> Result<Config, string> {
    let file = fs.readFile("config.json")
    if file.isErr() {
        return file
    }
    
    let parsed = json.parse(file.unwrap())
    if parsed.isErr() {
        return parsed
    }
    
    // ... and so on
}
```

### Manual Propagation

```astrixa
fn processData() -> Result<string, string> {
    let result = riskyOperation()
    
    if result.isErr() {
        return result  // Propagate error
    }
    
    let data = result.unwrap()
    return Ok(data.toUpperCase())
}
```

## Async Error Handling

### Try-Catch with Async

```astrixa
async fn fetchData(url: string) -> string {
    try {
        let response = await http.get(url)
        return response.body
    } catch error {
        print("Failed to fetch: " + error.message)
        return "fallback data"
    }
}
```

### Async Result

```astrixa
async fn fetchDataSafe(url: string) -> Result<string, string> {
    try {
        let response = await http.get(url)
        if response.status != 200 {
            return Err("HTTP " + response.status)
        }
        return Ok(response.body)
    } catch error {
        return Err(error.message)
    }
}
```

## Smart Contract Errors

### Revert with Message

```astrixa
contract Token {
    state balances: map<Address, U256>
    
    fn transfer(to: Address, amount: U256) {
        let senderBalance = balances[tx.sender]
        
        if senderBalance < amount {
            panic("Insufficient balance")  // Reverts transaction
        }
        
        balances[tx.sender] -= amount
        balances[to] += amount
    }
}
```

### Require Helper (coming soon)

```astrixa
contract Token {
    fn transfer(to: Address, amount: U256) {
        require(balances[tx.sender] >= amount, "Insufficient balance")
        require(to != 0x0, "Invalid recipient")
        require(amount > 0, "Amount must be positive")
        
        balances[tx.sender] -= amount
        balances[to] += amount
    }
}
```

### Custom Contract Errors (future)

```astrixa
contract Token {
    error InsufficientBalance(U256 requested, U256 available)
    error InvalidRecipient(Address recipient)
    
    fn transfer(to: Address, amount: U256) {
        if to == 0x0 {
            revert InvalidRecipient(to)
        }
        
        let balance = balances[tx.sender]
        if balance < amount {
            revert InsufficientBalance(amount, balance)
        }
        
        balances[tx.sender] -= amount
        balances[to] += amount
    }
}
```

## Error Handling Best Practices

### ✅ Do: Be Specific

```astrixa
// ✅ Good - specific message
if age < 0 {
    panic("Age cannot be negative: got " + age)
}

// ❌ Bad - vague message
if age < 0 {
    panic("Invalid input")
}
```

### ✅ Do: Handle Errors Close to Source

```astrixa
// ✅ Good - handle immediately
fn loadUser(id: int) -> User {
    try {
        return db.getUser(id)
    } catch error {
        print("Failed to load user " + id + ": " + error.message)
        return defaultUser()
    }
}
```

### ✅ Do: Clean Up Resources

```astrixa
// ✅ Good - always cleanup
let connection = null
try {
    connection = db.connect()
    let data = connection.query("SELECT * FROM users")
    processData(data)
} finally {
    if connection != null {
        connection.close()
    }
}
```

### ❌ Don't: Swallow Errors Silently

```astrixa
// ❌ Bad - error disappears
try {
    riskyOperation()
} catch error {
    // Nothing!
}

// ✅ Good - at least log it
try {
    riskyOperation()
} catch error {
    print("Error in riskyOperation: " + error.message)
}
```

### ✅ Do: Validate Early

```astrixa
fn processOrder(order: Order) {
    // Validate all inputs first
    require(order.id > 0, "Invalid order ID")
    require(order.items.length > 0, "Order has no items")
    require(order.total > 0, "Invalid total")
    
    // Then proceed with processing
    let result = saveOrder(order)
    sendConfirmation(order.email)
}
```

## Testing Error Handling

```astrixa
fn testDivideByZero() {
    try {
        divide(10, 0)
        assert(false, "Should have panicked")
    } catch error {
        assert(error.message.contains("Division by zero"))
    }
}

fn testFileNotFound() {
    let result = fs.readFile("nonexistent.txt")
    
    assert(result.isErr())
    assert(result.err().type == "FileNotFound")
}
```

## Common Error Patterns

### Retry Logic

```astrixa
async fn fetchWithRetry(url: string, maxRetries: int) -> Result<string, string> {
    let attempts = 0
    
    while attempts < maxRetries {
        try {
            let response = await http.get(url)
            return Ok(response.body)
        } catch error {
            attempts += 1
            if attempts >= maxRetries {
                return Err("Failed after " + maxRetries + " attempts")
            }
            await sleep(1000 * attempts)  // Exponential backoff
        }
    }
}
```

### Fallback Values

```astrixa
fn getConfig(key: string) -> string {
    try {
        return configFile.get(key)
    } catch error {
        // Return sensible default
        return defaultConfigs[key]
    }
}
```

### Circuit Breaker (future)

```astrixa
let circuitOpen = false
let failureCount = 0

async fn callWithCircuitBreaker(url: string) -> Result<string, string> {
    if circuitOpen {
        return Err("Circuit breaker open")
    }
    
    try {
        let response = await http.get(url)
        failureCount = 0  // Reset on success
        return Ok(response.body)
    } catch error {
        failureCount += 1
        
        if failureCount > 5 {
            circuitOpen = true
            // Reset after timeout
            setTimeout(fn() { circuitOpen = false }, 60000)
        }
        
        return Err(error.message)
    }
}
```

## Next Steps

- [Modules →](modules.md)
- [Testing →](../testing.md)
- [Standard Library →](../stdlib/web.md)

---

**Philosophy:** Errors are not exceptions. They're expected outcomes that deserve first-class treatment.
