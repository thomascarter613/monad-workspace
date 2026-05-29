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

echo "Seeding E13 issues in repo: $REPO"
echo

E13_SCOPE_IN="$(cat <<'EOF'
- Define the first MVP-safe task model.
- Discover runnable tasks from `monad.toml` and common native manifests.
- Add dry-run task execution planning.
- Add guarded command execution for explicitly requested tasks.
- Add package/component filtering.
- Add graph-aware task ordering foundation.
- Add task evidence reports and smoke tests.
EOF
)"

E13_SCOPE_OUT="$(cat <<'EOF'
- Replacing native package managers.
- Replacing native build tools.
- Distributed execution.
- Remote runners.
- Cloud/SaaS orchestration.
- Long-running daemon execution.
- Arbitrary shell automation without policy boundaries.
- Full Nx/Turborepo replacement in one step.
EOF
)"

E13_WORK_PACKETS="$(cat <<'EOF'
- WP-E13-001 — Define task model and native-tool coordination contract
- WP-E13-002 — Add task discovery from manifests and `monad.toml`
- WP-E13-003 — Add `monad run --dry-run` plan output
- WP-E13-004 — Add guarded command execution runner
- WP-E13-005 — Add package/component filtering and graph-aware ordering
- WP-E13-006 — Add task evidence reports and smoke tests
EOF
)"

E13_DOD="$(cat <<'EOF'
- Monad has a documented first-MVP task model.
- Monad can discover simple runnable tasks from supported repository metadata.
- `monad run <task> --dry-run` previews execution without running commands.
- `monad run <task>` can execute approved local native commands with clear boundaries.
- Users can filter by package/component where supported.
- Task execution produces reviewable evidence.
- Root verification passes.
EOF
)"

E13_USER_VALUE="$(cat <<'EOF'
After Monad can initialize and grow a monorepo, users need a way to run common tasks through a consistent interface without learning every project’s internal layout immediately. E13 gives Monad the first bounded version of native-tool orchestration: discover tasks, explain what would run, execute only approved commands, and report evidence.

This keeps Monad aligned with its architecture: coordinate native tools rather than replacing them recklessly.
EOF
)"

E13_NUMBER="$(create_epic \
  "E13" \
  "Task Execution and Native Tool Orchestration Foundation" \
  "Task Execution / Native Tool Coordination / Monorepo Runtime" \
  "Implement the first MVP-safe task execution foundation so Monad can discover, plan, and run repository tasks by coordinating native tools through a consistent local CLI." \
  "$E13_USER_VALUE" \
  "$E13_SCOPE_IN" \
  "$E13_SCOPE_OUT" \
  "$E13_WORK_PACKETS" \
  "$E13_DOD"
)"

echo "E13 issue: #$E13_NUMBER"

WP_E13_001="$(create_work_packet \
  "E13" \
  "$E13_NUMBER" \
  "WP-E13-001" \
  "Define task model and native-tool coordination contract" \
  "Architecture / Task Execution" \
  "Define Monad’s first MVP-safe task model and clarify how Monad coordinates native tools without replacing them." \
  "- Define task identity, source, command, working directory, ecosystem, package/component scope, and safety metadata.
- Define what a task runner may and may not execute.
- Document native-tool coordination principles.
- Define supported first-MVP task sources." \
  "- Implementing execution.
- Distributed execution.
- Replacing package managers.
- Shell automation without boundaries." \
  "- \`docs/commands/RUN.md\`
- \`docs/architecture/TASK-EXECUTION-MODEL.md\`
- ADR if needed" \
  "Monad has a clear task execution contract before implementation begins." \
  "docs(run): define task execution model" \
  "Medium"
)"

WP_E13_002="$(create_work_packet \
  "E13" \
  "$E13_NUMBER" \
  "WP-E13-002" \
  "Add task discovery from manifests and \`monad.toml\`" \
  "Core Runtime / Task Discovery" \
  "Add task discovery from Monad configuration and common native ecosystem manifests." \
  "- Discover tasks from `monad.toml` if configured.
- Discover npm/package.json scripts where present.
- Discover Cargo commands for Rust packages where safe.
- Create a normalized task inventory model.
- Add deterministic ordering and tests." \
  "- Executing tasks.
- Discovering every ecosystem.
- Parsing complex workspace runners.
- Running package-manager install commands." \
  "- \`crates/monad-core/src/tasks.rs\`
- \`crates/monad-core/src/tasks/\`
- tests" \
  "Monad can produce a deterministic inventory of supported runnable tasks." \
  "feat(tasks): add task discovery foundation" \
  "Large"
)"

WP_E13_003="$(create_work_packet \
  "E13" \
  "$E13_NUMBER" \
  "WP-E13-003" \
  "Add \`monad run --dry-run\` plan output" \
  "CLI / Task Planning" \
  "Add a dry-run command path that shows what task would run without executing anything." \
  "- Parse `monad run <task> --dry-run`.
- Resolve task by name.
- Render command, working directory, ecosystem, and source.
- Fail clearly for unknown or ambiguous tasks.
- Add CLI smoke tests." \
  "- Executing commands.
- Running all tasks automatically.
- Parallel execution.
- Dependency graph scheduling." \
  "- \`crates/monad-cli/src/main.rs\`
- \`crates/monad-core/src/tasks/\`
- CLI smoke tests" \
  "`monad run <task> --dry-run` explains exactly what would run and writes/runs nothing." \
  "feat(run): add dry-run task plan" \
  "Medium"
)"

WP_E13_004="$(create_work_packet \
  "E13" \
  "$E13_NUMBER" \
  "WP-E13-004" \
  "Add guarded command execution runner" \
  "Execution / Safety" \
  "Add guarded local command execution for explicitly selected tasks." \
  "- Execute only resolved supported tasks.
- Use explicit working directory.
- Capture exit status.
- Capture stdout/stderr summary.
- Return clear success/failure evidence.
- Add tests around command runner behavior." \
  "- Arbitrary shell execution.
- Remote execution.
- Long-running daemon management.
- Parallel task orchestration.
- Hidden writes outside native command behavior." \
  "- \`crates/monad-core/src/exec/\`
- \`crates/monad-core/src/tasks/\`
- \`crates/monad-cli/src/main.rs\`
- tests" \
  "`monad run <task>` can execute a supported local task and report evidence clearly." \
  "feat(run): add guarded task execution" \
  "Large"
)"

WP_E13_005="$(create_work_packet \
  "E13" \
  "$E13_NUMBER" \
  "WP-E13-005" \
  "Add package/component filtering and graph-aware ordering" \
  "Monorepo Runtime / Graph" \
  "Add the first bounded package/component filtering and graph-aware ordering foundation for task plans." \
  "- Allow filtering tasks by package/component where metadata exists.
- Use repository graph data where available.
- Define deterministic ordering.
- Add dry-run output showing selected task order.
- Add tests for filtering and ordering." \
  "- Full dependency-aware scheduler.
- Remote cache.
- Parallel execution.
- Distributed execution.
- Nx/Turborepo replacement in one step." \
  "- \`crates/monad-core/src/tasks/\`
- \`crates/monad-core/src/graph/\`
- CLI tests" \
  "Monad can plan task execution for selected packages/components in deterministic order." \
  "feat(run): add task filtering and ordering" \
  "Large"
)"

WP_E13_006="$(create_work_packet \
  "E13" \
  "$E13_NUMBER" \
  "WP-E13-006" \
  "Add task evidence reports and smoke tests" \
  "Verification / Evidence" \
  "Add task execution evidence reports and smoke tests for the first MVP-safe `monad run` behavior." \
  "- Add evidence report structure for run results.
- Add text output.
- Add JSON output if feasible.
- Add CLI smoke tests.
- Document sample usage." \
  "- Hosted reports.
- Dashboard UI.
- Full analytics.
- Remote log storage." \
  "- \`crates/monad-core/src/tasks/\`
- \`crates/monad-core/src/checks/\` if reused
- CLI smoke tests
- \`docs/commands/RUN.md\`" \
  "`monad run` behavior is tested and produces reviewable task evidence." \
  "test(run): add task execution evidence tests" \
  "Medium"
)"

E13_COMMENT="$(cat <<EOF
## Child Work Packets

- #${WP_E13_001} — WP-E13-001 — Define task model and native-tool coordination contract
- #${WP_E13_002} — WP-E13-002 — Add task discovery from manifests and \`monad.toml\`
- #${WP_E13_003} — WP-E13-003 — Add \`monad run --dry-run\` plan output
- #${WP_E13_004} — WP-E13-004 — Add guarded command execution runner
- #${WP_E13_005} — WP-E13-005 — Add package/component filtering and graph-aware ordering
- #${WP_E13_006} — WP-E13-006 — Add task evidence reports and smoke tests
EOF
)"

add_epic_child_index_comment "$E13_NUMBER" "$E13_COMMENT"

echo
echo "Created/confirmed E13 roadmap issues:"
echo "E13 #$E13_NUMBER"
echo "  WP-E13-001 #$WP_E13_001"
echo "  WP-E13-002 #$WP_E13_002"
echo "  WP-E13-003 #$WP_E13_003"
echo "  WP-E13-004 #$WP_E13_004"
echo "  WP-E13-005 #$WP_E13_005"
echo "  WP-E13-006 #$WP_E13_006"
