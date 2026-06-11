#!/usr/bin/env bash
set -euo pipefail

# Focused E36 repair:
# Replace unnecessary sort_by calls with sort_by_key in github_workflow.rs.
#
# Safety:
# - touches only crates/monad-core/src/github_workflow.rs
# - backs up touched file
# - no network access
# - no package-manager invocation

TARGET="crates/monad-core/src/github_workflow.rs"

if [[ ! -f "Cargo.toml" || ! -f "$TARGET" ]]; then
  echo "Run this script from the monad-workspace repository root." >&2
  exit 1
fi

BACKUP_DIR=".monad/script-backups/repair-e36-github-workflow-sort-by-key-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$BACKUP_DIR/crates/monad-core/src"
cp "$TARGET" "$BACKUP_DIR/$TARGET"

python3 - <<'PY'
from pathlib import Path

path = Path("crates/monad-core/src/github_workflow.rs")
text = path.read_text()

replacements = {
    "        issue_exports.sort_by(|left, right| left.issue_number().cmp(&right.issue_number()));":
    "        issue_exports.sort_by_key(GithubIssueExportRecord::issue_number);",
    "        closeout_evidence.sort_by(|left, right| left.issue_number().cmp(&right.issue_number()));":
    "        closeout_evidence.sort_by_key(GithubIssueCloseoutEvidence::issue_number);",
}

missing = []
for old, new in replacements.items():
    if old in text:
        text = text.replace(old, new, 1)
    else:
        missing.append(old.strip())

if missing:
    raise SystemExit("Could not find expected sort_by line(s):\n- " + "\n- ".join(missing))

path.write_text(text)
PY

cargo fmt

echo "Applied E36 sort_by_key clippy repair."
echo "Backup written under: $BACKUP_DIR"
echo
echo "Run focused verification:"
echo "  cargo clippy -p monad-core --all-targets --all-features -- -D warnings"
echo
echo "Then run full verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-github-workflow.sh"
echo "  tools/scripts/verify-e36.sh"
