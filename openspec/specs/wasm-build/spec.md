# Capability: WebAssembly Build System

## Purpose
Provides build system for compiling Rust code to WebAssembly with optimization, bundling, and deployment support for both NPM package distribution and browser usage.

## Requirements

### Requirement: WASM Compilation
The system SHALL compile Rust code to optimized WebAssembly binaries.

#### Scenario: Development build
- **WHEN** running `npm run build:wasm`
- **THEN** compile WASM with bundler target for NPM package

#### Scenario: Web build
- **WHEN** running `npm run build:web`
- **THEN** compile WASM with web target for browser usage

#### Scenario: Build optimization
- **WHEN** compiling for release
- **THEN** apply size optimization (opt-level = "z", LTO enabled)

### Requirement: Size Optimization
The system SHALL minimize WASM binary size.

#### Scenario: Target size
- **WHEN** building production WASM
- **THEN** output binary SHALL be less than 250 KB

#### Scenario: Code generation units
- **WHEN** compiling with LTO
- **THEN** use single codegen unit for maximum optimization

#### Scenario: Strip debug info
- **WHEN** building release
- **THEN** strip debug symbols and metadata

### Requirement: JavaScript Bundling
The system SHALL bundle JavaScript code with WASM loader.

#### Scenario: Production bundle
- **WHEN** running `npm run bundle`
- **THEN** create single `app.min.js` file with embedded WASM loader

#### Scenario: Minification
- **WHEN** bundling JavaScript
- **THEN** apply Terser minification with optimal settings

#### Scenario: Source maps
- **WHEN** building development version
- **THEN** optionally generate source maps for debugging

### Requirement: Build Targets
The system SHALL support multiple build targets.

#### Scenario: NPM package target
- **WHEN** building for NPM distribution
- **THEN** output to `pkg/` directory with package.json

#### Scenario: Browser target
- **WHEN** building for direct browser usage
- **THEN** output to `public/` directory with HTML demo

#### Scenario: Development target
- **WHEN** building for development
- **THEN** output unminified code to `dev/` directory

### Requirement: Build Scripts
The system SHALL provide automated build scripts.

#### Scenario: Full build
- **WHEN** running `./build.sh`
- **THEN** execute complete build pipeline (WASM + TypeScript + bundling)

#### Scenario: Incremental build
- **WHEN** running `npm run build`
- **THEN** build only changed components

#### Scenario: Clean build
- **WHEN** build script detects stale artifacts
- **THEN** clean previous outputs before building

### Requirement: TypeScript Compilation
The system SHALL compile TypeScript helpers and type definitions.

#### Scenario: Helper compilation
- **WHEN** running `npm run build:types`
- **THEN** compile `src/ts/helpers.ts` to `dist/helpers.js`

#### Scenario: Type definition generation
- **WHEN** building package
- **THEN** generate `.d.ts` files for all TypeScript sources

#### Scenario: Type checking
- **WHEN** compiling TypeScript
- **THEN** enforce strict type checking rules

### Requirement: Package Preparation
The system SHALL prepare NPM package for publishing.

#### Scenario: Package structure
- **WHEN** building NPM package
- **THEN** include WASM binary, JS glue code, type definitions, and package.json

#### Scenario: Package metadata
- **WHEN** preparing package
- **THEN** include version, description, keywords, and license

#### Scenario: File filtering
- **WHEN** packaging
- **THEN** exclude source files, tests, and development artifacts

### Requirement: Development Server
The system SHALL provide local development server.

#### Scenario: Dev server
- **WHEN** running `npm run dev`
- **THEN** start HTTP server on port 8888 serving dev/ directory

#### Scenario: Production server
- **WHEN** running `npm run serve`
- **THEN** start HTTP server serving public/ directory

#### Scenario: Hot reload
- **WHEN** files change during development
- **THEN** disable caching for immediate updates

### Requirement: Build Validation
The system SHALL validate build outputs.

#### Scenario: WASM validation
- **WHEN** WASM build completes
- **THEN** verify binary is valid WebAssembly module

#### Scenario: Size check
- **WHEN** production build completes
- **THEN** warn if WASM binary exceeds size targets

#### Scenario: Type check
- **WHEN** TypeScript compilation completes
- **THEN** verify no type errors

### Requirement: Error Handling
The system SHALL provide clear build error messages.

#### Scenario: Rust compilation error
- **WHEN** Rust code has compilation errors
- **THEN** display error with file location and description

#### Scenario: WASM-pack failure
- **WHEN** wasm-pack build fails
- **THEN** show detailed error message with resolution steps

#### Scenario: Missing dependencies
- **WHEN** required tools are not installed
- **THEN** display clear message about missing prerequisites

## Build Pipeline

### Build Stages
1. **Rust Compilation** - Compile Rust to WASM
2. **TypeScript Compilation** - Compile TS helpers
3. **Bundling** - Bundle JS + WASM loader
4. **Minification** - Minify JavaScript
5. **Package Preparation** - Prepare NPM package
6. **Validation** - Verify outputs

## Build Outputs

### NPM Package (`pkg/`)
- `upload_analyzer.js` - Main entry point
- `upload_analyzer_bg.js` - WASM glue code
- `upload_analyzer.d.ts` - TypeScript definitions
- `upload_analyzer_bg.wasm` - WebAssembly binary
- `package.json` - Package metadata

### Production Demo (`public/`)
- `index.html` - Demo page
- `app.min.js` - Bundled JavaScript (~6 KB)
- `upload_analyzer_bg.wasm` - WASM binary (~228 KB)

### Development Demo (`dev/`)
- `index.html` - Dev page
- `app.js` - Unminified JavaScript

## Build Configuration

### Rust Configuration (`Cargo.toml`)
```toml
[profile.release]
opt-level = "z"        # Optimize for size
lto = true             # Link-time optimization
codegen-units = 1      # Single codegen unit
```

### TypeScript Configuration (`tsconfig.json`)
```json
{
  "compilerOptions": {
    "strict": true,
    "target": "ES2020",
    "module": "ES2020"
  }
}
```

## NPM Scripts

- `npm run build` - Build WASM + TypeScript
- `npm run build:wasm` - Build WASM only (bundler target)
- `npm run build:web` - Build WASM for web
- `npm run build:types` - Compile TypeScript
- `npm run bundle` - Create production bundle
- `npm run dev` - Start development server
- `npm run serve` - Start production server
- `./build.sh` - Full production build
