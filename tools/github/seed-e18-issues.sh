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

echo "Seeding E18 issues in repo: $REPO"
echo

E18_SCOPE_IN="$(cat <<'EOF'
- Define provider-agnostic AI workflow and memory boundaries.
- Define repo-native AI memory records and context snapshots.
- Add model/provider configuration without requiring a paid subscription.
- Extend supervised planning around work packets and context packs.
- Add assistant handoff/export artifacts.
- Add AI context verification and smoke tests.
EOF
)"

E18_SCOPE_OUT="$(cat <<'EOF'
- Requiring a paid LLM subscription.
- Hard-coding one hosted AI provider.
- Sending repository data to remote AI providers by default.
- Autonomous agent execution.
- Automatic patch application.
- Long-running agent daemon behavior.
- MCP marketplace or plugin marketplace.
- Hosted/SaaS AI service.
EOF
)"

E18_WORK_PACKETS="$(cat <<'EOF'
- WP-E18-001 — Define provider-agnostic AI workflow and memory contract
- WP-E18-002 — Add AI provider configuration model
- WP-E18-003 — Add repo-native memory record schema
- WP-E18-004 — Add context snapshot and work-packet planning artifacts
- WP-E18-005 — Add supervised assistant handoff/export workflow
- WP-E18-006 — Add AI context verification and smoke tests
EOF
)"

E18_DOD="$(cat <<'EOF'
- AI usage is optional and provider-agnostic.
- Monad does not require a paid AI subscription.
- Repo-native memory/context artifacts are documented and test-covered.
- Context snapshots and work-packet planning artifacts can be generated deterministically.
- Assistant handoff/export artifacts are usable without sending data anywhere by default.
- AI context verification is included in root verification or a clear verification path.
EOF
)"

E18_USER_VALUE="$(cat <<'EOF'
Monad should make AI-assisted development safer, more durable, and less dependent on fragile chat history. E18 gives users a provider-agnostic way to preserve project memory, export context, prepare work-packet plans, and hand work to an assistant without locking the project to one model vendor or paid subscription.

This turns Monad’s context bridge into the foundation for a practical AI-native SDLC workflow.
EOF
)"

E18_NUMBER="$(create_epic \
  "E18" \
  "AI Context Memory and Provider-Agnostic Assistant Workflow Foundation" \
  "AI Context / Memory / Assistant Workflow" \
  "Implement the first MVP-safe AI context and memory foundation so Monad can support provider-agnostic, repo-native, supervised assistant workflows without requiring autonomous execution or a paid AI subscription." \
  "$E18_USER_VALUE" \
  "$E18_SCOPE_IN" \
  "$E18_SCOPE_OUT" \
  "$E18_WORK_PACKETS" \
  "$E18_DOD"
)"

echo "E18 issue: #$E18_NUMBER"

WP_E18_001="$(create_work_packet \
  "E18" \
  "$E18_NUMBER" \
  "WP-E18-001" \
  "Define provider-agnostic AI workflow and memory contract" \
  "AI Context / Product Design / Safety" \
  "Define Monad’s AI workflow boundary, memory model, provider-agnostic rules, and non-autonomous safety contract." \
  "- Define AI workflow principles.
- Define repo-native memory boundaries.
- Define local-first and provider-agnostic requirements.
- Define what Monad may export.
- Define what Monad must not send remotely by default.
- Define explicit non-goals around autonomous agents." \
  "- Implementing provider calls.
- Autonomous execution.
- Patch application.
- Hosted AI service.
- MCP/plugin marketplace." \
  "- \`docs/ai/AI-WORKFLOW-CONTRACT.md\`
- \`docs/ai/MEMORY-CONTRACT.md\`
- \`docs/architecture/AI-CONTEXT-MODEL.md\`
- ADR if needed" \
  "Monad has a clear AI workflow and memory safety contract before implementation begins." \
  "docs(ai): define provider agnostic workflow contract" \
  "Medium"
)"

WP_E18_002="$(create_work_packet \
  "E18" \
  "$E18_NUMBER" \
  "WP-E18-002" \
  "Add AI provider configuration model" \
  "Core Runtime / AI Configuration" \
  "Add a provider-agnostic AI configuration model that supports hosted, self-hosted, local, and disabled AI modes without requiring any paid subscription." \
  "- Add provider config data model.
- Support disabled/default no-provider mode.
- Support named provider records.
- Support local/self-hosted provider metadata.
- Avoid storing secrets directly in committed config.
- Add validation and tests." \
  "- Calling providers.
- Managing API keys.
- Requiring a specific model vendor.
- Sending repo data remotely.
- Building an MCP server." \
  "- \`crates/monad-core/src/ai.rs\`
- \`crates/monad-core/src/ai/\`
- \`docs/ai/PROVIDER-CONFIGURATION.md\`
- tests" \
  "Monad can represent provider-agnostic AI configuration safely without requiring provider calls." \
  "feat(ai): add provider configuration model" \
  "Medium"
)"

WP_E18_003="$(create_work_packet \
  "E18" \
  "$E18_NUMBER" \
  "WP-E18-003" \
  "Add repo-native memory record schema" \
  "AI Memory / Context Bridge" \
  "Add the repo-native schema and storage conventions for durable project memory records." \
  "- Define memory record types.
- Define frontmatter fields.
- Define storage paths.
- Define provenance and freshness metadata.
- Define conflict/drift handling expectations.
- Add schema validation tests." \
  "- Vector database integration.
- Semantic search.
- Automatic memory summarization by model.
- Remote memory storage." \
  "- \`docs/ai/memory/\`
- \`docs/ai/MEMORY-SCHEMA.md\`
- \`crates/monad-core/src/ai_context/\`
- tests" \
  "Monad has a durable, repo-native memory record schema suitable for future retrieval and handoff workflows." \
  "feat(ai-context): add memory record schema" \
  "Large"
)"

WP_E18_004="$(create_work_packet \
  "E18" \
  "$E18_NUMBER" \
  "WP-E18-004" \
  "Add context snapshot and work-packet planning artifacts" \
  "AI Context / Work Planning" \
  "Add deterministic artifacts that package current repo state, active epic/work packet, decisions, and next steps for supervised assistant planning." \
  "- Define context snapshot artifact.
- Define work-packet planning artifact.
- Include active epic/work packet metadata.
- Include verification expectations.
- Include implementation boundaries.
- Add generation and tests." \
  "- Autonomous planning execution.
- Applying generated changes.
- Remote model calls.
- Hidden mutation." \
  "- \`crates/monad-core/src/context/\`
- \`crates/monad-core/src/ai_context/\`
- \`.monad/context/\`
- \`docs/ai/\`
- tests" \
  "Monad can generate deterministic AI-readable context snapshots and work-packet planning artifacts." \
  "feat(ai-context): add planning artifacts" \
  "Large"
)"

WP_E18_005="$(create_work_packet \
  "E18" \
  "$E18_NUMBER" \
  "WP-E18-005" \
  "Add supervised assistant handoff/export workflow" \
  "Assistant Workflow / Context Export" \
  "Add a supervised export workflow that prepares context for an assistant while preserving local-first and no-remote-by-default behavior." \
  "- Define assistant handoff export command or artifact.
- Include repo summary, current state, active work, constraints, and verification commands.
- Add redaction/safety notes if needed.
- Add docs for using exported context with any assistant/provider.
- Add tests." \
  "- Sending context to providers automatically.
- Requiring ChatGPT/OpenAI/Anthropic/etc.
- Autonomous code changes.
- Tool/plugin execution." \
  "- \`docs/ai/ASSISTANT-HANDOFF.md\`
- \`crates/monad-core/src/ai_context/\`
- \`crates/monad-cli/src/main.rs\` if command path is added
- tests" \
  "Monad can prepare assistant-ready context exports without contacting any AI provider." \
  "feat(ai-context): add assistant handoff export" \
  "Medium"
)"

WP_E18_006="$(create_work_packet \
  "E18" \
  "$E18_NUMBER" \
  "WP-E18-006" \
  "Add AI context verification and smoke tests" \
  "Verification / AI Context Evidence" \
  "Add verification and smoke tests for AI context, memory schema, provider config, and handoff artifacts." \
  "- Add schema validation tests.
- Add provider config validation tests.
- Add context snapshot smoke tests.
- Add handoff export smoke tests.
- Add verification evidence docs if needed." \
  "- Provider integration tests.
- Remote model calls.
- Autonomous execution tests.
- Vector database tests." \
  "- \`crates/monad-core/src/ai_context/\`
- \`crates/monad-core/src/ai/\`
- CLI smoke tests if command path exists
- \`tools/scripts/verify.sh\` if updated" \
  "AI context and memory foundations are test-covered and safe for first-MVP evaluation." \
  "test(ai-context): add memory and handoff smoke tests" \
  "Medium"
)"

E18_COMMENT="$(cat <<EOF
## Child Work Packets

- #${WP_E18_001} — WP-E18-001 — Define provider-agnostic AI workflow and memory contract
- #${WP_E18_002} — WP-E18-002 — Add AI provider configuration model
- #${WP_E18_003} — WP-E18-003 — Add repo-native memory record schema
- #${WP_E18_004} — WP-E18-004 — Add context snapshot and work-packet planning artifacts
- #${WP_E18_005} — WP-E18-005 — Add supervised assistant handoff/export workflow
- #${WP_E18_006} — WP-E18-006 — Add AI context verification and smoke tests
EOF
)"

add_epic_child_index_comment_once "$E18_NUMBER" "WP-E18-001" "$E18_COMMENT"

echo
echo "Created/confirmed E18 roadmap issues:"
echo "E18 #$E18_NUMBER"
echo "  WP-E18-001 #$WP_E18_001"
echo "  WP-E18-002 #$WP_E18_002"
echo "  WP-E18-003 #$WP_E18_003"
echo "  WP-E18-004 #$WP_E18_004"
echo "  WP-E18-005 #$WP_E18_005"
echo "  WP-E18-006 #$WP_E18_006"
