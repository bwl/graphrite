Graphrite: Plan of Action

Decision points
- Fork-first strategy: Build Graphrite as a standalone ecosystem. Maintain optional compatibility adapters for Mermaid inputs if needed.
- Implementation language: Rust selected for core (speed, WASM). TS bridge planned for Node/web.
- Language form: Define the Graphrite dialect; spec is authoritative and may diverge from Mermaid where clarity/perf require.

Milestones
1) Spec and prototype (Week 1) – COMPLETE
- SPEC.md, ERROR_CODES.md, graphrite.schema.json drafted.
- Sample diagrams added (bluesky, rube_goldberg_posting).

2) Core parser/validator (Weeks 2-3) – COMPLETE
- Rust workspace scaffolded: graphrite-core, graphrite-cli.
- Lexer implemented; parser supports direction, node declarations, edges (flow/conditional).
- Validations implemented: direction-first-line, quoted labels, snake_case IDs, dangling edge checks, spans on nodes/edges, no-orphan-nodes, line length.
- Diagnostics stabilized with spans.

3) CLI tooling (Week 3) – COMPLETE
- Commands: parse/ast (JSON AST), check, lint (human/pretty/json), diag (human/json), fmt (identity, --check).
- Exit codes: aligned with diagnostics; nonzero on any error.
- Output formats: human, JSON (diagnostics), JSON AST.

Someday/Maybe
- Node package (@graphrite/parser) bindings.
- Rendering interop: adapter or minimal renderer; golden tests.
- Playground/VSCode extension.

5) Docs and ecosystem
- Syntax guide, JSON AST schema polish, migration guide.

Testing strategy
- Unit tests for lexer/parser; edge kind/span tests; ID validation tests; diagnostics span tests; CLI command smoke tests; property/fuzz tests; corpus tests over samples.

Risks & mitigations
- Ecosystem split: provide adapters; keep CLI drop-in for CI.
- Rendering gaps: scope subset conservatively; document unsupported features.
- Performance: benchmark in CI; profile lexer/parser; avoid backtracking.
