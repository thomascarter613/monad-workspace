#!/usr/bin/env bash
set -euo pipefail

# Repair — E14 sync Clippy collapsible_if
#
# Symptom:
#   error: this `if` statement can be collapsed
#   crates/monad-core/src/sync.rs
#
# This script applies the Clippy-suggested form:
#
#   if entry_path.is_dir()
#       && let Some(name) = entry.file_name().to_str()
#   {
#       directories.push(name.to_string());
#   }

echo "==> Repair: E14 sync clippy collapsible_if"

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

cd "$REPO_ROOT"

SYNC_FILE="crates/monad-core/src/sync.rs"

if [ ! -f "$SYNC_FILE" ]; then
  echo "ERROR: expected file not found: $SYNC_FILE" >&2
  exit 1
fi

mkdir -p .monad/script-backups/E14/REPAIR-sync-clippy-collapsible-if
BACKUP_STAMP="$(date +%Y%m%d%H%M%S)"
cp "$SYNC_FILE" ".monad/script-backups/E14/REPAIR-sync-clippy-collapsible-if/sync.rs.$BACKUP_STAMP.bak"

python3 <<'PY'
from pathlib import Path

path = Path("crates/monad-core/src/sync.rs")
text = path.read_text()

old = '''        let entry_path = entry.path();
        if entry_path.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                directories.push(name.to_string());
            }
        }
'''

new = '''        let entry_path = entry.path();
        if entry_path.is_dir()
            && let Some(name) = entry.file_name().to_str()
        {
            directories.push(name.to_string());
        }
'''

if old not in text:
    # Idempotency: if already fixed, do nothing.
    if new in text:
        print("collapsible_if repair already applied")
    else:
        raise SystemExit("ERROR: expected nested if block not found in sync.rs")
else:
    text = text.replace(old, new, 1)
    path.write_text(text)
PY

echo "==> Formatting Rust code"
cargo fmt

echo
echo "==> Clippy repair complete."
echo
echo "Focused verification:"
echo "  cargo clippy -p monad-core --all-targets --all-features -- -D warnings"
echo
echo "Then continue:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-sync.sh"
echo "  tools/scripts/verify-e14.sh"
