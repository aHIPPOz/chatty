#!/bin/bash
set -e

cd /workspaces/chatty

echo "🐧 Chatty Shell Desktop - WASM Build"
echo "======================================"
echo ""

# Try to find and wait for any existing cargo processes
for i in {1..10}; do
    if [ -f "target/.cargo-lock" ]; then
        echo "⏳ Waiting for existing lock ($i/10)..."
        sleep 2
    fi
done

echo "🔨 Building WASM..."
wasm-pack build --release --target web

echo ""
echo "✅ Build complete!"
echo ""
echo "📦 Output:"
ls -lh pkg/*.wasm 2>/dev/null || echo "WASM file not found"
echo ""
echo "🚀 To demo:"
echo "   python3 -m http.server 8000"
echo "   Open http://localhost:8000"
