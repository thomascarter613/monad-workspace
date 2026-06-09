#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-doctor: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

(
  cd "$tmpdir"

  echo "==> verify doctor text in uninitialized directory"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- doctor >/tmp/monad-e15-doctor.out
  grep -q "Monad doctor report" /tmp/monad-e15-doctor.out
  grep -q "No tools were installed." /tmp/monad-e15-doctor.out
  grep -q "No environment files were modified." /tmp/monad-e15-doctor.out
  grep -q "No package managers were run." /tmp/monad-e15-doctor.out
  grep -q "doctor.monad.manifest-missing" /tmp/monad-e15-doctor.out

  echo "==> verify doctor json"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- doctor --format=json >/tmp/monad-e15-doctor.json
  grep -q '"command":"doctor"' /tmp/monad-e15-doctor.json
  grep -q '"items"' /tmp/monad-e15-doctor.json

  echo "==> verify doctor is non-mutating"
  test ! -e monad.toml
  test ! -e Cargo.lock
  test ! -e package-lock.json
  test ! -e pnpm-lock.yaml
  test ! -e yarn.lock
  test ! -e go.sum

  echo "==> verify doctor in initialized workspace"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- init --yes >/tmp/monad-e15-init.out
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- doctor >/tmp/monad-e15-doctor-initialized.out
  grep -q "doctor.monad.manifest-readable" /tmp/monad-e15-doctor-initialized.out
)

echo "verify-doctor: PASS"
