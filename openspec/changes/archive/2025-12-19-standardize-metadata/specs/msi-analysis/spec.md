# Capability: MSI File Analysis

## Purpose
Provides analysis of Microsoft Installer (MSI) files using Compound File Binary (CFB) format parsing. Extracts package metadata, product information, and installation properties.

## MODIFIED Requirements

### Requirement: MSI File Detection
The system SHALL detect and identify MSI file format from binary data.

#### Scenario: Valid MSI file
- **WHEN** binary data contains valid CFB signature with MSI-specific streams
- **THEN** return `Format` as "MSI" with package information

### Requirement: Product Information Extraction
The system SHALL extract product metadata from the MSI internal database.

#### Scenario: Structured Property Extraction
- **WHEN** an MSI file contains a `Property` table
- **THEN** extract `ProductCode`, `UpgradeCode`, `ProductName`, `ProductVersion`, and `Manufacturer` directly from the table rows.

### Requirement: Summary Information
The system SHALL parse the MSI summary information stream using structured OLE property parsing.

#### Scenario: Comprehensive Summary Information
- **WHEN** an MSI file contains a `SummaryInformation` stream
- **THEN** extract all standard OLE properties including Title, Author, Keywords, and Revision Number (Package Code).

### Requirement: Installer Framework Detection (Improved)
The system SHALL identify the installer framework based on property markers.

#### Scenario: WiX Detection
- **WHEN** the `Property` table contains WiX-specific properties or the `SummaryInformation` matches WiX patterns
- **THEN** set `InstallerFramework` to "WiX Toolset"

## MODIFIED Data Structures

### MSIAnalysis Type (Standardized)
```typescript
{
  Format: "MSI",
  Architecture: "Windows Installer Package",

  // Package Constants
  ProductCode?: string,
  UpgradeCode?: string,
  PackageCode?: string,

  // Product Information
  ProductName?: string,
  ProductVersion?: string,
  Manufacturer?: string,

  // Extended Metadata
  Title?: string,
  Subject?: string,
  Author?: string,
  Keywords?: string,
  Comments?: string,
  CreateTime?: string,
  LastSaveTime?: string,

  // Package Inventory
  FileCount?: string,
  TotalFileSize?: string,
  ComponentCount?: string,
  FeatureCount?: string,

  // Requirements & Launch Conditions
  LaunchConditions?: string,
  MinOSVersion?: string,

  // Installer Context
  InstallerFramework?: string,
  Languages?: string,

  // UI/Experience
  HasUI?: string,
  HasIcon?: string,
  CanModify?: string,
  CanRepair?: string,
  CanUninstall?: string
}
```
