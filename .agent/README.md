# AI Agent Configuration Directory

This directory contains configuration files and guidelines for AI agents working on this project.

## Quick Links

- [AI Rules](file:///Users/dmytro.zvieriev/sandbox/upload-analyzer/.agent/AI_RULES.md)
- [Rust Guidelines](file:///Users/dmytro.zvieriev/sandbox/upload-analyzer/.agent/workflows/rust-guidelines.md)

## Purpose

The `.agent` directory is a standard location for AI-specific project configuration:
- **Workflows**: Reusable guidelines and procedures for AI agents
- **Rules**: Language-specific coding standards and best practices
- **Automation**: Scripts and configurations for AI-assisted development

## Contents

### `/workflows/`

Contains workflow files (`.md` format) that define procedures and guidelines for AI agents:

- **`rust-guidelines.md`** - Mandatory Rust coding standards for AI agents
  - Enforced by clippy linter
  - Defines best practices and forbidden constructs
  - WASM-specific optimization rules

## File Format

Workflow files use YAML frontmatter for metadata:

```yaml
---
description: Human-readable description
ai-rules: true                    # Indicates AI agent rules
ai-agent-guidelines: <language>   # Language these rules apply to
language: <language>               # Programming language
scope: project                     # Scope of rules (project/global)
enforcement: <tool>                # Tool that enforces rules (clippy/eslint/etc)
version: <version>                 # Version of the guidelines
---
```

## IDE Integration

Modern IDEs and AI coding assistants recognize the `.agent` directory:
- **GitHub Copilot**: Uses workflows for context
- **Cursor**: Reads guidelines for code generation
- **Codeium**: Applies rules during suggestions
- **Custom AI Agents**: Follow defined workflows

## Usage

### For AI Agents

AI agents should:
1. Read workflow files before generating code
2. Follow all rules marked with `ai-rules: true`
3. Respect enforcement tools (clippy, eslint, etc.)
4. Apply language-specific guidelines

### For Developers

Developers can:
1. Add new workflow files for different languages
2. Update existing guidelines as project evolves
3. Reference workflows in code reviews
4. Use workflows to train team members

## Adding New Workflows

To add a new workflow:

1. Create a new `.md` file in `/workflows/`
2. Add YAML frontmatter with metadata
3. Include clear, actionable guidelines
4. Reference enforcement tools if applicable

Example:

```markdown
---
description: TypeScript coding guidelines
ai-rules: true
ai-agent-guidelines: typescript
language: typescript
scope: project
enforcement: eslint
version: 1.0
---

# TypeScript Guidelines

...
```

## Enforcement

Rules in this directory are enforced by:
- **Rust**: `clippy` (configured in `Cargo.toml` and `clippy.toml`)
- **TypeScript**: `eslint` (if configured)
- **General**: Code review and AI agent compliance

## Version Control

This directory should be committed to version control to ensure:
- Consistent AI behavior across team members
- Historical tracking of guideline changes
- Shared understanding of project standards

---

**Note**: This is a standard directory structure recognized by AI development tools. Do not rename or move this directory without updating tool configurations.
