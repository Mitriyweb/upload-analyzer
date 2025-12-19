# Proposal: Enhanced MSI Analysis

## Goal
Significantly improve the depth and accuracy of MSI (Windows Installer) file analysis. The goal is to provide a **comprehensive superset** of current data by transitioning from heuristic-based string searching to structured parsing of the internal MSI database, while retaining current heuristics as fallbacks.

## Motivation
The current implementation relies on scanning the entire file for patterns or extracting limited fields from the `SummaryInformation` stream. This approach is prone to false positives and misses a vast amount of structured data (files, components, features, launch conditions) stored in the MSI database tables. We need a robust foundation to extract all possible metadata.

## User Review Required
> [!IMPORTANT]
> This change introduces a specialized parser for the MSI internal database format. While more accurate, it increases the complexity of the `msi` module. We aim to keep the WASM size increase minimal by focusing only on high-value tables like `Property`.

## Proposed Changes

### Rust (WASM Engine)
- [MODIFY] `src/rs/msi.rs`:
    - Implement MSI table name de-mangling.
    - Implement `!StringPool` and `!StringData` parsing.
    - Implement structured row reading for the `Property` table.
    - Extend `SummaryInformation` extraction to include all standard OLE properties.

### TypeScript Integration
- [MODIFY] `src/ts/types/index.d.ts`: Update `MSIAnalysis` with new fields (PackageCode, Revision, Keywords, etc.).

## Verification Plan
### Automated Tests
- `npm run lint:rust` - Ensure no guideline violations.
- `cargo test msi` - Add unit tests with mock MSI database streams.

### Manual Verification
- Compare results of the new parser vs old heuristic approach using real-world MSI files.
- Monitor WASM binary size to ensure it remains under 250KB (current is ~500KB due to recent additions, target might need adjustment or further optimization).
