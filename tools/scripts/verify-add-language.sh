#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-add-language: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

echo "==> verify-add-language: temp workspace: $tmpdir"

(
  cd "$tmpdir"

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- init --yes >/tmp/monad-e13-init.out

  echo "==> dry-run checks write no files"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add service api-dry --language rust --dry-run >/tmp/monad-e13-rust-dry.out
  grep -q "language: rust" /tmp/monad-e13-rust-dry.out
  grep -q "No files were written." /tmp/monad-e13-rust-dry.out
  test ! -e services/api-dry/README.md

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add app web-dry --language typescript --dry-run >/tmp/monad-e13-typescript-dry.out
  grep -q "language: typescript" /tmp/monad-e13-typescript-dry.out
  test ! -e apps/web-dry/package.json

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add service worker-dry --language python --dry-run >/tmp/monad-e13-python-dry.out
  grep -q "language: python" /tmp/monad-e13-python-dry.out
  test ! -e services/worker-dry/pyproject.toml

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add tool repo-lint-dry --language go --dry-run >/tmp/monad-e13-go-dry.out
  grep -q "language: go" /tmp/monad-e13-go-dry.out
  test ! -e tools/repo-lint-dry/go.mod

  echo "==> guarded write checks create language files"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add service api --language rust --yes >/tmp/monad-e13-rust-yes.out
  test -f services/api/README.md
  test -f services/api/Cargo.toml
  test -f services/api/src/main.rs
  grep -q 'name = "api"' services/api/Cargo.toml
  grep -q "No Git commands were run." /tmp/monad-e13-rust-yes.out

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add package shared-core --language rust --yes >/tmp/monad-e13-rust-package-yes.out
  test -f packages/shared-core/README.md
  test -f packages/shared-core/Cargo.toml
  test -f packages/shared-core/src/lib.rs
  test ! -e packages/shared-core/src/main.rs

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add app web --language typescript --yes >/tmp/monad-e13-typescript-yes.out
  test -f apps/web/README.md
  test -f apps/web/package.json
  test -f apps/web/tsconfig.json
  test -f apps/web/src/index.ts
  grep -q '"name": "web"' apps/web/package.json

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add service worker --language python --yes >/tmp/monad-e13-python-yes.out
  test -f services/worker/README.md
  test -f services/worker/pyproject.toml
  test -f services/worker/src/worker/__init__.py
  test -f services/worker/tests/test_smoke.py
  grep -q 'name = "worker"' services/worker/pyproject.toml

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add service my-worker --language python --yes >/tmp/monad-e13-python-hyphen-yes.out
  test -f services/my-worker/src/my_worker/__init__.py
  grep -q "my_worker" services/my-worker/README.md

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add tool repo-lint --language go --yes >/tmp/monad-e13-go-yes.out
  test -f tools/repo-lint/README.md
  test -f tools/repo-lint/go.mod
  test -f tools/repo-lint/main.go
  grep -q "module monad.local/repo-lint" tools/repo-lint/go.mod

  echo "==> unsupported language fails safely"
  if cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add service nope --language ruby --dry-run >/tmp/monad-e13-ruby.out 2>&1; then
    echo "Expected unsupported language to fail" >&2
    exit 1
  fi
  grep -q "unsupported component language" /tmp/monad-e13-ruby.out
  test ! -e services/nope

  echo "==> overwrite protection still works"
  if cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- add service api --language rust --yes >/tmp/monad-e13-conflict.out 2>&1; then
    echo "Expected duplicate language-aware add to fail" >&2
    exit 1
  fi
  grep -q "add plan has conflicts" /tmp/monad-e13-conflict.out
)

echo "verify-add-language: PASS"
