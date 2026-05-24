#!/usr/bin/env python3
"""
Check that the required E0 foundation and E1 handoff paths exist.
"""

from pathlib import Path


REQUIRED_PATHS = [
    # WP-E0-001 — Repository foundation
    "README.md",
    "LICENSE",
    ".gitignore",
    ".editorconfig",
    "rust-toolchain.toml",
    "Cargo.toml",
    "docs/README.md",
    "work/README.md",
    "work/epics/README.md",
    "work/packets/README.md",
    "work/tasks/README.md",
    "work/records/README.md",
    ".monad/README.md",
    ".monad/context/README.md",
    ".monad/reports/README.md",

    # WP-E0-003 — Context bridge foundation
    "docs/09-ai/CURRENT-STATE.md",
    "docs/09-ai/FRESH-CHAT-HANDOFF.md",
    ".monad/context/current-state.md",
    ".monad/context/latest-handoff.md",
    ".monad/context/latest-context-pack.md",
    ".monad/context/decision-log.md",
    ".monad/context/session-chronicles/README.md",
    ".monad/context/work-packet-handoffs/README.md",
    ".monad/context/decision-records/README.md",

    # WP-E0-004 — Workflow standards
    "docs/07-workflow/OPERATING-MODEL.md",
    "docs/07-workflow/WORK-HIERARCHY.md",
    "docs/07-workflow/WORK-PACKET-STANDARD.md",
    "docs/07-workflow/DEFINITION-OF-READY.md",
    "docs/07-workflow/DEFINITION-OF-DONE.md",
    "docs/07-workflow/README.md",
    "docs/07-workflow/EPIC-STANDARD.md",
    "docs/07-workflow/TASK-STANDARD.md",
    "docs/07-workflow/DELIVERABLE-STANDARD.md",
    "docs/07-workflow/VERIFICATION-STANDARD.md",
    "docs/07-workflow/COMMIT-STANDARD.md",
    "docs/07-workflow/BRANCHING-STANDARD.md",
    "docs/07-workflow/REVIEW-STANDARD.md",
    "docs/07-workflow/CONTEXT-UPDATE-STANDARD.md",

    # Verification scripts
    "tools/scripts/verify.sh",
    "tools/scripts/check-required-paths.py",
    "tools/scripts/check-markdown-frontmatter.py",
    "tools/scripts/check-work-records.py",
    "tools/scripts/check-task-records.py",
    "tools/scripts/check-deliverable-records.py",
    "tools/scripts/check-epic-records.py",
    "tools/scripts/check-adr-records.py",
    "tools/scripts/check-context-records.py",
    "docs/12-verification/VERIFICATION-BASELINE.md",

    # ADRs
    "docs/06-adrs/README.md",
    "docs/06-adrs/ADR-0000-template.md",
    "docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md",
    "docs/06-adrs/ADR-0002-use-monad-as-unified-product-name.md",

    # E0 records
    "work/epics/E0-project-foundation.md",
    "work/packets/E0/README.md",
    "work/packets/E0/WP-E0-001-establish-repository-foundation.md",
    "work/packets/E0/WP-E0-002-establish-documentation-architecture.md",
    "work/packets/E0/WP-E0-003-establish-context-bridge-foundation.md",
    "work/packets/E0/WP-E0-004-establish-workflow-standards.md",
    "work/packets/E0/WP-E0-005-establish-verification-baseline.md",
    "work/packets/E0/WP-E0-006-establish-work-packet-records.md",
    "work/packets/E0/WP-E0-007-establish-adr-verification.md",
    "work/packets/E0/WP-E0-008-establish-epic-record-verification.md",
    "work/packets/E0/WP-E0-009-establish-task-record-foundation.md",
    "work/packets/E0/WP-E0-010-establish-deliverable-record-foundation.md",
    "work/packets/E0/WP-E0-011-close-e0-and-prepare-e1-handoff.md",

    # E0 task records
    "work/tasks/E0/README.md",
    "work/tasks/E0/T-WP-E0-009-001-create-task-record-directory-and-index.md",
    "work/tasks/E0/T-WP-E0-009-002-add-task-record-verification.md",
    "work/tasks/E0/T-WP-E0-009-003-update-e0-planning-and-verification-records.md",
    "work/tasks/E0/T-WP-E0-010-001-create-deliverable-record-directory-and-index.md",
    "work/tasks/E0/T-WP-E0-010-002-add-deliverable-record-verification.md",
    "work/tasks/E0/T-WP-E0-010-003-update-e0-planning-and-verification-records.md",
    "work/tasks/E0/T-WP-E0-011-001-close-e0-records.md",
    "work/tasks/E0/T-WP-E0-011-002-update-context-handoff.md",
    "work/tasks/E0/T-WP-E0-011-003-create-e1-starting-point.md",

    # E0 deliverable records
    "work/deliverables/E0/README.md",
    "work/deliverables/E0/D-WP-E0-010-001-deliverable-record-index.md",
    "work/deliverables/E0/D-WP-E0-010-002-deliverable-record-verifier.md",
    "work/deliverables/E0/D-WP-E0-010-003-verification-baseline-update.md",
    "work/deliverables/E0/D-WP-E0-011-001-e0-closure-record.md",
    "work/deliverables/E0/D-WP-E0-011-002-e1-starting-point.md",
    "work/deliverables/E0/D-WP-E0-011-003-context-handoff-update.md",

    # E1 handoff records
    "work/epics/E1-runtime-foundation.md",
    "work/packets/E1/README.md",
    "work/packets/E1/WP-E1-001-establish-rust-workspace-runtime-foundation.md",
    ".monad/context/work-packet-handoffs/WP-E1-001.md",
]


def main() -> int:
    missing: list[str] = []

    for path_text in REQUIRED_PATHS:
        path = Path(path_text)
        if not path.exists():
            missing.append(path_text)

    if missing:
        print("Required foundation/handoff paths are missing:")
        for item in missing:
            print(f"  {item}")
        return 1

    print("All required foundation and E1 handoff paths exist.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
