#!/usr/bin/env python3
"""
Check Monad deliverable records for required baseline structure.

Deliverable records connect planned work to concrete artifacts. This checker
keeps the early Monad workflow hierarchy mechanically verifiable without adding
external dependencies.
"""

from pathlib import Path
import re


DELIVERABLE_ROOT = Path("work/deliverables")

DELIVERABLE_FILENAME_PATTERN = re.compile(
    r"^D-WP-E\d+-\d{3}-\d{3}-[a-z0-9][a-z0-9-]*\.md$"
)

REQUIRED_FRONTMATTER_KEYS = [
    "title:",
    "document_type:",
    "status:",
    "version:",
    "created:",
    "updated:",
    "owner:",
    "epic:",
    "work_packet:",
    "deliverable:",
]

REQUIRED_HEADINGS_IN_ORDER = [
    "## Product Area",
    "## Objective",
    "## Source Work Packet",
    "## Deliverable Type",
    "## Artifact Path",
    "## Expected Result After Verification",
    "## Verification",
    "## Status",
]


def split_frontmatter(text: str) -> tuple[str, str] | None:
    if not text.startswith("---\n"):
        return None

    marker = "\n---\n"
    end = text.find(marker, len("---\n"))

    if end == -1:
        return None

    frontmatter = text[len("---\n"):end]
    body = text[end + len(marker):]
    return frontmatter, body


def find_heading_position(text: str, heading: str) -> int:
    return text.find(f"\n{heading}\n")


def main() -> int:
    failures: list[str] = []

    if not DELIVERABLE_ROOT.exists():
        print(f"Deliverable root does not exist: {DELIVERABLE_ROOT}")
        return 1

    deliverable_files = sorted(DELIVERABLE_ROOT.rglob("D-*.md"))

    if not deliverable_files:
        failures.append("No deliverable records found under work/deliverables/.")

    for path in deliverable_files:
        if not DELIVERABLE_FILENAME_PATTERN.match(path.name):
            failures.append(
                f"{path}: deliverable filename does not match "
                "D-WP-E<number>-<work-packet-number>-<deliverable-number>-kebab-case.md"
            )

        text = path.read_text(encoding="utf-8")
        split = split_frontmatter(text)

        if split is None:
            failures.append(f"{path}: missing or malformed YAML frontmatter")
            continue

        frontmatter, body = split

        for key in REQUIRED_FRONTMATTER_KEYS:
            if key not in frontmatter:
                failures.append(f"{path}: frontmatter missing {key}")

        if 'document_type: "deliverable"' not in frontmatter and "document_type: deliverable" not in frontmatter:
            failures.append(f"{path}: frontmatter document_type must be deliverable")

        if "# " not in body:
            failures.append(f"{path}: missing top-level Markdown heading")

        positions: list[int] = []

        for heading in REQUIRED_HEADINGS_IN_ORDER:
            position = find_heading_position(body, heading)
            if position == -1:
                failures.append(f"{path}: missing required heading {heading}")
            positions.append(position)

        found_positions = [position for position in positions if position != -1]
        if found_positions != sorted(found_positions):
            failures.append(
                f"{path}: required headings are not in the expected order "
                "(Product Area, Objective, Source Work Packet, Deliverable Type, "
                "Artifact Path, Expected Result After Verification, Verification, Status)"
            )

    if failures:
        print("Deliverable record check failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("All deliverable records satisfy the required baseline structure.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
