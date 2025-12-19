# Capability: TypeScript Integration

## MODIFIED Requirements

### Requirement: Type Guards
The system SHALL provide type-safe access to standardized metadata fields.

#### Scenario: Accessing Format
- **WHEN** using `PEAnalysis` or any format result
- **THEN** field `Format` SHALL be available and contain the format name.

## MODIFIED Data Structures

### Type Guard Signatures (Standardized)
Type guards SHALL use the `Format` field for discrimination.

```typescript
function isPEAnalysis(result: FileAnalysis): result is PEAnalysis;
function isMSIAnalysis(result: FileAnalysis): result is MSIAnalysis;
```

### FileInfo Type (Standardized)
```typescript
export interface FileInfo {
  Format: string;
  Size: string;
}
```
