# ASTRIXA Gas Model Documentation

## Overview
ASTRIXA implements a deterministic gas model similar to Ethereum but simplified and optimized for clarity. Every instruction has a predictable cost, ensuring security, fairness, and blockchain compatibility.

## Gas Cost Table

| Instruction | Cost | Purpose |
|-------------|------|---------|
| LoadConst | 1 | Push constant to stack |
| LoadVar | 1 | Load variable from memory |
| StoreVar | 2 | Store to memory |
| Pop | 1 | Discard stack top |
| Add | 2 | Addition operation |
| Sub | 2 | Subtraction operation |
| Mul | 5 | Multiplication operation |
| Div | 8 | Division operation |
| Greater | 2 | Greater than comparison |
| Less | 2 | Less than comparison |
| Jump | 1 | Unconditional jump |
| JumpIfFalse | 2 | Conditional jump |
| Call | 10 | Function call |
| Return | 3 | Function return |
| Print | 5 | Output (script mode only) |
| Array | 3 | Array creation |
| Index | 4 | Array/string indexing |

## Example: Gas Calculation

```rust
contract Counter {
    let count

    fn increment() {
        count = count + 1
    }
}
```

Breaking down `count = count + 1`:
- LoadVar (count) → 1 gas
- LoadConst (1) → 1 gas
- Add → 2 gas
- StoreVar (count) → 2 gas
- **Total: 6 gas**

## Gas Limits

### Script Mode
- Default: 1,000,000 gas
- Use case: Development scripts, one-off execution
- Gas is tracked but not enforced (optional)

### Contract Mode
- Default: 100,000 gas per call
- Use case: Smart contracts
- Gas is **mandatory** and enforced
- Out of gas = revert

### Transaction Mode
- Default: 10,000,000 gas
- Use case: Full transaction execution
- Includes contract setup and execution

## Safety Features

### Infinite Loop Protection
```rust
while true {
    // This will safely stop with "Out of gas" error
    // No infinite loops possible
}
```

### Deterministic Execution
- No external I/O (except Print in script mode)
- No random number generation
- Same input → Same output → Same gas cost

### Attack Prevention
- DOS attacks limited by gas
- Resource exhaustion prevented
- Fair execution guaranteed

## Gas Tracking in Code

```rust
// Create VM with custom gas limit
let mut vm = VM::new()
    .with_gas(100_000, 1); // 100k gas at 1 wei/gas

// Execute
vm.run(instructions)?;

// Check results
println!("Gas used: {}", vm.gas_used());
println!("Gas remaining: {}", vm.gas_remaining());
```

## Integration with Blockchains

The gas model maps directly to blockchain concepts:

```rust
GasContext {
    gas_limit: 21_000,    // Max gas allowed
    gas_price: 20_gwei,   // Cost per gas
    gas_used: 15_234,     // Consumed so far
}

// Total cost = gas_used * gas_price
// Cost = 15,234 * 20 gwei = 304,680 gwei
```

## Future Governance

The gas cost table can be updated via:
1. **Community proposals** - Suggest new costs
2. **Benchmarking** - Measure real execution costs
3. **Governance voting** - ASTRIXA token holders vote
4. **Hard fork** - Deploy updated costs

This ensures the language evolves with blockchain needs.

## Why This Matters

### Security
- Prevents infinite loops
- Limits resource usage
- Protects network

### Fairness
- Every user pays for computation
- No free execution
- Transparent costs

### Compatibility
- Directly mapable to real blockchains
- Ready for Ethereum, Polygon, Solana
- Future-proof design

### Determinism
- Same code = Same gas cost
- No surprises
- Predictable behavior

## Testing

```rust
fn test_simple_contract() {
    let instructions = compile_contract();
    let mut vm = VM::new().with_gas(50_000, 1);
    
    let result = vm.run(instructions)?;
    
    assert!(vm.gas_used() < 50_000);
    assert_eq!(result, expected_value);
}
```

---

**ASTRIXA's gas model is not a limitation—it's a feature.**
It's what makes the language production-ready for blockchain.
