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
    """Return frontmatter and body if a Markdown file starts with YAML frontmatter."""
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

        # ADR-0000 is the template. It may describe required sections without
        # itself being an accepted decision.
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
