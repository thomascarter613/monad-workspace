#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E10-003 — Decide source-only versus packaged pre-release posture.
#
# This script creates:
# - docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
# - work/deliverables/E10/WP-E10-003-distribution-posture.md
#
# It also updates docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md if present,
# changing the source/package posture gate from Pending to Pass.

mkdir -p docs/release
mkdir -p work/deliverables/E10

cat > docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md <<'EOF'
---
title: "Public Pre-Release Distribution Posture"
document_type: "release-decision"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-003
tags:
  - monad
  - release
  - public-prerelease
  - distribution
  - source-only
related:
  - README.md
  - docs/release/PUBLIC-CLAIMS-AUDIT.md
  - docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md
  - docs/project/MVP-COMMAND-REFERENCE.md
---

# Public Pre-Release Distribution Posture

## Status

Accepted.

## Work Packet

WP-E10-003 — Decide source-only versus packaged pre-release posture.

## Decision

Monad's first public pre-release posture is:

```text
Source-only public pre-release.
```

The first public pre-release should not be presented as:

- a packaged binary release;
- an installer release;
- a package-manager-published release;
- a hosted service;
- a SaaS product;
- an autonomous agent runtime;
- a production-ready monorepo platform.

## Distribution Boundary

For the first public pre-release, users may evaluate Monad by:

1. cloning the repository;
2. reading the README and release notes;
3. building locally with Cargo;
4. running the documented MVP command surface through Cargo.

The expected local path is:

```bash
cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

## Explicitly Deferred Distribution Channels

The following are deferred beyond the first public pre-release:

| Channel | Posture | Reason |
| --- | --- | --- |
| GitHub binary release artifacts | Deferred | The first boundary should validate public claims and source-build workflow before packaging. |
| Installer scripts/packages | Deferred | Installer support requires separate safety, platform, and maintenance work. |
| Crates.io publication | Deferred | Public package publication should follow stronger release, versioning, and installation guarantees. |
| Homebrew/Apt/Nix/Winget | Deferred | Package manager distribution is out of scope for the first public pre-release. |
| Docker image | Deferred | Monad is a local-first CLI and should not imply a hosted/runtime product yet. |
| Hosted/SaaS service | Deferred | Hosted product work is explicitly outside E10 scope. |

## Rationale

A source-only public pre-release is the safest and most honest first public boundary.

It allows external readers or early evaluators to inspect the repository and run the existing CLI without implying that Monad is ready for package-manager installation, binary distribution, hosted deployment, or autonomous execution.

This posture protects the project from premature public claims while still allowing a responsible public milestone.

## Public Claim Rules

Public-facing docs should say:

```text
Monad is available as source code for local evaluation.
```

Public-facing docs should not say:

```text
Install Monad with a package manager.
Download a production-ready binary.
Use Monad as an autonomous agent runtime.
Use Monad as a hosted service.
Use Monad as a finished monorepo platform.
```

Unless and until future work explicitly implements and verifies those capabilities.

## Required README Boundary

The README should preserve language equivalent to:

```text
Monad is not yet a public release, published package, installer distribution, hosted service, or autonomous agent runtime.
```

If a public pre-release tag is later cut, it should be described as:

```text
source-only public pre-release
```

not as:

```text
general availability
production release
installer release
package release
```

## Impact on Remaining E10 Work

This decision affects the remaining E10 packets:

### WP-E10-004

Public pre-release notes must describe the release as source-only.

### WP-E10-005

Final verification must verify source-build/local-run commands, not installer/package workflows.

### WP-E10-006

The tag decision, if approved, should cut a source-only public pre-release tag and must not publish package artifacts unless a later explicit decision supersedes this document.

## Definition of Done for WP-E10-003

WP-E10-003 is complete when:

- source-only distribution posture is documented;
- deferred distribution channels are explicit;
- README/release-note claim rules are clear;
- the evidence checklist is updated if present;
- remaining E10 work is unblocked.

## Verification Commands

```bash
git status --short
test -f docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
test -f work/deliverables/E10/WP-E10-003-distribution-posture.md
grep -n "Source-only public pre-release" docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
tools/scripts/verify.sh
git status --short
```

## Outcome

Accepted.

Monad's first public pre-release posture is source-only.
EOF

cat > work/deliverables/E10/WP-E10-003-distribution-posture.md <<'EOF'
---
title: "WP-E10-003 Distribution Posture Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-003
tags:
  - monad
  - e10
  - release
  - distribution
  - source-only
related:
  - docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
  - docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md
  - README.md
---

# WP-E10-003 Distribution Posture Deliverable

## Work Packet

WP-E10-003 — Decide source-only versus packaged pre-release posture.

## Outcome

Completed.

## Decision

Monad's first public pre-release posture is:

```text
Source-only public pre-release.
```

## Summary

This work packet records that the first public pre-release, if approved, should be source-only.

It explicitly defers:

- packaged binary artifacts;
- installer generation;
- Crates.io/package-manager publication;
- hosted/SaaS launch;
- autonomous agent runtime claims;
- production-readiness claims.

## Deliverables

- `docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md`
- `work/deliverables/E10/WP-E10-003-distribution-posture.md`
- optional update to `docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md` if that checklist exists

## Verification

Run:

```bash
git status --short
test -f docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
test -f work/deliverables/E10/WP-E10-003-distribution-posture.md
grep -n "Source-only public pre-release" docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
tools/scripts/verify.sh
git status --short
```

## Recommended Commit

```bash
git commit -m "docs(release): decide source-only prerelease posture"
```

## Closeout Note

WP-E10-003 is complete once the distribution posture is committed and the corresponding GitHub work-packet issue or tracking item is closed.
EOF

if [[ -f docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md ]]; then
  python3 - <<'PY'
from pathlib import Path

path = Path("docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md")
text = path.read_text(encoding="utf-8")

replacements = {
    "| Source/package posture decided | Pending | WP-E10-003 remains required |": "| Source/package posture decided | Pass | `docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md` |",
    "| Source-only vs packaged pre-release posture is decided. | Pending | WP-E10-003 | Required before release tag approval. |": "| Source-only vs packaged pre-release posture is decided. | Pass | `docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md` | Source-only public pre-release posture accepted. |",
    "| Any package/install claims are removed or explicitly deferred. | Pending | WP-E10-003 | Depends on posture decision. |": "| Any package/install claims are removed or explicitly deferred. | Pass | `docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md` | Binary, installer, package-manager, and hosted distribution are deferred. |",
    "WP-E10-003 — Decide source-only versus packaged pre-release posture\n": "",
}

for old, new in replacements.items():
    text = text.replace(old, new)

text = text.replace(
    "The public claims audit gate is being addressed, but source/package posture,\nrelease notes, final verification, and tag decision gates remain pending.",
    "The public claims audit and source-only distribution posture gates are addressed, but\nrelease notes, final verification, and tag decision gates remain pending.",
)

path.write_text(text, encoding="utf-8")
PY
  echo "Updated docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md"
else
  echo "Skipped checklist update because docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md does not exist yet."
fi

echo
echo "WP-E10-003 files written:"
echo "  docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md"
echo "  work/deliverables/E10/WP-E10-003-distribution-posture.md"
echo
echo "Next verification:"
echo "  git status --short"
echo "  test -f docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md"
echo "  tools/scripts/verify.sh"
