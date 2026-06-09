#!/usr/bin/env bash
set -euo pipefail

# repair_e21_work_packet_cli_insertion_point.sh
#
# Focused repair for E21 script failure:
#   Could not find import insertion point: build_ai_context_plan, build_local_agent_plan, build_patch_plan, build_policy_report,
#
# The original E21 script used a brittle exact import-string match in
# crates/monad-cli/src/main.rs. This repair keeps the already-generated
# monad-core work_packet module and finishes E21 wiring using resilient
# insertion logic.

SCRIPT_NAME="repair_e21_work_packet_cli_insertion_point"
BACKUP_ROOT=".monad/script-backups/${SCRIPT_NAME}-$(date -u +%Y%m%dT%H%M%SZ)"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "ERROR: required file not found: $path" >&2
    exit 1
  fi
}

backup_path() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "${BACKUP_ROOT}/$(dirname "$path")"
    cp -a "$path" "${BACKUP_ROOT}/$path"
  fi
}

if [[ ! -f Cargo.toml ]]; then
  echo "ERROR: run this script from the repository root." >&2
  exit 1
fi

require_file crates/monad-core/src/lib.rs
require_file crates/monad-core/src/work_packet.rs
require_file crates/monad-cli/src/main.rs

mkdir -p "$BACKUP_ROOT"
backup_path crates/monad-core/src/lib.rs
backup_path crates/monad-cli/src/main.rs
backup_path docs/work-packets/README.md
backup_path docs/roadmap/epic-21-work-packet-execution-workflow.md
backup_path tools/scripts/verify-work-packet.sh
backup_path tools/scripts/verify-e21.sh

mkdir -p docs/work-packets docs/roadmap tools/scripts

python3 - <<'PY'
from pathlib import Path


def insert_once(text: str, marker: str, addition: str, *, before: bool = True, label: str = "marker") -> str:
    if addition.strip() in text:
        return text
    if marker not in text:
        raise SystemExit(f"Could not find {label}: {marker!r}")
    if before:
        return text.replace(marker, addition + marker, 1)
    return text.replace(marker, marker + addition, 1)


# Ensure monad-core exports remain present even if the original script stopped mid-flight.
lib_path = Path("crates/monad-core/src/lib.rs")
lib = lib_path.read_text()

if "pub mod work_packet;" not in lib:
    lib = insert_once(
        lib,
        "pub mod upgrade;\n",
        "pub mod work_packet;\n",
        before=False,
        label="pub mod work_packet insertion point in monad-core lib.rs",
    )

work_packet_pub_use = '''pub use work_packet::{
    WORK_PACKET_HANDOFF_PATH, WORK_PACKET_PLAN_JSON_PATH, WORK_PACKET_PLAN_MARKDOWN_PATH,
    WorkPacketApplyResult, WorkPacketEvidenceItem, WorkPacketExecutionPlan,
    WorkPacketImplementationStep, WorkPacketMetadata, WorkPacketStatus,
    apply_work_packet_execution_plan, build_work_packet_execution_plan,
    parse_work_packet_metadata, render_work_packet_apply_result,
    render_work_packet_apply_result_json, render_work_packet_execution_plan,
    render_work_packet_execution_plan_json,
};
'''

if "pub use work_packet::{" not in lib:
    lib = insert_once(
        lib,
        "pub use workspace::{WorkspaceContext, discover_workspace_root, is_workspace_root};\n",
        work_packet_pub_use,
        before=True,
        label="work_packet pub use insertion point in monad-core lib.rs",
    )

lib_path.write_text(lib)

# Patch monad-cli resiliently. Do not depend on an exact import ordering.
main_path = Path("crates/monad-cli/src/main.rs")
main = main_path.read_text()

# Add missing monad_core imports directly inside the existing grouped import.
import_start = main.find("use monad_core::{")
if import_start == -1:
    raise SystemExit("Could not find monad_core grouped import in crates/monad-cli/src/main.rs")
import_end = main.find("};", import_start)
if import_end == -1:
    raise SystemExit("Could not find end of monad_core grouped import in crates/monad-cli/src/main.rs")

needed_imports = [
    "apply_work_packet_execution_plan",
    "build_work_packet_execution_plan",
    "render_work_packet_apply_result",
    "render_work_packet_apply_result_json",
    "render_work_packet_execution_plan",
    "render_work_packet_execution_plan_json",
]

import_block = main[import_start:import_end]
missing = [name for name in needed_imports if name not in import_block]
if missing:
    insertion = "\n    " + ", ".join(missing) + ","
    main = main[:import_end] + insertion + main[import_end:]

work_packet_variant = '''
    /// Plan or write generated work-packet workflow evidence.
    WorkPacket {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write generated workflow evidence after explicit approval.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    },
'''
if "WorkPacket {" not in main:
    markers = [
        "\n    /// Plan or apply generated patch artifacts under E19 approval gates.\n",
        "\n    /// Produce a supervised plan from a user intent.\n",
    ]
    for marker in markers:
        if marker in main:
            main = main.replace(marker, work_packet_variant + marker, 1)
            break
    else:
        raise SystemExit("Could not find enum insertion point for WorkPacket command")

# Allow --yes for work-packet.
if 'parts.first().copied() != Some("work-packet")' not in main:
    marker = '&& parts.first().copied() != Some("patch")'
    if marker not in main:
        raise SystemExit("Could not find --yes allow-list insertion point for work-packet")
    main = main.replace(
        marker,
        marker + '\n            && parts.first().copied() != Some("work-packet")',
        1,
    )

main = main.replace(
    "--yes is only supported for init, add, sync, upgrade, ai-context, policy, and patch commands",
    "--yes is only supported for init, add, sync, upgrade, ai-context, policy, patch, and work-packet commands",
)
main = main.replace(
    "--yes is only supported for init, add, sync, upgrade, ai-context, and policy commands",
    "--yes is only supported for init, add, sync, upgrade, ai-context, policy, patch, and work-packet commands",
)

work_packet_parse = '''
            ["work-packet"] => {
                reject_write_for_non_context(write)?;
                require_work_packet_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::WorkPacket {
                    dry_run,
                    yes,
                    output_format,
                })
            }
            ["work-packet", other, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown work-packet argument: {other}"))
            }
'''
if '["work-packet"] =>' not in main:
    markers = [
        '            ["patch"] => {\n',
        '            ["sync"] => {\n',
    ]
    for marker in markers:
        if marker in main:
            main = main.replace(marker, work_packet_parse + marker, 1)
            break
    else:
        raise SystemExit("Could not find parse insertion point for work-packet command")

work_packet_run = '''
        CliCommand::WorkPacket {
            dry_run,
            yes,
            output_format,
        } => render_work_packet(dry_run, yes, output_format),
'''
if "=> render_work_packet(dry_run, yes, output_format)," not in main:
    markers = [
        "        CliCommand::Patch {\n",
        "        CliCommand::Sync {\n",
    ]
    for marker in markers:
        if marker in main:
            main = main.replace(marker, work_packet_run + marker, 1)
            break
    else:
        raise SystemExit("Could not find run insertion point for work-packet command")

require_work_packet = '''
/// Requires exactly one work-packet mode.
fn require_work_packet_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => Err(
            "work-packet currently requires either --dry-run to preview or --yes to write generated workflow evidence"
                .to_string(),
        ),
        (true, true) => Err("work-packet accepts either --dry-run or --yes, not both".to_string()),
    }
}

'''
if "fn require_work_packet_mode" not in main:
    markers = [
        "/// Requires exactly one patch mode.\n",
        "/// Requires exactly one sync mode for the guarded sync implementation.\n",
    ]
    for marker in markers:
        if marker in main:
            main = main.replace(marker, require_work_packet + marker, 1)
            break
    else:
        raise SystemExit("Could not find require_work_packet_mode insertion point")

render_work_packet = '''
/// Renders or writes generated work-packet workflow evidence.
fn render_work_packet(
    dry_run: bool,
    yes: bool,
    output_format: OutputFormat,
) -> Result<String, String> {
    let root = std::env::current_dir().map_err(|error| error.to_string())?;

    if dry_run {
        let plan = build_work_packet_execution_plan(&root);
        return match output_format {
            OutputFormat::Text => Ok(render_work_packet_execution_plan(&plan)),
            OutputFormat::Json => Ok(render_work_packet_execution_plan_json(&plan)),
        };
    }

    if yes {
        let result = apply_work_packet_execution_plan(&root)?;
        return match output_format {
            OutputFormat::Text => Ok(render_work_packet_apply_result(&result)),
            OutputFormat::Json => Ok(render_work_packet_apply_result_json(&result)),
        };
    }

    Err("work-packet currently requires either --dry-run to preview or --yes to write generated workflow evidence".to_string())
}

'''
if "fn render_work_packet(" not in main:
    markers = [
        "/// Renders or applies generated patch artifacts under E19 approval gates.\n",
        "/// Renders or applies repository sync output.\n",
    ]
    for marker in markers:
        if marker in main:
            main = main.replace(marker, render_work_packet + marker, 1)
            break
    else:
        raise SystemExit("Could not find render_work_packet insertion point")

# Help text is user-facing but should not make this repair brittle.
help_edits = [
    (
        '        "  patch --yes                             Apply generated patch artifacts after review",\n',
        '        "  patch --yes                             Apply generated patch artifacts after review",\n'
        '        "  work-packet --dry-run                   Preview work-packet workflow plan",\n'
        '        "  work-packet --dry-run --format=json     Preview work-packet workflow as JSON",\n'
        '        "  work-packet --yes                       Write generated workflow evidence",\n',
    ),
    (
        '        "  monad patch --yes",\n',
        '        "  monad patch --yes",\n'
        '        "  monad work-packet --dry-run",\n'
        '        "  monad work-packet --dry-run --format=json",\n'
        '        "  monad work-packet --yes",\n',
    ),
    (
        '        "  patch applies generated local evidence only; user-owned source mutation remains blocked.",\n',
        '        "  patch applies generated local evidence only; user-owned source mutation remains blocked.",\n'
        '        "  work-packet writes generated workflow evidence only and never closes issues automatically.",\n',
    ),
]
for before, after in help_edits:
    if after not in main and before in main:
        main = main.replace(before, after, 1)

work_packet_tests = r'''

    #[test]
    fn work_packet_dry_run_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "work-packet", "--dry-run"])
                .expect("work-packet dry-run should parse"),
            CliCommand::WorkPacket {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "work-packet", "--dry-run", "--format=json"])
                .expect("work-packet dry-run json should parse"),
            CliCommand::WorkPacket {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Json,
            }
        );
    }

    #[test]
    fn work_packet_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "work-packet", "--yes"])
                .expect("work-packet yes should parse"),
            CliCommand::WorkPacket {
                dry_run: false,
                yes: true,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn work_packet_requires_mode() {
        let error = parse_arguments(&["monad", "work-packet"])
            .expect_err("work-packet should require mode");

        assert!(error.contains("work-packet currently requires either --dry-run"));
        assert!(error.contains("--yes"));
    }

    #[test]
    fn work_packet_rejects_dry_run_and_yes_together() {
        let error = parse_arguments(&["monad", "work-packet", "--dry-run", "--yes"])
            .expect_err("work-packet should reject conflicting modes");

        assert!(error.contains("work-packet accepts either --dry-run or --yes"));
    }
'''
if "fn work_packet_dry_run_command_parses" not in main:
    idx = main.rfind("}\n")
    if idx == -1:
        raise SystemExit("Could not find end of crates/monad-cli/src/main.rs for test insertion")
    main = main[:idx] + work_packet_tests + main[idx:]

main_path.write_text(main)
PY

cat > docs/work-packets/README.md <<'MD'
# Work-packet execution workflow

Monad work-packet workflow support is a local-first foundation for planning, verifying, evidencing, and handing off roadmap work packets.

## Command surface

```bash
monad work-packet --dry-run
monad work-packet --dry-run --format=json
monad work-packet --yes
```

## What E21 does

E21 adds a deterministic work-packet execution model inside `monad-core`:

- work-packet lifecycle status labels;
- work-packet metadata records;
- simple Markdown metadata parsing;
- implementation-plan generation;
- verification checklist rendering;
- evidence checklist rendering;
- generated closeout and handoff records;
- smoke tests and verification scripts.

## Safety boundaries

The work-packet command does not execute implementation commands, mutate GitHub issues, close work packets remotely, contact hosted services, or rewrite user-owned source files. `monad work-packet --yes` writes generated Monad workflow evidence only, through E19 generated-write approval gates.

## Generated evidence

`monad work-packet --yes` writes:

- `.monad/reports/work-packet-plan.md`
- `.monad/reports/work-packet-plan.json`
- `.monad/work-packets/e21-closeout-handoff.md`

Existing files with different content are not silently overwritten.
MD

cat > docs/roadmap/epic-21-work-packet-execution-workflow.md <<'MD'
# E21 — Work Packet Execution Workflow Foundation

## Product area

Work Packet Execution Workflow Foundation

## Objective

Create the first local, deterministic workflow foundation for executing Monad roadmap work packets with implementation planning, verification checklists, evidence records, closeout notes, and handoff artifacts.

## Implemented work packets

- WP-E21-001 — Define work-packet execution model.
- WP-E21-002 — Add work-packet metadata parser.
- WP-E21-003 — Add work-packet implementation plan generator.
- WP-E21-004 — Add verification and evidence checklist automation.
- WP-E21-005 — Add closeout and handoff record generation.
- WP-E21-006 — Add work-packet workflow smoke tests.

## Command surface

```bash
monad work-packet --dry-run
monad work-packet --dry-run --format=json
monad work-packet --yes
```

## Safety posture

E21 remains local-first and supervised:

- no autonomous work-packet execution;
- no GitHub issue mutation or closeout automation;
- no arbitrary command execution;
- no user-owned source rewrites;
- no remote service calls;
- generated writes require explicit `--yes` and E19 approval gates.

## Evidence outputs

`monad work-packet --yes` writes generated local artifacts only:

- `.monad/reports/work-packet-plan.md`
- `.monad/reports/work-packet-plan.json`
- `.monad/work-packets/e21-closeout-handoff.md`

## Verification

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-work-packet.sh
tools/scripts/verify-e21.sh
```
MD

cat > tools/scripts/verify-work-packet.sh <<'SH_VERIFY_WORK_PACKET'
#!/usr/bin/env bash
set -euo pipefail

echo "[verify-work-packet] cargo test -p monad-core --lib work_packet"
cargo test -p monad-core --lib work_packet

echo "[verify-work-packet] monad work-packet --dry-run"
cargo run -p monad-cli -- work-packet --dry-run >/tmp/monad-work-packet-dry-run.txt
grep -q "Monad work-packet execution workflow plan" /tmp/monad-work-packet-dry-run.txt
grep -q "Verification checklist" /tmp/monad-work-packet-dry-run.txt
grep -q "No GitHub issues were modified" /tmp/monad-work-packet-dry-run.txt

echo "[verify-work-packet] monad work-packet --dry-run --format=json"
cargo run -p monad-cli -- work-packet --dry-run --format=json >/tmp/monad-work-packet-dry-run.json
grep -q '"command":"work-packet"' /tmp/monad-work-packet-dry-run.json
grep -q '"mode":"dry-run"' /tmp/monad-work-packet-dry-run.json
grep -q '"work_packet_count":6' /tmp/monad-work-packet-dry-run.json

echo "[verify-work-packet] monad work-packet --yes"
cargo run -p monad-cli -- work-packet --yes >/tmp/monad-work-packet-apply.txt
grep -q "Monad work-packet workflow evidence write result" /tmp/monad-work-packet-apply.txt

test -f .monad/reports/work-packet-plan.md
test -f .monad/reports/work-packet-plan.json
test -f .monad/work-packets/e21-closeout-handoff.md

grep -q "Monad work-packet execution workflow plan" .monad/reports/work-packet-plan.md
grep -q '"command":"work-packet"' .monad/reports/work-packet-plan.json
grep -q "E21 Work-Packet Workflow Closeout Handoff" .monad/work-packets/e21-closeout-handoff.md

echo "[verify-work-packet] ok"
SH_VERIFY_WORK_PACKET

cat > tools/scripts/verify-e21.sh <<'SH_VERIFY_E21'
#!/usr/bin/env bash
set -euo pipefail

echo "[verify-e21] cargo fmt --check"
cargo fmt --check

echo "[verify-e21] cargo test"
cargo test

echo "[verify-e21] cargo clippy --all-targets --all-features -- -D warnings"
cargo clippy --all-targets --all-features -- -D warnings

echo "[verify-e21] tools/scripts/verify-work-packet.sh"
tools/scripts/verify-work-packet.sh

echo "[verify-e21] ok"
SH_VERIFY_E21

chmod +x tools/scripts/verify-work-packet.sh tools/scripts/verify-e21.sh

cargo fmt

cat <<NEXT
E21 CLI insertion-point repair applied.
Backups, if any, are under: ${BACKUP_ROOT}

Suggested verification:
  cargo check -p monad-core
  cargo test -p monad-core --lib work_packet
  cargo test -p monad-cli
  cargo clippy --all-targets --all-features -- -D warnings
  tools/scripts/verify-work-packet.sh
  tools/scripts/verify-e21.sh
NEXT
