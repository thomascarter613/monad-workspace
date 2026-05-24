#!/usr/bin/env python3
"""
Check Monad epic records for required baseline structure.

This script verifies that durable epic records under work/epics/ follow the
early Monad planning structure:
- filename convention;
- YAML frontmatter;
- required frontmatter keys;
- required planning headings;
- Product Area before Objective;
- Expected Result After Verification present;
- Priority before Size.
"""

from pathlib import Path
import re


EPIC_DIR = Path("work/epics")

EPIC_FILENAME_PATTERN = re.compile(r"^E\d+-[a-z0-9][a-z0-9-]*\.md$")

REQUIRED_FRONTMATTER_KEYS = [
    "title:",
    "document_type:",
    "status:",
    "version:",
    "created:",
    "updated:",
    "owner:",
    "epic:",
]

REQUIRED_HEADINGS_IN_ORDER = [
    "## Product Area",
    "## Objective",
    "## Rationale",
    "## Scope",
    "## Out of Scope",
    "## Work Packets",
    "## Expected Result After Verification",
    "## Verification",
    "## Priority",
    "## Size",
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

    if not EPIC_DIR.exists():
        print(f"Epic directory does not exist: {EPIC_DIR}")
        return 1

    epic_files = sorted(path for path in EPIC_DIR.glob("*.md") if path.name != "README.md")

    if not epic_files:
        failures.append("No epic records found under work/epics/.")

    for path in epic_files:
        if not EPIC_FILENAME_PATTERN.match(path.name):
            failures.append(f"{path}: epic filename does not match E<number>-kebab-case.md")

        text = path.read_text(encoding="utf-8")
        split = split_frontmatter(text)

        if split is None:
            failures.append(f"{path}: missing or malformed YAML frontmatter")
            continue

        frontmatter, body = split

        for key in REQUIRED_FRONTMATTER_KEYS:
            if key not in frontmatter:
                failures.append(f"{path}: frontmatter missing {key}")

        if 'document_type: "epic"' not in frontmatter and "document_type: epic" not in frontmatter:
            failures.append(f"{path}: frontmatter document_type must be epic")

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
                "(Product Area, Objective, Rationale, Scope, Out of Scope, Work Packets, "
                "Expected Result After Verification, Verification, Priority, Size)"
            )

        if "| Work Packet | Title | Status |" not in body:
            failures.append(f"{path}: missing expected work packet summary table header")

    if failures:
        print("Epic record check failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("All epic records satisfy the required baseline structure.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
