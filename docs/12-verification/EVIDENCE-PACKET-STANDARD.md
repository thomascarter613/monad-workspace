---
title: "Evidence Packet Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - verification
  - evidence
  - reports
related:
  - docs/12-verification/VERIFICATION-MODEL.md
  - docs/12-verification/CHECK-REGISTRY-STANDARD.md
  - docs/12-verification/EXIT-CODE-STANDARD.md
  - docs/07-workflow/DEFINITION-OF-DONE.md
---

# Evidence Packet Standard

## Purpose

This document defines Monad’s evidence packet standard.

An evidence packet is a reviewable artifact that records what verification was performed, what happened, and what conclusions can reasonably be drawn.

## Core Rule

An evidence packet must show the work.

It should make verification visible enough that a human reviewer or future AI session can understand what was checked and what remains uncertain.

## Evidence Packet Goals

An evidence packet should answer:

- What work was being verified?
- What checks ran?
- What commands ran?
- What passed?
- What failed?
- What was skipped?
- What files or reports were generated?
- What limitations remain?
- What should happen next?

## Recommended Locations

Generated evidence packets may live under:

```text
.monad/reports/
```

Examples:

```text
.monad/reports/latest-verification.md
.monad/reports/verification-2026-05-23.md
```

If an evidence packet becomes canonical documentation, it may be linked from `docs/`.

## Human-Readable First

Evidence packets should first be human-readable.

Markdown is preferred for early MVP evidence.

Machine-readable JSON can be added for automation.

## Required Evidence Packet Sections

An evidence packet should include:

```text
Metadata
Scope
Summary
Checks Run
Command Results
Files Examined
Files Changed
Failures
Skipped Checks
Warnings
Expected Result
Actual Result
Conclusion
Limitations
Next Recommended Action
```

## Section: Metadata

Include:

- generated time if applicable;
- Monad version if known;
- repository root;
- active work packet if known;
- command that generated the packet if known.

## Section: Scope

State what the evidence packet covers.

Example:

```text
This packet verifies WP-E1-003, which adds the core error and diagnostic model.
```

## Section: Summary

Provide a concise summary.

Example:

```text
Rust formatting, tests, and Clippy checks passed.
```

## Section: Checks Run

List check IDs and names.

Example:

```text
rust.fmt — passed
rust.test — passed
rust.clippy — passed
```

## Section: Command Results

List commands and outcomes.

Example:

```text
cargo fmt --check — exit code 0
cargo test — exit code 0
cargo clippy --all-targets --all-features -- -D warnings — exit code 0
```

## Section: Files Examined

List relevant files if useful.

This should not become a noisy dump of every file unless needed.

## Section: Files Changed

List changed files if the evidence packet is tied to a work packet.

Example:

```text
crates/monad-core/src/diagnostics.rs
crates/monad-core/src/lib.rs
```

## Section: Failures

If there are failures, list them clearly.

If none, write:

```text
No failures.
```

## Section: Skipped Checks

Skipped checks must not be hidden.

Example:

```text
js.test was skipped because package.json was not detected.
```

## Section: Warnings

Warnings should be visible even if they do not fail the run.

## Section: Expected Result

Restate the expected result from the work packet.

## Section: Actual Result

State what actually happened.

## Section: Conclusion

State whether the evidence supports marking the work complete.

Examples:

```text
The evidence supports marking WP-E1-003 complete.
```

or:

```text
The evidence does not support completion because Rust tests failed.
```

## Section: Limitations

State what the evidence does not prove.

Examples:

- This does not prove all future CLI commands work.
- This does not prove security correctness.
- This does not prove semantic correctness beyond the tests run.

## Section: Next Recommended Action

State what should happen next.

## Evidence Packet Template

```markdown
---
title: "Verification Evidence"
status: generated
generated_at: YYYY-MM-DD
reviewed: false
work_packet: WP-EX-000
---

# Verification Evidence

## Metadata

## Scope

## Summary

## Checks Run

## Command Results

## Files Examined

## Files Changed

## Failures

## Skipped Checks

## Warnings

## Expected Result

## Actual Result

## Conclusion

## Limitations

## Next Recommended Action
```

## MVP Evidence Packet

The MVP evidence packet should be able to represent:

- Rust formatting result;
- Rust test result;
- Rust Clippy result;
- documentation frontmatter result;
- CLI smoke test result;
- skipped checks;
- command outputs or summaries;
- overall pass/fail conclusion.

## What Evidence Packets Must Not Do

Evidence packets must not:

- claim more than was checked;
- hide failures;
- hide skipped checks;
- include secrets;
- store unnecessary full logs forever;
- mark generated content as accepted without review;
- replace actual verification commands.

## Evidence and AI

AI may summarize evidence, but AI summary is not a substitute for command results.

If an AI assistant describes verification, it should cite or quote the actual command results when available.

## Current Status

This evidence packet standard is a draft. It is authoritative enough to guide E4 evidence packet implementation.
