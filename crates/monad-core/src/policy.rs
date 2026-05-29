//! Policy and governance primitives.
//!
//! This module defines the first approval-gate and audit-log model for
//! supervised Monad workflows.
//!
//! WP-E6-005 intentionally does not implement enterprise RBAC, SSO,
//! cryptographic signing, tamper-proof ledgers, remote audit storage,
//! compliance certification, or a full policy engine.

pub mod approval;
pub mod audit;

pub use approval::{
    ApprovalDecision, ApprovalDecisionKind, ApprovalGate, ApprovalGateId, ApprovalGateKind,
    ApprovalRequirement, ProposedAction,
};
pub use audit::{AuditActor, AuditEvent, AuditEventKind, AuditLog};
