---
title: "Verification Standard"
document_type: "workflow-standard"
status: "draft"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E0"
work_packet: "WP-E0-004"
tags:

* workflow
* verification
* quality
* testing

---

# Verification Standard

## 1. Purpose

This document defines how Monad work is verified.

Monad treats verification as a first-class delivery requirement, not an afterthought.

AI output, generated files, scripts, and implementation changes are proposed until verified.

## 2. Core Principle

A change is not complete merely because it was written.

A change is complete only when the expected result has been checked by an appropriate verification method.

## 3. Verification Levels

Monad uses multiple verification levels.

| Level             | Meaning                                               |
| ----------------- | ----------------------------------------------------- |
| Inspection        | Human review of file contents, structure, or diff     |
| Static Check      | Formatting, linting, schema validation, type checking |
| Unit Test         | Small isolated behavior test                          |
| Integration Test  | Multiple components verified together                 |
| Contract Test     | API, schema, manifest, or repo contract verification  |
| End-to-End Test   | User-visible workflow verification                    |
| Operational Check | Runtime, install, release, or environment validation  |
| Context Check     | Handoff/current-state/decision-log verification       |

## 4. Required Verification Mindset

Every work packet MUST include:

* verification commands when automation exists;
* expected results after verification;
* known limitations;
* manual review notes when automation is insufficient.

## 5. Expected Result After Verification

Work packets MUST include an `Expected Result After Verification` section.

This section describes what the user should see after commands complete successfully.

Examples:

* `All docs/work/.monad Markdown files have YAML frontmatter.`
* `cargo test passes.`
* `bun run verify exits with code 0.`
* `The generated graph output is deterministic.`
* `No unexpected files are modified.`

## 6. Verification Command Standards

Verification commands SHOULD be:

* copy-pasteable;
* runnable from the repository root;
* deterministic;
* safe;
* explicit about interpreter or tool versions where needed.

Use `python3`, not `python`.

Good:

```bash
python3 tools/scripts/check-frontmatter.py
```

Weak:

```bash
run the script
```

## 7. Verification Before Commit

Before committing, run the narrowest relevant verification first, then broader verification if available.

Recommended sequence:

```bash
git status --short
git diff --check
```

Then run the relevant project-specific command.

For the current documentation foundation:

```bash
python3 <<'PY'
from pathlib import Path

missing = []

for root in ["docs", "work", ".monad"]:
    root_path = Path(root)
    if not root_path.exists():
        continue

    for path in sorted(root_path.rglob("*.md")):
        text = path.read_text(encoding="utf-8")
        if not text.startswith("---\\n"):
            missing.append(str(path))

if missing:
    print("Markdown files missing frontmatter:")
    for item in missing:
        print(f"  {item}")
    raise SystemExit(1)

print("All docs/work/.monad Markdown files have YAML frontmatter.")
PY
```

## 8. Verification Failure Rules

If verification fails:

1. Stop and inspect the failure.
2. Do not commit the failing state unless intentionally capturing a failing test before a fix.
3. Identify whether the failure is caused by:

   * the new change;
   * pre-existing repo state;
   * environment/tooling issue;
   * incorrect verification command;
   * outdated documentation.
4. Fix the smallest responsible issue.
5. Re-run the relevant verification.

## 9. AI Output Verification

AI-generated output MUST be treated as unverified proposal.

Before accepting AI-generated work:

* inspect the diff;
* run applicable verification;
* check for architectural drift;
* check consistency with ADRs;
* check for hallucinated files, commands, APIs, or dependencies;
* update context state if the change is accepted.

## 10. Documentation Verification

Documentation verification SHOULD check:

* YAML frontmatter;
* correct canonical path;
* internal consistency;
* no stale references;
* no contradiction with ADRs;
* no accidental placeholder text;
* expected cross-links when needed.

## 11. Code Verification

Code verification SHOULD check:

* formatting;
* linting;
* type checking;
* unit tests;
* integration tests where relevant;
* security-sensitive behavior;
* error handling;
* deterministic output when expected.

## 12. Repository Contract Verification

Monad SHOULD grow repository contract checks that validate:

* required root files;
* required docs areas;
* required work areas;
* ADR structure;
* context bridge files;
* package/crate boundaries;
* command behavior;
* generated artifacts;
* CI parity.

## 13. Verification Evidence

For important changes, the response or work record SHOULD include:

* commands run;
* exit status if useful;
* relevant output;
* expected result;
* known caveats.

## 14. Completion Rule

A work packet is not done until verification is either:

* passed; or
* explicitly documented as blocked, unavailable, or deferred with a reason.
