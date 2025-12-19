# Design: Enhanced MSI Database Parsing

## Architecture
We will implement a minimal "MSI Database Reader" within the `msi.rs` module. It will NOT be a full SQL engine, but a specialized parser for the table streams stored within the Compound File (CFB).

## Implementation Strategy

### 1. Table Name De-mangling
MSI table names and stream names are encoded using a custom base64-like scheme. We need to implement the reverse transformation (e.g., `Property` table might be stored in a stream with a mangled name).

### 2. String Pool Decoding
All strings in an MSI are stored in two special streams:
- `!StringPool`: Header with string counts, lengths, and reference counts.
- `!StringData`: Concatenated raw UTF-16 or ANSI string data.
We will implement a reader for these streams to resolve string references (indices) found in other tables.

### 3. Comprehensive Table Parsing
We will prioritize the following tables for maximum data extraction:
- **`Property`**: Core metadata (ProductCode, UpgradeCode, version, manufacturer, etc.).
- **`File`**: Total file count and size accumulation.
- **`Component` / `Feature`**: Counts to represent package complexity.
- **`LaunchCondition`**: System requirements (e.g., minimum OS version, prerequisite software).
- **`Binary` / `Icon`**: Extraction of embedded assets (icons) or at least their metadata.
- **`Media`**: Installation source information.

### 4. Direct OLE Property Set Extraction
Extract ALL standard and custom OLE properties from the Summary Information stream to ensure no metadata is missed.

## Data Mapping (New Fields)
| MSI Source | Tag | Metadata Field |
| --- | --- | --- |
| Summary Info | 9 (Revision) | `PackageCode` |
| Property Table | `ProductCode` | `ProductCode` (Structured) |
| Property Table | `UpgradeCode` | `UpgradeCode` (Structured) |
| Property Table | `ARPNOMODIFY` | `CanModify` |
| Property Table | `DefaultUIFont` | `HasUI` (Inferred) |

## Performance and Size
- **Size**: We will avoid heavy crates like `msi-parser` if possible and implement a minimal version to save space.
- **Complexity**: Keep the parser synchronous and allocation-light.
