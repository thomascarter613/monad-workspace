//! Verification check registry for Monad.
//!
//! The registry is a deterministic collection of known check definitions.
//! It does not execute checks. It only records which checks are available.

use super::{CheckDefinition, CheckId};
use std::collections::BTreeMap;

/// Registry of available verification checks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CheckRegistry {
    checks: BTreeMap<CheckId, CheckDefinition>,
}

impl CheckRegistry {
    /// Creates an empty check registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry from check definitions.
    #[must_use]
    pub fn from_definitions(definitions: impl IntoIterator<Item = CheckDefinition>) -> Self {
        let mut registry = Self::new();

        for definition in definitions {
            registry.register(definition);
        }

        registry
    }

    /// Registers or replaces a check definition.
    pub fn register(&mut self, definition: CheckDefinition) {
        self.checks.insert(definition.id().clone(), definition);
    }

    /// Returns true when the registry contains a check ID.
    #[must_use]
    pub fn contains(&self, id: &CheckId) -> bool {
        self.checks.contains_key(id)
    }

    /// Returns a check definition by ID.
    #[must_use]
    pub fn get(&self, id: &CheckId) -> Option<&CheckDefinition> {
        self.checks.get(id)
    }

    /// Returns all check definitions in deterministic ID order.
    #[must_use]
    pub fn definitions(&self) -> Vec<&CheckDefinition> {
        self.checks.values().collect()
    }

    /// Returns the number of registered checks.
    #[must_use]
    pub fn len(&self) -> usize {
        self.checks.len()
    }

    /// Returns true when no checks are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.checks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::super::CheckSeverity;
    use super::*;

    fn definition(id: &str, name: &str) -> CheckDefinition {
        CheckDefinition::new(
            CheckId::new(id),
            name,
            CheckSeverity::Warning,
            format!("{name} description"),
        )
    }

    #[test]
    fn registry_starts_empty() {
        let registry = CheckRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registry_registers_and_finds_checks() {
        let mut registry = CheckRegistry::new();
        let id = CheckId::new("MONAD-CHECK-0001");

        registry.register(CheckDefinition::new(
            id.clone(),
            "Example check",
            CheckSeverity::Info,
            "Example description",
        ));

        assert!(registry.contains(&id));
        assert_eq!(registry.len(), 1);

        let found = registry.get(&id).expect("check should be registered");
        assert_eq!(found.name(), "Example check");
    }

    #[test]
    fn registry_replaces_duplicate_check_ids() {
        let mut registry = CheckRegistry::new();
        let id = CheckId::new("MONAD-CHECK-0001");

        registry.register(CheckDefinition::new(
            id.clone(),
            "Old name",
            CheckSeverity::Info,
            "Old description",
        ));

        registry.register(CheckDefinition::new(
            id.clone(),
            "New name",
            CheckSeverity::Warning,
            "New description",
        ));

        assert_eq!(registry.len(), 1);

        let found = registry.get(&id).expect("check should be registered");
        assert_eq!(found.name(), "New name");
        assert_eq!(found.severity(), CheckSeverity::Warning);
    }

    #[test]
    fn registry_definitions_are_deterministically_ordered_by_id() {
        let registry = CheckRegistry::from_definitions([
            definition("MONAD-CHECK-0003", "Third"),
            definition("MONAD-CHECK-0001", "First"),
            definition("MONAD-CHECK-0002", "Second"),
        ]);

        let ids = registry
            .definitions()
            .iter()
            .map(|definition| definition.id().as_str().to_string())
            .collect::<Vec<_>>();

        assert_eq!(
            ids,
            vec![
                "MONAD-CHECK-0001".to_string(),
                "MONAD-CHECK-0002".to_string(),
                "MONAD-CHECK-0003".to_string(),
            ]
        );
    }
}
