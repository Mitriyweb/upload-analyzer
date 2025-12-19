# Tasks: Standardize Metadata Fields

## 1. Specification Updates
- [x] Update `pe-analysis` spec delta <!-- id: 0 -->
- [x] Update `msi-analysis` spec delta <!-- id: 4 -->
- [x] Update `deb-analysis` spec delta <!-- id: 6 -->
- [x] Update `rpm-analysis` spec delta <!-- id: 8 -->
- [x] Update `dmg-analysis` spec delta <!-- id: 10 -->
- [x] Update `file-detection` spec delta (rename `file_type` -> `Format`) <!-- id: 14 -->
- [x] Update `typescript-integration` spec delta <!-- id: 15 -->

## 2. Code & Type Updates
- [x] Update `FileInfo` in `src/ts/types/index.d.ts` <!-- id: 16 -->
- [x] Update `get_file_info` in `src/rs/lib.rs` <!-- id: 17 -->
- [x] Update all `get_file_info` implementations in format modules <!-- id: 18 -->

## 3. Verification
- [x] Run `openspec validate standardize-metadata --strict` <!-- id: 12 -->
- [x] Verify `FORMATS.md` consistency <!-- id: 13 -->
- [x] Run `npm run build:wasm` and `npm run build:types` <!-- id: 19 -->
