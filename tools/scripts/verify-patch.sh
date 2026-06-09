#!/usr/bin/env bash
set -euo pipefail

echo "[verify-patch] cargo test -p monad-core --lib patch"
cargo test -p monad-core --lib patch

echo "[verify-patch] monad patch --dry-run"
cargo run -p monad-cli -- patch --dry-run >/tmp/monad-patch-dry-run.txt
grep -q "Monad patch planning and supervised apply plan" /tmp/monad-patch-dry-run.txt
grep -q "No autonomous patch application" /tmp/monad-patch-dry-run.txt

echo "[verify-patch] monad patch --dry-run --format=json"
cargo run -p monad-cli -- patch --dry-run --format=json >/tmp/monad-patch-dry-run.json
grep -q '"command":"patch"' /tmp/monad-patch-dry-run.json
grep -q '"mode":"dry-run"' /tmp/monad-patch-dry-run.json

echo "[verify-patch] monad patch --yes"
cargo run -p monad-cli -- patch --yes >/tmp/monad-patch-apply.txt
grep -q "Monad supervised patch apply result" /tmp/monad-patch-apply.txt

test -f .monad/patches/e20-supervised-apply-foundation.md
test -f .monad/reports/patch-plan.md
test -f .monad/reports/patch-plan.json

grep -q "E20 Supervised Patch Apply Foundation" .monad/patches/e20-supervised-apply-foundation.md
grep -q "Monad patch planning and supervised apply plan" .monad/reports/patch-plan.md
grep -q '"command":"patch"' .monad/reports/patch-plan.json

echo "[verify-patch] ok"
