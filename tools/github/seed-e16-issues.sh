#!/usr/bin/env bash
set -euo pipefail

REPO="${REPO:-thomascarter613/monad-workspace}"

require_command() {
  local command_name="$1"

  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "Missing required command: $command_name" >&2
    exit 1
  fi
}

issue_number_by_exact_title() {
  local title="$1"

  gh issue list \
    --repo "$REPO" \
    --state all \
    --search "\"$title\" in:title" \
    --limit 300 \
    | awk -v wanted="$title" -F '\t' '
        {
          for (i = 1; i <= NF; i++) {
            if ($i == wanted) {
              number = $1
              gsub(/^#/, "", number)
              print number
              exit
            }
          }
        }
      '
}

create_issue_once() {
  local title="$1"
  local body_file="$2"

  local existing_number
  existing_number="$(issue_number_by_exact_title "$title")"

  if [[ -n "$existing_number" ]]; then
    echo "$existing_number"
    return 0
  fi

  local issue_url
  issue_url="$(
    gh issue create \
      --repo "$REPO" \
      --title "$title" \
      --body-file "$body_file"
  )"

  echo "$issue_url" | sed -E 's#.*/issues/([0-9]+).*#\1#'
}

make_body_file() {
  local file
  file="$(mktemp)"
  cat > "$file"
  echo "$file"
}

create_epic() {
  local epic_id="$1"
  local epic_title="$2"
  local product_area="$3"
  local objective="$4"
  local user_value="$5"
  local scope_in="$6"
  local scope_out="$7"
  local work_packets="$8"
  local definition_of_done="$9"

  local title="[Epic]: ${epic_id} — ${epic_title}"

  local body_file
  body_file="$(make_body_file <<EOF
## Epic ID

${epic_id}

## Epic Title

${epic_title}

## Product Area

${product_area}

## Objective

${objective}

## User Value

${user_value}

## Scope

### In scope

${scope_in}

### Out of scope

${scope_out}

## Work Packets

${work_packets}

## Definition of Done

${definition_of_done}

## Verification Strategy

\`\`\`bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
\`\`\`

## Priority

High

## Size

Large
EOF
)"

  local issue_number
  issue_number="$(create_issue_once "$title" "$body_file")"
  rm -f "$body_file"

  echo "$issue_number"
}

create_work_packet() {
  local parent_epic_id="$1"
  local parent_issue_number="$2"
  local work_packet_id="$3"
  local work_packet_title="$4"
  local product_area="$5"
  local objective="$6"
  local scope_in="$7"
  local scope_out="$8"
  local expected_files="$9"
  local expected_result="${10}"
  local commit_message="${11}"
  local size="${12:-Medium}"

  local title="[Work Packet]: ${work_packet_id} — ${work_packet_title}"

  local body_file
  body_file="$(make_body_file <<EOF
## Work Packet ID

${work_packet_id}

## Parent Epic ID

${parent_epic_id}

## Parent Epic Issue

#${parent_issue_number}

## Work Packet Title

${work_packet_title}

## Product Area

${product_area}

## Objective

${objective}

## Scope

### In scope

${scope_in}

### Out of scope

${scope_out}

## Expected Files or Directories Affected

${expected_files}

## Tasks

- [ ] Confirm current repository state.
- [ ] Implement the scoped work.
- [ ] Add or update tests.
- [ ] Update documentation if behavior changes.
- [ ] Verify formatting.
- [ ] Verify tests.
- [ ] Verify clippy.
- [ ] Verify root verification.
- [ ] Commit as one atomic Conventional Commit.
- [ ] Close this work packet with verification evidence.

## Verification Commands / Evidence

\`\`\`bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
\`\`\`

## Expected Result

${expected_result}

## Commit Message

\`\`\`bash
git commit -m "${commit_message}"
\`\`\`

## Priority

High

## Size

${size}
EOF
)"

  local issue_number
  issue_number="$(create_issue_once "$title" "$body_file")"
  rm -f "$body_file"

  echo "$issue_number"
}

add_epic_child_index_comment() {
  local epic_issue_number="$1"
  local comment_body="$2"

  gh issue comment "$epic_issue_number" \
    --repo "$REPO" \
    --body "$comment_body"
}

require_command gh

if ! gh auth status >/dev/null 2>&1; then
  echo "GitHub CLI is not authenticated. Run: gh auth login" >&2
  exit 1
fi

echo "Seeding E16 issues in repo: $REPO"
echo

E16_SCOPE_IN="$(cat <<'EOF'
- Define the first MVP-safe release contract.
- Add release planning and go/no-go checks.
- Add version/tag validation.
- Add binary artifact packaging and checksum generation.
- Add release notes/changelog verification.
- Add GitHub release draft workflow or documentation.
- Add release evidence reports and smoke tests.
EOF
)"

E16_SCOPE_OUT="$(cat <<'EOF'
- Crates.io publishing unless explicitly approved.
- Homebrew publishing.
- npm package publishing.
- Installer generation unless explicitly approved.
- Hosted/SaaS launch.
- Automatic public release without verification.
- Signing/cosign/SBOM unless explicitly moved into scope.
EOF
)"

E16_WORK_PACKETS="$(cat <<'EOF'
- WP-E16-001 — Define `monad release` contract and release boundary
- WP-E16-002 — Add release readiness model and go/no-go plan
- WP-E16-003 — Add version and tag validation
- WP-E16-004 — Add binary artifact packaging and checksums
- WP-E16-005 — Add release notes and changelog validation
- WP-E16-006 — Add GitHub release draft workflow and release evidence tests
EOF
)"

E16_DOD="$(cat <<'EOF'
- `monad release --dry-run` can evaluate release readiness without tagging or publishing.
- Release plans include version, tag, artifact, changelog, verification, and blocker status.
- Tag validation prevents malformed or conflicting release tags.
- Binary artifact packaging and checksum generation are supported or documented.
- Release notes/changelog validation exists.
- GitHub release draft process is documented or automated in a bounded way.
- Root verification passes.
EOF
)"

E16_USER_VALUE="$(cat <<'EOF'
Once users can initialize, grow, run, sync, and diagnose a Monad-managed repository, they need a trusted way to prepare a release. E16 gives Monad a disciplined release path: verify first, plan the release, package artifacts, produce checksums, validate notes, and only then decide whether to tag or publish.

This keeps release work evidence-backed and prevents accidental public claims beyond what the binary supports.
EOF
)"

E16_NUMBER="$(create_epic \
  "E16" \
  "Release and Distribution Foundation" \
  "Release Engineering / Distribution / Verification" \
  "Implement the first MVP-safe `monad release` foundation so Monad can plan, validate, package, and document releases without prematurely publishing packages or launching hosted services." \
  "$E16_USER_VALUE" \
  "$E16_SCOPE_IN" \
  "$E16_SCOPE_OUT" \
  "$E16_WORK_PACKETS" \
  "$E16_DOD"
)"

echo "E16 issue: #$E16_NUMBER"

WP_E16_001="$(create_work_packet \
  "E16" \
  "$E16_NUMBER" \
  "WP-E16-001" \
  "Define \`monad release\` contract and release boundary" \
  "Release Engineering / Product Design" \
  "Define the release command contract, release boundary, allowed release modes, and non-goals for first MVP release/distribution behavior." \
  "- Define `monad release` command shape.
- Define `--dry-run` behavior.
- Define source-only, binary-artifact, package-published, and deferred release modes.
- Define what release may validate.
- Define what release must not publish automatically." \
  "- Implementing release execution.
- Publishing packages.
- Creating installers.
- Hosted launch." \
  "- \`docs/commands/RELEASE.md\`
- \`docs/release/RELEASE-CONTRACT.md\`
- ADR if needed" \
  "Monad has a documented release contract and bounded release modes before implementation begins." \
  "docs(release): define release command contract" \
  "Medium"
)"

WP_E16_002="$(create_work_packet \
  "E16" \
  "$E16_NUMBER" \
  "WP-E16-002" \
  "Add release readiness model and go/no-go plan" \
  "Core Runtime / Release Planning" \
  "Add the core model for evaluating release readiness and rendering go/no-go release plans." \
  "- Add release readiness model.
- Add blocker/warning/pass status.
- Include verification, changelog, version, tag, artifact, and distribution posture checks.
- Add deterministic release plan rendering.
- Add tests." \
  "- Tag creation.
- Artifact upload.
- Publishing.
- Installer generation." \
  "- \`crates/monad-core/src/release.rs\`
- \`crates/monad-core/src/release/\`
- tests" \
  "`monad release --dry-run` can be backed by a deterministic release readiness model." \
  "feat(release): add release readiness model" \
  "Large"
)"

WP_E16_003="$(create_work_packet \
  "E16" \
  "$E16_NUMBER" \
  "WP-E16-003" \
  "Add version and tag validation" \
  "Release Engineering / Versioning" \
  "Add validation for release version strings, tag names, and tag conflict detection." \
  "- Validate semantic/pre-release tag shapes.
- Check current manifest/package version where available.
- Check whether a tag already exists.
- Report malformed or conflicting tags.
- Add tests." \
  "- Creating tags.
- Publishing releases.
- Editing versions automatically.
- Multi-package version orchestration." \
  "- \`crates/monad-core/src/release/\`
- \`crates/monad-cli/src/main.rs\`
- tests" \
  "Monad can validate release tags and versions before a release is attempted." \
  "feat(release): add version and tag validation" \
  "Medium"
)"

WP_E16_004="$(create_work_packet \
  "E16" \
  "$E16_NUMBER" \
  "WP-E16-004" \
  "Add binary artifact packaging and checksums" \
  "Distribution / Artifacts" \
  "Add first MVP-safe packaging and checksum support for locally built Monad binaries." \
  "- Define artifact naming convention.
- Package release binary into archive.
- Generate SHA-256 checksum.
- Record artifact metadata.
- Add tests or script-level verification where feasible." \
  "- Cross-platform matrix unless explicitly added.
- Signing.
- SBOM generation.
- Uploading artifacts to package registries." \
  "- \`crates/monad-core/src/release/\`
- \`tools/scripts/\`
- \`.github/workflows/\` if needed
- tests" \
  "Monad can produce or validate a local binary archive and checksum for release evaluation." \
  "feat(release): add binary artifact packaging" \
  "Large"
)"

WP_E16_005="$(create_work_packet \
  "E16" \
  "$E16_NUMBER" \
  "WP-E16-005" \
  "Add release notes and changelog validation" \
  "Release Documentation" \
  "Add validation that release notes and changelog entries exist and match the intended release boundary." \
  "- Check changelog presence.
- Check release notes presence.
- Verify implemented/deferred/not-implemented sections exist.
- Report missing or stale release documentation.
- Add tests where feasible." \
  "- Writing full release notes automatically.
- Publishing release notes.
- Marketing launch content." \
  "- \`docs/release/\`
- \`CHANGELOG.md\`
- \`crates/monad-core/src/release/\`
- tests" \
  "Monad can detect whether release notes and changelog evidence are ready for a release candidate." \
  "feat(release): add release notes validation" \
  "Medium"
)"

WP_E16_006="$(create_work_packet \
  "E16" \
  "$E16_NUMBER" \
  "WP-E16-006" \
  "Add GitHub release draft workflow and release evidence tests" \
  "Release Automation / Evidence" \
  "Add a bounded GitHub release draft workflow or documented release process plus smoke tests for release dry-run behavior." \
  "- Add release dry-run CLI path if not already wired.
- Add release evidence report.
- Add GitHub release draft documentation or workflow.
- Add smoke tests.
- Ensure no automatic public publishing without explicit approval." \
  "- Publishing packages.
- Automatically pushing tags without approval.
- Hosted launch.
- Installer publication." \
  "- \`crates/monad-cli/src/main.rs\`
- \`crates/monad-core/src/release/\`
- \`.github/workflows/\`
- \`docs/release/\`
- tests" \
  "`monad release --dry-run` is test-covered and release drafting is documented or automated safely." \
  "test(release): add release dry-run evidence tests" \
  "Medium"
)"

E16_COMMENT="$(cat <<EOF
## Child Work Packets

- #${WP_E16_001} — WP-E16-001 — Define \`monad release\` contract and release boundary
- #${WP_E16_002} — WP-E16-002 — Add release readiness model and go/no-go plan
- #${WP_E16_003} — WP-E16-003 — Add version and tag validation
- #${WP_E16_004} — WP-E16-004 — Add binary artifact packaging and checksums
- #${WP_E16_005} — WP-E16-005 — Add release notes and changelog validation
- #${WP_E16_006} — WP-E16-006 — Add GitHub release draft workflow and release evidence tests
EOF
)"

add_epic_child_index_comment "$E16_NUMBER" "$E16_COMMENT"

echo
echo "Created/confirmed E16 roadmap issues:"
echo "E16 #$E16_NUMBER"
echo "  WP-E16-001 #$WP_E16_001"
echo "  WP-E16-002 #$WP_E16_002"
echo "  WP-E16-003 #$WP_E16_003"
echo "  WP-E16-004 #$WP_E16_004"
echo "  WP-E16-005 #$WP_E16_005"
echo "  WP-E16-006 #$WP_E16_006"
