# Mermaid Repos Scan

## mermaid (monorepo)
- README.md, README.zh-CN.md
- LICENSE, CODE_OF_CONDUCT.md, CONTRIBUTING.md, CHANGELOG.md, CITATION.cff
- Package management: package.json, pnpm-workspace.yaml
- Build/tooling: tsconfig*.json, vite.config.ts, eslint.config.js, .husky/, .lintstagedrc.mjs
- CI: .github/workflows/* (build, test, lint, docs publish, release, codeql, dependency review, link checker, stale)
- Docs: /docs (syntax guides, config, community, news, landing, etc.) plus packages/mermaid/docs
- Examples/demos: /demos/*.html, docs/public assets
- Packages: packages/* (core mermaid, parser, layout elk, zenuml, examples, tiny)
- Scripts: /scripts (coverage, size, compare-timings, fixCSpell)
- Testing: vitest configs, cypress config, tests under packages/* and docs/scripts

## mermaid-cli
- README.md, LICENSE, CONTRIBUTING.md, CODE_OF_CONDUCT.md
- CI: .github/workflows/* (build, test, release, codeql, pr labeling)
- Docs: /docs (docker permission issues, linux sandbox)
- Source: /src (cli.js, index.js), TypeScript config present
- Tests: /src-test/test.js; test fixtures under test-positive/ and test-negative/
- Scripts: scripts/version.js, run-tests.sh
- Build: tsconfig.json, vite.config.ts
- Config: puppeteer-config.json, GitVersion.yml
