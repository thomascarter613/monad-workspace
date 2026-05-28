//! Template registry foundation.
//!
//! Monad's evolution engine should apply known, reviewable source material
//! rather than unpredictable ad hoc generation.
//!
//! WP-E5-003 intentionally keeps this local and simple:
//!
//! - no remote marketplace;
//! - no plugin installation;
//! - no template signing;
//! - no version negotiation;
//! - no complex templating engine;
//! - no user-authored template packs.
//!
//! Later E5 slices can build baseline evolution commands on top of this
//! registry foundation.

pub mod model;
pub mod registry;

pub use model::{TemplateDefinition, TemplateId, TemplateMetadata, TemplateSourceKind};
pub use registry::{TemplateRegistry, initial_template_registry};
