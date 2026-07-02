#!/bin/bash
# CID WASM Build Script
# Builds the CID WASM module and generates JavaScript bindings

set -e

echo "🔧 Building CID WASM module..."

# Source cargo env
. "$HOME/.cargo/env"

# Navigate to cid-wasm directory
cd "$(dirname "$0")"

# Clean previous builds
echo "📦 Cleaning previous builds..."
cargo clean

# Build for WASM target
echo "🏗️  Compiling for wasm32-unknown-unknown..."
cargo build --target wasm32-unknown-unknown --release

# Check if wasm-bindgen is available
if command -v wasm-bindgen &> /dev/null; then
    echo "📄 Generating JavaScript bindings with wasm-bindgen..."
    wasm-bindgen target/wasm32-unknown-unknown/release/cid_wasm.wasm \
        --out-dir www \
        --target web
    echo "✅ Generated www/cid_wasm.js and www/cid_wasm_bg.wasm"
else
    echo "⚠️  wasm-bindgen not found. Using raw WASM file."
    echo "   To generate proper JS bindings, install wasm-bindgen-cli:"
    echo "   cargo install wasm-bindgen-cli --locked"
    echo ""
    echo "   Or use wasm-pack:"
    echo "   cargo install wasm-pack"
    echo "   wasm-pack build --target web --out-dir www"
fi

# Copy files to www directory
echo "📁 Copying files to www/..."
mkdir -p www
cp target/wasm32-unknown-unknown/release/cid_wasm.wasm www/ 2>/dev/null || true
cp index.html www/ 2>/dev/null || true
cp cid-wasm.js www/ 2>/dev/null || true
cp loader.js www/ 2>/dev/null || true

# Report size
WASM_FILE="target/wasm32-unknown-unknown/release/cid_wasm.wasm"
if [ -f "$WASM_FILE" ]; then
    SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')
    echo ""
    echo "✅ Build complete!"
    echo "📦 WASM binary size: $SIZE"
    echo "🌐 To test: cd www && python3 -m http.server 8080"
    echo "   Then open http://localhost:8080 in your browser"
fi
