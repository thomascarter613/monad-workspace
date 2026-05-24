//! Core runtime library for Monad.
//!
//! In Rust, files can be compiled as either binaries or libraries.
//! This file is the root of the `monad-core` library crate.
//!
//! Monad's architecture keeps durable product logic here, while the CLI crate
//! stays thin and delegates to this library.

pub mod diagnostics;

pub use diagnostics::{Diagnostic, DiagnosticReport, Severity};

/// Describes the currently compiled Monad runtime identity.
///
/// A `struct` groups related values into one named type.
/// Here we use string slices with a `'static` lifetime because these values are
/// fixed text baked into the compiled program.
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
    ///
    /// `const fn` means this function can be evaluated at compile time because
    /// it only returns fixed values and does not allocate memory.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            product_name: "Monad",
            runtime_crate: "monad-core",
            execution_model: "local-first",
        }
    }

    /// Builds a human-readable startup banner.
    ///
    /// This returns a `String` because `format!` creates owned text at runtime.
    #[must_use]
    pub fn banner(self) -> String {
        format!(
            "{} runtime foundation ready (crate: {}, model: {})",
            self.product_name, self.runtime_crate, self.execution_model
        )
    }

    /// Builds a structured startup diagnostic.
    ///
    /// Future CLI commands can use diagnostics when they need stable structured
    /// output instead of plain strings.
    #[must_use]
    pub fn startup_diagnostic(self) -> Diagnostic {
        Diagnostic::info("MONAD0001", self.banner())
    }
}

impl Default for RuntimeIdentity {
    /// Provides the default runtime identity.
    ///
    /// `Default` is a common Rust trait used when a type has an obvious default
    /// value. We forward to `RuntimeIdentity::new()` so there is only one source
    /// of truth for the default fields.
    fn default() -> Self {
        Self::new()
    }
}

/// Returns Monad's canonical runtime identity.
///
/// Free functions like this are useful API entrypoints for other crates.
/// The CLI crate will call this rather than duplicating core runtime facts.
#[must_use]
pub fn runtime_identity() -> RuntimeIdentity {
    RuntimeIdentity::new()
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
}
