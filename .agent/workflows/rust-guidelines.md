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

## Table of Contents

- [Project Context](#-project-context)
- [WASM-Specific Development](#-wasm-specific-development)
  - [Required Best Practices](#-required-best-practices)
  - [Forbidden Constructs](#-forbidden-constructs)
  - [WASM Integration](#-wasm-integration)
- [Backend Development (Optional)](#-backend-development-optional)
  - [Backend Stack & Libraries](#backend-stack--libraries)
  - [OpenAPI Generation](#openapi-generation)
  - [Data Types & Serialization](#data-types--serialization)
  - [Error Handling (Problem Details)](#error-handling-problem-details)
  - [HTTP Handlers](#http-handlers)
  - [Idempotency & ETags](#idempotency--etags)
  - [Timestamp Handling](#timestamp-handling)
- [Universal Best Practices](#-universal-best-practices)
  - [Error Handling](#error-handling)
  - [Iterator Methods](#iterator-methods-over-manual-loops)
  - [String Handling](#string-handling)
  - [Constants](#const-and-static)
  - [Code Organization](#-code-organization)
  - [Performance Guidelines](#-performance-guidelines)
- [Clippy Configuration](#-clippy-configuration)
- [Learning Resources](#-learning-resources)

## 🎯 Project Context

This is primarily a **WebAssembly (WASM) project** that analyzes PE, MSI, and DMG files in the browser. The guidelines are organized into:

1. **WASM-Specific Development** - Rules for browser-based WASM code (current primary focus)
2. **Backend Development** - Patterns for server-side Rust APIs (if/when backend is added)
3. **Universal Best Practices** - Rules that apply to both contexts

### WASM Code Requirements

WASM code must be:
- **Size-optimized** for WASM bundle size
- **Performance-focused** for browser execution
- **Error-resilient** for user-facing functionality

---

## 🌐 WASM-Specific Development

This section contains rules specific to WebAssembly development. **These restrictions do NOT apply to backend code.**

## ✅ Required Best Practices (WASM)

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

### No Generic Types

**CRITICAL: Do not use generic types (`<T>`) in this project**

Generics increase WASM bundle size and compilation complexity. Always use concrete types.

```rust
// ❌ FORBIDDEN - Generic function
fn parse_data<T: Deserialize>(data: &[u8]) -> Result<T, String> {
    serde_json::from_slice(data).map_err(|e| e.to_string())
}

fn process<T: Clone>(item: T) -> T {
    item.clone()
}

// ✅ CORRECT - Concrete types
fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String> {
    serde_json::from_slice(data).map_err(|e| e.to_string())
}

fn process_string(item: &str) -> String {
    item.to_string()
}

// ❌ FORBIDDEN - Generic struct
struct Container<T> {
    value: T,
}

// ✅ CORRECT - Concrete struct
struct MetadataContainer {
    value: HashMap<String, String>,
}

// ❌ FORBIDDEN - Generic trait implementation
impl<T: Display> FileAnalyzer for GenericAnalyzer<T> {
    // ...
}

// ✅ CORRECT - Concrete trait implementation
impl FileAnalyzer for PEAnalyzer {
    fn get_file_info(data: &[u8]) -> HashMap<String, String> {
        // concrete implementation
    }
}
```

**Why no generics?**
- Increases WASM bundle size (each generic instantiation = more code)
- Complicates compilation and optimization
- Not needed for this project's scope
- Concrete types are clearer and more maintainable

**Use type aliases for complex types:**

```rust
// ✅ CORRECT - Type alias for common pattern
pub type MetadataResult = Result<HashMap<String, String>, String>;
type CfbFile<'a> = CompoundFile<Cursor<&'a [u8]>>;

fn parse_metadata(data: &[u8]) -> MetadataResult {
    // Clean and readable
}

fn extract_summary(cfb: &mut CfbFile) {
    // Much better than CompoundFile<Cursor<&[u8]>>
}

// ❌ WRONG - Inline complex types
fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String> {
    // Verbose and triggers type_complexity lint
}
```

**Enforcement:**
- `type_complexity` lint denies complex types
- Threshold set to 100 (very low) to catch generic usage
- Manual code review for any `<T>` syntax
- Use type aliases to simplify complex concrete types

### No Concurrency Primitives

**CRITICAL: Do not use concurrency in this project**

WASM runs in a single-threaded environment. Concurrency primitives don't work and add unnecessary complexity.

```rust
// ❌ FORBIDDEN - Arc and Mutex (no multi-threading in WASM)
use std::sync::{Arc, Mutex};

fn process_shared(data: Arc<Mutex<Vec<u8>>>) {
    // This doesn't make sense in WASM - no threads!
    let mut d = data.lock().unwrap();
    d.push(1);
}

// ✅ CORRECT - Direct ownership
fn process_data(data: &mut Vec<u8>) {
    // Simple and clear
    data.push(1);
}

// ❌ FORBIDDEN - thread::spawn (doesn't work in WASM)
use std::thread;

fn process_async(data: Vec<u8>) {
    thread::spawn(move || {
        // This will fail in WASM!
        analyze(data);
    });
}

// ✅ CORRECT - Synchronous processing
fn process_sync(data: &[u8]) -> MetadataResult {
    // Direct, synchronous call
    parse_metadata(data)
}

// ❌ FORBIDDEN - async/await (unnecessary overhead in WASM)
async fn analyze_async(data: &[u8]) -> MetadataResult {
    // WASM doesn't benefit from async
    // Just adds complexity and bundle size
    parse_metadata(data)
}

// ✅ CORRECT - Synchronous function
fn analyze(data: &[u8]) -> MetadataResult {
    // Clean, simple, fast
    parse_metadata(data)
}

// ❌ FORBIDDEN - Channels (no concurrent receivers in WASM)
use std::sync::mpsc;

fn use_channels() {
    let (tx, rx) = mpsc::channel();
    // No threads to receive from!
}

// ✅ CORRECT - Direct function calls
fn process_pipeline(data: &[u8]) -> MetadataResult {
    let step1 = parse_metadata(data)?;
    let step2 = enrich_metadata(step1)?;
    Ok(step2)
}
```

**Why no concurrency?**
- WASM is single-threaded - no actual parallelism
- Arc/Mutex add overhead without benefit
- async/await increases bundle size
- Synchronous code is simpler and faster in WASM
- JavaScript handles async at the boundary

**Enforcement:**
- `future_not_send` - denies async futures
- `await_holding_lock` - denies mutex usage with await
- `await_holding_refcell_ref` - denies RefCell issues
- `mutex_atomic` - denies mutex (suggests atomics, but we don't use either!)
- Manual code review for `Arc`, `Mutex`, `thread::`, `async`

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
9. **Generic types (`<T>`)** - Use concrete types; generics complicate WASM and increase bundle size
10. **Concurrency primitives** - No `thread::spawn`, `Arc`, `Mutex`, `async/await`; WASM is single-threaded

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

// ❌ FORBIDDEN - Generic types increase WASM size
fn parse_data<T: Deserialize>(data: &[u8]) -> Result<T, String> {
    serde_json::from_slice(data).map_err(|e| e.to_string())
}

// ✅ CORRECT - Use concrete types
fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String> {
    serde_json::from_slice(data).map_err(|e| e.to_string())
}

// ❌ FORBIDDEN - Concurrency primitives (WASM is single-threaded)
use std::sync::{Arc, Mutex};
use std::thread;

fn process_async(data: Vec<u8>) {
    let shared = Arc::new(Mutex::new(data));
    thread::spawn(move || {
        // This will not work in WASM!
    });
}

// ✅ CORRECT - Synchronous processing
fn process_sync(data: &[u8]) -> Result<HashMap<String, String>, String> {
    parse_metadata(data)
}

// ❌ FORBIDDEN - async/await (not needed in WASM)
async fn analyze_file_async(data: &[u8]) -> Result<String, String> {
    // WASM doesn't benefit from async
    Ok("result".to_string())
}

// ✅ CORRECT - Synchronous functions
fn analyze_file(data: &[u8]) -> Result<String, String> {
    // Direct, synchronous processing
    Ok("result".to_string())
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

---

## 🚀 Backend Development (Optional)

This section contains patterns for server-side Rust API development. These guidelines apply **if/when a backend component is added** to the project.

> [!NOTE]
> Backend code is **exempt** from WASM-specific restrictions (no generics, no concurrency). Backend services can use async/await, Arc, Mutex, and generic types as appropriate.

### Backend Stack & Libraries

**Recommended stack for Rust backend APIs:**

- **Routing/middleware**: `axum`, `tower-http` (CORS, compression, timeouts)
- **JSON & validation**: `serde`, `validator`
- **OpenAPI**: `utoipa`, `utoipa-swagger-ui` (optional docs UI)
- **Observability**: `tracing`, `tracing-subscriber`, `tracing-opentelemetry`
- **DB access**: `sqlx` or `sea-orm`; use query builders for filters/sorts safely
- **IDs & time**: `uuid` (v7) or `ulid`; `time` crate for UTC (`OffsetDateTime`)

### OpenAPI Generation

Use `utoipa` to generate OpenAPI 3.1 documentation automatically:

```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        list_tickets,
        get_ticket,
        create_ticket,
    ),
    components(
        schemas(Ticket, TicketPriority, TicketStatus, Problem)
    ),
    tags(
        (name = "tickets", description = "Ticket management endpoints")
    )
)]
struct ApiDoc;

// Export OpenAPI JSON at /v1/openapi.json
pub fn openapi_json() -> String {
    ApiDoc::openapi().to_json().unwrap()
}
```

**Best practices:**
- Annotate handlers with `#[utoipa::path]`
- Export OpenAPI 3.1 JSON at `/v1/openapi.json`
- Validate in CI with an OpenAPI linter
- Use `utoipa-swagger-ui` for interactive documentation

### Data Types & Serialization

**Use proper serialization with serde and OpenAPI schemas:**

```rust
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid; // enable the v7 feature in Cargo.toml

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub priority: TicketPriority,
    pub status: TicketStatus,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum TicketPriority { 
    Low, 
    Medium, 
    High 
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum TicketStatus { 
    Open, 
    InProgress, 
    Resolved, 
    Closed 
}
```

**Key patterns:**
- Use `#[serde(rename_all = "camelCase")]` for JavaScript compatibility
- Use `time::OffsetDateTime` for timestamps (not `chrono`)
- Use `#[serde(with = "time::serde::rfc3339")]` for RFC3339/ISO-8601 formatting
- Use `utoipa::ToSchema` for OpenAPI schema generation
- Use `uuid` v7 for sortable, time-based IDs

### Error Handling (Problem Details)

**Implement RFC 9457 Problem Details for HTTP APIs:**

```rust
use serde::Serialize;
use utoipa::ToSchema;

/// RFC 9457 Problem Details
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    #[schema(example = "https://api.example.com/errors/validation")]
    pub r#type: String,
    #[schema(example = "Invalid request")]
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(rename = "traceId")]
    pub trace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationError>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ValidationError {
    pub field: String,
    pub code: String,
    pub message: String,
}
```

**Usage in handlers:**

```rust
use axum::{http::StatusCode, response::{IntoResponse, Json}};

impl IntoResponse for Problem {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

// Return Problem from handlers
pub async fn create_ticket(payload: Json<CreateTicketRequest>) -> Result<Json<Ticket>, Problem> {
    if payload.title.is_empty() {
        return Err(Problem {
            r#type: "https://api.example.com/errors/validation".to_string(),
            title: "Validation Error".to_string(),
            status: 422,
            detail: Some("Title cannot be empty".to_string()),
            instance: None,
            trace_id: "01J...".to_string(),
            errors: Some(vec![ValidationError {
                field: "title".to_string(),
                code: "required".to_string(),
                message: "Title is required".to_string(),
            }]),
        });
    }
    // ... create ticket
    Ok(Json(ticket))
}
```

### HTTP Handlers

**axum handler with query parameters and envelope pattern:**

```rust
use axum::{extract::Query, http::HeaderMap, response::{IntoResponse, Json}};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListParams {
    // Capture all query params into a HashMap to handle filters dynamically
    #[serde(flatten)]
    pub filters: HashMap<String, String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Envelope<T> { 
    pub data: T, 
    pub meta: Meta, 
    pub links: Links 
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Meta { 
    pub limit: u16, 
    pub has_next: bool, 
    pub has_prev: bool 
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Links { 
    pub next: Option<String>, 
    pub prev: Option<String> 
}

/// List tickets with cursor pagination
#[utoipa::path(
    get,
    path = "/v1/tickets",
    params(ListParams),
    responses(
        (status = 200, description = "List tickets", body = Envelope<Vec<Ticket>>),
        (status = 422, description = "Validation error", body = Problem)
    ),
    security(("oauth2" = []))
)]
pub async fn list_tickets(Query(params): Query<ListParams>) -> impl IntoResponse {
    // Example of extracting params from the filters map
    let limit = params.filters.get("limit").and_then(|s| s.parse().ok()).unwrap_or(25);
    let after = params.filters.get("after").cloned();
    // status.in=open,in_progress -> ["open", "in_progress"]
    let statuses: Option<Vec<_>> = params.filters.get("status.in").map(|s| s.split(',').collect());

    // ... database logic to fetch tickets based on filters ...
    let tickets: Vec<Ticket> = vec![]; // Placeholder

    let response = Envelope {
        data: tickets,
        meta: Meta { limit, has_next: false, has_prev: false },
        links: Links { next: None, prev: None },
    };

    // Build response with headers
    let mut headers = HeaderMap::new();
    headers.insert("traceId", "01J...".parse().unwrap());
    (headers, Json(response))
}
```

**Key patterns:**
- Use `Query<T>` for query parameters
- Use `Json<T>` for request/response bodies
- Use envelope pattern for list responses (data, meta, links)
- Use cursor pagination (not offset/limit)
- Include trace IDs in headers for observability
- Use `#[utoipa::path]` for OpenAPI documentation

### Idempotency & ETags

**Implement idempotency for write operations:**

```rust
// Persist idempotency keys in database
// Table: idempotency_keys (key, request_fingerprint, response_hash, expires_at)

pub async fn create_ticket_idempotent(
    headers: HeaderMap,
    Json(payload): Json<CreateTicketRequest>,
) -> Result<(HeaderMap, Json<Ticket>), Problem> {
    let idempotency_key = headers
        .get("idempotency-key")
        .and_then(|v| v.to_str().ok());
    
    if let Some(key) = idempotency_key {
        // Check if request with this key was already processed
        if let Some(cached_response) = check_idempotency_cache(key).await {
            let mut response_headers = HeaderMap::new();
            response_headers.insert("idempotency-replayed", "true".parse().unwrap());
            return Ok((response_headers, Json(cached_response)));
        }
    }
    
    // Process request normally
    let ticket = create_ticket_in_db(payload).await?;
    
    // Compute and return ETag for concurrency control
    let etag = compute_etag(&ticket);
    let mut response_headers = HeaderMap::new();
    response_headers.insert("etag", etag.parse().unwrap());
    
    // Cache response for idempotency
    if let Some(key) = idempotency_key {
        cache_idempotent_response(key, &ticket).await;
    }
    
    Ok((response_headers, Json(ticket)))
}

// Clients send If-Match header for updates
pub async fn update_ticket(
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTicketRequest>,
) -> Result<Json<Ticket>, Problem> {
    let if_match = headers.get("if-match").and_then(|v| v.to_str().ok());
    
    let current_ticket = get_ticket_by_id(id).await?;
    let current_etag = compute_etag(&current_ticket);
    
    if let Some(client_etag) = if_match {
        if client_etag != current_etag {
            return Err(Problem {
                r#type: "https://api.example.com/errors/conflict".to_string(),
                title: "Conflict".to_string(),
                status: 409,
                detail: Some("Resource was modified by another request".to_string()),
                instance: Some(format!("/v1/tickets/{}", id)),
                trace_id: "01J...".to_string(),
                errors: None,
            });
        }
    }
    
    // Update ticket
    let updated_ticket = update_ticket_in_db(id, payload).await?;
    Ok(Json(updated_ticket))
}
```

**Best practices:**
- Persist `(idempotency_key, request_fingerprint, response_hash, expires_at)`
- On replay with same fingerprint: return stored response + `Idempotency-Replayed: true`
- For writes, compute and return `ETag`; clients send `If-Match` for concurrency
- Use 409 Conflict for ETag mismatches

### Timestamp Handling

**Use `time::OffsetDateTime` for all timestamps:**

```rust
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    // Required timestamp - RFC3339 format
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    
    // Optional timestamp - RFC3339 format
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<OffsetDateTime>,
}

// Get current time in UTC
let now = OffsetDateTime::now_utc();

// Parse from RFC3339 string
let timestamp = OffsetDateTime::parse("2025-12-15T11:40:25Z", &time::format_description::well_known::Rfc3339)?;
```

**Rules:**
- Use `time::OffsetDateTime` (not `chrono::DateTime`)
- Always use UTC for storage and APIs
- Use `#[serde(with = "time::serde::rfc3339")]` for required fields
- Use `#[serde(with = "time::serde::rfc3339::option")]` for optional fields
- Format as RFC3339/ISO-8601 with milliseconds

---

## 🎯 Universal Best Practices

These practices apply to **both WASM and backend code**.

## 📋 Clippy Configuration

**AI agents MUST follow all lints and thresholds defined in:**
- `Cargo.toml` - `[lints.clippy]` section (lint levels: deny/warn)
- `clippy.toml` - Threshold values for complexity and size limits

Run before committing:

```bash
cargo clippy --all-targets --all-features -- -D warnings
# Or use npm script:
npm run lint:rust
```

### Lints from Cargo.toml [lints.clippy]

**Denied lints (critical issues):**
- `unwrap_used` = "deny" - Never use `.unwrap()`, use proper error handling
- `expect_used` = "deny" - Never use `.expect()`, use proper error handling
- `panic` = "deny" - No explicit `panic!()` calls in library code
- `todo` = "deny" - Complete all code before committing
- `unimplemented` = "deny" - Implement all functions

**Warned lints (performance and style):**
- `inefficient_to_string` = "warn" - Use efficient string conversions
- `clone_on_ref_ptr` = "warn" - Avoid unnecessary clones on Arc/Rc
- `needless_return` = "warn" - Remove redundant return statements
- `redundant_closure` = "warn" - Simplify closures where possible
- `explicit_auto_deref` = "warn" - Remove unnecessary explicit derefs
- `manual_range_contains` = "warn" - Use `.contains()` for range checks

**Denied lints (concurrency - WASM is single-threaded):**
- `future_not_send` = "deny" - No async futures (not needed in WASM)
- `await_holding_lock` = "deny" - No mutex usage with await
- `await_holding_refcell_ref` = "deny" - No RefCell issues with await
- `mutex_atomic` = "deny" - No mutex (WASM is single-threaded)

**Denied lints (complexity - catches generics):**
- `type_complexity` = "deny" - Deny complex types (use type aliases)

### Thresholds from clippy.toml

**Complexity thresholds:**
- `cognitive-complexity-threshold = 30` - Maximum cognitive complexity per function
- `type-complexity-threshold = 100` - **Very low to discourage generics** - use type aliases

**Size thresholds (WASM optimization):**
- `pass-by-value-size-limit = 256` - Max bytes for pass-by-value (use references for larger)
- `too-large-for-stack = 512` - Max bytes for stack allocation
- `trivial-copy-size-limit = 128` - Max bytes for trivial Copy types

**Other thresholds:**
- `too-many-arguments-threshold = 8` - Max function parameters
- `too-many-lines-threshold = 150` - Max lines per function

### Project-specific restrictions

**❌ FORBIDDEN:**
1. **Generic types (`<T>`)** - Use concrete types for WASM optimization
   - Generics increase bundle size
   - Caught by `type_complexity` lint (threshold = 100)
   - Use type aliases for complex concrete types

2. **Concurrency primitives** - WASM is single-threaded
   - No `Arc`, `Mutex`, `RwLock`
   - No `thread::spawn`, `thread::*`
   - No `async/await`, `Future`, `async fn`
   - No `mpsc::channel` or other channels
   - Caught by `future_not_send`, `await_holding_lock`, `mutex_atomic` lints

**✅ REQUIRED:**
- Follow ALL thresholds in `clippy.toml`
- Respect ALL lint levels in `Cargo.toml [lints.clippy]`
- Use type aliases when approaching `type-complexity-threshold`
- Keep functions under `too-many-lines-threshold`

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
8. 🚫 No generic types - use concrete types
9. 🚫 No concurrency primitives - WASM is single-threaded
10. 📝 Document public APIs
11. 🚀 Optimize for WASM bundle size
12. 🔍 Run clippy before committing
