//! Local template registry.

use std::collections::BTreeMap;

use crate::{MonadError, MonadResult, TemplateDefinition, TemplateId};

/// Registry of known local templates.
///
/// The registry is deterministic because it uses a `BTreeMap`. This keeps
/// iteration and tests stable.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TemplateRegistry {
    templates: BTreeMap<TemplateId, TemplateDefinition>,
}

impl TemplateRegistry {
    /// Creates an empty template registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry from templates.
    pub fn from_templates(
        templates: impl IntoIterator<Item = TemplateDefinition>,
    ) -> MonadResult<Self> {
        let mut registry = Self::new();

        for template in templates {
            registry.register(template)?;
        }

        Ok(registry)
    }

    /// Registers a template.
    ///
    /// Duplicate IDs are rejected so evolution commands cannot silently replace
    /// known template source material.
    pub fn register(&mut self, template: TemplateDefinition) -> MonadResult<()> {
        let id = template.id().clone();

        if self.templates.contains_key(&id) {
            return Err(MonadError::invalid_input(format!(
                "template `{}` is already registered",
                id.as_str()
            )));
        }

        self.templates.insert(id, template);

        Ok(())
    }

    /// Returns true if a template ID is registered.
    #[must_use]
    pub fn contains(&self, id: &TemplateId) -> bool {
        self.templates.contains_key(id)
    }

    /// Looks up a template by ID.
    #[must_use]
    pub fn get(&self, id: &TemplateId) -> Option<&TemplateDefinition> {
        self.templates.get(id)
    }

    /// Looks up a template by string ID.
    #[must_use]
    pub fn get_by_str(&self, id: &str) -> Option<&TemplateDefinition> {
        self.get(&TemplateId::new(id))
    }

    /// Returns registered templates in deterministic ID order.
    #[must_use]
    pub fn templates(&self) -> Vec<&TemplateDefinition> {
        self.templates.values().collect()
    }

    /// Returns registered template IDs in deterministic order.
    #[must_use]
    pub fn ids(&self) -> Vec<&TemplateId> {
        self.templates.keys().collect()
    }

    /// Returns the number of registered templates.
    #[must_use]
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    /// Returns true when no templates are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
}

/// Builds the initial embedded template registry.
///
/// WP-E5-003 only needs the foundation, but including a tiny local registry
/// proves the model supports deterministic embedded templates.
pub fn initial_template_registry() -> MonadResult<TemplateRegistry> {
    TemplateRegistry::from_templates([
        TemplateDefinition::embedded(
            "verify-baseline.readme",
            "Verification Baseline README",
            "1",
            "Creates a placeholder README for future verification baseline documentation.",
            "docs/verification/README.md",
            "# Verification\n\nThis directory documents repository verification practices.\n",
        ),
        TemplateDefinition::embedded(
            "context-baseline.readme",
            "Context Baseline README",
            "1",
            "Creates a placeholder README for future context bridge documentation.",
            "docs/ai/README.md",
            "# AI Context\n\nThis directory contains AI-readable project context.\n",
        ),
    ])
}

#[cfg(test)]
mod tests {
    use crate::TemplateSourceKind;

    use super::*;

    fn example_template(id: &str) -> TemplateDefinition {
        TemplateDefinition::embedded(
            id,
            "Example Template",
            "1",
            "Example local embedded template.",
            "docs/example.md",
            "# Example\n",
        )
    }

    #[test]
    fn registry_starts_empty() {
        let registry = TemplateRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registry_registers_and_retrieves_template() -> MonadResult<()> {
        let mut registry = TemplateRegistry::new();
        let template = example_template("example.template");

        registry.register(template)?;

        let retrieved = registry
            .get_by_str("example.template")
            .ok_or_else(|| MonadError::not_found("template should be registered"))?;

        assert_eq!(retrieved.id().as_str(), "example.template");
        assert_eq!(retrieved.metadata().version(), "1");
        assert_eq!(
            retrieved.metadata().source_kind(),
            TemplateSourceKind::Embedded
        );
        assert_eq!(retrieved.content(), "# Example\n");

        Ok(())
    }

    #[test]
    fn registry_rejects_duplicate_template_ids() -> MonadResult<()> {
        let mut registry = TemplateRegistry::new();

        registry.register(example_template("duplicate.template"))?;
        let error = registry
            .register(example_template("duplicate.template"))
            .expect_err("duplicate template registration should fail");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("already registered"));

        Ok(())
    }

    #[test]
    fn registry_lists_templates_in_deterministic_order() -> MonadResult<()> {
        let registry = TemplateRegistry::from_templates([
            example_template("z.template"),
            example_template("a.template"),
            example_template("m.template"),
        ])?;

        let ids = registry
            .ids()
            .iter()
            .map(|id| id.as_str())
            .collect::<Vec<_>>();

        assert_eq!(ids, vec!["a.template", "m.template", "z.template"]);

        Ok(())
    }

    #[test]
    fn initial_registry_contains_baseline_templates() -> MonadResult<()> {
        let registry = initial_template_registry()?;

        assert!(registry.contains(&TemplateId::new("verify-baseline.readme")));
        assert!(registry.contains(&TemplateId::new("context-baseline.readme")));
        assert_eq!(registry.len(), 2);

        Ok(())
    }
}
