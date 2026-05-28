//! Safe repository evolution workflows.
//!
//! The evolution engine turns known templates and safe file operation plans
//! into reviewable repository improvements.
//!
//! WP-E5-004 adds the first command-level workflow foundation:
//! `evolve verify-baseline --dry-run`.
//!
//! This module does not write files, create commits, open pull requests, or
//! perform autonomous agent behavior.

pub mod context_baseline;
pub mod verify_baseline;

pub use context_baseline::{build_context_baseline_plan, render_context_baseline_dry_run};
pub use verify_baseline::{build_verify_baseline_plan, render_verify_baseline_dry_run};
