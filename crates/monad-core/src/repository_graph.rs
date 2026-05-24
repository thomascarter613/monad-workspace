//! Repository graph primitives for Monad.
//!
//! WP-E2-007 introduces the first graph model for repository intelligence.
//!
//! The graph is built from the bounded traversal foundation introduced in
//! WP-E2-006. It does not yet render Mermaid, DOT, or external graph formats.
//! That comes later. This slice only creates a durable internal graph model
//! that future commands and renderers can reuse.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::{
    RepositoryBoundedTraversal, RepositoryEntryCategory, RepositoryEntryKind, RepositoryEntryRole,
    RepositoryTraversalDecision,
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{WorkspaceContext, inspect_workspace, traverse_workspace_bounded};

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
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

    #[test]
    fn graph_contains_root_and_traversed_entries() {
        let root = create_graph_workspace("contains-root");
        let context = WorkspaceContext::new(&root).expect("context should be created");
        let inspection = inspect_workspace(&context).expect("inspection should run");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should run");

        let graph = build_repository_graph(&traversal);

        assert!(graph.node_count() > traversal.entry_count());
        assert_eq!(graph.edge_count(), traversal.entry_count());
        assert!(graph.contains_relative_path("monad.toml"));
        assert!(graph.contains_relative_path("docs/guide/intro.md"));
        assert!(graph.contains_relative_path("crates/monad-core/src/lib.rs"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn graph_edges_connect_parent_child_relationships() {
        let root = create_graph_workspace("edges");
        let context = WorkspaceContext::new(&root).expect("context should be created");
        let inspection = inspect_workspace(&context).expect("inspection should run");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should run");

        let graph = build_repository_graph(&traversal);

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
        let root = create_graph_workspace("counts");
        let context = WorkspaceContext::new(&root).expect("context should be created");
        let inspection = inspect_workspace(&context).expect("inspection should run");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should run");

        let graph = build_repository_graph(&traversal);
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
    fn graph_output_is_deterministically_ordered() {
        let root = create_graph_workspace("deterministic");
        let context = WorkspaceContext::new(&root).expect("context should be created");
        let inspection = inspect_workspace(&context).expect("inspection should run");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should run");

        let graph = build_repository_graph(&traversal);

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
