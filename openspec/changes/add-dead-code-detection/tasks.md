# Implementation Tasks

## 1. TypeScript/JavaScript Dead Code Detection (Knip)
- [x] 1.1 Install Knip as dev dependency (`npm install -D knip`)
- [x] 1.2 Create `knip.json` configuration file with project-specific settings
- [x] 1.3 Configure entry points (WASM bindings, TypeScript helpers)
- [x] 1.4 Configure ignore patterns for generated files (`pkg/`, `dist/`)
- [x] 1.5 Add `lint:knip` script to `package.json`
- [x] 1.6 Update main `lint` script to include Knip
- [x] 1.7 Run Knip and document initial findings

## 2. Rust Unused Dependency Detection (cargo-machete)
- [x] 2.1 Document cargo-machete installation in README (global tool)
- [x] 2.2 Add `lint:rust:deps` script to `package.json` for cargo-machete
- [x] 2.3 Update main `lint` script to include cargo-machete
- [x] 2.4 Run cargo-machete and document initial findings

## 3. Pre-commit Integration
- [x] 3.1 Add Knip hook to `.pre-commit-config.yaml`
- [x] 3.2 Add cargo-machete hook to `.pre-commit-config.yaml`
- [x] 3.3 Test pre-commit hooks locally
- [x] 3.4 Update prek configuration if needed

## 4. Documentation
- [x] 4.1 Update `openspec/project.md` testing strategy section
- [x] 4.2 Add usage instructions to README
- [x] 4.3 Document how to handle false positives
- [x] 4.4 Add troubleshooting guide for common issues

## 5. Validation
- [x] 5.1 Run full lint suite (`npm run lint`)
- [x] 5.2 Verify pre-commit hooks work correctly
- [x] 5.3 Test on clean repository clone
- [x] 5.4 Validate OpenSpec change with `openspec validate add-dead-code-detection --strict`
