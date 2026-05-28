//! Structured supervised planning.
//!
//! `monad plan` is the safe first step of agent-assisted work. It converts a
//! user intent into a reviewable plan without writing files, running commands,
//! calling real model providers, or starting autonomous execution.

use crate::{MockModelProvider, ModelProvider, ModelRequest, MonadError, MonadResult};

/// One reviewable step in a supervised plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentPlanStep {
    title: String,
    description: String,
    requires_approval: bool,
}

impl AgentPlanStep {
    /// Creates a plan step.
    #[must_use]
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        requires_approval: bool,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            requires_approval,
        }
    }

    /// Returns the step title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the step description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns true when the step requires explicit approval.
    #[must_use]
    pub const fn requires_approval(&self) -> bool {
        self.requires_approval
    }
}

/// Structured plan produced from a user intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentPlan {
    intent: String,
    provider_id: String,
    model_id: Option<String>,
    provider_note: String,
    steps: Vec<AgentPlanStep>,
    non_actions: Vec<String>,
    verification_commands: Vec<String>,
    risks: Vec<String>,
}

impl AgentPlan {
    /// Returns the user intent.
    #[must_use]
    pub fn intent(&self) -> &str {
        &self.intent
    }

    /// Returns the provider ID used to produce or support this plan.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Returns the model ID, if available.
    #[must_use]
    pub fn model_id(&self) -> Option<&str> {
        self.model_id.as_deref()
    }

    /// Returns provider note content.
    #[must_use]
    pub fn provider_note(&self) -> &str {
        &self.provider_note
    }

    /// Returns plan steps.
    #[must_use]
    pub fn steps(&self) -> &[AgentPlanStep] {
        &self.steps
    }

    /// Returns explicit non-actions.
    #[must_use]
    pub fn non_actions(&self) -> &[String] {
        &self.non_actions
    }

    /// Returns recommended verification commands.
    #[must_use]
    pub fn verification_commands(&self) -> &[String] {
        &self.verification_commands
    }

    /// Returns known risks and cautions.
    #[must_use]
    pub fn risks(&self) -> &[String] {
        &self.risks
    }
}

/// Builds a deterministic plan using the built-in mock provider.
///
/// This proves the command path without requiring an external model provider,
/// subscription, API key, network access, streaming, MCP, or tool calls.
pub fn build_local_agent_plan(intent: impl AsRef<str>) -> MonadResult<AgentPlan> {
    let provider = MockModelProvider::new(
        "Mock provider note: this plan was produced locally without a real model API call.",
    );

    build_agent_plan_with_provider(&provider, intent)
}

/// Builds a structured plan using any provider implementing the abstraction.
pub fn build_agent_plan_with_provider(
    provider: &dyn ModelProvider,
    intent: impl AsRef<str>,
) -> MonadResult<AgentPlan> {
    let intent = intent.as_ref().trim();

    if intent.is_empty() {
        return Err(MonadError::invalid_input("plan intent must not be empty"));
    }

    let request = ModelRequest::from_user_prompt("plan", intent);
    let response = provider.complete(&request)?;

    Ok(AgentPlan {
        intent: intent.to_string(),
        provider_id: response.provider_id().to_string(),
        model_id: response.model_id().map(ToOwned::to_owned),
        provider_note: response.content().to_string(),
        steps: default_plan_steps(intent),
        non_actions: default_non_actions(),
        verification_commands: default_verification_commands(),
        risks: default_risks(),
    })
}

/// Renders a plan as deterministic, human-readable Markdown-like text.
#[must_use]
pub fn render_agent_plan(plan: &AgentPlan) -> String {
    let mut lines = Vec::new();

    lines.push("Monad supervised plan".to_string());
    lines.push(String::new());
    lines.push(format!("Intent: {}", plan.intent()));
    lines.push(format!("Provider: {}", plan.provider_id()));

    if let Some(model_id) = plan.model_id() {
        lines.push(format!("Model: {model_id}"));
    } else {
        lines.push("Model: unavailable".to_string());
    }

    lines.push(String::new());
    lines.push("Provider note:".to_string());
    lines.push(format!("- {}", plan.provider_note()));
    lines.push(String::new());

    lines.push("Plan steps:".to_string());
    for (index, step) in plan.steps().iter().enumerate() {
        lines.push(format!(
            "{}. {}{}",
            index + 1,
            step.title(),
            if step.requires_approval() {
                " [approval required]"
            } else {
                ""
            }
        ));
        lines.push(format!("   - {}", step.description()));
    }

    lines.push(String::new());
    lines.push("Verification commands:".to_string());
    for command in plan.verification_commands() {
        lines.push(format!("- `{command}`"));
    }

    lines.push(String::new());
    lines.push("Explicit non-actions:".to_string());
    for non_action in plan.non_actions() {
        lines.push(format!("- {non_action}"));
    }

    lines.push(String::new());
    lines.push("Risks and cautions:".to_string());
    for risk in plan.risks() {
        lines.push(format!("- {risk}"));
    }

    lines.push(String::new());
    lines.push("Status: plan only; no files were written and no commands were run.".to_string());

    lines.join("\n")
}

fn default_plan_steps(intent: &str) -> Vec<AgentPlanStep> {
    vec![
        AgentPlanStep::new(
            "Clarify intent",
            format!("Confirm the requested outcome: {intent}"),
            false,
        ),
        AgentPlanStep::new(
            "Load repo-native context",
            "Use committed project documents, work packets, ADRs, and context files as source of truth.",
            false,
        ),
        AgentPlanStep::new(
            "Identify likely affected areas",
            "List files, modules, commands, and documentation areas likely to be involved before drafting changes.",
            false,
        ),
        AgentPlanStep::new(
            "Draft safe file operations",
            "Represent proposed changes as create, update, delete, skip, conflict, or no-op operations.",
            true,
        ),
        AgentPlanStep::new(
            "Dry-run before apply",
            "Evaluate planned file operations and expose conflicts before writing any files.",
            true,
        ),
        AgentPlanStep::new(
            "Verify and review",
            "Run relevant verification after approved changes and review evidence before committing.",
            true,
        ),
    ]
}

fn default_non_actions() -> Vec<String> {
    vec![
        "No files were created, updated, or deleted.".to_string(),
        "No shell commands were run.".to_string(),
        "No Git state was changed.".to_string(),
        "No commits, pushes, pull requests, or deployments were performed.".to_string(),
        "No real model provider or external AI API was called.".to_string(),
    ]
}

fn default_verification_commands() -> Vec<String> {
    vec![
        "cargo fmt --check".to_string(),
        "cargo test".to_string(),
        "cargo clippy --all-targets --all-features -- -D warnings".to_string(),
        "tools/scripts/verify.sh".to_string(),
    ]
}

fn default_risks() -> Vec<String> {
    vec![
        "This is a planning result, not verified truth.".to_string(),
        "Future file changes require explicit approval and dry-run review.".to_string(),
        "Provider output must not bypass repository policy, verification, or human review."
            .to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{ModelProviderCapabilities, ModelProviderMetadata};

    use super::*;

    #[test]
    fn local_plan_rejects_empty_intent() {
        let error = build_local_agent_plan("   ").expect_err("empty intent should be rejected");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("must not be empty"));
    }

    #[test]
    fn local_plan_contains_intent_and_no_write_non_actions() -> MonadResult<()> {
        let plan = build_local_agent_plan("explain this repository")?;

        assert_eq!(plan.intent(), "explain this repository");
        assert_eq!(plan.provider_id(), "mock");
        assert_eq!(plan.model_id(), Some("mock-model"));
        assert!(!plan.steps().is_empty());
        assert!(
            plan.non_actions()
                .iter()
                .any(|item| item.contains("No files were created"))
        );
        assert!(
            plan.non_actions()
                .iter()
                .any(|item| item.contains("No shell commands were run"))
        );

        Ok(())
    }

    #[test]
    fn plan_can_use_provider_trait() -> MonadResult<()> {
        let metadata = ModelProviderMetadata::new(
            "local-test",
            "Local Test Provider",
            Some("test-model".to_string()),
            ModelProviderCapabilities::mock(),
        );
        let provider = MockModelProvider::with_metadata(metadata, "provider-supported plan");

        let plan = build_agent_plan_with_provider(&provider, "plan a safe change")?;

        assert_eq!(plan.provider_id(), "local-test");
        assert_eq!(plan.model_id(), Some("test-model"));
        assert_eq!(plan.provider_note(), "provider-supported plan");

        Ok(())
    }

    #[test]
    fn rendered_plan_is_reviewable_and_non_mutating() -> MonadResult<()> {
        let plan = build_local_agent_plan("explain this repository")?;
        let rendered = render_agent_plan(&plan);

        assert!(rendered.contains("Monad supervised plan"));
        assert!(rendered.contains("Intent: explain this repository"));
        assert!(rendered.contains("Plan steps:"));
        assert!(rendered.contains("Verification commands:"));
        assert!(rendered.contains("Explicit non-actions:"));
        assert!(rendered.contains("no files were written"));

        Ok(())
    }
}
