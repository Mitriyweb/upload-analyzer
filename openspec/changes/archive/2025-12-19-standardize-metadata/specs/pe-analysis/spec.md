# Capability: PE File Analysis

## MODIFIED Requirements

### Requirement: PE File Detection
The system SHALL detect and identify PE file format from binary data.

#### Scenario: Valid PE file
- **WHEN** binary data contains valid PE signature (MZ header + PE magic)
- **THEN** return `Format` as "PE" with architecture information

### Requirement: Version Resource Extraction
The system SHALL extract version information from PE file resources.

#### Scenario: Standard version info
- **WHEN** PE file contains VS_VERSION_INFO resource
- **THEN** extract `CompanyName`, `ProductName`, `FileVersion`, `ProductVersion`, and `FileDescription` using primary keys.

### Requirement: Digital Signature Verification
The system SHALL detect and extract digital signature information.

#### Scenario: Signed executable
- **WHEN** PE file contains Authenticode signature
- **THEN** extract `SignedBy` as the primary key for the signer name.

## MODIFIED Data Structures

### PEAnalysis Type (Standardized)
```typescript
{
  // Basic Format
  Format: "PE";
  Architecture: "x86" | "x64";

  // File Header
  Machine?: string;
  NumberOfSections?: string;
  SizeOfOptionalHeader?: string;
  Characteristics?: string;
  PointerToSymbolTable?: string;
  NumberOfSymbols?: string;
  Timestamp?: string;

  // Optional Header
  EntryPoint?: string;
  ImageBase?: string;
  SizeOfImage?: string;
  Subsystem?: string;
  DllCharacteristics?: string;

  // Version Information
  HasVersionInfo?: "true" | "false";
  HasResources?: "true" | "false";
  FileVersionNumber?: string;
  ProductVersionNumber?: string;
  FileVersion?: string;
  ProductVersion?: string;
  FileFlags?: string;
  FileOS?: string;
  FileType?: string;

  // Standard Metadata
  CompanyName?: string;
  ProductName?: string;
  Manufacturer?: string;
  Publisher?: string;
  Vendor?: string;
  FileDescription?: string;
  InternalName?: string;
  OriginalFilename?: string;
  LegalCopyright?: string;
  LegalTrademarks?: string;
  Comments?: string;
  Keywords?: string;
  PrivateBuild?: string;
  SpecialBuild?: string;

  // Digital Signature
  SignedBy?: string;

  // Installer Detection
  InstallerType?: "Inno Setup" | "NSIS (Nullsoft)" | "InstallShield" | "WiX Toolset" | "Wise Installer" | "Setup Factory" | "Smart Install Maker";
  EmbeddedMSI?: "true" | "false";
  MSIOffset?: string;

  // Translation/Language
  TranslationCount?: string;
  Language?: string;
  StringsCount?: string;
  NoStringsFound?: "true" | "false";

  // Error states
  VersionInfoError?: string;
  ResourcesError?: string;
  CompoundFileError?: string;
}
```
