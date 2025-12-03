# AI Agent Rules and Guidelines

This project uses AI agent guidelines to maintain code quality and consistency.

## Quick Reference

- **Rust Guidelines**: See `.agent/workflows/rust-guidelines.md` or use `/rust-guidelines` command
- **Linter**: `npm run lint:rust` to check Rust code
- **Auto-fix**: `npm run lint:rust:fix` to automatically fix issues

## For AI Agents

When working on this project:
1. ✅ Read `.agent/workflows/rust-guidelines.md` before modifying Rust code
2. ✅ Follow all rules marked with `ai-rules: true` in YAML frontmatter
3. ✅ Run clippy before committing: `npm run lint:rust`
4. 🚫 Never use `unwrap()`, `expect()`, or `panic!()` in production code
5. 🚫 Avoid unnecessary allocations and string conversions

## For Developers

The `.agent` directory contains:
- Coding standards enforced by linters
- Best practices for this specific project
- WASM optimization guidelines
- Language-specific rules

These guidelines ensure AI-generated code meets project standards.
