#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E10-006 — Decide and cut first public pre-release tag, if approved.
#
# Safe default:
# - creates the release decision document
# - creates the work-packet deliverable record
# - updates the public pre-release evidence checklist if present
# - DOES NOT cut a tag unless --cut-tag <tag> is provided
#
# Recommended flow:
#   1. Run WP-E10-005 verification audit first.
#   2. Run this script without --cut-tag to write the decision docs.
#   3. Review, commit, and push the decision docs.
#   4. If approved and clean, rerun with --cut-tag <tag>.
#
# Example:
#   ./complete_wp_e10_006_public_prerelease_decision.sh
#   git add ...
#   git commit -m "docs(release): decide first public prerelease tag"
#   git push
#   ./complete_wp_e10_006_public_prerelease_decision.sh --cut-tag v0.1.0-public-prerelease.0
#   git push origin v0.1.0-public-prerelease.0

CUT_TAG=""
if [[ "${1:-}" == "--cut-tag" ]]; then
  CUT_TAG="${2:-}"
  if [[ -z "$CUT_TAG" ]]; then
    echo "Missing tag name after --cut-tag." >&2
    exit 2
  fi
elif [[ "$#" -gt 0 ]]; then
  echo "Unknown argument: $1" >&2
  echo "Usage: $0 [--cut-tag <tag-name>]" >&2
  exit 2
fi

mkdir -p docs/release
mkdir -p work/deliverables/E10

AUDIT_PATH="docs/release/PUBLIC-PRERELEASE-VERIFICATION-AUDIT.md"
CHECKLIST_PATH="docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md"
DECISION_PATH="docs/release/FIRST-PUBLIC-PRERELEASE-TAG-DECISION.md"
DELIVERABLE_PATH="work/deliverables/E10/WP-E10-006-public-prerelease-tag-decision.md"

if [[ ! -f "$AUDIT_PATH" ]]; then
  echo "Missing verification audit: $AUDIT_PATH" >&2
  echo "Run WP-E10-005 before WP-E10-006." >&2
  exit 1
fi

if grep -qE '^## Status[[:space:]]*$' "$AUDIT_PATH" && grep -qE '^Pass\.[[:space:]]*$' "$AUDIT_PATH"; then
  AUDIT_STATUS="Pass"
else
  AUDIT_STATUS="Fail"
fi

if grep -qE '^\\| Failed Checks \\| 0 \\|' "$AUDIT_PATH"; then
  FAILED_CHECKS_ZERO="yes"
else
  FAILED_CHECKS_ZERO="no"
fi

if [[ "$AUDIT_STATUS" == "Pass" && "$FAILED_CHECKS_ZERO" == "yes" ]]; then
  DECISION="Approved"
  DECISION_SUMMARY="Approved to cut a source-only public pre-release tag, subject to committing this decision record and using a clean working tree."
  TAG_GATE_STATUS="Pass"
else
  DECISION="Deferred"
  DECISION_SUMMARY="Public pre-release tag is deferred because final verification evidence is not fully passing."
  TAG_GATE_STATUS="Deferred"
fi

cat > "$DECISION_PATH" <<EOF
---
title: "First Public Pre-Release Tag Decision"
document_type: "release-decision"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-006
tags:
  - monad
  - release
  - public-prerelease
  - tag-decision
  - source-only
related:
  - README.md
  - docs/release/PUBLIC-CLAIMS-AUDIT.md
  - docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md
  - docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
  - docs/release/PUBLIC-PRERELEASE-NOTES.md
  - docs/release/PUBLIC-PRERELEASE-VERIFICATION-AUDIT.md
---

# First Public Pre-Release Tag Decision

## Status

Accepted.

## Work Packet

WP-E10-006 — Decide and cut first public pre-release tag, if approved.

## Decision

${DECISION}.

## Decision Summary

${DECISION_SUMMARY}

## Release Posture

Monad's first public pre-release posture remains:

\`\`\`text
Source-only public pre-release.
\`\`\`

This decision does not approve:

- binary artifact publication;
- installer publication;
- Crates.io publication;
- package-manager distribution;
- hosted service launch;
- SaaS launch;
- autonomous agent runtime claims;
- production-readiness claims.

## Verification Audit Dependency

The decision is based on:

\`\`\`text
${AUDIT_PATH}
\`\`\`

Verification audit status detected by this script:

| Field | Value |
| --- | --- |
| Audit Status | ${AUDIT_STATUS} |
| Failed Checks Zero | ${FAILED_CHECKS_ZERO} |
| Decision | ${DECISION} |

## Approved Tag Boundary

If the decision is Approved, the first public pre-release tag should use a pre-release-style tag name such as:

\`\`\`text
v0.1.0-public-prerelease.0
\`\`\`

The tag message should make the source-only boundary explicit:

\`\`\`text
Monad v0.1.0 public pre-release 0 — source-only
\`\`\`

## If Approved

The maintainer may cut the tag only after:

1. this decision record is committed;
2. the verification audit is committed;
3. release notes are committed;
4. the working tree is clean;
5. no final blockers remain.

Recommended command:

\`\`\`bash
git tag -a v0.1.0-public-prerelease.0 -m "Monad v0.1.0 public pre-release 0 — source-only"
git push origin v0.1.0-public-prerelease.0
\`\`\`

## If Deferred

If the decision is Deferred, do not cut a public pre-release tag.

Instead:

1. resolve the failing verification items;
2. rerun WP-E10-005;
3. rerun WP-E10-006;
4. create a new or updated decision record.

## Public Claim Boundary

Public-facing language may say:

\`\`\`text
source-only public pre-release
\`\`\`

Public-facing language must not imply:

\`\`\`text
general availability
production readiness
installer availability
package publication
hosted service availability
autonomous agent runtime
\`\`\`

## Definition of Done for WP-E10-006

WP-E10-006 is complete when:

- this decision record exists;
- the decision is explicit;
- if approved, the tag is cut only after the decision record is committed and the working tree is clean;
- if deferred, blockers are explicit;
- the E10 closeout state is clear.

## Outcome

${DECISION}.
EOF

cat > "$DELIVERABLE_PATH" <<EOF
---
title: "WP-E10-006 Public Pre-Release Tag Decision Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-006
tags:
  - monad
  - e10
  - release
  - public-prerelease
  - tag-decision
related:
  - docs/release/FIRST-PUBLIC-PRERELEASE-TAG-DECISION.md
  - docs/release/PUBLIC-PRERELEASE-VERIFICATION-AUDIT.md
  - docs/release/PUBLIC-PRERELEASE-NOTES.md
---

# WP-E10-006 Public Pre-Release Tag Decision Deliverable

## Work Packet

WP-E10-006 — Decide and cut first public pre-release tag, if approved.

## Outcome

${DECISION}.

## Summary

This work packet records the first public pre-release tag decision.

Detected final verification state:

| Field | Value |
| --- | --- |
| Audit Status | ${AUDIT_STATUS} |
| Failed Checks Zero | ${FAILED_CHECKS_ZERO} |
| Decision | ${DECISION} |

## Decision Record

\`\`\`text
docs/release/FIRST-PUBLIC-PRERELEASE-TAG-DECISION.md
\`\`\`

## Tag Status

EOF

if [[ "$DECISION" == "Approved" ]]; then
  cat >> "$DELIVERABLE_PATH" <<'EOF'
The release is approved to proceed to tag cutting only after this decision record is committed and the working tree is clean.

Recommended tag:

```text
v0.1.0-public-prerelease.0
```
EOF
else
  cat >> "$DELIVERABLE_PATH" <<'EOF'
The public pre-release tag is deferred.

Do not cut a public tag until verification blockers are resolved and WP-E10-005/WP-E10-006 are rerun.
EOF
fi

cat >> "$DELIVERABLE_PATH" <<'EOF'

## Recommended Commit

```bash
git commit -m "docs(release): decide first public prerelease tag"
```

## Closeout Note

WP-E10-006 is complete once this decision is committed and the corresponding GitHub work-packet issue or tracking item is closed.

If a tag is approved and cut, include the tag name in the issue closeout comment.
EOF

if [[ -f "$CHECKLIST_PATH" ]]; then
  python3 - <<PY
from pathlib import Path

path = Path("${CHECKLIST_PATH}")
text = path.read_text(encoding="utf-8")
status = "${TAG_GATE_STATUS}"
evidence = "`docs/release/FIRST-PUBLIC-PRERELEASE-TAG-DECISION.md`"
decision = "${DECISION}"

replacements = {
    "| Tag/release decision made | Pending | WP-E10-006 remains required |": f"| Tag/release decision made | {status} | {evidence} |",
    "| Public pre-release go/no-go decision is documented. | Pending | WP-E10-006 | Must happen after gates above are reviewed. |": f"| Public pre-release go/no-go decision is documented. | {status} | {evidence} | Decision: {decision}. |",
    "| Tag decision is explicit. | Pending | WP-E10-006 | Either cut tag or defer it. |": f"| Tag decision is explicit. | {status} | {evidence} | Decision: {decision}. |",
    "| If tag is cut, release evidence is attached. | Pending | WP-E10-006 | Required only if approved. |": f"| If tag is cut, release evidence is attached. | {'Pending' if decision == 'Approved' else 'Not Applicable'} | {evidence} | {'Tag cut remains a separate explicit command.' if decision == 'Approved' else 'Tag not approved.'} |",
    "| If tag is deferred, blockers are documented. | Pending | WP-E10-006 | Required if not approved. |": f"| If tag is deferred, blockers are documented. | {'Not Applicable' if decision == 'Approved' else 'Pass'} | {evidence} | {'Tag approved.' if decision == 'Approved' else 'Tag deferred by decision record.'} |",
    "WP-E10-006 — Decide and cut first public pre-release tag, if approved\\n": "",
}
for old, new in replacements.items():
    text = text.replace(old, new)

if decision == "Approved":
    text = text.replace("Current public pre-release status:\\n\\n```text\\nNot ready yet.\\n```", "Current public pre-release status:\\n\\n```text\\nApproved for source-only public pre-release tag after decision commit.\\n```")
    text = text.replace("Current readiness status:\\n\\n```text\\nNot ready for tag yet.\\n```", "Current readiness status:\\n\\n```text\\nApproved for source-only public pre-release tag after decision commit.\\n```")
else:
    text = text.replace("Current public pre-release status:\\n\\n```text\\nNot ready yet.\\n```", "Current public pre-release status:\\n\\n```text\\nDeferred.\\n```")
    text = text.replace("Current readiness status:\\n\\n```text\\nNot ready for tag yet.\\n```", "Current readiness status:\\n\\n```text\\nDeferred.\\n```")

path.write_text(text, encoding="utf-8")
PY
  echo "Updated ${CHECKLIST_PATH}"
fi

echo
echo "WP-E10-006 decision files written:"
echo "  ${DECISION_PATH}"
echo "  ${DELIVERABLE_PATH}"
echo
echo "Decision: ${DECISION}"
echo "Audit status: ${AUDIT_STATUS}"
echo "Failed checks zero: ${FAILED_CHECKS_ZERO}"

if [[ -n "$CUT_TAG" ]]; then
  echo
  echo "Tag cutting requested: ${CUT_TAG}"

  if [[ "$DECISION" != "Approved" ]]; then
    echo "Refusing to cut tag because decision is ${DECISION}." >&2
    exit 1
  fi

  if [[ -n "$(git status --short)" ]]; then
    echo "Refusing to cut tag because the working tree is not clean." >&2
    echo "Commit the decision and verification evidence first, then rerun with --cut-tag ${CUT_TAG}." >&2
    exit 1
  fi

  if git rev-parse "$CUT_TAG" >/dev/null 2>&1; then
    echo "Tag already exists: ${CUT_TAG}" >&2
    exit 1
  fi

  git tag -a "$CUT_TAG" -m "Monad ${CUT_TAG} — source-only public pre-release"
  echo "Created annotated tag: ${CUT_TAG}"
  echo "Push it with:"
  echo "  git push origin ${CUT_TAG}"
fi
