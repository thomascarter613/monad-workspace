#!/usr/bin/env python3
"""
Check Monad context records for E1 closure and E2 handoff readiness.
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

GLOBAL_REQUIRED_TERMS = [
    "E0",
    "E1",
    "E2",
    "Runtime Foundation",
    "Repository Intelligence",
    "WP-E1-013",
    "WP-E2-001",
    "JSON Output",
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
        for term in ["E2", "WP-E2-001", "Repository Intelligence"]:
            if term not in text:
                failures.append(f"{path}: missing current-context term {term}")

    if failures:
        print("Context record check failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("All context records satisfy the E1 closure and E2 handoff baseline.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
