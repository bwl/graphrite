# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Graphrite is a deterministic, AI-friendly diagram syntax and CLI tooling written in Rust. The project implements a strict subset of Mermaid diagrams focused on flowcharts with deterministic parsing, validation, and rendering capabilities.

## Architecture

### Workspace Structure
- **graphrite-core**: Core library containing the lexer, parser, AST definitions, and error handling
- **graphrite-cli**: Command-line interface that wraps the core functionality

### Core Components
- **Lexer** (`crates/graphrite-core/src/lexer.rs`): Tokenizes Graphrite syntax
- **Parser** (`crates/graphrite-core/src/parser.rs`): Builds AST from tokens with comprehensive validation
- **AST** (`crates/graphrite-core/src/ast.rs`): Type-safe representation of diagrams with Position/Span tracking
- **Error** (`crates/graphrite-core/src/error.rs`): Structured diagnostic system with error codes

### Key Features
- Snake_case identifier validation
- Orphan node detection
- Line length enforcement (100 chars)
- Direction validation (LR/TD)
- Edge validation (no dangling references)
- Built-in SVG rendering with automatic layout

## Common Development Commands

### Building and Testing
```bash
# Build the entire workspace
cargo build

# Build specific crate
cargo build -p graphrite-core
cargo build -p graphrite-cli

# Run all tests
cargo test

# Run tests for specific crate
cargo test -p graphrite-core

# Run specific test file
cargo test -p graphrite-core --test basic
cargo test -p graphrite-core --test validation
```

### CLI Usage for Development
```bash
# Parse and validate a diagram
cargo run -p graphrite-cli -- check samples/valid/simple_flow.mmd

# Get JSON AST output
cargo run -p graphrite-cli -- parse samples/valid/simple_flow.mmd

# Lint with pretty output
cargo run -p graphrite-cli -- lint --pretty samples/valid/simple_flow.mmd

# Lint with JSON diagnostics
cargo run -p graphrite-cli -- lint --diag-json samples/valid/simple_flow.mmd

# Render to DOT format
cargo run -p graphrite-cli -- render --format dot samples/valid/simple_flow.mmd

# Render to SVG
cargo run -p graphrite-cli -- render --format svg samples/valid/simple_flow.mmd

# Format check (identity for now)
cargo run -p graphrite-cli -- fmt --check samples/valid/simple_flow.mmd

# Test with stdin
echo "direction LR\na[\"Start\"]\nb[\"End\"]\na --> b" | cargo run -p graphrite-cli -- ast
```

### Testing with Sample Files
- **Valid samples**: `samples/valid/*.mmd` - Demonstrates correct syntax
- **Invalid samples**: `samples/invalid/*.mmd` - Shows various error conditions
- **Golden tests**: Tests compare output against expected results in `target/`

## Graphrite Syntax Rules

### Required Structure
1. Optional comment header: `%% Diagram: <title>`
2. Direction declaration: `direction LR` or `direction TD` (must be first non-comment line)
3. Node declarations: `id["Label"]` (snake_case ids, quoted labels)
4. Edge declarations: `src --> dst` (flow) or `src -.-> dst` (conditional)

### Validation Rules
- Identifiers: snake_case only (`[a-z][a-z0-9_]*`, no trailing underscore)
- No orphan nodes (nodes with zero edges)
- No dangling edges (edges referencing undefined nodes)
- Max line length: 100 characters
- Single diagram per file

## Error Codes
The system uses structured error codes (see ERROR_CODES.md):
- E0001: Missing direction
- E0003: Unquoted labels
- E0100: Invalid identifiers
- E0201/E0202: Unknown edge references
- E0203: Orphan nodes
- E0300: Line length violations

## Development Patterns

### Adding New Validation Rules
1. Add error code to `error.rs`
2. Implement validation logic in `parser.rs`
3. Add test cases in `tests/validation.rs`
4. Update ERROR_CODES.md

### Parser Extension
- The parser uses recursive descent with token lookahead
- All spans are tracked for precise error reporting
- Validation happens during parsing, not as separate pass

### AST Serialization
- All AST nodes implement Serde for JSON output
- Use `#[serde(rename_all = "lowercase")]` for enums
- Maintain backward compatibility for JSON schema

## Testing Strategy
- Unit tests for each parser component
- Golden file tests for CLI output consistency
- Sample-based integration tests
- Validation rule coverage in dedicated test files