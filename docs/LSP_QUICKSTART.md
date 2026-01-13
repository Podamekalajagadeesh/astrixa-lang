# 🚀 ASTRIXA LSP & VS Code Extension - Quick Start

**Get professional IDE support in 10 minutes**

---

## What You're Installing

✅ **Language Server** - Real-time errors, autocomplete, hover docs, go-to-def  
✅ **VS Code Extension** - Syntax highlighting, bracket matching, language support  
✅ **First-class DX** - Feels like Python/TypeScript/Rust

---

## Installation Steps

### 1. Build the LSP Server

```bash
cd /workspaces/astrixa-lang/lsp
cargo build --release
```

✅ Creates: `lsp/target/release/astrixa-lsp`

### 2. Install Extension Dependencies

```bash
cd /workspaces/astrixa-lang/astrixa-vscode
npm install
npm run compile
```

✅ Creates: `astrixa-vscode/out/extension.js`

### 3. Install Extension in VS Code

**Option A: From VS Code**
```bash
# From workspace root
code --install-extension ./astrixa-vscode
```

**Option B: Manual Copy**
```bash
# Copy extension to VS Code extensions folder
cp -r astrixa-vscode ~/.vscode/extensions/astrixa-vscode/
```

### 4. Reload VS Code

Press **Ctrl+Shift+P** → **"Reload Window"**

### 5. Configure LSP Path (Optional)

Open VS Code Settings (Ctrl+,) and add:

```json
{
  "astrixa.lsp.path": "/workspaces/astrixa-lang/lsp/target/release/astrixa-lsp"
}
```

If you skip this, the extension will look for `astrixa-lsp` in your PATH.

---

## Test It Works

### Create test.ax

```astrixa
fn add(a: Int, b: Int) -> Int {
    return a + b
}

let result = add(5, 3)
print(result)
```

Open in VS Code - you should see:
- ✅ Syntax highlighting (keywords in color)
- ✅ Bracket matching
- ✅ Line comments work (`//`)

### Test Autocomplete

Type: `std::`

Press: **Ctrl+Space**

You should see:
```
std::io
std::fs
std::net
std::json
std::async
std::crypto
std::ai
std::web
```

### Test Hover Docs

Hover over `print` → should show:
```
fn print(msg: String)
Prints text to stdout
```

### Test Go-to-Definition

**Ctrl+Click** on `add` in `add(5, 3)` → should jump to `fn add(...)` definition.

### Test Diagnostics

Add a type error:
```astrixa
return a + "string"
```

You should see a **red squiggle** with message: `Cannot add Int and String`

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| Extension not found | Run `code --list-extensions \| grep astrixa` |
| LSP not starting | Check Output panel (View → Output → ASTRIXA) |
| No syntax highlighting | Make sure file extension is `.ax` |
| Autocomplete shows nothing | Wait 1-2 seconds for LSP to initialize |
| "Cannot find astrixa-lsp" | Set path in settings or add to PATH |

### Check Extension Installed

```bash
code --list-extensions | grep astrixa
```

Should show: `astrixa`

### Check LSP Binary Exists

```bash
ls -lh /workspaces/astrixa-lang/lsp/target/release/astrixa-lsp
```

Should show the binary file.

### Check LSP is Running

Open VS Code → **View** → **Output** → Select **"ASTRIXA Language Server"**

You should see:
```
[Info] ASTRIXA LSP Server initializing
[Info] ASTRIXA LSP Server initialized
[Info] Document opened: file:///path/to/test.ax
```

---

## Feature Checklist

Test each feature with `test.ax`:

- [ ] Syntax highlighting works
- [ ] Type `std::` and see completions
- [ ] Type `pri` and see `print()` completion
- [ ] Hover over `print` shows docs
- [ ] Ctrl+Click on function name goes to definition
- [ ] Add type error and see red squiggle
- [ ] Press Ctrl+Shift+O and see document outline

---

## Next Steps

### Write ASTRIXA Code

```astrixa
// HTTP client example
fn fetch_users() {
    let response = http_get("https://jsonplaceholder.typicode.com/users")
    if response.is_ok() {
        let users = parse(response.body)
        for user in users {
            print(user.name)
        }
    }
}

// AI example
let poem = generate("Write a haiku about coding")
print(poem)

// Async example
async fn process() {
    await sleep(1000)
    print("Done!")
}
```

### Extend the LSP

- Add more diagnostics (`diagnostics.rs`)
- Add more completions (`completion.rs`)
- Add inlay hints (type annotations)
- Add formatting support

### Share Your Extension

Package for distribution:
```bash
cd astrixa-vscode
npm install -g @vscode/vsce
vsce package
```

Creates: `astrixa-0.1.0.vsix`

Install with:
```bash
code --install-extension astrixa-0.1.0.vsix
```

---

## Architecture Recap

```
┌─────────────────────────┐
│  VS Code (User)         │
│  - Types code in .ax    │
└───────────┬─────────────┘
            │ Language Server Protocol (JSON-RPC)
            ↓
┌─────────────────────────┐
│  ASTRIXA LSP Server     │
│  - Parses code          │
│  - Checks types         │
│  - Returns completions  │
└───────────┬─────────────┘
            │
            ├─ diagnostics.rs  (errors)
            ├─ completion.rs   (autocomplete)
            ├─ hover.rs        (docs)
            └─ symbols.rs      (definitions)
```

---

## Files Created

```
/workspaces/astrixa-lang/
├── lsp/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs          # LSP server entry point
│   │   ├── diagnostics.rs   # Error detection
│   │   ├── completion.rs    # Autocomplete logic
│   │   ├── hover.rs         # Hover documentation
│   │   └── symbols.rs       # Go-to-definition
│   └── README.md
├── astrixa-vscode/
│   ├── package.json         # VS Code manifest
│   ├── tsconfig.json        # TypeScript config
│   ├── language-configuration.json  # Bracket matching
│   ├── src/
│   │   └── extension.ts     # Extension entry point
│   └── syntaxes/
│       └── astrixa.tmLanguage.json  # Syntax highlighting
├── LSP_COMPLETE.md          # Full documentation
└── LSP_QUICKSTART.md        # This file
```

---

**You now have professional IDE support for ASTRIXA. Code faster, catch errors sooner, ship better software.**
