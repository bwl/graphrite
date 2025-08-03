# Invalid Samples (Graphrite vs Mermaid)

These examples are valid (or tolerated) in Mermaid.js but fail in Graphrite due to stricter rules.

- 01-missing-direction.mmd: Mermaid auto-detects, Graphrite requires direction on first non-comment line.
- 02-id-not-snake.mmd: Mermaid allows MixedCase, Graphrite enforces snake_case.
- 03-unquoted-label.mmd: Mermaid often allows bare words, Graphrite requires quotes.
- 04-inline-comment.mmd: Mermaid tolerates inline %% comments, Graphrite forbids inline comments mid-line.
- 05-dangling-edge.mmd: Mermaid may allow forward references, Graphrite errors if nodes are missing.
- 06-orphan-node.mmd: Mermaid allows isolated nodes, Graphrite forbids orphans.
- 07-long-line.mmd: Mermaid has no strict max line length, Graphrite enforces <=100.
- 08-arrow-spacing.mmd: Mermaid permits flexible spacing, Graphrite normalizes to single space around arrows (future fmt).
