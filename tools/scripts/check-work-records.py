#!/usr/bin/env python3
"""
Check Monad work packet records for required structural fields.

This script verifies the early work-packet discipline required by Monad:
- records must have YAML frontmatter;
- Product Area must appear before Objective;
- Expected Result After Verification must be present;
- Priority and Size must be the final required planning fields.
"""

from pathlib import Path


WORK_PACKET_ROOT = Path("work/packets")

REQUIRED_HEADINGS_IN_ORDER = [
    "## Product Area",
    "## Objective",
    "## Expected Result After Verification",
    "## Priority",
    "## Size",
]


def find_heading_position(text: str, heading: str) -> int:
    return text.find(f"\n{heading}\n")


def main() -> int:
    failures: list[str] = []

    packet_files = sorted(WORK_PACKET_ROOT.rglob("WP-*.md"))

    if not packet_files:
        failures.append("No work packet records found under work/packets/.")

    for path in packet_files:
        text = path.read_text(encoding="utf-8")

        if not text.startswith("---\n"):
            failures.append(f"{path}: missing YAML frontmatter")

        positions: list[int] = []

        for heading in REQUIRED_HEADINGS_IN_ORDER:
            position = find_heading_position(text, heading)
            if position == -1:
                failures.append(f"{path}: missing required heading {heading}")
            positions.append(position)

        found_positions = [position for position in positions if position != -1]
        if found_positions != sorted(found_positions):
            failures.append(
                f"{path}: required headings are not in the expected order "
                "(Product Area, Objective, Expected Result After Verification, Priority, Size)"
            )

        priority_position = find_heading_position(text, "## Priority")
        size_position = find_heading_position(text, "## Size")

        if priority_position != -1 and size_position != -1 and priority_position > size_position:
            failures.append(f"{path}: Priority must appear before Size")

    if failures:
        print("Work packet record check failed:")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print("All work packet records satisfy the required structure.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
