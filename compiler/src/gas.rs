// ASTRIXA Gas Model - Deterministic Execution Cost System
// Every instruction has a predictable gas cost
// This ensures security, fairness, and blockchain compatibility

use crate::bytecode::OpCode;

/// Returns the gas cost for executing a specific opcode
/// These costs are inspired by Ethereum but simplified for clarity
/// Can be adjusted via governance in the future
pub fn gas_cost(op: &OpCode) -> u64 {
    match op {
        // Stack & variable operations (cheap)
        OpCode::LoadConst => 1,      // Push constant to stack
        OpCode::LoadVar => 1,        // Load variable from memory
        OpCode::StoreVar => 2,       // Store to memory
        OpCode::Pop => 1,            // Discard stack top
        
        // Arithmetic operations
        OpCode::Add => 2,            // Addition
        OpCode::Sub => 2,            // Subtraction
        OpCode::Mul => 5,            // Multiplication (more expensive)
        OpCode::Div => 8,            // Division (most expensive)
        
        // Comparison operations
        OpCode::Greater => 2,
        OpCode::Less => 2,
        
        // Control flow (cheap)
        OpCode::Jump => 1,           // Unconditional jump
        OpCode::JumpIfFalse => 2,    // Conditional jump (evaluate condition)
        
        // Function calls (expensive)
        OpCode::Call => 10,          // Function call (setup overhead)
        OpCode::Return => 3,         // Function return
        
        // I/O operations (expensive, for security)
        OpCode::Print => 5,          // Output (only in script mode)
        
        // Array operations
        OpCode::Array => 3,          // Array creation
        OpCode::Index => 4,          // Array/string indexing
    }
}

/// Total gas required for a program (rough estimate)
pub fn estimate_gas(instructions_count: usize) -> u64 {
    // Rough estimate: average 2 gas per instruction
    (instructions_count as u64) * 2
}

/// Gas limits for different execution modes
pub struct GasLimits {
    pub script: u64,          // Script mode (relaxed)
    pub contract: u64,        // Contract mode (strict)
    pub transaction: u64,     // Full transaction
}

impl GasLimits {
    pub fn default() -> Self {
        GasLimits {
            script: 1_000_000,      // 1 million gas for scripts
            contract: 100_000,      // 100k gas for contract calls
            transaction: 10_000_000, // 10 million gas for full transactions
        }
    }

    pub fn test() -> Self {
        GasLimits {
            script: 100_000,
            contract: 10_000,
            transaction: 1_000_000,
        }
    }
}

/// Transaction context with gas information
#[derive(Clone)]
pub struct GasContext {
    pub gas_limit: u64,      // Maximum gas allowed
    pub gas_price: u64,      // Cost per unit of gas (wei/gwei)
    pub gas_used: u64,       // Gas consumed so far
}

impl GasContext {
    pub fn new(gas_limit: u64, gas_price: u64) -> Self {
        GasContext {
            gas_limit,
            gas_price,
            gas_used: 0,
        }
    }

    pub fn remaining(&self) -> u64 {
        if self.gas_used > self.gas_limit {
            0
        } else {
            self.gas_limit - self.gas_used
        }
    }

    pub fn is_out_of_gas(&self) -> bool {
        self.gas_used > self.gas_limit
    }

    pub fn total_cost(&self) -> u128 {
        self.gas_used as u128 * self.gas_price as u128
    }
}
