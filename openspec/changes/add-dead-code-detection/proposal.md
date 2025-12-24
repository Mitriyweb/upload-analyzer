# Change: Add Dead Code Detection Tools

## Why
The project currently lacks automated tools to detect unused code, dependencies, and exports in both the TypeScript/JavaScript frontend and Rust backend. Dead code increases bundle size, maintenance burden, and can hide potential bugs. Adding automated detection tools will improve code quality, reduce WASM binary size (critical for browser performance), and ensure the codebase remains lean and maintainable.

## What Changes
- Add **Knip** for TypeScript/JavaScript dead code detection (unused files, exports, dependencies, and types)
- Add **cargo-machete** for Rust unused dependency detection
- Configure both tools with project-specific settings
- Integrate tools into existing lint workflow (`npm run lint`)
- Add pre-commit hooks to catch dead code before commits
- Document usage and configuration in project documentation

## Impact
- **Affected specs**: `code-quality` (new capability)
- **Affected code**:
  - `package.json` - Add Knip as dev dependency, update lint scripts
  - `.pre-commit-config.yaml` - Add Knip and cargo-machete hooks
  - `knip.json` - Knip configuration file (new)
  - `.github/workflows/` - CI integration (if exists)
  - `openspec/project.md` - Update testing strategy section
- **Performance impact**: Potential WASM binary size reduction through unused code removal
- **Breaking changes**: None (tooling only)
