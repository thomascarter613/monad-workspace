#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E12-004 verification instructions for workspace discovery.
#
# What happened:
#   Running `monad add app web --yes` from a completely empty temp directory
#   failed with:
#     MONAD2002: required resource not found: Monad workspace root from .
#
# Why:
#   `monad add` discovers the Monad workspace from the current directory.
#   A plain mktemp directory is not a Monad workspace yet.
#
# Correct verification:
#   1. Create temp dir.
#   2. Run `monad init --yes` in that temp dir.
#   3. Run `monad add app web --yes` in that initialized workspace.
#
# This script updates learning/deliverable notes and adds a reusable local
# verification helper for the corrected flow.

mkdir -p tools/scripts
mkdir -p work/learning/E12
mkdir -p work/deliverables/E12

cat > tools/scripts/verify-add.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

# Verify Monad add behavior safely in temporary directories.
#
# This script should be run from the Monad repository root.
# It writes only to temporary directories.
#
# It verifies:
# - add dry-run from the real Monad repo
# - init + add guarded write in a temp workspace
# - add conflict refusal in an initialized temp workspace

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

add_temp=""
conflict_temp=""
trap 'rm -rf "${add_temp:-}" "${conflict_temp:-}"' EXIT

echo "==> Verifying add dry-run from repository workspace"

dry_run_output="$(run_from_repo cargo run -p monad-cli -- add app web --dry-run)"
assert_output_contains "${dry_run_output}" "Monad add dry-run plan"
assert_output_contains "${dry_run_output}" "kind: app"
assert_output_contains "${dry_run_output}" "name: web"
assert_output_contains "${dry_run_output}" "apps/web/README.md"
assert_output_contains "${dry_run_output}" "No files were written."

echo "==> Verifying init + add --yes in temporary workspace"

add_temp="$(mktemp -d)"

run_from_temp "${add_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes
run_from_temp "${add_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add app web --yes

assert_file_exists "${add_temp}/monad.toml"
assert_file_exists "${add_temp}/apps/web/README.md"
assert_file_exists "${add_temp}/apps/web/.gitkeep"

echo "==> Verifying add refuses existing component files"

conflict_temp="$(mktemp -d)"

run_from_temp "${conflict_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes
mkdir -p "${conflict_temp}/apps/api"
printf '# Existing API\n' > "${conflict_temp}/apps/api/README.md"

if run_from_temp "${conflict_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add app api --yes; then
  echo "Expected add --yes to fail when apps/api/README.md already exists." >&2
  exit 1
fi

assert_file_exists "${conflict_temp}/apps/api/README.md"

echo
echo "Monad add verification passed."
EOF

chmod +x tools/scripts/verify-add.sh

python3 - <<'PY'
from pathlib import Path

learning_path = Path("work/learning/E12/WP-E12-004-guarded-add-write-path.md")
if learning_path.exists():
    learning = learning_path.read_text(encoding="utf-8")
    marker = "## Fix Note — Add Requires a Monad Workspace"
    if marker not in learning:
        learning += "\n\n" + "\n".join([
            marker,
            "",
            "A plain `mktemp -d` directory is not automatically a Monad workspace.",
            "",
            "The command:",
            "",
            "```bash",
            "cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- add app web --yes",
            "```",
            "",
            "fails from an uninitialized temp directory because `monad add` tries to discover the Monad workspace root from `.`.",
            "",
            "That is correct for the current command model: `add` adds a component to an existing Monad workspace.",
            "",
            "The corrected temporary verification flow is:",
            "",
            "```bash",
            "tmpdir=\"$(mktemp -d)\"",
            "(",
            "  cd \"$tmpdir\"",
            "  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- init --yes",
            "  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- add app web --yes",
            "  test -f apps/web/README.md",
            "  test -f apps/web/.gitkeep",
            ")",
            "rm -rf \"$tmpdir\"",
            "```",
            "",
            "The important lesson is that command verification should match the command's preconditions.",
            "",
        ])
        learning_path.write_text(learning, encoding="utf-8")

deliverable_path = Path("work/deliverables/E12/WP-E12-004-guarded-add-write-path.md")
if deliverable_path.exists():
    text = deliverable_path.read_text(encoding="utf-8")
    old = """tmpdir="$(mktemp -d)"
(
  cd "$tmpdir"
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- add app web --yes
  test -f apps/web/README.md
  test -f apps/web/.gitkeep
)
rm -rf "$tmpdir"
"""
    new = """tmpdir="$(mktemp -d)"
(
  cd "$tmpdir"
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- init --yes
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- add app web --yes
  test -f apps/web/README.md
  test -f apps/web/.gitkeep
)
rm -rf "$tmpdir"
"""
    if old in text:
        text = text.replace(old, new)
    if "tools/scripts/verify-add.sh" not in text:
        text += "\n\n## Reusable Add Verification\n\n```bash\ntools/scripts/verify-add.sh\n```\n"
    deliverable_path.write_text(text, encoding="utf-8")
PY

cargo fmt

echo
echo "Updated WP-E12-004 verification flow."
echo
echo "Added:"
echo "  tools/scripts/verify-add.sh"
echo
echo "Run:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-add.sh"
echo "  tools/scripts/verify.sh"
