# Capability: MSI File Analysis

## Overview
Provides analysis of Microsoft Installer (MSI) files using Compound File Binary (CFB) format parsing. Extracts package metadata, product information, and installation properties.

## Requirements

### Requirement: MSI File Detection
The system SHALL detect and identify MSI file format from binary data.

#### Scenario: Valid MSI file
- **WHEN** binary data contains valid CFB signature with MSI-specific streams
- **THEN** return file type as "MSI" with package information

#### Scenario: Invalid MSI file
- **WHEN** binary data does not contain valid CFB/MSI structure
- **THEN** return error indicating invalid MSI format

### Requirement: Product Information Extraction
The system SHALL extract product metadata from MSI database.

#### Scenario: Standard MSI package
- **WHEN** MSI file contains Property table with product info
- **THEN** extract ProductName, Manufacturer, ProductVersion, ProductCode

#### Scenario: Missing product properties
- **WHEN** MSI file lacks standard property table entries
- **THEN** return null or empty values for missing properties

### Requirement: Package Architecture Detection
The system SHALL identify target platform architecture from MSI metadata.

#### Scenario: 64-bit package
- **WHEN** MSI contains Intel64 or x64 platform indicator
- **THEN** return architecture as "x86_64"

#### Scenario: 32-bit package
- **WHEN** MSI contains Intel or x86 platform indicator
- **THEN** return architecture as "x86"

#### Scenario: Platform-neutral package
- **WHEN** MSI does not specify platform
- **THEN** return architecture as "Any" or null

### Requirement: Installation Properties
The system SHALL extract installation-related properties from MSI database.

#### Scenario: Installation directory
- **WHEN** MSI contains TARGETDIR or INSTALLDIR properties
- **THEN** extract default installation path

#### Scenario: Required Windows version
- **WHEN** MSI specifies minimum Windows version
- **THEN** extract version requirement information

### Requirement: Component Analysis
The system SHALL parse MSI component and feature tables.

#### Scenario: Multiple components
- **WHEN** MSI contains Component table entries
- **THEN** return list of components with GUIDs and attributes

#### Scenario: Feature hierarchy
- **WHEN** MSI contains Feature table
- **THEN** extract feature names and parent-child relationships

### Requirement: File Table Extraction
The system SHALL extract file information from MSI database.

#### Scenario: Installed files list
- **WHEN** MSI contains File table
- **THEN** return list of files with names, sizes, and versions

#### Scenario: Large file count
- **WHEN** MSI contains hundreds of files
- **THEN** efficiently parse and return file list without performance degradation

### Requirement: Custom Action Detection
The system SHALL identify custom actions defined in MSI.

#### Scenario: DLL custom actions
- **WHEN** MSI contains CustomAction table with DLL entries
- **THEN** extract custom action names and target DLLs

#### Scenario: Script custom actions
- **WHEN** MSI contains VBScript or JScript custom actions
- **THEN** identify script-based actions

### Requirement: Upgrade Code Extraction
The system SHALL extract upgrade and related codes from MSI.

#### Scenario: Upgrade code present
- **WHEN** MSI contains UpgradeCode property
- **THEN** extract GUID for upgrade detection

#### Scenario: Related products
- **WHEN** MSI contains Upgrade table
- **THEN** extract related product codes and version ranges

### Requirement: Summary Information
The system SHALL parse MSI summary information stream.

#### Scenario: Summary stream present
- **WHEN** MSI contains \x05SummaryInformation stream
- **THEN** extract title, subject, author, keywords, comments

#### Scenario: Creation timestamp
- **WHEN** summary information contains creation time
- **THEN** return formatted creation date

### Requirement: Error Handling
The system SHALL provide clear error messages for MSI analysis failures.

#### Scenario: Corrupted MSI structure
- **WHEN** CFB structure is corrupted or incomplete
- **THEN** return error object with descriptive message

#### Scenario: Encrypted MSI
- **WHEN** MSI file is encrypted or password-protected
- **THEN** return error indicating encryption

## Data Structures

### MSIAnalysis Type
```typescript
{
  file_type: "MSI",
  architecture?: "x86" | "x86_64" | "Any",
  ProductName?: string,
  Manufacturer?: string,
  ProductVersion?: string,
  ProductCode?: string,
  UpgradeCode?: string,
  PackageCode?: string,
  InstallDir?: string,
  MinWindowsVersion?: string,
  components?: Array<{
    name: string,
    guid: string,
    directory: string
  }>,
  features?: Array<{
    name: string,
    title: string,
    level: number
  }>,
  files?: Array<{
    name: string,
    size: number,
    version?: string
  }>,
  customActions?: string[],
  summary?: {
    title?: string,
    subject?: string,
    author?: string,
    keywords?: string,
    comments?: string,
    createdAt?: string
  }
}
```

## API

### Function: `analyze_msi_file(data: Uint8Array): string`
Analyzes MSI file and returns JSON string with analysis results.

**Parameters:**
- `data`: Uint8Array containing MSI file binary data

**Returns:**
- JSON string containing MSIAnalysis object or error object

**Throws:**
- Never throws - all errors returned as JSON error objects
