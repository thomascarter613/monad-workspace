#!/usr/bin/env bash
set -euo pipefail

# Repair — E14 sync workspace discovery fallback
#
# Symptom:
#   MONAD2002: required resource not found: Monad workspace root from .
#
# Cause:
#   `monad sync` was wired through `WorkspaceContext::discover_from(".")`.
#   That is correct for commands that require an initialized Monad workspace, but
#   it is too strict for sync planning. E14 sync is supposed to report missing
#   contract pieces such as `monad.toml` and `.monad/`, not fail before planning.
#
# Fix:
#   Add a sync-specific context helper:
#
#     discover_sync_context()
#
#   It first tries the normal initialized workspace discovery path. If that fails,
#   it falls back to treating the current directory as the repository root so the
#   sync planner can emit missing-contract findings.

echo "==> Repair: E14 sync uninitialized workspace fallback"

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

cd "$REPO_ROOT"

CLI_FILE="crates/monad-cli/src/main.rs"

if [ ! -f "$CLI_FILE" ]; then
  echo "ERROR: expected file not found: $CLI_FILE" >&2
  exit 1
fi

if ! grep -q "fn render_sync" "$CLI_FILE"; then
  echo "ERROR: render_sync was not found in $CLI_FILE." >&2
  echo "Run the E14 CLI repair/completion script first." >&2
  exit 1
fi

mkdir -p .monad/script-backups/E14/REPAIR-sync-workspace-fallback
BACKUP_STAMP="$(date +%Y%m%d%H%M%S)"
cp "$CLI_FILE" ".monad/script-backups/E14/REPAIR-sync-workspace-fallback/main.rs.$BACKUP_STAMP.bak"

python3 <<'PY'
from pathlib import Path

path = Path("crates/monad-cli/src/main.rs")
text = path.read_text()


def insert_before(text: str, marker: str, insertion: str, label: str) -> str:
    if insertion.strip() in text:
        return text
    if marker not in text:
        raise SystemExit(f"ERROR: could not find insertion marker for {label}")
    return text.replace(marker, insertion + marker, 1)


old = '''fn render_sync(dry_run: bool, yes: bool, output_format: OutputFormat) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;
'''

new = '''fn render_sync(dry_run: bool, yes: bool, output_format: OutputFormat) -> Result<String, String> {
    let context = discover_sync_context()?;
'''

if old in text:
    text = text.replace(old, new, 1)
elif new in text:
    pass
else:
    raise SystemExit("ERROR: expected render_sync context discovery line not found")

helper = '''/// Discovers a workspace context for sync.
///
/// Most commands should require an initialized Monad workspace. Sync is a
/// little different: it is the command that reports whether the current
/// directory is missing the core Monad contract. If normal discovery fails, we
/// intentionally fall back to treating `.` as the root so the planner can emit
/// `monad.toml` / `.monad` missing findings instead of failing early.
fn discover_sync_context() -> Result<WorkspaceContext, String> {
    match WorkspaceContext::discover_from(".") {
        Ok(context) => Ok(context),
        Err(_) => WorkspaceContext::new(".").map_err(|error| error.to_string()),
    }
}

'''

if "fn discover_sync_context()" not in text:
    marker = "/// Renders or applies repository sync output.\n"
    text = insert_before(text, marker, helper, "discover_sync_context helper")

# Add parser/runtime-level test if there is a suitable test module.
if "fn sync_discovery_falls_back_to_current_directory" not in text:
    test = '''    #[test]
    fn sync_discovery_falls_back_to_current_directory() {
        let current = std::env::current_dir().expect("test current directory should be available");
        let temp_root = std::env::temp_dir().join(format!(
            "monad-sync-cli-fallback-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock should be after unix epoch")
                .as_nanos()
        ));

        std::fs::create_dir_all(&temp_root).expect("temp root should be created");
        std::env::set_current_dir(&temp_root).expect("test should enter temp root");

        let output = render_sync(true, false, OutputFormat::Text)
            .expect("sync dry-run should fall back to current directory");

        assert!(output.contains("Monad sync dry-run plan"));
        assert!(output.contains("monad.manifest.missing"));
        assert!(output.contains("No files were written."));

        std::env::set_current_dir(current).expect("test should restore current directory");
        std::fs::remove_dir_all(temp_root).ok();
    }

'''
    marker = "    #[test]\n    fn sync_dry_run_command_parses()"
    if marker in text:
        text = text.replace(marker, test + marker, 1)
    else:
        # Some local layouts put tests in a different order. It is safe to skip
        # this test if the marker is unavailable; the smoke script covers it.
        print("sync parser test marker not found; skipping CLI fallback unit test")

path.write_text(text)
PY

echo "==> Formatting Rust code"
cargo fmt

echo
echo "==> Sync workspace fallback repair complete."
echo
echo "Focused verification:"
echo "  cargo test -p monad-cli sync_discovery_falls_back_to_current_directory"
echo "  cargo run -p monad-cli -- sync --dry-run"
echo
echo "Then continue full E14 verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-sync.sh"
echo "  tools/scripts/verify-e14.sh"
