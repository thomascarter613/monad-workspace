---
title: "E1 Deliverable Records"
document_type: "deliverable-index"
status: "draft"
version: "0.10.0"
created: "2026-05-23"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
tags:

* deliverables
* runtime
* rust
* index

---

# E1 Deliverable Records

This directory contains durable deliverable records for E1 — Runtime Foundation.

## Current Deliverable Records

| Deliverable     | Source Work Packet | Artifact Path                                      | Status      |
| --------------- | ------------------ | -------------------------------------------------- | ----------- |
| D-WP-E1-001-001 | WP-E1-001          | `Cargo.toml`                                       | Complete    |
| D-WP-E1-001-002 | WP-E1-001          | `crates/monad-core/src/lib.rs`                     | Complete    |
| D-WP-E1-001-003 | WP-E1-001          | `crates/monad-cli/src/main.rs`                     | Complete    |
| D-WP-E1-002-001 | WP-E1-002          | `crates/monad-core/src/diagnostics.rs`             | Complete    |
| D-WP-E1-002-002 | WP-E1-002          | `crates/monad-core/src/lib.rs`                     | Complete    |
| D-WP-E1-002-003 | WP-E1-002          | `.monad/context/work-packet-handoffs/WP-E1-002.md` | Complete    |
| D-WP-E1-003-001 | WP-E1-003          | `crates/monad-core/src/error.rs`                   | Complete    |
| D-WP-E1-003-002 | WP-E1-003          | `crates/monad-core/src/lib.rs`                     | Complete    |
| D-WP-E1-003-003 | WP-E1-003          | `.monad/context/work-packet-handoffs/WP-E1-003.md` | Complete    |
| D-WP-E1-004-001 | WP-E1-004          | `crates/monad-core/src/workspace.rs`               | Complete    |
| D-WP-E1-004-002 | WP-E1-004          | `crates/monad-core/src/lib.rs`                     | Complete    |
| D-WP-E1-004-003 | WP-E1-004          | `.monad/context/work-packet-handoffs/WP-E1-004.md` | Complete    |
| D-WP-E1-005-001 | WP-E1-005          | `monad.toml`                                       | Complete    |
| D-WP-E1-005-002 | WP-E1-005          | `crates/monad-core/src/manifest.rs`                | Complete    |
| D-WP-E1-005-003 | WP-E1-005          | `crates/monad-core/src/lib.rs`                     | Complete    |
| D-WP-E1-005-004 | WP-E1-005          | `.monad/context/work-packet-handoffs/WP-E1-005.md` | Complete    |
| D-WP-E1-006-001 | WP-E1-006          | `crates/monad-core/Cargo.toml`                     | Complete    |
| D-WP-E1-006-002 | WP-E1-006          | `crates/monad-core/src/manifest.rs`                | Complete    |
| D-WP-E1-006-003 | WP-E1-006          | `crates/monad-core/src/lib.rs`                     | Complete    |
| D-WP-E1-006-004 | WP-E1-006          | `.monad/context/work-packet-handoffs/WP-E1-006.md` | Complete    |
| D-WP-E1-007-001 | WP-E1-007          | `crates/monad-cli/src/main.rs`                     | Complete    |
| D-WP-E1-007-002 | WP-E1-007          | `tools/scripts/verify.sh`                          | Complete    |
| D-WP-E1-007-003 | WP-E1-007          | `.monad/context/work-packet-handoffs/WP-E1-007.md` | In Progress |
|                 |                    |                                                    |             |

cat > tools/scripts/check-required-paths.py <<'PY'
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
    "Cargo.lock",
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
    ".monad/context/work-packet-handoffs/WP-E1-006.md",
    ".monad/context/work-packet-handoffs/WP-E1-007.md",
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

    # E1 work packets
    "work/epics/E1-runtime-foundation.md",
    "work/packets/E1/README.md",
    "work/packets/E1/WP-E1-001-establish-rust-workspace-runtime-foundation.md",
    "work/packets/E1/WP-E1-002-establish-core-diagnostics-foundation.md",
    "work/packets/E1/WP-E1-003-establish-core-error-foundation.md",
    "work/packets/E1/WP-E1-004-establish-workspace-context-foundation.md",
    "work/packets/E1/WP-E1-005-establish-manifest-model-foundation.md",
    "work/packets/E1/WP-E1-006-establish-manifest-loading-foundation.md",
    "work/packets/E1/WP-E1-007-establish-cli-info-command-foundation.md",

    # E1 tasks
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
    "work/tasks/E1/T-WP-E1-006-001-add-manifest-parsing-dependencies.md",
    "work/tasks/E1/T-WP-E1-006-002-add-manifest-loading.md",
    "work/tasks/E1/T-WP-E1-006-003-update-e1-records-and-context.md",
    "work/tasks/E1/T-WP-E1-007-001-add-cli-command-parser.md",
    "work/tasks/E1/T-WP-E1-007-002-add-cli-info-rendering.md",
    "work/tasks/E1/T-WP-E1-007-003-update-e1-records-and-context.md",

    # E1 deliverables
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
    "work/deliverables/E1/D-WP-E1-006-001-manifest-parsing-dependencies.md",
    "work/deliverables/E1/D-WP-E1-006-002-manifest-loading-runtime.md",
    "work/deliverables/E1/D-WP-E1-006-003-manifest-loading-exports.md",
    "work/deliverables/E1/D-WP-E1-006-004-manifest-loading-handoff.md",
    "work/deliverables/E1/D-WP-E1-007-001-cli-info-command.md",
    "work/deliverables/E1/D-WP-E1-007-002-cli-info-verification.md",
    "work/deliverables/E1/D-WP-E1-007-003-cli-info-handoff.md",
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
PY

cat > tools/scripts/check-context-records.py <<'PY'
#!/usr/bin/env python3
"""
Check Monad context records for E1 runtime handoff readiness.
"""

from pathlib import Path


REQUIRED_CONTEXT_FILES = [
    Path("docs/09-ai/CURRENT-STATE.md"),
    Path("docs/09-ai/FRESH-CHAT-HANDOFF.md"),
    Path(".monad/context/current-state.md"),
    Path(".monad/context/latest-handoff.md"),
    Path(".monad/context/latest-context-pack.md"),
    Path(".monad/context/decision-log.md"),
    Path(".monad/context/work-packet-handoffs/WP-E1-001.md"),
    Path(".monad/context/work-packet-handoffs/WP-E1-002.md"),
    Path(".monad/context/work-packet-handoffs/WP-E1-003.md"),
    Path(".monad/context/work-packet-handoffs/WP-E1-004.md"),
    Path(".monad/context/work-packet-handoffs/WP-E1-005.md"),
    Path(".monad/context/work-packet-handoffs/WP-E1-006.md"),
    Path(".monad/context/work-packet-handoffs/WP-E1-007.md"),
]

GLOBAL_REQUIRED_TERMS = [
    "E0",
    "E1",
    "WP-E1-001",
    "WP-E1-002",
    "WP-E1-003",
    "WP-E1-004",
    "WP-E1-005",
    "WP-E1-006",
    "WP-E1-007",
    "Runtime Foundation",
    "Core Diagnostics",
    "Core Error",
    "Workspace Context",
    "Manifest Model",
    "Manifest Loading",
    "CLI Info",
]

CURRENT_CONTEXT_FILES = [
    Path("docs/09-ai/CURRENT-STATE.md"),
    Path("docs/09-ai/FRESH-CHAT-HANDOFF.md"),
    Path(".monad/context/current-state.md"),
    Path(".monad/context/latest-handoff.md"),
    Path(".monad/context/latest-context-pack.md"),
    Path(".monad/context/decision-log.md"),
]


def has_frontmatter(text: str) -> bool:
    return text.startswith("---\n") and "\n---\n" in text[len("---\n"):]


def main() -> int:
    failures: list[str] = []
    combined_text_parts: list[str] = []

    for path in REQUIRED_CONTEXT_FILES:
        if not path.exists():
            failures.append(f"missing context file: {path}")
            continue

        text = path.read_text(encoding="utf-8")
        combined_text_parts.append(text)

        if not has_frontmatter(text):
            failures.append(f"{path}: missing or malformed YAML frontmatter")

    combined_text = "\n".join(combined_text_parts)

    for term in GLOBAL_REQUIRED_TERMS:
        if term not in combined_text:
            failures.append(f"context corpus missing required handoff term {term}")

    for path in CURRENT_CONTEXT_FILES:
        if not path.exists():
            continue

        text = path.read_text(encoding="utf-8")
        for term in ["E1", "WP-E1-007", "Runtime Foundation", "CLI Info"]:
            if term not in text:
                failures.append(f"{path}: missing current-context term {term}")

    if failures:
        print("Context record check failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("All context records satisfy the E1 runtime handoff baseline.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
PY

cat > docs/09-ai/CURRENT-STATE.md <<'MD'
---
title: "Current State"
document_type: "ai-context"
status: "current"
version: "1.6.0"
created: "2026-05-23"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-007"
tags:
  - current-state
  - handoff
  - e1
  - cli-info
---

# Current State

## Project

Monad is an AI-native, repo-native, local-first Software Foundry OS for understanding, verifying, and safely evolving software repositories.

## Current Epic

E1 — Runtime Foundation

## Current Work Packet

WP-E1-007 — Establish CLI Info Command Foundation

## Prior Work

E0 — Project Foundation is complete.

WP-E1-001 — Establish Rust Workspace Runtime Foundation is complete.

WP-E1-002 — Establish Core Diagnostics Foundation is complete.

WP-E1-003 — Establish Core Error Foundation is complete.

WP-E1-004 — Establish Workspace Context Foundation is complete.

WP-E1-005 — Establish Manifest Model Foundation is complete.

WP-E1-006 — Establish Manifest Loading Foundation is complete.

## Active Runtime Focus

CLI Info.

The current slice adds:

- early command parsing;
- `monad help`;
- `monad info`;
- CLI rendering from loaded workspace manifest state;
- CLI info smoke verification.

## Next Expected Slice

After WP-E1-007, proceed to a CLI check command foundation that can return diagnostics from repository checks.

## Verification

Run:

```bash
tools/scripts/verify.sh
````

Expected result:

```text
Verification baseline passed.
```

