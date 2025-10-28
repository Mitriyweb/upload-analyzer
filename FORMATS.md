# Supported File Formats

This document lists all file formats currently supported by the Upload Analyzer.

## Overview

| Format | Platform | Status | Module | Analyzer |
|--------|----------|--------|--------|----------|
| **PE** | Windows | ✅ Full Support | `pe.rs` | `PEAnalyzer` |
| **MSI** | Windows | ✅ Full Support | `msi.rs` | `MSIAnalyzer` |
| **DMG** | macOS | ✅ Full Support | `dmg.rs` | `DMGAnalyzer` |
| **ELF** | Linux/Unix | ❌ Not Supported | - | - |
| **Mach-O** | macOS | ❌ Not Supported | - | - |

## Format Details

### PE (Portable Executable)

**Platform:** Windows (x86, x64)

**File Extensions:** `.exe`, `.dll`, `.sys`

**Detection:** Via goblin parser

**Extracted Metadata:**
- Format, Architecture (x86/x64)
- Version information (FileVersion, ProductVersion)
- Company and product details (CompanyName, ProductName)
- Digital signatures (SignedBy)
- File description
- Installer type detection (Inno Setup, NSIS, etc.)
- Embedded MSI detection

**TypeScript Interface:** `PEAnalysis`

**Documentation:** See main README.md

---

### MSI (Windows Installer)

**Platform:** Windows

**File Extensions:** `.msi`

**Detection:** Compound File Binary format signature check (first 8 bytes)

**Extracted Metadata:**
- Format, Architecture
- Product information (ProductName, ProductVersion)
- Manufacturer details
- GUIDs (ProductCode, UpgradeCode)
- Installer framework (WiX, InstallShield, Advanced Installer)
- Language information
- Version aliases (compatible with PE fields)

**TypeScript Interface:** `MSIAnalysis`

**Documentation:** See main README.md

---

### DMG (Apple Disk Image)

**Platform:** macOS

**File Extensions:** `.dmg`

**Detection:** Multiple signature checks (compression formats, UDIF koly signature)

**Extracted Metadata:**
- Format, Architecture
- Image type (UDIF)
- Compression format (bzip2, zlib, gzip, uncompressed)
- UDIF version (from koly block)
- Koly signature presence and offset
- Product name (extracted from readable strings)
- File description

**TypeScript Interface:** `DMGAnalysis`

**Documentation:** See `DMG_SUPPORT.md`

---

## Detection Priority

Files are checked in the following order:

1. **MSI** - Fast signature check (8 bytes)
2. **DMG** - Fast signature check (compression/koly patterns)
3. **PE** - Goblin parser (comprehensive but slower)
4. **Other** - Returns unsupported error

## Adding New Formats

To add support for a new file format:

1. Create a new module in `src/rs/` (e.g., `newformat.rs`)
2. Implement the `FileAnalyzer` trait
3. Add detection logic to `lib.rs`
4. Create TypeScript interface in `src/ts/types/index.d.ts`
5. Add type guard in `src/ts/helpers.ts`
6. Update documentation

See `ARCHITECTURE.md` for detailed implementation guide.

## Format Comparison

### Metadata Completeness

| Feature | PE | MSI | DMG |
|---------|----|----|-----|
| Product Name | ✅ | ✅ | ⚠️ Limited |
| Version | ✅ | ✅ | ❌ |
| Company/Vendor | ✅ | ✅ | ❌ |
| Architecture | ✅ | ⚠️ Package | ⚠️ Image |
| Digital Signature | ✅ | ❌ | ❌ |
| Compression Info | ❌ | ❌ | ✅ |
| GUIDs | ❌ | ✅ | ❌ |

### Performance

| Format | Detection Speed | Parse Speed |
|--------|----------------|-------------|
| MSI | ⚡ Very Fast (8 bytes) | 🚀 Fast |
| DMG | ⚡ Very Fast (pattern) | 🚀 Fast |
| PE | ⚠️ Moderate (goblin) | ⚠️ Moderate |

## TypeScript Type Guards

All formats have corresponding type guard functions:

```typescript
import { isPEAnalysis, isMSIAnalysis, isDMGAnalysis, isAnalysisError } from 'upload-analyzer/helpers';

if (isPEAnalysis(analysis)) {
  // analysis is PEAnalysis
  console.log(analysis.CompanyName);
}

if (isMSIAnalysis(analysis)) {
  // analysis is MSIAnalysis
  console.log(analysis.ProductCode);
}

if (isDMGAnalysis(analysis)) {
  // analysis is DMGAnalysis
  console.log(analysis.Compression);
}

if (isAnalysisError(analysis)) {
  // analysis is AnalysisError
  console.error(analysis.error);
}
```

## Response Structure

All analyzers follow consistent principles:

- **No placeholders:** Missing data = missing fields
- **Type indicator:** `Format` field identifies the file type
- **Architecture field:** Platform-specific architecture info
- **Optional fields:** Only present when data exists
- **Error handling:** Errors as `{"error": "message"}` objects

## Future Formats

Potential future additions:

- **DEB** - Debian packages
- **RPM** - Red Hat packages
- **APK** - Android packages
- **IPA** - iOS applications
- **AppImage** - Linux portable applications
- **MSIX** - Modern Windows packages

## Version History

- **v0.1.0** - Initial release (PE, MSI)
- **v0.1.1** - Added DMG support

---

For implementation details, see:
- `ARCHITECTURE.md` - System architecture and trait design
- `DMG_SUPPORT.md` - DMG-specific implementation details
- `README.md` - Usage examples and API documentation
