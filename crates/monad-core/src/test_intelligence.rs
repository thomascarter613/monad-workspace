//! Test intelligence and verification planning foundation.
//!
//! E26 introduces a local-only verification planning model. Monad discovers
//! likely test commands from local manifests and recommends targeted
//! verification. It does not execute the commands, invoke package managers,
//! call remote services, or rewrite user-owned source files.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

/// Source used to discover a verification command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TestCommandSource {
    /// Rust Cargo workspace or package manifest.
    Cargo,

    /// Node package manifest.
    PackageJson,

    /// Python project manifest.
    PyProject,

    /// Go module manifest.
    GoModule,

    /// Java Maven manifest.
    Maven,

    /// Java Gradle manifest.
    Gradle,

    /// Monad local verification script.
    Script,

    /// Fallback command when no manifest-specific command is known.
    Fallback,
}

impl TestCommandSource {
    /// Stable source label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::PackageJson => "package-json",
            Self::PyProject => "pyproject",
            Self::GoModule => "go-module",
            Self::Maven => "maven",
            Self::Gradle => "gradle",
            Self::Script => "script",
            Self::Fallback => "fallback",
        }
    }
}

/// Confidence level for a verification recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerificationConfidence {
    /// Broad fallback verification.
    Low,

    /// Manifest-derived recommendation.
    Medium,

    /// Direct component or script-specific recommendation.
    High,
}

impl VerificationConfidence {
    /// Stable confidence label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

/// One discovered verification command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TestCommand {
    id: String,
    command: String,
    source: TestCommandSource,
    component: String,
    manifest_path: PathBuf,
}

impl TestCommand {
    /// Creates a test command model.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        command: impl Into<String>,
        source: TestCommandSource,
        component: impl Into<String>,
        manifest_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id: id.into(),
            command: command.into(),
            source,
            component: component.into(),
            manifest_path: manifest_path.into(),
        }
    }

    /// Stable command ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Shell command text. This is recommended text only and is not executed by Monad.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Discovery source.
    #[must_use]
    pub const fn source(&self) -> TestCommandSource {
        self.source
    }

    /// Component or package this command verifies.
    #[must_use]
    pub fn component(&self) -> &str {
        &self.component
    }

    /// Manifest or script path that caused this command to be recommended.
    #[must_use]
    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }
}

/// A component-to-test mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TestComponentMapping {
    component: String,
    command_ids: Vec<String>,
    evidence: Vec<String>,
}

impl TestComponentMapping {
    /// Creates a deterministic component mapping.
    #[must_use]
    pub fn new(
        component: impl Into<String>,
        mut command_ids: Vec<String>,
        mut evidence: Vec<String>,
    ) -> Self {
        command_ids.sort();
        command_ids.dedup();
        evidence.sort();
        evidence.dedup();

        Self {
            component: component.into(),
            command_ids,
            evidence,
        }
    }

    /// Component name.
    #[must_use]
    pub fn component(&self) -> &str {
        &self.component
    }

    /// Command IDs mapped to this component.
    #[must_use]
    pub fn command_ids(&self) -> &[String] {
        &self.command_ids
    }

    /// Mapping evidence.
    #[must_use]
    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }
}

/// One verification recommendation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VerificationRecommendation {
    command: String,
    confidence: VerificationConfidence,
    reason: String,
}

impl VerificationRecommendation {
    /// Creates a verification recommendation.
    #[must_use]
    pub fn new(
        command: impl Into<String>,
        confidence: VerificationConfidence,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            command: command.into(),
            confidence,
            reason: reason.into(),
        }
    }

    /// Recommended command text.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Recommendation confidence.
    #[must_use]
    pub const fn confidence(&self) -> VerificationConfidence {
        self.confidence
    }

    /// Recommendation rationale.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Full test intelligence and verification plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TestIntelligencePlan {
    command: String,
    discovered_commands: Vec<TestCommand>,
    component_mappings: Vec<TestComponentMapping>,
    recommendations: Vec<VerificationRecommendation>,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl TestIntelligencePlan {
    /// Creates a deterministic plan.
    #[must_use]
    pub fn new(
        mut discovered_commands: Vec<TestCommand>,
        mut component_mappings: Vec<TestComponentMapping>,
        mut recommendations: Vec<VerificationRecommendation>,
    ) -> Self {
        discovered_commands.sort_by(|left, right| left.id().cmp(right.id()));
        discovered_commands.dedup_by(|left, right| left.id() == right.id());

        component_mappings.sort_by(|left, right| left.component().cmp(right.component()));
        component_mappings.dedup_by(|left, right| left.component() == right.component());

        recommendations.sort_by(|left, right| {
            left.command()
                .cmp(right.command())
                .then(left.reason().cmp(right.reason()))
        });
        recommendations.dedup_by(|left, right| left.command() == right.command());

        Self {
            command: "verify-plan".to_string(),
            discovered_commands,
            component_mappings,
            recommendations,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/test-intelligence-report.md"),
                PathBuf::from(".monad/reports/test-intelligence-report.json"),
            ],
            safety_notes: vec![
                "No test commands are executed by Monad.".to_string(),
                "No package managers are invoked by Monad.".to_string(),
                "No remote services are called.".to_string(),
                "No user-owned source files are rewritten.".to_string(),
                "Only generated local evidence is written when --yes is used.".to_string(),
            ],
        }
    }

    /// Discovered test commands.
    #[must_use]
    pub fn discovered_commands(&self) -> &[TestCommand] {
        &self.discovered_commands
    }

    /// Component mappings.
    #[must_use]
    pub fn component_mappings(&self) -> &[TestComponentMapping] {
        &self.component_mappings
    }

    /// Verification recommendations.
    #[must_use]
    pub fn recommendations(&self) -> &[VerificationRecommendation] {
        &self.recommendations
    }

    /// Evidence paths.
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

/// Apply result for generated verification planning evidence writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestIntelligenceApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl TestIntelligenceApplyResult {
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

/// Builds a test intelligence and verification plan for a repository root.
#[must_use]
pub fn build_test_intelligence_plan(root: impl AsRef<Path>) -> TestIntelligencePlan {
    let root = root.as_ref();
    let commands = discover_test_commands(root);
    let mappings = map_tests_to_components(&commands);
    let recommendations = generate_verification_recommendations(&commands, &mappings);

    TestIntelligencePlan::new(commands, mappings, recommendations)
}

/// Discovers likely test commands from local manifests and scripts.
#[must_use]
pub fn discover_test_commands(root: &Path) -> Vec<TestCommand> {
    let mut commands = Vec::new();

    if root.join("Cargo.toml").is_file() {
        commands.push(TestCommand::new(
            "cargo:workspace:test",
            "cargo test",
            TestCommandSource::Cargo,
            "rust-workspace",
            "Cargo.toml",
        ));
        commands.push(TestCommand::new(
            "cargo:workspace:clippy",
            "cargo clippy --all-targets --all-features -- -D warnings",
            TestCommandSource::Cargo,
            "rust-workspace",
            "Cargo.toml",
        ));
        commands.push(TestCommand::new(
            "cargo:workspace:fmt",
            "cargo fmt --check",
            TestCommandSource::Cargo,
            "rust-workspace",
            "Cargo.toml",
        ));
    }

    for (component, path) in rust_component_manifests(root) {
        commands.push(TestCommand::new(
            format!("cargo:{component}:test"),
            format!("cargo test -p {component}"),
            TestCommandSource::Cargo,
            component,
            path,
        ));
    }

    if root.join("package.json").is_file() {
        commands.push(TestCommand::new(
            "node:package:test",
            "bun test || npm test",
            TestCommandSource::PackageJson,
            "node-package",
            "package.json",
        ));
    }

    if root.join("pyproject.toml").is_file() {
        commands.push(TestCommand::new(
            "python:pyproject:test",
            "pytest",
            TestCommandSource::PyProject,
            "python-project",
            "pyproject.toml",
        ));
    }

    if root.join("go.mod").is_file() {
        commands.push(TestCommand::new(
            "go:module:test",
            "go test ./...",
            TestCommandSource::GoModule,
            "go-module",
            "go.mod",
        ));
    }

    if root.join("pom.xml").is_file() {
        commands.push(TestCommand::new(
            "java:maven:test",
            "mvn test",
            TestCommandSource::Maven,
            "maven-project",
            "pom.xml",
        ));
    }

    if root.join("build.gradle").is_file() || root.join("build.gradle.kts").is_file() {
        let manifest = if root.join("build.gradle.kts").is_file() {
            "build.gradle.kts"
        } else {
            "build.gradle"
        };

        commands.push(TestCommand::new(
            "java:gradle:test",
            "./gradlew test",
            TestCommandSource::Gradle,
            "gradle-project",
            manifest,
        ));
    }

    if root.join("tools/scripts/verify.sh").is_file() {
        commands.push(TestCommand::new(
            "script:verify",
            "tools/scripts/verify.sh",
            TestCommandSource::Script,
            "repo-verification",
            "tools/scripts/verify.sh",
        ));
    }

    commands.sort_by(|left, right| left.id().cmp(right.id()));
    commands.dedup_by(|left, right| left.id() == right.id());

    commands
}

/// Maps discovered commands to components.
#[must_use]
pub fn map_tests_to_components(commands: &[TestCommand]) -> Vec<TestComponentMapping> {
    let mut components = BTreeSet::new();
    for command in commands {
        components.insert(command.component().to_string());
    }

    components
        .into_iter()
        .map(|component| {
            let command_ids = commands
                .iter()
                .filter(|command| command.component() == component)
                .map(|command| command.id().to_string())
                .collect::<Vec<_>>();

            let evidence = commands
                .iter()
                .filter(|command| command.component() == component)
                .map(|command| {
                    format!(
                        "{} discovered from {}",
                        command.command(),
                        command.manifest_path().display()
                    )
                })
                .collect::<Vec<_>>();

            TestComponentMapping::new(component, command_ids, evidence)
        })
        .collect()
}

/// Generates targeted verification recommendations.
#[must_use]
pub fn generate_verification_recommendations(
    commands: &[TestCommand],
    mappings: &[TestComponentMapping],
) -> Vec<VerificationRecommendation> {
    let mut recommendations = Vec::new();

    if commands.is_empty() {
        recommendations.push(VerificationRecommendation::new(
            "Review repository manifests and define at least one verification command.",
            VerificationConfidence::Low,
            "no local test manifests were discovered",
        ));
        return recommendations;
    }

    for command in commands {
        let confidence = match command.source() {
            TestCommandSource::Cargo | TestCommandSource::Script => VerificationConfidence::High,
            TestCommandSource::Fallback => VerificationConfidence::Low,
            _ => VerificationConfidence::Medium,
        };

        recommendations.push(VerificationRecommendation::new(
            command.command().to_string(),
            confidence,
            format!(
                "{} verification for {} from {}",
                command.source().as_str(),
                command.component(),
                command.manifest_path().display()
            ),
        ));
    }

    if mappings
        .iter()
        .any(|mapping| mapping.component() == "rust-workspace")
    {
        recommendations.push(VerificationRecommendation::new(
            "cargo test",
            VerificationConfidence::High,
            "workspace-level Rust test command provides baseline confidence",
        ));
    }

    recommendations.push(VerificationRecommendation::new(
        "tools/scripts/verify-test-intelligence.sh",
        VerificationConfidence::High,
        "E26 smoke verification confirms planner behavior",
    ));
    recommendations.push(VerificationRecommendation::new(
        "tools/scripts/verify-e26.sh",
        VerificationConfidence::High,
        "E26 closeout verification confirms docs and smoke path",
    ));

    recommendations
}

/// Writes generated test-intelligence evidence reports.
pub fn write_test_intelligence_evidence(
    root: impl AsRef<Path>,
) -> Result<TestIntelligenceApplyResult, String> {
    let root = root.as_ref();
    let plan = build_test_intelligence_plan(root);
    let markdown = render_test_intelligence_plan(&plan);
    let json = render_test_intelligence_plan_json(&plan);

    let requests = [
        GatedWriteRequest::new(".monad/reports/test-intelligence-report.md", markdown, true),
        GatedWriteRequest::new(".monad/reports/test-intelligence-report.json", json, true),
    ];

    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(TestIntelligenceApplyResult::new(write_results))
}

/// Renders a text verification planning report.
#[must_use]
pub fn render_test_intelligence_plan(plan: &TestIntelligencePlan) -> String {
    let mut lines = vec![
        "Monad test intelligence and verification plan".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!(
            "  discovered_commands: {}",
            plan.discovered_commands().len()
        ),
        format!("  component_mappings: {}", plan.component_mappings().len()),
        format!("  recommendations: {}", plan.recommendations().len()),
        String::new(),
        "Discovered commands:".to_string(),
    ];

    if plan.discovered_commands().is_empty() {
        lines.push("  - no test commands discovered".to_string());
    } else {
        for command in plan.discovered_commands() {
            lines.push(format!(
                "  - {} [{}] component={} source={} manifest={}",
                command.command(),
                command.id(),
                command.component(),
                command.source().as_str(),
                command.manifest_path().display()
            ));
        }
    }

    lines.push(String::new());
    lines.push("Component mappings:".to_string());
    if plan.component_mappings().is_empty() {
        lines.push("  - no component mappings discovered".to_string());
    } else {
        for mapping in plan.component_mappings() {
            lines.push(format!("  - {}", mapping.component()));
            lines.push(format!(
                "    commands: {}",
                mapping.command_ids().join(", ")
            ));
            lines.push(format!("    evidence: {}", mapping.evidence().join(" | ")));
        }
    }

    lines.push(String::new());
    lines.push("Recommended verification:".to_string());
    for recommendation in plan.recommendations() {
        lines.push(format!(
            "  - {} confidence={} reason={}",
            recommendation.command(),
            recommendation.confidence().as_str(),
            recommendation.reason()
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

/// Renders a JSON verification planning report.
#[must_use]
pub fn render_test_intelligence_plan_json(plan: &TestIntelligencePlan) -> String {
    serde_json::to_string_pretty(plan).unwrap_or_else(|_| {
        "{\n  \"command\": \"verify-plan\",\n  \"error\": \"test intelligence plan serialization failed\"\n}".to_string()
    })
}

/// Renders generated evidence write results.
#[must_use]
pub fn render_test_intelligence_apply_result(result: &TestIntelligenceApplyResult) -> String {
    let mut lines = vec![
        "Monad test-intelligence evidence write result".to_string(),
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
    lines.push("No test commands were executed by Monad.".to_string());
    lines.push("No package managers were invoked.".to_string());
    lines.push("No remote services were called.".to_string());
    lines.push("No user-owned source files were rewritten.".to_string());

    lines.join("\n")
}

fn rust_component_manifests(root: &Path) -> Vec<(String, PathBuf)> {
    let crates_dir = root.join("crates");
    let Ok(entries) = fs::read_dir(crates_dir) else {
        return Vec::new();
    };

    let mut manifests = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path().join("Cargo.toml");
            if !path.is_file() {
                return None;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            Some((
                name,
                path.strip_prefix(root)
                    .map_or(path.clone(), Path::to_path_buf),
            ))
        })
        .collect::<Vec<_>>();

    manifests.sort_by(|left, right| left.0.cmp(&right.0));
    manifests
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
            "monad-test-intelligence-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);
        assert!(fs::create_dir_all(root.join("crates/monad-core")).is_ok());
        assert!(fs::create_dir_all(root.join("crates/monad-cli")).is_ok());
        assert!(fs::create_dir_all(root.join("tools/scripts")).is_ok());
        assert!(fs::write(root.join("Cargo.toml"), "[workspace]\n").is_ok());
        assert!(
            fs::write(
                root.join("crates/monad-core/Cargo.toml"),
                "[package]\nname = \"monad-core\"\n"
            )
            .is_ok()
        );
        assert!(
            fs::write(
                root.join("crates/monad-cli/Cargo.toml"),
                "[package]\nname = \"monad-cli\"\n"
            )
            .is_ok()
        );
        root
    }

    #[test]
    fn cargo_workspace_commands_are_discovered() {
        let root = create_workspace("cargo");
        let commands = discover_test_commands(&root);
        let command_texts = commands
            .iter()
            .map(TestCommand::command)
            .collect::<Vec<_>>();

        assert!(command_texts.contains(&"cargo test"));
        assert!(command_texts.contains(&"cargo fmt --check"));
        assert!(
            command_texts
                .iter()
                .any(|command| command.contains("cargo clippy"))
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn component_manifests_create_package_specific_tests() {
        let root = create_workspace("components");
        let commands = discover_test_commands(&root);

        assert!(
            commands
                .iter()
                .any(|command| command.command() == "cargo test -p monad-core")
        );
        assert!(
            commands
                .iter()
                .any(|command| command.command() == "cargo test -p monad-cli")
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn mappings_group_commands_by_component() {
        let root = create_workspace("mapping");
        let commands = discover_test_commands(&root);
        let mappings = map_tests_to_components(&commands);

        assert!(
            mappings
                .iter()
                .any(|mapping| mapping.component() == "rust-workspace")
        );
        assert!(
            mappings
                .iter()
                .any(|mapping| mapping.component() == "monad-core")
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn plan_contains_e26_smoke_recommendations() {
        let root = create_workspace("plan");
        let plan = build_test_intelligence_plan(&root);

        assert!(
            plan.recommendations()
                .iter()
                .any(|recommendation| recommendation.command() == "tools/scripts/verify-e26.sh")
        );
        assert!(
            plan.recommendations()
                .iter()
                .any(|recommendation| recommendation.confidence() == VerificationConfidence::High)
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn text_render_mentions_no_execution_safety() {
        let root = create_workspace("text");
        let plan = build_test_intelligence_plan(&root);
        let text = render_test_intelligence_plan(&plan);

        assert!(text.contains("Monad test intelligence and verification plan"));
        assert!(text.contains("No test commands are executed by Monad"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn json_render_contains_verify_plan_command() {
        let root = create_workspace("json");
        let plan = build_test_intelligence_plan(&root);
        let json = render_test_intelligence_plan_json(&plan);

        assert!(json.contains("\"command\": \"verify-plan\""));
        assert!(json.contains("cargo test"));

        fs::remove_dir_all(root).ok();
    }
}
