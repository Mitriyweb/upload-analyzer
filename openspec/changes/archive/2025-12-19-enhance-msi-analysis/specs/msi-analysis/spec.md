# Capability: Enhanced MSI Analysis

## Overview
Enhances the MSI analysis to extract structured metadata directly from the internal MSI database tables, providing higher accuracy and more comprehensive information compared to heuristic scanning.

## MODIFIED Requirements

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

## ADDED Requirements

### Requirement: Installer Framework Detection (Improved)
The system SHALL identify the installer framework based on property markers.

#### Scenario: WiX Detection
- **WHEN** the `Property` table contains WiX-specific properties or the `SummaryInformation` matches WiX patterns
- **THEN** set `InstallerFramework` to "WiX Toolset"

## Data Structures

### MSIAnalysis Type (Updated)
```typescript
{
  Format: "MSI",
  Architecture: "Windows Installer Package",

  // Package Constants
  ProductCode?: string,
  UpgradeCode?: string,
  PackageCode?: string,      // New: From SummaryInfo Revision

  // Product Information
  ProductName?: string,
  ProductVersion?: string,
  Manufacturer?: string,

  // Extended Metadata (New)
  Title?: string,
  Subject?: string,
  Author?: string,
  Keywords?: string,
  Comments?: string,
  CreateTime?: string,
  LastSaveTime?: string,

  // Package Inventory (New)
  FileCount?: number,
  TotalFileSize?: number,
  ComponentCount?: number,
  FeatureCount?: number,

  // Requirements & Launch Conditions (New)
  LaunchConditions?: string[],
  MinOSVersion?: string,

  // Installer Context (New)
  InstallerFramework?: string,
  Languages?: string[],      // List of supported LCIDs

  // UI/Experience (New)
  HasUI?: boolean,
  HasIcon?: boolean,
  CanModify?: boolean,
  CanRepair?: boolean,
  CanUninstall?: boolean
}
```
