# RFC 0003: Smart Contract Restricted Subset

- **Start Date:** 2025-01-06
- **Author:** Core Team
- **Status:** Accepted
- **Tracking Issue:** #TBD

## Summary

Define a restricted subset of ASTRIXA for smart contracts to ensure deterministic execution, prevent security vulnerabilities, and enable formal verification.

## Motivation

**Smart contracts are different from regular programs:**
- Must be deterministic (same input → same output always)
- Execute in adversarial environment
- Bugs can lose millions of dollars
- Must be verifiable and auditable
- Gas costs matter

**Current problem:**
```astrixa
// This works in regular ASTRIXA but is DANGEROUS in contracts
contract BadContract {
    fn risky_function() {
        let data = read("external_file.txt");     // ❌ Non-deterministic
        let price = http_get("api.com/price");    // ❌ Network dependency
        let rand = random();                       // ❌ Non-deterministic
        
        // All of these make contract unpredictable!
    }
}
```

**Solution: Restricted subset enforced at compile time**

## Guide-level Explanation

### What's Allowed in Contracts

```astrixa
contract SafeToken {
    state: ["balances", "total_supply"]
    
    // ✅ ALLOWED: Deterministic logic
    fn transfer(to: Address, amount: U256) -> bool {
        // ✅ State access
        let sender_balance = state["balances"][msg.sender] or 0;
        
        // ✅ Validation
        require(to != "0x0", "Invalid address");
        require(amount > 0, "Invalid amount");
        require(sender_balance >= amount, "Insufficient balance");
        
        // ✅ Arithmetic
        state["balances"][msg.sender] = sender_balance - amount;
        state["balances"][to] = (state["balances"][to] or 0) + amount;
        
        // ✅ Events
        emit("Transfer", {
            from: msg.sender,
            to: to,
            value: amount
        });
        
        // ✅ Blockchain context
        let timestamp = tx.timestamp;
        let sender = msg.sender;
        
        return true;
    }
    
    // ✅ ALLOWED: Pure functions
    fn calculate_fee(amount: U256) -> U256 {
        return amount * 3 / 1000;  // 0.3% fee
    }
    
    // ✅ ALLOWED: AI inference (deterministic model)
    fn moderate_content(text: string) -> bool {
        let sentiment = ai.infer(ai.model("sentiment"), text);
        return sentiment.score > 0.7;
    }
}
```

### What's NOT Allowed in Contracts

```astrixa
contract DangerousContract {
    // ❌ File system operations
    fn read_config() {
        let config = read("config.txt");  // COMPILE ERROR
    }
    
    // ❌ Network requests
    fn get_price() {
        let price = http_get("api.com/price");  // COMPILE ERROR
    }
    
    // ❌ Random numbers
    fn generate_id() {
        return random();  // COMPILE ERROR
    }
    
    // ❌ Non-deterministic time
    fn current_time() {
        return now();  // COMPILE ERROR (use tx.timestamp)
    }
    
    // ❌ Floating point
    fn calculate(x: f64) {  // COMPILE ERROR
        return x * 1.5;
    }
    
    // ❌ Dynamic memory allocation
    fn allocate() {
        let vec = Vec::new();  // COMPILE ERROR
        vec.push(42);
    }
    
    // ❌ Unbounded loops
    fn dangerous_loop() {
        loop {  // COMPILE ERROR: No gas limit
            // Could run forever!
        }
    }
}
```

### Compiler Enforcement

```rust
Error: File system operations not allowed in contracts
  --> contract.ax:5
   |
5  |     let data = read("file.txt");
   |                ^^^^ Cannot use 'read' in contract mode
   |
   = help: Contracts must be deterministic
   = note: Consider storing data on-chain or using oracles

Error: Network operations not allowed in contracts
  --> contract.ax:10
   |
10 |     let price = http_get("api.com");
   |                 ^^^^^^^^ Cannot use 'http_get' in contract mode
   |
   = help: Use oracles like Chainlink for external data
   = note: See: https://docs.astrixa.dev/oracles
```

## Reference-level Explanation

### Forbidden Operations

#### 1. File System I/O

**Forbidden functions:**
- `read(path)` - Read file
- `write(path, data)` - Write file
- `open(path)` - Open file
- `delete(path)` - Delete file
- `exists(path)` - Check file exists
- `list_dir(path)` - List directory

**Reason:** Files are external state, non-deterministic

#### 2. Network Operations

**Forbidden functions:**
- `http_get(url)` - HTTP GET request
- `http_post(url, data)` - HTTP POST request
- `fetch(url)` - Fetch resource
- `ws_connect(url)` - WebSocket connection
- `tcp_connect(host, port)` - TCP connection

**Reason:** Network is non-deterministic, can change between executions

#### 3. Random Number Generation

**Forbidden functions:**
- `random()` - Random number
- `rand(min, max)` - Random in range
- `uuid()` - Random UUID

**Reason:** Non-deterministic

**Alternative:** Use block hash + nonce (predictable but sufficient)
```astrixa
fn pseudo_random(nonce: U256) -> U256 {
    return keccak256(chain.block_hash + nonce);
}
```

#### 4. System Time

**Forbidden functions:**
- `now()` - Current system time
- `date()` - Current date
- `timestamp()` - Unix timestamp

**Reason:** System time varies across nodes

**Alternative:** Use `tx.timestamp` (block timestamp)
```astrixa
fn get_time() -> U256 {
    return tx.timestamp;  // ✅ Deterministic (block time)
}
```

#### 5. Floating Point

**Forbidden types:**
- `f32` - 32-bit float
- `f64` - 64-bit float

**Reason:** Float operations are platform-dependent, rounding issues

**Alternative:** Use fixed-point arithmetic with U256
```astrixa
// ❌ Floating point
let price = 1.5 * amount;

// ✅ Fixed-point (18 decimals)
let price = (amount * 15) / 10;  // 1.5x
```

#### 6. Dynamic Memory

**Forbidden operations:**
- `Vec::new()` - Dynamic vector
- `HashMap::new()` - Dynamic hash map
- `String::new()` - Dynamic string
- `Box::new()` - Heap allocation

**Reason:** Unbounded gas costs, potential DoS

**Alternative:** Fixed-size arrays, bounded collections
```astrixa
// ❌ Dynamic vector
let mut vec = Vec::new();
vec.push(item);

// ✅ Fixed-size array
let arr: [Item; 100] = [];
arr[0] = item;
```

#### 7. Unbounded Loops

**Forbidden:**
```astrixa
// ❌ Infinite loop potential
loop {
    // No gas limit check
}

// ❌ Unbounded iteration
for item in unknown_size_array {
    // Could exceed gas limit
}
```

**Required:**
```astrixa
// ✅ Gas-bounded loop
let max_iterations = 100;
for i in 0..min(max_iterations, array.len()) {
    // Limited iterations
}
```

### Allowed Operations

#### ✅ State Management

```astrixa
state["key"] = value;
let val = state["key"];
state["map"][address] = balance;
```

#### ✅ Blockchain Context

```astrixa
msg.sender    // Transaction sender
msg.value     // ETH sent
tx.timestamp  // Block time
tx.gasprice   // Gas price
chain.id      // Chain ID
```

#### ✅ Validation

```astrixa
require(condition, "Error message");
assert(condition);
```

#### ✅ Events

```astrixa
emit("EventName", { data });
```

#### ✅ Deterministic AI

```astrixa
// AI with deterministic model
let result = ai.infer(ai.model("sentiment"), text);
```

#### ✅ Cryptography

```astrixa
sha256(data);
keccak256(data);
sign(data, key);
verify(signature, data, pubkey);
```

#### ✅ Math Operations

```astrixa
x + y
x - y
x * y
x / y
x % y
pow(x, y)
sqrt(x)
```

## Drawbacks

### Limited Functionality

**Contracts can't:**
- Access external data directly (need oracles)
- Use random numbers (need VRF)
- Do complex math (gas limits)

### Learning Curve

**Developers need to learn:**
- Why restrictions exist
- How to work around them
- When to use off-chain computation

## Alternatives

### Alternative 1: No Restrictions

**Allow everything:**
```astrixa
contract Unrestricted {
    fn anything_goes() {
        let data = http_get("api.com");  // Allowed
        let rand = random();              // Allowed
    }
}
```

**Rejected:** Leads to non-deterministic contracts, security issues, consensus failures

### Alternative 2: Runtime Checks

**Check at runtime instead of compile time:**
```astrixa
contract RuntimeChecks {
    fn risky() {
        if is_contract_mode() {
            panic("Not allowed!");
        }
        let data = read("file.txt");
    }
}
```

**Rejected:** Errors found too late, deployment wastes gas

### Alternative 3: Sandboxing

**Run contract in sandbox:**
- Intercept forbidden calls
- Throw errors at runtime

**Rejected:** Performance overhead, complexity, still fails at runtime

### Decision

**Compile-time enforcement is best:**
- Catch errors early
- Zero runtime overhead
- Clear documentation
- Better UX

## Prior Art

### Solidity

**Restrictions:**
- No file I/O (no filesystem)
- No network calls
- No true random
- Limited types

**Differences:**
- Solidity enforces at runtime (revert)
- ASTRIXA enforces at compile time
- ASTRIXA has better error messages

### Move (Aptos/Sui)

**Restrictions:**
- Resource types (no copy/drop)
- No dynamic dispatch
- No recursion

**Similarities:**
- Static analysis for safety
- Formal verification focus

### Teal (Algorand)

**Restrictions:**
- Very limited operations
- Stack-based
- No loops

**Differences:**
- Teal is more restricted
- ASTRIXA is higher-level

## Unresolved Questions

1. **Should we allow limited dynamic allocation?**
   - e.g., `Vec` with max size?
   
2. **How to handle upgradeable contracts?**
   - Proxy pattern allowed?
   
3. **Bounded loops - what's the limit?**
   - 1000 iterations? 10,000?

## Future Possibilities

### Formal Verification

```astrixa
contract VerifiedToken {
    // Formal spec
    invariant total_supply == sum(balances);
    
    fn transfer(to: Address, amount: U256) {
        // Compiler verifies invariant holds
    }
}
```

### Gradual Relaxation

**For Layer 2s with different security models:**
```astrixa
#[target = "optimistic_rollup"]
contract RelaxedContract {
    // More operations allowed on L2
}
```

## Implementation Plan

### Phase 1: Detection (v0.5)
- Identify forbidden operations
- Basic error messages

### Phase 2: Enforcement (v0.5)
- Compile-time errors
- Clear error messages with suggestions

### Phase 3: Documentation (v0.5)
- Security guide
- Best practices
- Migration from Solidity

### Phase 4: Verification (v1.0)
- Static analysis
- Formal verification tools

**Estimated Effort:** Medium (1-2 months)
**Target Release:** v0.5.0

## Backwards Compatibility

**Breaking change:** No

**Rationale:**
- New restriction (not removing features)
- Only affects `contract` blocks
- Regular code unaffected

**Migration:** N/A (new feature)

## Security Considerations

**This RFC IS a security feature.**

**Prevents:**
- Non-deterministic execution
- Consensus failures
- Unpredictable gas costs
- Reentrancy attacks (partially)
- Oracle manipulation

**Does NOT prevent:**
- Logic bugs (need formal verification)
- Front-running (need MEV protection)
- Economic attacks (need game theory)

## Performance Considerations

**Compile time:** +5% (static analysis)
**Runtime:** No impact (compile-time only)
**Gas:** Potentially lower (prevents inefficient patterns)

## Tooling Impact

### LSP

- Real-time validation
- Suggest contract-safe alternatives
- Highlight forbidden operations

### VS Code Extension

- Syntax highlighting for errors
- Quick fixes (suggest alternatives)

### Documentation

- Security guide
- Contract best practices
- Oracle integration

## Teaching

### Tutorial

**Chapter: "Writing Safe Smart Contracts"**

Topics:
1. Why restrictions exist
2. What's allowed and not
3. Working with restrictions
4. Using oracles
5. Testing contracts

### Examples

- Safe ERC20 implementation
- DEX with oracles
- DAO with time locks

## References

- Solidity security: https://consensys.github.io/smart-contract-best-practices/
- Move language: https://move-language.github.io/move/
- Formal verification: https://arxiv.org/abs/2104.04896

---

**Status:** ✅ Accepted  
**Implemented:** v0.5.0  
**Last Updated:** January 2025
