# Capability: PE File Analysis

## Purpose
Provides comprehensive analysis of Portable Executable (PE) files including Windows executables (.exe), dynamic link libraries (.dll), and system drivers (.sys). Extracts metadata, version information, imports/exports, sections, and digital signatures.
## Requirements
### Requirement: PE File Detection
The system SHALL detect and identify PE file format from binary data.

#### Scenario: Valid PE file
- **WHEN** binary data contains valid PE signature (MZ header + PE magic)
- **THEN** return `Format` as "PE" with architecture information

### Requirement: Architecture Detection
The system SHALL identify the target CPU architecture of PE files.

#### Scenario: 32-bit x86 executable
- **WHEN** PE file has IMAGE_FILE_MACHINE_I386 machine type
- **THEN** return architecture as "x86" and is_64bit as false

#### Scenario: 64-bit x64 executable
- **WHEN** PE file has IMAGE_FILE_MACHINE_AMD64 machine type
- **THEN** return architecture as "x86_64" and is_64bit as true

#### Scenario: ARM architecture
- **WHEN** PE file has ARM machine type
- **THEN** return architecture as "ARM" or "ARM64"

### Requirement: Version Resource Extraction
The system SHALL extract version information from PE file resources.

#### Scenario: Standard version info
- **WHEN** PE file contains VS_VERSION_INFO resource
- **THEN** extract `CompanyName`, `ProductName`, `FileVersion`, `ProductVersion`, and `FileDescription` using primary keys.

### Requirement: Section Analysis
The system SHALL parse and extract PE section information.

#### Scenario: Multiple sections
- **WHEN** PE file contains multiple sections (.text, .data, .rdata)
- **THEN** return array of sections with name, virtual_size, virtual_address, raw_data_size

#### Scenario: Section characteristics
- **WHEN** analyzing section headers
- **THEN** include section characteristics flags (executable, readable, writable)

### Requirement: Import Table Parsing
The system SHALL extract imported functions and DLLs.

#### Scenario: Standard imports
- **WHEN** PE file imports functions from system DLLs
- **THEN** return list of imports in format "FunctionName (DllName)"

#### Scenario: No imports
- **WHEN** PE file has no import table
- **THEN** return empty imports array

### Requirement: Export Table Parsing
The system SHALL extract exported functions from DLLs.

#### Scenario: DLL with exports
- **WHEN** PE file is a DLL with exported functions
- **THEN** return list of exported function names

#### Scenario: EXE without exports
- **WHEN** PE file is an executable without exports
- **THEN** return empty exports array

### Requirement: Digital Signature Verification
The system SHALL detect and extract digital signature information.

#### Scenario: Signed executable
- **WHEN** PE file contains Authenticode signature
- **THEN** extract `SignedBy` as the primary key for the signer name.

### Requirement: Installer Detection
The system SHALL identify installer types from PE metadata.

#### Scenario: NSIS installer
- **WHEN** PE file contains NSIS-specific strings or resources
- **THEN** return InstallerType as "NSIS"

#### Scenario: Inno Setup installer
- **WHEN** PE file contains Inno Setup markers
- **THEN** return InstallerType as "Inno Setup"

#### Scenario: MSI wrapper
- **WHEN** PE file is an MSI wrapper/bootstrapper
- **THEN** return InstallerType as "MSI Wrapper"

#### Scenario: Non-installer executable
- **WHEN** PE file does not match known installer patterns
- **THEN** return null for InstallerType

### Requirement: Entry Point Extraction
The system SHALL extract the entry point address from PE headers.

#### Scenario: Standard entry point
- **WHEN** PE file has valid optional header
- **THEN** return entry_point as RVA (Relative Virtual Address)

### Requirement: Timestamp Extraction
The system SHALL extract compilation timestamp from PE headers.

#### Scenario: Valid timestamp
- **WHEN** PE file contains TimeDateStamp in COFF header
- **THEN** return timestamp as Unix epoch or formatted date string

### Requirement: Error Handling
The system SHALL provide clear error messages for analysis failures.

#### Scenario: Corrupted PE file
- **WHEN** PE file structure is corrupted or incomplete
- **THEN** return error object with descriptive message

#### Scenario: Unsupported PE variant
- **WHEN** PE file uses unsupported features or formats
- **THEN** return error with specific unsupported feature information

## Data Structures

### PEAnalysis Type
```typescript
{
  Format: "PE",
  Architecture: "x86" | "x86_64" | "ARM" | "ARM64",
  is_64bit: boolean,
  entry_point: number,
  sections: Array<{
    name: string,
    virtual_size: number,
    virtual_address: number,
    raw_data_size: number
  }>,
  imports: string[],
  exports: string[],
  CompanyName?: string,
  ProductName?: string,
  FileVersion?: string,
  ProductVersion?: string,
  FileVersionNumber?: string,
  ProductVersionNumber?: string,
  FileDescription?: string,
  InternalName?: string,
  OriginalFilename?: string,
  LegalCopyright?: string,
  Comments?: string,
  SignedBy?: string,
  InstallerType?: string,
  timestamp?: string
}
```

## API

### Function: `analyze_pe_file(data: Uint8Array): string`
Analyzes PE file and returns JSON string with analysis results.

**Parameters:**
- `data`: Uint8Array containing PE file binary data

**Returns:**
- JSON string containing PEAnalysis object or error object

**Throws:**
- Never throws - all errors returned as JSON error objects
