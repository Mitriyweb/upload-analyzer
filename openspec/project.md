# Project Context

## Purpose
Upload Analyzer is a WebAssembly-powered binary file analyzer that provides detailed analysis of PE (Windows executables), MSI (Windows Installer), DMG (Apple Disk Image), DEB (Debian Package), and RPM (Red Hat Package Manager) files directly in the browser with native Rust performance.

## Tech Stack

### Backend
- **Rust** - Core analysis engine compiled to WebAssembly
- **goblin** - Multi-format binary parsing (PE)
- **pelite** - Advanced PE file analysis
- **cfb** - Compound File Binary format parsing (MSI)
- **plist** - Apple property list parsing (DMG)
- **ar** - Archive parsing (DEB)
- **tar** - Tar archive parsing (DEB/RPM)
- **flate2** - Compression support (DEB/RPM)
- **wasm-bindgen** - Rust/JavaScript interop

### Frontend
- **Vanilla JavaScript** (ES6 modules)
- **TypeScript** - Type definitions and helpers
- **Terser** - JavaScript minification

### Build Tools
- **wasm-pack** - WebAssembly compilation
- **esbuild** - Fast bundling
- **http-server** - Development server

## Project Conventions

### Code Style

**Rust:**
- Follow Rust 2021 edition standards
- Use `clippy.toml` and `rustfmt.toml` for formatting
- Strict clippy lints enforced (see `Cargo.toml` and `.agent/workflows/rust-guidelines.md`)
- **NO generic types** - WASM optimization requirement
- **NO concurrency primitives** - WASM is single-threaded
- Type complexity threshold: 100 (enforced via clippy)

**TypeScript:**
- ESLint configuration in `eslint.config.mjs`
- Strict type checking enabled
- Export type definitions from `src/ts/types/index.d.ts`

### Architecture Patterns

**WASM Module Structure:**
- All Rust code in `src/rs/`
- Main entry point: `src/rs/lib.rs`
- Format-specific modules: `pe.rs`, `msi.rs`, `dmg.rs`, `deb.rs`, `rpm.rs`
- Public API exposed via `wasm-bindgen`

**Type Safety:**
- Rust types serialized to JSON
- TypeScript definitions mirror Rust structures
- Type guards in `src/ts/helpers.ts` for runtime validation

**Build Outputs:**
- `pkg/` - NPM package with WASM binary
- `public/` - Production demo (3 files: HTML, JS, WASM)
- `dist/` - Compiled TypeScript helpers

### Testing Strategy
- Clippy lints run on all code (`npm run lint:rust`)
- ESLint for TypeScript (`npm run lint:ts`)
- Knip for dead code detection (`npm run lint:knip`)
- cargo-machete for unused dependencies (`npm run lint:rust:deps`)
- Full lint suite: `npm run lint`
- Pre-commit hooks enforce all lints automatically
- Manual testing via dev/production servers
- Type safety validated at compile time

### Git Workflow
- Main branch for stable releases
- Feature branches for new capabilities
- OpenSpec workflow for spec-driven development
- Changes archived after deployment

## Domain Context

**Binary File Analysis:**
- PE files: Windows executables (.exe, .dll, .sys)
- MSI files: Windows Installer packages
- DMG files: Apple Disk Images
- DEB files: Debian Packages
- RPM files: Red Hat Package Manager files

**Key Analysis Features:**
- File type detection
- Architecture identification (x86, x86_64, ARM)
- Section/segment parsing
- Import/export table extraction
- Version resource extraction
- Digital signature verification
- Installer type detection

## Important Constraints

### Technical Constraints
- **WASM Limitations:**
  - Single-threaded execution only
  - No file system access (browser sandbox)
  - Limited memory (browser heap)
  - No native OS APIs

- **Rust for WASM:**
  - Avoid generic types (code bloat)
  - Avoid async/await and concurrency primitives
  - Minimize dependencies
  - Optimize for binary size (`opt-level = "z"`)

### Performance Targets
- WASM binary size: < 250 KB
- Analysis time: < 1 second for typical files
- Memory usage: < 100 MB for large files

### Browser Compatibility
- Modern browsers with WebAssembly support
- ES6 module support required
- No polyfills for legacy browsers

## External Dependencies

### Rust Crates
- `goblin` - Binary parsing
- `pelite` - PE-specific analysis
- `cfb` - MSI/OLE parsing
- `plist` - DMG/plist parsing
- `ar`, `tar`, `flate2` - Linux package parsing
- `wasm-bindgen` - JS interop
- `serde`/`serde_json` - Serialization

### NPM Packages
- `terser` - JS minification
- `esbuild` - Fast bundling
- `http-server` - Development server
- `typescript` - Type checking

### Build Requirements
- Rust toolchain (rustup)
- wasm-pack
- Node.js >= 25.1.0
