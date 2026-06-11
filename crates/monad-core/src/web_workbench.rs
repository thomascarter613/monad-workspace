//! Web Workbench Foundation.
//!
//! E35 defines Monad's local web workbench architecture, API contract, and
//! view model foundation. It is deterministic and evidence-only: it does not
//! start an HTTP server, open sockets, add browser/frontend dependencies, call
//! GitHub, access the network, or mutate user-owned source files.

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

/// Local web workbench route kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WebWorkbenchRouteKind {
    Dashboard,
    RepositoryGraph,
    WorkPackets,
    Reports,
    Approvals,
    Context,
    Health,
}

impl WebWorkbenchRouteKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dashboard => "dashboard",
            Self::RepositoryGraph => "repository-graph",
            Self::WorkPackets => "work-packets",
            Self::Reports => "reports",
            Self::Approvals => "approvals",
            Self::Context => "context",
            Self::Health => "health",
        }
    }
}

/// Local API method model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebWorkbenchApiMethod {
    Get,
    PostReviewOnly,
}

impl WebWorkbenchApiMethod {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::PostReviewOnly => "POST_REVIEW_ONLY",
        }
    }
}

/// Local web architecture boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WebWorkbenchArchitecture {
    host: String,
    port: u16,
    server_start_disabled: bool,
    network_disabled: bool,
    frontend_dependency_disabled: bool,
    rules: Vec<String>,
}

impl WebWorkbenchArchitecture {
    #[must_use]
    pub fn default_architecture() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8765,
            server_start_disabled: true,
            network_disabled: true,
            frontend_dependency_disabled: true,
            rules: vec![
                "E35 models a local web workbench contract but does not start a server."
                    .to_string(),
                "API endpoints are described as a deterministic local contract.".to_string(),
                "Views are rendered as metadata/evidence, not as a browser runtime.".to_string(),
                "Approval/context views are read-only planning artifacts.".to_string(),
            ],
        }
    }

    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }

    #[must_use]
    pub const fn server_start_disabled(&self) -> bool {
        self.server_start_disabled
    }

    #[must_use]
    pub const fn network_disabled(&self) -> bool {
        self.network_disabled
    }

    #[must_use]
    pub const fn frontend_dependency_disabled(&self) -> bool {
        self.frontend_dependency_disabled
    }

    #[must_use]
    pub fn rules(&self) -> &[String] {
        &self.rules
    }
}

/// API endpoint contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WebWorkbenchApiEndpoint {
    id: String,
    method: WebWorkbenchApiMethod,
    path: String,
    route: WebWorkbenchRouteKind,
    description: String,
    mutation_disabled: bool,
}

impl WebWorkbenchApiEndpoint {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        method: WebWorkbenchApiMethod,
        path: impl Into<String>,
        route: WebWorkbenchRouteKind,
        description: impl Into<String>,
        mutation_disabled: bool,
    ) -> Self {
        Self {
            id: id.into(),
            method,
            path: path.into(),
            route,
            description: description.into(),
            mutation_disabled,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub const fn method(&self) -> WebWorkbenchApiMethod {
        self.method
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub const fn route(&self) -> WebWorkbenchRouteKind {
        self.route
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub const fn mutation_disabled(&self) -> bool {
        self.mutation_disabled
    }
}

/// Repository graph view model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WebRepositoryGraphView {
    id: String,
    nodes: Vec<String>,
    edges: Vec<String>,
    source_paths: Vec<PathBuf>,
}

impl WebRepositoryGraphView {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        mut nodes: Vec<String>,
        mut edges: Vec<String>,
        mut source_paths: Vec<PathBuf>,
    ) -> Self {
        nodes.sort();
        nodes.dedup();
        edges.sort();
        edges.dedup();
        source_paths.sort();
        source_paths.dedup();

        Self {
            id: id.into(),
            nodes,
            edges,
            source_paths,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn nodes(&self) -> &[String] {
        &self.nodes
    }

    #[must_use]
    pub fn edges(&self) -> &[String] {
        &self.edges
    }

    #[must_use]
    pub fn source_paths(&self) -> &[PathBuf] {
        &self.source_paths
    }
}

/// Work-packet/report view model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WebWorkbenchListView {
    id: String,
    title: String,
    items: Vec<String>,
    read_only: bool,
}

impl WebWorkbenchListView {
    #[must_use]
    pub fn new(id: impl Into<String>, title: impl Into<String>, mut items: Vec<String>) -> Self {
        items.sort();
        items.dedup();

        Self {
            id: id.into(),
            title: title.into(),
            items,
            read_only: true,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub fn items(&self) -> &[String] {
        &self.items
    }

    #[must_use]
    pub const fn read_only(&self) -> bool {
        self.read_only
    }
}

/// Approval/context viewer foundation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WebApprovalContextView {
    id: String,
    title: String,
    context_sources: Vec<PathBuf>,
    approval_required: bool,
    mutation_disabled: bool,
    notes: Vec<String>,
}

impl WebApprovalContextView {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        mut context_sources: Vec<PathBuf>,
        mut notes: Vec<String>,
    ) -> Self {
        context_sources.sort();
        context_sources.dedup();
        notes.sort();
        notes.dedup();

        Self {
            id: id.into(),
            title: title.into(),
            context_sources,
            approval_required: true,
            mutation_disabled: true,
            notes,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub fn context_sources(&self) -> &[PathBuf] {
        &self.context_sources
    }

    #[must_use]
    pub const fn approval_required(&self) -> bool {
        self.approval_required
    }

    #[must_use]
    pub const fn mutation_disabled(&self) -> bool {
        self.mutation_disabled
    }

    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }
}

/// Full local web workbench plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WebWorkbenchPlan {
    command: String,
    architecture: WebWorkbenchArchitecture,
    endpoints: Vec<WebWorkbenchApiEndpoint>,
    repository_graph: WebRepositoryGraphView,
    work_packet_view: WebWorkbenchListView,
    report_view: WebWorkbenchListView,
    approval_context_view: WebApprovalContextView,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl WebWorkbenchPlan {
    #[must_use]
    pub fn new(
        architecture: WebWorkbenchArchitecture,
        mut endpoints: Vec<WebWorkbenchApiEndpoint>,
        repository_graph: WebRepositoryGraphView,
        work_packet_view: WebWorkbenchListView,
        report_view: WebWorkbenchListView,
        approval_context_view: WebApprovalContextView,
    ) -> Self {
        endpoints.sort_by(|left, right| left.path().cmp(right.path()));

        Self {
            command: "web-workbench-plan".to_string(),
            architecture,
            endpoints,
            repository_graph,
            work_packet_view,
            report_view,
            approval_context_view,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/web-workbench-plan.md"),
                PathBuf::from(".monad/reports/web-workbench-plan.json"),
            ],
            safety_notes: vec![
                "No HTTP server is started by Monad.".to_string(),
                "No socket is opened by Monad.".to_string(),
                "No browser/frontend dependency is added by Monad.".to_string(),
                "No GitHub or network API call is made by Monad.".to_string(),
                "Generated web workbench evidence is written only when --yes is used.".to_string(),
            ],
        }
    }

    #[must_use]
    pub const fn architecture(&self) -> &WebWorkbenchArchitecture {
        &self.architecture
    }

    #[must_use]
    pub fn endpoints(&self) -> &[WebWorkbenchApiEndpoint] {
        &self.endpoints
    }

    #[must_use]
    pub const fn repository_graph(&self) -> &WebRepositoryGraphView {
        &self.repository_graph
    }

    #[must_use]
    pub const fn work_packet_view(&self) -> &WebWorkbenchListView {
        &self.work_packet_view
    }

    #[must_use]
    pub const fn report_view(&self) -> &WebWorkbenchListView {
        &self.report_view
    }

    #[must_use]
    pub const fn approval_context_view(&self) -> &WebApprovalContextView {
        &self.approval_context_view
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

/// Apply result for generated web workbench evidence writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebWorkbenchApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl WebWorkbenchApplyResult {
    #[must_use]
    pub fn new(write_results: Vec<GatedWriteResult>) -> Self {
        Self { write_results }
    }

    #[must_use]
    pub fn write_results(&self) -> &[GatedWriteResult] {
        &self.write_results
    }
}

/// Builds the E35 web workbench foundation plan.
#[must_use]
pub fn build_web_workbench_plan(_root: impl AsRef<Path>) -> WebWorkbenchPlan {
    let architecture = WebWorkbenchArchitecture::default_architecture();
    let endpoints = vec![
        WebWorkbenchApiEndpoint::new(
            "api:health",
            WebWorkbenchApiMethod::Get,
            "/api/health",
            WebWorkbenchRouteKind::Health,
            "Return local web workbench health metadata.",
            true,
        ),
        WebWorkbenchApiEndpoint::new(
            "api:graph",
            WebWorkbenchApiMethod::Get,
            "/api/repository-graph",
            WebWorkbenchRouteKind::RepositoryGraph,
            "Return repository graph view model.",
            true,
        ),
        WebWorkbenchApiEndpoint::new(
            "api:work-packets",
            WebWorkbenchApiMethod::Get,
            "/api/work-packets",
            WebWorkbenchRouteKind::WorkPackets,
            "Return work-packet list view model.",
            true,
        ),
        WebWorkbenchApiEndpoint::new(
            "api:reports",
            WebWorkbenchApiMethod::Get,
            "/api/reports",
            WebWorkbenchRouteKind::Reports,
            "Return local report list view model.",
            true,
        ),
        WebWorkbenchApiEndpoint::new(
            "api:approval-review",
            WebWorkbenchApiMethod::PostReviewOnly,
            "/api/approvals/review",
            WebWorkbenchRouteKind::Approvals,
            "Describe future approval review action without mutating state.",
            true,
        ),
    ];

    let repository_graph = WebRepositoryGraphView::new(
        "graph:repository",
        vec![
            "crates/monad-core".to_string(),
            "crates/monad-cli".to_string(),
            "docs".to_string(),
            "tools/scripts".to_string(),
        ],
        vec![
            "monad-cli -> monad-core".to_string(),
            "tools/scripts -> monad-cli".to_string(),
            "docs -> roadmap".to_string(),
        ],
        vec![
            PathBuf::from("crates/monad-core"),
            PathBuf::from("crates/monad-cli"),
            PathBuf::from("docs"),
        ],
    );

    let work_packet_view = WebWorkbenchListView::new(
        "view:work-packets",
        "E35 Work Packets",
        vec![
            "WP-E35-001 — Define local web workbench architecture".to_string(),
            "WP-E35-002 — Add local server/API foundation".to_string(),
            "WP-E35-003 — Add repository graph view".to_string(),
            "WP-E35-004 — Add work-packet and report views".to_string(),
            "WP-E35-005 — Add approval/context viewer foundation".to_string(),
            "WP-E35-006 — Add web workbench smoke tests".to_string(),
        ],
    );

    let report_view = WebWorkbenchListView::new(
        "view:reports",
        "Local Reports",
        vec![
            ".monad/reports/web-workbench-plan.md".to_string(),
            ".monad/reports/web-workbench-plan.json".to_string(),
            ".monad/reports/interactive-workbench-plan.md".to_string(),
        ],
    );

    let approval_context_view = WebApprovalContextView::new(
        "view:approval-context",
        "Approval and Context Viewer",
        vec![
            PathBuf::from(".monad/reports"),
            PathBuf::from("docs/roadmap/epic-35-web-workbench-foundation.md"),
        ],
        vec![
            "Approval actions are described but not executed.".to_string(),
            "Context viewer is read-only.".to_string(),
            "Mutations require a future explicit approval path.".to_string(),
        ],
    );

    WebWorkbenchPlan::new(
        architecture,
        endpoints,
        repository_graph,
        work_packet_view,
        report_view,
        approval_context_view,
    )
}

/// Writes generated web workbench evidence.
pub fn write_web_workbench_evidence(
    root: impl AsRef<Path>,
) -> Result<WebWorkbenchApplyResult, String> {
    let root = root.as_ref();
    let plan = build_web_workbench_plan(root);
    let requests = [
        GatedWriteRequest::new(
            ".monad/reports/web-workbench-plan.md",
            render_web_workbench_plan(&plan),
            true,
        ),
        GatedWriteRequest::new(
            ".monad/reports/web-workbench-plan.json",
            render_web_workbench_plan_json(&plan),
            true,
        ),
    ];

    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(WebWorkbenchApplyResult::new(write_results))
}

/// Renders a text web workbench plan.
#[must_use]
pub fn render_web_workbench_plan(plan: &WebWorkbenchPlan) -> String {
    let mut lines = vec![
        "Monad web workbench foundation plan".to_string(),
        String::new(),
        "Local web workbench architecture:".to_string(),
        format!("  host: {}", plan.architecture().host()),
        format!("  port: {}", plan.architecture().port()),
        format!(
            "  server_start_disabled: {}",
            plan.architecture().server_start_disabled()
        ),
        format!(
            "  network_disabled: {}",
            plan.architecture().network_disabled()
        ),
        format!(
            "  frontend_dependency_disabled: {}",
            plan.architecture().frontend_dependency_disabled()
        ),
        String::new(),
        "Architecture rules:".to_string(),
    ];

    for rule in plan.architecture().rules() {
        lines.push(format!("  - {rule}"));
    }

    lines.push(String::new());
    lines.push("Local server/API foundation:".to_string());
    for endpoint in plan.endpoints() {
        lines.push(format!(
            "  - {} {} route={} mutation_disabled={} description={}",
            endpoint.method().as_str(),
            endpoint.path(),
            endpoint.route().as_str(),
            endpoint.mutation_disabled(),
            endpoint.description()
        ));
    }

    lines.push(String::new());
    lines.push("Repository graph view:".to_string());
    lines.push(format!("  id: {}", plan.repository_graph().id()));
    for node in plan.repository_graph().nodes() {
        lines.push(format!("  - node: {node}"));
    }
    for edge in plan.repository_graph().edges() {
        lines.push(format!("  - edge: {edge}"));
    }

    lines.push(String::new());
    lines.push("Work-packet view:".to_string());
    lines.push(format!("  title: {}", plan.work_packet_view().title()));
    for item in plan.work_packet_view().items() {
        lines.push(format!("  - {item}"));
    }

    lines.push(String::new());
    lines.push("Report view:".to_string());
    for item in plan.report_view().items() {
        lines.push(format!("  - {item}"));
    }

    lines.push(String::new());
    lines.push("Approval/context viewer foundation:".to_string());
    lines.push(format!("  title: {}", plan.approval_context_view().title()));
    lines.push(format!(
        "  approval_required: {}",
        plan.approval_context_view().approval_required()
    ));
    lines.push(format!(
        "  mutation_disabled: {}",
        plan.approval_context_view().mutation_disabled()
    ));
    for source in plan.approval_context_view().context_sources() {
        lines.push(format!("  - context_source: {}", source.display()));
    }
    for note in plan.approval_context_view().notes() {
        lines.push(format!("  - note: {note}"));
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

/// Renders a JSON web workbench plan.
#[must_use]
pub fn render_web_workbench_plan_json(plan: &WebWorkbenchPlan) -> String {
    serde_json::to_string_pretty(plan).unwrap_or_else(|_| {
        "{\n  \"command\": \"web-workbench-plan\",\n  \"error\": \"web workbench plan serialization failed\"\n}".to_string()
    })
}

/// Renders generated evidence write results.
#[must_use]
pub fn render_web_workbench_apply_result(result: &WebWorkbenchApplyResult) -> String {
    let mut lines = vec![
        "Monad web workbench evidence write result".to_string(),
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
    lines.push("No HTTP server was started.".to_string());
    lines.push("No socket was opened.".to_string());
    lines.push("No browser/frontend dependency was added.".to_string());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn architecture_does_not_start_server_or_network() {
        let architecture = WebWorkbenchArchitecture::default_architecture();

        assert!(architecture.server_start_disabled());
        assert!(architecture.network_disabled());
        assert!(architecture.frontend_dependency_disabled());
    }

    #[test]
    fn api_contract_contains_repository_graph_endpoint() {
        let plan = build_web_workbench_plan(".");

        assert!(plan.endpoints().iter().any(|endpoint| {
            endpoint.path() == "/api/repository-graph"
                && endpoint.route() == WebWorkbenchRouteKind::RepositoryGraph
        }));
    }

    #[test]
    fn repository_graph_view_contains_core_and_cli_nodes() {
        let plan = build_web_workbench_plan(".");

        assert!(
            plan.repository_graph()
                .nodes()
                .contains(&"crates/monad-core".to_string())
        );
        assert!(
            plan.repository_graph()
                .nodes()
                .contains(&"crates/monad-cli".to_string())
        );
    }

    #[test]
    fn work_packet_view_contains_six_work_packets() {
        let plan = build_web_workbench_plan(".");

        assert_eq!(plan.work_packet_view().items().len(), 6);
        assert!(plan.work_packet_view().read_only());
    }

    #[test]
    fn approval_context_view_is_read_only() {
        let plan = build_web_workbench_plan(".");

        assert!(plan.approval_context_view().approval_required());
        assert!(plan.approval_context_view().mutation_disabled());
    }

    #[test]
    fn text_render_mentions_web_workbench_sections() {
        let plan = build_web_workbench_plan(".");
        let text = render_web_workbench_plan(&plan);

        assert!(text.contains("Monad web workbench foundation plan"));
        assert!(text.contains("Local server/API foundation"));
        assert!(text.contains("Approval/context viewer foundation"));
    }

    #[test]
    fn json_render_contains_web_workbench_command() {
        let plan = build_web_workbench_plan(".");
        let json = render_web_workbench_plan_json(&plan);

        assert!(json.contains("\"command\": \"web-workbench-plan\""));
        assert!(json.contains("/api/repository-graph"));
    }
}
