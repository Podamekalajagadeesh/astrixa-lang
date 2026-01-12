# STEP 38: BASIC OPTIMIZATIONS - MAKE ASTRIXA FAST ⚡

## 🎯 Goal
Implement foundational optimizations that:
- ✅ Improve performance
- ✅ Reduce unnecessary instructions
- ✅ Prepare for WASM / native / contract targets

A language that is correct but slow ❌
A language that is correct and optimized ✅

---

## 🧠 Two Powerful Optimizations

### 1️⃣ Constant Folding

**What it means:**
Move constant computation from runtime to compile time.

**Example:**
```ax
let x = 2 + 3
```

**Before optimization:**
```
LoadConstInt(2)
LoadConstInt(3)
Add
StoreVar("x")
```

**After optimization:**
```
LoadConstInt(5)
StoreVar("x")
```

**Result:** ⚡ 3 instructions → 2 instructions (33% reduction)

**How it works:**
1. Track a stack of constant values during IR traversal
2. When we see `LoadConstInt`, push it to the stack
3. When we see `Add`, if both operands are constants, fold them
4. If we can't fold (e.g., non-constant operands), emit instructions as-is

**Supported operations:**
- Arithmetic: `Add`, `Sub`, `Mul`, `Div`, `Mod`
- Comparison: `Eq`, `Ne`, `Lt`, `Le`, `Gt`, `Ge`

---

### 2️⃣ Dead Code Elimination (Basic)

**What it means:**
Remove unreachable instructions.

**Example:**
```ax
fn test {
    return
    print("never runs")
}
```

**Before optimization:**
```
Return
LoadConstString("never runs")
Call("print", 1)
```

**After optimization:**
```
Return
```

**Result:** ⚡ 33% code removal

**Simple rule:**
Once `Return` or `Jump` is seen → stop adding instructions.

---

## 🏗️ Implementation

### File 1: `compiler/src/optimize.rs` (NEW)

Located at [compiler/src/optimize.rs](compiler/src/optimize.rs)

**Key functions:**

#### `optimize_module(module: IRModule) -> IRModule`
Main entry point that applies all optimization passes to the module.

```rust
pub fn optimize_module(module: IRModule) -> IRModule {
    let mut optimized_module = IRModule::new();
    
    for function in module.functions {
        // Apply all optimization passes to each function
        let mut optimized_instructions = function.instructions;
        optimized_instructions = optimize(&optimized_instructions);
        optimized_instructions = remove_dead_code(&optimized_instructions);
        
        let mut optimized_function = function;
        optimized_function.instructions = optimized_instructions;
        optimized_module.add_function(optimized_function);
    }
    
    optimized_module
}
```

#### `optimize(instrs: &[IRInstr]) -> Vec<IRInstr>`
Performs constant folding on arithmetic and comparison operations.

**Algorithm:**
```
1. For each instruction:
   - If LoadConstInt: push to stack (no emission yet)
   - If Arithmetic Op: check if both operands are on stack
     - If yes: fold them and keep result on stack
     - If no: emit pending stack values, emit operation
   - If other: emit stack, emit instruction as-is
2. At end: emit remaining stack values
```

#### `remove_dead_code(instrs: &[IRInstr]) -> Vec<IRInstr>`
Removes code after unconditional jumps or returns.

```rust
pub fn remove_dead_code(instrs: &[IRInstr]) -> Vec<IRInstr> {
    let mut result = Vec::new();
    for instr in instrs {
        result.push(instr.clone());
        match instr {
            IRInstr::Return | IRInstr::Jump(_) => {
                break;  // Stop processing
            }
            _ => {}
        }
    }
    result
}
```

### File 2: Updated `compiler/src/main.rs`

**Changes:**
1. Add module declaration: `mod optimize;`
2. Import: `use optimize::optimize_module;`
3. Update main function:

```rust
// After lowering (AST → IR)
let ir = lower(&ast);
println!("📊 IR Module (before optimization):");
println!("  Functions: {}", ir.functions.len());
for func in &ir.functions {
    println!("  - {} ({} instructions)", func.name, func.instructions.len());
}

// Apply optimizations
let optimized_ir = optimize_module(ir);
println!("🚀 IR Module (after optimization):");
println!("  Functions: {}", optimized_ir.functions.len());
for func in &optimized_ir.functions {
    println!("  - {} ({} instructions)", func.name, func.instructions.len());
}

println!("\n📊 Optimized IR Details:\n{:#?}", optimized_ir);
```

---

## 📊 Compiler Pipeline

```
Source Code
    ↓
Lexer (tokenize)
    ↓
Parser (build AST)
    ↓
Type Checker (verify correctness)
    ↓
Lowering (AST → IR)         ← Step 37
    ↓
OPTIMIZATIONS (IR → IR)     ← Step 38 🆕
    ├─ Constant Folding
    └─ Dead Code Elimination
    ↓
Backend (IR → bytecode/WASM)
    ↓
Output
```

---

## 🧪 Tests

The optimizer includes comprehensive tests in [compiler/src/optimize.rs](compiler/src/optimize.rs#L209-L286):

```rust
#[test]
fn test_constant_folding_addition() {
    let instrs = vec![
        IRInstr::LoadConstInt(2),
        IRInstr::LoadConstInt(3),
        IRInstr::Add,
        IRInstr::Return,
    ];
    let optimized = optimize(&instrs);
    assert!(optimized.iter().any(|i| matches!(i, IRInstr::LoadConstInt(5))));
}

#[test]
fn test_dead_code_after_return() {
    let instrs = vec![
        IRInstr::LoadConstInt(42),
        IRInstr::Return,
        IRInstr::LoadConstInt(99),  // Dead code
        IRInstr::Add,
    ];
    let cleaned = remove_dead_code(&instrs);
    assert_eq!(cleaned.len(), 2);
}
```

**Run tests:**
```bash
cd /workspaces/astrixa-lang/compiler
cargo test optimize
```

---

## 🚀 What STEP 38 Achieves

✅ **Compile-time computation**
   - Constants folded before runtime
   - Reduces execution time

✅ **Faster runtime code**
   - Fewer instructions to execute
   - Better CPU cache locality

✅ **Smaller output**
   - WASM modules are smaller
   - Contracts use less storage

✅ **Professional compiler structure**
   - Clear separation: AST (correctness) ↔ IR (performance)
   - Extensible optimization framework
   - Foundation for advanced optimizations

✅ **Never mix concerns**
   - AST = semantic correctness only
   - IR = performance optimization only
   - This principle makes compilers maintainable

---

## 💡 Advanced Optimizations (Future)

The foundation is now set for:

1. **Copy Propagation**
   - Eliminate redundant variable assignments

2. **Common Subexpression Elimination (CSE)**
   - Detect `let x = a + b; let y = a + b;` → reuse result

3. **Loop Invariant Motion**
   - Move constant computations out of loops

4. **Strength Reduction**
   - Replace `x * 2` with `x << 1` (shift is faster)

5. **Tail Call Optimization**
   - Convert tail recursion to loops

6. **Inlining**
   - Replace function calls with function body

---

## 📋 Key Principles

### 🎯 RULE: Optimizations Happen on IR, Never on AST

**Why?**
- AST is for semantics (correctness)
- IR is for implementation (performance)
- Mixing them creates maintenance nightmares

### 🎯 RULE: Each Optimization is Independent

**Why?**
- Easy to add new optimizations
- Easy to debug problems
- Easy to benchmark individual optimizations

### 🎯 RULE: Optimizations are Composable

```rust
// Apply multiple passes in sequence
let ir = lower(&ast);
let opt1 = optimize(&ir);
let opt2 = remove_dead_code(&opt1);
// Can add more: opt3 = inline(&opt2), etc.
```

---

## ✨ Example: Full Pipeline

**Input:**
```ax
fn calculate {
    let x = 2 + 3 + 4
    return x
}
```

**After Parsing (AST):**
```
Function("calculate", [
    Let("x", BinOp(
        BinOp(2, Add, 3),
        Add,
        4
    )),
    Return(Var("x"))
])
```

**After Lowering (IR - before optimization):**
```
LoadConstInt(2)
LoadConstInt(3)
Add
LoadConstInt(4)
Add
StoreVar("x")
LoadVar("x")
Return
```

**After Optimization (IR - after optimization):**
```
LoadConstInt(9)        ← Constant folding: 2 + 3 + 4 = 9
StoreVar("x")
LoadVar("x")
Return
```

**Result:** 8 instructions → 4 instructions (50% reduction)

---

## 📝 Summary

| Component | Purpose | Location |
|-----------|---------|----------|
| Lexer | Tokenize | `compiler/src/lexer.rs` |
| Parser | Build AST | `compiler/src/parser.rs` |
| Type Checker | Verify correctness | `compiler/src/typechecker.rs` |
| Lowering | AST → IR | `compiler/src/lowering.rs` |
| **Optimizer** | **IR optimization** | **`compiler/src/optimize.rs`** ⭐ |
| Interpreter/VM | Execute | `compiler/src/interpreter.rs` |

---

## ✅ Completion Checklist

- [x] Create `compiler/src/optimize.rs`
- [x] Implement constant folding for arithmetic operations
- [x] Implement constant folding for comparison operations
- [x] Implement dead code elimination
- [x] Integrate optimizer into main pipeline
- [x] Add comprehensive tests
- [x] Document optimization passes
- [x] Verify no regressions

---

## 🔗 References

- Previous: [STEP 37: IR Complete](../STEP_37_IR_COMPLETE.md)
- Next: [STEP 39: Code Generation](../STEP_39_CODEGEN.md)

---

**Status:** ✅ COMPLETE

*The Astrixa compiler now has a professional optimization pipeline. We've moved from "correct" to "correct and fast."*
