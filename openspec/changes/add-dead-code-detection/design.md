# Design: Dead Code Detection Tools

## Context
The upload-analyzer project is a hybrid TypeScript/Rust codebase compiled to WebAssembly. The project has strict performance requirements (WASM binary < 250 KB) and follows a spec-driven development workflow with OpenSpec. Currently, there's no automated way to detect:
- Unused TypeScript/JavaScript files, exports, or dependencies
- Unused Rust dependencies (crates)
- Dead code that increases bundle size

## Goals / Non-Goals

**Goals:**
- Automate detection of unused code and dependencies in both TypeScript and Rust
- Integrate tools into existing lint workflow with minimal friction
- Catch dead code early via pre-commit hooks
- Provide clear, actionable feedback to developers
- Support the WASM size optimization constraint

**Non-Goals:**
- Automatic removal of dead code (manual review required)
- Detection of unused Rust code within modules (clippy handles this)
- Runtime dead code analysis
- Code coverage analysis (different concern)

## Decisions

### Decision 1: Knip for TypeScript/JavaScript
**What:** Use Knip as the dead code detection tool for TypeScript/JavaScript.

**Why:**
- Comprehensive detection: unused files, exports, dependencies, types, and enum members
- Excellent TypeScript support with type-aware analysis
- Configurable entry points (critical for WASM bindings)
- Active maintenance and good documentation
- Works well with ES modules (project uses ES6 modules)

**Alternatives considered:**
- **ts-prune**: Only detects unused exports, not dependencies or files
- **depcheck**: Only checks dependencies, not code usage
- **unimported**: Less TypeScript-aware, weaker type detection

### Decision 2: cargo-machete for Rust
**What:** Use cargo-machete to detect unused Rust dependencies.

**Why:**
- Fast and accurate unused dependency detection
- Simple CLI with no configuration needed
- Works with workspaces and multi-crate projects
- Actively maintained
- Complements clippy (which detects unused code, not dependencies)

**Alternatives considered:**
- **cargo-udeps**: Requires nightly Rust, more complex setup
- **cargo-geiger**: Focused on unsafe code, not dead dependencies
- Manual `cargo tree` analysis: Not automated, error-prone

### Decision 3: Integration Strategy
**What:** Integrate both tools into the existing `npm run lint` workflow and pre-commit hooks.

**Why:**
- Consistent developer experience (single command for all lints)
- Early detection via pre-commit prevents dead code from entering repository
- Aligns with existing workflow (prek for pre-commit, npm scripts for CI)
- No additional CI configuration needed

**Configuration approach:**
- Knip: `knip.json` for project-specific settings (entry points, ignore patterns)
- cargo-machete: No config needed, run via npm script for consistency
- Pre-commit: Add both tools to `.pre-commit-config.yaml`

## Risks / Trade-offs

### Risk 1: False Positives
**Risk:** Tools may flag code as unused when it's actually needed (e.g., WASM exports, dynamic imports).

**Mitigation:**
- Configure Knip entry points to include all WASM bindings (`pkg/upload_analyzer.js`)
- Add ignore patterns for generated files (`pkg/`, `dist/`)
- Document how to handle false positives in README
- Allow developers to add exceptions via configuration

### Risk 2: Build Time Impact
**Risk:** Adding more lint tools increases build/CI time.

**Mitigation:**
- Knip is fast (< 1 second for small projects)
- cargo-machete is very fast (< 1 second)
- Run in parallel with other lints where possible
- Only run on changed files in pre-commit (if supported)

### Risk 3: Developer Friction
**Risk:** Developers may find additional lint failures annoying, especially for existing code.

**Mitigation:**
- Run initial analysis and clean up before enforcing
- Provide clear error messages and fix instructions
- Allow configuration to reduce noise
- Make pre-commit hooks optional initially (can be bypassed with `--no-verify`)

## Configuration Details

### Knip Configuration (`knip.json`)
```json
{
  "entry": [
    "src/ts/helpers.ts",
    "pkg/upload_analyzer.js"
  ],
  "project": [
    "src/ts/**/*.ts",
    "scripts/**/*.js"
  ],
  "ignore": [
    "pkg/**",
    "dist/**",
    "public/**"
  ],
  "ignoreDependencies": [
    "wasm-pack",
    "http-server"
  ]
}
```

### Pre-commit Hook Configuration
```yaml
- repo: local
  hooks:
    - id: knip
      name: Knip (dead code detection)
      entry: npx knip
      language: node
      pass_filenames: false
      files: \.(ts|js)$

    - id: cargo-machete
      name: cargo-machete (unused dependencies)
      entry: cargo machete
      language: system
      pass_filenames: false
      files: Cargo\.toml$
```

## Migration Plan

### Phase 1: Installation and Configuration
1. Install Knip: `npm install -D knip`
2. Create `knip.json` with initial configuration
3. Document cargo-machete installation (global tool)

### Phase 2: Initial Analysis
1. Run Knip: `npx knip`
2. Run cargo-machete: `cargo machete`
3. Document findings and create cleanup tasks (separate from this change)

### Phase 3: Integration
1. Add npm scripts: `lint:knip`, `lint:rust:deps`
2. Update main `lint` script
3. Add pre-commit hooks
4. Test locally

### Phase 4: Documentation and Rollout
1. Update `openspec/project.md`
2. Add README section
3. Announce to team
4. Monitor for issues

### Rollback Plan
If tools cause significant issues:
1. Remove from `package.json` scripts
2. Remove pre-commit hooks
3. Uninstall Knip: `npm uninstall knip`
4. Document decision and reasons

## Open Questions

1. **Should we enforce zero dead code in CI?**
   - Initial answer: No, start with warnings only. Enforce after initial cleanup.

2. **Should cargo-machete be a required global tool or installed per-project?**
   - Initial answer: Document as global tool (simpler), but allow `cargo install cargo-machete` in CI.

3. **How should we handle generated WASM bindings that Knip might flag?**
   - Initial answer: Add `pkg/` to ignore patterns and configure entry points correctly.
