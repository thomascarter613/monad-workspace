#!/usr/bin/env bash
set -euo pipefail

REPO="thomascarter613/monad-workspace"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

ensure_label() {
  local name="$1"
  local color="$2"
  local description="$3"

  if gh label list --repo "$REPO" --search "$name" --json name --jq '.[].name' | grep -Fxq "$name"; then
    gh label edit "$name" \
      --repo "$REPO" \
      --color "$color" \
      --description "$description" >/dev/null
  else
    gh label create "$name" \
      --repo "$REPO" \
      --color "$color" \
      --description "$description" >/dev/null
  fi
}

existing_issue_url() {
  local title="$1"

  gh issue list \
    --repo "$REPO" \
    --state all \
    --search "$title in:title" \
    --json title,url \
    --jq ".[] | select(.title == \"$title\") | .url" \
    | head -n 1
}

create_issue_if_missing() {
  local title="$1"
  local labels="$2"
  local body_file="$3"

  local existing
  existing="$(existing_issue_url "$title")"

  if [[ -n "$existing" ]]; then
    echo "Exists: $title"
    echo "$existing"
    return 0
  fi

  echo "Creating: $title" >&2

  gh issue create \
    --repo "$REPO" \
    --title "$title" \
    --label "$labels" \
    --body-file "$body_file"
}

issue_number_from_url() {
  local url="$1"
  sed -E 's#.*/issues/([0-9]+).*#\1#' <<<"$url"
}

write_wp_body() {
  local file="$1"
  local wp_id="$2"
  local title="$3"
  local product_area="$4"
  local objective="$5"
  local in_scope="$6"
  local out_scope="$7"
  local expected_files="$8"
  local verification="$9"
  local commit_msg="${10}"
  local epic_number="${11}"
  local epic_url="${12}"

  cat > "$file" <<EOF_BODY
## Work Packet ID

${wp_id}

## Parent Epic ID

E8

## Parent Epic Issue

#${epic_number} — ${epic_url}

## Work Packet Title

${title}

## Product Area

${product_area}

## Objective

${objective}

## Scope

### In scope

${in_scope}

### Out of scope

${out_scope}

## Expected Files or Directories Affected

${expected_files}

## Tasks

- [ ] Confirm current repository state.
- [ ] Implement the scoped work.
- [ ] Verify formatting.
- [ ] Verify tests.
- [ ] Verify Clippy.
- [ ] Record evidence or update documentation where appropriate.
- [ ] Commit as one atomic release-preparation commit.

## Verification Commands / Evidence

\`\`\`bash
${verification}
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
git commit -m "${commit_msg}"
\`\`\`

## Risks / Blockers / Open Questions

- Keep this work focused on release preparation.
- Do not expand MVP scope.
- Record blockers honestly rather than hiding failures.

## Priority

P1 High

## Size

M
EOF_BODY
}

echo "==> Ensuring E8 labels exist"

ensure_label "epic:e8" "5319e7" "Epic E8: MVP candidate cut and release preparation."
ensure_label "release-prep" "0052cc" "Release preparation, candidate cut, changelog, build, and verification work."
ensure_label "scope-freeze" "0e8a16" "Scope freeze and cut-line decisions."
ensure_label "area:release" "1d76db" "Release readiness, versioning, packaging, changelog, and distribution preparation."
ensure_label "area:docs" "1d76db" "Documentation, README, guides, tutorials, and repo-native written artifacts."
ensure_label "area:verification" "1d76db" "Checks, evidence, validation, reporting, and verification workflows."
ensure_label "area:ci" "1d76db" "CI, automation, local/remote verification parity, and workflow checks."
ensure_label "type:epic" "5319e7" "Large body of work composed of multiple work packets."
ensure_label "type:work-packet" "8250df" "Bounded delivery unit with objective, scope, deliverables, verification, and commit."
ensure_label "priority:p1" "d93f0b" "High priority; important for the current milestone or near-term progress."
ensure_label "status:ready" "0e8a16" "Clear, scoped, and ready to start."
ensure_label "needs-verification" "fbca04" "Requires test, check, review, or evidence before completion."
ensure_label "context-update-required" "5319e7" "Requires Monad context, handoff, or current-state documentation update."

echo "==> Creating or reusing E8 epic"

epic_body="$tmpdir/e8-epic.md"
cat > "$epic_body" <<'EOF_BODY'
## Epic ID

E8

## Epic Title

MVP Candidate Cut and Release Preparation

## Product Area

Release Readiness

## Objective

Prepare Monad for a disciplined internal MVP candidate cut without expanding the MVP scope beyond the cut line defined in `docs/project/MVP-READINESS-REPORT.md`.

## Scope

### In scope

- Freeze MVP candidate scope.
- Add changelog and release notes foundation.
- Harden version and build metadata.
- Add installation and local build documentation.
- Run release-candidate verification audit.
- Cut an internal MVP candidate tag after verification passes.

### Out of scope

- Public release.
- Crates.io publishing.
- Installer generation.
- Hosted service launch.
- Marketing launch.
- Autonomous agent execution.
- Apply/write evolution behavior.
- MCP server release.
- Enterprise SaaS features.

## Work Packets

- WP-E8-001 — Freeze MVP candidate scope
- WP-E8-002 — Add changelog and release notes foundation
- WP-E8-003 — Harden version and build metadata
- WP-E8-004 — Add installation and local build documentation
- WP-E8-005 — Run release-candidate verification audit
- WP-E8-006 — Cut internal MVP candidate tag

## Definition of Done

- All E8 work packets are complete.
- Final verification passes.
- MVP candidate scope is frozen.
- Changelog/release notes foundation exists.
- Build/install documentation exists.
- Release-candidate audit is recorded.
- Internal MVP candidate tag is cut only after verification passes.

## Priority

P1 High
EOF_BODY

epic_title="[Epic]: E8 — MVP Candidate Cut and Release Preparation"
epic_url="$(create_issue_if_missing \
  "$epic_title" \
  "type:epic,epic:e8,release-prep,area:release,priority:p1,status:ready,needs-verification" \
  "$epic_body")"

epic_url="$(tail -n 1 <<<"$epic_url")"
epic_number="$(issue_number_from_url "$epic_url")"

echo "E8 epic: #$epic_number — $epic_url"

echo "==> Creating or reusing E8 work packets"

wp1="$tmpdir/wp-e8-001.md"
write_wp_body \
  "$wp1" \
  "WP-E8-001" \
  "Freeze MVP candidate scope" \
  "Product / Release Scope" \
  "Convert the MVP readiness report into an explicit scope-freeze artifact that separates MVP-candidate behavior from deferred future capabilities." \
  "- Define the MVP candidate cut line.
- List included features.
- List deferred features.
- List prohibited release claims.
- Add scope-freeze evidence." \
  "- New runtime behavior.
- New commands.
- Public release.
- Marketing copy." \
  "- \`docs/project/MVP-SCOPE-FREEZE.md\`
- \`docs/project/MVP-READINESS-REPORT.md\`
- \`.monad/context/\` if context state needs updating" \
  "git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh" \
  "docs(project): freeze mvp candidate scope" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E8-001 — Freeze MVP candidate scope" \
  "type:work-packet,epic:e8,release-prep,scope-freeze,area:release,area:docs,priority:p1,status:ready,needs-verification,context-update-required" \
  "$wp1"

wp2="$tmpdir/wp-e8-002.md"
write_wp_body \
  "$wp2" \
  "WP-E8-002" \
  "Add changelog and release notes foundation" \
  "Release Documentation" \
  "Create the initial changelog and release notes foundation for tracking MVP candidate changes without publishing a public release yet." \
  "- Add \`CHANGELOG.md\`.
- Add release notes template.
- Add unreleased MVP candidate section.
- Document release-note rules." \
  "- Automated release publishing.
- Semantic-release setup.
- Package registry publishing.
- Public announcement." \
  "- \`CHANGELOG.md\`
- \`docs/release/README.md\`
- \`docs/release/RELEASE-NOTES-TEMPLATE.md\`" \
  "git status --short
find docs/release -maxdepth 3 -type f | sort
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh" \
  "docs(release): add changelog and release notes foundation" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E8-002 — Add changelog and release notes foundation" \
  "type:work-packet,epic:e8,release-prep,area:release,area:docs,priority:p1,status:ready,needs-verification" \
  "$wp2"

wp3="$tmpdir/wp-e8-003.md"
write_wp_body \
  "$wp3" \
  "WP-E8-003" \
  "Harden version and build metadata" \
  "Build / Release Metadata" \
  "Ensure Monad version and build metadata are explicit, reviewable, and aligned with the internal MVP candidate cut." \
  "- Review Cargo workspace version.
- Review CLI version output.
- Decide internal MVP candidate version identifier.
- Document version policy for pre-public releases." \
  "- Release automation.
- Binary signing.
- Installer generation.
- Public package publishing." \
  "- \`Cargo.toml\`
- \`crates/monad-cli/src/main.rs\` if version output needs adjustment
- \`docs/release/VERSIONING.md\`" \
  "cargo run -p monad-cli -- version
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh" \
  "chore(release): harden version and build metadata" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E8-003 — Harden version and build metadata" \
  "type:work-packet,epic:e8,release-prep,area:release,area:ci,priority:p1,status:ready,needs-verification" \
  "$wp3"

wp4="$tmpdir/wp-e8-004.md"
write_wp_body \
  "$wp4" \
  "WP-E8-004" \
  "Add installation and local build documentation" \
  "Developer Documentation" \
  "Document how to build, run, and verify Monad locally as an internal MVP candidate." \
  "- Add local build guide.
- Add local run guide.
- Add verification guide.
- Document supported development assumptions.
- Make command examples match \`monad-cli\` package and \`monad\` binary." \
  "- Installer docs.
- Public distribution docs.
- Hosted deployment docs.
- Cloud environment docs." \
  "- \`README.md\`
- \`docs/development/LOCAL-BUILD.md\`
- \`docs/development/LOCAL-VERIFY.md\`
- \`docs/project/MVP-COMMAND-REFERENCE.md\` if needed" \
  "grep -R \"cargo run -p monad-cli\" README.md docs || true
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh" \
  "docs(dev): add local build and verification guide" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E8-004 — Add installation and local build documentation" \
  "type:work-packet,epic:e8,release-prep,area:docs,area:release,priority:p1,status:ready,needs-verification" \
  "$wp4"

wp5="$tmpdir/wp-e8-005.md"
write_wp_body \
  "$wp5" \
  "WP-E8-005" \
  "Run release-candidate verification audit" \
  "Verification / Release Readiness" \
  "Run and record a release-candidate verification audit for the internal MVP candidate cut." \
  "- Run full verification.
- Record environment and command evidence.
- Record pass/fail status.
- Identify blockers.
- Create a release-candidate audit artifact." \
  "- Fixing major blockers unless they are tiny documentation corrections.
- Public release.
- Tag creation." \
  "- \`docs/release/MVP-CANDIDATE-VERIFICATION-AUDIT.md\`
- \`.artifacts/\` audit outputs if intentionally retained
- \`.monad/context/\` if context state needs updating" \
  "git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
git status --short" \
  "docs(release): record mvp candidate verification audit" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E8-005 — Run release-candidate verification audit" \
  "type:work-packet,epic:e8,release-prep,area:verification,area:release,priority:p1,status:ready,needs-verification,context-update-required" \
  "$wp5"

wp6="$tmpdir/wp-e8-006.md"
write_wp_body \
  "$wp6" \
  "WP-E8-006" \
  "Cut internal MVP candidate tag" \
  "Release Candidate Cut" \
  "Cut an internal MVP candidate tag only after scope freeze, documentation, version metadata, and verification audit are complete." \
  "- Confirm E8 work packets are complete.
- Confirm final verification is green.
- Confirm working tree is clean.
- Decide internal tag name.
- Create tag locally.
- Push tag after explicit review." \
  "- Public release.
- Crates.io publishing.
- Installer generation.
- Marketing launch.
- Hosted deployment." \
  "- Git tag
- \`docs/release/MVP-CANDIDATE-TAG-RECORD.md\`
- \`.monad/context/\` if context state needs updating" \
  "git status --short
git log --oneline --max-count=12
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
git status --short" \
  "chore(release): cut internal mvp candidate tag" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E8-006 — Cut internal MVP candidate tag" \
  "type:work-packet,epic:e8,release-prep,area:release,area:verification,priority:p1,status:ready,needs-verification,context-update-required" \
  "$wp6"

echo
echo "E8 issue creation complete."
