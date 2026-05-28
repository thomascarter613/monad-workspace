//! Supervised agent foundations.
//!
//! Monad agent features must remain provider-agnostic, local-first where
//! possible, and human-in-command.
//!
//! WP-E6-002 intentionally adds only a small model-provider abstraction:
//!
//! - no real provider API calls;
//! - no API key management;
//! - no streaming;
//! - no tool calling;
//! - no MCP implementation;
//! - no model routing;
//! - no provider marketplace;
//! - no cloud service.

pub mod model;
pub mod provider;

pub use model::{ModelMessage, ModelMessageRole, ModelRequest, ModelResponse};
pub use provider::{
    MockModelProvider, ModelProvider, ModelProviderCapabilities, ModelProviderMetadata,
};
