# Upload Analyzer

A WebAssembly-powered binary file analyzer built with Rust.

## Features

- **Multi-Format Analysis**: PE (Windows), MSI (Windows), DMG (macOS), DEB (Linux), RPM (Linux)
- **PE Metadata Extraction**: Version info, company, product details, timestamps
- **32-bit & 64-bit Support**: Handles both x86 and x64 PE files
- **WebAssembly**: Runs directly in the browser with native Rust performance
- **Production Ready**: Minified JS, optimized WASM builds
- **TypeScript Support**: Full type definitions with IntelliSense

## Tech Stack

**Backend:**

- **Rust** - Core analysis engine
- **goblin** - Multi-format binary parsing (PE)
- **pelite** - Advanced PE file analysis
- **cfb** - Compound File Binary format parsing (MSI)
- **wasm-bindgen** - Rust/JavaScript interop

**Frontend:**

- **Vanilla JavaScript** (ES6 modules)
- **Terser** - JavaScript minification

## Prerequisites

1. **Rust** - Install from [https://rustup.rs/](https://rustup.rs/)
2. **wasm-pack** - Install via:

   ```bash
   cargo install wasm-pack
   ```

3. **Node.js** - For development server and minification

## Development

### Development Mode (with unminified JS)

```bash
# Build WASM only
npm run build

# Start dev server (uses dev/app.js - unminified)
npm run dev
```

Opens at `http://localhost:8888/dev/` with unminified JavaScript for debugging.

### Production Mode (with minified JS)

```bash
# Full build (WASM + minification)
./build.sh

# Start production server (uses public/app.min.js - minified)
npm run serve
```

Opens at `http://localhost:8888/public/` with optimized code.

## Project Structure

```text
upload-analyzer/
├── src/
│   ├── rs/                # Rust source code
│   │   ├── lib.rs         # Main Rust entry point
│   │   ├── pe.rs          # PE file analysis module
│   │   ├── msi.rs         # MSI file analysis module
│   │   ├── dmg.rs         # DMG file analysis module
│   │   ├── deb.rs         # DEB file analysis module
│   │   └── rpm.rs         # RPM file analysis module
│   │
│   └── ts/                # TypeScript source code
│       ├── helpers.ts     # Type guards and parsers (source)
│       ├── types/         # TypeScript definitions
│       │   ├── index.d.ts # Main type definitions
│       │   └── README.md  # Types documentation
│       └── examples/      # Usage examples (not published)
│           └── typescript-usage.ts
│
├── dist/                  # Compiled TypeScript (auto-generated)
│   ├── helpers.js
│   └── helpers.d.ts
│
├── dev/                   # Development demo
│   ├── index.html         # Dev page
│   └── app.js             # UI JavaScript (unminified)
│
├── public/                # Production demo
│   ├── index.html         # Production page
│   ├── app.min.js         # Bundled JS + WASM loader (auto-generated)
│   └── upload_analyzer_bg.wasm # WASM binary (auto-generated)
│
├── pkg/                   # NPM package (auto-generated)
│   ├── upload_analyzer.js
│   ├── upload_analyzer_bg.js
│   ├── upload_analyzer.d.ts # TypeScript definitions
│   └── upload_analyzer_bg.wasm
│
├── scripts/               # Build scripts
│   └── bundle-single.js   # Production bundler
│
├── .well-known/          # Browser config
├── Cargo.toml            # Rust dependencies
├── package.json          # Node.js dependencies
├── tsconfig.json         # TypeScript configuration
├── build.sh              # Build script
└── README.md
```

**Key Directories:**

- **`src/rs/`** - Rust source code
- **`src/ts/`** - TypeScript source code, type definitions, and examples
- **`dist/`** - Compiled TypeScript output
- **`dev/`** - Development demo with unminified JavaScript
- **`public/`** - Production-ready demo (3 files: HTML, JS, WASM)
- **`pkg/`** - NPM package with TypeScript definitions (auto-generated)
- **`scripts/`** - Build and bundling scripts

## NPM Package Publishing

To publish this library to npm:

```bash
# Build everything (WASM + production demo)
./build.sh

# Or just build the NPM package
npm run build

# Publish to npm
cd pkg
npm publish
```

**The `pkg/` folder contains:**

- `upload_analyzer.js` - Main entry point
- `upload_analyzer_bg.js` - WASM glue code
- `upload_analyzer.d.ts` - TypeScript definitions
- `upload_analyzer_bg.wasm` - WebAssembly binary (~223 KB)
- `package.json` - Package metadata

**Package Features:**

- ✅ Full TypeScript support
- ✅ Works with bundlers (webpack, rollup, vite)
- ✅ Tree-shakeable ES modules
- ✅ Optimized for production

## Deployment

The `public/` folder is production-ready and contains only 3 files (~235 KB total):

```bash
./build.sh    # Build everything
```

Then deploy the entire `public/` folder to any static host:

- Netlify
- Vercel
- GitHub Pages
- AWS S3
- Any CDN or web server

**What's in public/:**

- `index.html` - Main page
- `app.min.js` - Bundled JavaScript with WASM loader (6.4 KB)
- `upload_analyzer_bg.wasm` - WebAssembly binary (228 KB)

## Usage

### TypeScript (Recommended)

Full type safety with IntelliSense support:

```typescript
import init, { analyze_pe_file, get_file_info } from 'upload-analyzer';
import type { FileAnalysis, PEAnalysis } from 'upload-analyzer/types';
import { isPEAnalysis, isAnalysisError } from 'upload-analyzer/helpers';

// Initialize WASM
await init();

// Analyze file
const file = await fetch('example.exe').then(r => r.arrayBuffer());
const data = new Uint8Array(file);

const result = analyze_pe_file(data);
const analysis = JSON.parse(result) as FileAnalysis;

if (isAnalysisError(analysis)) {
  console.error('Error:', analysis.error);
} else if (isPEAnalysis(analysis)) {
  // TypeScript knows all available fields
  console.log('Company:', analysis.CompanyName);      // ✓ Autocomplete
  console.log('Product:', analysis.ProductName);      // ✓ Type-safe
  console.log('Version:', analysis.FileVersionNumber); // ✓ IntelliSense
  console.log('Signed by:', analysis.SignedBy);

  if (analysis.InstallerType) {
    console.log('Installer:', analysis.InstallerType);
  }
}
```

**See [`types/README.md`](types/README.md) for complete TypeScript documentation.**

### In Browser (with bundler)

```javascript
import * as analyzer from 'upload-analyzer';

// Initialize
analyzer.init();

// Read file
const fileInput = document.getElementById('fileInput');
fileInput.addEventListener('change', async (e) => {
  const file = e.target.files[0];
  const buffer = await file.arrayBuffer();
  const data = new Uint8Array(buffer);

  try {
    // Get basic file info
    const info = analyzer.get_file_info(data);
    console.log('File info:', JSON.parse(info));

    // Analyze PE file
    const analysis = analyzer.analyze_pe_file(data);
    console.log('Analysis:', JSON.parse(analysis));
  } catch (error) {
    console.error('Analysis failed:', error);
  }
});
```

### In Node.js

```javascript
const fs = require('fs');
const analyzer = require('upload-analyzer');

// Read file
const data = fs.readFileSync('example.exe');
const buffer = new Uint8Array(data);

// Get file info
const info = analyzer.get_file_info(buffer);
console.log('File info:', JSON.parse(info));

// Analyze PE file
try {
  const analysis = analyzer.analyze_pe_file(buffer);
  const result = JSON.parse(analysis);

  console.log('File Type:', result.file_type);
  console.log('Architecture:', result.architecture);
  console.log('Sections:', result.sections);
  console.log('Imports:', result.imports);
  console.log('Exports:', result.exports);
} catch (error) {
  console.error('Analysis failed:', error);
}
```

## API

### `init()`

Initialize the WASM module. Call this once before using other functions.

### `get_file_info(data: Uint8Array): string`

Get basic information about a binary file.

**Parameters:**
- `data`: Uint8Array containing the file data

**Returns:** JSON string with file information

### `analyze_pe_file(data: Uint8Array): string`

Perform detailed analysis of a PE file.

**Parameters:**
- `data`: Uint8Array containing the PE file data

**Returns:** JSON string with analysis results including:
- `file_type`: Type of file (PE, ELF, etc.)
- `architecture`: CPU architecture (x86, x86_64)
- `sections`: Array of section information
- `imports`: List of imported functions
- `exports`: List of exported functions
- `is_64bit`: Boolean indicating 64-bit PE
- `entry_point`: Entry point address

## Example Output

```json
{
  "file_type": "PE",
  "architecture": "x86_64",
  "is_64bit": true,
  "entry_point": 4096,
  "sections": [
    {
      "name": ".text",
      "virtual_size": 8192,
      "virtual_address": 4096,
      "raw_data_size": 8192
    }
  ],
  "imports": [
    "ExitProcess (KERNEL32.dll)",
    "GetLastError (KERNEL32.dll)"
  ],
  "exports": [
    "MainFunction"
  ]
}
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
