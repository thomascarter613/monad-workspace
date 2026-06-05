#!/usr/bin/env bash
set -euo pipefail

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

assert_file_missing() {
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

basic_temp=""
polyglot_temp=""
conflict_temp=""
trap 'rm -rf "${basic_temp:-}" "${polyglot_temp:-}" "${conflict_temp:-}"' EXIT

echo "==> Verifying init dry-run commands"

basic_dry_run="$(run_from_repo cargo run -p monad-cli -- init --preset=basic --dry-run)"
assert_output_contains "${basic_dry_run}" "Monad init dry-run plan"
assert_output_contains "${basic_dry_run}" "preset: minimal"
assert_output_contains "${basic_dry_run}" "Template source:"
assert_output_contains "${basic_dry_run}" "No files were written."

minimal_dry_run="$(run_from_repo cargo run -p monad-cli -- init --preset=minimal --dry-run)"
assert_output_contains "${minimal_dry_run}" "Monad init dry-run plan"
assert_output_contains "${minimal_dry_run}" "preset: minimal"

polyglot_dry_run="$(run_from_repo cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run)"
assert_output_contains "${polyglot_dry_run}" "Monad init dry-run plan"
assert_output_contains "${polyglot_dry_run}" "preset: polyglot-minimal"
assert_output_contains "${polyglot_dry_run}" "apps/.gitkeep"
assert_output_contains "${polyglot_dry_run}" "packages/.gitkeep"
assert_output_contains "${polyglot_dry_run}" "services/.gitkeep"
assert_output_contains "${polyglot_dry_run}" "tools/.gitkeep"

echo "==> Verifying init --yes basic apply in temporary directory"

basic_temp="$(mktemp -d)"
run_from_temp "${basic_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --preset=basic --yes

assert_file_exists "${basic_temp}/monad.toml"
assert_file_exists "${basic_temp}/README.md"
assert_file_exists "${basic_temp}/docs/README.md"
assert_file_exists "${basic_temp}/work/README.md"
assert_file_exists "${basic_temp}/.monad/.gitignore"

echo "==> Verifying init --yes polyglot-minimal apply in temporary directory"

polyglot_temp="$(mktemp -d)"
run_from_temp "${polyglot_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --preset=polyglot-minimal --yes

assert_file_exists "${polyglot_temp}/monad.toml"
assert_file_exists "${polyglot_temp}/README.md"
assert_file_exists "${polyglot_temp}/docs/README.md"
assert_file_exists "${polyglot_temp}/work/README.md"
assert_file_exists "${polyglot_temp}/.monad/.gitignore"
assert_file_exists "${polyglot_temp}/apps/.gitkeep"
assert_file_exists "${polyglot_temp}/packages/.gitkeep"
assert_file_exists "${polyglot_temp}/services/.gitkeep"
assert_file_exists "${polyglot_temp}/tools/.gitkeep"

echo "==> Verifying init refuses conflicts"

conflict_temp="$(mktemp -d)"
printf '# Existing README\n' > "${conflict_temp}/README.md"

if run_from_temp "${conflict_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes; then
  echo "Expected init --yes to fail when README.md already exists." >&2
  exit 1
fi

assert_file_exists "${conflict_temp}/README.md"
assert_file_missing "${conflict_temp}/monad.toml"

echo "==> Verifying init rejects conflicting mode flags"

if run_from_repo cargo run -p monad-cli -- init --dry-run --yes; then
  echo "Expected init --dry-run --yes to fail because modes conflict." >&2
  exit 1
fi

echo "==> Verifying init help mentions presets"

help_output="$(run_from_repo cargo run -p monad-cli -- --help)"
assert_output_contains "${help_output}" "init --dry-run"
assert_output_contains "${help_output}" "init --preset=basic --dry-run"
assert_output_contains "${help_output}" "init --yes"

echo
echo "Monad init smoke verification passed."
