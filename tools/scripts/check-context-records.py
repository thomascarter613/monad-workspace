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
]

REQUIRED_TERMS = [
    "E0",
    "E1",
    "WP-E1-001",
    "WP-E1-002",
    "Runtime Foundation",
    "Core Diagnostics",
]


def has_frontmatter(text: str) -> bool:
    return text.startswith("---\n") and "\n---\n" in text[len("---\n"):]


def main() -> int:
    failures: list[str] = []

    for path in REQUIRED_CONTEXT_FILES:
        if not path.exists():
            failures.append(f"missing context file: {path}")
            continue

        text = path.read_text(encoding="utf-8")

        if not has_frontmatter(text):
            failures.append(f"{path}: missing or malformed YAML frontmatter")

        for term in REQUIRED_TERMS:
            if term not in text:
                failures.append(f"{path}: missing required handoff term {term}")

    if failures:
        print("Context record check failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("All context records satisfy the E1 runtime handoff baseline.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
