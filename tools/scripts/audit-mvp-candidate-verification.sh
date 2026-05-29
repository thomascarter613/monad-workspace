#!/usr/bin/env bash
set -uo pipefail

AUDIT_ROOT=".artifacts/release/mvp-candidate-verification"
STAMP="$(date -u +"%Y%m%dT%H%M%SZ")"
AUDIT_DIR="$AUDIT_ROOT/$STAMP"
REPORT_PATH="docs/release/MVP-CANDIDATE-VERIFICATION-AUDIT.md"
SUMMARY_PATH="$AUDIT_DIR/summary.tsv"

mkdir -p "$AUDIT_DIR"
mkdir -p "$(dirname "$REPORT_PATH")"

total=0
failed=0

record_result() {
  local name="$1"
  local status="$2"
  local exit_code="$3"
  local log_path="$4"

  printf '%s\t%s\t%s\t%s\n' "$name" "$status" "$exit_code" "$log_path" >>"$SUMMARY_PATH"
}

run_step() {
  local name="$1"
  shift

  local safe_name
  safe_name="$(printf '%s' "$name" | tr '[:upper:] ' '[:lower:]-' | tr -cd 'a-z0-9-')"
  local log_path="$AUDIT_DIR/${safe_name}.log"

  total=$((total + 1))

  echo
  echo "==> $name"
  echo "Command: $*" | tee "$log_path"

  "$@" >>"$log_path" 2>&1
  local exit_code=$?

  if [[ "$exit_code" -eq 0 ]]; then
    echo "PASS: $name"
    record_result "$name" "PASS" "$exit_code" "$log_path"
  else
    echo "FAIL: $name (exit $exit_code)"
    record_result "$name" "FAIL" "$exit_code" "$log_path"
    failed=$((failed + 1))
  fi
}

write_environment() {
  {
    echo "# Environment"
    echo
    echo "Generated at UTC: $STAMP"
    echo
    echo "## Git"
    echo
    git --version || true
    echo
    echo "## Rust"
    echo
    rustc --version || true
    cargo --version || true
    echo
    echo "## Current branch"
    echo
    git branch --show-current || true
    echo
    echo "## Latest commit"
    echo
    git log --oneline --max-count=1 || true
  } >"$AUDIT_DIR/environment.md"
}

write_report() {
  local overall_status="PASS"
  if [[ "$failed" -ne 0 ]]; then
    overall_status="FAIL"
  fi

cat > "$REPORT_PATH" <<EOF_REPORT
---
title: MVP Candidate Verification Audit
description: Release-candidate verification audit for Monad internal MVP candidate preparation.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: MVP Candidate Cut
epic: E8
work_packet: WP-E8-005
---

# MVP Candidate Verification Audit

## 1. Purpose

This document records the release-candidate verification audit for Monad's internal MVP candidate cut.

This is not a public release record. It does not authorize package publishing, installer generation, hosted launch, public announcement, or tag creation.

## 2. Audit status

| Field | Value |
|---|---|
| Overall status | ${overall_status} |
| Audit timestamp UTC | ${STAMP} |
| Audit artifact directory | \`${AUDIT_DIR}\` |
| Commands run | ${total} |
| Failed commands | ${failed} |

## 3. Scope

This audit verifies the internal MVP candidate preparation baseline.

In scope:

- formatting verification
- test verification
- Clippy verification
- root verification script
- working-tree status before and after verification
- environment evidence

Out of scope:

- fixing major blockers
- public release
- package publishing
- installer generation
- hosted service launch
- tag creation

## 4. Environment evidence

Environment evidence is recorded at:

\`\`\`text
${AUDIT_DIR}/environment.md
\`\`\`

## 5. Command evidence

| Step | Status | Exit code | Log |
|---|---:|---:|---|
EOF_REPORT

  while IFS=$'\t' read -r name status exit_code log_path; do
    printf '| %s | %s | %s | `%s` |\n' "$name" "$status" "$exit_code" "$log_path" >>"$REPORT_PATH"
  done <"$SUMMARY_PATH"

  cat >> "$REPORT_PATH" <<EOF_REPORT

## 6. Blockers

EOF_REPORT

  if [[ "$failed" -eq 0 ]]; then
    cat >> "$REPORT_PATH" <<'EOF_REPORT'
No verification blockers were detected by this audit run.
EOF_REPORT
  else
    cat >> "$REPORT_PATH" <<EOF_REPORT
This audit detected ${failed} failing verification step(s).

Review the failed log files in:

\`\`\`text
${AUDIT_DIR}
\`\`\`

Do not close WP-E8-005 as complete until blockers are fixed or explicitly documented for carry-forward.
EOF_REPORT
  fi

  cat >> "$REPORT_PATH" <<'EOF_REPORT'

## 7. Required follow-up

If this audit passes:

- commit this audit record
- close WP-E8-005
- proceed to WP-E8-006 internal tag preparation

If this audit fails:

- do not tag
- do not publish
- fix only in-scope issues if they are small
- otherwise create follow-up work packets or record blockers

## 8. Verification command set

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
git status --short
```

EOF_REPORT
}

: >"$SUMMARY_PATH"

write_environment

run_step "git status before verification" git status --short
run_step "cargo fmt check" cargo fmt --check
run_step "cargo test" cargo test
run_step "cargo clippy strict" cargo clippy --all-targets --all-features -- -D warnings
run_step "root verifier" tools/scripts/verify.sh
run_step "git status after verification" git status --short

write_report

echo
echo "Audit report written to: $REPORT_PATH"
echo "Audit artifacts written to: $AUDIT_DIR"

if [[ "$failed" -ne 0 ]]; then
echo "Audit completed with failures: $failed"
exit 1
fi

echo "Audit completed successfully."
