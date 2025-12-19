# Capability: RPM File Analysis

## Overview
Provides analysis of RPM (Red Hat Package Manager) packages. Extracts package metadata, version information, and architecture details from the RPM header.

## ADDED Requirements

### Requirement: RPM File Detection
The system SHALL detect and identify RPM file format from binary data.

#### Scenario: Valid RPM file
- **WHEN** binary data starts with `\xed\xab\xee\xdb` (RPM Lead magic)
- **THEN** return file type as "RPM"

#### Scenario: Invalid RPM file
- **WHEN** binary data lacks the RPM Lead magic
- **THEN** return error indicating unsupported or invalid format

### Requirement: RPM Metadata Extraction
The system SHALL extract metadata from the RPM Header segment.

#### Scenario: Standard RPM package
- **WHEN** RPM file contains a valid Header structure
- **THEN** extract Name, Version, Release, Architecture, Vendor, and Summary

#### Scenario: Corrupt RPM header
- **WHEN** RPM Header is malformed or inaccessible
- **THEN** return error indicating metadata extraction failure

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
  // Compatibility Aliases
  ProductName?: string,
  ProductVersion?: string,
  CompanyName?: string
}
```

## API

### Function: `analyze_rpm_file(data: Uint8Array): string` (Integrated into `analyze_file`)
Analyzes RPM file and returns JSON string with results.
