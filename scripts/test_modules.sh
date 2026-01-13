#!/bin/bash
# STEP 49: Build and test module system

echo "🔨 Building ASTRIXA compiler with module support..."
cd compiler
cargo build --release --bin compile_modules

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "📦 Testing module system..."
    echo ""
    
    # Test the module compilation
    ../target/release/compile_modules ../test_step49_main.ax ../test_step49_main.wat
    
    if [ $? -eq 0 ]; then
        echo ""
        echo "✅ Module test successful!"
        echo ""
        echo "📄 Generated WASM:"
        head -30 ../test_step49_main.wat
    else
        echo "❌ Module test failed"
        exit 1
    fi
else
    echo "❌ Build failed"
    exit 1
fi
