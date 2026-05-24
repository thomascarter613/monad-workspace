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
    "monad.toml",
    "crates/monad-cli/Cargo.toml",
    "crates/monad-cli/src/main.rs",
    "crates/monad-core/Cargo.toml",
    "crates/monad-core/src/lib.rs",
    "crates/monad-core/src/diagnostics.rs",
    "crates/monad-core/src/error.rs",
    "crates/monad-core/src/workspace.rs",
    "crates/monad-core/src/manifest.rs",

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
    ".monad/context/work-packet-handoffs/WP-E1-003.md",
    ".monad/context/work-packet-handoffs/WP-E1-004.md",
    ".monad/context/work-packet-handoffs/WP-E1-005.md",
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
    "work/packets/E1/WP-E1-003-establish-core-error-foundation.md",
    "work/packets/E1/WP-E1-004-establish-workspace-context-foundation.md",
    "work/packets/E1/WP-E1-005-establish-manifest-model-foundation.md",
    "work/tasks/E1/README.md",
    "work/tasks/E1/T-WP-E1-001-001-create-rust-workspace-crates.md",
    "work/tasks/E1/T-WP-E1-001-002-add-minimal-core-runtime-identity.md",
    "work/tasks/E1/T-WP-E1-001-003-add-thin-cli-entrypoint.md",
    "work/tasks/E1/T-WP-E1-001-004-add-rust-verification-to-baseline.md",
    "work/tasks/E1/T-WP-E1-002-001-add-diagnostics-module.md",
    "work/tasks/E1/T-WP-E1-002-002-export-diagnostics-from-core-runtime.md",
    "work/tasks/E1/T-WP-E1-002-003-update-e1-records-and-context.md",
    "work/tasks/E1/T-WP-E1-003-001-add-core-error-module.md",
    "work/tasks/E1/T-WP-E1-003-002-export-core-error-model.md",
    "work/tasks/E1/T-WP-E1-003-003-update-e1-records-and-context.md",
    "work/tasks/E1/T-WP-E1-004-001-add-workspace-context-module.md",
    "work/tasks/E1/T-WP-E1-004-002-export-workspace-context-from-core-runtime.md",
    "work/tasks/E1/T-WP-E1-004-003-update-e1-records-and-context.md",
    "work/tasks/E1/T-WP-E1-005-001-add-root-monad-manifest.md",
    "work/tasks/E1/T-WP-E1-005-002-add-manifest-model-module.md",
    "work/tasks/E1/T-WP-E1-005-003-export-manifest-model-from-core-runtime.md",
    "work/tasks/E1/T-WP-E1-005-004-update-e1-records-and-context.md",
    "work/deliverables/E1/README.md",
    "work/deliverables/E1/D-WP-E1-001-001-rust-workspace-manifest.md",
    "work/deliverables/E1/D-WP-E1-001-002-core-runtime-library.md",
    "work/deliverables/E1/D-WP-E1-001-003-thin-cli-entrypoint.md",
    "work/deliverables/E1/D-WP-E1-002-001-diagnostics-module.md",
    "work/deliverables/E1/D-WP-E1-002-002-core-runtime-exports.md",
    "work/deliverables/E1/D-WP-E1-002-003-diagnostics-context-handoff.md",
    "work/deliverables/E1/D-WP-E1-003-001-core-error-module.md",
    "work/deliverables/E1/D-WP-E1-003-002-core-error-exports.md",
    "work/deliverables/E1/D-WP-E1-003-003-core-error-context-handoff.md",
    "work/deliverables/E1/D-WP-E1-004-001-workspace-context-module.md",
    "work/deliverables/E1/D-WP-E1-004-002-workspace-context-exports.md",
    "work/deliverables/E1/D-WP-E1-004-003-workspace-context-handoff.md",
    "work/deliverables/E1/D-WP-E1-005-001-root-monad-manifest.md",
    "work/deliverables/E1/D-WP-E1-005-002-manifest-model-module.md",
    "work/deliverables/E1/D-WP-E1-005-003-manifest-model-exports.md",
    "work/deliverables/E1/D-WP-E1-005-004-manifest-model-handoff.md",
    "crates/monad-core/src/checks.rs",
    ".monad/context/work-packet-handoffs/WP-E1-008.md",
    "work/packets/E1/WP-E1-008-establish-cli-check-command-foundation.md",
    "work/tasks/E1/T-WP-E1-008-001-add-core-workspace-checks.md",
    "work/tasks/E1/T-WP-E1-008-002-add-cli-check-command.md",
    "work/tasks/E1/T-WP-E1-008-003-update-e1-records-and-context.md",
    "work/deliverables/E1/D-WP-E1-008-001-core-workspace-checks.md",
    "work/deliverables/E1/D-WP-E1-008-002-cli-check-command.md",
    "work/deliverables/E1/D-WP-E1-008-003-cli-check-verification.md",
    "work/deliverables/E1/D-WP-E1-008-004-cli-check-handoff.md",
    "crates/monad-core/src/repo_contract.rs",
    ".monad/context/work-packet-handoffs/WP-E1-009.md",
    "work/packets/E1/WP-E1-009-establish-repository-contract-check-foundation.md",
    "work/tasks/E1/T-WP-E1-009-001-add-repository-contract-module.md",
    "work/tasks/E1/T-WP-E1-009-002-integrate-contract-checks-with-monad-check.md",
    "work/tasks/E1/T-WP-E1-009-003-update-e1-records-and-context.md",
    "work/deliverables/E1/D-WP-E1-009-001-repository-contract-module.md",
    "work/deliverables/E1/D-WP-E1-009-002-checks-integration.md",
    "work/deliverables/E1/D-WP-E1-009-003-repository-contract-handoff.md",
    "crates/monad-core/src/output.rs",
    ".monad/context/work-packet-handoffs/WP-E1-010.md",
    "work/packets/E1/WP-E1-010-establish-runtime-output-formatting-foundation.md",
    "work/tasks/E1/T-WP-E1-010-001-add-output-formatting-module.md",
    "work/tasks/E1/T-WP-E1-010-002-use-output-formatting-in-cli.md",
    "work/tasks/E1/T-WP-E1-010-003-update-e1-records-and-context.md",
    "work/deliverables/E1/D-WP-E1-010-001-output-formatting-module.md",
    "work/deliverables/E1/D-WP-E1-010-002-cli-output-integration.md",
    "work/deliverables/E1/D-WP-E1-010-003-output-formatting-handoff.md",
    ".monad/context/work-packet-handoffs/WP-E1-011.md",
    "work/packets/E1/WP-E1-011-establish-cli-output-format-argument-foundation.md",
    "work/tasks/E1/T-WP-E1-011-001-add-cli-output-format-parsing.md",
    "work/tasks/E1/T-WP-E1-011-002-wire-output-format-into-cli-commands.md",
    "work/tasks/E1/T-WP-E1-011-003-update-e1-records-and-context.md",
    "work/deliverables/E1/D-WP-E1-011-001-cli-output-format-argument.md",
    "work/deliverables/E1/D-WP-E1-011-002-cli-output-format-tests.md",
    "work/deliverables/E1/D-WP-E1-011-003-cli-output-format-handoff.md",
    ".monad/context/work-packet-handoffs/WP-E1-013.md",
    ".monad/context/work-packet-handoffs/WP-E2-001.md",
    "work/epics/E2-repository-intelligence-foundation.md",
    "work/packets/E2/README.md",
    "work/packets/E2/WP-E2-001-establish-repository-inspection-foundation.md",
    "work/packets/E1/WP-E1-013-close-e1-and-prepare-e2-handoff.md",
    "work/tasks/E1/T-WP-E1-013-001-close-e1-records.md",
    "work/tasks/E1/T-WP-E1-013-002-update-runtime-context-handoff.md",
    "work/tasks/E1/T-WP-E1-013-003-create-e2-starting-point.md",
    "work/deliverables/E1/D-WP-E1-013-001-e1-closure-record.md",
    "work/deliverables/E1/D-WP-E1-013-002-e2-starting-point.md",
    "work/deliverables/E1/D-WP-E1-013-003-e2-context-handoff.md",
    "crates/monad-core/src/repository_inspection.rs",
    "work/tasks/E2/README.md",
    "work/tasks/E2/T-WP-E2-001-001-add-repository-inspection-module.md",
    "work/tasks/E2/T-WP-E2-001-002-export-repository-inspection-types.md",
    "work/tasks/E2/T-WP-E2-001-003-integrate-inspection-with-workspace-checks.md",
    "work/tasks/E2/T-WP-E2-001-004-update-e2-records-and-context.md",
    "work/deliverables/E2/README.md",
    "work/deliverables/E2/D-WP-E2-001-001-repository-inspection-module.md",
    "work/deliverables/E2/D-WP-E2-001-002-repository-inspection-exports.md",
    "work/deliverables/E2/D-WP-E2-001-003-workspace-check-integration.md",
    "work/deliverables/E2/D-WP-E2-001-004-repository-inspection-handoff.md",
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
