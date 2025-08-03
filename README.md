# Graphrite CLI

Graphrite is a deterministic, AI-friendly diagram syntax and tooling.

Install
- Requires Rust toolchain
- Build: cargo build

Usage
- parse|ast: Print JSON AST (stderr diagnostics or --diag-json)
  cargo run -p graphrite-cli -- parse <file>
  echo "..." | cargo run -p graphrite-cli -- ast
- check: Validate diagram; prints "ok" or diagnostics
  cargo run -p graphrite-cli -- check <file>
- lint: Same validations; human, pretty, or JSON
  cargo run -p graphrite-cli -- lint [--pretty|--diag-json] <file>
- fmt: Format (identity for now); --check for CI
  cargo run -p graphrite-cli -- fmt [--check|--write] <file>
- diag: Pretty diagnostics only
  cargo run -p graphrite-cli -- diag [--json] <file>
- render: Render DOT or SVG (built-in SVG)
  cargo run -p graphrite-cli -- render --format dot <file>
  cargo run -p graphrite-cli -- render --format svg <file>

Examples
- Valid: samples/valid/*.mmd
- Invalid: samples/invalid/*.mmd (differences vs Mermaid)
- Rendered SVGs: docs/examples/*.svg

Exit codes
- 0: success
- 1: validation errors

Docs
- GETTING_STARTED.md, SPEC.md, ERROR_CODES.md, PLAN.md, FUTURE_STEPS.md
