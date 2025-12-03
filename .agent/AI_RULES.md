# AI Agent Rules and Guidelines

This project uses AI agent guidelines to maintain code quality and consistency.

## Quick Reference

- **Rust Guidelines**: See `.agent/workflows/rust-guidelines.md` or use `/rust-guidelines` command
- **Linter**: `npm run lint:rust` to check Rust code
- **Auto-fix**: `npm run lint:rust:fix` to automatically fix issues

## For AI Agents

When working on this project:

### ✅ MUST DO
1. Read `.agent/workflows/rust-guidelines.md` before modifying Rust code
2. Follow all rules marked with `ai-rules: true` in YAML frontmatter
3. Run clippy before committing: `npm run lint:rust`
4. Use `Result<T, E>` for all fallible operations
5. Use iterator methods over manual loops
6. Prefer `&str` over `String` for parameters
7. Use `const` for all constants
8. Use type aliases for complex types (e.g., `MetadataResult`)
9. Document all public APIs with `///` comments

### 🚫 NEVER DO
1. **Never use `unwrap()`, `expect()`, or `panic!()`** in production code
2. **Never use generic types (`<T>`)** - use concrete types only
3. **Never use concurrency primitives**:
   - No `Arc`, `Mutex`, `RwLock`
   - No `async fn` or `.await`
   - No `thread::spawn`
   - No `mpsc::channel`
4. **Never use `.to_string()` in hot paths** - causes allocations
5. **Never use `todo!()` or `unimplemented!()`** - complete all code
6. **Never use `unsafe`** without detailed safety documentation

### 🎯 WASM-Specific Rules

This is a **WebAssembly project** - special rules apply:

- **Size matters**: Every byte counts in WASM bundle
- **Single-threaded**: No concurrency, no async/await
- **Concrete types only**: Generics increase bundle size
- **Synchronous code**: JavaScript handles async at boundary
- **Error handling**: Convert all errors to JSON at WASM boundary

### 📋 Clippy Lints

**Denied (compilation fails):**
- `unwrap_used`, `expect_used` - No panicking
- `panic` - No explicit panics
- `todo`, `unimplemented` - Complete all code

**Denied (concurrency & complexity):**
- `type_complexity` - Use type aliases, avoid generics
- `future_not_send` - No async/await
- `await_holding_lock` - No mutex with await
- `await_holding_refcell_ref` - No RefCell issues
- `mutex_atomic` - No mutex/atomics

**Warned (must fix):**
- `inefficient_to_string` - Use efficient conversions
- `clone_on_ref_ptr` - Avoid unnecessary clones

### 🔧 Type Aliases

Use type aliases to simplify complex types:

```rust
// ✅ CORRECT
pub type MetadataResult = Result<HashMap<String, String>, String>;
type CfbFile<'a> = CompoundFile<Cursor<&'a [u8]>>;

fn parse_metadata(data: &[u8]) -> MetadataResult { }

// ❌ WRONG
fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String> { }
```

### 📝 Error Handling Pattern

```rust
// ✅ CORRECT - Proper error handling
fn parse_data(data: &[u8]) -> MetadataResult {
    let obj = Object::parse(data)
        .map_err(|e| format!("Failed to parse: {}", e))?;
    Ok(process_object(obj))
}

// ❌ WRONG - Never panic
fn parse_data(data: &[u8]) -> HashMap<String, String> {
    let obj = Object::parse(data).unwrap(); // FORBIDDEN!
    process_object(obj)
}
```

### 🚀 Performance Guidelines

1. **Minimize allocations** - Reuse `HashMap`, use `&[u8]` slices
2. **Use `#[inline]`** for small, frequently-called functions
3. **Avoid string conversions** - Work with bytes when possible
4. **No redundant work** - Check once, use result
5. **Early returns** - Exit fast paths quickly

## For Developers

The `.agent` directory contains:
- **Coding standards** enforced by linters
- **Best practices** for this specific project
- **WASM optimization** guidelines
- **Language-specific rules** for Rust

These guidelines ensure AI-generated code meets project standards.

## Enforcement

- **Clippy**: Automatically checks code on every build
- **Type aliases**: Required for complex types
- **Code review**: Manual check for `<T>`, `Arc`, `async`, etc.
- **Documentation**: All public APIs must be documented

## Version

- **Guidelines version**: 1.0
- **Last updated**: 2025-12-03
- **Clippy threshold**: type-complexity = 100

---

**Remember**: WASM is single-threaded, size-sensitive, and synchronous. Keep it simple, concrete, and fast!
