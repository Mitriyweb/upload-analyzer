# Proposal: Add RPM Format Support

## Goal
Add support for analyzing RPM (Red Hat Package Manager) files to the `upload-analyzer` project. This includes format detection, metadata extraction from the RPM header, and exposing this information through the WASM API and TypeScript definitions.

## Motivation
Expanding the supported package formats beyond DEB to include RPM will make the tool more versatile for Linux-bound binaries and installers.

## User Review Required
> [!IMPORTANT]
> The RPM analysis will focus on the Header structure of the RPM file to extract metadata like Name, Version, Release, Architecture, etc. Full payload extraction (CPIO) is not planned for the initial version to keep the WASM size small.

## Proposed Changes

### Rust (WASM Engine)
- [NEW] `src/rs/rpm.rs`: Implement RPM format detection and header parsing.
- [MODIFY] `src/rs/lib.rs`: Integrate the `rpm` module into the main analysis pipeline.

### TypeScript Integration
- [MODIFY] `src/ts/types/index.d.ts`: Add `RPMAnalysis` interface and update `FileAnalysis` union.
- [MODIFY] `src/ts/helpers.ts`: Add `isRPMAnalysis` type guard.

## Verification Plan
### Automated Tests
- Run `npm run lint:rust` to ensure no violations of `rust-guidelines`.
- Run `npm run lint:ts` to ensure TypeScript compliance.

### Manual Verification
- Verify detection and metadata extraction with sample RPM files (to be provided/generated).
- Check that the UI (if any) or consumers of the package correctly handle the new `RPMAnalysis` type.
