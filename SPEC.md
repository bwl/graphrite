# Graphrite Specification (Draft)

Scope
- Deterministic subset for AI parsing and validation.

Lexical rules
- Identifiers: snake_case, start with letter, [a-z][a-z0-9_]*, no trailing underscore.
- Strings: double-quoted, allow escaped \" and \\.
- Whitespace: spaces and tabs; no inline comments mid-line.
- Comments: full-line only starting with %%.
- Direction: first non-comment line must be `direction LR` or `direction TD`.
- Arrows: `-->` flow, `-.->` conditional; single space around arrows.

Structure
- Single diagram per file.
- Node declaration: id["Label"]
- Edge: src --> dst [optional label not supported in v1].
- No orphan nodes or dangling edges.

Formatting
- Max line length: 100.
- One statement per line.

Metadata
- Required header: %% Diagram: <title>
- Optional: %% Meta: key=value; key2=value2

Errors
- Hard-fail on violations; provide code, message, span.

Outputs
- AST JSON v1: nodes[], edges[], metadata{}, directives{direction}, sourceSpans.
