//! MCP and external tool integration foundation.
//!
//! E31 defines Monad's MCP boundary, local tool capability model, context export
//! proof of concept, and external tool invocation policy checks. This module
//! never connects to MCP servers, invokes tools, executes subprocesses, opens
//! sockets, or mutates user-owned source files.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

/// Capability kind exposed through a future MCP/local-tool boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum McpCapabilityKind {
    /// Read-only context export.
    ContextExport,

    /// Read-only repository metadata lookup.
    RepoMetadata,

    /// Generated report lookup.
    ReportLookup,

    /// External command/tool invocation.
    ToolInvocation,
}

impl McpCapabilityKind {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContextExport => "context-export",
            Self::RepoMetadata => "repo-metadata",
            Self::ReportLookup => "report-lookup",
            Self::ToolInvocation => "tool-invocation",
        }
    }
}

/// Policy decision for an external tool capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExternalToolPolicyDecision {
    /// Capability is allowed as a read-only planning/export capability.
    AllowReadOnly,

    /// Capability is visible only for human review.
    ReviewRequired,

    /// Capability is blocked by the E31 foundation.
    Blocked,
}

impl ExternalToolPolicyDecision {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowReadOnly => "allow-read-only",
            Self::ReviewRequired => "review-required",
            Self::Blocked => "blocked",
        }
    }
}

/// MCP integration boundary contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct McpIntegrationBoundary {
    local_only: bool,
    network_disabled: bool,
    tool_invocation_disabled: bool,
    context_export_path: PathBuf,
    rules: Vec<String>,
}

impl McpIntegrationBoundary {
    /// Default E31 MCP boundary.
    #[must_use]
    pub fn default_boundary() -> Self {
        Self {
            local_only: true,
            network_disabled: true,
            tool_invocation_disabled: true,
            context_export_path: PathBuf::from(".monad/reports/mcp-context-export.json"),
            rules: vec![
                "MCP integration is planning-only in E31.".to_string(),
                "No MCP server connection is opened.".to_string(),
                "No external tool invocation is executed.".to_string(),
                "Only generated context/export evidence may be written with --yes.".to_string(),
            ],
        }
    }

    /// Whether the boundary is local-only.
    #[must_use]
    pub const fn local_only(&self) -> bool {
        self.local_only
    }

    /// Whether network access is disabled.
    #[must_use]
    pub const fn network_disabled(&self) -> bool {
        self.network_disabled
    }

    /// Whether external tool invocation is disabled.
    #[must_use]
    pub const fn tool_invocation_disabled(&self) -> bool {
        self.tool_invocation_disabled
    }

    /// Context export path.
    #[must_use]
    pub fn context_export_path(&self) -> &Path {
        &self.context_export_path
    }

    /// Boundary rules.
    #[must_use]
    pub fn rules(&self) -> &[String] {
        &self.rules
    }
}

/// Tool/capability model for MCP planning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct McpToolCapability {
    id: String,
    kind: McpCapabilityKind,
    description: String,
    read_only: bool,
    requires_invocation: bool,
}

impl McpToolCapability {
    /// Creates a tool capability record.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        kind: McpCapabilityKind,
        description: impl Into<String>,
        read_only: bool,
        requires_invocation: bool,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            description: description.into(),
            read_only,
            requires_invocation,
        }
    }

    /// Capability ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Capability kind.
    #[must_use]
    pub const fn kind(&self) -> McpCapabilityKind {
        self.kind
    }

    /// Description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Whether the capability is read-only.
    #[must_use]
    pub const fn read_only(&self) -> bool {
        self.read_only
    }

    /// Whether the capability would require external invocation.
    #[must_use]
    pub const fn requires_invocation(&self) -> bool {
        self.requires_invocation
    }
}

/// External tool policy check result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExternalToolPolicyCheck {
    capability_id: String,
    decision: ExternalToolPolicyDecision,
    reason: String,
}

impl ExternalToolPolicyCheck {
    /// Creates a policy check.
    #[must_use]
    pub fn new(
        capability_id: impl Into<String>,
        decision: ExternalToolPolicyDecision,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            capability_id: capability_id.into(),
            decision,
            reason: reason.into(),
        }
    }

    /// Capability ID.
    #[must_use]
    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }

    /// Policy decision.
    #[must_use]
    pub const fn decision(&self) -> ExternalToolPolicyDecision {
        self.decision
    }

    /// Rationale.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Local MCP context export proof-of-concept.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct McpContextExport {
    repository_root: PathBuf,
    exported_paths: Vec<PathBuf>,
    summary: Vec<String>,
}

impl McpContextExport {
    /// Creates a context export record.
    #[must_use]
    pub fn new(
        repository_root: impl Into<PathBuf>,
        mut exported_paths: Vec<PathBuf>,
        mut summary: Vec<String>,
    ) -> Self {
        exported_paths.sort();
        exported_paths.dedup();
        summary.sort();
        summary.dedup();

        Self {
            repository_root: repository_root.into(),
            exported_paths,
            summary,
        }
    }

    /// Repository root.
    #[must_use]
    pub fn repository_root(&self) -> &Path {
        &self.repository_root
    }

    /// Exported path summaries.
    #[must_use]
    pub fn exported_paths(&self) -> &[PathBuf] {
        &self.exported_paths
    }

    /// Summary lines.
    #[must_use]
    pub fn summary(&self) -> &[String] {
        &self.summary
    }
}

/// Full MCP integration plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct McpIntegrationPlan {
    command: String,
    boundary: McpIntegrationBoundary,
    capabilities: Vec<McpToolCapability>,
    policy_checks: Vec<ExternalToolPolicyCheck>,
    context_export: McpContextExport,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl McpIntegrationPlan {
    /// Creates a deterministic MCP integration plan.
    #[must_use]
    pub fn new(
        boundary: McpIntegrationBoundary,
        mut capabilities: Vec<McpToolCapability>,
        mut policy_checks: Vec<ExternalToolPolicyCheck>,
        context_export: McpContextExport,
    ) -> Self {
        capabilities.sort_by(|left, right| left.id().cmp(right.id()));
        capabilities.dedup_by(|left, right| left.id() == right.id());

        policy_checks.sort_by(|left, right| left.capability_id().cmp(right.capability_id()));
        policy_checks.dedup_by(|left, right| left.capability_id() == right.capability_id());

        Self {
            command: "mcp-plan".to_string(),
            boundary,
            capabilities,
            policy_checks,
            context_export,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/mcp-integration-plan.md"),
                PathBuf::from(".monad/reports/mcp-integration-plan.json"),
                PathBuf::from(".monad/reports/mcp-context-export.json"),
            ],
            safety_notes: vec![
                "No MCP server connection is opened by Monad.".to_string(),
                "No external tools are invoked by Monad.".to_string(),
                "No subprocesses are executed by Monad.".to_string(),
                "No network access is performed by Monad.".to_string(),
                "Generated MCP evidence is written only when --yes is used.".to_string(),
            ],
        }
    }

    /// Boundary.
    #[must_use]
    pub const fn boundary(&self) -> &McpIntegrationBoundary {
        &self.boundary
    }

    /// Capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &[McpToolCapability] {
        &self.capabilities
    }

    /// Policy checks.
    #[must_use]
    pub fn policy_checks(&self) -> &[ExternalToolPolicyCheck] {
        &self.policy_checks
    }

    /// Context export.
    #[must_use]
    pub const fn context_export(&self) -> &McpContextExport {
        &self.context_export
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

/// Apply result for generated MCP integration evidence writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpIntegrationApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl McpIntegrationApplyResult {
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

/// Builds the MCP integration plan.
#[must_use]
pub fn build_mcp_integration_plan(root: impl AsRef<Path>) -> McpIntegrationPlan {
    let root = root.as_ref();
    let boundary = McpIntegrationBoundary::default_boundary();
    let capabilities = default_mcp_capabilities();
    let policy_checks = evaluate_external_tool_policy(&capabilities, &boundary);
    let context_export = build_mcp_context_export(root);

    McpIntegrationPlan::new(boundary, capabilities, policy_checks, context_export)
}

/// Built-in MCP/local tool capabilities for E31.
#[must_use]
pub fn default_mcp_capabilities() -> Vec<McpToolCapability> {
    vec![
        McpToolCapability::new(
            "context:repo-summary",
            McpCapabilityKind::ContextExport,
            "Export a read-only repository context summary.",
            true,
            false,
        ),
        McpToolCapability::new(
            "context:reports",
            McpCapabilityKind::ReportLookup,
            "Expose generated report paths for review.",
            true,
            false,
        ),
        McpToolCapability::new(
            "metadata:workspace",
            McpCapabilityKind::RepoMetadata,
            "Expose basic workspace metadata without executing tools.",
            true,
            false,
        ),
        McpToolCapability::new(
            "tool:external-invocation",
            McpCapabilityKind::ToolInvocation,
            "Placeholder for future supervised external tool invocation.",
            false,
            true,
        ),
    ]
}

/// Evaluates external tool policy checks.
#[must_use]
pub fn evaluate_external_tool_policy(
    capabilities: &[McpToolCapability],
    boundary: &McpIntegrationBoundary,
) -> Vec<ExternalToolPolicyCheck> {
    capabilities
        .iter()
        .map(|capability| {
            if capability.requires_invocation()
                || capability.kind() == McpCapabilityKind::ToolInvocation
            {
                ExternalToolPolicyCheck::new(
                    capability.id().to_string(),
                    ExternalToolPolicyDecision::Blocked,
                    "external tool invocation is disabled in E31",
                )
            } else if capability.read_only()
                && boundary.local_only()
                && boundary.network_disabled()
                && boundary.tool_invocation_disabled()
            {
                ExternalToolPolicyCheck::new(
                    capability.id().to_string(),
                    ExternalToolPolicyDecision::AllowReadOnly,
                    "read-only local capability is allowed for planning/export evidence",
                )
            } else {
                ExternalToolPolicyCheck::new(
                    capability.id().to_string(),
                    ExternalToolPolicyDecision::ReviewRequired,
                    "capability requires human review before exposure",
                )
            }
        })
        .collect()
}

/// Builds a local context export proof of concept.
#[must_use]
pub fn build_mcp_context_export(root: &Path) -> McpContextExport {
    let mut paths = vec![
        PathBuf::from("Cargo.toml"),
        PathBuf::from("monad.toml"),
        PathBuf::from("docs"),
        PathBuf::from(".monad/reports"),
    ];

    paths.retain(|path| root.join(path).exists());

    let summary = paths
        .iter()
        .map(|path| summarize_path(root, path))
        .collect::<Vec<_>>();

    McpContextExport::new(".", paths, summary)
}

/// Writes generated MCP integration evidence reports.
pub fn write_mcp_integration_evidence(
    root: impl AsRef<Path>,
) -> Result<McpIntegrationApplyResult, String> {
    let root = root.as_ref();
    let plan = build_mcp_integration_plan(root);
    let markdown = render_mcp_integration_plan(&plan);
    let json = render_mcp_integration_plan_json(&plan);
    let context_json = render_mcp_context_export_json(plan.context_export());

    let requests = [
        GatedWriteRequest::new(".monad/reports/mcp-integration-plan.md", markdown, true),
        GatedWriteRequest::new(".monad/reports/mcp-integration-plan.json", json, true),
        GatedWriteRequest::new(".monad/reports/mcp-context-export.json", context_json, true),
    ];

    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(McpIntegrationApplyResult::new(write_results))
}

/// Renders a text MCP integration plan.
#[must_use]
pub fn render_mcp_integration_plan(plan: &McpIntegrationPlan) -> String {
    let mut lines = vec![
        "Monad MCP and external tool integration plan".to_string(),
        String::new(),
        "Boundary:".to_string(),
        format!("  local_only: {}", plan.boundary().local_only()),
        format!("  network_disabled: {}", plan.boundary().network_disabled()),
        format!(
            "  tool_invocation_disabled: {}",
            plan.boundary().tool_invocation_disabled()
        ),
        format!(
            "  context_export_path: {}",
            plan.boundary().context_export_path().display()
        ),
        String::new(),
        "Boundary rules:".to_string(),
    ];

    for rule in plan.boundary().rules() {
        lines.push(format!("  - {rule}"));
    }

    lines.push(String::new());
    lines.push(format!("Capabilities: {}", plan.capabilities().len()));
    for capability in plan.capabilities() {
        lines.push(format!(
            "  - {} kind={} read_only={} requires_invocation={} description={}",
            capability.id(),
            capability.kind().as_str(),
            capability.read_only(),
            capability.requires_invocation(),
            capability.description()
        ));
    }

    lines.push(String::new());
    lines.push("Policy checks:".to_string());
    for check in plan.policy_checks() {
        lines.push(format!(
            "  - {} decision={} reason={}",
            check.capability_id(),
            check.decision().as_str(),
            check.reason()
        ));
    }

    lines.push(String::new());
    lines.push("Context export:".to_string());
    lines.push(format!(
        "  repository_root: {}",
        plan.context_export().repository_root().display()
    ));
    for summary in plan.context_export().summary() {
        lines.push(format!("  - {summary}"));
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

/// Renders a JSON MCP integration plan.
#[must_use]
pub fn render_mcp_integration_plan_json(plan: &McpIntegrationPlan) -> String {
    serde_json::to_string_pretty(plan).unwrap_or_else(|_| {
        "{\n  \"command\": \"mcp-plan\",\n  \"error\": \"MCP integration plan serialization failed\"\n}".to_string()
    })
}

/// Renders a JSON MCP context export.
#[must_use]
pub fn render_mcp_context_export_json(export: &McpContextExport) -> String {
    serde_json::to_string_pretty(export).unwrap_or_else(|_| {
        "{\n  \"error\": \"MCP context export serialization failed\"\n}".to_string()
    })
}

/// Renders generated evidence write results.
#[must_use]
pub fn render_mcp_integration_apply_result(result: &McpIntegrationApplyResult) -> String {
    let mut lines = vec![
        "Monad MCP integration evidence write result".to_string(),
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
    lines.push("No MCP server connection was opened.".to_string());
    lines.push("No external tools were invoked.".to_string());
    lines.push("No subprocesses were executed.".to_string());

    lines.join("\n")
}

fn summarize_path(root: &Path, path: &Path) -> String {
    let absolute = root.join(path);

    if absolute.is_dir() {
        let entry_count = fs::read_dir(&absolute).map_or(0, Iterator::count);
        format!("{} directory entries={entry_count}", path.display())
    } else {
        let byte_len = fs::metadata(&absolute).map_or(0, |metadata| metadata.len());
        format!("{} file bytes={byte_len}", path.display())
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
            "monad-mcp-integration-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);
        assert!(fs::create_dir_all(root.join(".monad/reports")).is_ok());
        assert!(fs::create_dir_all(root.join("docs")).is_ok());
        assert!(fs::write(root.join("Cargo.toml"), "[workspace]\n").is_ok());
        assert!(fs::write(root.join("monad.toml"), "schema_version = 1\n").is_ok());
        root
    }

    #[test]
    fn boundary_disables_network_and_tool_invocation() {
        let boundary = McpIntegrationBoundary::default_boundary();

        assert!(boundary.local_only());
        assert!(boundary.network_disabled());
        assert!(boundary.tool_invocation_disabled());
    }

    #[test]
    fn tool_invocation_capability_is_blocked() {
        let boundary = McpIntegrationBoundary::default_boundary();
        let capabilities = default_mcp_capabilities();
        let checks = evaluate_external_tool_policy(&capabilities, &boundary);

        assert!(checks.iter().any(|check| {
            check.capability_id() == "tool:external-invocation"
                && check.decision() == ExternalToolPolicyDecision::Blocked
        }));
    }

    #[test]
    fn readonly_capabilities_are_allowed_for_export() {
        let boundary = McpIntegrationBoundary::default_boundary();
        let capabilities = default_mcp_capabilities();
        let checks = evaluate_external_tool_policy(&capabilities, &boundary);

        assert!(checks.iter().any(|check| {
            check.capability_id() == "context:repo-summary"
                && check.decision() == ExternalToolPolicyDecision::AllowReadOnly
        }));
    }

    #[test]
    fn context_export_discovers_local_paths() {
        let root = create_workspace("context");
        let export = build_mcp_context_export(&root);

        assert!(
            export
                .exported_paths()
                .contains(&PathBuf::from("Cargo.toml"))
        );
        assert!(
            export
                .exported_paths()
                .contains(&PathBuf::from("monad.toml"))
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn text_render_mentions_no_external_tools() {
        let root = create_workspace("text");
        let plan = build_mcp_integration_plan(&root);
        let text = render_mcp_integration_plan(&plan);

        assert!(text.contains("Monad MCP and external tool integration plan"));
        assert!(text.contains("No external tools are invoked by Monad"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn json_render_contains_mcp_plan_command() {
        let root = create_workspace("json");
        let plan = build_mcp_integration_plan(&root);
        let json = render_mcp_integration_plan_json(&plan);

        assert!(json.contains("\"command\": \"mcp-plan\""));
        assert!(json.contains("context:repo-summary"));

        let _ = fs::remove_dir_all(root);
    }
}
