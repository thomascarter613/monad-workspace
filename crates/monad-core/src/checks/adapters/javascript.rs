//! JavaScript/package-manager adapter checks.

use crate::{
    AdapterCheckRun, CheckDefinition, CheckId, CheckResult, CheckSeverity,
    RepositoryToolchainDetection, RepositoryToolchainKind, WorkspaceContext,
};

/// Selects conservative JavaScript checks when JavaScript or TypeScript tooling
/// is detected.
///
/// This first version does not run package-manager commands because package
/// manager behavior varies and dependency installation is explicitly out of
/// scope for WP-E4-005.
#[must_use]
pub fn select_javascript_checks(
    context: &WorkspaceContext,
    detection: &RepositoryToolchainDetection,
) -> AdapterCheckRun {
    let definitions = javascript_check_definitions();

    let js_detected = detection.has_toolchain(RepositoryToolchainKind::JavaScript)
        || detection.has_toolchain(RepositoryToolchainKind::TypeScript);

    if !js_detected {
        return AdapterCheckRun::from_parts(
            definitions,
            vec![
                CheckResult::skipped(
                    CheckId::new("MONAD-CHECK-JS-0001"),
                    "JavaScript checks skipped because no JavaScript or TypeScript signals were detected.",
                ),
                CheckResult::skipped(
                    CheckId::new("MONAD-CHECK-JS-0002"),
                    "Package-manager lockfile check skipped because no JavaScript or TypeScript signals were detected.",
                ),
            ],
            Vec::new(),
        );
    }

    let package_json_result = if context.root().join("package.json").is_file() {
        CheckResult::passed(
            CheckId::new("MONAD-CHECK-JS-0001"),
            "JavaScript manifest exists: package.json",
        )
    } else {
        CheckResult::failed(
            CheckId::new("MONAD-CHECK-JS-0001"),
            "JavaScript or TypeScript was detected but package.json was not found at the workspace root.",
        )
    };

    let lockfile_result = if detected_lockfile(context).is_some() {
        CheckResult::passed(
            CheckId::new("MONAD-CHECK-JS-0002"),
            format!(
                "package-manager lockfile exists: {}",
                detected_lockfile(context).unwrap_or("unknown")
            ),
        )
    } else {
        CheckResult::skipped(
            CheckId::new("MONAD-CHECK-JS-0002"),
            "No supported JavaScript lockfile found at the workspace root; command execution skipped.",
        )
    };

    AdapterCheckRun::from_parts(
        definitions,
        vec![package_json_result, lockfile_result],
        Vec::new(),
    )
}

fn javascript_check_definitions() -> Vec<CheckDefinition> {
    vec![
        CheckDefinition::new(
            CheckId::new("MONAD-CHECK-JS-0001"),
            "JavaScript manifest exists",
            CheckSeverity::Warning,
            "Checks whether a detected JavaScript/TypeScript repository has package.json.",
        ),
        CheckDefinition::new(
            CheckId::new("MONAD-CHECK-JS-0002"),
            "JavaScript lockfile exists",
            CheckSeverity::Advisory,
            "Checks whether a supported package-manager lockfile exists at the workspace root.",
        ),
    ]
}

fn detected_lockfile(context: &WorkspaceContext) -> Option<&'static str> {
    [
        "bun.lock",
        "bun.lockb",
        "package-lock.json",
        "pnpm-lock.yaml",
        "yarn.lock",
    ]
    .into_iter()
    .find(|file_name| context.root().join(file_name).is_file())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{RepositoryToolchainSignal, RepositoryToolchainSignalKind};

    use super::*;

    fn unique_temp_root(name: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        std::env::temp_dir().join(format!("monad-js-adapter-{name}-{unique}"))
    }

    fn javascript_detection() -> RepositoryToolchainDetection {
        RepositoryToolchainDetection::from_signals(vec![RepositoryToolchainSignal::new(
            RepositoryToolchainKind::JavaScript,
            RepositoryToolchainSignalKind::Manifest,
            "package.json",
        )])
    }

    #[test]
    fn javascript_checks_are_skipped_when_javascript_is_not_detected() {
        let context = WorkspaceContext::new(".").expect("workspace context should be created");
        let detection = RepositoryToolchainDetection::from_signals(Vec::new());

        let run = select_javascript_checks(&context, &detection);

        assert_eq!(run.results().len(), 2);
        assert!(
            run.results()
                .iter()
                .all(|result| { result.status() == crate::CheckStatus::Skipped })
        );
    }

    #[test]
    fn javascript_checks_pass_for_package_json_and_lockfile() {
        let root = unique_temp_root("package-json-and-lockfile");
        fs::create_dir_all(&root).expect("test root should be created");
        fs::write(root.join("package.json"), "{}\n").expect("package.json should be written");
        fs::write(root.join("bun.lock"), "# lock\n").expect("bun.lock should be written");

        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let run = select_javascript_checks(&context, &javascript_detection());

        assert!(run.results().iter().any(|result| {
            result.check_id().as_str() == "MONAD-CHECK-JS-0001"
                && result.status() == crate::CheckStatus::Passed
        }));
        assert!(run.results().iter().any(|result| {
            result.check_id().as_str() == "MONAD-CHECK-JS-0002"
                && result.status() == crate::CheckStatus::Passed
        }));

        fs::remove_dir_all(&root).ok();
    }
}
