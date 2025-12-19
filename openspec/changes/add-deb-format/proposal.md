# Change: Add DEB Format Support

## Why
Users need to analyze Debian (`.deb`) packages to extract metadata like package name, version, and architecture. This is a common requirement for Linux software distribution analysis and complements the existing PE, MSI, and DMG support.

## What Changes
- Add `DEB` format detection to the core analyzer.
- Implement `DEBAnalyzer` in Rust to extract metadata from the `control` file inside the `.deb` archive.
- Support extraction of:
    - Package Name
    - Version
    - Architecture
    - Maintainer
    - Description
    - Depends
- Add TypeScript definitions and type guards for the new format.

## Impact
- Affected specs: `deb-analysis` (new capability)
- Affected code: `src/rs/lib.rs` (detection), `src/rs/deb.rs` (new), `src/ts/types/index.d.ts` (types), `src/ts/helpers.ts` (guards).
