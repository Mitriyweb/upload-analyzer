# Capability: DMG File Analysis

## Overview
Provides analysis of Apple Disk Image (DMG) files. Extracts metadata, volume information, and application bundle details from macOS disk images.

## Requirements

### Requirement: DMG File Detection
The system SHALL detect and identify DMG file format from binary data.

#### Scenario: Valid DMG file
- **WHEN** binary data contains valid DMG signature and structure
- **THEN** return file type as "DMG" with volume information

#### Scenario: Invalid DMG file
- **WHEN** binary data does not contain valid DMG structure
- **THEN** return error indicating invalid DMG format

### Requirement: Volume Information Extraction
The system SHALL extract volume metadata from DMG files.

#### Scenario: Volume name
- **WHEN** DMG contains volume name metadata
- **THEN** extract and return volume name

#### Scenario: Volume size
- **WHEN** DMG contains size information
- **THEN** return volume size in bytes

### Requirement: Application Bundle Detection
The system SHALL identify macOS application bundles within DMG.

#### Scenario: .app bundle present
- **WHEN** DMG contains .app directory structure
- **THEN** detect and extract application name

#### Scenario: Multiple applications
- **WHEN** DMG contains multiple .app bundles
- **THEN** return list of all applications

### Requirement: Info.plist Parsing
The system SHALL parse Info.plist files from application bundles.

#### Scenario: Standard Info.plist
- **WHEN** .app bundle contains Info.plist
- **THEN** extract CFBundleName, CFBundleIdentifier, CFBundleVersion, CFBundleShortVersionString

#### Scenario: Missing Info.plist
- **WHEN** .app bundle lacks Info.plist
- **THEN** return null for bundle information

### Requirement: Architecture Detection
The system SHALL identify target CPU architectures from DMG contents.

#### Scenario: Universal binary
- **WHEN** DMG contains universal binary (multiple architectures)
- **THEN** return list of supported architectures (x86_64, ARM64)

#### Scenario: Single architecture
- **WHEN** DMG contains single-architecture binary
- **THEN** return specific architecture

#### Scenario: Apple Silicon native
- **WHEN** DMG contains ARM64-only binary
- **THEN** return architecture as "ARM64"

### Requirement: Minimum macOS Version
The system SHALL extract minimum required macOS version.

#### Scenario: LSMinimumSystemVersion present
- **WHEN** Info.plist contains LSMinimumSystemVersion
- **THEN** return minimum macOS version requirement

#### Scenario: Version not specified
- **WHEN** Info.plist lacks version requirement
- **THEN** return null or default value

### Requirement: Code Signature Detection
The system SHALL detect code signing information.

#### Scenario: Signed application
- **WHEN** .app bundle contains valid code signature
- **THEN** extract signing identity and team ID

#### Scenario: Unsigned application
- **WHEN** .app bundle is not signed
- **THEN** return null or "Not signed" for signature fields

### Requirement: DMG Format Detection
The system SHALL identify DMG compression and encoding formats.

#### Scenario: Compressed DMG
- **WHEN** DMG uses zlib or bzip2 compression
- **THEN** identify compression method

#### Scenario: Encrypted DMG
- **WHEN** DMG is password-protected
- **THEN** return error indicating encryption

### Requirement: Resource Fork Analysis
The system SHALL parse macOS resource forks if present.

#### Scenario: Resource fork present
- **WHEN** DMG contains resource fork data
- **THEN** extract resource information

#### Scenario: No resource fork
- **WHEN** DMG uses modern format without resource forks
- **THEN** skip resource fork analysis

### Requirement: Error Handling
The system SHALL provide clear error messages for DMG analysis failures.

#### Scenario: Corrupted DMG structure
- **WHEN** DMG structure is corrupted or incomplete
- **THEN** return error object with descriptive message

#### Scenario: Unsupported DMG format
- **WHEN** DMG uses unsupported compression or encoding
- **THEN** return error with specific format information

## Data Structures

### DMGAnalysis Type
```typescript
{
  file_type: "DMG",
  volumeName?: string,
  volumeSize?: number,
  architecture?: "x86_64" | "ARM64" | "Universal",
  architectures?: string[],
  applications?: Array<{
    name: string,
    bundleIdentifier?: string,
    version?: string,
    shortVersion?: string,
    minOSVersion?: string,
    executableName?: string
  }>,
  signedBy?: string,
  teamId?: string,
  compressionFormat?: string,
  encrypted?: boolean,
  createdAt?: string
}
```

## API

### Function: `analyze_dmg_file(data: Uint8Array): string`
Analyzes DMG file and returns JSON string with analysis results.

**Parameters:**
- `data`: Uint8Array containing DMG file binary data

**Returns:**
- JSON string containing DMGAnalysis object or error object

**Throws:**
- Never throws - all errors returned as JSON error objects
