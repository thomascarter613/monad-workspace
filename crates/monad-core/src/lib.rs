//! Core runtime library for Monad.
//!
//! In Rust, files can be compiled as either binaries or libraries.
//! This file is the root of the `monad-core` library crate.
//!
//! Monad's architecture keeps durable product logic here, while the CLI crate
//! stays thin and delegates to this library.

pub mod diagnostics;
pub mod error;
pub mod workspace;

pub use diagnostics::{Diagnostic, DiagnosticReport, Severity};
pub use error::{MonadError, MonadResult};
pub use workspace::{WorkspaceContext, discover_workspace_root, is_workspace_root};

/// Describes the currently compiled Monad runtime identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeIdentity {
    /// The public product name.
    pub product_name: &'static str,

    /// The crate responsible for durable runtime logic.
    pub runtime_crate: &'static str,

    /// The operating model Monad is currently enforcing.
    pub execution_model: &'static str,
}

impl RuntimeIdentity {
    /// Creates the canonical runtime identity for this early workspace slice.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            product_name: "Monad",
            runtime_crate: "monad-core",
            execution_model: "local-first",
        }
    }

    /// Builds a human-readable startup banner.
    #[must_use]
    pub fn banner(self) -> String {
        format!(
            "{} runtime foundation ready (crate: {}, model: {})",
            self.product_name, self.runtime_crate, self.execution_model
        )
    }

    /// Builds a structured startup diagnostic.
    #[must_use]
    pub fn startup_diagnostic(self) -> Diagnostic {
        Diagnostic::info("MONAD0001", self.banner())
    }
}

impl Default for RuntimeIdentity {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns Monad's canonical runtime identity.
#[must_use]
pub fn runtime_identity() -> RuntimeIdentity {
    RuntimeIdentity::new()
}

/// Returns Monad's runtime identity through the shared result type.
pub fn checked_runtime_identity() -> MonadResult<RuntimeIdentity> {
    Ok(runtime_identity())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_identity_names_monad() {
        let identity = runtime_identity();

        assert_eq!(identity.product_name, "Monad");
        assert_eq!(identity.runtime_crate, "monad-core");
        assert_eq!(identity.execution_model, "local-first");
    }

    #[test]
    fn runtime_banner_is_human_readable() {
        let banner = runtime_identity().banner();

        assert!(banner.contains("Monad"));
        assert!(banner.contains("monad-core"));
        assert!(banner.contains("local-first"));
    }

    #[test]
    fn runtime_identity_can_produce_startup_diagnostic() {
        let diagnostic = runtime_identity().startup_diagnostic();

        assert_eq!(diagnostic.severity, Severity::Info);
        assert_eq!(diagnostic.code, "MONAD0001");
        assert!(diagnostic.message.contains("Monad"));
        assert!(diagnostic.render().contains("[INFO] MONAD0001"));
    }

    #[test]
    fn checked_runtime_identity_uses_monad_result() {
        let identity = checked_runtime_identity().expect("runtime identity should be available");

        assert_eq!(identity.product_name, "Monad");
    }

    #[test]
    fn workspace_context_is_exported_from_core_root() {
        let context = WorkspaceContext::new(".").expect("workspace context should be created");

        assert_eq!(context.root(), std::path::Path::new("."));
    }
}
