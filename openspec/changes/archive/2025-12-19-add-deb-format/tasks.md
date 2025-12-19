## 1. Documentation & Planning
- [x] 1.1 Finalize design decisions for `deb` parsing <!-- id: 1.1 -->
- [x] 1.2 Update `FORMATS.md` to include `DEB` status <!-- id: 1.2 -->

## 2. Backend Implementation (Rust)
- [x] 2.1 Add `ar` and `tar` dependencies to `Cargo.toml` <!-- id: 2.1 -->
- [x] 2.2 Implement `src/rs/deb.rs` with `DEBAnalyzer` <!-- id: 2.2 -->
- [x] 2.3 Add `deb` detection and registration in `src/rs/lib.rs` <!-- id: 2.3 -->

## 3. Frontend Integration (TypeScript)
- [x] 3.1 Update `src/ts/types/index.d.ts` with `DEBAnalysis` interface <!-- id: 3.1 -->
- [x] 3.2 Add `isDEBAnalysis` type guard to `src/ts/helpers.ts` <!-- id: 3.2 -->

## 4. Verification
- [x] 4.1 Verify with sample `.deb` files <!-- id: 4.1 -->
- [x] 4.2 Run `npm run lint` and `npm run build` <!-- id: 4.2 -->
