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

echo "Seeding E19 issues in repo: $REPO"
echo

E19_SCOPE_IN="$(cat <<'EOF'
- Define Monad's first policy and approval-gate model.
- Classify operations by risk and mutability.
- Add approval-plan and approval-evidence records.
- Add policy checks for file operations, command execution, sync, upgrade, release, and AI context export.
- Add a gated write/apply foundation for future use.
- Add policy reports and smoke tests.
EOF
)"

E19_SCOPE_OUT="$(cat <<'EOF'
- Fully autonomous agent execution.
- Automatic approval of risky operations.
- Destructive file rewrites.
- Remote/cloud policy service.
- Enterprise policy engine.
- OPA/Rego integration unless explicitly approved later.
- Secrets scanning beyond simple MVP-safe checks.
- Security compliance certification.
EOF
)"

E19_WORK_PACKETS="$(cat <<'EOF'
- WP-E19-001 — Define policy and approval-gate contract
- WP-E19-002 — Add operation classification and risk model
- WP-E19-003 — Add approval plan and approval evidence model
- WP-E19-004 — Add policy checks for file operations and command execution
- WP-E19-005 — Add gated write/apply foundation
- WP-E19-006 — Add policy reports and smoke tests
EOF
)"

E19_DOD="$(cat <<'EOF'
- Monad has a documented policy and approval-gate model.
- Operations can be classified by risk, mutability, and required approval.
- Dry-run plans can state whether approval is required.
- File operations and command execution can be checked against policy.
- Gated write/apply foundation exists for future controlled evolution.
- Policy reports are generated and smoke-tested.
- Root verification passes.
EOF
)"

E19_USER_VALUE="$(cat <<'EOF'
Monad's core promise is safe repository evolution. Users need confidence that Monad will not silently overwrite files, execute risky commands, publish releases, send context to AI providers, or apply changes without an explicit review boundary.

E19 establishes the policy and approval foundation that will protect init, add, run, sync, doctor, release, upgrade, AI handoff, and future patch/apply workflows.
EOF
)"

E19_NUMBER="$(create_epic \
  "E19" \
  "Policy, Safety, and Approval Gate Foundation" \
  "Policy / Safety / Approval Gates / Controlled Evolution" \
  "Implement the first MVP-safe policy and approval-gate foundation so Monad can classify operations, require review for risky actions, and prevent unsafe repository mutation." \
  "$E19_USER_VALUE" \
  "$E19_SCOPE_IN" \
  "$E19_SCOPE_OUT" \
  "$E19_WORK_PACKETS" \
  "$E19_DOD"
)"

echo "E19 issue: #$E19_NUMBER"

WP_E19_001="$(create_work_packet \
  "E19" \
  "$E19_NUMBER" \
  "WP-E19-001" \
  "Define policy and approval-gate contract" \
  "Policy / Product Design / Safety" \
  "Define Monad's first policy model, approval-gate terminology, safety levels, and command behavior expectations." \
  "- Define policy principles.
- Define approval gate types.
- Define operation categories.
- Define what requires dry-run only.
- Define what requires explicit user approval.
- Define what is forbidden in MVP.
- Document examples across init, add, run, sync, upgrade, release, and AI handoff." \
  "- Implementing policy enforcement.
- Enterprise policy languages.
- Remote policy service.
- Autonomous approval." \
  "- \`docs/architecture/POLICY-MODEL.md\`
- \`docs/architecture/APPROVAL-GATES.md\`
- \`docs/security/SAFETY-BOUNDARIES.md\`
- ADR if needed" \
  "Monad has a clear policy and approval-gate contract before enforcement implementation begins." \
  "docs(policy): define approval gate model" \
  "Medium"
)"

WP_E19_002="$(create_work_packet \
  "E19" \
  "$E19_NUMBER" \
  "WP-E19-002" \
  "Add operation classification and risk model" \
  "Core Runtime / Policy" \
  "Add a core model for classifying operations by type, mutability, risk, and required approval level." \
  "- Add operation kind model.
- Add risk level model.
- Add mutability classification.
- Add required approval classification.
- Add deterministic rendering.
- Add unit tests." \
  "- Enforcing policy.
- Running commands.
- Writing files.
- External policy engines." \
  "- \`crates/monad-core/src/policy.rs\`
- \`crates/monad-core/src/policy/\`
- tests" \
  "Monad can classify planned operations consistently before execution or writing." \
  "feat(policy): add operation risk model" \
  "Medium"
)"

WP_E19_003="$(create_work_packet \
  "E19" \
  "$E19_NUMBER" \
  "WP-E19-003" \
  "Add approval plan and approval evidence model" \
  "Core Runtime / Approval Evidence" \
  "Add the data model for approval plans and approval evidence so dry-run output can explain what requires approval and why." \
  "- Add approval plan model.
- Add approval evidence model.
- Link approval requirements to planned operations.
- Add text rendering.
- Add JSON rendering if feasible.
- Add tests." \
  "- Interactive approval prompts.
- Remote approvals.
- GitHub PR approval integration.
- Persistent approval database." \
  "- \`crates/monad-core/src/policy/\`
- \`crates/monad-core/src/output/\`
- tests" \
  "Monad can produce reviewable approval plans and evidence records for planned operations." \
  "feat(policy): add approval evidence model" \
  "Medium"
)"

WP_E19_004="$(create_work_packet \
  "E19" \
  "$E19_NUMBER" \
  "WP-E19-004" \
  "Add policy checks for file operations and command execution" \
  "Policy Enforcement / File Operations / Execution" \
  "Add first policy checks for planned file operations and command execution so unsafe operations can be blocked or marked approval-required." \
  "- Check create/update/delete file operations.
- Check overwrite risk.
- Check command execution risk.
- Check known forbidden operations.
- Return pass, warn, approval-required, or blocked status.
- Add tests." \
  "- Executing commands.
- Writing files directly.
- Full sandboxing.
- Enterprise policy engine." \
  "- \`crates/monad-core/src/policy/\`
- \`crates/monad-core/src/file_ops/\`
- \`crates/monad-core/src/exec/\`
- tests" \
  "Monad can evaluate planned file and command operations against policy before execution." \
  "feat(policy): add operation policy checks" \
  "Large"
)"

WP_E19_005="$(create_work_packet \
  "E19" \
  "$E19_NUMBER" \
  "WP-E19-005" \
  "Add gated write/apply foundation" \
  "Controlled Evolution / Safety" \
  "Add the first shared gated write/apply foundation for future init, add, sync, upgrade, and patch workflows." \
  "- Define gated operation flow.
- Require plan before write.
- Require policy check before write.
- Require explicit caller approval marker.
- Preserve no-overwrite protections.
- Add tests for allowed, approval-required, and blocked operations." \
  "- Autonomous apply.
- Arbitrary patch application.
- Destructive rewrites.
- Interactive UI approval.
- Remote execution." \
  "- \`crates/monad-core/src/policy/\`
- \`crates/monad-core/src/file_ops/\`
- \`crates/monad-core/src/commands/\`
- tests" \
  "Monad has a reusable gated write/apply foundation for future controlled evolution commands." \
  "feat(policy): add gated write foundation" \
  "Large"
)"

WP_E19_006="$(create_work_packet \
  "E19" \
  "$E19_NUMBER" \
  "WP-E19-006" \
  "Add policy reports and smoke tests" \
  "Verification / Policy Evidence" \
  "Add policy report output and smoke tests proving policy classification and approval-gate behavior." \
  "- Add policy report renderer.
- Add approval-required examples.
- Add blocked-operation examples.
- Add CLI smoke tests where command paths exist.
- Document examples." \
  "- Hosted reports.
- Remote policy service.
- Enterprise dashboard.
- Telemetry." \
  "- \`crates/monad-core/src/policy/\`
- \`crates/monad-cli/src/main.rs\` if command path is added
- \`docs/architecture/APPROVAL-GATES.md\`
- tests" \
  "Policy behavior is test-covered and produces reviewable safety evidence." \
  "test(policy): add approval gate smoke tests" \
  "Medium"
)"

E19_COMMENT="$(cat <<EOF
## Child Work Packets

- #${WP_E19_001} — WP-E19-001 — Define policy and approval-gate contract
- #${WP_E19_002} — WP-E19-002 — Add operation classification and risk model
- #${WP_E19_003} — WP-E19-003 — Add approval plan and approval evidence model
- #${WP_E19_004} — WP-E19-004 — Add policy checks for file operations and command execution
- #${WP_E19_005} — WP-E19-005 — Add gated write/apply foundation
- #${WP_E19_006} — WP-E19-006 — Add policy reports and smoke tests
EOF
)"

add_epic_child_index_comment_once "$E19_NUMBER" "WP-E19-001" "$E19_COMMENT"

echo
echo "Created/confirmed E19 roadmap issues:"
echo "E19 #$E19_NUMBER"
echo "  WP-E19-001 #$WP_E19_001"
echo "  WP-E19-002 #$WP_E19_002"
echo "  WP-E19-003 #$WP_E19_003"
echo "  WP-E19-004 #$WP_E19_004"
echo "  WP-E19-005 #$WP_E19_005"
echo "  WP-E19-006 #$WP_E19_006"
