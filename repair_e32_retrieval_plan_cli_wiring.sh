#!/usr/bin/env bash
set -euo pipefail

# Focused E32 repair:
# Adds missing CLI wiring for retrieval-plan/local-retrieval/vector-memory.

TARGET="crates/monad-cli/src/main.rs"

if [[ ! -f "Cargo.toml" || ! -f "$TARGET" ]]; then
  echo "Run this script from the monad-workspace repository root." >&2
  exit 1
fi

BACKUP_DIR=".monad/script-backups/repair-e32-retrieval-plan-cli-wiring-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$BACKUP_DIR/crates/monad-cli/src"
cp "$TARGET" "$BACKUP_DIR/$TARGET"

python3 - <<'PY'
from pathlib import Path

path = Path("crates/monad-cli/src/main.rs")
text = path.read_text()

variant = """    /// Plan local AI retrieval and vector memory without provider calls.
    RetrievalPlan {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write generated retrieval evidence.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    },

"""
if "RetrievalPlan {" not in text:
    anchors = [
        "    /// Plan MCP/context export and external tool policy without invoking tools.\n    McpPlan {\n",
        "    /// Plan plugin and extension loading without loading or executing plugins.\n    PluginPlan {\n",
        "    /// Index local templates and presets without rendering or applying them.\n    TemplateRegistry {\n",
        "    /// Index the local report/artifact store without uploading or deleting objects.\n    ReportStore {\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, variant + anchor, 1)
            break
    else:
        raise SystemExit("Could not find CliCommand enum insertion point for RetrievalPlan.")

if 'Some("retrieval-plan")' not in text:
    anchor = '            && parts.first().copied() != Some("sync")'
    if anchor in text:
        text = text.replace(
            anchor,
            anchor
            + '\n            && parts.first().copied() != Some("retrieval-plan")'
            + '\n            && parts.first().copied() != Some("local-retrieval")'
            + '\n            && parts.first().copied() != Some("vector-memory")',
            1,
        )

parse_arm = """            ["retrieval-plan"] | ["local-retrieval"] | ["vector-memory"] => {
                reject_write_for_non_context(write)?;
                require_retrieval_plan_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::RetrievalPlan {
                    dry_run,
                    yes,
                    output_format,
                })
            }
            ["retrieval-plan", other, ..]
            | ["local-retrieval", other, ..]
            | ["vector-memory", other, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown retrieval-plan argument: {other}"))
            }
"""
if '["retrieval-plan"] | ["local-retrieval"] | ["vector-memory"]' not in text:
    anchors = [
        '            ["mcp-plan"] | ["mcp"] | ["external-tools"] => {\n',
        '            ["plugin-plan"] | ["plugins"] | ["extensions"] => {\n',
        '            ["template-registry"] | ["templates"] | ["presets"] => {\n',
        '            ["report-store"] | ["reports"] | ["artifacts"] => {\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, parse_arm + anchor, 1)
            break
    else:
        raise SystemExit("Could not find command parse insertion point for retrieval-plan.")

run_arm = """        CliCommand::RetrievalPlan {
            dry_run,
            yes,
            output_format,
        } => render_retrieval_plan(dry_run, yes, output_format),
"""
if "render_retrieval_plan(dry_run, yes, output_format)" not in text:
    anchors = [
        "        CliCommand::McpPlan {\n",
        "        CliCommand::PluginPlan {\n",
        "        CliCommand::TemplateRegistry {\n",
        "        CliCommand::ReportStore {\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, run_arm + anchor, 1)
            break
    else:
        raise SystemExit("Could not find run match insertion point for retrieval-plan.")

helper = """/// Requires exactly one retrieval-plan mode.
fn require_retrieval_plan_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => Err(
            "retrieval-plan currently requires either --dry-run to preview or --yes to write generated retrieval evidence".to_string(),
        ),
        (true, true) => Err("retrieval-plan accepts either --dry-run or --yes, not both".to_string()),
    }
}

"""
if "fn require_retrieval_plan_mode" not in text:
    anchors = [
        "/// Requires exactly one MCP plan mode.\n",
        "/// Requires exactly one plugin-plan mode.\n",
        "/// Requires exactly one template-registry mode.\n",
        "/// Requires exactly one report-store mode.\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, helper + anchor, 1)
            break
    else:
        raise SystemExit("Could not find helper insertion point for retrieval-plan.")

render_fn = """/// Renders or writes local AI retrieval evidence.
fn render_retrieval_plan(
    dry_run: bool,
    yes: bool,
    output_format: OutputFormat,
) -> Result<String, String> {
    let root = std::env::current_dir().map_err(|error| error.to_string())?;

    if dry_run {
        let plan = monad_core::build_local_retrieval_plan(&root);
        return match output_format {
            OutputFormat::Text => Ok(monad_core::render_local_retrieval_plan(&plan)),
            OutputFormat::Json => Ok(monad_core::render_local_retrieval_plan_json(&plan)),
        };
    }

    if yes {
        let result =
            monad_core::write_local_retrieval_evidence(&root).map_err(|error| error.to_string())?;
        return Ok(monad_core::render_local_retrieval_apply_result(&result));
    }

    Err("retrieval-plan currently requires either --dry-run to preview or --yes to write generated retrieval evidence".to_string())
}

"""
if "fn render_retrieval_plan(" not in text:
    anchors = [
        "/// Renders or writes local MCP integration evidence.\n",
        "/// Renders or writes local plugin-system evidence.\n",
        "/// Renders or writes local template-registry evidence.\n",
        "/// Renders or writes local report/artifact store evidence.\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, render_fn + anchor, 1)
            break
    else:
        raise SystemExit("Could not find render function insertion point for retrieval-plan.")

if "  retrieval-plan --dry-run" not in text:
    anchors = [
        '        "  mcp-plan --yes                           Write generated MCP integration evidence",\n',
        '        "  plugin-plan --yes                        Write generated plugin-system evidence",\n',
        '        "  template-registry --yes                   Write generated template-registry evidence",\n',
        '        "  report-store --yes                        Write generated report-store evidence",\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(
                anchor,
                anchor
                + '        "  retrieval-plan --dry-run                 Preview local AI retrieval/vector-memory plan",\n'
                + '        "  retrieval-plan --dry-run --format=json   Preview retrieval plan as JSON",\n'
                + '        "  retrieval-plan --yes                     Write generated retrieval evidence",\n',
                1,
            )
            break

if "monad vector-memory --dry-run" not in text:
    anchors = [
        '        "  monad external-tools --dry-run",\n',
        '        "  monad plugins --dry-run",\n',
        '        "  monad templates --dry-run",\n',
        '        "  monad reports --dry-run",\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(
                anchor,
                anchor
                + '        "  monad retrieval-plan --dry-run",\n'
                + '        "  monad retrieval-plan --dry-run --format=json",\n'
                + '        "  monad local-retrieval --dry-run",\n'
                + '        "  monad vector-memory --dry-run",\n',
                1,
            )
            break

if "retrieval-plan writes generated evidence only" not in text:
    anchors = [
        '        "  mcp-plan writes generated evidence only and does not invoke external tools.",\n',
        '        "  plugin-plan writes generated evidence only and does not load plugins.",\n',
        '        "  template-registry writes generated evidence only and does not apply templates.",\n',
        '        "  report-store writes generated evidence only and does not delete artifacts.",\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(
                anchor,
                anchor
                + '        "  retrieval-plan writes generated evidence only and does not call model providers.",\n',
                1,
            )
            break

test_block = """    #[test]
    fn retrieval_plan_dry_run_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "retrieval-plan", "--dry-run"]),
            Ok(CliCommand::RetrievalPlan {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            })
        );

        assert_eq!(
            parse_arguments(&["monad", "retrieval-plan", "--dry-run", "--format=json"]),
            Ok(CliCommand::RetrievalPlan {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Json,
            })
        );
    }

    #[test]
    fn retrieval_plan_aliases_parse() {
        assert_eq!(
            parse_arguments(&["monad", "local-retrieval", "--dry-run"]),
            Ok(CliCommand::RetrievalPlan {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            })
        );

        assert_eq!(
            parse_arguments(&["monad", "vector-memory", "--dry-run"]),
            Ok(CliCommand::RetrievalPlan {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            })
        );
    }

    #[test]
    fn retrieval_plan_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "retrieval-plan", "--yes"]),
            Ok(CliCommand::RetrievalPlan {
                dry_run: false,
                yes: true,
                output_format: OutputFormat::Text,
            })
        );
    }

    #[test]
    fn retrieval_plan_requires_mode() {
        match parse_arguments(&["monad", "retrieval-plan"]) {
            Ok(_) => panic!("retrieval-plan should require --dry-run or --yes"),
            Err(error) => {
                assert!(error.contains("retrieval-plan currently requires either --dry-run"));
            }
        }
    }

"""
if "fn retrieval_plan_dry_run_command_parses" not in text:
    anchors = [
        "    #[test]\n    fn mcp_plan_dry_run_command_parses() {\n",
        "    #[test]\n    fn plugin_plan_dry_run_command_parses() {\n",
        "    #[test]\n    fn template_registry_dry_run_command_parses() {\n",
        "    #[test]\n    fn report_store_dry_run_command_parses() {\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, test_block + anchor, 1)
            break
    else:
        raise SystemExit("Could not find CLI test insertion point for retrieval-plan.")

path.write_text(text)
PY

cargo fmt

echo "Applied focused E32 CLI wiring repair."
echo "Backup written under: $BACKUP_DIR"
echo
echo "Run focused verification:"
echo "  cargo test -p monad-cli retrieval_plan"
echo "  cargo run -p monad-cli -- retrieval-plan --dry-run"
echo "  cargo run -p monad-cli -- retrieval-plan --dry-run --format=json"
echo
echo "Then run full verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-local-ai-retrieval.sh"
echo "  tools/scripts/verify-e32.sh"
