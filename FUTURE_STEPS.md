# Future Steps: Bridging Gaps with Mermaid Ecosystem

Areas to address
1) Documentation & Site
- Action: Build docs site with mdbook or VitePress; import SPEC, CLI README, GETTING_STARTED, ERROR_CODES.
- Pipeline: GitHub Pages workflow to build and publish on main.

2) Samples & Demos
- Action: Curate sample diagrams; add golden tests; gallery in docs.
- Pipeline: Add script to validate all samples in CI.

3) CI Workflows
- Action: Add GitHub Actions: lint (rustfmt/clippy), test (cargo test), docs build, release (cargo publish), security (codeql).
- Pipeline: matrices for macOS/Linux; cache cargo and sccache.

4) Rendering Pipeline
- Action: Keep DOT export; add layout via Graphviz by default in docs; explore ELK integration later.
- CLI: render --format dot/svg (via dot), --layout elk (future).

5) CLI UX & Diagnostics
- Action: Stabilize pretty diagnostics with code, span, snippet; add --format json|human|github.
- Consistency: exit codes, --quiet/--verbose, batch mode over glob.

6) Packaging & Distribution
- Action: Produce prebuilt binaries via GitHub Releases (cross, goreleaser-like via cargo-dist).
- Archives per OS/arch; Homebrew tap for macOS.

7) Roadmap & Governance
- Action: CONTRIBUTING.md guidelines; issue templates; PR templates; CODE_OF_CONDUCT.
- Steering: RFCs for syntax changes; versioning policy (semver) and SPEC versioning.

Milestones
- M4: CI and Docs site online
- M5: Renderer polish (SVG export path), prebuilt binaries
- M6: Ecosystem: extensions, adapters, playground
