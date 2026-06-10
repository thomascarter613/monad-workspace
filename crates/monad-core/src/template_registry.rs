//! Template registry and preset evolution foundation.
//!
//! Local-only metadata discovery, compatibility validation, and preset upgrade
//! planning. No remote registry access, template rendering, preset application,
//! package installation, or user-owned source rewrites are performed.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TemplateCompatibility {
    Compatible,
    Review,
    Incompatible,
}

impl TemplateCompatibility {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::Review => "review",
            Self::Incompatible => "incompatible",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PresetUpgradeAction {
    Keep,
    PlanUpgrade,
    Review,
}

impl PresetUpgradeAction {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keep => "keep",
            Self::PlanUpgrade => "plan-upgrade",
            Self::Review => "review",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TemplateMetadata {
    id: String,
    path: PathBuf,
    name: String,
    version: String,
    min_schema_version: u16,
    compatibility: TemplateCompatibility,
    reason: String,
}

impl TemplateMetadata {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        path: impl Into<PathBuf>,
        name: impl Into<String>,
        version: impl Into<String>,
        min_schema_version: u16,
        compatibility: TemplateCompatibility,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            path: path.into(),
            name: name.into(),
            version: version.into(),
            min_schema_version,
            compatibility,
            reason: reason.into(),
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
    #[must_use]
    pub const fn min_schema_version(&self) -> u16 {
        self.min_schema_version
    }
    #[must_use]
    pub const fn compatibility(&self) -> TemplateCompatibility {
        self.compatibility
    }
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PresetMetadata {
    id: String,
    path: PathBuf,
    name: String,
    version: String,
    target_template: String,
    compatibility: TemplateCompatibility,
    upgrade_action: PresetUpgradeAction,
    reason: String,
}

impl PresetMetadata {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        path: impl Into<PathBuf>,
        name: impl Into<String>,
        version: impl Into<String>,
        target_template: impl Into<String>,
        compatibility: TemplateCompatibility,
        upgrade_action: PresetUpgradeAction,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            path: path.into(),
            name: name.into(),
            version: version.into(),
            target_template: target_template.into(),
            compatibility,
            upgrade_action,
            reason: reason.into(),
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
    #[must_use]
    pub fn target_template(&self) -> &str {
        &self.target_template
    }
    #[must_use]
    pub const fn compatibility(&self) -> TemplateCompatibility {
        self.compatibility
    }
    #[must_use]
    pub const fn upgrade_action(&self) -> PresetUpgradeAction {
        self.upgrade_action
    }
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TemplateRegistryContract {
    schema_version: u16,
    templates_dir: PathBuf,
    presets_dir: PathBuf,
    index_path: PathBuf,
    safety_rules: Vec<String>,
}

impl TemplateRegistryContract {
    #[must_use]
    pub fn default_contract() -> Self {
        Self {
            schema_version: 1,
            templates_dir: PathBuf::from("templates"),
            presets_dir: PathBuf::from("presets"),
            index_path: PathBuf::from(".monad/reports/template-registry-index.json"),
            safety_rules: vec![
                "Templates are discovered locally only.".to_string(),
                "Templates are not rendered by this foundation.".to_string(),
                "Presets are not applied by this foundation.".to_string(),
                "Preset upgrades are planned only; they are not performed.".to_string(),
            ],
        }
    }

    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }
    #[must_use]
    pub fn templates_dir(&self) -> &Path {
        &self.templates_dir
    }
    #[must_use]
    pub fn presets_dir(&self) -> &Path {
        &self.presets_dir
    }
    #[must_use]
    pub fn index_path(&self) -> &Path {
        &self.index_path
    }
    #[must_use]
    pub fn safety_rules(&self) -> &[String] {
        &self.safety_rules
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PresetUpgradePlan {
    preset_id: String,
    action: PresetUpgradeAction,
    target_template: String,
    reason: String,
}

impl PresetUpgradePlan {
    #[must_use]
    pub fn new(
        preset_id: impl Into<String>,
        action: PresetUpgradeAction,
        target_template: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            preset_id: preset_id.into(),
            action,
            target_template: target_template.into(),
            reason: reason.into(),
        }
    }

    #[must_use]
    pub fn preset_id(&self) -> &str {
        &self.preset_id
    }
    #[must_use]
    pub const fn action(&self) -> PresetUpgradeAction {
        self.action
    }
    #[must_use]
    pub fn target_template(&self) -> &str {
        &self.target_template
    }
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TemplateRegistryPlan {
    command: String,
    contract: TemplateRegistryContract,
    templates: Vec<TemplateMetadata>,
    presets: Vec<PresetMetadata>,
    preset_upgrades: Vec<PresetUpgradePlan>,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl TemplateRegistryPlan {
    #[must_use]
    pub fn new(
        contract: TemplateRegistryContract,
        mut templates: Vec<TemplateMetadata>,
        mut presets: Vec<PresetMetadata>,
        mut preset_upgrades: Vec<PresetUpgradePlan>,
    ) -> Self {
        templates.sort_by(|left, right| left.id().cmp(right.id()));
        presets.sort_by(|left, right| left.id().cmp(right.id()));
        preset_upgrades.sort_by(|left, right| left.preset_id().cmp(right.preset_id()));
        Self {
            command: "template-registry".to_string(),
            contract,
            templates,
            presets,
            preset_upgrades,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/template-registry-index.md"),
                PathBuf::from(".monad/reports/template-registry-index.json"),
            ],
            safety_notes: vec![
                "No remote template registry is queried.".to_string(),
                "No templates were rendered or applied.".to_string(),
                "No preset upgrades are performed.".to_string(),
                "No user-owned source files are rewritten.".to_string(),
            ],
        }
    }

    #[must_use]
    pub const fn contract(&self) -> &TemplateRegistryContract {
        &self.contract
    }
    #[must_use]
    pub fn templates(&self) -> &[TemplateMetadata] {
        &self.templates
    }
    #[must_use]
    pub fn presets(&self) -> &[PresetMetadata] {
        &self.presets
    }
    #[must_use]
    pub fn preset_upgrades(&self) -> &[PresetUpgradePlan] {
        &self.preset_upgrades
    }
    #[must_use]
    pub fn evidence_paths(&self) -> &[PathBuf] {
        &self.evidence_paths
    }
    #[must_use]
    pub fn safety_notes(&self) -> &[String] {
        &self.safety_notes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateRegistryApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl TemplateRegistryApplyResult {
    #[must_use]
    pub fn new(write_results: Vec<GatedWriteResult>) -> Self {
        Self { write_results }
    }
    #[must_use]
    pub fn write_results(&self) -> &[GatedWriteResult] {
        &self.write_results
    }
}

#[must_use]
pub fn build_template_registry_plan(root: impl AsRef<Path>) -> TemplateRegistryPlan {
    let root = root.as_ref();
    let contract = TemplateRegistryContract::default_contract();
    let templates = discover_templates(root, contract.templates_dir(), contract.schema_version());
    let presets = discover_presets(root, contract.presets_dir(), &templates);
    let preset_upgrades = plan_preset_upgrades(&presets);
    TemplateRegistryPlan::new(contract, templates, presets, preset_upgrades)
}

#[must_use]
pub fn discover_templates(
    root: &Path,
    templates_dir: &Path,
    schema_version: u16,
) -> Vec<TemplateMetadata> {
    let mut templates = discover_metadata_files(root, templates_dir)
        .into_iter()
        .map(|path| template_from_path(root, path, schema_version))
        .collect::<Vec<_>>();
    if templates.is_empty() {
        templates.push(TemplateMetadata::new(
            "template:default-rust",
            "templates/default-rust/template.toml",
            "default-rust",
            "0.1.0",
            1,
            TemplateCompatibility::Compatible,
            "built-in placeholder template metadata used until repo templates exist",
        ));
    }
    templates.sort_by(|left, right| left.id().cmp(right.id()));
    templates
}

#[must_use]
pub fn discover_presets(
    root: &Path,
    presets_dir: &Path,
    templates: &[TemplateMetadata],
) -> Vec<PresetMetadata> {
    let mut presets = discover_metadata_files(root, presets_dir)
        .into_iter()
        .map(|path| preset_from_path(root, path, templates))
        .collect::<Vec<_>>();
    if presets.is_empty() {
        let target_template = templates
            .first()
            .map_or("template:default-rust", TemplateMetadata::id);
        presets.push(PresetMetadata::new(
            "preset:default-local",
            "presets/default-local/preset.toml",
            "default-local",
            "0.1.0",
            target_template,
            TemplateCompatibility::Compatible,
            PresetUpgradeAction::Keep,
            "built-in placeholder preset metadata used until repo presets exist",
        ));
    }
    presets.sort_by(|left, right| left.id().cmp(right.id()));
    presets
}

#[must_use]
pub const fn validate_template_compatibility(
    current_schema_version: u16,
    min_schema_version: u16,
) -> TemplateCompatibility {
    if min_schema_version > current_schema_version {
        TemplateCompatibility::Incompatible
    } else if min_schema_version == current_schema_version {
        TemplateCompatibility::Compatible
    } else {
        TemplateCompatibility::Review
    }
}

#[must_use]
pub fn validate_preset_compatibility(
    target_template: &str,
    templates: &[TemplateMetadata],
) -> TemplateCompatibility {
    if templates
        .iter()
        .any(|template| template.id() == target_template)
    {
        TemplateCompatibility::Compatible
    } else {
        TemplateCompatibility::Review
    }
}

#[must_use]
pub fn plan_preset_upgrades(presets: &[PresetMetadata]) -> Vec<PresetUpgradePlan> {
    presets
        .iter()
        .map(|preset| {
            PresetUpgradePlan::new(
                preset.id().to_string(),
                preset.upgrade_action(),
                preset.target_template().to_string(),
                preset.reason().to_string(),
            )
        })
        .collect()
}

pub fn write_template_registry_evidence(
    root: impl AsRef<Path>,
) -> Result<TemplateRegistryApplyResult, String> {
    let root = root.as_ref();
    let plan = build_template_registry_plan(root);
    let requests = [
        GatedWriteRequest::new(
            ".monad/reports/template-registry-index.md",
            render_template_registry_plan(&plan),
            true,
        ),
        GatedWriteRequest::new(
            ".monad/reports/template-registry-index.json",
            render_template_registry_plan_json(&plan),
            true,
        ),
    ];
    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TemplateRegistryApplyResult::new(write_results))
}

#[must_use]
pub fn render_template_registry_plan(plan: &TemplateRegistryPlan) -> String {
    let mut lines = vec![
        "Monad template registry and preset evolution plan".to_string(),
        String::new(),
        "Contract:".to_string(),
        format!("  schema_version: {}", plan.contract().schema_version()),
        format!(
            "  templates_dir: {}",
            plan.contract().templates_dir().display()
        ),
        format!("  presets_dir: {}", plan.contract().presets_dir().display()),
        format!("  index_path: {}", plan.contract().index_path().display()),
        String::new(),
        "Safety rules:".to_string(),
    ];
    for rule in plan.contract().safety_rules() {
        lines.push(format!("  - {rule}"));
    }
    lines.push(String::new());
    lines.push(format!("Templates: {}", plan.templates().len()));
    for template in plan.templates() {
        lines.push(format!(
            "  - {} version={} compatibility={} min_schema={} path={}",
            template.id(),
            template.version(),
            template.compatibility().as_str(),
            template.min_schema_version(),
            template.path().display()
        ));
        lines.push(format!("    reason: {}", template.reason()));
    }
    lines.push(String::new());
    lines.push(format!("Presets: {}", plan.presets().len()));
    for preset in plan.presets() {
        lines.push(format!(
            "  - {} version={} target={} compatibility={} action={} path={}",
            preset.id(),
            preset.version(),
            preset.target_template(),
            preset.compatibility().as_str(),
            preset.upgrade_action().as_str(),
            preset.path().display()
        ));
        lines.push(format!("    reason: {}", preset.reason()));
    }
    lines.push(String::new());
    lines.push("Preset upgrade plan:".to_string());
    for upgrade in plan.preset_upgrades() {
        lines.push(format!(
            "  - {} action={} target={} reason={}",
            upgrade.preset_id(),
            upgrade.action().as_str(),
            upgrade.target_template(),
            upgrade.reason()
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

#[must_use]
pub fn render_template_registry_plan_json(plan: &TemplateRegistryPlan) -> String {
    serde_json::to_string_pretty(plan).unwrap_or_else(|_| {
        "{\n  \"command\": \"template-registry\",\n  \"error\": \"template registry plan serialization failed\"\n}".to_string()
    })
}

#[must_use]
pub fn render_template_registry_apply_result(result: &TemplateRegistryApplyResult) -> String {
    let mut lines = vec![
        "Monad template-registry evidence write result".to_string(),
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
    lines.push("No templates were rendered or applied.".to_string());
    lines.push("No presets were upgraded.".to_string());
    lines.push("No remote template registry was contacted.".to_string());
    lines.push("No user-owned source files were rewritten.".to_string());
    lines.join("\n")
}

fn template_from_path(root: &Path, path: PathBuf, schema_version: u16) -> TemplateMetadata {
    let text = fs::read_to_string(root.join(&path)).unwrap_or_default();
    let name = metadata_value(&text, "name").unwrap_or_else(|| fallback_name(&path));
    let version = metadata_value(&text, "version").unwrap_or_else(|| "0.0.0".to_string());
    let min_schema_version = metadata_value(&text, "min_schema_version")
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(1);
    let compatibility = validate_template_compatibility(schema_version, min_schema_version);
    let id = metadata_value(&text, "id").unwrap_or_else(|| make_registry_id("template", &name));
    let reason = template_reason(compatibility, schema_version, min_schema_version);
    TemplateMetadata::new(
        id,
        path,
        name,
        version,
        min_schema_version,
        compatibility,
        reason,
    )
}

fn preset_from_path(root: &Path, path: PathBuf, templates: &[TemplateMetadata]) -> PresetMetadata {
    let text = fs::read_to_string(root.join(&path)).unwrap_or_default();
    let name = metadata_value(&text, "name").unwrap_or_else(|| fallback_name(&path));
    let version = metadata_value(&text, "version").unwrap_or_else(|| "0.0.0".to_string());
    let target_template = metadata_value(&text, "target_template").unwrap_or_else(|| {
        templates
            .first()
            .map_or("template:unknown".to_string(), |template| {
                template.id().to_string()
            })
    });
    let id = metadata_value(&text, "id").unwrap_or_else(|| make_registry_id("preset", &name));
    let compatibility = validate_preset_compatibility(&target_template, templates);
    let upgrade_action =
        if compatibility == TemplateCompatibility::Compatible && version.starts_with("0.") {
            PresetUpgradeAction::PlanUpgrade
        } else if compatibility == TemplateCompatibility::Compatible {
            PresetUpgradeAction::Keep
        } else {
            PresetUpgradeAction::Review
        };
    let reason = preset_reason(compatibility, upgrade_action, &target_template);
    PresetMetadata::new(
        id,
        path,
        name,
        version,
        target_template,
        compatibility,
        upgrade_action,
        reason,
    )
}

fn discover_metadata_files(root: &Path, relative_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_metadata_files(root, &root.join(relative_dir), &mut files);
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
        } else if is_metadata_file(&path) {
            files.push(
                path.strip_prefix(root)
                    .map_or(path.clone(), Path::to_path_buf),
            );
        }
    }
}

fn is_metadata_file(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|value| value.to_str()),
        Some(
            "template.toml"
                | "template.json"
                | "preset.toml"
                | "preset.json"
                | "metadata.toml"
                | "metadata.json"
        )
    )
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

fn fallback_name(path: &Path) -> String {
    path.parent().and_then(Path::file_name).map_or_else(
        || "unnamed".to_string(),
        |name| name.to_string_lossy().to_string(),
    )
}

fn make_registry_id(kind: &str, name: &str) -> String {
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
    format!("{kind}:{slug}")
}

fn template_reason(
    compatibility: TemplateCompatibility,
    current_schema_version: u16,
    min_schema_version: u16,
) -> String {
    match compatibility {
        TemplateCompatibility::Compatible => {
            format!(
                "template requires schema {min_schema_version}; current schema is {current_schema_version}"
            )
        }
        TemplateCompatibility::Review => {
            format!("template uses older schema {min_schema_version}; review before evolution")
        }
        TemplateCompatibility::Incompatible => {
            format!(
                "template requires future schema {min_schema_version}; current schema is {current_schema_version}"
            )
        }
    }
}

fn preset_reason(
    compatibility: TemplateCompatibility,
    action: PresetUpgradeAction,
    target_template: &str,
) -> String {
    match (compatibility, action) {
        (TemplateCompatibility::Compatible, PresetUpgradeAction::Keep) => {
            format!("preset target `{target_template}` is compatible and no upgrade is required")
        }
        (TemplateCompatibility::Compatible, PresetUpgradeAction::PlanUpgrade) => {
            format!(
                "preset target `{target_template}` is compatible; pre-1.0 preset can receive a supervised upgrade plan"
            )
        }
        _ => format!(
            "preset target `{target_template}` requires human review before upgrade planning"
        ),
    }
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
            "monad-template-registry-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);
        assert!(fs::create_dir_all(root.join("templates/rust")).is_ok());
        assert!(fs::create_dir_all(root.join("presets/local")).is_ok());
        assert!(fs::write(root.join("templates/rust/template.toml"), "id = \"template:rust\"\nname = \"rust\"\nversion = \"1.0.0\"\nmin_schema_version = 1\n").is_ok());
        assert!(fs::write(root.join("presets/local/preset.toml"), "id = \"preset:local\"\nname = \"local\"\nversion = \"0.1.0\"\ntarget_template = \"template:rust\"\n").is_ok());
        root
    }

    #[test]
    fn template_compatibility_blocks_future_schema() {
        assert_eq!(
            validate_template_compatibility(1, 2),
            TemplateCompatibility::Incompatible
        );
        assert_eq!(
            validate_template_compatibility(1, 1),
            TemplateCompatibility::Compatible
        );
    }

    #[test]
    fn templates_are_discovered_from_local_metadata() {
        let root = create_workspace("templates");
        let templates = discover_templates(&root, Path::new("templates"), 1);
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].id(), "template:rust");
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn presets_are_mapped_to_template_compatibility() {
        let root = create_workspace("presets");
        let templates = discover_templates(&root, Path::new("templates"), 1);
        let presets = discover_presets(&root, Path::new("presets"), &templates);
        assert_eq!(presets.len(), 1);
        assert_eq!(
            presets[0].upgrade_action(),
            PresetUpgradeAction::PlanUpgrade
        );
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn registry_plan_contains_upgrade_plan() {
        let root = create_workspace("plan");
        let plan = build_template_registry_plan(&root);
        assert_eq!(plan.templates().len(), 1);
        assert_eq!(plan.presets().len(), 1);
        assert_eq!(plan.preset_upgrades().len(), 1);
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn text_render_mentions_safety_notes() {
        let root = create_workspace("text");
        let plan = build_template_registry_plan(&root);
        let text = render_template_registry_plan(&plan);
        assert!(text.contains("Monad template registry and preset evolution plan"));
        assert!(text.contains("No templates were rendered or applied"));
        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn json_render_contains_template_registry_command() {
        let root = create_workspace("json");
        let plan = build_template_registry_plan(&root);
        let json = render_template_registry_plan_json(&plan);
        assert!(json.contains("\"command\": \"template-registry\""));
        assert!(json.contains("template:rust"));
        fs::remove_dir_all(root).ok();
    }
}
