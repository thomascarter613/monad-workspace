#!/usr/bin/env bash
set -euo pipefail

SCRIPT_NAME="repair_e20_patch_unused_policy_finding_severity_import"
PATCH_FILE="crates/monad-core/src/patch.rs"
BACKUP_ROOT=".monad/script-backups/${SCRIPT_NAME}/$(date -u +%Y%m%dT%H%M%SZ)"

log() {
  printf '[%s] %s\n' "$SCRIPT_NAME" "$1"
}

fail() {
  printf '[%s] ERROR: %s\n' "$SCRIPT_NAME" "$1" >&2
  exit 1
}

if [[ ! -f "Cargo.toml" ]]; then
  fail "run this script from the repository root"
fi

if [[ ! -f "$PATCH_FILE" ]]; then
  fail "expected file not found: $PATCH_FILE"
fi

mkdir -p "$BACKUP_ROOT/$(dirname "$PATCH_FILE")"
cp "$PATCH_FILE" "$BACKUP_ROOT/$PATCH_FILE"
log "backed up $PATCH_FILE to $BACKUP_ROOT/$PATCH_FILE"

python3 - <<'PY'
from pathlib import Path

path = Path("crates/monad-core/src/patch.rs")
text = path.read_text()
original = text

text = text.replace(
    "    ApprovalPlan, FileOperationIntent, GatedWriteRequest, GatedWriteResult, PolicyFindingSeverity,\n",
    "    ApprovalPlan, FileOperationIntent, GatedWriteRequest, GatedWriteResult,\n",
)
text = text.replace(
    "    ApprovalPlan, FileOperationIntent, GatedWriteRequest, GatedWriteResult, PolicyFindingSeverity,",
    "    ApprovalPlan, FileOperationIntent, GatedWriteRequest, GatedWriteResult,",
)
text = text.replace(" PolicyFindingSeverity,", "")
text = text.replace("PolicyFindingSeverity, ", "")
text = text.replace("PolicyFindingSeverity", "")

if text == original:
    raise SystemExit("no PolicyFindingSeverity import was removed; file may already be repaired")

path.write_text(text)
PY

if grep -q 'PolicyFindingSeverity' "$PATCH_FILE"; then
  fail "PolicyFindingSeverity is still present in $PATCH_FILE"
fi

log "removed unused PolicyFindingSeverity import from $PATCH_FILE"
log "running cargo fmt"
cargo fmt

log "repair complete"
log "next suggested checks:"
log "  cargo check -p monad-core"
log "  cargo test -p monad-core --lib patch"
log "  cargo clippy --all-targets --all-features -- -D warnings"
