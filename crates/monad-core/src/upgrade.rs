//! Repository upgrade planning and guarded evolution.
//!
//! E17 introduces the first MVP-safe `monad upgrade` foundation. Upgrade can
//! inspect repository contract state, preview deterministic steps, and apply
//! only guarded non-destructive metadata/evidence writes.

use std::fs;
use std::path::{Path, PathBuf};

/// The current supported repository contract version for E17.
pub const SUPPORTED_REPOSITORY_CONTRACT_VERSION: &str = "1";

/// Upgrade plan status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpgradeStatus {
    /// Current repo contract version is unknown.
    Unknown,

    /// Repository is missing a Monad manifest.
    MissingManifest,

    /// Repository is older than the supported target.
    UpgradeNeeded,

    /// Repository already matches the supported target.
    UpToDate,

    /// Repository declares a future/unsupported contract version.
    UnsupportedFuture,
}

impl UpgradeStatus {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::MissingManifest => "missing-manifest",
            Self::UpgradeNeeded => "upgrade-needed",
            Self::UpToDate => "up-to-date",
            Self::UnsupportedFuture => "unsupported-future",
        }
    }

    /// Whether this status blocks guarded apply.
    #[must_use]
    pub const fn blocks_apply(self) -> bool {
        matches!(
            self,
            Self::MissingManifest | Self::Unknown | Self::UnsupportedFuture
        )
    }
}

/// Upgrade step safety class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpgradeStepSafety {
    /// Writes only generated metadata/evidence and does not touch user source.
    GeneratedMetadata,

    /// No file write is needed.
    NoOp,

    /// Step is known but intentionally not automated.
    Manual,
}

impl UpgradeStepSafety {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeneratedMetadata => "generated-metadata",
            Self::NoOp => "no-op",
            Self::Manual => "manual",
        }
    }

    /// Whether this safety class may be applied automatically.
    #[must_use]
    pub const fn is_auto_applicable(self) -> bool {
        matches!(self, Self::GeneratedMetadata | Self::NoOp)
    }
}

/// One upgrade step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpgradeStep {
    id: String,
    order: u32,
    title: String,
    description: String,
    safety: UpgradeStepSafety,
    target_path: Option<PathBuf>,
    content: Option<String>,
    applicable: bool,
}

impl UpgradeStep {
    /// Creates a generated metadata step.
    #[must_use]
    pub fn generated_metadata(
        id: impl Into<String>,
        order: u32,
        title: impl Into<String>,
        description: impl Into<String>,
        target_path: impl Into<PathBuf>,
        content: impl Into<String>,
        applicable: bool,
    ) -> Self {
        Self {
            id: id.into(),
            order,
            title: title.into(),
            description: description.into(),
            safety: UpgradeStepSafety::GeneratedMetadata,
            target_path: Some(target_path.into()),
            content: Some(content.into()),
            applicable,
        }
    }

    /// Creates a no-op step.
    #[must_use]
    pub fn no_op(
        id: impl Into<String>,
        order: u32,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            order,
            title: title.into(),
            description: description.into(),
            safety: UpgradeStepSafety::NoOp,
            target_path: None,
            content: None,
            applicable: false,
        }
    }

    /// Stable step ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Deterministic order.
    #[must_use]
    pub const fn order(&self) -> u32 {
        self.order
    }

    /// Step title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Step description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Step safety class.
    #[must_use]
    pub const fn safety(&self) -> UpgradeStepSafety {
        self.safety
    }

    /// Optional target path.
    #[must_use]
    pub fn target_path(&self) -> Option<&Path> {
        self.target_path.as_deref()
    }

    /// Optional generated content.
    #[must_use]
    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    /// Whether the step is applicable to the current repository.
    #[must_use]
    pub const fn applicable(&self) -> bool {
        self.applicable
    }
}

/// Upgrade plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpgradePlan {
    current_version: Option<String>,
    target_version: String,
    status: UpgradeStatus,
    steps: Vec<UpgradeStep>,
    blockers: Vec<String>,
}

impl UpgradePlan {
    /// Creates a deterministic upgrade plan.
    #[must_use]
    pub fn new(
        current_version: Option<String>,
        target_version: String,
        status: UpgradeStatus,
        mut steps: Vec<UpgradeStep>,
        blockers: Vec<String>,
    ) -> Self {
        steps.sort_by(|left, right| {
            left.order()
                .cmp(&right.order())
                .then(left.id().cmp(right.id()))
        });

        Self {
            current_version,
            target_version,
            status,
            steps,
            blockers,
        }
    }

    /// Current repo version, if detected.
    #[must_use]
    pub fn current_version(&self) -> Option<&str> {
        self.current_version.as_deref()
    }

    /// Target supported version.
    #[must_use]
    pub fn target_version(&self) -> &str {
        &self.target_version
    }

    /// Plan status.
    #[must_use]
    pub const fn status(&self) -> UpgradeStatus {
        self.status
    }

    /// Planned steps.
    #[must_use]
    pub fn steps(&self) -> &[UpgradeStep] {
        &self.steps
    }

    /// Blockers.
    #[must_use]
    pub fn blockers(&self) -> &[String] {
        &self.blockers
    }

    /// Applicable step count.
    #[must_use]
    pub fn applicable_step_count(&self) -> usize {
        self.steps.iter().filter(|step| step.applicable()).count()
    }

    /// Whether guarded apply is allowed.
    #[must_use]
    pub fn can_apply(&self) -> bool {
        !self.status().blocks_apply() && self.blockers.is_empty()
    }
}

/// Result of applying a guarded upgrade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpgradeApplyResult {
    plan: UpgradePlan,
    written_paths: Vec<PathBuf>,
    skipped_steps: Vec<String>,
    conflicts: Vec<String>,
}

impl UpgradeApplyResult {
    /// Creates an apply result.
    #[must_use]
    pub fn new(
        plan: UpgradePlan,
        mut written_paths: Vec<PathBuf>,
        mut skipped_steps: Vec<String>,
        mut conflicts: Vec<String>,
    ) -> Self {
        written_paths.sort();
        skipped_steps.sort();
        conflicts.sort();

        Self {
            plan,
            written_paths,
            skipped_steps,
            conflicts,
        }
    }

    /// Original plan.
    #[must_use]
    pub const fn plan(&self) -> &UpgradePlan {
        &self.plan
    }

    /// Written paths.
    #[must_use]
    pub fn written_paths(&self) -> &[PathBuf] {
        &self.written_paths
    }

    /// Skipped step IDs.
    #[must_use]
    pub fn skipped_steps(&self) -> &[String] {
        &self.skipped_steps
    }

    /// Conflicts.
    #[must_use]
    pub fn conflicts(&self) -> &[String] {
        &self.conflicts
    }

    /// Whether apply encountered conflicts.
    #[must_use]
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}

/// Builds an upgrade plan for a root directory.
#[must_use]
pub fn build_upgrade_plan(root: impl AsRef<Path>) -> UpgradePlan {
    let root = root.as_ref();
    let current_version = detect_current_contract_version(root);
    let target_version = SUPPORTED_REPOSITORY_CONTRACT_VERSION.to_string();
    let status = classify_upgrade_status(current_version.as_deref(), &target_version);
    let mut blockers = Vec::new();

    if status.blocks_apply() {
        blockers.push(match status {
            UpgradeStatus::MissingManifest => {
                "monad.toml is missing; initialize the repository before upgrade apply".to_string()
            }
            UpgradeStatus::Unknown => {
                "current repository contract version could not be detected".to_string()
            }
            UpgradeStatus::UnsupportedFuture => {
                "repository declares a future contract version this binary cannot upgrade"
                    .to_string()
            }
            UpgradeStatus::UpgradeNeeded | UpgradeStatus::UpToDate => String::new(),
        });
        blockers.retain(|blocker| !blocker.is_empty());
    }

    let steps = upgrade_step_registry(root, current_version.as_deref(), &target_version, status);

    UpgradePlan::new(current_version, target_version, status, steps, blockers)
}

fn classify_upgrade_status(current: Option<&str>, target: &str) -> UpgradeStatus {
    let Some(current) = current else {
        return UpgradeStatus::MissingManifest;
    };

    let Some(current_number) = current.parse::<u32>().ok() else {
        return UpgradeStatus::Unknown;
    };

    let Some(target_number) = target.parse::<u32>().ok() else {
        return UpgradeStatus::Unknown;
    };

    if current_number < target_number {
        UpgradeStatus::UpgradeNeeded
    } else if current_number == target_number {
        UpgradeStatus::UpToDate
    } else {
        UpgradeStatus::UnsupportedFuture
    }
}

fn detect_current_contract_version(root: &Path) -> Option<String> {
    let monad_toml = root.join("monad.toml");

    if let Ok(text) = fs::read_to_string(&monad_toml) {
        if let Some(version) = parse_contract_version(&text) {
            return Some(version);
        }
    } else {
        return None;
    }

    let fallback = root.join(".monad/upgrade/contract-version");
    fs::read_to_string(fallback)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn parse_contract_version(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();

        for key in [
            "schema_version",
            "contract_version",
            "repository_contract_version",
        ] {
            let Some(value) = trimmed.strip_prefix(key) else {
                continue;
            };
            let value = value.trim().strip_prefix('=')?.trim();
            let value = value.trim_matches('"');

            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    None
}

fn upgrade_step_registry(
    _root: &Path,
    current_version: Option<&str>,
    target_version: &str,
    status: UpgradeStatus,
) -> Vec<UpgradeStep> {
    let metadata_applicable = !status.blocks_apply();

    vec![
        UpgradeStep::generated_metadata(
            "upgrade.metadata.contract-version",
            10,
            "Record supported repository contract version",
            "Writes generated upgrade metadata recording the supported contract version.",
            ".monad/upgrade/contract-version",
            format!("{target_version}\n"),
            metadata_applicable,
        ),
        UpgradeStep::generated_metadata(
            "upgrade.metadata.readme",
            20,
            "Record upgrade safety boundary",
            "Writes generated upgrade metadata documenting the non-destructive upgrade boundary.",
            ".monad/upgrade/README.md",
            upgrade_readme_content(current_version, target_version),
            metadata_applicable,
        ),
        UpgradeStep::generated_metadata(
            "upgrade.metadata.evidence-placeholder",
            30,
            "Prepare upgrade evidence directory",
            "Creates generated upgrade evidence output during guarded apply.",
            ".monad/reports/upgrade-report.md",
            String::new(),
            metadata_applicable,
        ),
        UpgradeStep::no_op(
            "upgrade.noop.source-code-rewrites",
            90,
            "Source-code rewrites are intentionally not automated",
            "E17 does not rewrite user source code or run third-party migrations.",
        ),
    ]
}

fn upgrade_readme_content(current_version: Option<&str>, target_version: &str) -> String {
    format!(
        "# Monad Upgrade Metadata\n\nCurrent detected contract version: {}\nTarget supported contract version: {target_version}\n\nThis directory contains generated upgrade metadata.\n\nSafety boundary:\n\n- no destructive migrations;\n- no automatic source-code rewrites;\n- no remote/cloud upgrades;\n- no dependency version upgrades;\n- no arbitrary third-party migration execution.\n",
        current_version.unwrap_or("unknown")
    )
}

/// Applies an upgrade plan using guarded non-destructive writes.
pub fn apply_upgrade_plan(root: impl AsRef<Path>) -> Result<UpgradeApplyResult, String> {
    let root = root.as_ref();
    let plan = build_upgrade_plan(root);

    if !plan.can_apply() {
        return Err(format!(
            "upgrade cannot be applied: status={}, blockers={}",
            plan.status().as_str(),
            plan.blockers().join("; ")
        ));
    }

    let mut written_paths = Vec::new();
    let mut skipped_steps = Vec::new();
    let mut conflicts = Vec::new();

    for step in plan.steps() {
        if !step.applicable() {
            skipped_steps.push(step.id().to_string());
            continue;
        }

        if !step.safety().is_auto_applicable() {
            skipped_steps.push(step.id().to_string());
            continue;
        }

        let Some(relative_path) = step.target_path() else {
            skipped_steps.push(step.id().to_string());
            continue;
        };

        if step.id() == "upgrade.metadata.evidence-placeholder" {
            continue;
        }

        let Some(content) = step.content() else {
            skipped_steps.push(step.id().to_string());
            continue;
        };

        let absolute_path = root.join(relative_path);

        match guarded_write(&absolute_path, content) {
            Ok(written) => {
                if written {
                    written_paths.push(relative_path.to_path_buf());
                } else {
                    skipped_steps.push(step.id().to_string());
                }
            }
            Err(error) => conflicts.push(format!("{}: {error}", relative_path.display())),
        }
    }

    let mut result = UpgradeApplyResult::new(plan, written_paths, skipped_steps, conflicts);

    if result.has_conflicts() {
        return Ok(result);
    }

    write_upgrade_evidence(root, &result)?;
    result
        .written_paths
        .push(PathBuf::from(".monad/reports/upgrade-report.md"));
    result
        .written_paths
        .push(PathBuf::from(".monad/reports/upgrade-report.json"));
    result.written_paths.sort();

    Ok(result)
}

fn guarded_write(path: &Path, content: &str) -> Result<bool, String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    if path.exists() {
        let existing = fs::read_to_string(path).map_err(|error| error.to_string())?;
        if existing == content {
            return Ok(false);
        }

        return Err(
            "refusing unsafe overwrite of existing file with different content".to_string(),
        );
    }

    fs::write(path, content).map_err(|error| error.to_string())?;
    Ok(true)
}

fn write_upgrade_evidence(root: &Path, result: &UpgradeApplyResult) -> Result<(), String> {
    let reports = root.join(".monad/reports");
    fs::create_dir_all(&reports).map_err(|error| error.to_string())?;

    fs::write(
        reports.join("upgrade-report.md"),
        render_upgrade_evidence_markdown(result),
    )
    .map_err(|error| error.to_string())?;

    fs::write(
        reports.join("upgrade-report.json"),
        render_upgrade_apply_result_json(result),
    )
    .map_err(|error| error.to_string())?;

    Ok(())
}

/// Renders a human-readable upgrade plan.
#[must_use]
pub fn render_upgrade_plan(plan: &UpgradePlan) -> String {
    let mut lines = vec![
        "Monad upgrade dry-run plan".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!(
            "  current_version: {}",
            plan.current_version().unwrap_or("unknown")
        ),
        format!("  target_version: {}", plan.target_version()),
        format!("  status: {}", plan.status().as_str()),
        format!("  applicable_steps: {}", plan.applicable_step_count()),
        format!("  blockers: {}", plan.blockers().len()),
        String::new(),
        "Steps:".to_string(),
    ];

    for step in plan.steps() {
        lines.push(format!(
            "  - [{}] {} {}",
            step.safety().as_str(),
            step.order(),
            step.title()
        ));
        lines.push(format!("    id: {}", step.id()));
        lines.push(format!("    applicable: {}", step.applicable()));
        lines.push(format!("    description: {}", step.description()));
        if let Some(path) = step.target_path() {
            lines.push(format!("    target: {}", path.display()));
        }
    }

    if !plan.blockers().is_empty() {
        lines.push(String::new());
        lines.push("Blockers:".to_string());
        for blocker in plan.blockers() {
            lines.push(format!("  - {blocker}"));
        }
    }

    lines.push(String::new());
    lines.push("No files were written.".to_string());
    lines.push("No source code was rewritten.".to_string());
    lines.push("No dependency versions were changed.".to_string());
    lines.push("No remote upgrades were performed.".to_string());

    lines.join("\n")
}

/// Renders upgrade plan JSON.
#[must_use]
pub fn render_upgrade_plan_json(plan: &UpgradePlan) -> String {
    let steps = plan
        .steps()
        .iter()
        .map(|step| {
            let target = step
                .target_path()
                .map(|path| format!("\"{}\"", json_escape(&path.display().to_string())))
                .unwrap_or_else(|| "null".to_string());

            format!(
                "{{\"id\":\"{}\",\"order\":{},\"title\":\"{}\",\"safety\":\"{}\",\"applicable\":{},\"target\":{}}}",
                json_escape(step.id()),
                step.order(),
                json_escape(step.title()),
                step.safety().as_str(),
                step.applicable(),
                target
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let blockers = plan
        .blockers()
        .iter()
        .map(|blocker| format!("\"{}\"", json_escape(blocker)))
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"upgrade\",\"mode\":\"dry-run\",\"current_version\":{},\"target_version\":\"{}\",\"status\":\"{}\",\"applicable_steps\":{},\"blockers\":[{}],\"steps\":[{}]}}",
        plan.current_version()
            .map(|version| format!("\"{}\"", json_escape(version)))
            .unwrap_or_else(|| "null".to_string()),
        json_escape(plan.target_version()),
        plan.status().as_str(),
        plan.applicable_step_count(),
        blockers,
        steps
    )
}

/// Renders guarded apply summary.
#[must_use]
pub fn render_upgrade_apply_result(result: &UpgradeApplyResult) -> String {
    let mut lines = vec![
        "Monad upgrade apply result".to_string(),
        String::new(),
        format!("status: {}", result.plan().status().as_str()),
        format!("target_version: {}", result.plan().target_version()),
        format!("written_paths: {}", result.written_paths().len()),
        format!("skipped_steps: {}", result.skipped_steps().len()),
        format!("conflicts: {}", result.conflicts().len()),
        String::new(),
        "Written paths:".to_string(),
    ];

    for path in result.written_paths() {
        lines.push(format!("  - {}", path.display()));
    }

    if !result.conflicts().is_empty() {
        lines.push(String::new());
        lines.push("Conflicts:".to_string());
        for conflict in result.conflicts() {
            lines.push(format!("  - {conflict}"));
        }
    }

    lines.push(String::new());
    lines.push("No source code was rewritten.".to_string());
    lines.push("No dependency versions were changed.".to_string());
    lines.push("No remote upgrades were performed.".to_string());

    lines.join("\n")
}

/// Renders upgrade apply JSON.
#[must_use]
pub fn render_upgrade_apply_result_json(result: &UpgradeApplyResult) -> String {
    let written = result
        .written_paths()
        .iter()
        .map(|path| format!("\"{}\"", json_escape(&path.display().to_string())))
        .collect::<Vec<_>>()
        .join(",");

    let skipped = result
        .skipped_steps()
        .iter()
        .map(|step| format!("\"{}\"", json_escape(step)))
        .collect::<Vec<_>>()
        .join(",");

    let conflicts = result
        .conflicts()
        .iter()
        .map(|conflict| format!("\"{}\"", json_escape(conflict)))
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"upgrade\",\"mode\":\"apply\",\"status\":\"{}\",\"target_version\":\"{}\",\"written_paths\":[{}],\"skipped_steps\":[{}],\"conflicts\":[{}]}}",
        result.plan().status().as_str(),
        json_escape(result.plan().target_version()),
        written,
        skipped,
        conflicts
    )
}

/// Renders upgrade evidence markdown.
#[must_use]
pub fn render_upgrade_evidence_markdown(result: &UpgradeApplyResult) -> String {
    format!(
        "# Monad Upgrade Evidence Report\n\n{}\n\n## Safety\n\n- No source code was rewritten.\n- No dependency versions were changed.\n- No remote upgrades were performed.\n- No arbitrary third-party migrations were executed.\n",
        render_upgrade_apply_result(result)
    )
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
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn unique_temp_root(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        std::env::temp_dir().join(format!("monad-upgrade-{name}-{unique}"))
    }

    #[test]
    fn upgrade_plan_reports_missing_manifest() {
        let root = unique_temp_root("missing-manifest");
        fs::create_dir_all(&root).expect("temp root should be created");

        let plan = build_upgrade_plan(&root);

        assert_eq!(plan.status(), UpgradeStatus::MissingManifest);
        assert!(!plan.can_apply());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn upgrade_plan_reports_up_to_date_manifest() {
        let root = unique_temp_root("up-to-date");
        fs::create_dir_all(&root).expect("temp root should be created");
        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("manifest should be written");

        let plan = build_upgrade_plan(&root);

        assert_eq!(plan.status(), UpgradeStatus::UpToDate);
        assert!(plan.can_apply());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn upgrade_apply_writes_only_generated_metadata_and_evidence() {
        let root = unique_temp_root("apply");
        fs::create_dir_all(&root).expect("temp root should be created");
        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("manifest should be written");

        let result = apply_upgrade_plan(&root).expect("upgrade should apply");

        assert!(!result.has_conflicts());
        assert!(root.join(".monad/upgrade/contract-version").is_file());
        assert!(root.join(".monad/reports/upgrade-report.md").is_file());
        assert!(root.join(".monad/reports/upgrade-report.json").is_file());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn upgrade_apply_refuses_unsafe_overwrite() {
        let root = unique_temp_root("conflict");
        fs::create_dir_all(root.join(".monad/upgrade")).expect("upgrade dir should be created");
        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("manifest should be written");
        fs::write(
            root.join(".monad/upgrade/README.md"),
            "user-owned content\n",
        )
        .expect("conflicting readme should be written");

        let result = apply_upgrade_plan(&root).expect("upgrade should return conflict result");

        assert!(result.has_conflicts());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn upgrade_plan_json_contains_core_fields() {
        let root = unique_temp_root("json");
        fs::create_dir_all(&root).expect("temp root should be created");
        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("manifest should be written");

        let plan = build_upgrade_plan(&root);
        let output = render_upgrade_plan_json(&plan);

        assert!(output.contains("\"command\":\"upgrade\""));
        assert!(output.contains("\"mode\":\"dry-run\""));
        assert!(output.contains("\"target_version\":\"1\""));

        fs::remove_dir_all(root).ok();
    }
}
