#!/bin/bash

# Build script for Astrixa WASM

echo "🚀 Building Astrixa WASM Runtime..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Check if wasm32 target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "📦 Installing wasm32 target..."
    rustup target add wasm32-unknown-unknown
fi

# Navigate to compiler directory
cd "$(dirname "$0")/compiler" || exit

# Build for web
echo "🔨 Building WASM module..."
wasm-pack build --target web --out-dir ../examples/pkg --release

if [ $? -eq 0 ]; then
    echo "✅ WASM build successful!"
    echo "📦 Output: examples/pkg/"
    echo ""
    echo "To test in browser:"
    echo "  cd examples"
    echo "  python3 -m http.server 8000"
    echo "  Open http://localhost:8000/wasm_playground.html"
else
    echo "❌ WASM build failed"
    exit 1
fi
