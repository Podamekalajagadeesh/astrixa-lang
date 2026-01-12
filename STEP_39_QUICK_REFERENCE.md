# STEP 39: QUICK REFERENCE ⚡

## 🎯 What Is This?

Convert optimized IR → WebAssembly (WAT)

**Why WASM?**
- Runs in browser
- Runs on server (Wasmtime)
- Web + AI friendly
- Safe sandbox
- Future smart-contract compatible

---

## 📦 What Was Built

### New Files
- `compiler/src/codegen/mod.rs` - Module declaration
- `compiler/src/codegen/wasm.rs` - WASM code generator (600+ lines)

### Updated Files
- `compiler/src/main.rs` - Added codegen module

---

## 🧠 IR → WASM Mapping

| IR | WASM |
|----|------|
| LoadConstInt(5) | i32.const 5 |
| Add | i32.add |
| Sub | i32.sub |
| Mul | i32.mul |
| Return | return |

---

## 💻 Key Functions

### `generate_wasm_module(module: &IRModule) -> String`
Main entry point - generates complete WASM module.

### `generate_function(name: &str, instrs: &[IRInstr]) -> String`
Generates single function with exports.

### `generate_wat(name: &str, instrs: &[IRInstr]) -> String`
Helper for testing - minimal WAT output.

---

## 🚀 Usage

```bash
cd compiler
cargo run

# Output: Valid WASM (WAT format)
```

**Expected output:**
```wasm
(module
  (func $greet (result i32)
    i32.const 42
    return
  )
  (export "greet" (func $greet))
)
```

---

## 🧪 Tests

4 comprehensive tests:
- ✅ Simple addition
- ✅ Multiplication
- ✅ Comparison
- ✅ Full module generation

**Run:** `cargo test wasm`

---

## 🔄 Complete Pipeline

```
.ax source
   ↓ Lexer
Parser
   ↓
AST
   ↓ Type Checker
IR (Step 37)
   ↓ Optimizer (Step 38)
Optimized IR
   ↓ WASM Codegen (Step 39)
.wat (Valid WebAssembly!)
   ↓
Browser / Wasmtime / Runtime
```

---

## 🌟 Key Achievement

✅ End-to-end compiler working
✅ Real executable output
✅ Web-ready code generation

---

**Status:** ✅ COMPLETE

*Astrixa now produces real WebAssembly.*
