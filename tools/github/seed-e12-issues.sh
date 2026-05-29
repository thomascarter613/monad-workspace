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
        $2 == wanted {
          gsub(/^#/, "", $1)
          print $1
          exit
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

echo "Seeding E12 issues in repo: $REPO"
echo

E12_SCOPE_IN="$(cat <<'EOF'
- Define the `monad add` command family.
- Define supported component types for the first MVP path.
- Add dry-run component scaffold plans.
- Add embedded component scaffold templates.
- Add guarded write behavior for component scaffolds.
- Add first polyglot-minimal component presets.
- Add tests proving non-destructive behavior.
EOF
)"

E12_SCOPE_OUT="$(cat <<'EOF'
- Destructive overwrites.
- Arbitrary third-party template execution.
- Plugin marketplace.
- Full framework-specific generators for every ecosystem.
- Package publishing.
- Hosted/SaaS orchestration.
- Autonomous agent execution.
- Apply/write evolution outside explicitly approved scaffold operations.
EOF
)"

E12_WORK_PACKETS="$(cat <<'EOF'
- WP-E12-001 — Define `monad add` UX and component taxonomy
- WP-E12-002 — Add component scaffold plan model
- WP-E12-003 — Add app/package/service/library scaffold templates
- WP-E12-004 — Add `monad add --dry-run` command path
- WP-E12-005 — Add guarded component scaffold write path
- WP-E12-006 — Add polyglot-minimal component presets and smoke tests
EOF
)"

E12_DOD="$(cat <<'EOF'
- `monad add --dry-run` can preview creation of supported component types.
- Component scaffold plans are deterministic and explain file operations before writing.
- Guarded writes refuse unsafe overwrites.
- First MVP-safe component templates exist for apps, packages, services, and libraries.
- Polyglot-minimal component presets are documented and tested.
- Root verification passes.
EOF
)"

E12_USER_VALUE="$(cat <<'EOF'
After `monad init` creates a repository skeleton, users need a safe way to grow the monorepo without hand-building the same structure repeatedly. E12 gives Monad the beginning of a real monorepo evolution workflow: add a component, preview the files, verify safety, then write only approved scaffold files.

This turns Monad from a repo initializer into the early version of a monorepo runtime and evolver.
EOF
)"

E12_NUMBER="$(create_epic \
  "E12" \
  "Component Add and Polyglot Scaffold Foundation" \
  "Scaffolding / Monorepo Evolution / Polyglot Foundation" \
  "Implement the first safe `monad add` foundation so Monad can plan and scaffold new monorepo components after initialization without overwriting existing work." \
  "$E12_USER_VALUE" \
  "$E12_SCOPE_IN" \
  "$E12_SCOPE_OUT" \
  "$E12_WORK_PACKETS" \
  "$E12_DOD"
)"

echo "E12 issue: #$E12_NUMBER"

WP_E12_001="$(create_work_packet \
  "E12" \
  "$E12_NUMBER" \
  "WP-E12-001" \
  "Define \`monad add\` UX and component taxonomy" \
  "Product Design / CLI UX" \
  "Define the command UX, component taxonomy, supported first-MVP component types, safety rules, and generated-file expectations for `monad add`." \
  "- Define `monad add` command forms.
- Define first supported component types: app, package, service, library, tool.
- Define component naming rules.
- Define output paths.
- Define dry-run and write-safety expectations.
- Document what is not yet supported." \
  "- Implementing file writes.
- Adding full framework-specific generators.
- Adding external template plugins.
- Supporting every language/framework combination." \
  "- \`docs/commands/ADD.md\`
- \`docs/product/COMPONENT-TAXONOMY.md\`
- \`docs/product/ADD-SAFETY-CONTRACT.md\` if needed" \
  "`monad add` has a documented UX and component taxonomy before implementation begins." \
  "docs(add): define component scaffold contract" \
  "Medium"
)"

WP_E12_002="$(create_work_packet \
  "E12" \
  "$E12_NUMBER" \
  "WP-E12-002" \
  "Add component scaffold plan model" \
  "Core Runtime / Scaffolding" \
  "Add the core data model for representing a component scaffold plan before files are written." \
  "- Add component kind enum.
- Add component name/path model.
- Add file operation plan model if not already reusable.
- Add scaffold plan renderer.
- Add tests for deterministic ordering and validation." \
  "- Writing files.
- Running external tools.
- Executing package managers.
- Full template expansion engine if not needed yet." \
  "- \`crates/monad-core/src/add.rs\`
- \`crates/monad-core/src/add/\`
- \`crates/monad-core/src/file_ops/\` if reused
- tests" \
  "Monad can construct and render a deterministic scaffold plan for a supported component." \
  "feat(add): add component scaffold plan model" \
  "Medium"
)"

WP_E12_003="$(create_work_packet \
  "E12" \
  "$E12_NUMBER" \
  "WP-E12-003" \
  "Add app/package/service/library scaffold templates" \
  "Templates / Scaffolding" \
  "Add minimal embedded scaffold templates for first-MVP component types." \
  "- Add minimal app scaffold template.
- Add minimal package scaffold template.
- Add minimal service scaffold template.
- Add minimal library scaffold template.
- Keep templates framework-light and language-agnostic where possible.
- Add tests proving expected template paths and contents." \
  "- Full React/FastAPI/Go/Rust/etc. framework generation.
- External template downloads.
- Package installation.
- Dependency manager execution." \
  "- \`crates/monad-core/src/templates/\`
- \`crates/monad-core/src/add/\`
- tests" \
  "Monad has minimal embedded component templates suitable for dry-run plans and guarded writes." \
  "feat(add): add minimal component scaffold templates" \
  "Large"
)"

WP_E12_004="$(create_work_packet \
  "E12" \
  "$E12_NUMBER" \
  "WP-E12-004" \
  "Add \`monad add --dry-run\` command path" \
  "CLI / Scaffolding" \
  "Add the CLI command path for previewing component scaffold plans without writing files." \
  "- Parse `monad add`.
- Parse component kind and component name.
- Require or default dry-run behavior for the first implementation.
- Render planned file operations.
- Add CLI smoke tests." \
  "- Writing scaffold files.
- Overwriting existing files.
- Adding package manager execution.
- Running formatters or installers." \
  "- \`crates/monad-cli/src/main.rs\`
- \`crates/monad-core/src/add.rs\`
- CLI smoke tests" \
  "`monad add <kind> <name> --dry-run` renders a scaffold plan and writes nothing." \
  "feat(add): add dry-run command path" \
  "Medium"
)"

WP_E12_005="$(create_work_packet \
  "E12" \
  "$E12_NUMBER" \
  "WP-E12-005" \
  "Add guarded component scaffold write path" \
  "File Operations / Safety" \
  "Add the guarded write implementation for approved component scaffold files, refusing unsafe overwrites." \
  "- Write only files from a validated scaffold plan.
- Refuse to overwrite existing files.
- Report conflicts clearly.
- Preserve deterministic output.
- Add tests for empty target, existing target, and partial conflict cases." \
  "- Destructive overwrite flags.
- Arbitrary patch application.
- Git branch/worktree automation.
- Package manager execution." \
  "- \`crates/monad-core/src/add/\`
- \`crates/monad-core/src/file_ops/\`
- \`crates/monad-cli/src/main.rs\`
- tests" \
  "`monad add` can create approved component scaffold files safely and refuses conflicts." \
  "feat(add): add guarded component scaffold writes" \
  "Large"
)"

WP_E12_006="$(create_work_packet \
  "E12" \
  "$E12_NUMBER" \
  "WP-E12-006" \
  "Add polyglot-minimal component presets and smoke tests" \
  "Presets / Verification" \
  "Add and verify first MVP-safe polyglot-minimal component presets for controlled monorepo growth." \
  "- Define a polyglot-minimal preset boundary.
- Add smoke tests for app/package/service/library scaffolds.
- Verify generated layout remains small and reviewable.
- Document usage examples." \
  "- Full enterprise monorepo generation.
- Framework-specific dependency installation.
- Publishing package artifacts.
- Hosted/SaaS orchestration." \
  "- \`docs/commands/ADD.md\`
- \`docs/product/POLYGLOT-MINIMAL-PRESET.md\`
- CLI smoke tests
- core tests" \
  "Polyglot-minimal component scaffolding is documented, smoke-tested, and ready for first-MVP evaluation." \
  "test(add): add polyglot scaffold smoke tests" \
  "Medium"
)"

E12_COMMENT="$(cat <<EOF
## Child Work Packets

- #${WP_E12_001} — WP-E12-001 — Define \`monad add\` UX and component taxonomy
- #${WP_E12_002} — WP-E12-002 — Add component scaffold plan model
- #${WP_E12_003} — WP-E12-003 — Add app/package/service/library scaffold templates
- #${WP_E12_004} — WP-E12-004 — Add \`monad add --dry-run\` command path
- #${WP_E12_005} — WP-E12-005 — Add guarded component scaffold write path
- #${WP_E12_006} — WP-E12-006 — Add polyglot-minimal component presets and smoke tests
EOF
)"

add_epic_child_index_comment "$E12_NUMBER" "$E12_COMMENT"

echo
echo "Created/confirmed E12 roadmap issues:"
echo "E12 #$E12_NUMBER"
echo "  WP-E12-001 #$WP_E12_001"
echo "  WP-E12-002 #$WP_E12_002"
echo "  WP-E12-003 #$WP_E12_003"
echo "  WP-E12-004 #$WP_E12_004"
echo "  WP-E12-005 #$WP_E12_005"
echo "  WP-E12-006 #$WP_E12_006"
