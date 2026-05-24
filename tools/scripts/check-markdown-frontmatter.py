#!/usr/bin/env python3
"""
Check that every Markdown file under docs/, work/, and .monad/ starts with YAML frontmatter.

This intentionally avoids external dependencies so the earliest Monad repository
verification baseline works on any machine with Python 3 installed.
"""

from pathlib import Path


# These are the repository areas where Markdown files are expected to be
# project records, documentation records, work records, or context records.
ROOTS_TO_SCAN = [
    Path("docs"),
    Path("work"),
    Path(".monad"),
]


def main() -> int:
    missing_frontmatter: list[str] = []

    for root in ROOTS_TO_SCAN:
        # Some roots may not exist during very early bootstrapping.
        # Missing required roots are checked by check-required-paths.py.
        if not root.exists():
            continue

        for path in sorted(root.rglob("*.md")):
            text = path.read_text(encoding="utf-8")

            if not text.startswith("---\n"):
                missing_frontmatter.append(str(path))

    if missing_frontmatter:
        print("Markdown files missing YAML frontmatter:")
        for item in missing_frontmatter:
            print(f"  {item}")
        return 1

    print("All docs/work/.monad Markdown files have YAML frontmatter.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
