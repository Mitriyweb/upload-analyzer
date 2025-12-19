# Design: RPM Format Support

## Architecture
The RPM support will follow the existing pattern in the project:
1.  **Detection**: Identification of the RPM Lead magic bytes.
2.  **Parsing**: Extraction of metadata from the RPM Header segment.
3.  **Representation**: Mapping RPM tags to the standard `RPMAnalysis` structure.
4.  **Integration**: Exposing the analysis through `lib.rs` and TypeScript.

## RPM File Structure
| Segment | Size | Purpose |
| --- | --- | --- |
| Lead | 96 bytes | Magic (`0xedabeedb`), Version, Type, etc. |
| Signature | Variable | Header structure with signatures. |
| Header | Variable | Header structure with metadata. |
| Payload | Variable | Compressed CPIO archive. |

## Implementation Strategy
- **Rust**: A new module `src/rs/rpm.rs` will be created. It will implement a minimal parser for the RPM Header structure.
- **Header Parser**:
    - Validate Header Magic (`0x8eade801`).
    - Read Index Entries to find tags like `NAME`, `VERSION`, `RELEASE`, `ARCH`, `VENDOR`, etc.
    - Extract data from the Store based on Index Entry offsets.
- **WASM Constraints**:
    - No external heavy dependencies.
    - No generics or concurrency.
    - Synchronous parsing (fast for headers).

## Data Mapping
| RPM Tag | Metadata Field |
| --- | --- |
| `NAME` (1000) | `Package` / `ProductName` |
| `VERSION` (1001) | `Version` / `ProductVersion` |
| `RELEASE` (1002) | `Release` |
| `ARCH` (1022) | `Architecture` |
| `VENDOR` (1011) | `Vendor` / `CompanyName` |
| `SUMMARY` (1004) | `Description` |
| `LICENSE` (1014) | `License` |

## TypeScript Changes
- `RPMAnalysis` interface will be added to `src/ts/types/index.d.ts`.
- `isRPMAnalysis` helper in `src/ts/helpers.ts`.
