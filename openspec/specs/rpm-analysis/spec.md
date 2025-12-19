# Capability: RPM File Analysis

## Purpose
Provides analysis of RPM (Red Hat Package Manager) packages. Extracts package metadata, version information, and architecture details from the RPM header.
## Requirements
### Requirement: RPM File Detection
The system SHALL detect and identify RPM file format from binary data.

#### Scenario: Valid RPM file
- **WHEN** binary data starts with `\xed\xab\xee\xdb` (RPM Lead magic)
- **THEN** return `Format` as "RPM"

### Requirement: RPM Metadata Extraction
The system SHALL extract metadata from the RPM Header.

#### Scenario: Standard RPM package
- **WHEN** RPM file contains a valid Header structure
- **THEN** extract Name, Version, Release, and Vendor, mapping them to standard metadata fields (`ProductName`, `ProductVersion`, `CompanyName`) without aliasing.

### Requirement: Package Architecture Identification
The system SHALL identify the target architecture from the RPM metadata tags.

#### Scenario: Common architectures
- **WHEN** metadata specifies architecture like `x86_64`, `i386`, `aarch64`, or `noarch`
- **THEN** return corresponding architecture string

## Data Structures

### RPMAnalysis Type
```typescript
{
  Format: "RPM",
  Architecture?: string,
  Package?: string,
  Version?: string,
  Release?: string,
  Vendor?: string,
  Summary?: string,
  License?: string,
  GroupName?: string,
  Url?: string,
  SourceRpm?: string,
  ProductName?: string,
  ProductVersion?: string,
  CompanyName?: string
}
```

## API

### Function: `analyze_rpm_file(data: Uint8Array): string` (Integrated into `analyze_file`)
Analyzes RPM file and returns JSON string with results.
