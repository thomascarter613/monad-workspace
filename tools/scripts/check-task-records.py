#!/usr/bin/env python3
"""
Check Monad task records for required baseline structure.

Task records are the execution-level records beneath work packets. This checker
keeps the early Monad workflow hierarchy mechanically verifiable without adding
external dependencies.
"""

from pathlib import Path
import re


TASK_ROOT = Path("work/tasks")

TASK_FILENAME_PATTERN = re.compile(
    r"^T-WP-E\d+-\d{3}-\d{3}-[a-z0-9][a-z0-9-]*\.md$"
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
    "task:",
]

REQUIRED_HEADINGS_IN_ORDER = [
    "## Product Area",
    "## Objective",
    "## Parent Work Packet",
    "## Expected Result",
    "## Verification",
    "## Status",
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

    if not TASK_ROOT.exists():
        print(f"Task root does not exist: {TASK_ROOT}")
        return 1

    task_files = sorted(TASK_ROOT.rglob("T-*.md"))

    if not task_files:
        failures.append("No task records found under work/tasks/.")

    for path in task_files:
        if not TASK_FILENAME_PATTERN.match(path.name):
            failures.append(
                f"{path}: task filename does not match "
                "T-WP-E<number>-<work-packet-number>-<task-number>-kebab-case.md"
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

        if 'document_type: "task"' not in frontmatter and "document_type: task" not in frontmatter:
            failures.append(f"{path}: frontmatter document_type must be task")

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
                "(Product Area, Objective, Parent Work Packet, Expected Result, "
                "Verification, Status, Priority, Size)"
            )

        priority_position = find_heading_position(body, "## Priority")
        size_position = find_heading_position(body, "## Size")

        if priority_position != -1 and size_position != -1 and priority_position > size_position:
            failures.append(f"{path}: Priority must appear before Size")

    if failures:
        print("Task record check failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("All task records satisfy the required baseline structure.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
