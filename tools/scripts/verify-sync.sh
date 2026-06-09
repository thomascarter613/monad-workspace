#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-sync: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

(
  cd "$tmpdir"

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- init --yes >/tmp/monad-e14-init.out

  echo "==> verify sync dry-run"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --dry-run >/tmp/monad-e14-sync-dry.out
  grep -q "Monad sync dry-run plan" /tmp/monad-e14-sync-dry.out
  grep -q "No files were written." /tmp/monad-e14-sync-dry.out
  test ! -e .monad/reports/sync-report.md
  test ! -e .monad/reports/sync-report.json

  echo "==> verify sync json dry-run"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --dry-run --format=json >/tmp/monad-e14-sync-json.out
  grep -q '"command":"sync"' /tmp/monad-e14-sync-json.out
  grep -q '"mode":"dry-run"' /tmp/monad-e14-sync-json.out
  grep -q '"writes_enabled":false' /tmp/monad-e14-sync-json.out

  echo "==> verify sync generated evidence writes"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --yes >/tmp/monad-e14-sync-yes.out
  grep -q "Monad sync evidence written" /tmp/monad-e14-sync-yes.out
  grep -q "No native manifests were rewritten." /tmp/monad-e14-sync-yes.out
  test -f .monad/reports/sync-report.md
  test -f .monad/reports/sync-report.json
  grep -q "Monad Sync Evidence Report" .monad/reports/sync-report.md
  grep -q "Native manifest rewrites: none" .monad/reports/sync-report.md
  grep -q '"command":"sync"' .monad/reports/sync-report.json

  echo "==> verify sync mode guard"
  if cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync >/tmp/monad-e14-sync-no-mode.out 2>&1; then
    echo "Expected sync without mode to fail" >&2
    exit 1
  fi
  grep -q "sync currently requires either --dry-run" /tmp/monad-e14-sync-no-mode.out

  echo "==> verify conflicting mode guard"
  if cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --dry-run --yes >/tmp/monad-e14-sync-conflict-mode.out 2>&1; then
    echo "Expected sync with conflicting modes to fail" >&2
    exit 1
  fi
  grep -q "sync accepts either --dry-run or --yes" /tmp/monad-e14-sync-conflict-mode.out
)

echo "verify-sync: PASS"
