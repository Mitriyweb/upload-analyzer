---
description: Rust development guidelines for AI agents
ai-rules: true
ai-agent-guidelines: rust
language: rust
scope: project
enforcement: clippy
version: 1.0
---

<!-- 
  AI AGENT GUIDELINES
  This file contains mandatory coding standards for AI agents working on Rust code.
  These rules are enforced by clippy linter configured in Cargo.toml and clippy.toml
  
  IDE Integration:
  - This file is part of the .agent/workflows directory
  - AI agents must follow these rules when generating or modifying Rust code
  - Clippy will automatically enforce these rules during compilation
-->

# Rust Development Guidelines for upload-analyzer

This document defines the Rust coding standards and best practices for AI agents working on the upload-analyzer project. These rules ensure code quality, performance, and maintainability.

## 🎯 Project Context

This is a **WebAssembly (WASM) project** that analyzes PE, MSI, and DMG files in the browser. Code must be:
- **Size-optimized** for WASM bundle size
- **Performance-focused** for browser execution
- **Error-resilient** for user-facing functionality

## ✅ Required Best Practices

### Error Handling

**ALWAYS use proper error handling:**

```rust
// ✅ CORRECT - Use Result and propagate errors
fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String> {
    let obj = Object::parse(data)
        .map_err(|e| format!("Failed to parse file: {}", e))?;
    Ok(process_object(obj))
}

// ❌ WRONG - Never use unwrap() or expect() in production
fn parse_metadata(data: &[u8]) -> HashMap<String, String> {
    let obj = Object::parse(data).unwrap(); // FORBIDDEN!
    process_object(obj)
}
```

**Rules:**
- Use `Result<T, E>` for fallible operations
- Use `?` operator to propagate errors
- Use `match` or `if let` to handle `Option<T>`
- Convert errors to user-friendly messages at WASM boundaries

### Iterator Methods Over Manual Loops

**Prefer functional iterator methods:**

```rust
// ✅ CORRECT - Functional, clear, optimized
let valid_strings: Vec<String> = buffer
    .iter()
    .filter(|&&byte| (32..=126).contains(&byte))
    .map(|&byte| byte as char)
    .collect();

// ❌ WRONG - Manual loop when iterator would work
let mut valid_strings = Vec::new();
for byte in buffer {
    if *byte >= 32 && *byte <= 126 {
        valid_strings.push(*byte as char);
    }
}
```

**Preferred methods:**
- `.filter()`, `.map()`, `.filter_map()`, `.flat_map()`
- `.find()`, `.any()`, `.all()`
- `.fold()`, `.collect()`
- `.windows()`, `.chunks()` for slice operations

### String Handling

**Use `&str` over `String` when possible:**

```rust
// ✅ CORRECT - Borrow when you don't need ownership
fn is_valid_metadata_string(s: &str) -> bool {
    s.len() >= 3 && s.chars().any(|c| c.is_alphabetic())
}

// ❌ WRONG - Unnecessary allocation
fn is_valid_metadata_string(s: String) -> bool {
    s.len() >= 3 && s.chars().any(|c| c.is_alphabetic())
}
```

**Rules:**
- Use `&str` for function parameters that only read strings
- Use `String` only when you need ownership or mutation
- Avoid `.to_string()` in hot paths - use `.into()` or `String::from()` when necessary
- Use `format!()` for string building, not repeated `+` operations

### Const and Static

**Use `const` for compile-time constants:**

```rust
// ✅ CORRECT - Compile-time constants
const MSI_SIGNATURE: &[u8] = &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
const MIN_METADATA_STRING_LEN: usize = 3;
const MAX_METADATA_STRING_LEN: usize = 100;

// ❌ WRONG - Magic numbers in code
if data.len() >= 8 && &data[0..8] == &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1] {
    // ...
}
```

### Type Safety

**Leverage Rust's type system:**

```rust
// ✅ CORRECT - Type-safe with trait
pub trait FileAnalyzer {
    fn get_file_info(data: &[u8]) -> HashMap<String, String>;
    fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String>;
}

// ✅ CORRECT - Newtype pattern for clarity
struct ProductCode(String);
struct UpgradeCode(String);

// ❌ WRONG - Stringly-typed data
fn process(code_type: &str, code: String) { /* ... */ }
```

### Inline Optimization

**Use `#[inline]` for small, frequently-called functions:**

```rust
// ✅ CORRECT - Inline small helper functions
#[inline]
fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

// ✅ CORRECT - Inline for WASM performance
#[inline]
pub fn is_msi_file(data: &[u8]) -> bool {
    data.len() >= MIN_MSI_SIGNATURE_SIZE && &data[0..MIN_MSI_SIGNATURE_SIZE] == MSI_SIGNATURE
}
```

## 🚫 Forbidden Constructs

### Never Use These in Production Code

1. **`unwrap()` and `expect()`** - Use proper error handling instead
2. **`panic!()`** - Library code must never panic; return `Result` instead
3. **Mutable global state** - Use parameters or thread-local storage
4. **`unsafe` without documentation** - Must have safety comment explaining invariants
5. **`.clone()` without consideration** - Understand the cost; prefer borrowing
6. **`.to_string()` in hot paths** - Causes allocations; use `&str` or cache strings
7. **`todo!()` or `unimplemented!()`** - Complete all code before committing
8. **Unnecessary allocations** - Reuse buffers, use `&[u8]` slices

### Examples of Forbidden Patterns

```rust
// ❌ FORBIDDEN - unwrap() can panic
let value = map.get("key").unwrap();

// ✅ CORRECT - Handle missing values
let value = map.get("key").ok_or("Key not found")?;
// OR
if let Some(value) = map.get("key") {
    // use value
}

// ❌ FORBIDDEN - panic in library code
if data.len() < 8 {
    panic!("Data too short!");
}

// ✅ CORRECT - Return error
if data.len() < 8 {
    return Err("Data too short".to_string());
}

// ❌ FORBIDDEN - Unnecessary clone
fn process(data: Vec<u8>) {
    let copy = data.clone(); // Why?
    analyze(copy);
}

// ✅ CORRECT - Borrow instead
fn process(data: &[u8]) {
    analyze(data);
}
```

## 📦 Code Organization

### Module Structure

```rust
// lib.rs - Public API and coordination
mod msi;
mod pe;
mod dmg;

pub trait FileAnalyzer {
    fn get_file_info(data: &[u8]) -> HashMap<String, String>;
    fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String>;
}

// Each analyzer in its own module
// msi.rs, pe.rs, dmg.rs
pub struct MSIAnalyzer;
impl FileAnalyzer for MSIAnalyzer { /* ... */ }
```

### Function Size

- Keep functions under 50 lines when possible
- Extract complex logic into helper functions
- Use descriptive names: `extract_signature_info` not `get_sig`

### Documentation

**Document all public APIs:**

```rust
/// Analyzes a file and returns metadata as JSON string.
///
/// # Arguments
/// * `data` - Raw file bytes to analyze
///
/// # Returns
/// JSON string containing file metadata or error message
///
/// # Examples
/// ```
/// let json = analyze_file(&file_bytes);
/// ```
#[wasm_bindgen]
pub fn analyze_file(data: &[u8]) -> String {
    // ...
}
```

## 🚀 Performance Guidelines

### WASM-Specific Optimizations

1. **Minimize allocations** - Reuse `HashMap`, use `&[u8]` slices
2. **Avoid string conversions** - Work with bytes when possible
3. **Use `#[inline]`** - Help LLVM optimize across module boundaries
4. **Limit recursion** - Stack is limited in WASM
5. **Prefer `&[u8]` over `Vec<u8>`** - Avoid unnecessary copies

### Pattern Matching Efficiency

```rust
// ✅ CORRECT - Efficient pattern matching
match obj {
    Object::PE(_) => pe::PEAnalyzer::parse_metadata(buf),
    _ => Err("Unsupported file format".to_string())
}

// ✅ CORRECT - Early returns
if msi::is_msi_file(buf) {
    return msi::MSIAnalyzer::parse_metadata(buf);
}
if dmg::is_dmg_file(buf) {
    return dmg::DMGAnalyzer::parse_metadata(buf);
}
```

### Avoid Redundant Work

```rust
// ✅ CORRECT - Check once, use result
let is_valid = is_valid_metadata_string(&s);
if is_valid {
    meta.insert(key, s.clone());
}

// ❌ WRONG - Checking twice
if is_valid_metadata_string(&s) {
    if is_valid_metadata_string(&s) { // Redundant!
        meta.insert(key, s.clone());
    }
}
```

## 🔧 WASM Integration

### Error Handling at Boundaries

```rust
// ✅ CORRECT - Convert errors to JSON for JavaScript
#[wasm_bindgen]
pub fn analyze_file(data: &[u8]) -> String {
    match parse_metadata(data) {
        Ok(meta) => serde_json::to_string(&meta)
            .unwrap_or_else(|_| "{}".to_string()),
        Err(e) => format!(r#"{{"error": "{}"}}"#, e)
    }
}
```

### Initialization

```rust
// ✅ CORRECT - Set panic hook for debugging
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
```

## 📋 Clippy Configuration

The project uses strict clippy rules. Run before committing:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**Denied lints:**
- `unwrap_used`, `expect_used` - No panicking
- `panic` - No explicit panics
- `todo`, `unimplemented` - Complete all code

**Warned lints:**
- `clone_on_ref_ptr` - Avoid unnecessary clones
- `inefficient_to_string` - Use efficient conversions
- `cognitive_complexity` - Keep functions simple

## 🎓 Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

## ✨ Summary

**Golden Rules:**
1. ✅ Always handle errors with `Result<T, E>`
2. ✅ Use iterators over manual loops
3. ✅ Prefer `&str` over `String`
4. ✅ Use `const` for constants
5. ✅ Inline small functions
6. 🚫 Never use `unwrap()`, `expect()`, or `panic!()`
7. 🚫 Avoid unnecessary allocations and clones
8. 📝 Document public APIs
9. 🚀 Optimize for WASM bundle size
10. 🔍 Run clippy before committing
