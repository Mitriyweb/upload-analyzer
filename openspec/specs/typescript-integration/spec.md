# Capability: TypeScript Integration

## Overview
Provides TypeScript type definitions, type guards, and helper functions for type-safe usage of the WASM analyzer in TypeScript projects.

## Requirements

### Requirement: Type Definitions
The system SHALL provide comprehensive TypeScript type definitions for all analysis results.

#### Scenario: PEAnalysis type
- **WHEN** TypeScript project imports types
- **THEN** provide complete PEAnalysis interface with all optional and required fields

#### Scenario: MSIAnalysis type
- **WHEN** TypeScript project imports types
- **THEN** provide complete MSIAnalysis interface

#### Scenario: DMGAnalysis type
- **WHEN** TypeScript project imports types
- **THEN** provide complete DMGAnalysis interface

#### Scenario: Union type for results
- **WHEN** handling analysis results
- **THEN** provide FileAnalysis union type covering all possible result types

### Requirement: Type Guards
The system SHALL provide runtime type guard functions for result discrimination.

#### Scenario: PE result detection
- **WHEN** checking if result is PEAnalysis
- **THEN** `isPEAnalysis()` function returns true for PE results

#### Scenario: MSI result detection
- **WHEN** checking if result is MSIAnalysis
- **THEN** `isMSIAnalysis()` function returns true for MSI results

#### Scenario: DMG result detection
- **WHEN** checking if result is DMGAnalysis
- **THEN** `isDMGAnalysis()` function returns true for DMG results

#### Scenario: Error detection
- **WHEN** checking if result is error
- **THEN** `isAnalysisError()` function returns true for error objects

### Requirement: Parser Functions
The system SHALL provide helper functions for parsing JSON results.

#### Scenario: Safe JSON parsing
- **WHEN** parsing WASM function results
- **THEN** provide `parseAnalysisResult()` with type inference

#### Scenario: Parse errors
- **WHEN** JSON parsing fails
- **THEN** return typed error object instead of throwing

### Requirement: IntelliSense Support
The system SHALL enable IDE autocomplete and type checking.

#### Scenario: Field autocomplete
- **WHEN** accessing analysis result fields
- **THEN** IDE shows available fields with documentation

#### Scenario: Type narrowing
- **WHEN** using type guard functions
- **THEN** TypeScript narrows type for subsequent code

### Requirement: Module Exports
The system SHALL provide proper module exports for different import styles.

#### Scenario: Named imports
- **WHEN** using `import { analyze_pe_file } from 'upload-analyzer'`
- **THEN** provide named exports for all functions

#### Scenario: Type-only imports
- **WHEN** using `import type { PEAnalysis } from 'upload-analyzer/types'`
- **THEN** provide separate type-only exports

#### Scenario: Helper imports
- **WHEN** using `import { isPEAnalysis } from 'upload-analyzer/helpers'`
- **THEN** provide helper function exports

### Requirement: Documentation Comments
The system SHALL include JSDoc comments for all exported types and functions.

#### Scenario: Type documentation
- **WHEN** hovering over types in IDE
- **THEN** show comprehensive JSDoc descriptions

#### Scenario: Function documentation
- **WHEN** hovering over functions in IDE
- **THEN** show parameter descriptions and return types

### Requirement: Backwards Compatibility
The system SHALL maintain type compatibility across minor versions.

#### Scenario: Optional field additions
- **WHEN** new optional fields are added to analysis types
- **THEN** existing TypeScript code continues to compile

#### Scenario: Required field changes
- **WHEN** required fields need to change
- **THEN** increment major version and provide migration guide

## Data Structures

### Type Guard Signatures
```typescript
function isPEAnalysis(result: FileAnalysis): result is PEAnalysis;
function isMSIAnalysis(result: FileAnalysis): result is MSIAnalysis;
function isDMGAnalysis(result: FileAnalysis): result is DMGAnalysis;
function isAnalysisError(result: FileAnalysis | AnalysisError): result is AnalysisError;
```

### Parser Function Signature
```typescript
function parseAnalysisResult(jsonString: string): FileAnalysis | AnalysisError;
```

## API

### Module: `upload-analyzer/types`
Exports all TypeScript type definitions.

**Exports:**
- `FileAnalysis` - Union type of all analysis results
- `PEAnalysis` - PE file analysis result type
- `MSIAnalysis` - MSI file analysis result type
- `DMGAnalysis` - DMG file analysis result type
- `AnalysisError` - Error result type
- `FileInfo` - Basic file info type

### Module: `upload-analyzer/helpers`
Exports type guard and helper functions.

**Exports:**
- `isPEAnalysis(result)` - Type guard for PE results
- `isMSIAnalysis(result)` - Type guard for MSI results
- `isDMGAnalysis(result)` - Type guard for DMG results
- `isAnalysisError(result)` - Type guard for errors
- `parseAnalysisResult(json)` - Safe JSON parser

## Build Integration

### Requirement: TypeScript Compilation
The system SHALL compile TypeScript helpers to JavaScript.

#### Scenario: Build process
- **WHEN** running `npm run build:types`
- **THEN** compile `src/ts/helpers.ts` to `dist/helpers.js` with type definitions

#### Scenario: Type definition generation
- **WHEN** building package
- **THEN** generate `.d.ts` files for all TypeScript sources

### Requirement: Package Exports
The system SHALL configure package.json exports for proper module resolution.

#### Scenario: Main module
- **WHEN** importing from 'upload-analyzer'
- **THEN** resolve to WASM module with types

#### Scenario: Helpers subpath
- **WHEN** importing from 'upload-analyzer/helpers'
- **THEN** resolve to compiled helpers with types

#### Scenario: Types subpath
- **WHEN** importing from 'upload-analyzer/types'
- **THEN** resolve to type definitions only
