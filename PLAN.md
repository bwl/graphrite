Graphrite: Plan of Action

Decision points
- Fork-first strategy: Build Graphrite as a standalone ecosystem. Maintain optional compatibility adapters for Mermaid inputs if needed.
- Implementation language: Rust selected for core (speed, WASM). TS bridge planned for Node/web.
- Language form: Define the Graphrite dialect; spec is authoritative and may diverge from Mermaid where clarity/perf require.

Milestones
1) Spec and prototype (Week 1) – COMPLETE
- SPEC.md, ERROR_CODES.md, graphrite.schema.json drafted.
- Sample diagrams added (bluesky, rube_goldberg_posting).

2) Core parser/validator (Weeks 2-3) – IN PROGRESS
- Rust workspace scaffolded: graphrite-core, graphrite-cli.
- Lexer implemented; parser supports direction, node declarations, edges (flow/conditional).
- Basic validations implemented: direction-first-line, quoted labels; tests added.
- Next: enforce snake_case IDs, no orphans/dangling, spacing/line length, stable spans and diagnostics.

3) Tooling and bindings (Week 3) – TODO
- CLI commands drafted; implement lint/compile output formats; plan Node package (@graphrite/parser).

4) Rendering interop (Week 4) – TODO
- Adapter or minimal renderer; golden tests.

5) Docs and ecosystem (Week 4) – TODO
- Syntax guide, JSON AST schema polish, migration guide, playground/VSCode.

Testing strategy
- Unit tests for lexer/parser; add property/fuzz tests; corpus tests over samples.

Risks & mitigations
- Ecosystem split: provide adapters; keep CLI drop-in for CI.
- Rendering gaps: scope subset conservatively; document unsupported features.
- Performance: benchmark in CI; profile lexer/parser; avoid backtracking.
