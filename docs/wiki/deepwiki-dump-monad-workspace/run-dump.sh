#!/usr/bin/env bash
set -Eeuo pipefail

REPO_NAME="${DEEPWIKI_REPO:-thomascarter613/monad-workspace}"
TARGET_ROOT="${1:-$PWD}"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
RUNNER_DIR="${TARGET_ROOT}/.artifacts/tools/deepwiki-dumper"
OUT_BASE="${TARGET_ROOT}/docs/deepwiki"

mkdir -p "${RUNNER_DIR}" "${OUT_BASE}"
cp "${SCRIPT_DIR}/package.json" "${RUNNER_DIR}/package.json"
cp "${SCRIPT_DIR}/dump-deepwiki.mjs" "${RUNNER_DIR}/dump-deepwiki.mjs"

cd "${RUNNER_DIR}"

if command -v npm >/dev/null 2>&1; then
  npm install
else
  echo "npm is required for this runner. Install Node.js/npm, then rerun." >&2
  exit 1
fi

node dump-deepwiki.mjs "${REPO_NAME}" "${OUT_BASE}"
