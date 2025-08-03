# Getting Started with Graphrite

Install
- Ensure Rust toolchain installed
- Build: cargo build

Quick start
1) Create a file sample.mmd
   %% Diagram: Hello
   direction LR
   a["Start"]
   b["End"]
   a --> b

2) Validate
   cargo run -p graphrite-cli -- check sample.mmd

3) Print AST
   cargo run -p graphrite-cli -- parse sample.mmd

4) Lint (human or JSON)
   cargo run -p graphrite-cli -- lint --pretty sample.mmd
   cargo run -p graphrite-cli -- lint --diag-json sample.mmd

5) Format (identity for now)
   cargo run -p graphrite-cli -- fmt --check sample.mmd

6) Render to DOT and SVG
   cargo run -p graphrite-cli -- render --format dot sample.mmd > sample.dot
   cat sample.dot | dot -Tsvg > sample.svg

Samples
- Browse samples/valid/*.mmd
