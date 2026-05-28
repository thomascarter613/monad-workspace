//! JSON verification report rendering.
//!
//! WP-E4-006 adds machine-readable `monad check --format json` output.
//! This is intentionally a near-term internal structure, not yet a permanent
//! public schema contract.

use serde_json::{Value, json};

use crate::{CheckDefinition, CheckRegistry, CheckResult, CheckRunReport, CommandResult};

/// Renders a check run report as pretty JSON text.
#[must_use]
pub fn render_check_run_report_json(report: &CheckRunReport) -> String {
    let value = check_run_report_json_value(report);

    serde_json::to_string_pretty(&value).unwrap_or_else(|error| {
        json!({
            "schema_version": 1,
            "kind": "monad_check_report",
            "result": "error",
            "error": {
                "code": "MONAD-CHECK-JSON-0001",
                "message": format!("failed to serialize check report JSON: {error}")
            }
        })
        .to_string()
    })
}

/// Builds the JSON value for a check run report.
///
/// Keeping this separate from string rendering makes tests and future report
/// formats easier to write.
#[must_use]
pub fn check_run_report_json_value(report: &CheckRunReport) -> Value {
    json!({
        "schema_version": 1,
        "kind": "monad_check_report",
        "result": if report.has_failures() { "failed" } else { "passed" },
        "summary": {
            "checks_run": report.result_count(),
            "passed": report.passed_count(),
            "failed": report.failed_count(),
            "warnings": report.warning_count(),
            "skipped": report.skipped_count(),
            "commands": report.command_results().len()
        },
        "checks": report
            .results()
            .iter()
            .map(|result| check_result_json_value(report.registry(), result))
            .collect::<Vec<_>>(),
        "commands": report
            .command_results()
            .iter()
            .map(command_result_json_value)
            .collect::<Vec<_>>()
    })
}

fn check_result_json_value(registry: &CheckRegistry, result: &CheckResult) -> Value {
    let definition = registry.get(result.check_id());

    json!({
        "id": result.check_id().as_str(),
        "name": definition.map(CheckDefinition::name).unwrap_or("unknown check"),
        "severity": definition
            .map(CheckDefinition::severity)
            .map(|severity| severity.as_str())
            .unwrap_or("unknown"),
        "status": result.status().as_str(),
        "message": result.message()
    })
}

fn command_result_json_value(result: &CommandResult) -> Value {
    json!({
        "command": result.command(),
        "working_directory": result.working_directory(),
        "exit_code": result.exit_code(),
        "success": result.success(),
        "stdout_summary": first_non_empty_line(result.stdout()),
        "stderr_summary": first_non_empty_line(result.stderr())
    })
}

fn first_non_empty_line(text: &str) -> Option<String> {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        CheckDefinition, CheckId, CheckRegistry, CheckResult, CheckRunReport, CheckSeverity,
        CommandResult,
    };

    use super::*;

    #[test]
    fn json_report_contains_summary_and_check_results() -> Result<(), Box<dyn std::error::Error>> {
        let registry = CheckRegistry::from_definitions([CheckDefinition::new(
            CheckId::new("MONAD-CHECK-TEST-0001"),
            "Test check",
            CheckSeverity::Error,
            "A test check.",
        )]);

        let report = CheckRunReport::new(
            registry,
            vec![CheckResult::passed(
                CheckId::new("MONAD-CHECK-TEST-0001"),
                "test passed",
            )],
        );

        let rendered = render_check_run_report_json(&report);
        let parsed: Value = serde_json::from_str(&rendered)?;

        assert_eq!(parsed["schema_version"], 1);
        assert_eq!(parsed["kind"], "monad_check_report");
        assert_eq!(parsed["result"], "passed");
        assert_eq!(parsed["summary"]["checks_run"], 1);
        assert_eq!(parsed["checks"][0]["id"], "MONAD-CHECK-TEST-0001");
        assert_eq!(parsed["checks"][0]["name"], "Test check");
        assert_eq!(parsed["checks"][0]["severity"], "error");
        assert_eq!(parsed["checks"][0]["status"], "passed");
        assert_eq!(parsed["checks"][0]["message"], "test passed");

        Ok(())
    }

    #[test]
    fn json_report_contains_command_summaries() {
        let registry = CheckRegistry::new();
        let report = CheckRunReport::with_command_results(
            registry,
            Vec::new(),
            vec![CommandResult::new(
                "cargo --version",
                ".",
                Some(0),
                true,
                "cargo 1.95.0\nextra\n",
                "",
            )],
        );

        let value = check_run_report_json_value(&report);

        assert_eq!(value["summary"]["commands"], 1);
        assert_eq!(value["commands"][0]["command"], "cargo --version");
        assert_eq!(value["commands"][0]["exit_code"], 0);
        assert_eq!(value["commands"][0]["success"], true);
        assert_eq!(value["commands"][0]["stdout_summary"], "cargo 1.95.0");
        assert!(value["commands"][0]["stderr_summary"].is_null());
    }

    #[test]
    fn json_report_marks_failed_result() {
        let registry = CheckRegistry::new();
        let report = CheckRunReport::new(
            registry,
            vec![CheckResult::failed(
                CheckId::new("MONAD-CHECK-TEST-0002"),
                "test failed",
            )],
        );

        let value = check_run_report_json_value(&report);

        assert_eq!(value["result"], "failed");
        assert_eq!(value["summary"]["failed"], 1);
        assert_eq!(value["checks"][0]["status"], "failed");
    }
}
