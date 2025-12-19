# Capability: DMG File Analysis

## MODIFIED Requirements

### Requirement: DMG File Detection
The system SHALL detect and identify DMG file format from binary data.

#### Scenario: Valid DMG file
- **WHEN** binary data contains valid DMG signature and structure
- **THEN** return `Format` as "DMG"

### Requirement: Architecture Detection
The system SHALL identify target CPU architectures from DMG contents.

#### Scenario: Universal binary
- **WHEN** DMG contains universal binary (multiple architectures)
- **THEN** return `Architecture` as "Universal" or a specific architecture if a primary one is detected.

## MODIFIED Data Structures

### DMGAnalysis Type (Standardized)
```typescript
{
  // Basic Format
  Format: "DMG";
  Architecture: "macOS Disk Image";

  // DMG Information
  ImageType?: string;
  Compression?: string;
  HasKolySignature?: "true" | "false";
  KolyOffset?: string;
  DMGVersion?: string;

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

  // Legal Information
  LegalCopyright?: string;

  // macOS Specific
  BundleIdentifier?: string;
  ApplicationBundle?: string;
  ApplicationCategory?: string;
  PrincipalClass?: string;
  ExecutableName?: string;
  PackageType?: string;
  IconFile?: string;
  MinimumSystemVersion?: string;
}
```
