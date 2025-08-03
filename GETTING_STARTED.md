# Getting Started with Graphrite

Install
- Rust toolchain: cargo build
- Or download a prebuilt binary from Releases (when available) and put it on PATH

Quick start
1) Create a file sample.mmd
   %% Diagram: Hello
   direction LR
   a["Start"]
   b["End"]
   a --> b

2) Validate
   graphrite check sample.mmd

3) Print AST
   graphrite parse sample.mmd

4) Lint (human or JSON)
   graphrite lint --pretty sample.mmd
   graphrite lint --diag-json sample.mmd

5) Format (identity for now)
   graphrite fmt --check sample.mmd

6) Render to SVG (built-in)
   graphrite render --format svg sample.mmd > sample.svg

Samples
- Browse samples/valid/*.mmd and samples/invalid/*.mmd
