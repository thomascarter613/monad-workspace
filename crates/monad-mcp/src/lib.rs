//! MCP integration foundation for Monad.
//!
//! This crate is intentionally a placeholder boundary.
//!
//! WP-E6-006 does not implement a full MCP server, streaming transport,
//! authentication, authorization, provider-specific integration, tool calling,
//! marketplace distribution, or unsafe write/apply tools.
//!
//! The initial purpose of this crate is to document the candidate capability
//! surface that future MCP-compatible work may expose.

/// Candidate MCP capability kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum McpCapabilityKind {
    /// Read-only resource exposed to an MCP-compatible client.
    Resource,

    /// Tool exposed to an MCP-compatible client.
    Tool,

    /// Prompt template exposed to an MCP-compatible client.
    Prompt,
}

impl McpCapabilityKind {
    /// Returns a stable capability-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resource => "resource",
            Self::Tool => "tool",
            Self::Prompt => "prompt",
        }
    }
}

/// Safety class for a candidate MCP capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum McpSafetyClass {
    /// Read-only capability.
    ReadOnly,

    /// Dry-run-only capability.
    DryRunOnly,

    /// Future capability requiring explicit approval before mutation.
    ApprovalGated,

    /// Capability that must not be exposed in early MCP integration.
    ForbiddenEarly,
}

impl McpSafetyClass {
    /// Returns a stable safety-class label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::DryRunOnly => "dry_run_only",
            Self::ApprovalGated => "approval_gated",
            Self::ForbiddenEarly => "forbidden_early",
        }
    }

    /// Returns true when this safety class may be exposed in the first MCP surface.
    #[must_use]
    pub const fn is_allowed_initially(self) -> bool {
        matches!(self, Self::ReadOnly | Self::DryRunOnly)
    }
}

/// Descriptor for a candidate MCP-exposed Monad capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpCapabilityDescriptor {
    id: String,
    title: String,
    kind: McpCapabilityKind,
    safety_class: McpSafetyClass,
    description: String,
}

impl McpCapabilityDescriptor {
    /// Creates a capability descriptor.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        kind: McpCapabilityKind,
        safety_class: McpSafetyClass,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            kind,
            safety_class,
            description: description.into(),
        }
    }

    /// Returns capability ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns capability title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns capability kind.
    #[must_use]
    pub const fn kind(&self) -> McpCapabilityKind {
        self.kind
    }

    /// Returns safety class.
    #[must_use]
    pub const fn safety_class(&self) -> McpSafetyClass {
        self.safety_class
    }

    /// Returns description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns true when the capability may be included in the initial MCP surface.
    #[must_use]
    pub const fn is_allowed_initially(&self) -> bool {
        self.safety_class.is_allowed_initially()
    }
}

/// Returns initial candidate MCP capabilities.
///
/// These are descriptors only. No MCP server or callable tool is implemented.
#[must_use]
pub fn initial_mcp_capabilities() -> Vec<McpCapabilityDescriptor> {
    vec![
        McpCapabilityDescriptor::new(
            "resource.repository_summary",
            "Repository Summary",
            McpCapabilityKind::Resource,
            McpSafetyClass::ReadOnly,
            "Expose a read-only summary of the current repository.",
        ),
        McpCapabilityDescriptor::new(
            "resource.context_state",
            "Context State",
            McpCapabilityKind::Resource,
            McpSafetyClass::ReadOnly,
            "Expose read-only repo-native context state.",
        ),
        McpCapabilityDescriptor::new(
            "tool.plan",
            "Supervised Plan",
            McpCapabilityKind::Tool,
            McpSafetyClass::ReadOnly,
            "Produce a supervised no-write plan from a user intent.",
        ),
        McpCapabilityDescriptor::new(
            "tool.evolve_verify_baseline_dry_run",
            "Verify Baseline Dry Run",
            McpCapabilityKind::Tool,
            McpSafetyClass::DryRunOnly,
            "Preview verification baseline file operations without writing files.",
        ),
        McpCapabilityDescriptor::new(
            "tool.apply_file_operations",
            "Apply File Operations",
            McpCapabilityKind::Tool,
            McpSafetyClass::ApprovalGated,
            "Future approval-gated apply capability; not exposed initially.",
        ),
        McpCapabilityDescriptor::new(
            "tool.shell",
            "Arbitrary Shell",
            McpCapabilityKind::Tool,
            McpSafetyClass::ForbiddenEarly,
            "Forbidden early because it can bypass Monad safety boundaries.",
        ),
    ]
}

/// Returns only capabilities allowed in the initial MCP surface.
#[must_use]
pub fn initial_allowed_mcp_capabilities() -> Vec<McpCapabilityDescriptor> {
    initial_mcp_capabilities()
        .into_iter()
        .filter(McpCapabilityDescriptor::is_allowed_initially)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_kind_labels_are_stable() {
        assert_eq!(McpCapabilityKind::Resource.as_str(), "resource");
        assert_eq!(McpCapabilityKind::Tool.as_str(), "tool");
        assert_eq!(McpCapabilityKind::Prompt.as_str(), "prompt");
    }

    #[test]
    fn safety_class_labels_are_stable() {
        assert_eq!(McpSafetyClass::ReadOnly.as_str(), "read_only");
        assert_eq!(McpSafetyClass::DryRunOnly.as_str(), "dry_run_only");
        assert_eq!(McpSafetyClass::ApprovalGated.as_str(), "approval_gated");
        assert_eq!(McpSafetyClass::ForbiddenEarly.as_str(), "forbidden_early");
    }

    #[test]
    fn initial_capabilities_include_allowed_and_guarded_descriptors() {
        let capabilities = initial_mcp_capabilities();

        assert!(capabilities.iter().any(|capability| {
            capability.id() == "tool.plan" && capability.safety_class() == McpSafetyClass::ReadOnly
        }));
        assert!(capabilities.iter().any(|capability| {
            capability.id() == "tool.apply_file_operations"
                && capability.safety_class() == McpSafetyClass::ApprovalGated
        }));
        assert!(capabilities.iter().any(|capability| {
            capability.id() == "tool.shell"
                && capability.safety_class() == McpSafetyClass::ForbiddenEarly
        }));
    }

    #[test]
    fn initial_allowed_capabilities_exclude_approval_gated_and_forbidden_items() {
        let allowed = initial_allowed_mcp_capabilities();

        assert!(
            allowed
                .iter()
                .all(McpCapabilityDescriptor::is_allowed_initially)
        );
        assert!(
            allowed
                .iter()
                .any(|capability| capability.id() == "tool.plan")
        );
        assert!(
            !allowed
                .iter()
                .any(|capability| capability.id() == "tool.apply_file_operations")
        );
        assert!(
            !allowed
                .iter()
                .any(|capability| capability.id() == "tool.shell")
        );
    }
}
