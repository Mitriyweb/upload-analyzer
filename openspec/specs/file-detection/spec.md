# Capability: File Type Detection

## Purpose
Provides basic file type detection and information extraction for binary files. Serves as the entry point for determining which specialized analyzer to use.
## Requirements
### Requirement: Magic Number Detection
The system SHALL identify file types based on magic number signatures.

#### Scenario: PE file signature
- **WHEN** binary data starts with "MZ" (0x4D5A)
- **THEN** identify as PE file format

#### Scenario: MSI file signature
- **WHEN** binary data starts with CFB signature (0xD0CF11E0A1B11AE1)
- **THEN** identify as MSI/CFB file format

#### Scenario: DMG file signature
- **WHEN** binary data contains DMG magic bytes
- **THEN** identify as DMG file format

#### Scenario: Unknown file type
- **WHEN** binary data does not match known signatures
- **THEN** return "Unknown" file type

### Requirement: File Size Validation
The system SHALL validate file size constraints and return standard fields.

#### Scenario: File size
- **WHEN** analyzing any file
- **THEN** return `Format` and `Size` using primary keys.

### Requirement: Basic File Information
The system SHALL extract basic file metadata.

#### Scenario: File size
- **WHEN** analyzing any file
- **THEN** return file size in bytes

#### Scenario: Format version
- **WHEN** file format includes version information in headers
- **THEN** extract and return format version

### Requirement: Multi-Format Support
The system SHALL support multiple binary file formats.

#### Scenario: Format routing
- **WHEN** file type is detected
- **THEN** route to appropriate specialized analyzer (PE, MSI, or DMG)

#### Scenario: Unsupported format
- **WHEN** file type is recognized but not supported for analysis
- **THEN** return basic info without detailed analysis

### Requirement: Error Handling
The system SHALL provide clear error messages for detection failures.

#### Scenario: Corrupted header
- **WHEN** file header is corrupted or incomplete
- **THEN** return error with specific corruption details

#### Scenario: Truncated file
- **WHEN** file appears truncated based on header information
- **THEN** return error indicating incomplete file

## Data Structures

### FileInfo Type
```typescript
{
  Format: "PE" | "MSI" | "DMG" | "DEB" | "RPM" | "Unknown" | "Invalid binary",
  Size: number,
  FormatVersion?: string
}
```

### AnalysisError Type
```typescript
{
  error: string,
  details?: string,
  Format?: string
}
```

## API

### Function: `get_file_info(data: Uint8Array): string`
Detects file type and returns basic information.

**Parameters:**
- `data`: Uint8Array containing file binary data

**Returns:**
- JSON string containing FileInfo object or AnalysisError object

**Throws:**
- Never throws - all errors returned as JSON error objects

### Function: `analyze_file(data: Uint8Array): string`
Automatically detects file type and performs appropriate analysis.

**Parameters:**
- `data`: Uint8Array containing file binary data

**Returns:**
- JSON string containing format-specific analysis (PEAnalysis, MSIAnalysis, DMGAnalysis) or AnalysisError

**Throws:**
- Never throws - all errors returned as JSON error objects
