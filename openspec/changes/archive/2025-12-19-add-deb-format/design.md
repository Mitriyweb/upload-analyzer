## Context
The project needs to support DEB package analysis. DEB files are `ar` archives containing three files:
1. `debian-binary`: Text file containing the version of the deb format (usually `2.0`).
2. `control.tar.gz`: Compressed tarball containing metadata (the `control` file).
3. `data.tar.gz` (or `.xz`, `.bz2`): Compressed tarball containing the application data.

## Goals
- Efficiently detect DEB files.
- Extract core metadata from the `control` file.
- Maintain WASM compatibility and performance.

## Decisions
- **Parser Choice**: Use `ar` and `tar` crates. Since we are in WASM, we must ensure these crates don't rely on native FS or threads.
- **Decompression**: Support `gzip` as it's the most common for `control.tar.gz`. If `.xz` or `.zst` are encountered for the control tarball, we may need additional crates, but `gzip` is the priority.
- **Library Integration**: Register the new analyzer in `lib.rs` and follow the `FileAnalyzer` trait pattern.

## Risks / Trade-offs
- **Binary Size**: Adding `ar`, `tar`, and `flate2` (gzip) will increase the WASM module size. We should monitor this against the 250KB target.
- **Compression Formats**: DEB can use various compression for the `control` and `data` tarballs. Supporting all of them (xz, zstd, bzip2) would significantly bloat the binary. We will start with `gzip` for the control member.

## Open Questions
- Should we analyze the `data.tar` content (list of files)? (Initial scope: No, only metadata from `control`).
