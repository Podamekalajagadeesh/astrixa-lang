# Installation

Get ASTRIXA running on your machine in under a minute.

## Quick Install (Recommended)

### macOS / Linux

Open your terminal and run:

```bash
curl -fsSL https://astrixa.org/install | sh
```

This will:
- Download the latest ASTRIXA compiler
- Install it to `/usr/local/bin/astrixa`
- Add it to your PATH
- Verify the installation

### Windows

Download the installer from [astrixa.org/download](https://astrixa.org/download) and run it.

Or use PowerShell:
```powershell
iwr https://astrixa.org/install.ps1 -useb | iex
```

## Manual Installation

### Prerequisites

- **Rust toolchain** (for building from source)
- **Git** (for cloning the repository)

### Build from Source

1. Clone the repository:
```bash
git clone https://github.com/Podamekalajagadeesh/astrixa-lang.git
cd astrixa-lang
```

2. Build the compiler:
```bash
cd compiler
cargo build --release
```

3. Add to PATH:
```bash
# Linux/macOS
export PATH="$PATH:$(pwd)/target/release"

# Or install globally
sudo cp target/release/astrixa /usr/local/bin/
```

4. Verify installation:
```bash
astrixa --version
```

## Verify Installation

Check that ASTRIXA is correctly installed:

```bash
astrixa --version
# Output: astrixa 0.1.0
```

Run a simple program:

```bash
echo 'print("Hello, ASTRIXA!")' > hello.ax
astrixa hello.ax
# Output: Hello, ASTRIXA!
```

## IDE Support

### Visual Studio Code

Install the official ASTRIXA extension:

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Search for "ASTRIXA"
4. Click Install

**Features:**
- Syntax highlighting
- Code completion
- Error diagnostics
- Hover documentation
- Go to definition

### Language Server (LSP)

ASTRIXA includes a Language Server Protocol implementation for any LSP-compatible editor.

Build the LSP server:
```bash
cd lsp
cargo build --release
```

Configure your editor to use `target/release/astrixa-lsp`.

## Package Manager

Initialize a new ASTRIXA project:

```bash
mkdir my-project
cd my-project
astrixa init
```

This creates an `astrixa.toml` file for managing dependencies.

Install packages:
```bash
astrixa install <package-name>
```

See [Package Manager Guide](../PACKAGE_MANAGER.md) for more details.

## WebAssembly Playground

Try ASTRIXA in your browser without installing anything:

👉 **[playground.astrixa.org](https://playground.astrixa.org)**

- Write code instantly
- See syntax highlighting
- Get immediate feedback
- Share code snippets

## Troubleshooting

### Command not found: astrixa

The installation directory is not in your PATH. Add it manually:

```bash
# Linux/macOS (add to ~/.bashrc or ~/.zshrc)
export PATH="$PATH:/usr/local/bin"
```

### Permission denied

The installer needs sudo access to write to `/usr/local/bin`. Run:

```bash
sudo chown -R $(whoami) /usr/local/bin
```

Or install to a user directory:
```bash
mkdir -p ~/.local/bin
cp astrixa ~/.local/bin/
export PATH="$PATH:~/.local/bin"
```

### Rust not installed

If building from source, install Rust first:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Updating ASTRIXA

To update to the latest version:

```bash
curl -fsSL https://astrixa.org/install | sh
```

Or if installed from source:
```bash
cd astrixa-lang
git pull
cd compiler && cargo build --release
```

## Uninstalling

Remove ASTRIXA from your system:

```bash
# If installed with script
sudo rm /usr/local/bin/astrixa

# If installed from source
rm -rf ~/astrixa-lang
```

## Next Steps

✅ ASTRIXA is installed!

Now learn the language:
- [Language Syntax →](language/syntax.md)
- [Standard Library →](stdlib/web.md)
- [Examples →](../examples/)

---

**Need help?** Join our community on [GitHub Discussions](https://github.com/Podamekalajagadeesh/astrixa-lang/discussions)
