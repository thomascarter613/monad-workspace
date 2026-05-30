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

echo "Seeding E14 issues in repo: $REPO"
echo

E14_SCOPE_IN="$(cat <<'EOF'
- Define the first MVP-safe repository intent and sync contract.
- Compare `monad.toml` intent against discovered repository state.
- Produce dry-run sync plans.
- Add non-destructive sync writes for approved metadata/context files.
- Add native manifest reconciliation checks.
- Produce sync evidence reports and smoke tests.
EOF
)"

E14_SCOPE_OUT="$(cat <<'EOF'
- Destructive overwrites.
- Automatic package-manager rewrites.
- Automatic dependency installation.
- Full lockfile management.
- Full package publishing.
- Cross-repo synchronization.
- Cloud synchronization.
- Autonomous agent-driven changes.
EOF
)"

E14_WORK_PACKETS="$(cat <<'EOF'
- WP-E14-001 — Define `monad sync` contract and repo intent model
- WP-E14-002 — Add repository contract diff model
- WP-E14-003 — Add `monad sync --dry-run` plan output
- WP-E14-004 — Add non-destructive manifest/context sync writes
- WP-E14-005 — Add native manifest reconciliation checks
- WP-E14-006 — Add sync evidence reports and smoke tests
EOF
)"

E14_DOD="$(cat <<'EOF'
- `monad sync --dry-run` compares declared intent with discovered repo state.
- Sync plans are deterministic and human-reviewable.
- Monad can identify drift between `monad.toml`, component directories, and supported native manifests.
- Non-destructive sync writes are limited to approved metadata/context outputs.
- Sync never silently overwrites user-owned source files.
- Sync evidence is test-covered and root verification passes.
EOF
)"

E14_USER_VALUE="$(cat <<'EOF'
After Monad can initialize a repo, add components, and run tasks, users need confidence that the repository still matches its declared intent. E14 gives Monad a controlled way to detect and explain drift before it becomes chaos.

This is the foundation for safe long-term monorepo evolution: intent, actual state, diff, plan, evidence, then controlled non-destructive sync.
EOF
)"

E14_NUMBER="$(create_epic \
  "E14" \
  "Manifest Sync and Repository Contract Foundation" \
  "Repository Contract / Sync / Drift Control" \
  "Implement the first MVP-safe `monad sync` foundation so Monad can compare declared repository intent against actual repository state and produce safe, reviewable synchronization plans." \
  "$E14_USER_VALUE" \
  "$E14_SCOPE_IN" \
  "$E14_SCOPE_OUT" \
  "$E14_WORK_PACKETS" \
  "$E14_DOD"
)"

echo "E14 issue: #$E14_NUMBER"

WP_E14_001="$(create_work_packet \
  "E14" \
  "$E14_NUMBER" \
  "WP-E14-001" \
  "Define \`monad sync\` contract and repo intent model" \
  "Architecture / Repository Contract" \
  "Define the `monad sync` command contract and the repository intent model that sync will compare against discovered state." \
  "- Define what `monad sync` means.
- Define supported first-MVP sync sources.
- Define what Monad may update automatically.
- Define what requires future approval.
- Define no-overwrite and non-destructive rules.
- Document command examples." \
  "- Implementing sync execution.
- Editing native manifests automatically.
- Dependency installation.
- Lockfile generation." \
  "- \`docs/commands/SYNC.md\`
- \`docs/architecture/REPOSITORY-CONTRACT.md\`
- ADR if needed" \
  "The sync contract is documented and bounded before implementation begins." \
  "docs(sync): define repository sync contract" \
  "Medium"
)"

WP_E14_002="$(create_work_packet \
  "E14" \
  "$E14_NUMBER" \
  "WP-E14-002" \
  "Add repository contract diff model" \
  "Core Runtime / Sync" \
  "Add the core model for comparing declared repository intent with discovered repository state." \
  "- Add declared intent model.
- Add discovered state summary model.
- Add contract diff model.
- Add severity categories such as match, missing, extra, stale, unsupported.
- Add deterministic ordering and tests." \
  "- Writing files.
- Editing native manifests.
- Running package managers.
- Full schema migration engine." \
  "- \`crates/monad-core/src/sync.rs\`
- \`crates/monad-core/src/sync/\`
- tests" \
  "Monad can compute a deterministic repository contract diff." \
  "feat(sync): add repository contract diff model" \
  "Large"
)"

WP_E14_003="$(create_work_packet \
  "E14" \
  "$E14_NUMBER" \
  "WP-E14-003" \
  "Add \`monad sync --dry-run\` plan output" \
  "CLI / Sync Planning" \
  "Add the dry-run CLI path for rendering repository sync plans without writing files." \
  "- Parse `monad sync --dry-run`.
- Compute repository contract diff.
- Render planned actions.
- Identify no-op, warning, and blocked states.
- Add CLI smoke tests." \
  "- Writing files.
- Editing user source files.
- Dependency installation.
- Package manager execution." \
  "- \`crates/monad-cli/src/main.rs\`
- \`crates/monad-core/src/sync/\`
- CLI smoke tests" \
  "`monad sync --dry-run` renders a safe, deterministic synchronization plan and writes nothing." \
  "feat(sync): add dry-run sync plan" \
  "Medium"
)"

WP_E14_004="$(create_work_packet \
  "E14" \
  "$E14_NUMBER" \
  "WP-E14-004" \
  "Add non-destructive manifest/context sync writes" \
  "Sync / File Operations" \
  "Add guarded write behavior for approved metadata and context sync outputs." \
  "- Write approved generated sync outputs only.
- Update generated context or reports if needed.
- Refuse to overwrite user-owned files.
- Clearly report conflicts and skipped writes.
- Add tests for safe and unsafe paths." \
  "- Editing package.json/Cargo.toml automatically.
- Overwriting user source code.
- Applying broad patches.
- Lockfile management." \
  "- \`crates/monad-core/src/sync/\`
- \`crates/monad-core/src/file_ops/\`
- \`.monad/reports/\`
- tests" \
  "Monad can perform narrowly approved sync writes without destructive behavior." \
  "feat(sync): add guarded metadata sync writes" \
  "Large"
)"

WP_E14_005="$(create_work_packet \
  "E14" \
  "$E14_NUMBER" \
  "WP-E14-005" \
  "Add native manifest reconciliation checks" \
  "Sync / Native Tool Coordination" \
  "Add checks that compare Monad intent against supported native manifests without automatically rewriting them." \
  "- Check package.json scripts/workspace fields where supported.
- Check Cargo workspace membership where supported.
- Check expected component paths.
- Report mismatches as evidence.
- Add fixture tests." \
  "- Automatic native manifest rewriting.
- Dependency install/update.
- Lockfile generation.
- Support for every ecosystem." \
  "- \`crates/monad-core/src/sync/\`
- \`crates/monad-core/src/adapters/\`
- tests" \
  "Monad can detect and report drift between Monad intent and supported native manifests." \
  "feat(sync): add native manifest reconciliation checks" \
  "Large"
)"

WP_E14_006="$(create_work_packet \
  "E14" \
  "$E14_NUMBER" \
  "WP-E14-006" \
  "Add sync evidence reports and smoke tests" \
  "Verification / Evidence" \
  "Add reviewable sync evidence reports and smoke tests covering dry-run and guarded sync behavior." \
  "- Add sync evidence report structure.
- Add text output.
- Add JSON output if feasible.
- Add CLI smoke tests.
- Document examples." \
  "- Hosted reports.
- Dashboard UI.
- Remote sync.
- Full analytics." \
  "- \`crates/monad-core/src/sync/\`
- CLI smoke tests
- \`docs/commands/SYNC.md\`
- \`.monad/reports/\` if generated" \
  "`monad sync` behavior is smoke-tested and produces reviewable sync evidence." \
  "test(sync): add sync evidence smoke tests" \
  "Medium"
)"

E14_COMMENT="$(cat <<EOF
## Child Work Packets

- #${WP_E14_001} — WP-E14-001 — Define \`monad sync\` contract and repo intent model
- #${WP_E14_002} — WP-E14-002 — Add repository contract diff model
- #${WP_E14_003} — WP-E14-003 — Add \`monad sync --dry-run\` plan output
- #${WP_E14_004} — WP-E14-004 — Add non-destructive manifest/context sync writes
- #${WP_E14_005} — WP-E14-005 — Add native manifest reconciliation checks
- #${WP_E14_006} — WP-E14-006 — Add sync evidence reports and smoke tests
EOF
)"

add_epic_child_index_comment "$E14_NUMBER" "$E14_COMMENT"

echo
echo "Created/confirmed E14 roadmap issues:"
echo "E14 #$E14_NUMBER"
echo "  WP-E14-001 #$WP_E14_001"
echo "  WP-E14-002 #$WP_E14_002"
echo "  WP-E14-003 #$WP_E14_003"
echo "  WP-E14-004 #$WP_E14_004"
echo "  WP-E14-005 #$WP_E14_005"
echo "  WP-E14-006 #$WP_E14_006"
