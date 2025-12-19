# Tasks: Enhanced MSI Analysis

- [x] Implement MSI Stream Name De-mangling <!-- id: 0 -->
    - [x] Implementation of `decode_msi_stream_name` helper <!-- id: 1 -->
- [x] Implement MSI String Pool Parser <!-- id: 2 -->
    - [x] Read `!StringPool` header and entries <!-- id: 3 -->
    - [x] Read and decompress `!StringData` (if necessary, though MSI strings are usually plain UTF-16/ANSI) <!-- id: 4 -->
- [x] Implement MSI Table Row Parser <!-- id: 5 -->
    - [x] Locate and read `Property` table stream <!-- id: 6 -->
    - [x] Map row values to decoded strings <!-- id: 7 -->
- [x] Refactor `src/rs/msi.rs` <!-- id: 8 -->
    - [x] Replace heuristic extraction with structured database extraction <!-- id: 9 -->
    - [x] Extend OLE Summary Information extraction <!-- id: 10 -->
- [x] Update TypeScript Definitions <!-- id: 11 -->
    - [x] Add new fields to `MSIAnalysis` interface <!-- id: 12 -->
- [x] Verification <!-- id: 13 -->
    - [x] Unit tests for de-mangling and string pool decoding <!-- id: 14 -->
    - [x] Integration test with sample MSI files <!-- id: 15 -->
    - [x] Verify WASM size impact <!-- id: 16 -->
