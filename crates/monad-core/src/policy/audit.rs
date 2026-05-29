//! Audit event and audit log model.
//!
//! This is a local in-memory model. It is not a tamper-proof ledger,
//! cryptographic audit trail, remote service, or compliance certification.

use crate::{
    ApprovalDecision, ApprovalDecisionKind, ApprovalGate, ApprovalGateId, ApprovalGateKind,
};

/// Actor associated with an audit event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditActor {
    actor_id: String,
    display_name: String,
}

impl AuditActor {
    /// Creates an audit actor.
    #[must_use]
    pub fn new(actor_id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            actor_id: actor_id.into(),
            display_name: display_name.into(),
        }
    }

    /// Creates a local user actor.
    #[must_use]
    pub fn local_user(display_name: impl Into<String>) -> Self {
        let display_name = display_name.into();

        Self::new(display_name.clone(), display_name)
    }

    /// Creates a Monad system actor.
    #[must_use]
    pub fn monad_system() -> Self {
        Self::new("monad", "Monad")
    }

    /// Returns actor ID.
    #[must_use]
    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }

    /// Returns display name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

/// Audit event kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditEventKind {
    /// A risky or consequential action was proposed.
    ActionProposed,

    /// Approval was requested.
    ApprovalRequested,

    /// Approval was granted.
    ApprovalGranted,

    /// Approval was rejected.
    ApprovalRejected,

    /// Approval was deferred.
    ApprovalDeferred,

    /// Execution was started.
    ExecutionStarted,

    /// Execution completed successfully.
    ExecutionCompleted,

    /// Execution failed.
    ExecutionFailed,

    /// Verification evidence was recorded.
    EvidenceRecorded,
}

impl AuditEventKind {
    /// Returns a stable event label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActionProposed => "action_proposed",
            Self::ApprovalRequested => "approval_requested",
            Self::ApprovalGranted => "approval_granted",
            Self::ApprovalRejected => "approval_rejected",
            Self::ApprovalDeferred => "approval_deferred",
            Self::ExecutionStarted => "execution_started",
            Self::ExecutionCompleted => "execution_completed",
            Self::ExecutionFailed => "execution_failed",
            Self::EvidenceRecorded => "evidence_recorded",
        }
    }
}

/// One audit event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    event_id: String,
    event_kind: AuditEventKind,
    actor: AuditActor,
    subject_id: String,
    message: String,
}

impl AuditEvent {
    /// Creates an audit event.
    #[must_use]
    pub fn new(
        event_id: impl Into<String>,
        event_kind: AuditEventKind,
        actor: AuditActor,
        subject_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            event_id: event_id.into(),
            event_kind,
            actor,
            subject_id: subject_id.into(),
            message: message.into(),
        }
    }

    /// Creates an event for a proposed action approval gate.
    #[must_use]
    pub fn action_proposed(event_id: impl Into<String>, gate: &ApprovalGate) -> Self {
        Self::new(
            event_id,
            AuditEventKind::ActionProposed,
            AuditActor::monad_system(),
            gate.id().as_str(),
            format!(
                "Proposed action `{}` requires `{}` approval: {}",
                gate.action().action_id(),
                gate.requirement().gate_kind().as_str(),
                gate.requirement().reason()
            ),
        )
    }

    /// Creates an approval-requested event.
    #[must_use]
    pub fn approval_requested(event_id: impl Into<String>, gate: &ApprovalGate) -> Self {
        Self::new(
            event_id,
            AuditEventKind::ApprovalRequested,
            AuditActor::monad_system(),
            gate.id().as_str(),
            format!(
                "Approval requested for `{}` using `{}` gate.",
                gate.action().title(),
                gate.requirement().gate_kind().as_str()
            ),
        )
    }

    /// Creates an event from an approval decision.
    #[must_use]
    pub fn from_decision(event_id: impl Into<String>, decision: &ApprovalDecision) -> Self {
        let event_kind = match decision.decision_kind() {
            ApprovalDecisionKind::Approved => AuditEventKind::ApprovalGranted,
            ApprovalDecisionKind::Rejected => AuditEventKind::ApprovalRejected,
            ApprovalDecisionKind::Deferred => AuditEventKind::ApprovalDeferred,
        };

        Self::new(
            event_id,
            event_kind,
            AuditActor::local_user(decision.actor()),
            decision.gate_id().as_str(),
            format!(
                "Approval decision `{}` recorded: {}",
                decision.decision_kind().as_str(),
                decision.reason()
            ),
        )
    }

    /// Returns event ID.
    #[must_use]
    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    /// Returns event kind.
    #[must_use]
    pub const fn event_kind(&self) -> AuditEventKind {
        self.event_kind
    }

    /// Returns actor.
    #[must_use]
    pub const fn actor(&self) -> &AuditActor {
        &self.actor
    }

    /// Returns subject ID.
    #[must_use]
    pub fn subject_id(&self) -> &str {
        &self.subject_id
    }

    /// Returns message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// In-memory audit log.
///
/// This is intentionally simple and repo-local in concept. Disk persistence,
/// append-only guarantees, signing, and remote storage are future work.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuditLog {
    events: Vec<AuditEvent>,
}

impl AuditLog {
    /// Creates an empty audit log.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an audit log from events.
    #[must_use]
    pub fn from_events(events: Vec<AuditEvent>) -> Self {
        Self { events }
    }

    /// Appends an event.
    pub fn push(&mut self, event: AuditEvent) {
        self.events.push(event);
    }

    /// Returns all events.
    #[must_use]
    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Returns event count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns true when no events are recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns events for a subject.
    #[must_use]
    pub fn events_for_subject(&self, subject_id: &str) -> Vec<&AuditEvent> {
        self.events()
            .iter()
            .filter(|event| event.subject_id() == subject_id)
            .collect()
    }

    /// Returns true when the log contains an approval-granted event for a gate.
    #[must_use]
    pub fn contains_approval_for_gate(&self, gate_id: &ApprovalGateId) -> bool {
        self.events().iter().any(|event| {
            event.subject_id() == gate_id.as_str()
                && event.event_kind() == AuditEventKind::ApprovalGranted
        })
    }
}

/// Creates a standard approval gate and audit trail for a local write proposal.
#[must_use]
pub fn local_write_gate_with_audit(
    gate_id: impl Into<String>,
    action_id: impl Into<String>,
    title: impl Into<String>,
    summary: impl Into<String>,
    targets: Vec<String>,
) -> (ApprovalGate, AuditLog) {
    let gate = ApprovalGate::new(
        ApprovalGateId::new(gate_id),
        crate::ProposedAction::new(action_id, title, summary, targets),
        crate::ApprovalRequirement::new(
            ApprovalGateKind::LocalWrite,
            "Local file writes require explicit user approval.",
        ),
    );

    let log = AuditLog::from_events(vec![
        AuditEvent::action_proposed("event-action-proposed", &gate),
        AuditEvent::approval_requested("event-approval-requested", &gate),
    ]);

    (gate, log)
}

#[cfg(test)]
mod tests {
    use crate::{
        ApprovalDecision, ApprovalGate, ApprovalGateId, ApprovalGateKind, ApprovalRequirement,
        ProposedAction,
    };

    use super::*;

    fn example_gate() -> ApprovalGate {
        ApprovalGate::new(
            ApprovalGateId::new("gate-1"),
            ProposedAction::new(
                "action-1",
                "Apply draft",
                "Apply reviewed draft operations.",
                vec!["docs/example.md".to_string()],
            ),
            ApprovalRequirement::new(
                ApprovalGateKind::LocalWrite,
                "Applying the draft writes files.",
            ),
        )
    }

    #[test]
    fn audit_event_kind_labels_are_stable() {
        assert_eq!(AuditEventKind::ActionProposed.as_str(), "action_proposed");
        assert_eq!(
            AuditEventKind::ApprovalRequested.as_str(),
            "approval_requested"
        );
        assert_eq!(AuditEventKind::ApprovalGranted.as_str(), "approval_granted");
        assert_eq!(
            AuditEventKind::ApprovalRejected.as_str(),
            "approval_rejected"
        );
        assert_eq!(
            AuditEventKind::ApprovalDeferred.as_str(),
            "approval_deferred"
        );
    }

    #[test]
    fn action_proposed_event_records_gate_metadata() {
        let gate = example_gate();
        let event = AuditEvent::action_proposed("event-1", &gate);

        assert_eq!(event.event_id(), "event-1");
        assert_eq!(event.event_kind(), AuditEventKind::ActionProposed);
        assert_eq!(event.actor().actor_id(), "monad");
        assert_eq!(event.subject_id(), "gate-1");
        assert!(event.message().contains("action-1"));
        assert!(event.message().contains("local_write"));
    }

    #[test]
    fn approval_decision_event_uses_decision_kind() {
        let decision = ApprovalDecision::approved(
            ApprovalGateId::new("gate-1"),
            "thomas",
            "Approved after review.",
        );
        let event = AuditEvent::from_decision("event-approval", &decision);

        assert_eq!(event.event_kind(), AuditEventKind::ApprovalGranted);
        assert_eq!(event.actor().actor_id(), "thomas");
        assert_eq!(event.subject_id(), "gate-1");
        assert!(event.message().contains("approved"));
    }

    #[test]
    fn audit_log_records_events_and_filters_by_subject() {
        let gate = example_gate();
        let mut log = AuditLog::new();

        log.push(AuditEvent::action_proposed("event-1", &gate));
        log.push(AuditEvent::approval_requested("event-2", &gate));

        assert_eq!(log.len(), 2);
        assert_eq!(log.events_for_subject("gate-1").len(), 2);
        assert!(log.events_for_subject("missing").is_empty());
    }

    #[test]
    fn audit_log_detects_approval_for_gate() {
        let gate_id = ApprovalGateId::new("gate-1");
        let decision =
            ApprovalDecision::approved(gate_id.clone(), "thomas", "Approved after dry-run review.");
        let log =
            AuditLog::from_events(vec![AuditEvent::from_decision("event-approval", &decision)]);

        assert!(log.contains_approval_for_gate(&gate_id));
        assert!(!log.contains_approval_for_gate(&ApprovalGateId::new("other")));
    }

    #[test]
    fn local_write_gate_helper_creates_initial_audit_trail() {
        let (gate, log) = local_write_gate_with_audit(
            "gate-local-write",
            "action-local-write",
            "Apply draft",
            "Apply approved draft operations.",
            vec!["docs/example.md".to_string()],
        );

        assert_eq!(gate.id().as_str(), "gate-local-write");
        assert_eq!(gate.requirement().gate_kind(), ApprovalGateKind::LocalWrite);
        assert_eq!(log.len(), 2);
        assert_eq!(log.events()[0].event_kind(), AuditEventKind::ActionProposed);
        assert_eq!(
            log.events()[1].event_kind(),
            AuditEventKind::ApprovalRequested
        );
    }
}
