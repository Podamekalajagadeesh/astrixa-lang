# ASTRIXA Language Server

This directory contains the LSP server implementation for ASTRIXA.

## Build

```bash
cargo build --release
```

## Run

```bash
./target/release/astrixa-lsp
```

The server listens on stdin/stdout for LSP protocol messages.
