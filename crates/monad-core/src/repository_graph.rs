//! Repository graph primitives and renderers for Monad.
//!
//! WP-E2-007 introduced the first internal graph model.
//! WP-E2-008 adds deterministic rendering formats while keeping graph
//! construction separate from graph rendering.
//!
//! Supported graph render formats:
//!
//! - text: human-readable summary;
//! - json: machine-readable graph payload;
//! - mermaid: Markdown-friendly flowchart output;
//! - dot: Graphviz-compatible directed graph output.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::json;

use crate::{
    MonadError, MonadResult, RepositoryBoundedTraversal, RepositoryEntryCategory,
    RepositoryEntryKind, RepositoryEntryRole, RepositoryTraversalDecision,
};

/// Stable node kind for the repository graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryGraphNodeKind {
    /// Synthetic graph node representing the workspace root.
    WorkspaceRoot,

    /// Graph node representing a filesystem entry discovered during traversal.
    RepositoryEntry,
}

impl RepositoryGraphNodeKind {
    /// Returns a stable node-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace_root",
            Self::RepositoryEntry => "repository_entry",
        }
    }
}

/// Stable edge kind for the repository graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryGraphEdgeKind {
    /// Parent-child containment relationship.
    Contains,
}

impl RepositoryGraphEdgeKind {
    /// Returns a stable edge-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Contains => "contains",
        }
    }
}

/// Supported graph rendering formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RepositoryGraphRenderFormat {
    /// Human-readable graph summary.
    #[default]
    Text,

    /// Machine-readable JSON graph.
    Json,

    /// Mermaid flowchart graph.
    Mermaid,

    /// DOT / Graphviz directed graph.
    Dot,
}

impl RepositoryGraphRenderFormat {
    /// Returns a stable render format label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Json => "json",
            Self::Mermaid => "mermaid",
            Self::Dot => "dot",
        }
    }

    /// Parses a graph render format.
    pub fn parse(value: &str) -> MonadResult<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            "mermaid" | "mmd" => Ok(Self::Mermaid),
            "dot" | "graphviz" => Ok(Self::Dot),
            other => Err(MonadError::invalid_input(format!(
                "unsupported repository graph render format: {other}"
            ))),
        }
    }
}

/// One graph node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryGraphNode {
    id: String,
    label: String,
    relative_path: Option<PathBuf>,
    depth: usize,
    node_kind: RepositoryGraphNodeKind,
    entry_kind: Option<RepositoryEntryKind>,
    category: Option<RepositoryEntryCategory>,
    role: Option<RepositoryEntryRole>,
    traversal_decision: Option<RepositoryTraversalDecision>,
}

impl RepositoryGraphNode {
    /// Creates the synthetic workspace-root node.
    #[must_use]
    pub fn workspace_root(root: &Path) -> Self {
        Self {
            id: "root".to_string(),
            label: root.display().to_string(),
            relative_path: None,
            depth: 0,
            node_kind: RepositoryGraphNodeKind::WorkspaceRoot,
            entry_kind: None,
            category: None,
            role: None,
            traversal_decision: None,
        }
    }

    /// Creates a graph node from bounded traversal metadata.
    #[must_use]
    pub fn repository_entry(
        relative_path: impl Into<PathBuf>,
        depth: usize,
        kind: RepositoryEntryKind,
        category: RepositoryEntryCategory,
        role: RepositoryEntryRole,
        traversal_decision: RepositoryTraversalDecision,
    ) -> Self {
        let relative_path = relative_path.into();
        let label = relative_path.display().to_string();

        Self {
            id: graph_node_id_for_path(&relative_path),
            label,
            relative_path: Some(relative_path),
            depth,
            node_kind: RepositoryGraphNodeKind::RepositoryEntry,
            entry_kind: Some(kind),
            category: Some(category),
            role: Some(role),
            traversal_decision: Some(traversal_decision),
        }
    }

    /// Returns the stable node id.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the node label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the node's relative path when this node represents a repository entry.
    #[must_use]
    pub fn relative_path(&self) -> Option<&Path> {
        self.relative_path.as_deref()
    }

    /// Returns the node depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the graph node kind.
    #[must_use]
    pub const fn node_kind(&self) -> RepositoryGraphNodeKind {
        self.node_kind
    }

    /// Returns the filesystem entry kind when available.
    #[must_use]
    pub const fn entry_kind(&self) -> Option<RepositoryEntryKind> {
        self.entry_kind
    }

    /// Returns the broad category when available.
    #[must_use]
    pub const fn category(&self) -> Option<RepositoryEntryCategory> {
        self.category
    }

    /// Returns the precise role when available.
    #[must_use]
    pub const fn role(&self) -> Option<RepositoryEntryRole> {
        self.role
    }

    /// Returns the traversal decision when available.
    #[must_use]
    pub const fn traversal_decision(&self) -> Option<RepositoryTraversalDecision> {
        self.traversal_decision
    }
}

/// One graph edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryGraphEdge {
    from_id: String,
    to_id: String,
    edge_kind: RepositoryGraphEdgeKind,
}

impl RepositoryGraphEdge {
    /// Creates a graph edge.
    #[must_use]
    pub fn new(
        from_id: impl Into<String>,
        to_id: impl Into<String>,
        edge_kind: RepositoryGraphEdgeKind,
    ) -> Self {
        Self {
            from_id: from_id.into(),
            to_id: to_id.into(),
            edge_kind,
        }
    }

    /// Returns the source node id.
    #[must_use]
    pub fn from_id(&self) -> &str {
        &self.from_id
    }

    /// Returns the target node id.
    #[must_use]
    pub fn to_id(&self) -> &str {
        &self.to_id
    }

    /// Returns the edge kind.
    #[must_use]
    pub const fn edge_kind(&self) -> RepositoryGraphEdgeKind {
        self.edge_kind
    }
}

/// Repository graph derived from bounded traversal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryGraph {
    root: PathBuf,
    nodes: Vec<RepositoryGraphNode>,
    edges: Vec<RepositoryGraphEdge>,
}

impl RepositoryGraph {
    /// Creates a repository graph.
    #[must_use]
    pub fn new(
        root: impl Into<PathBuf>,
        nodes: Vec<RepositoryGraphNode>,
        edges: Vec<RepositoryGraphEdge>,
    ) -> Self {
        Self {
            root: root.into(),
            nodes,
            edges,
        }
    }

    /// Returns the graph root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns graph nodes.
    #[must_use]
    pub fn nodes(&self) -> &[RepositoryGraphNode] {
        &self.nodes
    }

    /// Returns graph edges.
    #[must_use]
    pub fn edges(&self) -> &[RepositoryGraphEdge] {
        &self.edges
    }

    /// Returns the number of nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns the maximum observed node depth.
    #[must_use]
    pub fn max_depth(&self) -> usize {
        self.nodes
            .iter()
            .map(RepositoryGraphNode::depth)
            .max()
            .unwrap_or(0)
    }

    /// Returns true when the graph contains a node for a relative path.
    #[must_use]
    pub fn contains_relative_path(&self, relative_path: impl AsRef<Path>) -> bool {
        let relative_path = relative_path.as_ref();

        self.nodes
            .iter()
            .any(|node| node.relative_path() == Some(relative_path))
    }

    /// Counts nodes by broad repository category.
    #[must_use]
    pub fn category_counts(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();

        for node in &self.nodes {
            if let Some(category) = node.category() {
                *counts.entry(category.as_str().to_string()).or_insert(0) += 1;
            }
        }

        counts
    }

    /// Counts nodes by traversal decision.
    #[must_use]
    pub fn traversal_decision_counts(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();

        for node in &self.nodes {
            if let Some(decision) = node.traversal_decision() {
                *counts.entry(decision.as_str().to_string()).or_insert(0) += 1;
            }
        }

        counts
    }
}

/// Builds a repository graph from bounded traversal output.
///
/// This function is deterministic. Nodes and edges are sorted by stable ids.
#[must_use]
pub fn build_repository_graph(traversal: &RepositoryBoundedTraversal) -> RepositoryGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut known_node_ids = BTreeSet::new();

    let root_node = RepositoryGraphNode::workspace_root(traversal.root());
    known_node_ids.insert(root_node.id().to_string());
    nodes.push(root_node);

    for traversal_entry in traversal.entries() {
        let node = RepositoryGraphNode::repository_entry(
            traversal_entry.relative_path().to_path_buf(),
            traversal_entry.depth(),
            traversal_entry.kind(),
            traversal_entry.category(),
            traversal_entry.role(),
            traversal_entry.decision(),
        );

        known_node_ids.insert(node.id().to_string());
        nodes.push(node);
    }

    for traversal_entry in traversal.entries() {
        let child_path = traversal_entry.relative_path();
        let child_id = graph_node_id_for_path(child_path);

        let parent_id = parent_node_id_for_path(child_path);

        let from_id = if known_node_ids.contains(&parent_id) {
            parent_id
        } else {
            "root".to_string()
        };

        edges.push(RepositoryGraphEdge::new(
            from_id,
            child_id,
            RepositoryGraphEdgeKind::Contains,
        ));
    }

    nodes.sort_by(|left, right| left.id().cmp(right.id()));
    edges.sort_by(|left, right| {
        left.from_id()
            .cmp(right.from_id())
            .then_with(|| left.to_id().cmp(right.to_id()))
    });

    RepositoryGraph::new(traversal.root().to_path_buf(), nodes, edges)
}

/// Renders a repository graph in a supported format.
#[must_use]
pub fn render_repository_graph(
    graph: &RepositoryGraph,
    format: RepositoryGraphRenderFormat,
) -> String {
    match format {
        RepositoryGraphRenderFormat::Text => render_repository_graph_text(graph),
        RepositoryGraphRenderFormat::Json => render_repository_graph_json(graph),
        RepositoryGraphRenderFormat::Mermaid => render_repository_graph_mermaid(graph),
        RepositoryGraphRenderFormat::Dot => render_repository_graph_dot(graph),
    }
}

/// Renders a human-readable graph summary.
fn render_repository_graph_text(graph: &RepositoryGraph) -> String {
    let mut lines = vec![
        "Monad repository graph".to_string(),
        format!("  root: {}", graph.root().display()),
        format!("  nodes: {}", graph.node_count()),
        format!("  edges: {}", graph.edge_count()),
        format!("  max_depth: {}", graph.max_depth()),
        "  category_counts:".to_string(),
    ];

    for (category, count) in graph.category_counts() {
        lines.push(format!("    {category}: {count}"));
    }

    lines.push("  traversal_decision_counts:".to_string());

    for (decision, count) in graph.traversal_decision_counts() {
        lines.push(format!("    {decision}: {count}"));
    }

    lines.push("  nodes:".to_string());

    for node in graph.nodes() {
        lines.push(format!(
            "    - id={} label={} kind={} depth={}",
            node.id(),
            node.label(),
            node.node_kind().as_str(),
            node.depth()
        ));
    }

    lines.push("  edges:".to_string());

    for edge in graph.edges() {
        lines.push(format!(
            "    - {} -[{}]-> {}",
            edge.from_id(),
            edge.edge_kind().as_str(),
            edge.to_id()
        ));
    }

    lines.join("\n")
}

/// Renders a machine-readable JSON graph.
fn render_repository_graph_json(graph: &RepositoryGraph) -> String {
    let nodes = graph
        .nodes()
        .iter()
        .map(|node| {
            json!({
                "id": node.id(),
                "label": node.label(),
                "relative_path": node.relative_path().map(|path| path.display().to_string()),
                "depth": node.depth(),
                "node_kind": node.node_kind().as_str(),
                "entry_kind": node.entry_kind().map(RepositoryEntryKind::as_str),
                "category": node.category().map(RepositoryEntryCategory::as_str),
                "role": node.role().map(RepositoryEntryRole::as_str),
                "traversal_decision": node.traversal_decision().map(RepositoryTraversalDecision::as_str),
            })
        })
        .collect::<Vec<_>>();

    let edges = graph
        .edges()
        .iter()
        .map(|edge| {
            json!({
                "from_id": edge.from_id(),
                "to_id": edge.to_id(),
                "edge_kind": edge.edge_kind().as_str(),
            })
        })
        .collect::<Vec<_>>();

    serde_json::to_string_pretty(&json!({
        "kind": "repository_graph",
        "format": RepositoryGraphRenderFormat::Json.as_str(),
        "root": graph.root().display().to_string(),
        "node_count": graph.node_count(),
        "edge_count": graph.edge_count(),
        "max_depth": graph.max_depth(),
        "category_counts": graph.category_counts(),
        "traversal_decision_counts": graph.traversal_decision_counts(),
        "nodes": nodes,
        "edges": edges,
    }))
    .unwrap_or_else(|error| {
        json!({
            "kind": "repository_graph",
            "format": RepositoryGraphRenderFormat::Json.as_str(),
            "error": {
                "code": "MONAD-GRAPH-OUTPUT-0001",
                "message": format!("failed to serialize repository graph JSON: {error}")
            }
        })
        .to_string()
    })
}

/// Renders a Mermaid flowchart.
fn render_repository_graph_mermaid(graph: &RepositoryGraph) -> String {
    let mut lines = vec![
        "flowchart TD".to_string(),
        "  %% Monad repository graph".to_string(),
    ];

    for node in graph.nodes() {
        lines.push(format!(
            "  {}[\"{}\"]",
            mermaid_node_id(node.id()),
            escape_mermaid_label(node.label())
        ));
    }

    for edge in graph.edges() {
        lines.push(format!(
            "  {} --> {}",
            mermaid_node_id(edge.from_id()),
            mermaid_node_id(edge.to_id())
        ));
    }

    lines.join("\n")
}

/// Renders a DOT / Graphviz graph.
fn render_repository_graph_dot(graph: &RepositoryGraph) -> String {
    let mut lines = vec![
        "digraph repository {".to_string(),
        "  graph [label=\"Monad repository graph\"];".to_string(),
        "  node [shape=box];".to_string(),
    ];

    for node in graph.nodes() {
        lines.push(format!(
            "  \"{}\" [label=\"{}\", kind=\"{}\", depth=\"{}\"];",
            escape_dot_string(node.id()),
            escape_dot_string(node.label()),
            node.node_kind().as_str(),
            node.depth()
        ));
    }

    for edge in graph.edges() {
        lines.push(format!(
            "  \"{}\" -> \"{}\" [label=\"{}\"];",
            escape_dot_string(edge.from_id()),
            escape_dot_string(edge.to_id()),
            edge.edge_kind().as_str()
        ));
    }

    lines.push("}".to_string());
    lines.join("\n")
}

/// Returns a stable graph node id for a relative path.
fn graph_node_id_for_path(path: &Path) -> String {
    format!("path:{}", normalize_relative_path(path))
}

/// Returns the parent graph node id for a relative path.
fn parent_node_id_for_path(path: &Path) -> String {
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => graph_node_id_for_path(parent),
        _ => "root".to_string(),
    }
}

/// Normalizes a relative path to a slash-separated string.
fn normalize_relative_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("/")
}

/// Converts arbitrary graph ids into Mermaid-safe ids.
fn mermaid_node_id(id: &str) -> String {
    let sanitized = id
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();

    format!("n_{sanitized}")
}

/// Escapes a Mermaid node label.
fn escape_mermaid_label(label: &str) -> String {
    label.replace('"', "'").replace('\n', " ")
}

/// Escapes DOT string values.
fn escape_dot_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{WorkspaceContext, inspect_workspace, traverse_workspace_bounded};

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-repository-graph-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_graph_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("docs/guide")).expect("docs should be created");
        fs::create_dir_all(root.join("crates/monad-core/src"))
            .expect("crate source should be created");
        fs::create_dir_all(root.join("target/debug")).expect("target should be created");

        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("monad.toml should be written");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("README.md"), "# Monad\n").expect("README should be written");
        fs::write(root.join("docs/guide/intro.md"), "# Intro\n").expect("intro should be written");
        fs::write(
            root.join("crates/monad-core/src/lib.rs"),
            "pub fn test() {}\n",
        )
        .expect("lib should be written");
        fs::write(root.join("target/debug/cache.bin"), "cache\n").expect("cache should be written");

        root
    }

    fn build_test_graph(test_name: &str) -> (PathBuf, RepositoryGraph) {
        let root = create_graph_workspace(test_name);
        let context = WorkspaceContext::new(&root).expect("context should be created");
        let inspection = inspect_workspace(&context).expect("inspection should run");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should run");
        let graph = build_repository_graph(&traversal);

        (root, graph)
    }

    #[test]
    fn graph_contains_root_and_traversed_entries() {
        let (root, graph) = build_test_graph("contains-root");

        assert!(graph.node_count() > 0);
        assert!(graph.edge_count() > 0);
        assert!(graph.contains_relative_path("monad.toml"));
        assert!(graph.contains_relative_path("docs/guide/intro.md"));
        assert!(graph.contains_relative_path("crates/monad-core/src/lib.rs"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn graph_edges_connect_parent_child_relationships() {
        let (root, graph) = build_test_graph("edges");

        assert!(graph.edges().iter().any(|edge| {
            edge.from_id() == "root"
                && edge.to_id() == "path:docs"
                && edge.edge_kind() == RepositoryGraphEdgeKind::Contains
        }));

        assert!(graph.edges().iter().any(|edge| {
            edge.from_id() == "path:docs/guide"
                && edge.to_id() == "path:docs/guide/intro.md"
                && edge.edge_kind() == RepositoryGraphEdgeKind::Contains
        }));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn graph_exposes_category_and_decision_counts() {
        let (root, graph) = build_test_graph("counts");
        let category_counts = graph.category_counts();
        let decision_counts = graph.traversal_decision_counts();

        assert!(category_counts.contains_key("monad_control"));
        assert!(category_counts.contains_key("source"));
        assert!(category_counts.contains_key("documentation"));
        assert!(category_counts.contains_key("generated_or_external"));

        assert!(decision_counts.contains_key("candidate_for_future_traversal"));
        assert!(decision_counts.contains_key("inspect_shallow_only"));
        assert!(decision_counts.contains_key("skip_by_default"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn graph_render_format_parses_supported_formats() {
        assert_eq!(
            RepositoryGraphRenderFormat::parse("text"),
            Ok(RepositoryGraphRenderFormat::Text)
        );
        assert_eq!(
            RepositoryGraphRenderFormat::parse("json"),
            Ok(RepositoryGraphRenderFormat::Json)
        );
        assert_eq!(
            RepositoryGraphRenderFormat::parse("mermaid"),
            Ok(RepositoryGraphRenderFormat::Mermaid)
        );
        assert_eq!(
            RepositoryGraphRenderFormat::parse("mmd"),
            Ok(RepositoryGraphRenderFormat::Mermaid)
        );
        assert_eq!(
            RepositoryGraphRenderFormat::parse("dot"),
            Ok(RepositoryGraphRenderFormat::Dot)
        );
        assert_eq!(
            RepositoryGraphRenderFormat::parse("graphviz"),
            Ok(RepositoryGraphRenderFormat::Dot)
        );
    }

    #[test]
    fn graph_render_format_rejects_unsupported_formats() {
        let error = RepositoryGraphRenderFormat::parse("svg")
            .expect_err("svg is not a supported graph render format yet");

        assert_eq!(error.code(), "MONAD2001");
        assert!(
            error
                .message()
                .contains("unsupported repository graph render format")
        );
    }

    #[test]
    fn graph_renders_as_text() {
        let (root, graph) = build_test_graph("render-text");

        let rendered = render_repository_graph(&graph, RepositoryGraphRenderFormat::Text);

        assert!(rendered.contains("Monad repository graph"));
        assert!(rendered.contains("nodes:"));
        assert!(rendered.contains("edges:"));
        assert!(rendered.contains("category_counts:"));
        assert!(rendered.contains("traversal_decision_counts:"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn graph_renders_as_json() {
        let (root, graph) = build_test_graph("render-json");

        let rendered = render_repository_graph(&graph, RepositoryGraphRenderFormat::Json);

        assert!(rendered.contains(r#""kind": "repository_graph""#));
        assert!(rendered.contains(r#""format": "json""#));
        assert!(rendered.contains(r#""node_count""#));
        assert!(rendered.contains(r#""edge_count""#));
        assert!(rendered.contains(r#""nodes""#));
        assert!(rendered.contains(r#""edges""#));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn graph_renders_as_mermaid() {
        let (root, graph) = build_test_graph("render-mermaid");

        let rendered = render_repository_graph(&graph, RepositoryGraphRenderFormat::Mermaid);

        assert!(rendered.starts_with("flowchart TD"));
        assert!(rendered.contains("%% Monad repository graph"));
        assert!(rendered.contains("-->"));
        assert!(rendered.contains("n_root"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn graph_renders_as_dot() {
        let (root, graph) = build_test_graph("render-dot");

        let rendered = render_repository_graph(&graph, RepositoryGraphRenderFormat::Dot);

        assert!(rendered.starts_with("digraph repository"));
        assert!(rendered.contains("node [shape=box]"));
        assert!(rendered.contains("->"));
        assert!(rendered.contains("[label=\"contains\"]"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn graph_rendering_is_deterministic() {
        let (root, graph) = build_test_graph("render-deterministic");

        let first = render_repository_graph(&graph, RepositoryGraphRenderFormat::Json);
        let second = render_repository_graph(&graph, RepositoryGraphRenderFormat::Json);

        assert_eq!(first, second);

        let first_mermaid = render_repository_graph(&graph, RepositoryGraphRenderFormat::Mermaid);
        let second_mermaid = render_repository_graph(&graph, RepositoryGraphRenderFormat::Mermaid);

        assert_eq!(first_mermaid, second_mermaid);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn graph_output_is_deterministically_ordered() {
        let (root, graph) = build_test_graph("deterministic");

        let node_ids = graph
            .nodes()
            .iter()
            .map(|node| node.id().to_string())
            .collect::<Vec<_>>();

        let mut sorted_node_ids = node_ids.clone();
        sorted_node_ids.sort();

        assert_eq!(node_ids, sorted_node_ids);

        let edge_pairs = graph
            .edges()
            .iter()
            .map(|edge| format!("{}->{}", edge.from_id(), edge.to_id()))
            .collect::<Vec<_>>();

        let mut sorted_edge_pairs = edge_pairs.clone();
        sorted_edge_pairs.sort();

        assert_eq!(edge_pairs, sorted_edge_pairs);

        fs::remove_dir_all(root).ok();
    }
}
