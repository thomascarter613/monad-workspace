#!/usr/bin/env bash
set -euo pipefail

# Repair — E19 policy lib.rs public exports
#
# Symptom:
#   unresolved imports policy::ApprovalDecision, ApprovalDecisionKind,
#   ApprovalGateId, ApprovalGateKind, ApprovalRequirement, AuditActor,
#   AuditEvent, AuditEventKind, AuditLog, ProposedAction
#
# Cause:
#   lib.rs already had a stale `pub use policy::{ ... };` block from an older
#   policy shape. E19's policy.rs defines a newer MVP-safe policy model, so the
#   public export block needs to match the actual module symbols.
#
# Fix:
#   Replace the entire `pub use policy::{ ... };` block in
#   crates/monad-core/src/lib.rs with the E19 exports.

echo "==> Repair: E19 policy lib.rs exports"

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

cd "$REPO_ROOT"

LIB_FILE="crates/monad-core/src/lib.rs"
POLICY_FILE="crates/monad-core/src/policy.rs"

if [ ! -f "$LIB_FILE" ]; then
  echo "ERROR: expected file not found: $LIB_FILE" >&2
  exit 1
fi

if [ ! -f "$POLICY_FILE" ]; then
  echo "ERROR: expected file not found: $POLICY_FILE" >&2
  echo "Run the E19 completion script before this repair." >&2
  exit 1
fi

mkdir -p .monad/script-backups/E19/REPAIR-policy-lib-exports
BACKUP_STAMP="$(date +%Y%m%d%H%M%S)"
cp "$LIB_FILE" ".monad/script-backups/E19/REPAIR-policy-lib-exports/lib.rs.$BACKUP_STAMP.bak"

python3 <<'PY'
from pathlib import Path

path = Path("crates/monad-core/src/lib.rs")
text = path.read_text()

replacement = '''pub use policy::{
    ApprovalGate, ApprovalPlan, CommandExecutionIntent, FileOperationIntent, GatedWriteRequest,
    GatedWriteResult, OperationClassification, OperationKind, OperationMutability, PolicyFinding,
    PolicyFindingSeverity, PolicyReport, RiskLevel, build_approval_plan, build_policy_report,
    check_command_execution, check_file_operation, classify_operation, gated_generated_write,
    render_policy_evidence_results, render_policy_report, render_policy_report_json,
    write_policy_evidence,
};
'''

start = text.find("pub use policy::{")
if start == -1:
    # No existing export block. Insert near other pub use blocks.
    markers = [
        "pub use release::{",
        "pub use repository_context_pack::{",
        "pub use workspace::{",
        "pub use ai_context::{",
    ]
    for marker in markers:
        index = text.find(marker)
        if index != -1:
            text = text[:index] + replacement + text[index:]
            break
    else:
        raise SystemExit("ERROR: could not find insertion point for policy exports")
else:
    # Find the matching `};` for this pub use block. This is simpler than brace
    # matching because Rust import groups here do not contain nested braces in
    # the intended block.
    end = text.find("};", start)
    if end == -1:
        raise SystemExit("ERROR: could not find end of pub use policy block")
    end += len("};")
    text = text[:start] + replacement + text[end:]

path.write_text(text)
PY

echo "==> Formatting Rust code"
cargo fmt

echo
echo "==> E19 policy lib.rs export repair complete."
echo
echo "Focused verification:"
echo "  cargo check -p monad-core"
echo "  cargo test -p monad-core --lib policy"
echo
echo "Then continue full E19 verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-policy.sh"
echo "  tools/scripts/verify-e19.sh"
