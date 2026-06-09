---
title: Add Language Smoke Tests
status: complete
epic: E13
---

# Add Language Smoke Tests

Use this verification script:

```bash
tools/scripts/verify-add-language.sh
```

It checks:

- language-aware dry-runs write no files;
- Rust service scaffolds create `Cargo.toml` and `src/main.rs`;
- Rust package scaffolds create `Cargo.toml` and `src/lib.rs`;
- TypeScript scaffolds create `package.json`, `tsconfig.json`, and `src/index.ts`;
- Python scaffolds create `pyproject.toml`, normalized module path, and a smoke test;
- Go scaffolds create `go.mod` and `main.go`;
- unsupported languages fail safely;
- duplicate writes are blocked by conflict detection.

Full E13 verification:

```bash
tools/scripts/verify-e13.sh
```
