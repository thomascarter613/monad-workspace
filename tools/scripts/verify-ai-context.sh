#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-ai-context: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

(
  cd "$tmpdir"

  echo "==> verify ai-context dry-run writes no files"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- ai-context --dry-run >/tmp/monad-e18-ai-context.out
  grep -q "Monad AI context dry-run plan" /tmp/monad-e18-ai-context.out
  grep -q "No provider calls were made." /tmp/monad-e18-ai-context.out
  grep -q "No files were written." /tmp/monad-e18-ai-context.out
  test ! -e .monad/ai
  test ! -e .monad/context/assistant-handoff.md

  echo "==> verify ai-context json"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- ai-context --dry-run --format=json >/tmp/monad-e18-ai-context.json
  grep -q '"command":"ai-context"' /tmp/monad-e18-ai-context.json
  grep -q '"mode":"dry-run"' /tmp/monad-e18-ai-context.json
  grep -q '"provider_mode":"disabled"' /tmp/monad-e18-ai-context.json

  echo "==> verify ai-context apply"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- ai-context --yes >/tmp/monad-e18-ai-context-apply.out
  grep -q "Monad AI context apply result" /tmp/monad-e18-ai-context-apply.out
  grep -q "No provider calls were made." /tmp/monad-e18-ai-context-apply.out
  test -f .monad/ai/provider-config.example.toml
  test -f .monad/ai/memory/README.md
  test -f .monad/ai/memory/0001-project-memory-template.md
  test -f .monad/context/ai-context-snapshot.md
  test -f .monad/context/work-packet-plan.md
  test -f .monad/context/assistant-handoff.md
  test -f .monad/reports/ai-context-report.md
  test -f .monad/reports/ai-context-report.json

  echo "==> verify unsafe overwrite conflict"
  rm -rf .monad
  mkdir -p .monad/context
  printf 'user-owned\n' > .monad/context/assistant-handoff.md
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- ai-context --yes >/tmp/monad-e18-ai-context-conflict.out
  grep -q "conflicts:" /tmp/monad-e18-ai-context-conflict.out
)

echo "verify-ai-context: PASS"
