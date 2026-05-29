//! Approval gate model.
//!
//! Approval gates describe what kind of human authorization is required before
//! Monad performs risky or consequential actions.

/// Stable identifier for an approval gate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApprovalGateId(String);

impl ApprovalGateId {
    /// Creates an approval gate ID.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the ID as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Kind of approval gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApprovalGateKind {
    /// Read-only explanation or inspection.
    ReadOnly,

    /// Planning or dry-run behavior that does not mutate state.
    PlanOnly,

    /// Draft generation that remains reviewable and non-applied.
    Draft,

    /// Local file write.
    LocalWrite,

    /// Destructive local operation.
    DestructiveLocal,

    /// Git state or history mutation.
    GitMutation,

    /// Remote side effect such as push, PR creation, publish, or deploy.
    RemoteSideEffect,
}

impl ApprovalGateKind {
    /// Returns a stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PlanOnly => "plan_only",
            Self::Draft => "draft",
            Self::LocalWrite => "local_write",
            Self::DestructiveLocal => "destructive_local",
            Self::GitMutation => "git_mutation",
            Self::RemoteSideEffect => "remote_side_effect",
        }
    }

    /// Returns true when explicit approval is required.
    #[must_use]
    pub const fn requires_explicit_approval(self) -> bool {
        !matches!(self, Self::ReadOnly | Self::PlanOnly)
    }

    /// Returns true when elevated approval should be required.
    #[must_use]
    pub const fn requires_elevated_approval(self) -> bool {
        matches!(
            self,
            Self::DestructiveLocal | Self::GitMutation | Self::RemoteSideEffect
        )
    }
}

/// Approval requirement attached to a proposed action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalRequirement {
    gate_kind: ApprovalGateKind,
    reason: String,
}

impl ApprovalRequirement {
    /// Creates an approval requirement.
    #[must_use]
    pub fn new(gate_kind: ApprovalGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate_kind,
            reason: reason.into(),
        }
    }

    /// Returns the gate kind.
    #[must_use]
    pub const fn gate_kind(&self) -> ApprovalGateKind {
        self.gate_kind
    }

    /// Returns the reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Returns true when explicit approval is required.
    #[must_use]
    pub const fn requires_explicit_approval(&self) -> bool {
        self.gate_kind.requires_explicit_approval()
    }

    /// Returns true when elevated approval is required.
    #[must_use]
    pub const fn requires_elevated_approval(&self) -> bool {
        self.gate_kind.requires_elevated_approval()
    }
}

/// Metadata for a proposed action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposedAction {
    action_id: String,
    title: String,
    summary: String,
    targets: Vec<String>,
}

impl ProposedAction {
    /// Creates proposed action metadata.
    #[must_use]
    pub fn new(
        action_id: impl Into<String>,
        title: impl Into<String>,
        summary: impl Into<String>,
        targets: Vec<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            title: title.into(),
            summary: summary.into(),
            targets,
        }
    }

    /// Returns the action ID.
    #[must_use]
    pub fn action_id(&self) -> &str {
        &self.action_id
    }

    /// Returns the action title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the action summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Returns target paths or target descriptors.
    #[must_use]
    pub fn targets(&self) -> &[String] {
        &self.targets
    }
}

/// Approval gate combining a proposed action and approval requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalGate {
    id: ApprovalGateId,
    action: ProposedAction,
    requirement: ApprovalRequirement,
}

impl ApprovalGate {
    /// Creates an approval gate.
    #[must_use]
    pub fn new(
        id: ApprovalGateId,
        action: ProposedAction,
        requirement: ApprovalRequirement,
    ) -> Self {
        Self {
            id,
            action,
            requirement,
        }
    }

    /// Returns gate ID.
    #[must_use]
    pub const fn id(&self) -> &ApprovalGateId {
        &self.id
    }

    /// Returns proposed action metadata.
    #[must_use]
    pub const fn action(&self) -> &ProposedAction {
        &self.action
    }

    /// Returns approval requirement.
    #[must_use]
    pub const fn requirement(&self) -> &ApprovalRequirement {
        &self.requirement
    }

    /// Returns true when explicit approval is required.
    #[must_use]
    pub const fn requires_explicit_approval(&self) -> bool {
        self.requirement.requires_explicit_approval()
    }
}

/// Approval decision kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApprovalDecisionKind {
    /// Action approved.
    Approved,

    /// Action rejected.
    Rejected,

    /// Action deferred pending more information.
    Deferred,
}

impl ApprovalDecisionKind {
    /// Returns a stable decision label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Rejected => "rejected",
            Self::Deferred => "deferred",
        }
    }

    /// Returns true when the decision permits the action to proceed.
    #[must_use]
    pub const fn permits_action(self) -> bool {
        matches!(self, Self::Approved)
    }
}

/// Approval or rejection decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalDecision {
    gate_id: ApprovalGateId,
    decision_kind: ApprovalDecisionKind,
    actor: String,
    reason: String,
}

impl ApprovalDecision {
    /// Creates an approval decision.
    #[must_use]
    pub fn new(
        gate_id: ApprovalGateId,
        decision_kind: ApprovalDecisionKind,
        actor: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            gate_id,
            decision_kind,
            actor: actor.into(),
            reason: reason.into(),
        }
    }

    /// Creates an approved decision.
    #[must_use]
    pub fn approved(
        gate_id: ApprovalGateId,
        actor: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(gate_id, ApprovalDecisionKind::Approved, actor, reason)
    }

    /// Creates a rejected decision.
    #[must_use]
    pub fn rejected(
        gate_id: ApprovalGateId,
        actor: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(gate_id, ApprovalDecisionKind::Rejected, actor, reason)
    }

    /// Creates a deferred decision.
    #[must_use]
    pub fn deferred(
        gate_id: ApprovalGateId,
        actor: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(gate_id, ApprovalDecisionKind::Deferred, actor, reason)
    }

    /// Returns gate ID.
    #[must_use]
    pub const fn gate_id(&self) -> &ApprovalGateId {
        &self.gate_id
    }

    /// Returns decision kind.
    #[must_use]
    pub const fn decision_kind(&self) -> ApprovalDecisionKind {
        self.decision_kind
    }

    /// Returns actor.
    #[must_use]
    pub fn actor(&self) -> &str {
        &self.actor
    }

    /// Returns decision reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Returns true when the decision permits the action.
    #[must_use]
    pub const fn permits_action(&self) -> bool {
        self.decision_kind.permits_action()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approval_gate_kind_labels_are_stable() {
        assert_eq!(ApprovalGateKind::ReadOnly.as_str(), "read_only");
        assert_eq!(ApprovalGateKind::PlanOnly.as_str(), "plan_only");
        assert_eq!(ApprovalGateKind::Draft.as_str(), "draft");
        assert_eq!(ApprovalGateKind::LocalWrite.as_str(), "local_write");
        assert_eq!(
            ApprovalGateKind::DestructiveLocal.as_str(),
            "destructive_local"
        );
        assert_eq!(ApprovalGateKind::GitMutation.as_str(), "git_mutation");
        assert_eq!(
            ApprovalGateKind::RemoteSideEffect.as_str(),
            "remote_side_effect"
        );
    }

    #[test]
    fn approval_kind_identifies_required_approval() {
        assert!(!ApprovalGateKind::ReadOnly.requires_explicit_approval());
        assert!(!ApprovalGateKind::PlanOnly.requires_explicit_approval());
        assert!(ApprovalGateKind::Draft.requires_explicit_approval());
        assert!(ApprovalGateKind::LocalWrite.requires_explicit_approval());
        assert!(ApprovalGateKind::DestructiveLocal.requires_elevated_approval());
        assert!(ApprovalGateKind::GitMutation.requires_elevated_approval());
        assert!(ApprovalGateKind::RemoteSideEffect.requires_elevated_approval());
    }

    #[test]
    fn approval_gate_preserves_action_and_requirement() {
        let gate = ApprovalGate::new(
            ApprovalGateId::new("gate-1"),
            ProposedAction::new(
                "action-1",
                "Apply draft",
                "Apply the reviewed draft operations.",
                vec!["docs/example.md".to_string()],
            ),
            ApprovalRequirement::new(
                ApprovalGateKind::LocalWrite,
                "Applying draft operations writes repository files.",
            ),
        );

        assert_eq!(gate.id().as_str(), "gate-1");
        assert_eq!(gate.action().action_id(), "action-1");
        assert_eq!(gate.action().targets(), &["docs/example.md".to_string()]);
        assert_eq!(gate.requirement().gate_kind(), ApprovalGateKind::LocalWrite);
        assert!(gate.requires_explicit_approval());
    }

    #[test]
    fn approval_decision_records_actor_reason_and_result() {
        let gate_id = ApprovalGateId::new("gate-1");
        let approved =
            ApprovalDecision::approved(gate_id.clone(), "thomas", "Approved after review.");
        let rejected =
            ApprovalDecision::rejected(gate_id, "thomas", "Rejected because scope expanded.");

        assert_eq!(approved.decision_kind(), ApprovalDecisionKind::Approved);
        assert_eq!(approved.actor(), "thomas");
        assert!(approved.permits_action());
        assert_eq!(rejected.decision_kind(), ApprovalDecisionKind::Rejected);
        assert!(!rejected.permits_action());
    }
}
