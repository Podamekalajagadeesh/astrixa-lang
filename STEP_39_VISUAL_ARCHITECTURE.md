# STEP 39: VISUAL ARCHITECTURE 🏗️

## End-to-End Compiler Pipeline

```
┌─────────────────────────────────────────────────────────┐
│              ASTRIXA COMPILER PIPELINE                  │
│                                                         │
│          .ax source file                               │
│          fn calculate { 2 + 3 }                        │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
        ┌────────────────────┐
        │   LEXER            │
        │  (tokenize)        │
        │ Fn Let Int Add     │
        └────────┬───────────┘
                 │
                 ▼
        ┌────────────────────┐
        │   PARSER           │
        │  (build AST)       │
        │ Function(Add(2,3)) │
        └────────┬───────────┘
                 │
                 ▼
        ┌────────────────────┐
        │  TYPE CHECKER      │
        │  (verify)          │
        │ ✅ Correct        │
        └────────┬───────────┘
                 │
                 ▼
        ┌────────────────────┐
        │  LOWERING          │
        │  (AST → IR)        │
        │ [LoadConstInt(2),  │
        │  LoadConstInt(3),  │
        │  Add, Return]      │
        └────────┬───────────┘
                 │
                 ▼ STEP 37
        ╔════════════════════╗
        ║   IR Module        ║
        ║  (8 instructions)  ║
        ╚════════┬═══════════╝
                 │
                 ▼
        ┌────────────────────┐
        │  OPTIMIZER         │
        │  (constant folding)│
        │  (dead code elim)  │
        │ [LoadConstInt(5),  │
        │  Return]           │
        └────────┬───────────┘
                 │
                 ▼ STEP 38
        ╔════════════════════╗
        ║ Optimized IR       ║
        ║  (2 instructions)  ║
        ╚════════┬═══════════╝
                 │
                 ▼
      ┌──────────────────────────┐
      │  WASM CODEGEN (NEW!)     │
      │  (IR → WAT)              │
      │                          │
      │  Mapping:                │
      │  LoadConstInt(5)         │
      │    → i32.const 5         │
      │  Return                  │
      │    → return              │
      └──────────┬───────────────┘
                 │
                 ▼ STEP 39
        ╔════════════════════════════════╗
        ║  WAT (WebAssembly Text)        ║
        ║                                ║
        ║  (module                       ║
        ║    (func $calculate (result i32)
        ║      i32.const 5               ║
        ║      return                    ║
        ║    )                           ║
        ║    (export "calculate"...)     ║
        ║  )                             ║
        ╚════════┬═══════════════════════╝
                 │
                 ▼
      ┌──────────────────────────┐
      │  wasm-opt (optional)     │
      │  (optimize for size)     │
      └──────────┬───────────────┘
                 │
                 ▼
      ┌──────────────────────────┐
      │  WASM Binary (.wasm)     │
      │  (ready to run)          │
      └──────────┬───────────────┘
                 │
        ┌────────┴─────────────┐
        │                      │
        ▼                      ▼
    Browser             Wasmtime Runtime
   (Web app)           (Server/Edge)
```

---

## IR → WASM Transformation Detail

```
IR INSTRUCTION              WASM INSTRUCTION
═══════════════════════════════════════════════════

LoadConstInt(5)      →      i32.const 5
LoadConstInt(3)      →      i32.const 3
Add                  →      i32.add
Return               →      return

Stack State Changes:
══════════════════
Before:   []
After const 5:   [5]
After const 3:   [5, 3]
After add:   [8]
After return:   Result: 8
```

---

## WASM Generation Process

```
IR Module
  │
  ├─ Function 1: "calculate"
  │  ├─ Instruction: LoadConstInt(5)   → i32.const 5
  │  ├─ Instruction: LoadConstInt(3)   → i32.const 3
  │  ├─ Instruction: Add               → i32.add
  │  └─ Instruction: Return            → return
  │
  ├─ Function 2: "another"
  │  ├─ ...
  │
  └─ Function N: "..."
      └─ ...
        ↓
   Generate (module ...)
   Generate (func ...)
   Generate (export ...)
        ↓
   WAT String Output
```

---

## Code Generation Architecture

```
┌─────────────────────────────────────────────┐
│     compiler/src/codegen/                   │
│                                             │
│  mod.rs (Module declaration)                │
│  ├─ pub mod wasm;                           │
│  └─ (future: pub mod bytecode;)             │
│     (future: pub mod native;)               │
│                                             │
│  wasm.rs (WASM Generator)                   │
│  ├─ generate_wasm_module()                  │
│  │  └─ Entry point for entire module       │
│  ├─ generate_function()                     │
│  │  └─ Single function definition           │
│  ├─ generate_body()                         │
│  │  └─ Convert IR to WASM instructions      │
│  ├─ generate_wat() [helper for tests]       │
│  │  └─ Minimal WAT for testing              │
│  └─ tests (4 comprehensive tests)           │
│     ├─ test_generate_wat_simple_add         │
│     ├─ test_generate_wat_multiplication     │
│     ├─ test_generate_wat_comparison         │
│     └─ test_generate_wasm_module            │
│                                             │
│  (Future backends share same structure)     │
│  bytecode.rs (Bytecode generator)           │
│  native.rs (Native code generator)          │
│  solana.rs (Smart contract generator)       │
│                                             │
└─────────────────────────────────────────────┘
```

---

## IR Operations → WASM Operations Mapping

```
CONSTANTS
═════════
LoadConstInt(n)         → i32.const n
LoadConstFloat(f)       → f32.const f
LoadConstBool(true)     → i32.const 1
LoadConstBool(false)    → i32.const 0
LoadConstString(s)      → ;; string: s (comment)

ARITHMETIC (i32)
════════════════
Add                     → i32.add
Sub                     → i32.sub
Mul                     → i32.mul
Div                     → i32.div_s
Mod                     → i32.rem_s

COMPARISON (i32)
════════════════
Eq                      → i32.eq
Ne                      → i32.ne
Lt                      → i32.lt_s
Le                      → i32.le_s
Gt                      → i32.gt_s
Ge                      → i32.ge_s

LOGICAL
═══════
And                     → i32.and
Or                      → i32.or
Not                     → i32.const 1; i32.xor

VARIABLES
═════════
LoadVar(name)           → local.get $name
StoreVar(name)          → local.set $name

CONTROL FLOW
═════════════
Jump(target)            → br target
JumpIfFalse(target)     → i32.eqz; br_if target
Return                  → return
Call(name, args)        → call $name

STACK OPERATIONS
════════════════
Pop                     → drop
Dup                     → (local copy)
Nop                     → nop
```

---

## Example: Complete Transformation

### Input: Astrixa Code
```ax
fn add_five {
    let x = 5
    return x
}
```

### Step 1: Parse to AST
```
Function {
  name: "add_five",
  body: [
    Let("x", 5),
    Return(Var("x"))
  ]
}
```

### Step 2: Lower to IR
```
LoadConstInt(5)
StoreVar("x")
LoadVar("x")
Return
```

### Step 3: Optimize
```
LoadConstInt(5)  ← folded/constant
StoreVar("x")
LoadVar("x")
Return
```

### Step 4: WASM Codegen
```
IR → WASM
LoadConstInt(5) → i32.const 5
StoreVar("x")   → local.set $x
LoadVar("x")    → local.get $x
Return          → return
```

### Step 5: Final WAT
```wasm
(module
  (func $add_five (result i32)
    i32.const 5
    local.set $x
    local.get $x
    return
  )
  (export "add_five" (func $add_five))
)
```

---

## Data Flow Diagram

```
┌──────────────┐
│  IR Instr    │
│  LoadConst(5)│
└──────┬───────┘
       │
       ▼
┌──────────────────────┐
│ match on IR instr    │
│ (pattern matching)   │
└──────┬───────────────┘
       │
       ▼
┌──────────────────────┐
│ Get WASM equivalent  │
│ "i32.const 5"        │
└──────┬───────────────┘
       │
       ▼
┌──────────────────────┐
│ Format with indent   │
│ "    i32.const 5\n"  │
└──────┬───────────────┘
       │
       ▼
┌──────────────────────┐
│ Add to WAT output    │
│ (accumulate string)  │
└──────┬───────────────┘
       │
       ▼
┌──────────────────────┐
│ Repeat for all IR    │
│ instructions         │
└──────┬───────────────┘
       │
       ▼
┌──────────────────────┐
│ Wrap in module()     │
│ Wrap in func()       │
│ Add exports          │
└──────┬───────────────┘
       │
       ▼
┌──────────────────────┐
│ Return WAT string    │
│ (ready to use)       │
└──────────────────────┘
```

---

## Module Structure

```
(module
  ┌─────────────────────────────────────┐
  │ Function Definitions                │
  │                                     │
  │ (func $func1 (result i32)           │
  │   i32.const 42                      │
  │   return                            │
  │ )                                   │
  │                                     │
  │ (func $func2 (param i32) (result...) │
  │   local.get $0                      │
  │   i32.const 1                       │
  │   i32.add                           │
  │   return                            │
  │ )                                   │
  └─────────────────────────────────────┘
  │
  │  Plus...
  │
  ┌─────────────────────────────────────┐
  │ Export Declarations                 │
  │                                     │
  │ (export "func1" (func $func1))      │
  │ (export "func2" (func $func2))      │
  └─────────────────────────────────────┘
)
```

---

## Technology Stack

```
Astrixa Compiler
│
├─ Frontend (No changes needed)
│  ├─ Lexer (rust/parser)
│  ├─ Parser
│  └─ Type Checker
│
├─ Middle End (Done ✅)
│  ├─ Lowering (AST → IR)
│  ├─ Optimizer (constant folding, DCE)
│  └─ IR Representation
│
└─ Backend (Extensible 🎯)
   ├─ WASM (STEP 39 ✅)
   ├─ Bytecode (STEP 40 🔮)
   ├─ Native (STEP 41 🔮)
   └─ Smart Contract (STEP 42 🔮)
```

---

**Status:** ✅ COMPLETE

*The WASM backend is now generating valid WebAssembly.*
