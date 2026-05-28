//! Template metadata and definition model.
//!
//! This model describes template source material for future evolution commands.
//! It does not render templates, interpolate variables, write files, fetch
//! remote packs, or install plugins.

use std::path::{Path, PathBuf};

/// Stable identifier for a Monad template.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TemplateId(String);

impl TemplateId {
    /// Creates a template ID.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the template ID as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Source kind for a template.
///
/// WP-E5-003 only supports embedded templates. Additional source kinds can be
/// added later by ADR-backed design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TemplateSourceKind {
    /// Template content is embedded in Monad or constructed locally in code.
    Embedded,
}

impl TemplateSourceKind {
    /// Returns a stable source-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Embedded => "embedded",
        }
    }
}

/// Reviewable metadata for a template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateMetadata {
    id: TemplateId,
    name: String,
    version: String,
    description: String,
    target_path: PathBuf,
    source_kind: TemplateSourceKind,
}

impl TemplateMetadata {
    /// Creates template metadata.
    #[must_use]
    pub fn new(
        id: TemplateId,
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
        target_path: impl Into<PathBuf>,
        source_kind: TemplateSourceKind,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            version: version.into(),
            description: description.into(),
            target_path: target_path.into(),
            source_kind,
        }
    }

    /// Returns the template ID.
    #[must_use]
    pub const fn id(&self) -> &TemplateId {
        &self.id
    }

    /// Returns the human-readable template name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the template version label.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the template description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the default target path for this template.
    #[must_use]
    pub fn target_path(&self) -> &Path {
        &self.target_path
    }

    /// Returns the source kind.
    #[must_use]
    pub const fn source_kind(&self) -> TemplateSourceKind {
        self.source_kind
    }
}

/// Complete local template definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateDefinition {
    metadata: TemplateMetadata,
    content: String,
}

impl TemplateDefinition {
    /// Creates a template definition from metadata and content.
    #[must_use]
    pub fn new(metadata: TemplateMetadata, content: impl Into<String>) -> Self {
        Self {
            metadata,
            content: content.into(),
        }
    }

    /// Creates an embedded template definition.
    #[must_use]
    pub fn embedded(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
        target_path: impl Into<PathBuf>,
        content: impl Into<String>,
    ) -> Self {
        Self::new(
            TemplateMetadata::new(
                TemplateId::new(id),
                name,
                version,
                description,
                target_path,
                TemplateSourceKind::Embedded,
            ),
            content,
        )
    }

    /// Returns template metadata.
    #[must_use]
    pub const fn metadata(&self) -> &TemplateMetadata {
        &self.metadata
    }

    /// Returns the template ID.
    #[must_use]
    pub const fn id(&self) -> &TemplateId {
        self.metadata.id()
    }

    /// Returns the template content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_id_preserves_value() {
        let id = TemplateId::new("verify-baseline.readme");

        assert_eq!(id.as_str(), "verify-baseline.readme");
    }

    #[test]
    fn source_kind_label_is_stable() {
        assert_eq!(TemplateSourceKind::Embedded.as_str(), "embedded");
    }

    #[test]
    fn metadata_exposes_template_fields() {
        let metadata = TemplateMetadata::new(
            TemplateId::new("context.bootstrap"),
            "Bootstrap Prompt",
            "1",
            "Creates the AI bootstrap prompt.",
            "docs/ai/BOOTSTRAP-PROMPT.md",
            TemplateSourceKind::Embedded,
        );

        assert_eq!(metadata.id().as_str(), "context.bootstrap");
        assert_eq!(metadata.name(), "Bootstrap Prompt");
        assert_eq!(metadata.version(), "1");
        assert_eq!(metadata.description(), "Creates the AI bootstrap prompt.");
        assert_eq!(
            metadata.target_path(),
            Path::new("docs/ai/BOOTSTRAP-PROMPT.md")
        );
        assert_eq!(metadata.source_kind(), TemplateSourceKind::Embedded);
    }

    #[test]
    fn embedded_template_definition_contains_metadata_and_content() {
        let template = TemplateDefinition::embedded(
            "verify-baseline.readme",
            "Verification Baseline README",
            "1",
            "Creates a verification baseline README.",
            "docs/verification/README.md",
            "# Verification\n",
        );

        assert_eq!(template.id().as_str(), "verify-baseline.readme");
        assert_eq!(template.metadata().name(), "Verification Baseline README");
        assert_eq!(
            template.metadata().source_kind(),
            TemplateSourceKind::Embedded
        );
        assert_eq!(template.content(), "# Verification\n");
    }
}
