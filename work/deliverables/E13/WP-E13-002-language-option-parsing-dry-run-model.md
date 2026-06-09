---
title: WP-E13-002 Deliverable Record
epic: E13
work_packet: WP-E13-002
status: complete
---

# Deliverable Record: WP-E13-002

## Work packet

WP-E13-002 — Add language option parsing and dry-run model.

## Scope completed

This packet adds:

- `ComponentLanguage` core enum;
- canonical language parsing for `rust`, `typescript`, `python`, and `go`;
- `AddPlanOptions` language support;
- language labels in add dry-run output;
- CLI parsing for both `--language <language>` and `--language=<language>`;
- validation that `--language` is only supported for `monad add`;
- tests for supported and unsupported language IDs;
- tests for language-aware add parsing;
- a guard that rejects language-aware writes until concrete language templates are added.

## Scope intentionally deferred

This packet does not add:

- Rust component templates;
- TypeScript component templates;
- Python component templates;
- Go component templates;
- root `Cargo.toml` mutation;
- package manager workspace mutation;
- lockfile generation;
- package installation;
- native tool execution.

## Expected command behavior

Generic E12 behavior remains valid:

```bash
monad add app web --dry-run
monad add app web --yes
```

Language-aware dry-run is now valid:

```bash
monad add service api --language rust --dry-run
monad add app web --language typescript --dry-run
monad add service worker --language python --dry-run
monad add tool repo-lint --language go --dry-run
```

Language-aware writes are intentionally deferred:

```bash
monad add service api --language rust --yes
```

Expected result for the deferred write path:

```text
language-aware add writes ... are deferred
```

## Verification

Recommended verification:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Manual CLI smoke test from an initialized temporary workspace:

```bash
tmpdir="$(mktemp -d)"
repo_root="$(pwd)"
(
  cd "$tmpdir"
  cargo run --manifest-path "$repo_root/Cargo.toml" -p monad-cli -- init --yes
  cargo run --manifest-path "$repo_root/Cargo.toml" -p monad-cli -- add service api --language rust --dry-run
  test ! -e services/api/README.md
)
rm -rf "$tmpdir"
```

Unsupported language check:

```bash
tmpdir="$(mktemp -d)"
repo_root="$(pwd)"
(
  cd "$tmpdir"
  cargo run --manifest-path "$repo_root/Cargo.toml" -p monad-cli -- init --yes
  ! cargo run --manifest-path "$repo_root/Cargo.toml" -p monad-cli -- add service api --language ruby --dry-run
)
rm -rf "$tmpdir"
```

Deferred write check:

```bash
tmpdir="$(mktemp -d)"
repo_root="$(pwd)"
(
  cd "$tmpdir"
  cargo run --manifest-path "$repo_root/Cargo.toml" -p monad-cli -- init --yes
  ! cargo run --manifest-path "$repo_root/Cargo.toml" -p monad-cli -- add service api --language rust --yes
  test ! -e services/api/README.md
)
rm -rf "$tmpdir"
```

## Next packet

WP-E13-003 — Add embedded Rust component templates.
