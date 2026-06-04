#!/usr/bin/env bash
set -euo pipefail

mkdir -p docs/06-adrs

cat > docs/06-adrs/README.md <<'README'
---
title: "Architecture Decision Records"
document_type: "adr-index"
status: accepted
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-06-03
version: 1.0.0
tags:
  - monad
  - architecture
  - adr
  - decisions
related:
  - docs/05-architecture/SYSTEM-OVERVIEW.md
  - docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
  - docs/05-architecture/MODULE-BOUNDARIES.md
  - docs/00-meta/DOCUMENTATION-STANDARD.md
  - docs/00-meta/FRONTMATTER-STANDARD.md
---

# Architecture Decision Records

## Purpose

This directory contains Monad's Architecture Decision Records.

ADRs record consequential decisions that affect Monad's product direction, architecture, implementation strategy, workflow, safety model, or long-term maintainability.

ADR files are part of the canonical project record.

## Why ADRs Matter

Monad is intended to be a repo-native, AI-readable, verification-oriented software foundry.

That requires durable decision memory.

Without ADRs, important reasoning gets lost in chat history, issue comments, private notes, implicit assumptions, stale docs, and memory.

ADRs prevent that by recording what decision was made, why it was made, what alternatives were considered, what consequences follow, and when the decision may need review.

## ADR Naming

ADR files use this format:

```text
ADR-NNNN-short-kebab-case-title.md
```

The ADR number is permanent once assigned. Do not renumber ADRs after they are committed.

## ADR Statuses

ADR frontmatter uses the standard documentation statuses:

```text
stub
draft
review
accepted
superseded
archived
```

In the ADR body, also include a visible `Status` section.

Preferred ADR lifecycle:

```text
draft -> review -> accepted
accepted -> superseded
draft -> archived
```

## ADR Index

| ADR      | Title                                            | Status   | Summary                                                                       |
| -------- | ------------------------------------------------ | -------- | ----------------------------------------------------------------------------- |
| ADR-0000 | Template                                         | accepted | Defines the standard structure for Monad ADRs.                                |
| ADR-0001 | Use Rust for Core Runtime                        | accepted | Monad's durable local runtime and CLI foundation will be implemented in Rust. |
| ADR-0002 | Use Monad as Unified Product Name                | accepted | Monad is the canonical umbrella/product name for the consolidated system.     |
| ADR-0003 | Use Repo-Native Context as Source of Truth       | accepted | Repository-owned context and handoff artifacts are canonical project memory.  |
| ADR-0004 | Use Work Packets as Primary Delivery Unit        | accepted | Work packets are Monad's primary atomic delivery and verification unit.       |
| ADR-0005 | Use Multi-Crate Rust Workspace                   | accepted | Monad uses a multi-crate Rust workspace with durable crate boundaries.         |
| ADR-0006 | Keep CLI Thin and Core Durable                   | accepted | CLI code remains thin while product logic lives in durable core modules.      |
| ADR-0007 | Use Supervised Autonomy for Agent Workflows      | accepted | AI-assisted workflows must remain human-supervised and approval-gated.        |
| ADR-0008 | Coordinate Native Tools Rather Than Replace Them | accepted | Monad coordinates ecosystem-native tools instead of replacing them wholesale. |

## When to Write an ADR

Write an ADR when a decision affects core technology choices, crate boundaries, module boundaries, command architecture, file operation safety, command execution safety, AI/provider architecture, context model, verification model, evolution workflow, project workflow, public product identity, or long-term maintainability.

Do not write an ADR for every small implementation detail.

## ADR Review Questions

Before accepting an ADR, ask:

- Is the decision stated clearly?
- Is the context accurate?
- Are the alternatives represented fairly?
- Are consequences explicit?
- Does the decision align with Monad's product vision?
- Does the decision align with architecture principles?
- Does the decision protect safety and verification?
- Is the decision reversible or difficult to reverse?
- Does implementation need to change because of this ADR?
- Do related docs need to be updated?

## ADR Change Rule

Accepted ADRs should not be casually edited to change the decision.

Minor editorial fixes are acceptable.

Material changes should usually be handled by writing a new ADR, marking the old ADR as `superseded`, and linking the old and new ADRs.

## Current Status

This ADR index is accepted as the baseline ADR foundation for Monad.

WP-E0-006 is complete when this directory contains a durable ADR index, an accepted ADR template, accepted foundational ADR records, and a verification script that checks ADR record structure.
README

write_adr() {
  local path="$1"
  local number="$2"
  local title="$3"
  local decision="$4"
  local rationale="$5"
  local consequences="$6"
  cat > "$path" <<ADR
---
title: "ADR ${number}: ${title}"
document_type: "adr"
status: accepted
owner: "Thomas Carter"
created: 2026-06-03
updated: 2026-06-03
version: 1.0.0
tags:
  - monad
  - adr
related:
  - docs/06-adrs/README.md
---

# ADR ${number}: ${title}

## Status

Accepted.

## Context

Monad needs durable foundational decisions that are kept in the repository rather than scattered across chat history, private notes, or issue comments.

This ADR records one of the baseline project decisions needed to complete the E0 ADR foundation.

## Decision

${decision}

## Rationale

${rationale}

## Alternatives Considered

### Leave the decision implicit

Rejected because implicit decisions are difficult to review, cite, verify, and preserve across sessions.

### Record the decision only in issue comments

Rejected because issue comments are useful operational history but do not replace repo-native architectural decision records.

## Consequences

${consequences}

## Implementation Notes

Future implementation work should follow this decision unless a later accepted ADR supersedes it.

## Related Documents

- \\`docs/06-adrs/README.md\\`

## Review / Supersession Notes

Revisit this ADR if implementation experience shows that the decision creates avoidable friction or if a later project phase requires a more precise policy.
ADR
}

write_adr docs/06-adrs/ADR-0003-use-repo-native-context-as-source-of-truth.md "0003" "Use Repo-Native Context as Source of Truth" \
"Monad will treat repo-native context artifacts as the source of truth for project memory and handoff state." \
"Repo-native context is durable, reviewable, versioned, citeable, portable, AI-readable, and resilient across chat/session boundaries." \
"Context artifacts require maintenance, generated context must be clearly identified, and stale context can mislead if it is not verified."

write_adr docs/06-adrs/ADR-0004-use-work-packets-as-primary-delivery-unit.md "0004" "Use Work Packets as Primary Delivery Unit" \
"Monad will use work packets as the primary delivery unit between epics and tasks." \
"Work packets are the right granularity for scoped implementation, verification, commits, closeout, and AI-assisted handoff." \
"The project must maintain work packet records and prevent issue records, repo records, and context records from drifting."

write_adr docs/06-adrs/ADR-0005-use-multi-crate-rust-workspace.md "0005" "Use Multi-Crate Rust Workspace" \
"Monad will use a multi-crate Rust workspace with at least crates/monad-cli and crates/monad-core." \
"A multi-crate workspace separates command-line concerns from durable product logic and leaves room for future boundaries." \
"Workspace structure is slightly more complex and dependency direction must be enforced."

write_adr docs/06-adrs/ADR-0006-keep-cli-thin-and-core-durable.md "0006" "Keep CLI Thin and Core Durable" \
"Monad will keep CLI code thin and place durable product logic in monad-core." \
"This keeps product behavior testable, reusable, and available to future command surfaces beyond the CLI." \
"Developers must resist placing durable business logic directly in CLI handlers."

write_adr docs/06-adrs/ADR-0007-use-supervised-autonomy-for-agent-workflows.md "0007" "Use Supervised Autonomy for Agent Workflows" \
"Monad will use supervised autonomy for AI-assisted workflows, keeping the human in command." \
"This provides AI assistance while preserving trust, reviewability, explicit approval, and verification evidence." \
"The workflow has more review steps and cannot claim fully autonomous execution before policy and approval gates exist."

write_adr docs/06-adrs/ADR-0008-coordinate-native-tools-rather-than-replace-them.md "0008" "Coordinate Native Tools Rather Than Replace Them" \
"Monad will coordinate ecosystem-native tools rather than replacing them wholesale." \
"This reduces scope, preserves compatibility with real repositories, avoids lock-in, and enables a faster MVP path." \
"Monad must handle variation across ecosystems and provide good diagnostics when native tools are missing or misconfigured."

cat > tools/scripts/check-adr-records.py <<'PY'
#!/usr/bin/env python3
"""
Check Monad ADR records for baseline structure.

This checker is intentionally lightweight and dependency-free. It verifies that
ADR records exist, have YAML frontmatter, use the expected filename shape, and
contain the core sections needed for durable architectural decision records.
"""

from pathlib import Path
import re

ADR_DIR = Path("docs/06-adrs")

REQUIRED_PATHS = [
    ADR_DIR / "README.md",
    ADR_DIR / "ADR-0000-template.md",
    ADR_DIR / "ADR-0001-use-rust-for-core-runtime.md",
    ADR_DIR / "ADR-0002-use-monad-as-unified-product-name.md",
    ADR_DIR / "ADR-0003-use-repo-native-context-as-source-of-truth.md",
    ADR_DIR / "ADR-0004-use-work-packets-as-primary-delivery-unit.md",
    ADR_DIR / "ADR-0005-use-multi-crate-rust-workspace.md",
    ADR_DIR / "ADR-0006-keep-cli-thin-and-core-durable.md",
    ADR_DIR / "ADR-0007-use-supervised-autonomy-for-agent-workflows.md",
    ADR_DIR / "ADR-0008-coordinate-native-tools-rather-than-replace-them.md",
]

ADR_FILENAME_PATTERN = re.compile(r"^ADR-\d{4}-[a-z0-9][a-z0-9-]*\.md$")

REQUIRED_FRONTMATTER_KEYS = [
    "title:",
    "document_type:",
    "status:",
    "version:",
    "created:",
    "updated:",
]

REQUIRED_NON_TEMPLATE_SECTIONS = [
    "## Status",
    "## Context",
    "## Decision",
    "## Consequences",
]


def split_frontmatter(text: str) -> tuple[str, str] | None:
    if not text.startswith("---\n"):
        return None

    marker = "\n---\n"
    end = text.find(marker, len("---\n"))
    if end == -1:
        return None

    frontmatter = text[len("---\n"):end]
    body = text[end + len(marker):]
    return frontmatter, body


def main() -> int:
    failures: list[str] = []

    if not ADR_DIR.exists():
        print(f"ADR directory does not exist: {ADR_DIR}")
        return 1

    for required_path in REQUIRED_PATHS:
        if not required_path.exists():
            failures.append(f"Missing required ADR path: {required_path}")

    adr_files = sorted(ADR_DIR.glob("ADR-*.md"))
    if not adr_files:
        failures.append("No ADR files found under docs/06-adrs/.")

    for path in adr_files:
        if not ADR_FILENAME_PATTERN.match(path.name):
            failures.append(f"{path}: ADR filename does not match ADR-0000-kebab-case.md")

        text = path.read_text(encoding="utf-8")
        split = split_frontmatter(text)
        if split is None:
            failures.append(f"{path}: missing or malformed YAML frontmatter")
            continue

        frontmatter, body = split
        for key in REQUIRED_FRONTMATTER_KEYS:
            if key not in frontmatter:
                failures.append(f"{path}: frontmatter missing {key}")

        if "# " not in body:
            failures.append(f"{path}: missing top-level Markdown heading")

        if path.name.startswith("ADR-0000-"):
            continue

        for section in REQUIRED_NON_TEMPLATE_SECTIONS:
            if f"\n{section}\n" not in body:
                failures.append(f"{path}: missing required ADR section {section}")

    if failures:
        print("ADR record check failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("All ADR records satisfy the required baseline structure.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
PY

chmod +x tools/scripts/check-adr-records.py
python3 tools/scripts/check-adr-records.py
