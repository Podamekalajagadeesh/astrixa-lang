# STEP 40 QUICK REFERENCE 📖

**ASTRIXA Runtime + Standard Library Bindings**

---

## 🎯 What Was Built

✅ **Runtime:** Node.js WASM executor  
✅ **Stdlib:** print(), println()  
✅ **ABI:** WASM import system  
✅ **Integration:** Compiler → Runtime pipeline

---

## 📁 Key Files

| File | Purpose | Lines |
|------|---------|-------|
| [runtime/run.js](runtime/run.js) | Node.js WASM runtime | 140 |
| [compiler/src/ir.rs](compiler/src/ir.rs) | Added CallStd | +1 |
| [compiler/src/codegen/wasm.rs](compiler/src/codegen/wasm.rs) | Import generation | +60 |
| [compiler/src/lowering.rs](compiler/src/lowering.rs) | CallStd emission | +60 |

---

## 🚀 Usage

### Write Code:
```ax
fn main {
    println(42)
}
```

### Compile:
```bash
cargo run -- program.ax > output.wat
```

### Run:
```bash
node runtime/run.js output.wat
```

### Output:
```
42
```

---

## 🧠 Key Concepts

### CallStd Instruction
```rust
IRInstr::CallStd("println")
```
- Calls runtime-provided function
- Becomes WASM import
- Host-implemented

### WASM Import
```wat
(import "env" "println" (func $println (param i32)))
```

### Runtime Implementation
```js
astrixaStdlib = {
  env: {
    println: (value) => console.log(value)
  }
}
```

---

## 📊 Architecture

```
ASTRIXA Code
    ↓
Compiler (with CallStd)
    ↓
WASM (with imports)
    ↓
Runtime (provides stdlib)
    ↓
Output
```

---

## ✨ Available Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `print` | `(i32) -> void` | Print without newline |
| `println` | `(i32) -> void` | Print with newline |

---

## 🧪 Tests

**Total:** 7 tests  
**New:** 2 stdlib tests  
**Status:** ✅ All passing

---

## 📈 Impact

### Before:
- Compiler only
- No I/O
- Theoretical

### After:
- **Full pipeline**
- **Real output**
- **RUNNABLE LANGUAGE** 🚀

---

## 🔮 Next Steps

- Memory management (malloc/free)
- File I/O
- More stdlib functions
- Web3 integration

---

**Status:** ✅ COMPLETE  
**Achievement:** 🎉 ASTRIXA IS NOW EXECUTABLE!
