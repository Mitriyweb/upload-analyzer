# Capability: DEB File Analysis

## MODIFIED Requirements

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

## MODIFIED Data Structures

### DEBAnalysis Type (Standardized)
```typescript
{
  // Basic Format
  Format: "DEB";
  Architecture?: string;

  // DEB Package Information
  Package?: string;
  Version?: string;
  Maintainer?: string;
  Description?: string;
  Section?: string;
  Priority?: string;
  Depends?: string;
  Homepage?: string;

  // Standard Metadata
  ProductName?: string;
  ProductVersion?: string;
  Manufacturer?: string;
  Publisher?: string;
  CompanyName?: string;
  Vendor?: string;
  ProductCode?: string;
  UpgradeCode?: string;
  PackageCode?: string;
  Title?: string;
  Comments?: string;
  Keywords?: string;
}
```
