#!/usr/bin/env bash
set -euo pipefail

EPIC_ID="e23-language-adapter-foundation"
BACKUP_ROOT=".monad/script-backups/${EPIC_ID}-$(date -u +%Y%m%dT%H%M%SZ)"

log() {
  printf '[E23] %s\n' "$1"
}

fail() {
  printf '[E23][ERROR] %s\n' "$1" >&2
  exit 1
}

backup_file() {
  local path="$1"
  if [[ -f "$path" ]]; then
    mkdir -p "${BACKUP_ROOT}/$(dirname "$path")"
    cp "$path" "${BACKUP_ROOT}/${path}"
  fi
}

write_file() {
  local path="$1"
  local tmp
  tmp="$(mktemp)"
  cat > "$tmp"

  if [[ -f "$path" ]] && cmp -s "$tmp" "$path"; then
    rm -f "$tmp"
    log "unchanged: ${path}"
    return 0
  fi

  backup_file "$path"
  mkdir -p "$(dirname "$path")"
  mv "$tmp" "$path"
  log "wrote: ${path}"
}

[[ -f Cargo.toml ]] || fail "run this script from the repository root"
[[ -d crates/monad-core/src ]] || fail "missing crates/monad-core/src"
[[ -f crates/monad-cli/src/main.rs ]] || fail "missing crates/monad-cli/src/main.rs"
mkdir -p "$BACKUP_ROOT"

log "backups will be stored under ${BACKUP_ROOT}"

write_file "crates/monad-core/src/language_adapters.rs" <<'EOF'
//! Language adapter registry foundation.
//!
//! E23 defines Monad's first language adapter contract. Adapters describe how
//! Monad recognizes a language ecosystem and which native commands a future
//! supervised workflow may suggest. This module is intentionally planning-only:
//! it does not install tools, execute commands, fetch remote metadata, or mutate
//! user-owned source files.

use std::path::Path;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

/// Supported language adapter family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LanguageAdapterKind {
    /// Rust / Cargo repositories.
    Rust,

    /// Node.js and Bun repositories.
    NodeBun,

    /// Python repositories.
    Python,

    /// Go repositories.
    Go,

    /// Java / Gradle / Maven repositories.
    Java,
}

impl LanguageAdapterKind {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::NodeBun => "node-bun",
            Self::Python => "python",
            Self::Go => "go",
            Self::Java => "java",
        }
    }

    /// Human-readable label.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::NodeBun => "Node/Bun",
            Self::Python => "Python",
            Self::Go => "Go",
            Self::Java => "Java",
        }
    }
}

/// Capabilities a language adapter can expose to Monad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LanguageAdapterCapability {
    /// Detect whether the language is present in the workspace.
    Detect,

    /// Format source files through the ecosystem-native formatter.
    Format,

    /// Run ecosystem-native linting.
    Lint,

    /// Run ecosystem-native tests.
    Test,

    /// Run ecosystem-native builds/checks.
    Build,

    /// Describe package-manager files and lockfiles.
    PackageManager,
}

impl LanguageAdapterCapability {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Detect => "detect",
            Self::Format => "format",
            Self::Lint => "lint",
            Self::Test => "test",
            Self::Build => "build",
            Self::PackageManager => "package-manager",
        }
    }
}

/// Native command intent exposed by an adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LanguageAdapterCommandIntent {
    /// Format source files.
    Format,

    /// Lint or statically check source files.
    Lint,

    /// Run tests.
    Test,

    /// Build or type-check the project.
    Build,
}

impl LanguageAdapterCommandIntent {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Format => "format",
            Self::Lint => "lint",
            Self::Test => "test",
            Self::Build => "build",
        }
    }
}

/// A native command suggestion. Monad records this command; it does not run it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageAdapterCommand {
    intent: LanguageAdapterCommandIntent,
    command: &'static str,
    required_marker: &'static str,
    rationale: &'static str,
}

impl LanguageAdapterCommand {
    /// Creates a command definition.
    #[must_use]
    pub const fn new(
        intent: LanguageAdapterCommandIntent,
        command: &'static str,
        required_marker: &'static str,
        rationale: &'static str,
    ) -> Self {
        Self {
            intent,
            command,
            required_marker,
            rationale,
        }
    }

    /// Command intent.
    #[must_use]
    pub const fn intent(&self) -> LanguageAdapterCommandIntent {
        self.intent
    }

    /// Native command string.
    #[must_use]
    pub const fn command(&self) -> &'static str {
        self.command
    }

    /// Marker file normally required before this command is relevant.
    #[must_use]
    pub const fn required_marker(&self) -> &'static str {
        self.required_marker
    }

    /// Rationale.
    #[must_use]
    pub const fn rationale(&self) -> &'static str {
        self.rationale
    }
}

/// One language adapter definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageAdapter {
    kind: LanguageAdapterKind,
    root_markers: Vec<&'static str>,
    package_managers: Vec<&'static str>,
    capabilities: Vec<LanguageAdapterCapability>,
    commands: Vec<LanguageAdapterCommand>,
}

impl LanguageAdapter {
    /// Adapter kind.
    #[must_use]
    pub const fn kind(&self) -> LanguageAdapterKind {
        self.kind
    }

    /// Stable adapter identifier.
    #[must_use]
    pub const fn id(&self) -> &'static str {
        self.kind.as_str()
    }

    /// Human-readable adapter name.
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        self.kind.display_name()
    }

    /// Root markers used for detection.
    #[must_use]
    pub fn root_markers(&self) -> &[&'static str] {
        &self.root_markers
    }

    /// Package manager indicators.
    #[must_use]
    pub fn package_managers(&self) -> &[&'static str] {
        &self.package_managers
    }

    /// Capability list.
    #[must_use]
    pub fn capabilities(&self) -> &[LanguageAdapterCapability] {
        &self.capabilities
    }

    /// Native command suggestions.
    #[must_use]
    pub fn commands(&self) -> &[LanguageAdapterCommand] {
        &self.commands
    }
}

/// Deterministic registry of language adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageAdapterRegistry {
    adapters: Vec<LanguageAdapter>,
}

impl LanguageAdapterRegistry {
    /// Creates a registry and sorts adapters deterministically.
    #[must_use]
    pub fn new(mut adapters: Vec<LanguageAdapter>) -> Self {
        adapters.sort_by_key(LanguageAdapter::kind);
        Self { adapters }
    }

    /// Registered adapters.
    #[must_use]
    pub fn adapters(&self) -> &[LanguageAdapter] {
        &self.adapters
    }

    /// Adapter count.
    #[must_use]
    pub fn adapter_count(&self) -> usize {
        self.adapters.len()
    }

    /// Finds an adapter by kind.
    #[must_use]
    pub fn find(&self, kind: LanguageAdapterKind) -> Option<&LanguageAdapter> {
        self.adapters.iter().find(|adapter| adapter.kind() == kind)
    }
}

/// Builds Monad's initial language adapter registry.
#[must_use]
pub fn build_language_adapter_registry() -> LanguageAdapterRegistry {
    LanguageAdapterRegistry::new(vec![
        rust_adapter(),
        node_bun_adapter(),
        python_adapter(),
        go_adapter(),
        java_adapter(),
    ])
}

fn rust_adapter() -> LanguageAdapter {
    LanguageAdapter {
        kind: LanguageAdapterKind::Rust,
        root_markers: vec!["Cargo.toml", "Cargo.lock", "rust-toolchain.toml"],
        package_managers: vec!["cargo"],
        capabilities: vec![
            LanguageAdapterCapability::Detect,
            LanguageAdapterCapability::Format,
            LanguageAdapterCapability::Lint,
            LanguageAdapterCapability::Test,
            LanguageAdapterCapability::Build,
            LanguageAdapterCapability::PackageManager,
        ],
        commands: vec![
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Format,
                "cargo fmt --check",
                "Cargo.toml",
                "checks Rust formatting through Cargo/rustfmt",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Lint,
                "cargo clippy --all-targets --all-features -- -D warnings",
                "Cargo.toml",
                "runs Rust lint checks through Clippy",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Test,
                "cargo test",
                "Cargo.toml",
                "runs Rust tests through Cargo",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Build,
                "cargo check",
                "Cargo.toml",
                "checks Rust workspace compilation without producing release artifacts",
            ),
        ],
    }
}

fn node_bun_adapter() -> LanguageAdapter {
    LanguageAdapter {
        kind: LanguageAdapterKind::NodeBun,
        root_markers: vec!["package.json", "bun.lock", "bun.lockb", "tsconfig.json"],
        package_managers: vec!["bun", "npm", "pnpm", "yarn"],
        capabilities: vec![
            LanguageAdapterCapability::Detect,
            LanguageAdapterCapability::Format,
            LanguageAdapterCapability::Lint,
            LanguageAdapterCapability::Test,
            LanguageAdapterCapability::Build,
            LanguageAdapterCapability::PackageManager,
        ],
        commands: vec![
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Lint,
                "bun run lint",
                "package.json",
                "uses Bun when repository scripts define lint behavior",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Test,
                "bun test",
                "package.json",
                "uses Bun's native test runner when configured",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Build,
                "bun run build",
                "package.json",
                "uses package scripts for project-specific build behavior",
            ),
        ],
    }
}

fn python_adapter() -> LanguageAdapter {
    LanguageAdapter {
        kind: LanguageAdapterKind::Python,
        root_markers: vec!["pyproject.toml", "requirements.txt", "uv.lock", "poetry.lock"],
        package_managers: vec!["uv", "poetry", "pip"],
        capabilities: vec![
            LanguageAdapterCapability::Detect,
            LanguageAdapterCapability::Format,
            LanguageAdapterCapability::Lint,
            LanguageAdapterCapability::Test,
            LanguageAdapterCapability::PackageManager,
        ],
        commands: vec![
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Format,
                "python -m black --check .",
                "pyproject.toml",
                "checks Python formatting when Black is configured",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Lint,
                "python -m ruff check .",
                "pyproject.toml",
                "checks Python lint rules when Ruff is configured",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Test,
                "python -m pytest",
                "pyproject.toml",
                "runs Python tests through pytest when configured",
            ),
        ],
    }
}

fn go_adapter() -> LanguageAdapter {
    LanguageAdapter {
        kind: LanguageAdapterKind::Go,
        root_markers: vec!["go.mod", "go.sum"],
        package_managers: vec!["go"],
        capabilities: vec![
            LanguageAdapterCapability::Detect,
            LanguageAdapterCapability::Format,
            LanguageAdapterCapability::Lint,
            LanguageAdapterCapability::Test,
            LanguageAdapterCapability::Build,
            LanguageAdapterCapability::PackageManager,
        ],
        commands: vec![
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Format,
                "gofmt -w .",
                "go.mod",
                "records canonical Go formatting command; execution remains supervised",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Test,
                "go test ./...",
                "go.mod",
                "runs Go tests through the native Go toolchain",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Build,
                "go build ./...",
                "go.mod",
                "checks Go buildability through the native Go toolchain",
            ),
        ],
    }
}

fn java_adapter() -> LanguageAdapter {
    LanguageAdapter {
        kind: LanguageAdapterKind::Java,
        root_markers: vec!["build.gradle", "build.gradle.kts", "pom.xml", "settings.gradle"],
        package_managers: vec!["gradle", "maven"],
        capabilities: vec![
            LanguageAdapterCapability::Detect,
            LanguageAdapterCapability::Lint,
            LanguageAdapterCapability::Test,
            LanguageAdapterCapability::Build,
            LanguageAdapterCapability::PackageManager,
        ],
        commands: vec![
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Test,
                "./gradlew test",
                "build.gradle",
                "prefers repository-local Gradle wrapper when present",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Build,
                "./gradlew build",
                "build.gradle",
                "builds Java projects through repository-local Gradle wrapper when present",
            ),
            LanguageAdapterCommand::new(
                LanguageAdapterCommandIntent::Test,
                "mvn test",
                "pom.xml",
                "supports Maven projects when pom.xml is the primary marker",
            ),
        ],
    }
}

/// Renders the adapter registry as deterministic text.
#[must_use]
pub fn render_language_adapter_registry(registry: &LanguageAdapterRegistry) -> String {
    let mut lines = vec![
        "Monad language adapter registry".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!("  adapters: {}", registry.adapter_count()),
        String::new(),
        "Adapters:".to_string(),
    ];

    for adapter in registry.adapters() {
        let capabilities = adapter
            .capabilities()
            .iter()
            .map(|capability| capability.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        lines.push(format!("  - {} ({})", adapter.display_name(), adapter.id()));
        lines.push(format!(
            "    markers: {}",
            adapter.root_markers().join(", ")
        ));
        lines.push(format!(
            "    package_managers: {}",
            adapter.package_managers().join(", ")
        ));
        lines.push(format!("    capabilities: {capabilities}"));
        lines.push("    command suggestions:".to_string());

        for command in adapter.commands() {
            lines.push(format!(
                "      - {}: {} [marker: {}]",
                command.intent().as_str(),
                command.command(),
                command.required_marker()
            ));
            lines.push(format!("        rationale: {}", command.rationale()));
        }
    }

    lines.push(String::new());
    lines.push("Safety:".to_string());
    lines.push("  no commands were executed".to_string());
    lines.push("  no tools were installed".to_string());
    lines.push("  no package managers were invoked".to_string());
    lines.push("  no remote registries were contacted".to_string());
    lines.push("  no user-owned source files were rewritten".to_string());

    lines.join("\n")
}

/// Renders the adapter registry as deterministic JSON.
#[must_use]
pub fn render_language_adapter_registry_json(registry: &LanguageAdapterRegistry) -> String {
    let adapters = registry
        .adapters()
        .iter()
        .map(render_adapter_json)
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"adapters\",\"adapters\":{},\"items\":[{}],\"safety\":{{\"commands_executed\":false,\"tools_installed\":false,\"remote_registries_contacted\":false,\"source_files_rewritten\":false}}}}",
        registry.adapter_count(),
        adapters
    )
}

fn render_adapter_json(adapter: &LanguageAdapter) -> String {
    let markers = json_array(adapter.root_markers());
    let package_managers = json_array(adapter.package_managers());
    let capabilities = adapter
        .capabilities()
        .iter()
        .map(|capability| format!("\"{}\"", capability.as_str()))
        .collect::<Vec<_>>()
        .join(",");
    let commands = adapter
        .commands()
        .iter()
        .map(|command| {
            format!(
                "{{\"intent\":\"{}\",\"command\":\"{}\",\"required_marker\":\"{}\",\"rationale\":\"{}\"}}",
                command.intent().as_str(),
                json_escape(command.command()),
                json_escape(command.required_marker()),
                json_escape(command.rationale())
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"id\":\"{}\",\"display_name\":\"{}\",\"root_markers\":[{}],\"package_managers\":[{}],\"capabilities\":[{}],\"commands\":[{}]}}",
        adapter.id(),
        json_escape(adapter.display_name()),
        markers,
        package_managers,
        capabilities,
        commands
    )
}

fn json_array(values: &[&str]) -> String {
    values
        .iter()
        .map(|value| format!("\"{}\"", json_escape(value)))
        .collect::<Vec<_>>()
        .join(",")
}

/// Writes generated language-adapter evidence under `.monad/reports`.
pub fn write_language_adapter_evidence(root: impl AsRef<Path>) -> Result<Vec<GatedWriteResult>, String> {
    let registry = build_language_adapter_registry();
    let markdown = render_language_adapter_registry(&registry);
    let json = render_language_adapter_registry_json(&registry);

    let requests = [
        GatedWriteRequest::new(".monad/reports/language-adapters.md", markdown, true),
        GatedWriteRequest::new(".monad/reports/language-adapters.json", json, true),
    ];

    requests
        .iter()
        .map(|request| gated_generated_write(root.as_ref(), request))
        .collect()
}

/// Renders language-adapter evidence write results.
#[must_use]
pub fn render_language_adapter_evidence_results(results: &[GatedWriteResult]) -> String {
    let mut lines = vec![
        "Monad language adapter evidence write result".to_string(),
        String::new(),
        "Results:".to_string(),
    ];

    for result in results {
        match result {
            GatedWriteResult::Written(path) | GatedWriteResult::SkippedIdentical(path) => {
                lines.push(format!("  - [{}] {}", result.as_str(), path.display()));
            }
            GatedWriteResult::ApprovalRequired(message) | GatedWriteResult::Blocked(message) => {
                lines.push(format!("  - [{}] {message}", result.as_str()));
            }
        }
    }

    lines.push(String::new());
    lines.push("No commands were executed.".to_string());
    lines.push("No tools were installed.".to_string());
    lines.push("No package managers were invoked.".to_string());
    lines.push("No remote registries were contacted.".to_string());
    lines.push("No user-owned source files were rewritten.".to_string());

    lines.join("\n")
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-language-adapters-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    #[test]
    fn registry_contains_initial_polyglot_adapters() {
        let registry = build_language_adapter_registry();

        assert_eq!(registry.adapter_count(), 5);
        assert!(registry.find(LanguageAdapterKind::Rust).is_some());
        assert!(registry.find(LanguageAdapterKind::NodeBun).is_some());
        assert!(registry.find(LanguageAdapterKind::Python).is_some());
        assert!(registry.find(LanguageAdapterKind::Go).is_some());
        assert!(registry.find(LanguageAdapterKind::Java).is_some());
    }

    #[test]
    fn rust_adapter_records_cargo_commands_without_execution() {
        let registry = build_language_adapter_registry();
        let rust = registry
            .find(LanguageAdapterKind::Rust)
            .expect("Rust adapter should exist");

        let commands = rust
            .commands()
            .iter()
            .map(LanguageAdapterCommand::command)
            .collect::<Vec<_>>();

        assert!(commands.contains(&"cargo fmt --check"));
        assert!(commands.contains(&"cargo test"));
    }

    #[test]
    fn registry_text_render_is_deterministic_and_safety_first() {
        let registry = build_language_adapter_registry();
        let rendered = render_language_adapter_registry(&registry);

        assert!(rendered.contains("Monad language adapter registry"));
        assert!(rendered.contains("adapters: 5"));
        assert!(rendered.contains("Node/Bun"));
        assert!(rendered.contains("no commands were executed"));
        assert!(rendered.contains("no remote registries were contacted"));
    }

    #[test]
    fn registry_json_render_contains_all_adapter_ids() {
        let registry = build_language_adapter_registry();
        let rendered = render_language_adapter_registry_json(&registry);

        assert!(rendered.contains("\"command\":\"adapters\""));
        assert!(rendered.contains("\"id\":\"rust\""));
        assert!(rendered.contains("\"id\":\"node-bun\""));
        assert!(rendered.contains("\"id\":\"python\""));
        assert!(rendered.contains("\"id\":\"go\""));
        assert!(rendered.contains("\"id\":\"java\""));
    }

    #[test]
    fn evidence_write_uses_generated_write_gate() {
        let root = unique_temp_dir("evidence-write");
        fs::create_dir_all(&root).expect("temp root should be created");

        let results = write_language_adapter_evidence(&root).expect("evidence should write");

        assert_eq!(results.len(), 2);
        assert!(root.join(".monad/reports/language-adapters.md").is_file());
        assert!(root.join(".monad/reports/language-adapters.json").is_file());

        fs::remove_dir_all(root).ok();
    }
}
EOF

python3 - <<'PY'
from pathlib import Path

lib = Path('crates/monad-core/src/lib.rs')
text = lib.read_text()
changed = False

if 'pub mod language_adapters;' not in text:
    marker = 'pub mod init;\n'
    if marker in text:
        text = text.replace(marker, marker + 'pub mod language_adapters;\n', 1)
    else:
        text = 'pub mod language_adapters;\n' + text
    changed = True

export = '''pub use language_adapters::{
    LanguageAdapter, LanguageAdapterCapability, LanguageAdapterCommand, LanguageAdapterCommandIntent,
    LanguageAdapterKind, LanguageAdapterRegistry, build_language_adapter_registry,
    render_language_adapter_evidence_results, render_language_adapter_registry,
    render_language_adapter_registry_json, write_language_adapter_evidence,
};
'''

if 'build_language_adapter_registry' not in text:
    insert_at = text.find('#[cfg(test)]')
    if insert_at == -1:
        text = text.rstrip() + '\n' + export
    else:
        text = text[:insert_at] + export + '\n' + text[insert_at:]
    changed = True

if changed:
    backup = Path('${BACKUP_ROOT}') / 'crates/monad-core/src/lib.rs'
    backup.parent.mkdir(parents=True, exist_ok=True)
    backup.write_text(lib.read_text())
    lib.write_text(text)
    print('[E23] updated: crates/monad-core/src/lib.rs')
else:
    print('[E23] unchanged: crates/monad-core/src/lib.rs')
PY

python3 - <<'PY'
from pathlib import Path

path = Path('crates/monad-cli/src/main.rs')
text = path.read_text()
original = text

extra_import = '''use monad_core::{
    build_language_adapter_registry, render_language_adapter_evidence_results,
    render_language_adapter_registry, render_language_adapter_registry_json,
    write_language_adapter_evidence,
};
'''

if 'build_language_adapter_registry' not in text:
    start = text.find('use monad_core::{')
    if start == -1:
        raise SystemExit('Could not find monad_core import block')
    end = text.find('};', start)
    if end == -1:
        raise SystemExit('Could not find end of monad_core import block')
    end += len('};')
    text = text[:end] + '\n' + extra_import + text[end:]

variant = '''
    /// Render or write language adapter registry evidence.
    LanguageAdapters {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write generated adapter evidence.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    },
'''

if 'LanguageAdapters {' not in text:
    marker = '    /// Diagnose local environment and repository readiness.\n    Doctor {'
    if marker not in text:
        marker = '    /// Run workspace checks.\n    Check {'
    if marker not in text:
        raise SystemExit('Could not find CliCommand insertion point for LanguageAdapters')
    text = text.replace(marker, variant + '\n' + marker, 1)

if 'Some("adapters")' not in text and 'if yes' in text:
    yes_marker = '&& parts.first().copied() != Some("policy")'
    if yes_marker in text:
        text = text.replace(yes_marker, yes_marker + '\n            && parts.first().copied() != Some("adapters")\n            && parts.first().copied() != Some("language-adapters")', 1)
    else:
        yes_marker = '&& parts.first().copied() != Some("sync")'
        if yes_marker in text:
            text = text.replace(yes_marker, yes_marker + '\n            && parts.first().copied() != Some("adapters")\n            && parts.first().copied() != Some("language-adapters")', 1)
        else:
            print('[E23][WARN] Could not update global --yes allowlist; continuing because command parser may already be compatible')

if '--yes is only supported for init, add, sync, upgrade, ai-context, and policy commands' in text:
    text = text.replace(
        '--yes is only supported for init, add, sync, upgrade, ai-context, and policy commands',
        '--yes is only supported for init, add, sync, upgrade, ai-context, policy, and adapters commands'
    )

parse_arm = '''
            ["adapters"] | ["language-adapters"] => {
                reject_write_for_non_context(write)?;
                require_language_adapters_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::LanguageAdapters {
                    dry_run,
                    yes,
                    output_format,
                })
            }
            ["adapters", other, ..] | ["language-adapters", other, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown adapters argument: {other}"))
            }
'''

if '["adapters"] | ["language-adapters"]' not in text:
    marker = '            ["doctor"] => {'
    if marker not in text:
        marker = '            ["check"] => {'
    if marker not in text:
        raise SystemExit('Could not find parser match insertion point for adapters command')
    text = text.replace(marker, parse_arm + marker, 1)

run_arm = '''
        CliCommand::LanguageAdapters {
            dry_run,
            yes,
            output_format,
        } => render_language_adapters(dry_run, yes, output_format),
'''

if 'render_language_adapters(dry_run, yes, output_format)' not in text:
    marker = '        CliCommand::Doctor { output_format } => render_doctor(output_format),'
    if marker not in text:
        marker = '        CliCommand::Check { output_format } => render_check(output_format),'
    if marker not in text:
        raise SystemExit('Could not find run match insertion point for LanguageAdapters')
    text = text.replace(marker, run_arm + marker, 1)

require_fn = '''
/// Requires exactly one language-adapter mode.
fn require_language_adapters_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => Err(
            "adapters currently requires either --dry-run to preview or --yes to write generated adapter evidence".to_string(),
        ),
        (true, true) => Err("adapters accepts either --dry-run or --yes, not both".to_string()),
    }
}
'''

if 'fn require_language_adapters_mode' not in text:
    marker = '/// Requires exactly one sync mode for the guarded sync implementation.\nfn require_sync_mode'
    if marker in text:
        text = text.replace(marker, require_fn + '\n' + marker, 1)
    else:
        marker = '/// Requires exactly one init mode for the guarded init implementation.\nfn require_init_mode'
        if marker not in text:
            raise SystemExit('Could not find mode helper insertion point')
        text = text.replace(marker, require_fn + '\n' + marker, 1)

render_fn = '''
/// Renders or writes language adapter registry evidence.
fn render_language_adapters(
    dry_run: bool,
    yes: bool,
    output_format: OutputFormat,
) -> Result<String, String> {
    if dry_run {
        let registry = build_language_adapter_registry();
        return match output_format {
            OutputFormat::Text => Ok(render_language_adapter_registry(&registry)),
            OutputFormat::Json => Ok(render_language_adapter_registry_json(&registry)),
        };
    }

    if yes {
        let root = std::env::current_dir().map_err(|error| error.to_string())?;
        let results = write_language_adapter_evidence(root)?;
        return Ok(render_language_adapter_evidence_results(&results));
    }

    Err("adapters currently requires either --dry-run to preview or --yes to write generated adapter evidence".to_string())
}
'''

if 'fn render_language_adapters(' not in text:
    marker = '/// Renders doctor diagnostics.\nfn render_doctor'
    if marker not in text:
        marker = '/// Renders workspace checks.\nfn render_check'
    if marker not in text:
        raise SystemExit('Could not find render function insertion point')
    text = text.replace(marker, render_fn + '\n' + marker, 1)

if '  adapters --dry-run' not in text:
    text = text.replace(
        '  doctor                                   Diagnose local environment and repo readiness",',
        '  doctor                                   Diagnose local environment and repo readiness",\n        "  adapters --dry-run                      Preview language adapter registry",\n        "  adapters --dry-run --format=json        Preview language adapter registry as JSON",\n        "  adapters --yes                          Write generated adapter evidence",',
        1,
    )

if '  monad adapters --dry-run' not in text:
    text = text.replace(
        '  monad doctor",',
        '  monad doctor",\n        "  monad adapters --dry-run",\n        "  monad adapters --dry-run --format=json",',
        1,
    )

if '  adapters records command suggestions only; it does not run native tools.' not in text:
    text = text.replace(
        '  sync writes generated evidence reports only.",',
        '  sync writes generated evidence reports only.",\n        "  adapters records command suggestions only; it does not run native tools.",',
        1,
    )

if text != original:
    backup = Path('${BACKUP_ROOT}') / 'crates/monad-cli/src/main.rs'
    backup.parent.mkdir(parents=True, exist_ok=True)
    backup.write_text(original)
    path.write_text(text)
    print('[E23] updated: crates/monad-cli/src/main.rs')
else:
    print('[E23] unchanged: crates/monad-cli/src/main.rs')
PY

write_file "docs/language-adapters/README.md" <<'EOF'
---
title: Language Adapter Foundation
status: active
owner: Monad
last_reviewed: 2026-06-09
---

# Language Adapter Foundation

E23 introduces Monad's first language adapter registry.

Adapters are local, deterministic descriptions of language ecosystems. They tell
Monad how to recognize a language and which ecosystem-native commands may be
suggested in supervised plans. They do **not** execute commands by themselves.

## Initial adapters

- Rust / Cargo
- Node/Bun
- Python
- Go
- Java / Gradle / Maven

## Safety boundary

The E23 registry is planning and evidence only:

- no tool installation;
- no package-manager invocation;
- no arbitrary command execution;
- no remote registry calls;
- no AI provider calls;
- no user-owned source rewrites;
- no autonomous agent execution.

Generated evidence may be written under `.monad/reports` only when the caller
uses `monad adapters --yes`.
EOF

write_file "docs/roadmap/epic-23-language-adapter-foundation.md" <<'EOF'
---
title: E23 — Language Adapter Foundation
status: implemented
owner: Monad
last_reviewed: 2026-06-09
---

# E23 — Language Adapter Foundation

## Scope

E23 implements the first language adapter foundation for Monad.

## Work packets

- WP-E23-001 — Define language adapter interface contract
- WP-E23-002 — Add Rust adapter foundation
- WP-E23-003 — Add Node/Bun adapter foundation
- WP-E23-004 — Add Python adapter foundation
- WP-E23-005 — Add Go and Java adapter foundations
- WP-E23-006 — Add adapter registry tests and documentation

## Command surface

```bash
monad adapters --dry-run
monad adapters --dry-run --format=json
monad adapters --yes
```

`language-adapters` is accepted as an alias for `adapters`.

## Evidence

`monad adapters --yes` writes generated evidence to:

- `.monad/reports/language-adapters.md`
- `.monad/reports/language-adapters.json`

## Safety

E23 does not execute native tools. It records language adapter metadata and
command suggestions for future supervised workflows.
EOF

write_file "tools/scripts/verify-language-adapters.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "[verify-language-adapters] cargo test -p monad-core --lib language_adapters"
cargo test -p monad-core --lib language_adapters

echo "[verify-language-adapters] adapters dry-run smoke"
cargo run -q -p monad-cli -- adapters --dry-run > /tmp/monad-adapters.txt
grep -q "Monad language adapter registry" /tmp/monad-adapters.txt
grep -q "Rust" /tmp/monad-adapters.txt
grep -q "Node/Bun" /tmp/monad-adapters.txt
grep -q "Python" /tmp/monad-adapters.txt
grep -q "Go" /tmp/monad-adapters.txt
grep -q "Java" /tmp/monad-adapters.txt
grep -q "no commands were executed" /tmp/monad-adapters.txt

echo "[verify-language-adapters] adapters json smoke"
cargo run -q -p monad-cli -- adapters --dry-run --format=json > /tmp/monad-adapters.json
grep -q '"command":"adapters"' /tmp/monad-adapters.json
grep -q '"id":"rust"' /tmp/monad-adapters.json
grep -q '"id":"node-bun"' /tmp/monad-adapters.json
grep -q '"id":"python"' /tmp/monad-adapters.json
grep -q '"id":"go"' /tmp/monad-adapters.json
grep -q '"id":"java"' /tmp/monad-adapters.json

echo "[verify-language-adapters] ok"
EOF
chmod +x tools/scripts/verify-language-adapters.sh

write_file "tools/scripts/verify-e23.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "[verify-e23] checking expected files"
test -f crates/monad-core/src/language_adapters.rs
test -f docs/language-adapters/README.md
test -f docs/roadmap/epic-23-language-adapter-foundation.md
test -x tools/scripts/verify-language-adapters.sh

echo "[verify-e23] cargo fmt --check"
cargo fmt --check

echo "[verify-e23] cargo test"
cargo test

echo "[verify-e23] cargo clippy"
cargo clippy --all-targets --all-features -- -D warnings

echo "[verify-e23] language adapter verification"
tools/scripts/verify-language-adapters.sh

echo "[verify-e23] ok"
EOF
chmod +x tools/scripts/verify-e23.sh

cargo fmt

log "E23 language adapter foundation script completed"
log "Run: cargo fmt --check && cargo test && cargo clippy --all-targets --all-features -- -D warnings && tools/scripts/verify-language-adapters.sh && tools/scripts/verify-e23.sh"
