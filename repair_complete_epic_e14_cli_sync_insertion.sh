#!/usr/bin/env bash
set -euo pipefail

# Repair / completion script for Epic E14.
#
# Fixes:
#   ERROR: could not find CliCommand Sync insertion point
#
# Cause:
#   The first E14 epic script used an exact text marker for the CLI enum layout.
#   Your local `crates/monad-cli/src/main.rs` differs from that exact marker.
#
# This script:
#   - assumes the first E14 script already wrote `crates/monad-core/src/sync.rs`;
#   - robustly patches `lib.rs` and `main.rs` using broader structural insertion points;
#   - writes the E14 docs and verification scripts that the first script did not reach;
#   - runs `cargo fmt`.

echo "==> Repair/completion: Epic E14 CLI sync insertion"

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

cd "$REPO_ROOT"

echo "==> Repo root: $REPO_ROOT"

SYNC_FILE="crates/monad-core/src/sync.rs"
LIB_FILE="crates/monad-core/src/lib.rs"
CLI_FILE="crates/monad-cli/src/main.rs"

for required in "$SYNC_FILE" "$LIB_FILE" "$CLI_FILE"; do
  if [ ! -f "$required" ]; then
    echo "ERROR: expected file not found: $required" >&2
    echo "This repair assumes the E14 script already created sync.rs before failing." >&2
    exit 1
  fi
done

mkdir -p \
  docs/commands \
  docs/architecture \
  docs/workflows \
  docs/verification \
  tools/scripts \
  work/learning/E14 \
  work/deliverables/E14 \
  .monad/script-backups/E14/REPAIR-cli-sync-insertion

BACKUP_STAMP="$(date +%Y%m%d%H%M%S)"
cp "$LIB_FILE" ".monad/script-backups/E14/REPAIR-cli-sync-insertion/lib.rs.$BACKUP_STAMP.bak"
cp "$CLI_FILE" ".monad/script-backups/E14/REPAIR-cli-sync-insertion/main.rs.$BACKUP_STAMP.bak"

python3 <<'PY'
from pathlib import Path
import re

LIB = Path("crates/monad-core/src/lib.rs")
CLI = Path("crates/monad-cli/src/main.rs")


def insert_before_first(text: str, markers: list[str], insertion: str, label: str) -> str:
    if insertion.strip() in text:
        return text

    for marker in markers:
        index = text.find(marker)
        if index != -1:
            return text[:index] + insertion + text[index:]

    raise SystemExit(f"ERROR: could not find insertion point for {label}")


def add_imports_to_monad_core_use(text: str, names: list[str]) -> str:
    match = re.search(r"use monad_core::\{(?P<body>.*?)\};", text, re.DOTALL)
    if not match:
        raise SystemExit("ERROR: could not find monad_core import block in CLI")

    body = match.group("body")
    missing = [name for name in names if name not in body]

    if not missing:
        return text

    addition = "\n    " + ", ".join(missing) + ","
    new_body = body + addition
    return text[:match.start("body")] + new_body + text[match.end("body"):]


# ---------------------------------------------------------------------------
# Patch monad-core lib.rs.
# ---------------------------------------------------------------------------
lib = LIB.read_text()

if "pub mod sync;" not in lib:
    lib = insert_before_first(
        lib,
        ["pub mod templates;", "pub mod toolchain_detection;", "pub mod workspace;"],
        "pub mod sync;\n",
        "sync module declaration",
    )

if "pub use sync::" not in lib:
    sync_export = '''pub use sync::{
    SyncApplyResult, SyncFinding, SyncFindingKind, SyncFindingSeverity, SyncPlan, apply_sync_plan,
    build_sync_plan, render_sync_apply_result, render_sync_evidence_markdown, render_sync_plan,
    render_sync_plan_json,
};
'''
    lib = insert_before_first(
        lib,
        ["pub use templates::{", "pub use toolchain_detection::{", "pub use workspace::{"],
        sync_export,
        "sync public exports",
    )

LIB.write_text(lib)


# ---------------------------------------------------------------------------
# Patch monad-cli main.rs.
# ---------------------------------------------------------------------------
cli = CLI.read_text()

cli = add_imports_to_monad_core_use(
    cli,
    [
        "apply_sync_plan",
        "build_sync_plan",
        "render_sync_apply_result",
        "render_sync_plan",
        "render_sync_plan_json",
    ],
)

# Add CliCommand::Sync variant.
if "Sync {" not in cli:
    sync_variant = '''    /// Synchronize declared repository intent with discovered state.
    Sync {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write approved generated sync evidence.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    }

'''
    cli = insert_before_first(
        cli,
        [
            "    /// Inspect repository structure.\n",
            "    Inspect {\n",
            "    /// Render repository graph.\n",
            "    Graph {\n",
            "    Info {\n",
        ],
        sync_variant,
        "CliCommand::Sync variant",
    )

# Allow --yes for sync.
if 'Some("sync")' not in cli[cli.find("if yes") : cli.find("match parts.as_slice()", cli.find("if yes")) if cli.find("if yes") != -1 else 0]:
    old_exact = '''        if yes && parts.first().copied() != Some("init") && parts.first().copied() != Some("add") {
            return Err("--yes is only supported for init and add commands".to_string());
        }
'''
    new_block = '''        if yes
            && parts.first().copied() != Some("init")
            && parts.first().copied() != Some("add")
            && parts.first().copied() != Some("sync")
        {
            return Err("--yes is only supported for init, add, and sync commands".to_string());
        }
'''
    if old_exact in cli:
        cli = cli.replace(old_exact, new_block, 1)
    else:
        cli = re.sub(
            r'''        if\s+yes\s*&&\s*parts\.first\(\)\.copied\(\)\s*!=\s*Some\("init"\)\s*&&\s*parts\.first\(\)\.copied\(\)\s*!=\s*Some\("add"\)\s*\{\s*
            return\s+Err\("--yes is only supported for init and add commands"\.to_string\(\)\);\s*
        \}\s*''',
            new_block,
            cli,
            count=1,
            flags=re.VERBOSE,
        )

cli = cli.replace(
    "--yes is only supported for init and add commands",
    "--yes is only supported for init, add, and sync commands",
)

# Add sync parse arm.
if '["sync"] => {' not in cli:
    sync_parse_arm = '''            ["sync"] => {
                reject_write_for_non_context(write)?;
                require_sync_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Sync {
                    dry_run,
                    yes,
                    output_format,
                })
            }
            ["sync", other, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown sync argument: {other}"))
            }
'''
    cli = insert_before_first(
        cli,
        [
            '            ["check"] => {\n',
            '            ["inspect"] => {\n',
            '            ["graph"] => {\n',
            '            ["context"] => {\n',
        ],
        sync_parse_arm,
        "sync parse arm",
    )

# Add sync run arm.
if "CliCommand::Sync" not in cli.split("match command", 1)[-1].split("CliCommand::Context", 1)[0]:
    sync_run_arm = '''        CliCommand::Sync {
            dry_run,
            yes,
            output_format,
        } => render_sync(dry_run, yes, output_format),
'''
    cli = insert_before_first(
        cli,
        [
            "        CliCommand::Inspect { output_format } => render_inspect(output_format),\n",
            "        CliCommand::Graph { graph_format } => render_graph(graph_format),\n",
            "        CliCommand::Context {\n",
        ],
        sync_run_arm,
        "sync run arm",
    )

# Add require_sync_mode helper.
if "fn require_sync_mode" not in cli:
    require_sync_mode = '''/// Requires exactly one sync mode for the guarded sync implementation.
fn require_sync_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => {
            Err("sync currently requires either --dry-run to preview or --yes to write generated evidence".to_string())
        }
        (true, true) => Err("sync accepts either --dry-run or --yes, not both".to_string()),
    }
}

'''
    cli = insert_before_first(
        cli,
        [
            "/// Requires exactly one init mode for the guarded init implementation.\n",
            "fn require_init_mode",
            "/// Rejects output-format flags for the first init implementation.\n",
        ],
        require_sync_mode,
        "require_sync_mode helper",
    )

# Add render_sync helper.
if "fn render_sync(" not in cli:
    render_sync = '''/// Renders or applies repository sync output.
fn render_sync(dry_run: bool, yes: bool, output_format: OutputFormat) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;

    if dry_run {
        let plan = build_sync_plan(&context).map_err(|error| error.to_string())?;
        return match output_format {
            OutputFormat::Text => Ok(render_sync_plan(&plan)),
            OutputFormat::Json => Ok(render_sync_plan_json(&plan)),
        };
    }

    if yes {
        let result = apply_sync_plan(&context).map_err(|error| error.to_string())?;
        return Ok(render_sync_apply_result(&result));
    }

    Err("sync currently requires either --dry-run to preview or --yes to write generated evidence".to_string())
}

'''
    cli = insert_before_first(
        cli,
        [
            "/// Renders workspace checks.\n",
            "fn render_check",
            "/// Renders repository inspection.\n",
            "fn render_inspect",
        ],
        render_sync,
        "render_sync helper",
    )

# Help text additions.
if "sync --dry-run" not in cli:
    cli = cli.replace(
        '        "  check                                     Run workspace checks",\n',
        '        "  check                                     Run workspace checks",\n        "  sync --dry-run                            Preview repository sync plan",\n        "  sync --dry-run --format=json              Preview repository sync plan as JSON",\n        "  sync --yes                                Write generated sync evidence reports",\n',
        1,
    )
    cli = cli.replace(
        '        "  monad check --format=json",\n',
        '        "  monad check --format=json",\n        "  monad sync --dry-run",\n        "  monad sync --dry-run --format=json",\n',
        1,
    )
    cli = cli.replace(
        '        "  --write is only supported for the context command.",\n',
        '        "  sync writes generated evidence reports only.",\n        "  --write is only supported for the context command.",\n',
        1,
    )

# Add parser tests.
if "fn sync_dry_run_command_parses" not in cli:
    tests = '''    #[test]
    fn sync_dry_run_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "sync", "--dry-run"]).expect("sync dry-run should parse"),
            CliCommand::Sync {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn sync_dry_run_json_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "sync", "--dry-run", "--format=json"])
                .expect("sync dry-run json should parse"),
            CliCommand::Sync {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Json,
            }
        );
    }

    #[test]
    fn sync_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "sync", "--yes"]).expect("sync yes should parse"),
            CliCommand::Sync {
                dry_run: false,
                yes: true,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn sync_requires_dry_run_or_yes() {
        let error = parse_arguments(&["monad", "sync"]).expect_err("sync without mode should fail");

        assert!(error.contains("sync currently requires either --dry-run"));
        assert!(error.contains("--yes"));
    }

    #[test]
    fn sync_rejects_dry_run_and_yes_together() {
        let error = parse_arguments(&["monad", "sync", "--dry-run", "--yes"])
            .expect_err("sync should reject conflicting modes");

        assert!(error.contains("sync accepts either --dry-run or --yes"));
    }

'''
    cli = insert_before_first(
        cli,
        [
            "    #[test]\n    fn info_command_parses_text_and_json_formats()",
            "    #[test]\n    fn check_command_parses_text_and_json_formats()",
            "    #[test]\n    fn inspect_command_parses_text_and_json_formats()",
        ],
        tests,
        "sync parser tests",
    )

CLI.write_text(cli)
PY

echo "==> Writing E14 docs and verification scripts"

cat > docs/commands/SYNC.md <<'EOF'
---
title: monad sync
status: complete
epic: E14
---

# `monad sync`

`monad sync` compares Monad's declared repository intent with discovered repository state and produces a reviewable synchronization plan.

## Commands

```bash
monad sync --dry-run
monad sync --dry-run --format=json
monad sync --yes
```

## Safety contract

`monad sync` is MVP-safe and non-destructive.

It does not:

- rewrite native manifests such as `Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`, or `go.work`;
- install dependencies;
- generate lockfiles;
- run package managers;
- run language toolchains;
- overwrite user-owned source files;
- publish packages;
- synchronize with cloud services;
- perform autonomous agent-driven changes.

## Dry-run behavior

```bash
monad sync --dry-run
```

Dry-run:

- discovers repository state;
- checks core Monad paths such as `monad.toml` and `.monad/`;
- checks component family directories;
- discovers first-level components under `apps/`, `packages/`, `services/`, and `tools/`;
- discovers supported component-native manifests;
- reports mismatches and unsupported automatic changes;
- writes no files.

## JSON dry-run

```bash
monad sync --dry-run --format=json
```

JSON output is intended for future dashboards, automation, and AI-readable evidence.

## Guarded writes

```bash
monad sync --yes
```

The guarded write path writes generated evidence only:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

It does not rewrite source files or native manifests.
EOF

cat > docs/architecture/REPOSITORY-CONTRACT.md <<'EOF'
---
title: Repository Contract
status: complete
epic: E14
---

# Repository Contract

The repository contract is Monad's bounded understanding of what a healthy repository should look like.

E14 establishes the first MVP-safe version of that contract.

## Contract sources

The initial contract uses:

- `monad.toml` as declared Monad intent;
- `.monad/` as Monad operational state;
- component family directories:
  - `apps/`
  - `packages/`
  - `services/`
  - `tools/`
- supported native manifests discovered in components:
  - `Cargo.toml`
  - `package.json`
  - `pyproject.toml`
  - `go.mod`

## Finding severities

| Severity | Meaning |
| --- | --- |
| `match` | Expected state exists. |
| `missing` | Expected state is absent. |
| `extra` | State exists but is outside the first sync contract. |
| `stale` | State exists but appears incomplete or inconsistent. |
| `unsupported` | State is recognized but not automatically rewritten by sync. |

## Non-destructive rule

The contract may report drift, but sync does not silently fix user-owned files.

E14 only permits generated evidence writes under:

```text
.monad/reports/
```

Native manifest rewriting is intentionally deferred.
EOF

cat > docs/workflows/SYNC-WORKFLOW.md <<'EOF'
---
title: Sync Workflow
status: complete
epic: E14
---

# Sync Workflow

Use sync to understand repository drift before taking action.

## 1. Preview

```bash
monad sync --dry-run
```

## 2. Review

Read the findings:

- matches show expected state;
- missing items show expected state that was not found;
- stale items show incomplete or inconsistent state;
- unsupported items show recognized state that Monad will not rewrite automatically.

## 3. Write evidence

```bash
monad sync --yes
```

This writes generated reports only:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

## 4. Fix manually or with later approved commands

E14 does not rewrite native manifests. Future epics may add narrowly approved reconciliation commands.
EOF

cat > docs/verification/SYNC-SMOKE-TESTS.md <<'EOF'
---
title: Sync Smoke Tests
status: complete
epic: E14
---

# Sync Smoke Tests

Run:

```bash
tools/scripts/verify-sync.sh
```

This verifies:

- `monad sync --dry-run` writes no sync evidence;
- `monad sync --dry-run --format=json` renders JSON;
- `monad sync --yes` writes only generated sync evidence reports;
- missing command mode fails safely.

Full E14 verification:

```bash
tools/scripts/verify-e14.sh
```
EOF

cat > docs/verification/E14-CLOSEOUT.md <<'EOF'
---
title: E14 Closeout
status: complete
epic: E14
---

# E14 Closeout — Manifest Sync and Repository Contract Foundation

E14 is complete when:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-sync.sh
tools/scripts/verify-e14.sh
```

## Completed capability

`monad sync` now supports:

```bash
monad sync --dry-run
monad sync --dry-run --format=json
monad sync --yes
```

## Safety retained

Sync does not:

- rewrite native manifests;
- rewrite user source files;
- install dependencies;
- generate lockfiles;
- run package managers;
- publish packages;
- call cloud services.

## Generated evidence

Approved sync writes are limited to:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

## Next epic

E15 — Doctor and Environment Diagnostics Foundation.
EOF

cat > tools/scripts/verify-sync.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-sync: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

(
  cd "$tmpdir"

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- init --yes >/tmp/monad-e14-init.out

  echo "==> verify sync dry-run"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --dry-run >/tmp/monad-e14-sync-dry.out
  grep -q "Monad sync dry-run plan" /tmp/monad-e14-sync-dry.out
  grep -q "No files were written." /tmp/monad-e14-sync-dry.out
  test ! -e .monad/reports/sync-report.md
  test ! -e .monad/reports/sync-report.json

  echo "==> verify sync json dry-run"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --dry-run --format=json >/tmp/monad-e14-sync-json.out
  grep -q '"command":"sync"' /tmp/monad-e14-sync-json.out
  grep -q '"mode":"dry-run"' /tmp/monad-e14-sync-json.out
  grep -q '"writes_enabled":false' /tmp/monad-e14-sync-json.out

  echo "==> verify sync generated evidence writes"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --yes >/tmp/monad-e14-sync-yes.out
  grep -q "Monad sync evidence written" /tmp/monad-e14-sync-yes.out
  grep -q "No native manifests were rewritten." /tmp/monad-e14-sync-yes.out
  test -f .monad/reports/sync-report.md
  test -f .monad/reports/sync-report.json
  grep -q "Monad Sync Evidence Report" .monad/reports/sync-report.md
  grep -q "Native manifest rewrites: none" .monad/reports/sync-report.md
  grep -q '"command":"sync"' .monad/reports/sync-report.json

  echo "==> verify sync mode guard"
  if cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync >/tmp/monad-e14-sync-no-mode.out 2>&1; then
    echo "Expected sync without mode to fail" >&2
    exit 1
  fi
  grep -q "sync currently requires either --dry-run" /tmp/monad-e14-sync-no-mode.out

  echo "==> verify conflicting mode guard"
  if cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --dry-run --yes >/tmp/monad-e14-sync-conflict-mode.out 2>&1; then
    echo "Expected sync with conflicting modes to fail" >&2
    exit 1
  fi
  grep -q "sync accepts either --dry-run or --yes" /tmp/monad-e14-sync-conflict-mode.out
)

echo "verify-sync: PASS"
EOF
chmod +x tools/scripts/verify-sync.sh

cat > tools/scripts/verify-e14.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "==> E14 verification"

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-sync.sh

if [ -x tools/scripts/verify.sh ]; then
  tools/scripts/verify.sh
fi

echo "verify-e14: PASS"
EOF
chmod +x tools/scripts/verify-e14.sh

cat > work/learning/E14/EPIC-E14-manifest-sync-repository-contract.md <<'EOF'
---
title: Epic E14 Learning Note
epic: E14
---

# Epic E14 Learning Note: Manifest Sync and Repository Contract Foundation

E14 adds the first safe form of repository synchronization.

The important concept is that sync does not mean "rewrite everything."

In Monad, sync means:

1. discover declared repository intent;
2. discover actual repository state;
3. compare the two;
4. produce deterministic findings;
5. write generated evidence only when approved.

## Why it is safe

`monad sync --dry-run` writes nothing.

`monad sync --yes` writes only:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

It does not rewrite native manifests or source files.

## What to inspect

```bash
git diff -- crates/monad-core/src/sync.rs
git diff -- crates/monad-core/src/lib.rs
git diff -- crates/monad-cli/src/main.rs
git diff -- docs/commands/SYNC.md
git diff -- docs/architecture/REPOSITORY-CONTRACT.md
```

## Why this prepares E15

E15 is doctor diagnostics. Doctor can reuse E14's contract concepts to report whether the repo is healthy and ready.
EOF

cat > work/deliverables/E14/EPIC-E14-manifest-sync-repository-contract.md <<'EOF'
---
title: Epic E14 Deliverable Record
epic: E14
status: complete
---

# Epic E14 Deliverable Record

## Epic

E14 — Manifest Sync and Repository Contract Foundation.

## Completed work packets

- WP-E14-001 — Define `monad sync` contract and repo intent model
- WP-E14-002 — Add repository contract diff model
- WP-E14-003 — Add `monad sync --dry-run` plan output
- WP-E14-004 — Add non-destructive manifest/context sync writes
- WP-E14-005 — Add native manifest reconciliation checks
- WP-E14-006 — Add sync evidence reports and smoke tests

## Implementation files

```text
crates/monad-core/src/sync.rs
crates/monad-core/src/lib.rs
crates/monad-cli/src/main.rs
```

## Documentation files

```text
docs/commands/SYNC.md
docs/architecture/REPOSITORY-CONTRACT.md
docs/workflows/SYNC-WORKFLOW.md
docs/verification/SYNC-SMOKE-TESTS.md
docs/verification/E14-CLOSEOUT.md
```

## Verification files

```text
tools/scripts/verify-sync.sh
tools/scripts/verify-e14.sh
```

## Verification command

```bash
tools/scripts/verify-e14.sh
```

## Next epic

E15 — Doctor and Environment Diagnostics Foundation.
EOF

echo "==> Formatting Rust code"
cargo fmt

echo
echo "==> E14 repair/completion patch complete."
echo
echo "Focused inspection:"
echo "  git diff -- crates/monad-core/src/lib.rs"
echo "  git diff -- crates/monad-cli/src/main.rs"
echo "  git diff -- docs/commands/SYNC.md"
echo "  git diff -- tools/scripts/verify-sync.sh"
echo
echo "Focused verification:"
echo "  cargo test -p monad-cli"
echo "  cargo test -p monad-core --lib sync"
echo
echo "Full verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-sync.sh"
echo "  tools/scripts/verify-e14.sh"
