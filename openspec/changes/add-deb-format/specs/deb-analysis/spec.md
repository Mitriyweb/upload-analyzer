# Capability: DEB File Analysis

## Overview
Provides analysis of Debian (.deb) packages. Extracts package metadata, version information, and architecture details from the control file.

## ADDED Requirements

### Requirement: DEB File Detection
The system SHALL detect and identify DEB file format from binary data.

#### Scenario: Valid DEB file
- **WHEN** binary data starts with `!<arch>\n` and contains a `debian-binary` member
- **THEN** return file type as "DEB"

#### Scenario: Invalid DEB file
- **WHEN** binary data lacks the DEB signature
- **THEN** return error indicating unsupported or invalid format

### Requirement: DEB Metadata Extraction
The system SHALL extract metadata from the `control` file within the `control.tar.gz` (or similar) member of the DEB archive.

#### Scenario: Standard DEB package
- **WHEN** DEB file contains a valid `control` file
- **THEN** extract Package, Version, Architecture, Maintainer, and Description

#### Scenario: Missing control file
- **WHEN** DEB file lacks a `control` file or it's unreadable
- **THEN** return error indicating metadata extraction failure

### Requirement: Package Architecture Identification
The system SHALL identify the target architecture from the DEB metadata.

#### Scenario: Common architectures
- **WHEN** metadata specifies architecture like `amd64`, `i386`, `arm64`, or `all`
- **THEN** return corresponding architecture string

## Data Structures

### DEBAnalysis Type
```typescript
{
  file_type: "DEB",
  architecture: string,
  Package: string,
  Version: string,
  Maintainer?: string,
  Description?: string,
  Depends?: string,
  Section?: string,
  Priority?: string
}
```

## API

### Function: `analyze_deb_file(data: Uint8Array): string`
Analyzes DEB file and returns JSON string with results.
