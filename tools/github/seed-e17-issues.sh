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

add_epic_child_index_comment_once() {
  local epic_issue_number="$1"
  local marker="$2"
  local comment_body="$3"

  if gh issue view "$epic_issue_number" --repo "$REPO" --comments | grep -Fq "$marker"; then
    echo "Epic #$epic_issue_number already has child index comment."
    return 0
  fi

  gh issue comment "$epic_issue_number" \
    --repo "$REPO" \
    --body "$comment_body"
}

require_command gh

if ! gh auth status >/dev/null 2>&1; then
  echo "GitHub CLI is not authenticated. Run: gh auth login" >&2
  exit 1
fi

echo "Seeding E17 issues in repo: $REPO"
echo

E17_SCOPE_IN="$(cat <<'EOF'
- Define the first MVP-safe `monad upgrade` contract.
- Model Monad repo/manifest versions and upgrade targets.
- Add upgrade dry-run planning.
- Add upgrade step registry foundation.
- Add guarded non-destructive upgrade writes.
- Add upgrade evidence reports and smoke tests.
EOF
)"

E17_SCOPE_OUT="$(cat <<'EOF'
- Destructive migrations.
- Automatic source-code rewrites without explicit approval.
- Remote/cloud upgrades.
- Package publishing.
- Dependency version upgrades unless explicitly modeled.
- Arbitrary third-party migration execution.
- Autonomous agent-driven repo rewrites.
EOF
)"

E17_WORK_PACKETS="$(cat <<'EOF'
- WP-E17-001 — Define `monad upgrade` contract and safety model
- WP-E17-002 — Add repository version and upgrade target model
- WP-E17-003 — Add upgrade dry-run plan output
- WP-E17-004 — Add upgrade step registry foundation
- WP-E17-005 — Add guarded non-destructive upgrade writes
- WP-E17-006 — Add upgrade evidence reports and smoke tests
EOF
)"

E17_DOD="$(cat <<'EOF'
- `monad upgrade --dry-run` can explain whether a repository needs an upgrade.
- Monad can model current repo contract/schema version and desired target version.
- Upgrade steps are deterministic, ordered, and reviewable.
- Guarded upgrade writes refuse unsafe overwrites.
- Upgrade evidence is generated and test-covered.
- Root verification passes.
EOF
)"

E17_USER_VALUE="$(cat <<'EOF'
Once Monad-managed repositories exist in the world, they need to evolve safely as Monad itself improves. E17 gives Monad a disciplined upgrade path: inspect the current repo contract, compare it to the current expected contract, preview upgrade steps, and apply only safe, non-destructive changes.

This is essential for turning Monad from a one-time scaffold tool into a durable repository evolution system.
EOF
)"

E17_NUMBER="$(create_epic \
  "E17" \
  "Upgrade and Repository Evolution Foundation" \
  "Upgrade / Repository Evolution / Contract Migration" \
  "Implement the first MVP-safe `monad upgrade` foundation so Monad can plan and apply controlled, non-destructive upgrades to existing Monad-managed repositories." \
  "$E17_USER_VALUE" \
  "$E17_SCOPE_IN" \
  "$E17_SCOPE_OUT" \
  "$E17_WORK_PACKETS" \
  "$E17_DOD"
)"

echo "E17 issue: #$E17_NUMBER"

WP_E17_001="$(create_work_packet \
  "E17" \
  "$E17_NUMBER" \
  "WP-E17-001" \
  "Define \`monad upgrade\` contract and safety model" \
  "Upgrade / Product Design / Safety" \
  "Define what `monad upgrade` may inspect, plan, and modify, and establish the first MVP-safe upgrade boundary." \
  "- Define `monad upgrade` command shape.
- Define `--dry-run` behavior.
- Define allowed upgrade targets.
- Define non-destructive write rules.
- Define what requires explicit future approval.
- Document examples." \
  "- Implementing upgrade execution.
- Destructive migrations.
- Automatic dependency updates.
- Arbitrary code rewrites." \
  "- \`docs/commands/UPGRADE.md\`
- \`docs/architecture/UPGRADE-SAFETY-MODEL.md\`
- ADR if needed" \
  "Monad has a documented upgrade contract before implementation begins." \
  "docs(upgrade): define upgrade safety model" \
  "Medium"
)"

WP_E17_002="$(create_work_packet \
  "E17" \
  "$E17_NUMBER" \
  "WP-E17-002" \
  "Add repository version and upgrade target model" \
  "Core Runtime / Upgrade Model" \
  "Add the core model for representing current repository contract version, supported target version, and upgrade eligibility." \
  "- Read current Monad schema/contract version.
- Represent supported target versions.
- Detect up-to-date, upgrade-needed, unsupported, and unknown states.
- Add deterministic status rendering.
- Add tests." \
  "- Applying upgrade steps.
- Editing files.
- Remote version lookup.
- Package registry integration." \
  "- \`crates/monad-core/src/upgrade.rs\`
- \`crates/monad-core/src/upgrade/\`
- tests" \
  "Monad can determine whether a repo is current, upgradeable, unsupported, or unknown." \
  "feat(upgrade): add repository version model" \
  "Medium"
)"

WP_E17_003="$(create_work_packet \
  "E17" \
  "$E17_NUMBER" \
  "WP-E17-003" \
  "Add upgrade dry-run plan output" \
  "CLI / Upgrade Planning" \
  "Add `monad upgrade --dry-run` so users can preview repository upgrade steps without writing files." \
  "- Parse `monad upgrade --dry-run`.
- Resolve current and target repo versions.
- Render planned upgrade steps.
- Report blockers and no-op state.
- Add CLI smoke tests." \
  "- Writing files.
- Running package managers.
- Creating commits automatically.
- Applying code rewrites." \
  "- \`crates/monad-cli/src/main.rs\`
- \`crates/monad-core/src/upgrade/\`
- CLI smoke tests" \
  "`monad upgrade --dry-run` renders a deterministic upgrade plan and writes nothing." \
  "feat(upgrade): add dry-run upgrade plan" \
  "Medium"
)"

WP_E17_004="$(create_work_packet \
  "E17" \
  "$E17_NUMBER" \
  "WP-E17-004" \
  "Add upgrade step registry foundation" \
  "Core Runtime / Upgrade Steps" \
  "Add a deterministic internal registry of upgrade steps so future repo contract upgrades can be expressed as ordered, testable operations." \
  "- Define upgrade step trait/model.
- Register initial no-op or metadata upgrade steps.
- Sort/resolve steps deterministically.
- Add step applicability checks.
- Add tests." \
  "- External migration plugins.
- Dynamic script execution.
- Destructive transformations.
- Complex source-code migrations." \
  "- \`crates/monad-core/src/upgrade/\`
- tests" \
  "Monad has an internal upgrade step registry that can plan ordered upgrade operations." \
  "feat(upgrade): add upgrade step registry" \
  "Large"
)"

WP_E17_005="$(create_work_packet \
  "E17" \
  "$E17_NUMBER" \
  "WP-E17-005" \
  "Add guarded non-destructive upgrade writes" \
  "Upgrade / File Operations / Safety" \
  "Add guarded write behavior for approved upgrade steps, refusing unsafe overwrites and preserving user-owned files." \
  "- Apply only approved upgrade file operations.
- Refuse unsafe overwrites.
- Report conflicts clearly.
- Write generated metadata/context upgrades only where safe.
- Add tests for safe, conflict, and no-op cases." \
  "- Destructive file replacement.
- Source-code rewriting.
- Dependency upgrades.
- Package manager execution.
- Git commit automation." \
  "- \`crates/monad-core/src/upgrade/\`
- \`crates/monad-core/src/file_ops/\`
- tests" \
  "`monad upgrade` can apply narrowly approved, non-destructive upgrade operations safely." \
  "feat(upgrade): add guarded upgrade writes" \
  "Large"
)"

WP_E17_006="$(create_work_packet \
  "E17" \
  "$E17_NUMBER" \
  "WP-E17-006" \
  "Add upgrade evidence reports and smoke tests" \
  "Verification / Upgrade Evidence" \
  "Add reviewable upgrade evidence reports and smoke tests for upgrade dry-run and guarded write behavior." \
  "- Add upgrade evidence report structure.
- Add text output.
- Add JSON output if feasible.
- Add CLI smoke tests.
- Document examples.
- Verify no-op and upgrade-needed scenarios." \
  "- Hosted reports.
- Telemetry.
- Automatic PR creation.
- Remote upgrade service." \
  "- \`crates/monad-core/src/upgrade/\`
- \`crates/monad-cli/src/main.rs\`
- CLI smoke tests
- \`docs/commands/UPGRADE.md\`" \
  "`monad upgrade` behavior is smoke-tested and produces reviewable upgrade evidence." \
  "test(upgrade): add upgrade evidence smoke tests" \
  "Medium"
)"

E17_COMMENT="$(cat <<EOF
## Child Work Packets

- #${WP_E17_001} — WP-E17-001 — Define \`monad upgrade\` contract and safety model
- #${WP_E17_002} — WP-E17-002 — Add repository version and upgrade target model
- #${WP_E17_003} — WP-E17-003 — Add upgrade dry-run plan output
- #${WP_E17_004} — WP-E17-004 — Add upgrade step registry foundation
- #${WP_E17_005} — WP-E17-005 — Add guarded non-destructive upgrade writes
- #${WP_E17_006} — WP-E17-006 — Add upgrade evidence reports and smoke tests
EOF
)"

add_epic_child_index_comment_once "$E17_NUMBER" "WP-E17-001" "$E17_COMMENT"

echo
echo "Created/confirmed E17 roadmap issues:"
echo "E17 #$E17_NUMBER"
echo "  WP-E17-001 #$WP_E17_001"
echo "  WP-E17-002 #$WP_E17_002"
echo "  WP-E17-003 #$WP_E17_003"
echo "  WP-E17-004 #$WP_E17_004"
echo "  WP-E17-005 #$WP_E17_005"
echo "  WP-E17-006 #$WP_E17_006"
