//! Plugin and extension system foundation.
//!
//! E30 defines a disabled-by-default plugin boundary, local plugin manifest
//! schema, extension point registry, and loading-plan model. This module never
//! loads plugin code, opens dynamic libraries, executes plugin binaries, queries
//! remote registries, or enables plugins automatically.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

/// Trust level declared or inferred for a plugin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginTrustLevel {
    /// Local metadata exists, but no explicit trust signal was provided.
    Untrusted,

    /// Plugin is trusted only for planning and review.
    Review,

    /// Plugin is trusted for future supervised use after explicit enablement.
    Trusted,
}

impl PluginTrustLevel {
    /// Stable trust-level label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Untrusted => "untrusted",
            Self::Review => "review",
            Self::Trusted => "trusted",
        }
    }
}

/// Loading decision for a plugin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginLoadDecision {
    /// Plugin must not be loaded.
    Blocked,

    /// Plugin can be reviewed but not loaded automatically.
    Review,

    /// Plugin is allowed only as a future supervised loading candidate.
    Candidate,
}

impl PluginLoadDecision {
    /// Stable decision label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Review => "review",
            Self::Candidate => "candidate",
        }
    }
}

/// Built-in extension point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExtensionPoint {
    id: String,
    description: String,
    disabled_by_default: bool,
}

impl ExtensionPoint {
    /// Creates an extension point.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        disabled_by_default: bool,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            disabled_by_default,
        }
    }

    /// Extension point ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Whether extension point is disabled by default.
    #[must_use]
    pub const fn disabled_by_default(&self) -> bool {
        self.disabled_by_default
    }
}

/// Local plugin manifest metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginManifestRecord {
    id: String,
    path: PathBuf,
    name: String,
    version: String,
    extension_points: Vec<String>,
    trust_level: PluginTrustLevel,
    enabled: bool,
}

impl PluginManifestRecord {
    /// Creates a plugin manifest record.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        path: impl Into<PathBuf>,
        name: impl Into<String>,
        version: impl Into<String>,
        mut extension_points: Vec<String>,
        trust_level: PluginTrustLevel,
        enabled: bool,
    ) -> Self {
        extension_points.sort();
        extension_points.dedup();

        Self {
            id: id.into(),
            path: path.into(),
            name: name.into(),
            version: version.into(),
            extension_points,
            trust_level,
            enabled,
        }
    }

    /// Plugin ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Manifest path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Plugin name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Plugin version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Extension point IDs requested by the plugin.
    #[must_use]
    pub fn extension_points(&self) -> &[String] {
        &self.extension_points
    }

    /// Trust level.
    #[must_use]
    pub const fn trust_level(&self) -> PluginTrustLevel {
        self.trust_level
    }

    /// Whether manifest explicitly enables the plugin.
    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }
}

/// Plugin safety finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginSafetyFinding {
    plugin_id: String,
    severity: String,
    message: String,
}

impl PluginSafetyFinding {
    /// Creates a safety finding.
    #[must_use]
    pub fn new(
        plugin_id: impl Into<String>,
        severity: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            severity: severity.into(),
            message: message.into(),
        }
    }

    /// Plugin ID.
    #[must_use]
    pub fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    /// Severity.
    #[must_use]
    pub fn severity(&self) -> &str {
        &self.severity
    }

    /// Message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Loading plan entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginLoadingPlanEntry {
    plugin_id: String,
    decision: PluginLoadDecision,
    reason: String,
}

impl PluginLoadingPlanEntry {
    /// Creates a loading plan entry.
    #[must_use]
    pub fn new(
        plugin_id: impl Into<String>,
        decision: PluginLoadDecision,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            decision,
            reason: reason.into(),
        }
    }

    /// Plugin ID.
    #[must_use]
    pub fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    /// Decision.
    #[must_use]
    pub const fn decision(&self) -> PluginLoadDecision {
        self.decision
    }

    /// Reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Full plugin system plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginSystemPlan {
    command: String,
    extension_points: Vec<ExtensionPoint>,
    plugins: Vec<PluginManifestRecord>,
    safety_findings: Vec<PluginSafetyFinding>,
    loading_plan: Vec<PluginLoadingPlanEntry>,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl PluginSystemPlan {
    /// Creates a deterministic plugin system plan.
    #[must_use]
    pub fn new(
        mut extension_points: Vec<ExtensionPoint>,
        mut plugins: Vec<PluginManifestRecord>,
        mut safety_findings: Vec<PluginSafetyFinding>,
        mut loading_plan: Vec<PluginLoadingPlanEntry>,
    ) -> Self {
        extension_points.sort_by(|left, right| left.id().cmp(right.id()));
        extension_points.dedup_by(|left, right| left.id() == right.id());

        plugins.sort_by(|left, right| left.id().cmp(right.id()));
        plugins.dedup_by(|left, right| left.id() == right.id());

        safety_findings.sort_by(|left, right| {
            left.plugin_id()
                .cmp(right.plugin_id())
                .then(left.severity().cmp(right.severity()))
                .then(left.message().cmp(right.message()))
        });

        loading_plan.sort_by(|left, right| left.plugin_id().cmp(right.plugin_id()));
        loading_plan.dedup_by(|left, right| left.plugin_id() == right.plugin_id());

        Self {
            command: "plugin-plan".to_string(),
            extension_points,
            plugins,
            safety_findings,
            loading_plan,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/plugin-system-plan.md"),
                PathBuf::from(".monad/reports/plugin-system-plan.json"),
            ],
            safety_notes: vec![
                "Plugins are disabled by default.".to_string(),
                "No plugin code is loaded by Monad.".to_string(),
                "No plugin binaries are executed by Monad.".to_string(),
                "No dynamic libraries are opened by Monad.".to_string(),
                "No remote plugin registry is contacted.".to_string(),
                "Generated plugin-system evidence is written only when --yes is used.".to_string(),
            ],
        }
    }

    /// Extension points.
    #[must_use]
    pub fn extension_points(&self) -> &[ExtensionPoint] {
        &self.extension_points
    }

    /// Plugin records.
    #[must_use]
    pub fn plugins(&self) -> &[PluginManifestRecord] {
        &self.plugins
    }

    /// Safety findings.
    #[must_use]
    pub fn safety_findings(&self) -> &[PluginSafetyFinding] {
        &self.safety_findings
    }

    /// Loading plan entries.
    #[must_use]
    pub fn loading_plan(&self) -> &[PluginLoadingPlanEntry] {
        &self.loading_plan
    }

    /// Evidence output paths.
    #[must_use]
    pub fn evidence_paths(&self) -> &[PathBuf] {
        &self.evidence_paths
    }

    /// Safety notes.
    #[must_use]
    pub fn safety_notes(&self) -> &[String] {
        &self.safety_notes
    }
}

/// Apply result for generated plugin-system evidence writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginSystemApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl PluginSystemApplyResult {
    /// Creates an apply result.
    #[must_use]
    pub fn new(write_results: Vec<GatedWriteResult>) -> Self {
        Self { write_results }
    }

    /// Generated evidence write results.
    #[must_use]
    pub fn write_results(&self) -> &[GatedWriteResult] {
        &self.write_results
    }
}

/// Builds the local plugin-system plan.
#[must_use]
pub fn build_plugin_system_plan(root: impl AsRef<Path>) -> PluginSystemPlan {
    let root = root.as_ref();
    let extension_points = default_extension_points();
    let plugins = discover_plugin_manifests(root);
    let safety_findings = evaluate_plugin_safety(&plugins, &extension_points);
    let loading_plan = plan_plugin_loading(&plugins, &safety_findings);

    PluginSystemPlan::new(extension_points, plugins, safety_findings, loading_plan)
}

/// Built-in extension point registry.
#[must_use]
pub fn default_extension_points() -> Vec<ExtensionPoint> {
    vec![
        ExtensionPoint::new(
            "adapter.language",
            "Language adapter extension point for future supervised language integrations.",
            true,
        ),
        ExtensionPoint::new(
            "report.writer",
            "Generated report writer extension point for future supervised report formats.",
            true,
        ),
        ExtensionPoint::new(
            "policy.check",
            "Policy check extension point for future supervised local policy checks.",
            true,
        ),
        ExtensionPoint::new(
            "template.provider",
            "Template provider extension point for future local template sources.",
            true,
        ),
    ]
}

/// Discovers local plugin manifests.
#[must_use]
pub fn discover_plugin_manifests(root: &Path) -> Vec<PluginManifestRecord> {
    let mut plugins = discover_metadata_files(root, Path::new("plugins"))
        .into_iter()
        .map(|path| parse_plugin_manifest(root, path))
        .collect::<Vec<_>>();

    if plugins.is_empty() {
        plugins.push(PluginManifestRecord::new(
            "plugin:example-disabled",
            "plugins/example/plugin.toml",
            "example-disabled",
            "0.1.0",
            vec!["report.writer".to_string()],
            PluginTrustLevel::Untrusted,
            false,
        ));
    }

    plugins.sort_by(|left, right| left.id().cmp(right.id()));
    plugins
}

/// Evaluates disabled-by-default plugin safety checks.
#[must_use]
pub fn evaluate_plugin_safety(
    plugins: &[PluginManifestRecord],
    extension_points: &[ExtensionPoint],
) -> Vec<PluginSafetyFinding> {
    let mut findings = Vec::new();

    for plugin in plugins {
        if plugin.enabled() {
            findings.push(PluginSafetyFinding::new(
                plugin.id().to_string(),
                "high",
                "plugin manifest requests enabled=true, but plugins are disabled by default",
            ));
        }

        if plugin.trust_level() == PluginTrustLevel::Untrusted {
            findings.push(PluginSafetyFinding::new(
                plugin.id().to_string(),
                "medium",
                "plugin is untrusted and requires explicit human review",
            ));
        }

        for extension_point in plugin.extension_points() {
            if !extension_points
                .iter()
                .any(|registered| registered.id() == extension_point)
            {
                findings.push(PluginSafetyFinding::new(
                    plugin.id().to_string(),
                    "medium",
                    format!("unknown extension point `{extension_point}`"),
                ));
            }
        }
    }

    findings
}

/// Plans plugin loading without loading any plugin.
#[must_use]
pub fn plan_plugin_loading(
    plugins: &[PluginManifestRecord],
    safety_findings: &[PluginSafetyFinding],
) -> Vec<PluginLoadingPlanEntry> {
    plugins
        .iter()
        .map(|plugin| {
            let has_high = safety_findings
                .iter()
                .any(|finding| finding.plugin_id() == plugin.id() && finding.severity() == "high");
            let has_medium = safety_findings.iter().any(|finding| {
                finding.plugin_id() == plugin.id() && finding.severity() == "medium"
            });

            if has_high {
                PluginLoadingPlanEntry::new(
                    plugin.id().to_string(),
                    PluginLoadDecision::Blocked,
                    "high-severity safety finding blocks loading",
                )
            } else if has_medium || plugin.trust_level() != PluginTrustLevel::Trusted {
                PluginLoadingPlanEntry::new(
                    plugin.id().to_string(),
                    PluginLoadDecision::Review,
                    "plugin requires human review before it can become a supervised candidate",
                )
            } else {
                PluginLoadingPlanEntry::new(
                    plugin.id().to_string(),
                    PluginLoadDecision::Candidate,
                    "trusted plugin may become a supervised future loading candidate",
                )
            }
        })
        .collect()
}

/// Writes generated plugin-system evidence reports.
pub fn write_plugin_system_evidence(
    root: impl AsRef<Path>,
) -> Result<PluginSystemApplyResult, String> {
    let root = root.as_ref();
    let plan = build_plugin_system_plan(root);
    let markdown = render_plugin_system_plan(&plan);
    let json = render_plugin_system_plan_json(&plan);

    let requests = [
        GatedWriteRequest::new(".monad/reports/plugin-system-plan.md", markdown, true),
        GatedWriteRequest::new(".monad/reports/plugin-system-plan.json", json, true),
    ];

    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(PluginSystemApplyResult::new(write_results))
}

/// Renders a text plugin-system plan.
#[must_use]
pub fn render_plugin_system_plan(plan: &PluginSystemPlan) -> String {
    let mut lines = vec![
        "Monad plugin and extension system plan".to_string(),
        String::new(),
        format!("Extension points: {}", plan.extension_points().len()),
    ];

    for extension_point in plan.extension_points() {
        lines.push(format!(
            "  - {} disabled_by_default={} description={}",
            extension_point.id(),
            extension_point.disabled_by_default(),
            extension_point.description()
        ));
    }

    lines.push(String::new());
    lines.push(format!("Plugins: {}", plan.plugins().len()));
    for plugin in plan.plugins() {
        lines.push(format!(
            "  - {} name={} version={} trust={} enabled={} path={}",
            plugin.id(),
            plugin.name(),
            plugin.version(),
            plugin.trust_level().as_str(),
            plugin.enabled(),
            plugin.path().display()
        ));
        lines.push(format!(
            "    extension_points: {}",
            plugin.extension_points().join(", ")
        ));
    }

    lines.push(String::new());
    lines.push(format!("Safety findings: {}", plan.safety_findings().len()));
    if plan.safety_findings().is_empty() {
        lines.push("  - no plugin safety findings".to_string());
    } else {
        for finding in plan.safety_findings() {
            lines.push(format!(
                "  - {} severity={} message={}",
                finding.plugin_id(),
                finding.severity(),
                finding.message()
            ));
        }
    }

    lines.push(String::new());
    lines.push("Loading plan:".to_string());
    for entry in plan.loading_plan() {
        lines.push(format!(
            "  - {} decision={} reason={}",
            entry.plugin_id(),
            entry.decision().as_str(),
            entry.reason()
        ));
    }

    lines.push(String::new());
    lines.push("Evidence outputs:".to_string());
    for path in plan.evidence_paths() {
        lines.push(format!("  - {}", path.display()));
    }

    lines.push(String::new());
    lines.push("Safety notes:".to_string());
    for note in plan.safety_notes() {
        lines.push(format!("  - {note}"));
    }

    lines.join("\n")
}

/// Renders a JSON plugin-system plan.
#[must_use]
pub fn render_plugin_system_plan_json(plan: &PluginSystemPlan) -> String {
    serde_json::to_string_pretty(plan).unwrap_or_else(|_| {
        "{\n  \"command\": \"plugin-plan\",\n  \"error\": \"plugin system plan serialization failed\"\n}".to_string()
    })
}

/// Renders generated evidence write results.
#[must_use]
pub fn render_plugin_system_apply_result(result: &PluginSystemApplyResult) -> String {
    let mut lines = vec![
        "Monad plugin-system evidence write result".to_string(),
        String::new(),
        "Results:".to_string(),
    ];

    for write_result in result.write_results() {
        match write_result {
            GatedWriteResult::Written(path) | GatedWriteResult::SkippedIdentical(path) => {
                lines.push(format!(
                    "  - [{}] {}",
                    write_result.as_str(),
                    path.display()
                ));
            }
            GatedWriteResult::ApprovalRequired(message) | GatedWriteResult::Blocked(message) => {
                lines.push(format!("  - [{}] {message}", write_result.as_str()));
            }
        }
    }

    lines.push(String::new());
    lines.push("No plugin code was loaded.".to_string());
    lines.push("No plugin binaries were executed.".to_string());
    lines.push("No remote plugin registry was contacted.".to_string());

    lines.join("\n")
}

fn parse_plugin_manifest(root: &Path, path: PathBuf) -> PluginManifestRecord {
    let absolute = root.join(&path);
    let text = fs::read_to_string(&absolute).unwrap_or_default();

    let name = metadata_value(&text, "name").unwrap_or_else(|| fallback_name(&path));
    let id = metadata_value(&text, "id").unwrap_or_else(|| make_plugin_id(&name));
    let version = metadata_value(&text, "version").unwrap_or_else(|| "0.0.0".to_string());
    let trust_level = match metadata_value(&text, "trust").as_deref() {
        Some("trusted") => PluginTrustLevel::Trusted,
        Some("review") => PluginTrustLevel::Review,
        _ => PluginTrustLevel::Untrusted,
    };
    let enabled = matches!(metadata_value(&text, "enabled").as_deref(), Some("true"));
    let extension_points = metadata_list(&text, "extension_points");

    PluginManifestRecord::new(
        id,
        path,
        name,
        version,
        extension_points,
        trust_level,
        enabled,
    )
}

fn discover_metadata_files(root: &Path, relative_dir: &Path) -> Vec<PathBuf> {
    let start = root.join(relative_dir);
    let mut files = Vec::new();

    collect_metadata_files(root, &start, &mut files);
    files.sort();
    files
}

fn collect_metadata_files(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            collect_metadata_files(root, &path, files);
        } else if matches!(
            path.file_name().and_then(|value| value.to_str()),
            Some("plugin.toml" | "plugin.json" | "metadata.toml" | "metadata.json")
        ) {
            files.push(
                path.strip_prefix(root)
                    .map_or(path.clone(), Path::to_path_buf),
            );
        }
    }
}

fn metadata_value(text: &str, key: &str) -> Option<String> {
    for line in text.lines().map(str::trim) {
        let Some((left, right)) = line.split_once('=') else {
            continue;
        };

        if left.trim() == key {
            return Some(
                right
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string(),
            );
        }
    }

    None
}

fn metadata_list(text: &str, key: &str) -> Vec<String> {
    let Some(value) = metadata_value(text, key) else {
        return Vec::new();
    };

    value
        .trim_matches('[')
        .trim_matches(']')
        .split(',')
        .map(str::trim)
        .map(|item| item.trim_matches('"').trim_matches('\'').to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

fn fallback_name(path: &Path) -> String {
    path.parent().and_then(Path::file_name).map_or_else(
        || "unnamed-plugin".to_string(),
        |name| name.to_string_lossy().to_string(),
    )
}

fn make_plugin_id(name: &str) -> String {
    let slug = name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    format!("plugin:{slug}")
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());

        std::env::temp_dir().join(format!(
            "monad-plugin-system-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);
        assert!(fs::create_dir_all(root.join("plugins/review-plugin")).is_ok());
        assert!(
            fs::write(
                root.join("plugins/review-plugin/plugin.toml"),
                "id = \"plugin:review\"\nname = \"review\"\nversion = \"0.1.0\"\ntrust = \"review\"\nenabled = false\nextension_points = [\"report.writer\"]\n",
            )
            .is_ok()
        );
        root
    }

    #[test]
    fn default_extension_points_are_disabled_by_default() {
        let points = default_extension_points();

        assert!(points.iter().any(|point| point.id() == "report.writer"));
        assert!(points.iter().all(ExtensionPoint::disabled_by_default));
    }

    #[test]
    fn plugin_manifests_are_discovered_locally() {
        let root = create_workspace("discover");
        let plugins = discover_plugin_manifests(&root);

        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].id(), "plugin:review");
        assert_eq!(plugins[0].trust_level(), PluginTrustLevel::Review);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn enabled_plugin_is_blocked_by_safety_check() {
        let plugin = PluginManifestRecord::new(
            "plugin:enabled",
            "plugins/enabled/plugin.toml",
            "enabled",
            "0.1.0",
            vec!["report.writer".to_string()],
            PluginTrustLevel::Trusted,
            true,
        );
        let findings = evaluate_plugin_safety(&[plugin], &default_extension_points());

        assert!(findings.iter().any(|finding| finding.severity() == "high"));
    }

    #[test]
    fn untrusted_plugin_requires_review() {
        let plan = build_plugin_system_plan(unique_temp_dir("fallback"));

        assert!(
            plan.loading_plan()
                .iter()
                .any(|entry| entry.decision() == PluginLoadDecision::Review)
        );
        assert!(
            plan.safety_findings()
                .iter()
                .any(|finding| finding.severity() == "medium")
        );
    }

    #[test]
    fn text_render_mentions_disabled_by_default_safety() {
        let root = create_workspace("text");
        let plan = build_plugin_system_plan(&root);
        let text = render_plugin_system_plan(&plan);

        assert!(text.contains("Monad plugin and extension system plan"));
        assert!(text.contains("Plugins are disabled by default"));
        assert!(text.contains("No plugin code is loaded by Monad"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn json_render_contains_plugin_plan_command() {
        let root = create_workspace("json");
        let plan = build_plugin_system_plan(&root);
        let json = render_plugin_system_plan_json(&plan);

        assert!(json.contains("\"command\": \"plugin-plan\""));
        assert!(json.contains("plugin:review"));

        let _ = fs::remove_dir_all(root);
    }
}
