# Capability: RPM File Analysis

## MODIFIED Requirements

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

## MODIFIED Data Structures

### RPMAnalysis Type (Standardized)
```typescript
{
  // Basic Format
  Format: "RPM";
  Architecture?: string;

  // RPM Package Information
  Package?: string;
  Version?: string;
  Release?: string;
  Vendor?: string;
  Summary?: string;
  License?: string;
  GroupName?: string;
  Url?: string;
  SourceRpm?: string;

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
