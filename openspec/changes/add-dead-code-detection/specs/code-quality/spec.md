# Code Quality Specification

## ADDED Requirements

### Requirement: TypeScript/JavaScript Dead Code Detection
The project SHALL use Knip to automatically detect unused code, exports, dependencies, and types in TypeScript and JavaScript files.

#### Scenario: Detect unused exports
- **GIVEN** a TypeScript file with an exported function that is never imported
- **WHEN** Knip is run via `npm run lint:knip`
- **THEN** Knip SHALL report the unused export with file path and line number

#### Scenario: Detect unused dependencies
- **GIVEN** a package listed in `package.json` dependencies that is never imported
- **WHEN** Knip is run via `npm run lint:knip`
- **THEN** Knip SHALL report the unused dependency

#### Scenario: Detect unused files
- **GIVEN** a TypeScript file that is not imported by any other file or entry point
- **WHEN** Knip is run via `npm run lint:knip`
- **THEN** Knip SHALL report the unused file

#### Scenario: Respect configured entry points
- **GIVEN** WASM bindings in `pkg/upload_analyzer.js` are configured as entry points
- **WHEN** Knip analyzes the codebase
- **THEN** Knip SHALL NOT report WASM-exported functions as unused

#### Scenario: Ignore generated files
- **GIVEN** generated files in `pkg/`, `dist/`, and `public/` directories
- **WHEN** Knip analyzes the codebase
- **THEN** Knip SHALL NOT analyze or report issues in ignored directories

### Requirement: Rust Dependency Detection
The project SHALL use cargo-machete to automatically detect unused Rust dependencies (crates) in `Cargo.toml`.

#### Scenario: Detect unused crate dependencies
- **GIVEN** a crate listed in `Cargo.toml` dependencies that is never used in Rust code
- **WHEN** cargo-machete is run via `npm run lint:rust:deps`
- **THEN** cargo-machete SHALL report the unused dependency with crate name

#### Scenario: Handle WASM-specific dependencies
- **GIVEN** `wasm-bindgen` and `web-sys` crates that are used for WASM compilation
- **WHEN** cargo-machete analyzes dependencies
- **THEN** cargo-machete SHALL NOT report these as unused (they are used)

### Requirement: Lint Workflow Integration
The dead code detection tools SHALL be integrated into the existing lint workflow for consistent developer experience.

#### Scenario: Run all lints together
- **GIVEN** the command `npm run lint` is executed
- **WHEN** the lint script runs
- **THEN** it SHALL execute Rust clippy, TypeScript ESLint, Knip, and cargo-machete in sequence

#### Scenario: Individual tool execution
- **GIVEN** a developer wants to run only Knip
- **WHEN** the command `npm run lint:knip` is executed
- **THEN** only Knip SHALL run and report results

#### Scenario: Individual Rust dependency check
- **GIVEN** a developer wants to check only Rust dependencies
- **WHEN** the command `npm run lint:rust:deps` is executed
- **THEN** only cargo-machete SHALL run and report results

### Requirement: Pre-commit Hook Integration
Dead code detection tools SHALL run automatically before commits to catch issues early.

#### Scenario: Knip runs on TypeScript changes
- **GIVEN** a developer commits changes to TypeScript files
- **WHEN** the pre-commit hook executes
- **THEN** Knip SHALL run and block the commit if dead code is detected

#### Scenario: cargo-machete runs on Cargo.toml changes
- **GIVEN** a developer commits changes to `Cargo.toml`
- **WHEN** the pre-commit hook executes
- **THEN** cargo-machete SHALL run and block the commit if unused dependencies are detected

#### Scenario: Skip hooks when needed
- **GIVEN** a developer needs to commit work-in-progress code
- **WHEN** the commit is made with `--no-verify` flag
- **THEN** pre-commit hooks SHALL be bypassed

### Requirement: Configuration Management
The dead code detection tools SHALL be configurable to handle project-specific needs and reduce false positives.

#### Scenario: Knip configuration file
- **GIVEN** a `knip.json` file exists in the project root
- **WHEN** Knip runs
- **THEN** it SHALL use the configuration for entry points, ignore patterns, and dependency exceptions

#### Scenario: Update entry points
- **GIVEN** a new entry point is added to the project (e.g., new WASM export)
- **WHEN** the entry point is added to `knip.json`
- **THEN** Knip SHALL recognize code reachable from that entry point as used

#### Scenario: Ignore false positives
- **GIVEN** a dependency that is used indirectly (e.g., peer dependency)
- **WHEN** the dependency is added to `ignoreDependencies` in `knip.json`
- **THEN** Knip SHALL NOT report it as unused

### Requirement: Documentation and Developer Guidance
The project SHALL provide clear documentation on using dead code detection tools and handling their output.

#### Scenario: Tool usage documented
- **GIVEN** a developer reads the README
- **WHEN** they look for lint information
- **THEN** they SHALL find instructions for running Knip and cargo-machete

#### Scenario: False positive handling documented
- **GIVEN** Knip reports a false positive
- **WHEN** the developer consults documentation
- **THEN** they SHALL find instructions on how to configure exceptions

#### Scenario: Installation instructions for cargo-machete
- **GIVEN** a new developer sets up the project
- **WHEN** they read the setup documentation
- **THEN** they SHALL find instructions to install cargo-machete globally or in CI
