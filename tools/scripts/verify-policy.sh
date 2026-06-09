#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-policy: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

(
  cd "$tmpdir"

  echo "==> verify policy dry-run writes no files"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- policy --dry-run >/tmp/monad-e19-policy.out
  grep -q "Monad policy and approval-gate report" /tmp/monad-e19-policy.out
  grep -q "approval_required:" /tmp/monad-e19-policy.out
  grep -q "blocked:" /tmp/monad-e19-policy.out
  grep -q "No commands were executed." /tmp/monad-e19-policy.out
  test ! -e .monad/reports/policy-report.md

  echo "==> verify policy json"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- policy --dry-run --format=json >/tmp/monad-e19-policy.json
  grep -q '"command":"policy"' /tmp/monad-e19-policy.json
  grep -q '"approval_required"' /tmp/monad-e19-policy.json
  grep -q '"blocked"' /tmp/monad-e19-policy.json

  echo "==> verify policy evidence write"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- policy --yes >/tmp/monad-e19-policy-apply.out
  grep -q "Monad policy evidence write result" /tmp/monad-e19-policy-apply.out
  grep -q "No user source files were rewritten." /tmp/monad-e19-policy-apply.out
  test -f .monad/reports/policy-report.md
  test -f .monad/reports/policy-report.json
  grep -q "patch" .monad/reports/policy-report.md
  grep -q "forbidden" .monad/reports/policy-report.md
)

echo "verify-policy: PASS"
