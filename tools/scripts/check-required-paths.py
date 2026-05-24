#!/usr/bin/env python3
"""
Check that the required foundation, E1 handoff, and Rust runtime paths exist.
"""

from pathlib import Path


REQUIRED_PATHS = [
    # Root and Rust workspace
    "README.md",
    "LICENSE",
    ".gitignore",
    ".editorconfig",
    "rust-toolchain.toml",
    "Cargo.toml",
    "crates/monad-cli/Cargo.toml",
    "crates/monad-cli/src/main.rs",
    "crates/monad-core/Cargo.toml",
    "crates/monad-core/src/lib.rs",
    "crates/monad-core/src/diagnostics.rs",

    # Documentation and work roots
    "docs/README.md",
    "work/README.md",
    "work/epics/README.md",
    "work/packets/README.md",
    "work/tasks/README.md",
    "work/records/README.md",
    ".monad/README.md",
    ".monad/context/README.md",
    ".monad/reports/README.md",

    # Context bridge
    "docs/09-ai/CURRENT-STATE.md",
    "docs/09-ai/FRESH-CHAT-HANDOFF.md",
    ".monad/context/current-state.md",
    ".monad/context/latest-handoff.md",
    ".monad/context/latest-context-pack.md",
    ".monad/context/decision-log.md",
    ".monad/context/session-chronicles/README.md",
    ".monad/context/work-packet-handoffs/README.md",
    ".monad/context/work-packet-handoffs/WP-E1-001.md",
    ".monad/context/work-packet-handoffs/WP-E1-002.md",
    ".monad/context/decision-records/README.md",

    # Workflow standards
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

    # E1 records
    "work/epics/E1-runtime-foundation.md",
    "work/packets/E1/README.md",
    "work/packets/E1/WP-E1-001-establish-rust-workspace-runtime-foundation.md",
    "work/packets/E1/WP-E1-002-establish-core-diagnostics-foundation.md",
    "work/tasks/E1/README.md",
    "work/tasks/E1/T-WP-E1-001-001-create-rust-workspace-crates.md",
    "work/tasks/E1/T-WP-E1-001-002-add-minimal-core-runtime-identity.md",
    "work/tasks/E1/T-WP-E1-001-003-add-thin-cli-entrypoint.md",
    "work/tasks/E1/T-WP-E1-001-004-add-rust-verification-to-baseline.md",
    "work/tasks/E1/T-WP-E1-002-001-add-diagnostics-module.md",
    "work/tasks/E1/T-WP-E1-002-002-export-diagnostics-from-core-runtime.md",
    "work/tasks/E1/T-WP-E1-002-003-update-e1-records-and-context.md",
    "work/deliverables/E1/README.md",
    "work/deliverables/E1/D-WP-E1-001-001-rust-workspace-manifest.md",
    "work/deliverables/E1/D-WP-E1-001-002-core-runtime-library.md",
    "work/deliverables/E1/D-WP-E1-001-003-thin-cli-entrypoint.md",
    "work/deliverables/E1/D-WP-E1-002-001-diagnostics-module.md",
    "work/deliverables/E1/D-WP-E1-002-002-core-runtime-exports.md",
    "work/deliverables/E1/D-WP-E1-002-003-diagnostics-context-handoff.md",
]


def main() -> int:
    missing: list[str] = []

    for path_text in REQUIRED_PATHS:
        path = Path(path_text)
        if not path.exists():
            missing.append(path_text)

    if missing:
        print("Required foundation/runtime paths are missing:")
        for item in missing:
            print(f"  {item}")
        return 1

    print("All required foundation and runtime paths exist.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
