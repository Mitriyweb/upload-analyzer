# Capability: DEB File Analysis

## Purpose
Provides analysis of Debian (.deb) packages. Extracts package metadata, version information, and architecture details from the control file.
## Requirements
### Requirement: DEB File Detection
The system SHALL detect and identify DEB file format from binary data.

#### Scenario: Valid DEB file
- **WHEN** binary data starts with `!<arch>\n` and contains a `debian-binary` member
- **THEN** return `Format` as "DEB"

### Requirement: DEB Metadata Extraction
The system SHALL extract metadata from the `control` file.

#### Scenario: Standard DEB package
- **WHEN** DEB file contains a valid `control` file
- **THEN** extract Package, Version, Architecture, Maintainer, and Description, and map them to standard metadata fields (`ProductName`, `ProductVersion`, etc.) without aliasing.

### Requirement: Package Architecture Identification
The system SHALL identify the target architecture from the DEB metadata.

#### Scenario: Common architectures
- **WHEN** metadata specifies architecture like `amd64`, `i386`, `arm64`, or `all`
- **THEN** return corresponding architecture string

## Data Structures

### DEBAnalysis Type
```typescript
{
  Format: "DEB",
  architecture: string,
  Package: string,
  Version: string,
  Maintainer?: string,
  Description?: string,
  Depends?: string,
  Section?: string,
  Priority?: string,
  ProductName?: string,
  ProductVersion?: string,
  Manufacturer?: string,
  CompanyName?: string,
  Vendor?: string
}
```

## API

### Function: `analyze_deb_file(data: Uint8Array): string`
Analyzes DEB file and returns JSON string with results.
