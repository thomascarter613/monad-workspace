#!/usr/bin/env python3
"""
Check Monad context records for durable repository continuity.

This verifier intentionally checks:
1. Required context files exist.
2. Required context files have YAML frontmatter.
3. Historical handoff terms remain discoverable somewhere in the context corpus.
4. Current release/context state is discoverable in first-party release/context records.

It does not require every current context artifact to repeat the active epic,
work packet, or release-preparation language.
"""

from pathlib import Path


REQUIRED_CONTEXT_FILES = [
    Path("docs/09-ai/CURRENT-STATE.md"),
    Path("docs/09-ai/FRESH-CHAT-HANDOFF.md"),
    Path(".monad/context/current-state.md"),
    Path(".monad/context/latest-handoff.md"),
    Path(".monad/context/latest-context-pack.md"),
    Path(".monad/context/decision-log.md"),
    Path(".monad/context/work-packet-handoffs/WP-E1-013.md"),
    Path(".monad/context/work-packet-handoffs/WP-E2-001.md"),
]

RELEASE_CONTEXT_FILES = [
    Path("docs/context/CONTEXT-FRESHNESS-POLICY.md"),
    Path("docs/context/RELEASE-CONTEXT-STATE.md"),
    Path("docs/release/PUBLIC-READINESS-GAP-AUDIT.md"),
    Path("docs/release/E9-STABILIZATION-PLAN.md"),
]

GLOBAL_REQUIRED_TERMS = [
    "E0",
    "E1",
    "E2",
    "Runtime Foundation",
    "Repository Intelligence",
    "Repository Inspection",
    "WP-E1-013",
    "WP-E2-001",
    "JSON Output",
]

CURRENT_RELEASE_TERMS = [
    "E9",
    "Post-MVP Candidate Stabilization",
    "v0.1.0-internal-mvp-candidate.1",
]


def has_frontmatter(text: str) -> bool:
    return text.startswith("---\n") and "\n---\n" in text[len("---\n"):]


def read_existing(paths: list[Path], failures: list[str]) -> list[str]:
    text_parts: list[str] = []

    for path in paths:
        if not path.exists():
            failures.append(f"missing context file: {path}")
            continue

        text = path.read_text(encoding="utf-8")
        text_parts.append(text)

        if not has_frontmatter(text):
            failures.append(f"{path}: missing or malformed YAML frontmatter")

    return text_parts


def main() -> int:
    failures: list[str] = []

    context_text = "\n".join(read_existing(REQUIRED_CONTEXT_FILES, failures))
    release_text = "\n".join(read_existing(RELEASE_CONTEXT_FILES, failures))

    for term in GLOBAL_REQUIRED_TERMS:
        if term not in context_text:
            failures.append(f"context corpus missing required handoff term {term}")

    for term in CURRENT_RELEASE_TERMS:
        if term not in release_text:
            failures.append(f"release/context corpus missing current release term {term}")

    if failures:
        print("Context record check failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("All context records satisfy durable continuity and current release-context discoverability.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
