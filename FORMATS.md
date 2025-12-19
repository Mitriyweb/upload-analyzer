# Supported File Formats

This document lists all file formats currently supported by the Upload Analyzer.

## Overview

| Format | Platform | Status | Module | Analyzer |
|--------|----------|--------|--------|----------|
| **PE** | Windows | ✅ Full Support | `pe.rs` | `PEAnalyzer` |
| **MSI** | Windows | ✅ Full Support | `msi.rs` | `MSIAnalyzer` |
| **DMG** | macOS | ✅ Full Support | `dmg.rs` | `DMGAnalyzer` |
| **DEB** | Linux | ✅ Full Support | `deb.rs` | `DEBAnalyzer` |
| **RPM** | Linux | ✅ Full Support | `rpm.rs` | `RPMAnalyzer` |
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

**Planned Enhancements:**
- Structured parsing of the internal `Property` table (replacing heuristic scanning).
- Extraction of `PackageCode` from OLE Revision Number.
- Inventory counts (File, Component, Feature tables).
- System requirements from `LaunchCondition` table.

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

### DEB (Debian Package)

**Platform:** Linux

**File Extensions:** `.deb`

**Detection:** Archive signature check (`!<arch>\n`) and `debian-binary` member

**Extracted Metadata:**
- Package, Version, Architecture, Maintainer
- Description, Depends, Section, Priority
- Product aliases (compatible with PE fields)

**TypeScript Interface:** `DEBAnalysis`

---

### RPM (Red Hat Package Manager)

**Platform:** Linux

**File Extensions:** `.rpm`

**Detection:** RPM Lead magic bytes (`\xed\xab\xee\xdb`)

**Extracted Metadata:**
- Package, Version, Release, Architecture
- Vendor, Summary, License, GroupName
- Url, SourceRpm
- Product aliases (compatible with PE fields)

**TypeScript Interface:** `RPMAnalysis`

---

## Detection Priority

Files are checked in the following order:

1. **MSI** - Fast signature check (8 bytes)
2. **DMG** - Fast signature check (compression/koly patterns)
3. **DEB** - Archive signature check
4. **RPM** - Lead magic bytes
5. **PE** - Goblin parser (comprehensive but slower)
6. **Other** - Returns unsupported error

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

| Feature | PE | MSI | DMG | DEB | RPM |
|---------|----|----|-----|-----|-----|
| Product Name | ✅ | ✅ | ⚠️ Limited | ✅ | ✅ |
| Version | ✅ | ✅ | ❌ | ✅ | ✅ |
| Company/Vendor | ✅ | ✅ | ❌ | ✅ | ✅ |
| Architecture | ✅ | ⚠️ Package | ⚠️ Image | ✅ | ✅ |
| Digital Signature | ✅ | ❌ | ❌ | ❌ | ❌ |
| Package Code | ❌ | ⚠️ Planned | ❌ | ❌ | ❌ |
| Inventory Counts | ❌ | ⚠️ Planned | ❌ | ❌ | ❌ |
| Requirements | ❌ | ⚠️ Planned | ❌ | ❌ | ❌ |

### Performance

| Format | Detection Speed | Parse Speed |
|--------|----------------|-------------|
| MSI | ⚡ Very Fast (8 bytes) | 🚀 Fast |
| DMG | ⚡ Very Fast (pattern) | 🚀 Fast |
| DEB | ⚡ Very Fast (pattern) | 🚀 Fast |
| RPM | ⚡ Very Fast (pattern) | 🚀 Fast |
| PE | ⚠️ Moderate (goblin) | ⚠️ Moderate |

## TypeScript Type Guards

All formats have corresponding type guard functions:

```typescript
import { isPEAnalysis, isMSIAnalysis, isDMGAnalysis, isDEBAnalysis, isRPMAnalysis, isAnalysisError } from 'upload-analyzer/helpers';

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

## Metadata Standards

To ensure consistency across different file formats, all analyzers SHOULD attempt to populate the following fields if the data is available.

### Core Fields
- **`Format`**: The file format (PE, MSI, DMG, DEB, RPM). **Mandatory.**
- **`Architecture`**: The CPU architecture (x64, x86, arm64, etc.). **Mandatory.**
- **`ProductName`**: The full name of the software product.
- **`ProductVersion`**: The version string of the product.
- **`Manufacturer`**: The name of the organization that produced the software.

### Identity & Publishing
- **`Publisher`**: The entity responsible for publishing the software.
- **`CompanyName`**: The legal company name.
- **`Vendor`**: The software vendor (often used in Linux packages).
- **`ProductCode`**: A unique identifier for the product (e.g., GUID in MSI).
- **`UpgradeCode`**: A unique identifier for the product family.
- **`PackageCode`**: A unique identifier for the specific package.

### Content & Categorization
- **`Title`**: A short, descriptive title for the package.
- **`Comments`**: Detailed comments or description of the software.
- **`Keywords`**: Relevant tags or keywords.
- **`License`**: The software license.
- **`InstallerFramework`**: The tool used to build the installer (e.g., WiX, NSIS).

### 🚫 Rule: No Metadata Aliasing
Analyzers MUST NOT create duplicate mappings for the same data under different keys (aliases). Each piece of metadata should be mapped to its most accurate primary key.

**Bad:**
```rust
meta.insert("Manufacturer".into(), value.clone());
meta.insert("Vendor".into(), value.clone()); // ALIAS - FORBIDDEN
```

**Good:**
```rust
meta.insert("Manufacturer".into(), value); // Use primary key only
```

---

## Response Structure

All analyzers follow consistent principles:

- **No placeholders:** Missing data = missing fields
- **No aliases:** Each value is mapped to its primary key only.
- **Type indicator:** `Format` field identifies the file type
- **Architecture field:** Platform-specific architecture info
- **Optional fields:** Only present when data exists
- **Error handling:** Errors as `{"error": "message"}` objects

## Future Formats

Potential future additions:

- **DEB** - Debian packages (Included)
- **RPM** - Red Hat packages (Included)
- **APK** - Android packages
- **IPA** - iOS applications
- **AppImage** - Linux portable applications
- **MSIX** - Modern Windows packages

## Version History

- **v0.1.0** - Initial release (PE, MSI)
- **v0.1.1** - Added DMG support
