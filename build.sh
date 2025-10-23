#!/bin/bash

set -e

# Add cargo bin to PATH
export PATH="$HOME/.cargo/bin:$PATH"

echo "🦀 Building Rust WebAssembly project..."

if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

echo "📦 Building production bundle..."
npm run bundle

echo "✅ Build complete!"
echo ""
echo "📁 Generated outputs:"
echo "  pkg/                           (NPM package with TypeScript defs)"
echo "  public/                        (Production demo - deploy this!)"
echo "    ├── index.html"
echo "    ├── app.min.js               (~6 KB)"
echo "    └── upload_analyzer_bg.wasm  (~228 KB)"
echo ""
echo "✨ Ready for deployment and npm publishing!"
echo ""
echo "To test:"
echo "  Development: npm run dev     (uses dev/ + pkg/)"
echo "  Production:  npm run serve   (serves public/)"
echo ""
echo "To publish to npm:"
echo "  cd pkg && npm publish"
