# GitHub Copilot Instructions for upload-analyzer

This repository contains specific rules and workflows for AI agents. Please follow these instructions strictly when generating code or assisting with tasks.

## 🎯 Project Context
**upload-analyzer** is a WebAssembly (WASM) project focused on analyzing PE, MSI, and DMG files directly in the browser. It is built with Rust and optimized for size and performance.

---

## 🦀 Rust & WASM Coding Standards

### ✅ MUST DO
1. **Error Handling**: ALWAYS use `Result<T, E>` and the `?` operator. Propagation is mandatory.
2. **Functional Patterns**: Prefer iterator methods (`.map()`, `.filter()`, etc.) over manual loops.
3. **Optimized Strings**: Use `&str` for parameters instead of `String` to avoid unnecessary allocations.
4. **Constants**: Use `const` for all magic numbers and signatures.
5. **Type Aliases**: Use `pub type MetadataResult = Result<HashMap<String, String>, String>;` for complex types.
6. **Documentation**: Document all public APIs with `///` comments.
7. **Performance**: Use `#[inline]` for small, frequently-called functions.

### 🚫 NEVER DO
1. **No Panics**: Never use `unwrap()`, `expect()`, `panic!()`, `todo!()`, or `unimplemented!()`.
2. **No Generics**: Avoid generic types (`<T>`) to keep the WASM bundle size minimal. Use concrete types only.
3. **No Concurrency**: Do not use `Arc`, `Mutex`, `RwLock`, `async/await`, or `thread::spawn`. WASM is single-threaded.
4. **No Hot-Path Allocations**: Avoid `.to_string()` or `.clone()` in performance-critical sections.
5. **No Unsafe**: Do not use `unsafe` unless absolutely necessary and documented with safety invariants.

---

## 🛠 Workflows (OpenSpec)

This project uses **OpenSpec** for change management. Follow these processes:

### 1. Proposal Phase (`/openspec-proposal`)
- Scaffold `proposal.md`, `tasks.md`, and `design.md` under `openspec/changes/<id>/`.
- Do NOT write implementation code during this phase.
- Validate with `openspec validate <id> --strict`.

### 2. Implementation Phase (`/openspec-apply`)
- Read `proposal.md` and `tasks.md` before starting.
- Implement tasks sequentially.
- Keep changes minimal and focused.
- Mark tasks as completed in `tasks.md`.

### 3. Archive Phase (`/openspec-archive`)
- Use `openspec archive <id> --yes` to land changes.
- Update relevant specs in the `openspec/` directory.
- Verify with `openspec validate --strict`.

---

## 🔧 Maintenance
- **Linting**: Run `npm run lint:rust` to check for compliance.
- **Auto-fix**: Use `npm run lint:rust:fix` when possible.
- Rules are enforced via `clippy.toml` and `Cargo.toml`.

---
**Remember**: WASM is single-threaded, size-sensitive, and synchronous. Keep it simple, concrete, and fast!
