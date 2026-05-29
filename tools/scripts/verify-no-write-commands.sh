#!/usr/bin/env bash
set -euo pipefail

# Verifies that Monad planning and dry-run commands do not mutate Git status.
#
# This script intentionally checks only current MVP no-write commands:
# - monad plan
# - evolve verify-baseline --dry-run
# - evolve context-baseline --dry-run
#
# It does not cover commands that intentionally write artifacts, such as
# `monad check` text output or `monad context --write`.

before="$(mktemp)"
after="$(mktemp)"
trap 'rm -f "$before" "$after"' EXIT

git status --short >"$before"

cargo run -p monad-cli -- plan "explain this repository" >/tmp/monad-plan-no-write.out
cargo run -p monad-cli -- evolve verify-baseline --dry-run >/tmp/monad-verify-baseline-dry-run.out
cargo run -p monad-cli -- evolve context-baseline --dry-run >/tmp/monad-context-baseline-dry-run.out

git status --short >"$after"

if ! diff -u "$before" "$after"; then
  echo "No-write command verification failed: git status changed." >&2
  echo "Commands checked:" >&2
  echo "  cargo run -p monad-cli -- plan \"explain this repository\"" >&2
  echo "  cargo run -p monad-cli -- evolve verify-baseline --dry-run" >&2
  echo "  cargo run -p monad-cli -- evolve context-baseline --dry-run" >&2
  exit 1
fi

grep -q "No files were created, updated, or deleted." /tmp/monad-plan-no-write.out
grep -q "No shell commands were run." /tmp/monad-plan-no-write.out
grep -q "No Git state was changed." /tmp/monad-plan-no-write.out
grep -q "No real model provider or external AI API was called." /tmp/monad-plan-no-write.out

grep -q "Mode: dry-run" /tmp/monad-verify-baseline-dry-run.out
grep -q "No files were written." /tmp/monad-verify-baseline-dry-run.out

grep -q "Mode: dry-run" /tmp/monad-context-baseline-dry-run.out
grep -q "No files were written." /tmp/monad-context-baseline-dry-run.out
grep -q "No AI summarization was performed." /tmp/monad-context-baseline-dry-run.out

echo "No-write command verification passed."
