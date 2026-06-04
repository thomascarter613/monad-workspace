#!/usr/bin/env bash
set -uo pipefail

# Complete WP-E10-005 — Run final public pre-release verification audit.
#
# This script runs the final public pre-release verification command set,
# captures command output, writes a Markdown audit report, and creates a
# work-packet deliverable record.
#
# It intentionally does not cut a tag or approve release. That remains WP-E10-006.

mkdir -p docs/release
mkdir -p work/deliverables/E10
mkdir -p .monad/reports/e10

REPORT_PATH="docs/release/PUBLIC-PRERELEASE-VERIFICATION-AUDIT.md"
DELIVERABLE_PATH="work/deliverables/E10/WP-E10-005-public-prerelease-verification-audit.md"
RAW_LOG_DIR=".monad/reports/e10/wp-e10-005"
mkdir -p "$RAW_LOG_DIR"

RUN_STARTED_AT="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
FAILURES=0
TOTAL=0

commands=(
  "git status --short"
  "cargo fmt --check"
  "cargo test"
  "cargo clippy --all-targets --all-features -- -D warnings"
  "cargo run -p monad-cli -- --help"
  "cargo run -p monad-cli -- version"
  "cargo run -p monad-cli -- info"
  "cargo run -p monad-cli -- inspect"
  "cargo run -p monad-cli -- check"
  "cargo run -p monad-cli -- graph"
  "cargo run -p monad-cli -- context"
  "cargo run -p monad-cli -- context verify"
  "cargo run -p monad-cli -- plan \"explain this repository\""
  "cargo run -p monad-cli -- evolve verify-baseline --dry-run"
  "cargo run -p monad-cli -- evolve context-baseline --dry-run"
  "tools/scripts/verify.sh"
  "git status --short"
)

slugs=(
  "git-status-before"
  "cargo-fmt"
  "cargo-test"
  "cargo-clippy"
  "monad-help"
  "monad-version"
  "monad-info"
  "monad-inspect"
  "monad-check"
  "monad-graph"
  "monad-context"
  "monad-context-verify"
  "monad-plan"
  "monad-evolve-verify-baseline"
  "monad-evolve-context-baseline"
  "root-verify"
  "git-status-after"
)

results_md=""

run_check() {
  local idx="$1"
  local label="$2"
  local command="$3"
  local slug="$4"
  local output_path="${RAW_LOG_DIR}/${slug}.log"

  TOTAL=$((TOTAL + 1))

  echo
  echo "==> ${label}"
  echo "    ${command}"

  # Run through bash so quoted command examples work exactly as listed.
  bash -lc "$command" >"$output_path" 2>&1
  local exit_code=$?

  if [[ "$exit_code" -eq 0 ]]; then
    echo "    PASS"
    status="Pass"
  else
    echo "    FAIL (${exit_code})"
    status="Fail"
    FAILURES=$((FAILURES + 1))
  fi

  results_md+=$'\n'"| ${idx} | \`${command}\` | ${status} | ${exit_code} | \`${output_path}\` |"$'\n'

  {
    echo
    echo "### ${idx}. ${label}"
    echo
    echo '```bash'
    echo "$command"
    echo '```'
    echo
    echo "**Result:** ${status}"
    echo
    echo "**Exit code:** ${exit_code}"
    echo
    echo "**Raw log:** \`${output_path}\`"
    echo
    echo "<details>"
    echo "<summary>Command output</summary>"
    echo
    echo '```text'
    cat "$output_path"
    echo '```'
    echo
    echo "</details>"
    echo
  } >> "${RAW_LOG_DIR}/details.md"
}

rm -f "${RAW_LOG_DIR}/details.md"

for i in "${!commands[@]}"; do
  number=$((i + 1))
  run_check "$number" "${slugs[$i]}" "${commands[$i]}" "${slugs[$i]}"
done

RUN_COMPLETED_AT="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

if [[ "$FAILURES" -eq 0 ]]; then
  OVERALL_STATUS="Pass"
  REPORT_STATUS="accepted"
  DECISION_TEXT="Final verification audit passed. WP-E10-006 may proceed to the public pre-release tag go/no-go decision."
else
  OVERALL_STATUS="Fail"
  REPORT_STATUS="review"
  DECISION_TEXT="Final verification audit failed. WP-E10-006 should not approve a public pre-release tag until failures are fixed or explicitly deferred with rationale."
fi

cat > "$REPORT_PATH" <<EOF
---
title: "Public Pre-Release Verification Audit"
document_type: "release-verification-audit"
status: ${REPORT_STATUS}
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-005
tags:
  - monad
  - release
  - public-prerelease
  - verification
  - audit
related:
  - README.md
  - docs/release/PUBLIC-CLAIMS-AUDIT.md
  - docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md
  - docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
  - docs/release/PUBLIC-PRERELEASE-NOTES.md
---

# Public Pre-Release Verification Audit

## Status

${OVERALL_STATUS}.

## Work Packet

WP-E10-005 — Run final public pre-release verification audit.

## Purpose

This audit captures final verification evidence before the public pre-release tag decision.

It does not approve or cut a release.

Release approval remains governed by:

\`\`\`text
WP-E10-006 — Decide and cut first public pre-release tag, if approved
\`\`\`

## Run Metadata

| Field | Value |
| --- | --- |
| Started At | ${RUN_STARTED_AT} |
| Completed At | ${RUN_COMPLETED_AT} |
| Total Checks | ${TOTAL} |
| Failed Checks | ${FAILURES} |
| Overall Status | ${OVERALL_STATUS} |
| Raw Log Directory | \`${RAW_LOG_DIR}\` |

## Decision Guidance

${DECISION_TEXT}

## Verification Summary

| # | Command | Status | Exit Code | Raw Log |
| ---: | --- | --- | ---: | --- |
${results_md}

## Detailed Command Evidence
EOF

cat "${RAW_LOG_DIR}/details.md" >> "$REPORT_PATH"

cat >> "$REPORT_PATH" <<'EOF'

## Public Pre-Release Boundary

The verification audit assumes the public pre-release posture remains source-only.

The following remain out of scope:

- packaged binary release;
- installer release;
- Crates.io/package-manager publication;
- hosted service;
- SaaS launch;
- autonomous agent runtime;
- production-ready platform claim.

## Outcome

The outcome of this audit must be used by WP-E10-006.

If all checks pass, WP-E10-006 may decide whether to cut the source-only public pre-release tag.

If any checks fail, WP-E10-006 should either defer the public pre-release or document an explicit blocker-resolution path.
EOF

cat > "$DELIVERABLE_PATH" <<EOF
---
title: "WP-E10-005 Public Pre-Release Verification Audit Deliverable"
document_type: "deliverable-record"
status: ${REPORT_STATUS}
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-005
tags:
  - monad
  - e10
  - release
  - verification
  - audit
related:
  - docs/release/PUBLIC-PRERELEASE-VERIFICATION-AUDIT.md
  - docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md
---

# WP-E10-005 Public Pre-Release Verification Audit Deliverable

## Work Packet

WP-E10-005 — Run final public pre-release verification audit.

## Outcome

${OVERALL_STATUS}.

## Summary

This work packet ran the final public pre-release verification command set and captured evidence in:

\`\`\`text
docs/release/PUBLIC-PRERELEASE-VERIFICATION-AUDIT.md
\`\`\`

Raw command logs were written to:

\`\`\`text
${RAW_LOG_DIR}
\`\`\`

## Verification Result

| Field | Value |
| --- | --- |
| Total Checks | ${TOTAL} |
| Failed Checks | ${FAILURES} |
| Overall Status | ${OVERALL_STATUS} |

## Next Work Packet

\`\`\`text
WP-E10-006 — Decide and cut first public pre-release tag, if approved
\`\`\`

## Recommended Commit

\`\`\`bash
git commit -m "docs(release): add public prerelease verification audit"
\`\`\`

## Closeout Note

WP-E10-005 is complete once this audit is committed and the corresponding GitHub work-packet issue or tracking item is closed.

If the audit status is Fail, WP-E10-006 should not cut a public pre-release tag without first resolving or explicitly deferring the failures.
EOF

if [[ -f docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md ]]; then
  python3 - <<PY
from pathlib import Path

path = Path("docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md")
text = path.read_text(encoding="utf-8")
overall = "${OVERALL_STATUS}"
status = "Pass" if overall == "Pass" else "Fail"
evidence = "`docs/release/PUBLIC-PRERELEASE-VERIFICATION-AUDIT.md`"

replacements = {
    "| Final verification audit complete | Pending | WP-E10-005 remains required |": f"| Final verification audit complete | {status} | {evidence} |",
    "| `cargo fmt --check` passes. | Pending | WP-E10-005 | Final verification audit required. |": f"| `cargo fmt --check` passes. | {status} | {evidence} | See verification audit. |",
    "| `cargo test` passes. | Pending | WP-E10-005 | Final verification audit required. |": f"| `cargo test` passes. | {status} | {evidence} | See verification audit. |",
    "| `cargo clippy --all-targets --all-features -- -D warnings` passes. | Pending | WP-E10-005 | Final verification audit required. |": f"| `cargo clippy --all-targets --all-features -- -D warnings` passes. | {status} | {evidence} | See verification audit. |",
    "| `tools/scripts/verify.sh` passes. | Pending | WP-E10-005 | Final verification audit required. |": f"| `tools/scripts/verify.sh` passes. | {status} | {evidence} | See verification audit. |",
    "| Core smoke commands run successfully. | Pending | WP-E10-005 | Final verification audit required. |": f"| Core smoke commands run successfully. | {status} | {evidence} | See verification audit. |",
    "WP-E10-005 — Run final public pre-release verification audit\\n": "",
}
for old, new in replacements.items():
    text = text.replace(old, new)

if overall == "Pass":
    text = text.replace(
        "final verification and tag decision gates remain pending.",
        "the tag decision gate remains pending.",
    )
else:
    text = text.replace(
        "final verification and tag decision gates remain pending.",
        "final verification has failures and the tag decision gate remains pending.",
    )

path.write_text(text, encoding="utf-8")
PY
  echo "Updated docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md"
fi

echo
echo "WP-E10-005 verification audit written:"
echo "  ${REPORT_PATH}"
echo "  ${DELIVERABLE_PATH}"
echo "  ${RAW_LOG_DIR}/"
echo
echo "Overall status: ${OVERALL_STATUS}"
echo "Failures: ${FAILURES}/${TOTAL}"

if [[ "$FAILURES" -eq 0 ]]; then
  exit 0
else
  exit 1
fi
