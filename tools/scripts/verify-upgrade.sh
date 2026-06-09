#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-upgrade: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

(
  cd "$tmpdir"

  echo "==> verify upgrade dry-run blocks missing manifest without writing"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- upgrade --dry-run >/tmp/monad-e17-upgrade-missing.out
  grep -q "Monad upgrade dry-run plan" /tmp/monad-e17-upgrade-missing.out
  grep -q "missing-manifest" /tmp/monad-e17-upgrade-missing.out
  grep -q "No files were written." /tmp/monad-e17-upgrade-missing.out
  test ! -e .monad/upgrade
  test ! -e .monad/reports/upgrade-report.md

  echo "==> verify upgrade apply rejects missing manifest"
  if cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- upgrade --yes >/tmp/monad-e17-upgrade-missing-apply.out 2>&1; then
    echo "Expected upgrade apply without monad.toml to fail" >&2
    exit 1
  fi
  grep -q "upgrade cannot be applied" /tmp/monad-e17-upgrade-missing-apply.out

  echo "==> verify upgrade dry-run and apply with manifest"
  cat > monad.toml <<'TOML'
schema_version = 1

[project]
name = "example"
TOML

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- upgrade --dry-run >/tmp/monad-e17-upgrade.out
  grep -q "up-to-date" /tmp/monad-e17-upgrade.out
  grep -q "No files were written." /tmp/monad-e17-upgrade.out

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- upgrade --dry-run --format=json >/tmp/monad-e17-upgrade.json
  grep -q '"command":"upgrade"' /tmp/monad-e17-upgrade.json
  grep -q '"mode":"dry-run"' /tmp/monad-e17-upgrade.json
  grep -q '"target_version":"1"' /tmp/monad-e17-upgrade.json

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- upgrade --yes >/tmp/monad-e17-upgrade-apply.out
  grep -q "Monad upgrade apply result" /tmp/monad-e17-upgrade-apply.out
  grep -q "No source code was rewritten." /tmp/monad-e17-upgrade-apply.out
  test -f .monad/upgrade/contract-version
  test -f .monad/upgrade/README.md
  test -f .monad/reports/upgrade-report.md
  test -f .monad/reports/upgrade-report.json

  echo "==> verify unsafe overwrite is refused"
  rm -rf .monad
  mkdir -p .monad/upgrade
  printf 'user-owned content\n' > .monad/upgrade/README.md
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- upgrade --yes >/tmp/monad-e17-upgrade-conflict.out
  grep -q "conflicts:" /tmp/monad-e17-upgrade-conflict.out
)

echo "verify-upgrade: PASS"
