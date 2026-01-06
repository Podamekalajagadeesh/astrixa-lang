# RFC 0002: Async/Await Model

- **Start Date:** 2025-01-06
- **Author:** Core Team
- **Status:** Draft
- **Tracking Issue:** TBD

## Summary

Add async/await syntax to ASTRIXA for non-blocking I/O operations, particularly important for Web3 RPC calls, web servers, and AI inference.

## Motivation

**Current problem:**
```astrixa
// Synchronous - blocks entire thread
let balance = web3.balance(address);
let price = http_get("api.example.com/price");
let result = ai.infer(model, data);

// If any operation is slow, everything waits
```

**With async/await:**
```astrixa
// Concurrent - operations run in parallel
async fn get_data() {
    let balance = await web3.balance(address);
    let price = await http_get("api.example.com/price");
    let result = await ai.infer(model, data);
    // Runs concurrently!
}
```

**Why we need this:**
- Web3 RPC calls are slow (100-500ms)
- Web servers need to handle multiple requests
- AI inference can be long-running
- I/O operations dominate execution time

## Guide-level Explanation

### Basic Usage

```astrixa
// Mark function as async
async fn fetch_balance(address: Address) -> U256 {
    // Await async operation
    let balance = await web3.balance(address);
    return balance;
}

// Call async function
async fn main() {
    let balance = await fetch_balance("0x...");
    println("Balance: " + balance);
}
```

### Concurrent Operations

```astrixa
// Run multiple operations concurrently
async fn get_balances(addresses: [Address]) -> [U256] {
    let futures = [];
    
    // Start all operations
    for addr in addresses {
        futures.push(web3.balance(addr));
    }
    
    // Wait for all to complete
    return await Promise.all(futures);
}
```

### Error Handling

```astrixa
async fn safe_fetch() -> Result<U256, Error> {
    try {
        let balance = await web3.balance(address);
        return Ok(balance);
    } catch (error) {
        return Err(error);
    }
}
```

### Web Server Example

```astrixa
import std::web;

async fn main() {
    let server = web.server(3000);
    
    // Async route handler
    server.get("/balance/:address", async fn(req) {
        let address = req.params["address"];
        let balance = await web3.balance(address);
        
        return web.json({
            address: address,
            balance: web3.fromWei(balance, "ether")
        });
    });
    
    await server.listen();
}
```

## Reference-level Explanation

### Syntax

**Function Declaration:**
```
async fn name(params) -> ReturnType {
    body
}
```

**Await Expression:**
```
let result = await async_operation();
```

### Semantics

**Async Functions:**
- Return a `Promise<T>` (or `Future<T>`)
- Can call other async functions
- Can use `await` keyword
- Execute immediately until first `await`

**Await Operator:**
- Suspends function execution
- Yields control to runtime
- Resumes when Promise resolves
- Can only be used in async functions

### Type System

**Promise Type:**
```astrixa
type Promise<T> = {
    then: (T -> void) -> Promise<void>,
    catch: (Error -> void) -> Promise<void>,
    await: () -> T
}
```

**Type Inference:**
```astrixa
async fn get_data() -> U256 {
    return await web3.balance(address);  // Infers: Promise<U256>
}
```

### Runtime Model

**Executor:**
- Single-threaded event loop (like Node.js)
- Multiple async tasks can be pending
- Only one task executes at a time
- Fairness scheduler (no starvation)

**Promise Resolution:**
1. Async function called → Promise created
2. Function executes until `await`
3. `await` suspends → control to event loop
4. Promise resolves → function resumes
5. Function completes → Promise resolves

### Implementation

**Lexer:**
- Add `async` keyword (token)
- Add `await` keyword (token)

**Parser:**
- Parse `async fn` declarations
- Parse `await` expressions
- Validate `await` only in async functions

**Type Checker:**
- Check Promise types
- Ensure `await` in async context
- Infer return types

**Compiler:**
- Transform to state machine
- Generate bytecode for suspend/resume
- Handle Promise creation/resolution

## Drawbacks

### Complexity

- Adds async runtime to language
- More complex mental model
- Debugging is harder
- Stack traces are confusing

### Performance

- Overhead of Promise creation
- State machine allocation
- Event loop scheduling

### Learning Curve

- Developers need to understand async
- When to use async vs sync
- Avoiding common pitfalls (blocking in async)

## Alternatives

### Alternative 1: Callbacks

```astrixa
// Old style - callback hell
web3.balance(address, fn(balance) {
    http_get("api.com", fn(price) {
        ai.infer(model, data, fn(result) {
            // Nested callbacks
        });
    });
});
```

**Rejected:** Leads to callback hell, poor error handling

### Alternative 2: Green Threads

```astrixa
// Implicit async - Go style
fn main() {
    spawn(fn() {
        let balance = web3.balance(address);  // Blocks green thread
    });
}
```

**Pros:** Simpler model, no async/await needed
**Cons:** Hidden complexity, harder to control, memory overhead

**Deferred:** Maybe in v2.0, but async/await first

### Alternative 3: No Async

**Just block:**
```astrixa
let balance = web3.balance(address);  // Just waits
```

**Rejected:** Web servers would be single-threaded, terrible performance

## Prior Art

### JavaScript (Async/Await)

```javascript
async function getData() {
    const balance = await web3.getBalance(address);
    return balance;
}
```

**Good:** Familiar syntax, widely used
**Bad:** Uncolored functions, Promise confusion

### Rust (Async/Await)

```rust
async fn get_data() -> Result<u64, Error> {
    let balance = web3.balance(address).await?;
    Ok(balance)
}
```

**Good:** Zero-cost, composable
**Bad:** Complex (multiple runtimes), steep learning curve

### Python (Async/Await)

```python
async def get_data():
    balance = await web3.balance(address)
    return balance
```

**Good:** Simple, clean syntax
**Bad:** Runtime overhead, slower

### Go (Goroutines)

```go
func getData() {
    go func() {
        balance := web3.Balance(address)
    }()
}
```

**Good:** Simple model, no keywords
**Bad:** Hidden complexity, memory overhead

## Unresolved Questions

1. **Single runtime or multiple?**
   - Option A: One global runtime (like Node.js)
   - Option B: Multiple runtimes (like Rust)

2. **Promise vs Future naming?**
   - JavaScript uses Promise
   - Rust uses Future
   - Which is clearer?

3. **Cancellation?**
   - Should Promises be cancellable?
   - How to handle timeout?

4. **Structured concurrency?**
   - Should we enforce cleanup?
   - Task nurseries (like Trio)?

## Future Possibilities

### Async Streams

```astrixa
async fn* generate_blocks() {
    loop {
        let block = await web3.getBlock("latest");
        yield block;
        await sleep(12000);  // 12 seconds
    }
}
```

### Parallel Execution

```astrixa
// Use all CPU cores
let results = await parallel([
    async { compute1() },
    async { compute2() },
    async { compute3() }
]);
```

### Async Traits

```astrixa
trait AsyncReader {
    async fn read(&mut self, buf: [u8]) -> usize;
}
```

## Implementation Plan

### Phase 1: Core Syntax (v0.5)
- Lexer: `async`, `await` keywords
- Parser: async function declarations
- Type checker: Promise types
- Basic runtime (single-threaded)

### Phase 2: Standard Library (v0.6)
- Promise utilities
- Async I/O (files, network)
- Async Web3 operations
- Async AI inference

### Phase 3: Optimization (v0.7)
- Zero-allocation Promises
- Inline small async functions
- Reduce state machine overhead

### Phase 4: Advanced Features (v0.8+)
- Async streams
- Cancellation
- Structured concurrency

**Estimated Effort:** Large (3-4 months)
**Target Release:** v0.5.0

## Backwards Compatibility

**Breaking change:** No

**New syntax:** `async` and `await` are new keywords
- May break code using these as identifiers
- Rare in practice
- Easy to rename

**Migration:**
```astrixa
// Before (synchronous)
fn get_balance(addr: Address) -> U256 {
    return web3.balance(addr);  // Blocks
}

// After (can keep synchronous if desired)
fn get_balance(addr: Address) -> U256 {
    return web3.balance(addr);  // Still works!
}

// Or make async
async fn get_balance(addr: Address) -> U256 {
    return await web3.balance(addr);  // Non-blocking
}
```

## Security Considerations

### DoS Prevention

**Problem:** Infinite async loops could block runtime

**Solution:**
- Gas metering for async operations
- Timeout mechanisms
- Max pending tasks limit

### Reentrancy

**Problem:** Async can enable reentrancy attacks

**Solution:**
- Smart contracts are synchronous only
- Async only for off-chain code
- Clear separation

## Performance Considerations

**Compile time:** +5-10% (state machine generation)
**Runtime:** 
- Single Promise: ~10ns overhead
- Await: ~20ns suspend/resume
- Event loop: ~1μs per iteration

**Gas cost (contracts):** N/A - async not allowed in contracts

## Tooling Impact

### LSP

- Autocomplete `async`, `await`
- Suggest `async` when using `await`
- Warn if blocking in async function

### VS Code Extension

- Syntax highlighting for async/await
- Snippets for common patterns

### Package Manager

- Async dependencies declared in manifest

## Teaching

### Tutorial Section

"Async programming allows multiple operations to run concurrently..."

**Topics:**
1. Why async?
2. Basic syntax
3. Common patterns
4. Error handling
5. Best practices

### Examples

- Fetching multiple balances
- Web server with database
- AI inference pipeline
- Real-time data streaming

## References

- JavaScript async/await: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/async_function
- Rust async book: https://rust-lang.github.io/async-book/
- "Structured Concurrency" (Nathaniel Smith): https://vorpus.org/blog/notes-on-structured-concurrency/

---

**Status:** 📝 Draft (needs community feedback)  
**Last Updated:** January 2025
