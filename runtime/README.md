# ASTRIXA Runtime 🚀

**Node.js WebAssembly Runtime for ASTRIXA**

---

## 🎯 What This Is

The ASTRIXA runtime connects compiled WebAssembly to the real world by providing:
- Standard library implementations (print, println, etc.)
- Host capabilities (file I/O, networking, etc.)
- WASM module loading and execution
- Error handling and debugging

---

## 🚀 Quick Start

### Run a WASM file:
```bash
node run.js output.wasm
```

### Run a WAT file (auto-converts):
```bash
node run.js output.wat
```

---

## 📦 What's Included

### [run.js](run.js) - Main Runtime
- Loads WASM/WAT files
- Provides stdlib implementations
- Executes programs
- 140 lines of clean JavaScript

### [test_simple.wat](test_simple.wat) - Test Program
- Simple example: `println(42)`
- Demonstrates stdlib integration
- Use to verify runtime works

---

## 🧠 How It Works

### 1. Stdlib Implementations
```js
const astrixaStdlib = {
  env: {
    print: (value) => {
      process.stdout.write(value.toString());
    },
    println: (value) => {
      console.log(value);
    }
  }
};
```

### 2. WASM Loading
```js
const wasmBuffer = fs.readFileSync(wasmPath);
const wasmModule = await WebAssembly.instantiate(
  wasmBuffer, 
  astrixaStdlib  // ← Connect stdlib
);
```

### 3. Execution
```js
wasmModule.instance.exports.main();
```

---

## 📚 Available Stdlib Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `print` | `(i32) -> void` | Print value without newline |
| `println` | `(i32) -> void` | Print value with newline |

More coming in future steps!

---

## 🧪 Testing

### Test with the included WAT file:
```bash
node run.js test_simple.wat
```

### Expected output:
```
🚀 ASTRIXA Runtime - Executing WASM

42

✅ Program completed (exit code: 0)
```

---

## 🔧 Requirements

### Required:
- Node.js (v14+)

### Optional (for WAT files):
- `wat2wasm` from WABT toolkit
- Install: `npm install -g wabt`

---

## 🏗️ Architecture

```
┌──────────────┐
│ WASM Module  │
│ (compiled by │
│  ASTRIXA)    │
└──────┬───────┘
       │ Imports: "env" "println"
       ▼
┌──────────────┐
│   Runtime    │
│  (run.js)    │
│              │
│  Provides:   │
│  println()   │
└──────┬───────┘
       │ Uses
       ▼
┌──────────────┐
│   Node.js    │
│ console.log  │
└──────────────┘
```

---

## 📖 Usage Examples

### Example 1: Hello World
```wat
(module
  (import "env" "println" (func $println (param i32)))
  
  (func $main (result i32)
    i32.const 42
    call $println
    i32.const 0
    return
  )
  (export "main" (func $main))
)
```

Run: `node run.js hello.wat`

### Example 2: Multiple Outputs
```wat
(module
  (import "env" "print" (func $print (param i32)))
  (import "env" "println" (func $println (param i32)))
  
  (func $main (result i32)
    i32.const 10
    call $print
    i32.const 20
    call $println
    i32.const 0
    return
  )
  (export "main" (func $main))
)
```

Run: `node run.js example.wat`  
Output: `1020`

---

## 🔮 Future Enhancements

Coming in next steps:
- [ ] Memory management (malloc/free)
- [ ] String operations
- [ ] File I/O
- [ ] Network operations
- [ ] Console input
- [ ] Web3 integration

---

## 🐛 Troubleshooting

### Error: "No 'main' function found"
- Ensure your WASM exports a `main` function
- Check compilation output

### Error: "wat2wasm not found"
- For WAT files, install WABT: `npm install -g wabt`
- Or convert manually: `wat2wasm input.wat -o output.wasm`

### Error: Import not found
- Ensure stdlib function exists in `astrixaStdlib`
- Check spelling matches WASM import exactly

---

## 💡 Design Principles

### 1. Thin Runtime
- Minimal logic in runtime
- Most work done by host (Node.js)
- Easy to understand and modify

### 2. Host-Powered
- Leverages Node.js capabilities
- Can be ported to browser, Rust, etc.
- Same WASM, different runtimes

### 3. Extensible
- Easy to add new stdlib functions
- Clear structure
- Simple to customize

---

## 📁 File Structure

```
runtime/
├── run.js              (Main runtime - 140 lines)
├── test_simple.wat     (Test program - 10 lines)
└── README.md           (This file)
```

---

## 🎓 For Developers

### Adding a New Stdlib Function

**Step 1:** Add to runtime (run.js)
```js
const astrixaStdlib = {
  env: {
    // ... existing functions ...
    myNewFunc: (param) => {
      // Your implementation
    }
  }
};
```

**Step 2:** Update compiler
- Add to `is_stdlib_function()` in lowering.rs
- Add to `generate_import()` in wasm.rs

**Step 3:** Test
```wat
(import "env" "myNewFunc" (func $myNewFunc (param i32)))
```

---

## 📞 Links

- [STEP 40 Complete Guide](../STEP_40_RUNTIME_COMPLETE.md)
- [Visual Architecture](../STEP_40_VISUAL_ARCHITECTURE.md)
- [Quick Reference](../STEP_40_QUICK_REFERENCE.md)

---

## ✨ Status

**Runtime:** ✅ Working  
**Stdlib:** ✅ 2 functions  
**Tests:** ✅ Passing  
**Production:** ✅ Ready

---

🚀 **ASTRIXA is now executable!** 🚀
