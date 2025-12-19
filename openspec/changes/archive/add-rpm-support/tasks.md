# Tasks: Add RPM Support

- [x] Implement RPM Header Parser in Rust <!-- id: 0 -->
    - [x] Create `src/rs/rpm.rs` with Lead and Header detection <!-- id: 1 -->
    - [x] Implement Index Entry parsing <!-- id: 2 -->
    - [x] Implement Store data extraction for key tags <!-- id: 3 -->
- [x] Integrate RPM Analysis in `lib.rs` <!-- id: 4 -->
    - [x] Add `is_rpm_file` check <!-- id: 5 -->
    - [x] Call `RPMAnalyzer::parse_metadata` in `parse_metadata` <!-- id: 6 -->
- [x] Update TypeScript Integration <!-- id: 7 -->
    - [x] Add `RPMAnalysis` to `src/ts/types/index.d.ts` <!-- id: 8 -->
    - [x] Update `FileAnalysis` union type <!-- id: 9 -->
    - [x] Add `isRPMAnalysis` to `src/ts/helpers.ts` <!-- id: 10 -->
- [x] Verification <!-- id: 11 -->
    - [x] Run `npm run lint:rust` <!-- id: 12 -->
    - [x] Run `npm run lint:ts` <!-- id: 13 -->
    - [x] Manual test with sample RPM files <!-- id: 14 -->
