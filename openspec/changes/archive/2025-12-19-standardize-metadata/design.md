# Design: Metadata Standardization

## Context
As the `upload-analyzer` project grows to support more formats (PE, MSI, DMG, DEB, RPM), the metadata fields started to diverge or accumulate aliases for backwards compatibility. To maintain a clean and reliable API, we established a "No Aliases" rule in `FORMATS.md` and `AI_RULES.md`.

## Goals
- Every piece of information must have exactly one primary key.
- Field names must be consistent across all formats (e.g., `Format` instead of `file_type`).
- Specifications must be the source of truth for these standards.

## Technical Decisions
- **Primary Keys**: We use the keys defined in the "Metadata Standards" section of `FORMATS.md` as the definitive set.
- **Spec Deltas**: Each format spec will be updated via a delta to replace its old `Data Structures` section with the new aligned interface.
- **Architecture Strings**: We use normalized strings (x64, x86, arm64) instead of varying platform-specific formats where possible.

## Risks / Trade-offs
- **Breaking Changes**: Formally removing aliases from the spec might be seen as a breaking change for anyone relying on the older, undocumented aliases. However, since the code has already been refactored to remove them, the spec change is simply catching up to reality.
