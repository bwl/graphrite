# CRUSH.md

Concise repo guide for agentic tools.

Monorepo layout
- mermaid/: pnpm workspace with TypeScript, Vitest, ESLint, Prettier, Cypress
- mermaid-cli/: Node CLI (npm), Vitest, Vite, Puppeteer

Install
- Core: npm i -g pnpm
- mermaid/: pnpm install
- mermaid-cli/: npm ci

Build/lint/typecheck
- mermaid/: pnpm -w build; pnpm -w typecheck; pnpm -w lint
- mermaid-cli/: npm run build; npm run typecheck; npm run lint

Tests
- Unit (mermaid/ workspace): pnpm -w test
- Single test by name (Vitest): pnpm -w test -- -t "name"
- Single file (Vitest): pnpm -w vitest run path/to/file.spec.ts
- CLI tests (mermaid-cli/): npm test; single: npm test -- -t "name"
- E2E (Cypress in mermaid/): pnpm -w cypress run (headed: pnpm -w cypress open)

Formatting
- Prettier + ESLint (JS/TS). Run: pnpm -w lint --fix; pnpm -w format or prettier -w .

Imports
- Order: node builtin > external > internal; absolute where configured, else relative. No circular deps; no unused imports.

Types
- TS strict; avoid any; prefer readonly, exact object shapes; narrow with predicates/guards; surface precise return types.

Naming
- TS: camelCase for vars/functions, PascalCase for types/classes, UPPER_SNAKE for constants; test files *.spec.ts.

Errors & logging
- Never swallow; return Result/Error with context; throw typed errors in library code; do not log secrets; prefer invariant checks.

Testing style
- Vitest table-driven where sensible; deterministic; isolate timers/fetch; snapshot only for stable UI.

Security
- Validate inputs; sanitize user content; keep Puppeteer sandbox flags minimal.

Notes
- If Cursor/Copilot rules appear later, mirror them here. None detected at creation time.
