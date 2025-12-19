# Change: Standardize Metadata Fields

## Why
To ensure consistency and clarity across all supported file formats, we need to formalize the "Metadata Standards" and "No Aliases" rule in the project specifications. This aligns the formal requirements with the current implementation and `FORMATS.md`, making the analysis output predictable for consumers.

## What Changes
- **Standardized Field Names**: Update all specifications (including core detection and integration) to use `Format` instead of `file_type` and align `Architecture` strings.
- **No Metadata Aliasing**: Explicitly forbid redundant field mappings (e.g., mapping a value to both `Vendor` and `Manufacturer`).
- **Unified Data Structures**: Align the `MSIAnalysis`, `PEAnalysis`, `DEBAnalysis`, `RPMAnalysis`, `DMGAnalysis`, and `FileInfo` types in the specs with the current `FORMATS.md` and TypeScript definitions.
- **Spec Cleanup**: Remove obsolete scenarios or requirements that mention aliased fields.

## Impact
- **Affected Specs**: `pe-analysis`, `msi-analysis`, `deb-analysis`, `rpm-analysis`, `dmg-analysis`, `file-detection`, `typescript-integration`.
- **Affected Code**: Update `get_file_info` in `src/rs/lib.rs` and `FileInfo` in `src/ts/types/index.d.ts` for total consistency.
- **Breaking Changes**: Formally renames `type` to `Format` in `get_file_info` output.
