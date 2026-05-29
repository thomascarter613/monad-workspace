#!/usr/bin/env python3
"""
Check that first-party Monad Markdown records start with YAML frontmatter.

Generated artifacts, vendored dependencies, build output, and imported DeepWiki
dumps are intentionally excluded because they are not first-party Monad docs.
"""

from pathlib import Path


ROOTS_TO_SCAN = [
    Path("docs"),
    Path("work"),
    Path(".monad"),
]

IGNORED_PATH_PARTS = {
    ".artifacts",
    "node_modules",
    "target",
    ".git",
}

IGNORED_PREFIXES = (
    Path("docs/wiki/.artifacts"),
    Path("docs/wiki/deepwiki-dump-monad-workspace"),
    Path(".monad/reports"),
    Path(".monad/context/generated"),
)


def should_ignore(path: Path) -> bool:
    if any(part in IGNORED_PATH_PARTS for part in path.parts):
        return True

    for prefix in IGNORED_PREFIXES:
        try:
            path.relative_to(prefix)
            return True
        except ValueError:
            pass

    return False


def main() -> int:
    missing_frontmatter: list[str] = []

    for root in ROOTS_TO_SCAN:
        if not root.exists():
            continue

        for path in sorted(root.rglob("*.md")):
            if should_ignore(path):
                continue

            text = path.read_text(encoding="utf-8")

            if not text.startswith("---\n"):
                missing_frontmatter.append(str(path))

    if missing_frontmatter:
        print("Markdown files missing YAML frontmatter:")
        for item in missing_frontmatter:
            print(f"  {item}")
        return 1

    print("All first-party docs/work/.monad Markdown files have YAML frontmatter.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
