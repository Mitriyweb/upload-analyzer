# Capability: File Type Detection

## MODIFIED Requirements

### Requirement: File Size Validation
The system SHALL validate file size constraints and return standard fields.

#### Scenario: File size
- **WHEN** analyzing any file
- **THEN** return `Format` and `Size` using primary keys.

## MODIFIED Data Structures

### FileInfo Type (Standardized)
```typescript
{
  Format: "PE" | "MSI" | "DMG" | "DEB" | "RPM" | "Unknown" | "Invalid binary",
  Size: number,
  FormatVersion?: string
}
```
