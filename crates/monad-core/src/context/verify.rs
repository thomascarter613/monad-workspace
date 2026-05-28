//! Context verification checks for Monad.
//!
//! This module provides guardrails against context rot by verifying that
//! required Monad context files exist and meet minimal structural
//! expectations.
//!
//! **Required context files** are files that the context bridge needs in
//! order to support session handoff, bootstrap, and continuity. Missing
//! required files produce error diagnostics. Missing optional files produce
//! warnings.
//!
//! **Structural checks** verify minimal internal expectations such as YAML
//! frontmatter presence and required Markdown headings. These checks are
//! intentionally shallow — they confirm the scaffolding exists without
//! attempting full semantic validation.

use std::fs;

use crate::diagnostics::{Diagnostic, DiagnosticReport};
use crate::workspace::WorkspaceContext;

// ---------------------------------------------------------------------------
// Required context file definitions
// ---------------------------------------------------------------------------

/// Whether a context file is required or optional.
///
/// Required files produce error diagnostics when missing.
/// Optional files produce warning diagnostics when missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextFileRequirement {
    /// The file must exist for context bridge to function.
    Required,

    /// The file is recommended but not strictly necessary.
    Optional,
}

/// A context file that Monad expects to exist in a healthy workspace.
///
/// Each entry describes a file path relative to the workspace root,
/// its requirement level, a human-readable label, and any headings
/// that the file should contain.
#[derive(Debug, Clone)]
pub struct ExpectedContextFile {
    /// Path relative to workspace root (e.g. `.monad/context/current-state.md`).
    pub path: String,

    /// Human-readable label for diagnostics.
    pub label: String,

    /// Whether the file is required or optional.
    pub requirement: ContextFileRequirement,

    /// Markdown headings that should appear in the file content.
    ///
    /// Each entry is a heading prefix such as `"# Current State"`. The check
    /// looks for lines that start with this prefix (case-sensitive).
    pub expected_headings: Vec<String>,
}

/// Verification result for a single context file.
///
/// Captures whether the file was found, whether it has frontmatter,
/// whether expected headings were present, and any diagnostics produced.
#[derive(Debug, Clone)]
pub struct ContextFileCheckResult {
    /// Path that was checked.
    pub path: String,

    /// Human-readable label.
    pub label: String,

    /// Whether the file was found on disk.
    pub found: bool,

    /// Whether YAML frontmatter was detected (opening `---`).
    pub has_frontmatter: bool,

    /// Headings that were expected but not found.
    pub missing_headings: Vec<String>,

    /// Diagnostics produced by this check.
    pub diagnostics: Vec<Diagnostic>,
}

/// Full context verification report.
///
/// Aggregates individual file check results and provides summary statistics.
#[derive(Debug, Clone)]
pub struct ContextVerificationReport {
    /// Individual file check results.
    pub file_checks: Vec<ContextFileCheckResult>,
}

impl ContextVerificationReport {
    /// Returns the total number of files checked.
    #[must_use]
    pub fn total_checked(&self) -> usize {
        self.file_checks.len()
    }

    /// Returns the number of files that were found.
    #[must_use]
    pub fn found_count(&self) -> usize {
        self.file_checks.iter().filter(|c| c.found).count()
    }

    /// Returns the number of files that were missing.
    #[must_use]
    pub fn missing_count(&self) -> usize {
        self.file_checks.iter().filter(|c| !c.found).count()
    }

    /// Returns true if any check produced an error diagnostic.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.file_checks
            .iter()
            .any(|c| c.diagnostics.iter().any(|d| d.is_error()))
    }

    /// Collects all diagnostics from all file checks into a single report.
    #[must_use]
    pub fn to_diagnostic_report(&self) -> DiagnosticReport {
        let mut report = DiagnosticReport::new();
        for check in &self.file_checks {
            for diagnostic in &check.diagnostics {
                report.push(diagnostic.clone());
            }
        }
        report
    }
}

// ---------------------------------------------------------------------------
// Required file definitions
// ---------------------------------------------------------------------------

/// Returns the list of expected context files for a Monad workspace.
///
/// This is the canonical registry of context artifacts that the context
/// bridge depends on. Each entry defines:
///
/// - The file path relative to workspace root.
/// - Whether it is required or optional.
/// - Expected Markdown headings for structural checks.
///
/// Required files:
/// - `.monad/context/current-state.md` — project status snapshot
/// - `.monad/context/latest-handoff.md` — session continuity artifact
///
/// Optional files:
/// - `.monad/context/latest-context-pack.md` — assembled context pack
/// - `docs/ai/BOOTSTRAP-PROMPT.md` — AI session bootstrap prompt
/// - `monad.toml` — project manifest (checked for completeness)
#[must_use]
pub fn expected_context_files() -> Vec<ExpectedContextFile> {
    vec![
        ExpectedContextFile {
            path: ".monad/context/current-state.md".to_string(),
            label: "Current State".to_string(),
            requirement: ContextFileRequirement::Required,
            expected_headings: vec!["# Current State".to_string(), "## Epics".to_string()],
        },
        ExpectedContextFile {
            path: ".monad/context/latest-handoff.md".to_string(),
            label: "Latest Handoff".to_string(),
            requirement: ContextFileRequirement::Required,
            expected_headings: vec!["# Latest Handoff".to_string()],
        },
        ExpectedContextFile {
            path: ".monad/context/latest-context-pack.md".to_string(),
            label: "Context Pack".to_string(),
            requirement: ContextFileRequirement::Optional,
            expected_headings: vec!["# Context Pack".to_string()],
        },
        ExpectedContextFile {
            path: "docs/ai/BOOTSTRAP-PROMPT.md".to_string(),
            label: "Bootstrap Prompt".to_string(),
            requirement: ContextFileRequirement::Optional,
            expected_headings: vec![
                "# Bootstrap Prompt".to_string(),
                "## Required Reading Order".to_string(),
            ],
        },
        ExpectedContextFile {
            path: "monad.toml".to_string(),
            label: "Project Manifest".to_string(),
            requirement: ContextFileRequirement::Required,
            expected_headings: Vec::new(), // TOML, not Markdown — no heading checks.
        },
    ]
}

// ---------------------------------------------------------------------------
// Verification logic
// ---------------------------------------------------------------------------

/// Verifies context files in the given workspace.
///
/// Checks each expected context file for:
/// 1. Existence on disk.
/// 2. YAML frontmatter presence (for Markdown files).
/// 3. Expected Markdown headings.
///
/// Returns a structured report with per-file results and diagnostics.
#[must_use]
pub fn verify_context(context: &WorkspaceContext) -> ContextVerificationReport {
    let expected_files = expected_context_files();
    let mut file_checks = Vec::with_capacity(expected_files.len());

    for expected in &expected_files {
        file_checks.push(check_context_file(context, expected));
    }

    ContextVerificationReport { file_checks }
}

/// Checks a single expected context file.
fn check_context_file(
    context: &WorkspaceContext,
    expected: &ExpectedContextFile,
) -> ContextFileCheckResult {
    let full_path = context.root().join(&expected.path);
    let mut diagnostics = Vec::new();

    // 1. Check existence.
    if !full_path.is_file() {
        let diagnostic = match expected.requirement {
            ContextFileRequirement::Required => Diagnostic::error(
                "MONAD5001",
                format!(
                    "required context file missing: {} ({})",
                    expected.path, expected.label
                ),
            ),
            ContextFileRequirement::Optional => Diagnostic::warning(
                "MONAD5002",
                format!(
                    "optional context file missing: {} ({})",
                    expected.path, expected.label
                ),
            ),
        };

        diagnostics.push(diagnostic);

        return ContextFileCheckResult {
            path: expected.path.clone(),
            label: expected.label.clone(),
            found: false,
            has_frontmatter: false,
            missing_headings: expected.expected_headings.clone(),
            diagnostics,
        };
    }

    // File exists — report success.
    diagnostics.push(Diagnostic::info(
        "MONAD5000",
        format!("context file found: {} ({})", expected.path, expected.label),
    ));

    // 2. Read content for structural checks.
    let content = match fs::read_to_string(&full_path) {
        Ok(c) => c,
        Err(error) => {
            diagnostics.push(Diagnostic::warning(
                "MONAD5003",
                format!(
                    "could not read context file for structural checks: {} — {}",
                    expected.path, error
                ),
            ));

            return ContextFileCheckResult {
                path: expected.path.clone(),
                label: expected.label.clone(),
                found: true,
                has_frontmatter: false,
                missing_headings: expected.expected_headings.clone(),
                diagnostics,
            };
        }
    };

    // 3. Check for YAML frontmatter (Markdown files only).
    let is_markdown = expected.path.ends_with(".md");
    let has_frontmatter = if is_markdown {
        check_frontmatter(&content)
    } else {
        false // Not applicable for non-Markdown files.
    };

    if is_markdown && !has_frontmatter {
        diagnostics.push(Diagnostic::warning(
            "MONAD5004",
            format!(
                "context file missing YAML frontmatter: {} ({})",
                expected.path, expected.label
            ),
        ));
    }

    // 4. Check expected headings.
    let missing_headings = find_missing_headings(&content, &expected.expected_headings);

    if !missing_headings.is_empty() {
        for heading in &missing_headings {
            diagnostics.push(Diagnostic::warning(
                "MONAD5005",
                format!(
                    "context file missing expected heading: {} — '{}' ({})",
                    expected.path, heading, expected.label
                ),
            ));
        }
    }

    ContextFileCheckResult {
        path: expected.path.clone(),
        label: expected.label.clone(),
        found: true,
        has_frontmatter,
        missing_headings,
        diagnostics,
    }
}

/// Checks whether content starts with YAML frontmatter (`---`).
///
/// Frontmatter is detected by checking whether the first non-empty line
/// is exactly `---`.
fn check_frontmatter(content: &str) -> bool {
    content
        .lines()
        .find(|line| !line.trim().is_empty())
        .is_some_and(|first_line| first_line.trim() == "---")
}

/// Returns headings from `expected` that are not found in `content`.
///
/// A heading is considered present if any line in the content starts with the
/// expected heading prefix. This allows minor trailing variations.
fn find_missing_headings(content: &str, expected: &[String]) -> Vec<String> {
    expected
        .iter()
        .filter(|heading| {
            !content
                .lines()
                .any(|line| line.starts_with(heading.as_str()))
        })
        .cloned()
        .collect()
}

/// Renders a human-readable summary of the context verification report.
///
/// This is used by the CLI to display the result of `monad context verify`.
#[must_use]
pub fn render_context_verify_summary(report: &ContextVerificationReport) -> String {
    let mut lines = Vec::new();

    if report.has_errors() {
        lines.push("Monad context verification: FAILED".to_string());
    } else {
        lines.push("Monad context verification: PASSED".to_string());
    }

    lines.push(format!("  checked: {} files", report.total_checked()));
    lines.push(format!("  found: {}", report.found_count()));
    lines.push(format!("  missing: {}", report.missing_count()));

    lines.push(String::new());

    for check in &report.file_checks {
        let status = if check.found { "✓" } else { "✗" };
        lines.push(format!("  {status} {} ({})", check.path, check.label));

        if check.found && !check.missing_headings.is_empty() {
            for heading in &check.missing_headings {
                lines.push(format!("    ⚠ missing heading: '{heading}'"));
            }
        }

        if check.found && check.path.ends_with(".md") && !check.has_frontmatter {
            lines.push("    ⚠ missing YAML frontmatter".to_string());
        }
    }

    lines.join("\n")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    // -- Test helpers --------------------------------------------------------

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-context-verify-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_workspace_with_context(test_name: &str, files: &[(&str, &str)]) -> PathBuf {
        let root = unique_temp_dir(test_name);
        fs::create_dir_all(root.join(".monad/context")).expect(".monad/context should be created");
        fs::create_dir_all(root.join("docs/ai")).expect("docs/ai should be created");

        // Always create monad.toml so WorkspaceContext can be built.
        fs::write(
            root.join("monad.toml"),
            "[project]\nname = \"test\"\ndisplay_name = \"Test\"\n",
        )
        .expect("monad.toml should be written");

        for (path, content) in files {
            let full = root.join(path);
            if let Some(parent) = full.parent() {
                fs::create_dir_all(parent).expect("parent directory should be created");
            }
            fs::write(full, content).expect("file should be written");
        }

        root
    }

    // -- expected_context_files -----------------------------------------------

    #[test]
    fn expected_context_files_returns_five_entries() {
        let files = expected_context_files();
        assert_eq!(files.len(), 5);
    }

    #[test]
    fn expected_context_files_includes_required_and_optional() {
        let files = expected_context_files();

        let required_count = files
            .iter()
            .filter(|f| f.requirement == ContextFileRequirement::Required)
            .count();

        let optional_count = files
            .iter()
            .filter(|f| f.requirement == ContextFileRequirement::Optional)
            .count();

        assert_eq!(required_count, 3); // current-state, latest-handoff, monad.toml
        assert_eq!(optional_count, 2); // context-pack, bootstrap-prompt
    }

    // -- check_frontmatter ---------------------------------------------------

    #[test]
    fn frontmatter_detected_when_present() {
        let content = "---\ntitle: test\n---\n\n# Heading\n";
        assert!(check_frontmatter(content));
    }

    #[test]
    fn frontmatter_not_detected_when_absent() {
        let content = "# Heading\n\nSome content.\n";
        assert!(!check_frontmatter(content));
    }

    #[test]
    fn frontmatter_detected_with_leading_blank_lines() {
        // The implementation looks for the first non-empty line.
        let content = "\n\n---\ntitle: test\n---\n";
        assert!(check_frontmatter(content));
    }

    #[test]
    fn frontmatter_not_detected_for_empty_content() {
        assert!(!check_frontmatter(""));
        assert!(!check_frontmatter("   \n   \n"));
    }

    // -- find_missing_headings -----------------------------------------------

    #[test]
    fn all_headings_present_returns_empty() {
        let content = "# Current State\n\n## Epics\n\nSome content.\n";
        let expected = vec!["# Current State".to_string(), "## Epics".to_string()];

        assert!(find_missing_headings(content, &expected).is_empty());
    }

    #[test]
    fn missing_heading_is_reported() {
        let content = "# Current State\n\nSome content.\n";
        let expected = vec!["# Current State".to_string(), "## Epics".to_string()];

        let missing = find_missing_headings(content, &expected);
        assert_eq!(missing, vec!["## Epics"]);
    }

    #[test]
    fn no_expected_headings_returns_empty() {
        let content = "some content without headings";
        let expected: Vec<String> = Vec::new();

        assert!(find_missing_headings(content, &expected).is_empty());
    }

    // -- verify_context (integration) ----------------------------------------

    #[test]
    fn verify_context_reports_all_missing_files() {
        let root = unique_temp_dir("all-missing");
        fs::create_dir_all(&root).expect("root should be created");
        fs::write(
            root.join("monad.toml"),
            "[project]\nname = \"test\"\ndisplay_name = \"Test\"\n",
        )
        .expect("monad.toml should be written");

        let context = WorkspaceContext::new(&root).expect("context should be created");
        let report = verify_context(&context);

        // monad.toml exists, but 4 others are missing.
        assert_eq!(report.missing_count(), 4);
        assert!(report.has_errors()); // current-state and latest-handoff are required.

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn verify_context_passes_with_all_required_files() {
        let root = create_workspace_with_context(
            "all-required",
            &[
                (
                    ".monad/context/current-state.md",
                    "---\ntitle: Current State\n---\n\n# Current State\n\n## Epics\n",
                ),
                (
                    ".monad/context/latest-handoff.md",
                    "---\ntitle: Latest Handoff\n---\n\n# Latest Handoff\n",
                ),
            ],
        );

        let context = WorkspaceContext::new(&root).expect("context should be created");
        let report = verify_context(&context);

        // monad.toml + current-state + latest-handoff = 3 found.
        // context-pack + bootstrap-prompt = 2 missing but optional.
        assert!(!report.has_errors());
        assert_eq!(report.found_count(), 3);
        assert_eq!(report.missing_count(), 2);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn verify_context_passes_with_all_files_present() {
        let root = create_workspace_with_context(
            "all-present",
            &[
                (
                    ".monad/context/current-state.md",
                    "---\ntitle: Current State\n---\n\n# Current State\n\n## Epics\n",
                ),
                (
                    ".monad/context/latest-handoff.md",
                    "---\ntitle: Latest Handoff\n---\n\n# Latest Handoff\n",
                ),
                (
                    ".monad/context/latest-context-pack.md",
                    "---\ntitle: Context Pack\n---\n\n# Context Pack\n",
                ),
                (
                    "docs/ai/BOOTSTRAP-PROMPT.md",
                    "---\ntitle: Bootstrap Prompt\n---\n\n# Bootstrap Prompt\n\n## Required Reading Order\n",
                ),
            ],
        );

        let context = WorkspaceContext::new(&root).expect("context should be created");
        let report = verify_context(&context);

        assert!(!report.has_errors());
        assert_eq!(report.found_count(), 5);
        assert_eq!(report.missing_count(), 0);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn verify_context_warns_on_missing_frontmatter() {
        let root = create_workspace_with_context(
            "missing-frontmatter",
            &[
                (
                    ".monad/context/current-state.md",
                    "# Current State\n\n## Epics\n", // No frontmatter.
                ),
                (
                    ".monad/context/latest-handoff.md",
                    "---\ntitle: Latest Handoff\n---\n\n# Latest Handoff\n",
                ),
            ],
        );

        let context = WorkspaceContext::new(&root).expect("context should be created");
        let report = verify_context(&context);

        // No errors — all required files exist. But current-state has no frontmatter.
        assert!(!report.has_errors());

        let current_state_check = report
            .file_checks
            .iter()
            .find(|c| c.path.contains("current-state"))
            .expect("current-state check should exist");

        assert!(!current_state_check.has_frontmatter);

        // Should have a MONAD5004 warning.
        let has_frontmatter_warning = current_state_check
            .diagnostics
            .iter()
            .any(|d| d.code == "MONAD5004");

        assert!(has_frontmatter_warning);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn verify_context_warns_on_missing_headings() {
        let root = create_workspace_with_context(
            "missing-headings",
            &[
                (
                    ".monad/context/current-state.md",
                    "---\ntitle: Current State\n---\n\n# Current State\n", // Missing ## Epics.
                ),
                (
                    ".monad/context/latest-handoff.md",
                    "---\ntitle: Latest Handoff\n---\n\n# Latest Handoff\n",
                ),
            ],
        );

        let context = WorkspaceContext::new(&root).expect("context should be created");
        let report = verify_context(&context);

        assert!(!report.has_errors());

        let current_state_check = report
            .file_checks
            .iter()
            .find(|c| c.path.contains("current-state"))
            .expect("current-state check should exist");

        assert_eq!(current_state_check.missing_headings, vec!["## Epics"]);

        // Should have a MONAD5005 warning.
        let has_heading_warning = current_state_check
            .diagnostics
            .iter()
            .any(|d| d.code == "MONAD5005");

        assert!(has_heading_warning);

        fs::remove_dir_all(root).ok();
    }

    // -- render_context_verify_summary ---------------------------------------

    #[test]
    fn render_summary_shows_passed_when_no_errors() {
        let report = ContextVerificationReport {
            file_checks: vec![ContextFileCheckResult {
                path: "monad.toml".to_string(),
                label: "Project Manifest".to_string(),
                found: true,
                has_frontmatter: false,
                missing_headings: Vec::new(),
                diagnostics: vec![Diagnostic::info(
                    "MONAD5000",
                    "context file found: monad.toml (Project Manifest)",
                )],
            }],
        };

        let summary = render_context_verify_summary(&report);
        assert!(summary.contains("PASSED"));
        assert!(summary.contains("found: 1"));
        assert!(summary.contains("missing: 0"));
    }

    #[test]
    fn render_summary_shows_failed_when_errors_exist() {
        let report = ContextVerificationReport {
            file_checks: vec![ContextFileCheckResult {
                path: ".monad/context/current-state.md".to_string(),
                label: "Current State".to_string(),
                found: false,
                has_frontmatter: false,
                missing_headings: vec!["# Current State".to_string()],
                diagnostics: vec![Diagnostic::error(
                    "MONAD5001",
                    "required context file missing: .monad/context/current-state.md (Current State)",
                )],
            }],
        };

        let summary = render_context_verify_summary(&report);
        assert!(summary.contains("FAILED"));
        assert!(summary.contains("missing: 1"));
        assert!(summary.contains("✗"));
    }

    #[test]
    fn render_summary_shows_warning_indicators() {
        let report = ContextVerificationReport {
            file_checks: vec![ContextFileCheckResult {
                path: ".monad/context/current-state.md".to_string(),
                label: "Current State".to_string(),
                found: true,
                has_frontmatter: false,
                missing_headings: vec!["## Epics".to_string()],
                diagnostics: vec![
                    Diagnostic::info("MONAD5000", "context file found"),
                    Diagnostic::warning("MONAD5004", "missing YAML frontmatter"),
                    Diagnostic::warning("MONAD5005", "missing heading: ## Epics"),
                ],
            }],
        };

        let summary = render_context_verify_summary(&report);
        assert!(summary.contains("PASSED"));
        assert!(summary.contains("⚠ missing heading"));
        assert!(summary.contains("⚠ missing YAML frontmatter"));
    }

    // -- ContextVerificationReport methods -----------------------------------

    #[test]
    fn report_to_diagnostic_report_collects_all_diagnostics() {
        let report = ContextVerificationReport {
            file_checks: vec![
                ContextFileCheckResult {
                    path: "a.md".to_string(),
                    label: "A".to_string(),
                    found: true,
                    has_frontmatter: true,
                    missing_headings: Vec::new(),
                    diagnostics: vec![Diagnostic::info("MONAD5000", "found a.md")],
                },
                ContextFileCheckResult {
                    path: "b.md".to_string(),
                    label: "B".to_string(),
                    found: false,
                    has_frontmatter: false,
                    missing_headings: Vec::new(),
                    diagnostics: vec![Diagnostic::error("MONAD5001", "missing b.md")],
                },
            ],
        };

        let diagnostic_report = report.to_diagnostic_report();
        assert_eq!(diagnostic_report.len(), 2);
        assert!(diagnostic_report.has_errors());
    }

    // -- Integration test with real workspace --------------------------------

    #[test]
    fn verify_context_from_cargo_manifest_dir() {
        // Run against the actual monad-workspace repo root.
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let repo_root = std::path::Path::new(manifest_dir)
            .parent() // crates/
            .and_then(|p| p.parent()) // repo root
            .expect("repo root should exist");

        let context = WorkspaceContext::new(repo_root)
            .expect("workspace context should be created from repo root");

        let report = verify_context(&context);

        // monad.toml should always be found.
        let manifest_check = report
            .file_checks
            .iter()
            .find(|c| c.path == "monad.toml")
            .expect("monad.toml check should exist");

        assert!(manifest_check.found);

        // At minimum we should have checked all 5 expected files.
        assert_eq!(report.total_checked(), 5);
    }
}
