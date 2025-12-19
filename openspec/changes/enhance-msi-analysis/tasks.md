# Tasks: Enhanced MSI Analysis

- [ ] Implement MSI Stream Name De-mangling <!-- id: 0 -->
    - [ ] Implementation of `decode_msi_stream_name` helper <!-- id: 1 -->
- [ ] Implement MSI String Pool Parser <!-- id: 2 -->
    - [ ] Read `!StringPool` header and entries <!-- id: 3 -->
    - [ ] Read and decompress `!StringData` (if necessary, though MSI strings are usually plain UTF-16/ANSI) <!-- id: 4 -->
- [ ] Implement MSI Table Row Parser <!-- id: 5 -->
    - [ ] Locate and read `Property` table stream <!-- id: 6 -->
    - [ ] Map row values to decoded strings <!-- id: 7 -->
- [ ] Refactor `src/rs/msi.rs` <!-- id: 8 -->
    - [ ] Replace heuristic extraction with structured database extraction <!-- id: 9 -->
    - [ ] Extend OLE Summary Information extraction <!-- id: 10 -->
- [ ] Update TypeScript Definitions <!-- id: 11 -->
    - [ ] Add new fields to `MSIAnalysis` interface <!-- id: 12 -->
- [ ] Verification <!-- id: 13 -->
    - [ ] Unit tests for de-mangling and string pool decoding <!-- id: 14 -->
    - [ ] Integration test with sample MSI files <!-- id: 15 -->
    - [ ] Verify WASM size impact <!-- id: 16 -->
