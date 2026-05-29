#!/usr/bin/env bash
set -euo pipefail

REPO="${REPO:-thomascarter613/monad-workspace}"
EPIC_NUMBER="${EPIC_NUMBER:-53}"
EPIC_URL="${EPIC_URL:-https://github.com/thomascarter613/monad-workspace/issues/53}"

require_command() {
  local command_name="${1:-}"

  if [[ -z "$command_name" ]]; then
    echo "require_command called without a command name" >&2
    exit 1
  fi

  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "Required command not found: $command_name" >&2
    exit 1
  fi
}

ensure_label() {
  local name="${1:-}"
  local color="${2:-}"
  local description="${3:-}"

  if [[ -z "$name" || -z "$color" || -z "$description" ]]; then
    echo "ensure_label called with missing arguments" >&2
    exit 1
  fi

  gh label create "$name" \
    --repo "$REPO" \
    --color "$color" \
    --description "$description" \
    --force >/dev/null
}

create_issue_from_file() {
  local title="${1:-}"
  local body_file="${2:-}"
  shift 2

  if [[ -z "$title" || -z "$body_file" ]]; then
    echo "create_issue_from_file called with missing title/body_file" >&2
    exit 1
  fi

  local args=(
    issue create
    --repo "$REPO"
    --title "$title"
    --body-file "$body_file"
  )

  local label
  for label in "$@"; do
    args+=(--label "$label")
  done

  gh "${args[@]}"
}

require_command gh

echo "Ensuring labels in $REPO..."

ensure_label "type:work-packet" "8250df" "Bounded delivery unit with objective, scope, deliverables, verification, and commit."
ensure_label "area:cli" "1d76db" "Command-line interface, command routing, help output, and CLI UX."
ensure_label "area:docs" "1d76db" "Documentation, README, guides, tutorials, and repo-native written artifacts."
ensure_label "area:verification" "1d76db" "Checks, evidence, validation, reporting, and verification workflows."
ensure_label "area:testing" "1d76db" "Unit tests, integration tests, smoke tests, regression tests, and test infrastructure."
ensure_label "area:evolution" "1d76db" "Safe repository changes, generators, migrations, file operations, and apply workflows."
ensure_label "area:release" "1d76db" "Release readiness, versioning, packaging, changelog, and distribution preparation."
ensure_label "area:policy" "1d76db" "Policy, governance, safety boundaries, approvals, and enforcement rules."
ensure_label "priority:p1" "d93f0b" "High priority; important for the current milestone or near-term progress."
ensure_label "status:ready" "0e8a16" "Clear, scoped, and ready to start."
ensure_label "needs-verification" "fbca04" "Requires test, check, review, or evidence before completion."
ensure_label "context-update-required" "5319e7" "Requires Monad context, handoff, or current-state documentation update."
ensure_label "mvp-hardening" "0052cc" "MVP hardening, stabilization, polish, and release-readiness work."

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

write_body() {
  local path="$1"
  local id="$2"
  local title="$3"
  local area="$4"
  local objective="$5"
  local scope="$6"
  local files="$7"
  local verification="$8"
  local commit_message="$9"

  cat > "$path" <<BODY
## Work Packet ID

$id

## Parent Epic ID

E7

## Parent Epic Issue

#$EPIC_NUMBER — $EPIC_URL

## Work Packet Title

$title

## Product Area

$area

## Objective

$objective

## Scope

$scope

## Expected Files or Directories Affected

$files

## Tasks

- [ ] Confirm current repository state.
- [ ] Implement the scoped work.
- [ ] Verify formatting.
- [ ] Verify tests.
- [ ] Verify Clippy.
- [ ] Record evidence or update documentation where appropriate.
- [ ] Commit as one atomic MVP-hardening commit.

## Verification Commands / Evidence

\`\`\`bash
$verification
\`\`\`

## Expected Result After Verification

- Formatting passes where applicable.
- Tests pass where applicable.
- Clippy passes where applicable.
- The work packet outcome is visible and reviewable.
- No unrelated feature expansion is introduced.

## Definition of Done

- [ ] Scoped work is complete.
- [ ] Verification is complete or blockers are documented.
- [ ] Documentation is updated where needed.
- [ ] Atomic commit completed.

## Recommended Conventional Commit

\`\`\`bash
git commit -m "$commit_message"
\`\`\`

## Risks / Blockers / Open Questions

- Keep this work focused on MVP hardening.
- Do not add major new features unless they directly support MVP readiness.
- Record blockers honestly rather than hiding failures.

## Priority

P1 High

## Size

M
BODY
}

WP1="$TMPDIR/wp-e7-001.md"
WP2="$TMPDIR/wp-e7-002.md"
WP3="$TMPDIR/wp-e7-003.md"
WP4="$TMPDIR/wp-e7-004.md"
WP5="$TMPDIR/wp-e7-005.md"
WP6="$TMPDIR/wp-e7-006.md"

write_body \
  "$WP1" \
  "WP-E7-001" \
  "Run foundation closure audit" \
  "MVP Hardening / Verification" \
  "Run the full foundation closure verification sweep and record the result so Monad has a factual baseline before further MVP hardening work begins." \
  "### In scope

- Run closure verification commands.
- Record command results.
- Identify blockers.
- Identify warnings or inconsistencies.
- Update the foundation closure report or create an audit appendix.

### Out of scope

- Major feature development.
- New command implementation.
- New agent behavior.
- Full release packaging." \
  "- \`docs/project/FOUNDATION-CLOSURE-REPORT.md\`
- \`docs/project/FOUNDATION-CLOSURE-AUDIT.md\` if a separate audit artifact is preferred
- possibly \`.monad/context/\` if context state needs updating" \
  "git status --short
git log --oneline --max-count=12

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
cargo run -p monad-cli -- plan \"explain this repository\"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run

tools/scripts/verify.sh" \
  "docs(project): record foundation closure audit"

write_body \
  "$WP2" \
  "WP-E7-002" \
  "Normalize CLI help and command UX" \
  "CLI UX" \
  "Normalize Monad CLI help output, command examples, unsupported flag behavior, missing-argument errors, and user-facing messages for the current MVP command surface." \
  "### In scope

- Review help output.
- Ensure current commands are listed.
- Ensure examples work.
- Normalize missing argument errors.
- Normalize unsupported flag errors.
- Ensure dry-run language is clear.

### Out of scope

- New major commands.
- External provider integrations.
- Full CLI redesign." \
  "- \`crates/monad-cli/src/main.rs\`
- CLI tests if already present or newly added
- docs that reference command usage" \
  "cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

cargo run -p monad-cli -- --help
cargo run -p monad-cli -- plan
cargo run -p monad-cli -- plan \"explain this repository\"
cargo run -p monad-cli -- evolve verify-baseline
cargo run -p monad-cli -- evolve verify-baseline --dry-run" \
  "fix(cli): normalize help and command ux"

write_body \
  "$WP3" \
  "WP-E7-003" \
  "Harden command smoke tests" \
  "Testing / Verification" \
  "Add or strengthen command smoke tests for Monad's current MVP command surface so regressions are caught before release-readiness work." \
  "### In scope

- Smoke tests for help/version.
- Smoke tests for inspect.
- Smoke tests for check.
- Smoke tests for plan.
- Smoke tests for dry-run evolution commands.
- Failure tests for missing arguments where practical.

### Out of scope

- Full end-to-end test framework.
- Real model provider tests.
- MCP server tests.
- Remote Git tests.
- Deployment tests." \
  "- \`crates/monad-cli/\`
- \`crates/monad-core/\` if helper test utilities are needed
- \`tests/\` if integration tests are preferred
- \`tools/scripts/verify.sh\` if smoke tests are integrated into verification" \
  "cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh" \
  "test(cli): harden command smoke coverage"

write_body \
  "$WP4" \
  "WP-E7-004" \
  "Align documentation with implemented behavior" \
  "Documentation" \
  "Update Monad documentation so it accurately describes the implemented current command surface, safety boundaries, and MVP hardening state." \
  "### In scope

- README alignment.
- Getting-started command path.
- Current command reference.
- Safety boundary clarification.
- Mark aspirational content clearly.
- Align package names in docs.
- Align verification commands.

### Out of scope

- Full public website.
- Marketing copy.
- Deep tutorial series." \
  "- \`README.md\`
- \`docs/project/FOUNDATION-CLOSURE-REPORT.md\`
- \`docs/architecture/\`
- \`docs/security/\`
- \`docs/ai/\`
- \`docs/workflow/\`" \
  "find README.md docs -maxdepth 4 -type f | sort
grep -R \"monad-workspace-cli\" README.md docs || true
grep -R \"cargo run -p monad-cli\" README.md docs || true
tools/scripts/verify.sh" \
  "docs: align mvp behavior and command reference"

write_body \
  "$WP5" \
  "WP-E7-005" \
  "Harden dry-run and no-write guarantees" \
  "Evolution / Safety" \
  "Strengthen the proof that dry-run and planning commands do not write files, mutate Git state, or perform external side effects." \
  "### In scope

- Audit dry-run commands.
- Audit monad plan.
- Add no-write tests where practical.
- Add documentation notes for dry-run behavior.
- Confirm dry-run wording is explicit.
- Confirm no Git mutation occurs.

### Out of scope

- Implement apply command.
- Implement file writes.
- Implement branch/worktree creation.
- Implement remote side effects.
- Implement autonomous agents." \
  "- \`crates/monad-core/src/file_ops/\`
- \`crates/monad-core/src/evolution/\`
- \`crates/monad-core/src/agents/\`
- \`crates/monad-cli/src/main.rs\`
- tests and docs as appropriate" \
  "git status --short

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

cargo run -p monad-cli -- plan \"explain this repository\"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run

git status --short" \
  "test(evolution): harden dry-run no-write guarantees"

write_body \
  "$WP6" \
  "WP-E7-006" \
  "Create MVP readiness report" \
  "Release Readiness" \
  "Create a concrete MVP readiness report that summarizes what is ready, what remains blocked, and what must happen before Monad can be treated as an MVP candidate." \
  "### In scope

- Summarize E7 hardening results.
- Identify MVP-ready features.
- Identify non-MVP future features.
- Identify release blockers.
- Define final MVP cut line.
- Recommend next milestone after MVP hardening.

### Out of scope

- Actual public release.
- Package publishing.
- Installer generation.
- Hosted service launch.
- Marketing launch." \
  "- \`docs/project/MVP-READINESS-REPORT.md\`
- \`docs/project/FOUNDATION-CLOSURE-REPORT.md\` if status needs updating
- \`.monad/context/\` if context state needs updating" \
  "find docs/project .monad/context -maxdepth 4 -type f | sort
git status --short
tools/scripts/verify.sh" \
  "docs(project): add mvp readiness report"

echo "Creating E7 work packets under epic #$EPIC_NUMBER..."

WP1_URL="$(create_issue_from_file \
  "[Work Packet]: WP-E7-001 — Run foundation closure audit" \
  "$WP1" \
  "type:work-packet" "area:verification" "area:docs" "priority:p1" "status:ready" "needs-verification" "context-update-required" "mvp-hardening")"

WP2_URL="$(create_issue_from_file \
  "[Work Packet]: WP-E7-002 — Normalize CLI help and command UX" \
  "$WP2" \
  "type:work-packet" "area:cli" "priority:p1" "status:ready" "needs-verification" "mvp-hardening")"

WP3_URL="$(create_issue_from_file \
  "[Work Packet]: WP-E7-003 — Harden command smoke tests" \
  "$WP3" \
  "type:work-packet" "area:testing" "area:verification" "area:cli" "priority:p1" "status:ready" "needs-verification" "mvp-hardening")"

WP4_URL="$(create_issue_from_file \
  "[Work Packet]: WP-E7-004 — Align documentation with implemented behavior" \
  "$WP4" \
  "type:work-packet" "area:docs" "priority:p1" "status:ready" "needs-verification" "context-update-required" "mvp-hardening")"

WP5_URL="$(create_issue_from_file \
  "[Work Packet]: WP-E7-005 — Harden dry-run and no-write guarantees" \
  "$WP5" \
  "type:work-packet" "area:evolution" "area:policy" "area:testing" "priority:p1" "status:ready" "needs-verification" "mvp-hardening")"

WP6_URL="$(create_issue_from_file \
  "[Work Packet]: WP-E7-006 — Create MVP readiness report" \
  "$WP6" \
  "type:work-packet" "area:release" "area:docs" "priority:p1" "status:ready" "needs-verification" "context-update-required" "mvp-hardening")"

cat <<SUMMARY

Created E7 work packet issue set in $REPO:

Epic:
  E7 — $EPIC_URL

Work packets:
  WP-E7-001 — $WP1_URL
  WP-E7-002 — $WP2_URL
  WP-E7-003 — $WP3_URL
  WP-E7-004 — $WP4_URL
  WP-E7-005 — $WP5_URL
  WP-E7-006 — $WP6_URL

Recommended next command:
  gh issue view "${WP1_URL##*/}" --repo "$REPO"

SUMMARY
