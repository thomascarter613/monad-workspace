#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E12-005 — Add add-command smoke verification.
#
# Learning-first packet:
# - Keeps the one-script convenience.
# - Creates/updates tools/scripts/verify-add.sh as the canonical add smoke test.
# - Creates a learning note explaining verification design.
# - Creates verification evidence docs.
#
# Safety:
# - Writes only to temporary directories.
# - Does not mutate the Monad repository except for adding docs/scripts.
# - Does not install packages.
# - Does not run Git commands.

echo "==> WP-E12-005"
echo "Goal: make add-command smoke verification official."
echo "Mental model: verify dry-run, verify guarded write in initialized temp workspace, verify conflict refusal."
echo

mkdir -p tools/scripts
mkdir -p docs/verification
mkdir -p work/learning/E12
mkdir -p work/deliverables/E12

cat > tools/scripts/verify-add.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

# Verify Monad add behavior.
#
# Run from the Monad repository root:
#
#   tools/scripts/verify-add.sh
#
# This script writes only to temporary directories.
#
# It verifies:
# - add dry-run from the real repository workspace;
# - add requires an initialized Monad workspace for writes;
# - init + add --yes works in a temporary workspace;
# - add --dry-run writes no component files;
# - add refuses existing component target files;
# - add rejects conflicting mode flags.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
CARGO_MANIFEST="${REPO_ROOT}/Cargo.toml"

if [[ ! -f "${CARGO_MANIFEST}" ]]; then
  echo "Unable to locate Cargo manifest at ${CARGO_MANIFEST}" >&2
  exit 1
fi

run_from_repo() {
  (
    cd "${REPO_ROOT}"
    "$@"
  )
}

run_from_temp() {
  local temp_dir="$1"
  shift

  (
    cd "${temp_dir}"
    "$@"
  )
}

assert_file_exists() {
  local path="$1"

  if [[ ! -f "${path}" ]]; then
    echo "Expected file to exist: ${path}" >&2
    exit 1
  fi
}

assert_path_missing() {
  local path="$1"

  if [[ -e "${path}" ]]; then
    echo "Expected path to be missing: ${path}" >&2
    exit 1
  fi
}

assert_output_contains() {
  local output="$1"
  local expected="$2"

  if ! grep -Fq "${expected}" <<<"${output}"; then
    echo "Expected output to contain: ${expected}" >&2
    echo "Actual output:" >&2
    echo "${output}" >&2
    exit 1
  fi
}

write_temp=""
dry_temp=""
conflict_temp=""
uninitialized_temp=""
trap 'rm -rf "${write_temp:-}" "${dry_temp:-}" "${conflict_temp:-}" "${uninitialized_temp:-}"' EXIT

echo "==> Verifying add dry-run from repository workspace"

dry_run_output="$(run_from_repo cargo run -p monad-cli -- add app web --dry-run)"
assert_output_contains "${dry_run_output}" "Monad add dry-run plan"
assert_output_contains "${dry_run_output}" "kind: app"
assert_output_contains "${dry_run_output}" "name: web"
assert_output_contains "${dry_run_output}" "apps/web/README.md"
assert_output_contains "${dry_run_output}" "No files were written."

echo "==> Verifying add write path requires a Monad workspace"

uninitialized_temp="$(mktemp -d)"

if run_from_temp "${uninitialized_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add app web --yes; then
  echo "Expected add --yes to fail in an uninitialized temp directory." >&2
  exit 1
fi

assert_path_missing "${uninitialized_temp}/apps/web/README.md"
assert_path_missing "${uninitialized_temp}/apps/web/.gitkeep"

echo "==> Verifying init + add --yes in a temporary Monad workspace"

write_temp="$(mktemp -d)"

run_from_temp "${write_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes
add_output="$(run_from_temp "${write_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add app web --yes)"

assert_output_contains "${add_output}" "Monad add applied"
assert_output_contains "${add_output}" "kind: app"
assert_output_contains "${add_output}" "name: web"
assert_output_contains "${add_output}" "No Git commands were run."

assert_file_exists "${write_temp}/monad.toml"
assert_file_exists "${write_temp}/apps/web/README.md"
assert_file_exists "${write_temp}/apps/web/.gitkeep"

echo "==> Verifying add --dry-run writes no component files"

dry_temp="$(mktemp -d)"

run_from_temp "${dry_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes
dry_temp_output="$(run_from_temp "${dry_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add service api --dry-run)"

assert_output_contains "${dry_temp_output}" "Monad add dry-run plan"
assert_output_contains "${dry_temp_output}" "kind: service"
assert_output_contains "${dry_temp_output}" "services/api/README.md"
assert_output_contains "${dry_temp_output}" "No files were written."

assert_path_missing "${dry_temp}/services/api/README.md"
assert_path_missing "${dry_temp}/services/api/.gitkeep"

echo "==> Verifying add refuses existing component target files"

conflict_temp="$(mktemp -d)"

run_from_temp "${conflict_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes
mkdir -p "${conflict_temp}/apps/api"
printf '# Existing API\n' > "${conflict_temp}/apps/api/README.md"

if run_from_temp "${conflict_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add app api --yes; then
  echo "Expected add --yes to fail when apps/api/README.md already exists." >&2
  exit 1
fi

assert_file_exists "${conflict_temp}/apps/api/README.md"
assert_path_missing "${conflict_temp}/apps/api/.gitkeep"

echo "==> Verifying add rejects conflicting mode flags"

if run_from_repo cargo run -p monad-cli -- add app web --dry-run --yes; then
  echo "Expected add --dry-run --yes to fail because modes conflict." >&2
  exit 1
fi

echo
echo "Monad add smoke verification passed."
EOF

chmod +x tools/scripts/verify-add.sh

cat > docs/verification/ADD-SMOKE-TESTS.md <<'EOF'
---
title: "Add Command Smoke Tests"
document_type: "verification-evidence"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-005
tags:
  - monad
  - add
  - smoke-tests
  - verification
related:
  - tools/scripts/verify-add.sh
  - docs/commands/ADD.md
  - crates/monad-core/src/component_add.rs
  - crates/monad-cli/src/main.rs
---

# Add Command Smoke Tests

## Status

Accepted.

## Work Packet

WP-E12-005 — Add add-command smoke verification.

## Purpose

This document records the smoke verification path for `monad add`.

The purpose is to prove that the `add` command is safe enough to continue building on.

## Verification Script

The reusable smoke-test script is:

```text
tools/scripts/verify-add.sh
```

Run it from the repository root:

```bash
tools/scripts/verify-add.sh
```

## Coverage

The script verifies:

| Area | Coverage |
| --- | --- |
| Dry-run preview | `monad add app web --dry-run` |
| Workspace precondition | `monad add app web --yes` fails in an uninitialized temp directory |
| Guarded write | `monad init --yes`, then `monad add app web --yes` in a temp workspace |
| Dry-run non-write | `monad add service api --dry-run` creates no files |
| Conflict refusal | Existing `apps/api/README.md` blocks `monad add app api --yes` |
| Mode conflict | `monad add app web --dry-run --yes` fails |

## Expected Add Write Output

A successful guarded add should report:

```text
Monad add applied
```

It should also report:

```text
No Git commands were run.
```

## Expected Files

For:

```bash
monad add app web --yes
```

inside an initialized Monad workspace, expected files are:

```text
apps/web/README.md
apps/web/.gitkeep
```

## Safety Properties Verified

The smoke test verifies that:

- `add` previews before writing;
- `add --yes` works only inside a Monad workspace;
- `add --dry-run` writes nothing;
- `add --yes` refuses conflicts;
- `add --dry-run --yes` is rejected;
- verification writes only into temp directories.

## Full Verification Command Set

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-add.sh
tools/scripts/verify.sh
git status --short
```

## Acceptance Criteria

WP-E12-005 is complete when:

- `tools/scripts/verify-add.sh` exists;
- this verification evidence document exists;
- the deliverable record exists;
- add dry-run is smoke-tested;
- add guarded write is smoke-tested;
- add conflict refusal is smoke-tested;
- all normal verification commands pass.

## Outcome

Accepted.

Monad now has a reusable add-command smoke verification script and evidence record.
EOF

cat > work/learning/E12/WP-E12-005-add-smoke-verification.md <<'EOF'
---
title: "Learning Note — WP-E12-005 Add Smoke Verification"
document_type: "learning-note"
status: active
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-005
tags:
  - learning
  - verification
  - smoke-tests
  - add
  - monad
---

# Learning Note — WP-E12-005 Add Smoke Verification

## What You Are Building

You are turning manual checks into a reusable verification script:

```text
tools/scripts/verify-add.sh
```

This is important because each future `add` change should be able to prove it did not break the safety contract.

## Mental Model

A smoke test is not an exhaustive test.

It is a fast confidence check for the most important behavior.

For `monad add`, the important behavior is:

```text
preview safely
write only after --yes
write only inside a Monad workspace
refuse conflicts
reject conflicting mode flags
```

## Why the Temp Directory Must Be Initialized

`monad add` adds a component to a Monad workspace.

A plain `mktemp -d` directory is not automatically a Monad workspace.

That means the guarded write verification must do this:

```bash
monad init --yes
monad add app web --yes
```

inside the temp directory.

## Why This Test Matters

This test protects the command's contract:

- dry-run does not write;
- `--yes` writes only after explicit approval;
- uninitialized workspaces fail safely;
- existing files are not overwritten.

## What to Inspect

Read:

```text
tools/scripts/verify-add.sh
```

Look for these phases:

```text
add dry-run
uninitialized workspace failure
init + add write
dry-run non-write
conflict refusal
mode conflict rejection
```

## What This Teaches

Good CLI verification tests:

1. create their own temporary workspace;
2. prove successful behavior;
3. prove failure behavior;
4. clean up after themselves;
5. avoid mutating the real repo unless explicitly intended.

## Verification

Run:

```bash
tools/scripts/verify-add.sh
```

Then run the full repo verification:

```bash
tools/scripts/verify.sh
```
EOF

cat > work/deliverables/E12/WP-E12-005-add-smoke-verification.md <<'EOF'
---
title: "WP-E12-005 Add Smoke Verification Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-005
tags:
  - monad
  - e12
  - add
  - smoke-tests
  - verification
related:
  - tools/scripts/verify-add.sh
  - docs/verification/ADD-SMOKE-TESTS.md
  - work/learning/E12/WP-E12-005-add-smoke-verification.md
---

# WP-E12-005 Add Smoke Verification Deliverable

## Work Packet

WP-E12-005 — Add add-command smoke verification.

## Outcome

Implemented.

## Summary

This work packet adds the reusable smoke verification script for `monad add`.

The script verifies dry-run behavior, guarded write behavior, workspace preconditions, conflict refusal, and mode conflict rejection.

## Deliverables

- `tools/scripts/verify-add.sh`
- `docs/verification/ADD-SMOKE-TESTS.md`
- `work/learning/E12/WP-E12-005-add-smoke-verification.md`
- `work/deliverables/E12/WP-E12-005-add-smoke-verification.md`

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-add.sh
tools/scripts/verify.sh
git status --short
```

## Recommended Commit

```bash
git commit -m "test(add): add add-command smoke verification"
```

## Closeout Note

WP-E12-005 is complete once the smoke verification script and evidence docs are committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E12-006 — Document add workflow and close E12
```
EOF

python3 - <<'PY'
from pathlib import Path

add_doc = Path("docs/commands/ADD.md")
if add_doc.exists():
    text = add_doc.read_text(encoding="utf-8")
    marker = "## WP-E12-005 Verification Note"
    if marker not in text:
        lines = [
            "",
            "",
            marker,
            "",
            "The reusable add-command smoke verification script is:",
            "",
            "```bash",
            "tools/scripts/verify-add.sh",
            "```",
            "",
            "It verifies dry-run behavior, guarded writes in initialized temp workspaces, conflict refusal, and mode conflict rejection.",
            "",
        ]
        add_doc.write_text(text + "\n".join(lines), encoding="utf-8")
PY

cargo fmt

echo
echo "WP-E12-005 files updated:"
echo "  tools/scripts/verify-add.sh"
echo "  docs/verification/ADD-SMOKE-TESTS.md"
echo "  work/learning/E12/WP-E12-005-add-smoke-verification.md"
echo "  work/deliverables/E12/WP-E12-005-add-smoke-verification.md"
echo
echo "Learning checkpoint:"
echo "  Read work/learning/E12/WP-E12-005-add-smoke-verification.md before committing."
echo
echo "Next verification:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-add.sh"
echo "  tools/scripts/verify.sh"
