# Setup Guide

Complete guide to set up and build the Upload Analyzer project.

## Prerequisites

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Add WebAssembly target

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Install wasm-pack

```bash
cargo install wasm-pack
```

### 4. Install Node.js

Required for development server and build scripts.

Download from <https://nodejs.org/> or use a version manager:

```bash
# Using nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install --lts
```

### 5. Install Dependencies

```bash
npm install
```

## Building the Project

### Quick Build (Production)

```bash
./build.sh
```

This creates a production-ready `public/` folder with:

- `index.html` - Main page
- `app.min.js` - Bundled JavaScript (6.4 KB)
- `upload_analyzer_bg.wasm` - WebAssembly binary (228 KB)

### Build Commands

#### Build NPM Package Only

```bash
npm run build
```

Builds WASM package to `pkg/` directory with TypeScript definitions.

#### Build Production Bundle

```bash
npm run bundle
```

Builds WASM + creates minified production bundle in `public/`.

## Testing

### Development Mode

```bash
# Build WASM
npm run build

# Start dev server (unminified JavaScript)
npm run dev
```

Opens at <http://localhost:8888/dev/> with:

- Unminified JavaScript for debugging
- Uses `/pkg/` for WASM
- Fast iteration

### Production Mode

```bash
# Build production bundle
./build.sh

# Start production server (minified JavaScript)
npm run serve
```

Opens at <http://localhost:8888/public/> with:

- Minified JavaScript
- Optimized WASM
- Production-ready code

### Manual Testing

1. Open the dev or production server
2. Upload a binary file (.exe, .dll, .sys, .elf, etc.)
3. View analysis results

## Publishing to NPM

### 1. Update Package Metadata

Edit root `package.json` and update:

- `name` - Your package name (must be unique on NPM)
- `version` - Your version number
- `author` - Your name
- `repository` - Your git repository URL

### 2. Build Package

```bash
npm run build
```

This creates `pkg/` folder with:

- JavaScript bindings
- TypeScript definitions (.d.ts)
- WebAssembly binary
- package.json with metadata

Or build everything with `./build.sh`

### 3. Login to NPM

```bash
npm login
```

### 4. Publish

```bash
cd pkg
npm publish
```

### 5. Test Installation

```bash
npm install upload-analyzer
```

## Project Structure

```
upload-analyzer/
├── src/                    # Rust source code
│   ├── lib.rs             # Main entry point
│   ├── pe.rs              # PE file analysis
│   ├── elf.rs             # ELF file analysis
│   ├── macho.rs           # Mach-O analysis
│   └── msi.rs             # MSI detection
│
├── dev/                    # Development demo
│   ├── index.html         # Dev page
│   └── app.js             # UI JavaScript (unminified)
│
├── public/                 # Production demo (deploy this!)
│   ├── index.html         # Production page
│   ├── app.min.js         # Minified JS (auto-generated)
│   └── upload_analyzer_bg.wasm # WASM binary (auto-generated)
│
├── pkg/                    # NPM package (gitignored)
│   ├── upload_analyzer.js
│   ├── upload_analyzer_bg.js
│   ├── upload_analyzer.d.ts # TypeScript definitions
│   └── upload_analyzer_bg.wasm
│
├── scripts/                # Build scripts
│   └── bundle-single.js   # Production bundler
│
├── Cargo.toml              # Rust dependencies
├── package.json            # Node.js dependencies
├── build.sh                # Main build script
├── README.md               # Documentation
└── SETUP.md                # This file
```

## Dependencies

### Rust Crates
- **goblin** (0.8) - Multi-format binary parser
- **pelite** (0.10) - PE file analysis
- **wasm-bindgen** (0.2) - Rust/WASM/JavaScript interop
- **serde** (1.0) - Serialization framework
- **serde_json** (1.0) - JSON support
- **console_error_panic_hook** (0.1) - Better panic messages in browser

## Troubleshooting

### Build Errors

**Error: `wasm-pack` not found**
```bash
cargo install wasm-pack
```

**Error: `wasm32-unknown-unknown` target not installed**
```bash
rustup target add wasm32-unknown-unknown
```

**Error: linking with `rust-lld` failed**

- Make sure you have the latest Rust version: `rustup update`
- Try cleaning and rebuilding: `cargo clean && ./build.sh`

**Error: `wasm-pack` not in PATH**

- The build scripts automatically add `~/.cargo/bin` to PATH
- If still failing, manually run: `export PATH="$HOME/.cargo/bin:$PATH"`

### Runtime Errors

**WASM module fails to load in browser**

- Make sure you're serving over HTTP (not file://)
- Check browser console for CORS errors
- Use `npm run dev` or `npm run serve`

**Import errors**

- Dev mode: imports from `/pkg/`
- Production: imports from `./` (same folder)

## Development Tips

### Hot Reloading

For development, use a file watcher:

```bash
cargo install cargo-watch
cargo watch -s "npm run build"
```

Then run `npm run dev` in a separate terminal.

### Debugging

Add console logs in Rust:
```rust
use web_sys::console;
console::log_1(&"Debug message".into());
```

### Optimization

The release profile in `Cargo.toml` is already optimized for size:
- `opt-level = "z"` - Optimize for size
- `lto = true` - Link-time optimization
- `codegen-units = 1` - Better optimization, slower compile

## Next Steps

1. **Customize the analyzer** - Edit `src/lib.rs` to add more features
2. **Add tests** - Create `tests/` directory for integration tests
3. **Improve UI** - Enhance `example.html` with better styling
4. **Add documentation** - Document your API with rustdoc comments
5. **CI/CD** - Set up GitHub Actions for automated builds

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [goblin Documentation](https://docs.rs/goblin/)
- [pelite Documentation](https://docs.rs/pelite/)

## License

MIT - See LICENSE file for details
