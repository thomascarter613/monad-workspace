---
title: "Public Pre-Release Verification Audit"
document_type: "release-verification-audit"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-005
tags:
  - monad
  - release
  - public-prerelease
  - verification
  - audit
related:
  - README.md
  - docs/release/PUBLIC-CLAIMS-AUDIT.md
  - docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md
  - docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
  - docs/release/PUBLIC-PRERELEASE-NOTES.md
---

# Public Pre-Release Verification Audit

## Status

Pass.

## Work Packet

WP-E10-005 — Run final public pre-release verification audit.

## Purpose

This audit captures final verification evidence before the public pre-release tag decision.

It does not approve or cut a release.

Release approval remains governed by:

```text
WP-E10-006 — Decide and cut first public pre-release tag, if approved
```

## Run Metadata

| Field | Value |
| --- | --- |
| Started At | 2026-06-04T05:09:51Z |
| Completed At | 2026-06-04T05:09:59Z |
| Total Checks | 17 |
| Failed Checks | 0 |
| Overall Status | Pass |
| Raw Log Directory | `.monad/reports/e10/wp-e10-005` |

## Decision Guidance

Final verification audit passed. WP-E10-006 may proceed to the public pre-release tag go/no-go decision.

## Verification Summary

| # | Command | Status | Exit Code | Raw Log |
| ---: | --- | --- | ---: | --- |

| 1 | `git status --short` | Pass | 0 | `.monad/reports/e10/wp-e10-005/git-status-before.log` |

| 2 | `cargo fmt --check` | Pass | 0 | `.monad/reports/e10/wp-e10-005/cargo-fmt.log` |

| 3 | `cargo test` | Pass | 0 | `.monad/reports/e10/wp-e10-005/cargo-test.log` |

| 4 | `cargo clippy --all-targets --all-features -- -D warnings` | Pass | 0 | `.monad/reports/e10/wp-e10-005/cargo-clippy.log` |

| 5 | `cargo run -p monad-cli -- --help` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-help.log` |

| 6 | `cargo run -p monad-cli -- version` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-version.log` |

| 7 | `cargo run -p monad-cli -- info` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-info.log` |

| 8 | `cargo run -p monad-cli -- inspect` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-inspect.log` |

| 9 | `cargo run -p monad-cli -- check` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-check.log` |

| 10 | `cargo run -p monad-cli -- graph` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-graph.log` |

| 11 | `cargo run -p monad-cli -- context` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-context.log` |

| 12 | `cargo run -p monad-cli -- context verify` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-context-verify.log` |

| 13 | `cargo run -p monad-cli -- plan "explain this repository"` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-plan.log` |

| 14 | `cargo run -p monad-cli -- evolve verify-baseline --dry-run` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-evolve-verify-baseline.log` |

| 15 | `cargo run -p monad-cli -- evolve context-baseline --dry-run` | Pass | 0 | `.monad/reports/e10/wp-e10-005/monad-evolve-context-baseline.log` |

| 16 | `tools/scripts/verify.sh` | Pass | 0 | `.monad/reports/e10/wp-e10-005/root-verify.log` |

| 17 | `git status --short` | Pass | 0 | `.monad/reports/e10/wp-e10-005/git-status-after.log` |


## Detailed Command Evidence

### 1. git-status-before

```bash
git status --short
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/git-status-before.log`

<details>
<summary>Command output</summary>

```text
?? complete_wp_e10_005_public_prerelease_verification_audit.sh
```

</details>


### 2. cargo-fmt

```bash
cargo fmt --check
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/cargo-fmt.log`

<details>
<summary>Command output</summary>

```text
```

</details>


### 3. cargo-test

```bash
cargo test
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/cargo-test.log`

<details>
<summary>Command output</summary>

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/main.rs (target/debug/deps/monad-b4e8f786dd2c8393)

running 37 tests
test tests::check_command_parses_text_and_json_formats ... ok
test tests::context_command_parses_write_flag ... ok
test tests::context_generate_bootstrap_parses ... ok
test tests::context_command_rejects_unknown_formats ... ok
test tests::context_generate_current_state_parses ... ok
test tests::context_command_parses_supported_formats ... ok
test tests::context_generate_handoff_parses ... ok
test tests::context_generate_unknown_artifact_returns_error ... ok
test tests::context_generate_without_artifact_returns_error ... ok
test tests::context_pack_parses ... ok
test tests::context_unknown_subcommand_returns_error ... ok
test tests::context_verify_parses ... ok
test tests::evolve_context_baseline_requires_dry_run_with_specific_error ... ok
test tests::evolve_dry_run_commands_parse ... ok
test tests::evolve_verify_baseline_requires_dry_run_with_specific_error ... ok
test tests::format_can_appear_before_command ... ok
test tests::graph_command_parses_supported_formats ... ok
test tests::graph_command_rejects_unknown_formats ... ok
test tests::help_command_parses ... ok
test tests::help_text_mentions_context_command_formats_and_write_mode ... ok
test tests::help_text_mentions_context_generate_bootstrap ... ok
test tests::help_text_mentions_context_generate_current_state ... ok
test tests::help_text_mentions_context_generate_handoff ... ok
test tests::help_text_mentions_context_pack ... ok
test tests::help_text_mentions_context_verify ... ok
test tests::help_text_mentions_graph_command_and_formats ... ok
test tests::info_command_parses_text_and_json_formats ... ok
test tests::inspect_command_parses_text_and_json_formats ... ok
test tests::help_text_mentions_plan_and_evolve_dry_run_commands ... ok
test tests::no_command_defaults_to_info ... ok
test tests::non_graph_commands_reject_graph_only_formats ... ok
test tests::plan_command_parses_multi_word_intent ... ok
test tests::plan_rejects_format_flags_for_now ... ok
test tests::plan_without_intent_returns_actionable_error ... ok
test tests::unknown_command_returns_error ... ok
test tests::version_command_parses ... ok
test tests::write_flag_is_rejected_for_non_context_commands ... ok

test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli_smoke.rs (target/debug/deps/cli_smoke-6557de83d18f713e)

running 13 tests
test evolve_verify_baseline_requires_dry_run_smoke_test ... ok
test evolve_context_baseline_requires_dry_run_smoke_test ... ok
test plan_unsupported_format_failure_smoke_test ... ok
test unsupported_argument_failure_smoke_test ... ok
test unsupported_write_flag_failure_smoke_test ... ok
test plan_missing_intent_failure_smoke_test ... ok
test evolve_verify_baseline_dry_run_smoke_test ... ok
test evolve_context_baseline_dry_run_smoke_test ... ok
test help_command_smoke_test ... ok
test plan_command_smoke_test ... ok
test version_command_smoke_test ... ok
test inspect_command_smoke_test ... ok
test check_command_smoke_test ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s

     Running unittests src/lib.rs (target/debug/deps/monad_core-5025b4ec710d14a4)

running 315 tests
test agents::draft::tests::draft_state_labels_are_stable ... ok
test agents::draft::tests::draft_operation_can_be_created_from_planned_operation ... ok
test agents::draft::tests::draft_can_be_created_from_file_operation_plan ... ok
test agents::draft::tests::sandbox_kind_identifies_git_isolation ... ok
test agents::draft::tests::rendered_draft_is_reviewable_and_non_mutating ... ok
test agents::draft::tests::terminal_draft_state_cannot_be_reopened_by_review_or_approval ... ok
test agents::model::tests::message_role_labels_are_stable ... ok
test agents::model::tests::model_request_preserves_purpose_and_messages ... ok
test agents::model::tests::model_response_preserves_provider_model_and_content ... ok
test agents::model::tests::user_prompt_request_contains_one_user_message ... ok
test agents::plan::tests::local_plan_contains_intent_and_no_write_non_actions ... ok
test agents::plan::tests::local_plan_rejects_empty_intent ... ok
test agents::plan::tests::plan_can_use_provider_trait ... ok
test agents::provider::tests::capability_metadata_describes_mock_provider ... ok
test agents::provider::tests::mock_provider_rejects_empty_requests ... ok
test agents::plan::tests::rendered_plan_is_reviewable_and_non_mutating ... ok
test agents::provider::tests::mock_provider_returns_fixed_response_without_external_api ... ok
test agents::provider::tests::provider_metadata_exposes_stable_fields ... ok
test agents::provider::tests::provider_trait_can_be_used_through_trait_object ... ok
test checks::adapters::javascript::tests::javascript_checks_are_skipped_when_javascript_is_not_detected ... ok
test checks::adapters::rust::tests::rust_checks_are_skipped_when_rust_is_not_detected ... ok
test checks::adapters::tests::selected_adapter_checks_include_javascript_when_javascript_detected ... ok
test checks::evidence::tests::evidence_check_result_preserves_check_status ... ok
test checks::evidence::tests::evidence_command_summary_captures_first_output_lines ... ok
test checks::adapters::javascript::tests::javascript_checks_pass_for_package_json_and_lockfile ... ok
test checks::evidence::tests::evidence_packet_summarizes_check_report ... ok
test checks::json::tests::json_report_contains_command_summaries ... ok
test checks::model::tests::check_definition_exposes_metadata ... ok
test checks::json::tests::json_report_marks_failed_result ... ok
test checks::json::tests::json_report_contains_summary_and_check_results ... ok
test checks::model::tests::check_result_constructors_set_status ... ok
test checks::model::tests::status_labels_are_stable ... ok
test checks::registry::tests::registry_definitions_are_deterministically_ordered_by_id ... ok
test checks::model::tests::check_id_preserves_stable_text ... ok
test checks::registry::tests::registry_registers_and_finds_checks ... ok
test checks::report::tests::markdown_report_escapes_table_pipes ... ok
test checks::report::tests::markdown_report_includes_summary_and_check_results ... ok
test checks::model::tests::severity_labels_are_stable ... ok
test checks::registry::tests::registry_starts_empty ... ok
test checks::run::tests::report_counts_statuses ... ok
test checks::run::tests::required_file_check_fails_when_file_is_missing ... ok
test checks::registry::tests::registry_replaces_duplicate_check_ids ... ok
test checks::tests::check_model_exports_are_usable_from_checks_boundary ... ok
test context::bootstrap::tests::extract_description_returns_none_for_empty ... ok
test checks::run::tests::report_renders_human_readable_summary ... ok
test checks::run::tests::initial_registry_contains_expected_checks ... ok
test checks::report::tests::write_check_evidence_packet_writes_latest_report ... ok
test context::bootstrap::tests::extract_description_skips_badges ... ok
test context::bootstrap::tests::extract_project_name_from_toml ... ok
test context::bootstrap::tests::extract_project_name_missing_falls_back ... ok
test context::bootstrap::tests::extract_description_skips_frontmatter_and_headers ... ok
test context::bootstrap::tests::extract_project_name_no_project_section_falls_back ... ok
test context::bootstrap::tests::render_includes_continuation_protocol ... ok
test context::bootstrap::tests::render_includes_current_work ... ok
test context::bootstrap::tests::render_epic_progress_is_included ... ok
test context::bootstrap::tests::render_includes_project_identity ... ok
test context::bootstrap::tests::extract_project_name_prefers_display_name ... ok
test context::bootstrap::tests::render_includes_reading_order ... ok
test context::bootstrap::tests::render_includes_response_expectations ... ok
test context::bootstrap::tests::render_includes_frontmatter ... ok
test context::bootstrap::tests::render_includes_source_of_truth ... ok
test context::bootstrap::tests::render_includes_workflow_rules ... ok
test context::bootstrap::tests::source_of_truth_mentions_repository ... ok
test context::bootstrap::tests::render_with_no_active_work_packet ... ok
test context::bootstrap::tests::render_is_deterministic ... ok
test context::bootstrap::tests::render_with_no_active_epic ... ok
test context::bootstrap::tests::workflow_rules_mention_conventional_commits ... ok
test context::bootstrap::tests::workflow_rules_are_non_empty ... ok
test context::current_state::tests::current_state_artifact_returns_none_when_no_active_epic ... ok
test context::current_state::tests::current_state_artifact_finds_active_epic ... ok
test context::current_state::tests::extract_frontmatter_value_handles_quoted_values ... ok
test context::current_state::tests::extract_frontmatter_value_handles_unquoted_values ... ok
test context::current_state::tests::extract_frontmatter_value_returns_none_for_empty_value ... ok
test context::current_state::tests::extract_frontmatter_value_returns_none_for_wrong_key ... ok
test context::current_state::tests::natural_epic_sort_orders_correctly ... ok
test context::current_state::tests::parse_epic_frontmatter_extracts_fields ... ok
test context::current_state::tests::parse_epic_frontmatter_handles_in_progress ... ok
test context::current_state::tests::parse_epic_frontmatter_returns_none_without_epic_id ... ok
test context::current_state::tests::read_runtime_modules_from_content ... ok
test context::current_state::tests::render_current_state_handles_no_epics ... ok
test context::current_state::tests::render_current_state_includes_required_sections ... ok
test context::current_state::tests::render_current_state_is_deterministic ... ok
test context::current_state::tests::strip_epic_prefix_removes_em_dash_separator ... ok
test context::current_state::tests::strip_epic_prefix_removes_hyphen_separator ... ok
test context::handoff::tests::collapse_hyphens_collapses_multiple ... ok
test context::current_state::tests::generate_current_state_from_workspace ... ok
test context::handoff::tests::epic_filename_generates_slug ... ok
test context::bootstrap::tests::generate_bootstrap_prompt_from_workspace_produces_artifact ... ok
test context::current_state::tests::strip_epic_prefix_preserves_title_without_prefix ... ok
test context::handoff::tests::extract_frontmatter_field_handles_quoted_values ... ok
test context::handoff::tests::handoff_artifact_finds_active_work_packet ... ok
test context::handoff::tests::extract_frontmatter_field_handles_unquoted_values ... ok
test context::handoff::tests::extract_frontmatter_field_returns_none_for_empty_value ... ok
test context::handoff::tests::extract_frontmatter_field_returns_none_for_wrong_key ... ok
test context::handoff::tests::handoff_artifact_returns_none_when_no_active_work_packet ... ok
test context::handoff::tests::natural_wp_sort_orders_correctly ... ok
test context::handoff::tests::parse_work_packet_frontmatter_extracts_fields ... ok
test context::handoff::tests::parse_work_packet_frontmatter_handles_complete_status ... ok
test context::handoff::tests::parse_work_packet_frontmatter_returns_none_without_id ... ok
test context::current_state::tests::write_and_read_current_state_artifact ... ok
test context::handoff::tests::render_handoff_handles_no_active_work ... ok
test context::handoff::tests::render_handoff_is_deterministic ... ok
test context::handoff::tests::render_handoff_includes_required_sections ... ok
test context::handoff::tests::render_handoff_next_action_suggests_pending_when_none_active ... ok
test context::handoff::tests::strip_wp_prefix_removes_em_dash ... ok
test context::handoff::tests::strip_wp_prefix_removes_hyphen ... ok
test context::handoff::tests::strip_wp_prefix_preserves_title_without_prefix ... ok
test context::pack::tests::default_file_order_is_stable ... ok
test context::pack::tests::context_pack_work_packet_counts_are_correct ... ok
test context::pack::tests::extract_adr_title_from_frontmatter ... ok
test context::pack::tests::context_pack_all_required_sections_present ... ok
test context::pack::tests::extract_adr_title_from_heading ... ok
test context::pack::tests::extract_adr_title_returns_none_for_empty ... ok
test context::pack::tests::extract_first_paragraph_handles_no_frontmatter ... ok
test context::pack::tests::extract_first_paragraph_joins_multi_line ... ok
test context::pack::tests::extract_first_paragraph_returns_empty_for_empty_content ... ok
test context::pack::tests::extract_first_paragraph_skips_frontmatter ... ok
test context::pack::tests::render_context_pack_empty_architecture_summary ... ok
test context::pack::tests::render_context_pack_empty_decisions ... ok
test context::pack::tests::render_context_pack_empty_workflow_summary ... ok
test context::pack::tests::render_context_pack_includes_accepted_decisions ... ok
test context::pack::tests::render_context_pack_includes_architecture_summary ... ok
test context::pack::tests::render_context_pack_includes_current_status ... ok
test context::pack::tests::render_context_pack_includes_frontmatter ... ok
test context::pack::tests::render_context_pack_includes_important_documents ... ok
test context::pack::tests::render_context_pack_includes_next_recommended_action ... ok
test context::pack::tests::render_context_pack_includes_project_identity ... ok
test context::pack::tests::render_context_pack_includes_risks_and_blockers ... ok
test context::pack::tests::render_context_pack_includes_source_files ... ok
test context::pack::tests::render_context_pack_includes_trust_notes ... ok
test context::pack::tests::render_context_pack_includes_verification_summary ... ok
test context::pack::tests::render_context_pack_includes_workflow_summary ... ok
test context::pack::tests::render_context_pack_next_action_when_all_complete ... ok
test context::pack::tests::render_context_pack_is_deterministic ... ok
test context::verify::tests::all_headings_present_returns_empty ... ok
test context::pack::tests::render_context_pack_next_action_when_none_active ... ok
test context::verify::tests::expected_context_files_includes_required_and_optional ... ok
test context::pack::tests::render_context_pack_includes_active_work ... ok
test context::verify::tests::expected_context_files_returns_five_entries ... ok
test context::verify::tests::frontmatter_detected_when_present ... ok
test context::verify::tests::frontmatter_detected_with_leading_blank_lines ... ok
test context::verify::tests::frontmatter_not_detected_for_empty_content ... ok
test context::handoff::tests::generate_handoff_from_workspace ... ok
test context::verify::tests::frontmatter_not_detected_when_absent ... ok
test context::verify::tests::missing_heading_is_reported ... ok
test context::verify::tests::no_expected_headings_returns_empty ... ok
test context::verify::tests::render_summary_shows_failed_when_errors_exist ... ok
test context::verify::tests::render_summary_shows_passed_when_no_errors ... ok
test context::verify::tests::render_summary_shows_warning_indicators ... ok
test context::verify::tests::report_to_diagnostic_report_collects_all_diagnostics ... ok
test context::verify::tests::verify_context_from_cargo_manifest_dir ... ok
test context::verify::tests::verify_context_reports_all_missing_files ... ok
test dependency_detection::tests::dependency_signal_kind_labels_are_stable ... ok
test context::verify::tests::verify_context_passes_with_all_files_present ... ok
test context::verify::tests::verify_context_warns_on_missing_headings ... ok
test dependency_detection::tests::dependency_detection_is_empty_without_known_signals ... ok
test diagnostics::tests::diagnostic_renders_as_single_line_message ... ok
test context::pack::tests::generate_context_pack_from_workspace_produces_artifact ... ok
test diagnostics::tests::severity_labels_are_stable ... ok
test diagnostics::tests::report_knows_when_it_contains_errors ... ok
test context::verify::tests::verify_context_warns_on_missing_frontmatter ... ok
test error::tests::invalid_input_has_stable_code_and_message ... ok
test error::tests::monad_result_alias_can_return_success_or_error ... ok
test error::tests::not_found_names_missing_resource ... ok
test context::verify::tests::verify_context_passes_with_all_required_files ... ok
test error::tests::verification_failed_converts_to_error_diagnostic ... ok
test dependency_detection::tests::dependency_signal_kind_counts_are_stable ... ok
test evolution::context_baseline::tests::context_baseline_plan_contains_core_context_targets ... ok
test evolution::context_baseline::tests::context_baseline_dry_run_previews_creates_when_targets_are_missing ... ok
test evolution::context_baseline::tests::context_baseline_dry_run_detects_existing_file_conflicts ... ok
test evolution::context_baseline::tests::context_baseline_dry_run_output_states_no_files_or_ai_work ... ok
test evolution::verify_baseline::tests::verify_baseline_dry_run_previews_create_when_target_is_missing ... ok
test evolution::verify_baseline::tests::verify_baseline_dry_run_output_states_no_files_written ... ok
test evolution::verify_baseline::tests::verify_baseline_plan_uses_embedded_template_target ... ok
test exec::command::tests::command_runner_rejects_empty_program ... ok
test exec::command::tests::command_spec_builds_display_command ... ok
test exec::result::tests::command_result_can_represent_failure ... ok
test dependency_detection::tests::dependency_paths_can_be_grouped_by_toolchain ... ok
test exec::result::tests::command_result_exposes_execution_fields ... ok
test dependency_detection::tests::dependency_toolchain_counts_are_stable ... ok
test dependency_detection::tests::detects_dependency_signals_for_common_toolchains ... ok
test file_ops::dry_run::tests::dry_run_previews_create_when_file_is_missing ... ok
test evolution::verify_baseline::tests::verify_baseline_dry_run_detects_existing_file_conflict ... ok
test file_ops::model::tests::delete_operation_can_require_approval ... ok
test file_ops::model::tests::operation_kind_labels_are_stable ... ok
test file_ops::model::tests::create_operation_is_content_write ... ok
test file_ops::dry_run::tests::dry_run_summary_counts_outcomes ... ok
test file_ops::dry_run::tests::dry_run_detects_create_conflict_when_file_exists ... ok
test exec::command::tests::command_runner_captures_success_stdout_and_exit_code ... ok
test file_ops::model::tests::update_operation_is_content_write ... ok
test file_ops::dry_run::tests::dry_run_detects_update_conflict_when_file_is_missing ... ok
test file_ops::plan::tests::plan_preserves_operation_order ... ok
test exec::command::tests::command_runner_captures_failure_stderr_and_exit_code ... ok
test file_ops::plan::tests::plan_starts_empty ... ok
test file_ops::plan::tests::plan_summary_counts_operation_kinds ... ok
test file_ops::model::tests::skip_and_conflict_are_reviewable_states ... ok
test file_ops::model::tests::target_preserves_path ... ok
test file_ops::plan::tests::plan_can_be_built_incrementally ... ok
test file_ops::report::tests::dry_run_report_renders_empty_plan ... ok
test file_ops::report::tests::dry_run_report_renders_operations_and_conflicts ... ok
test git::status::tests::parses_branch_without_remote_tracking ... ok
test git::status::tests::parses_clean_branch_status ... ok
test git::status::tests::parses_detached_head_as_isolation_required ... ok
test git::status::tests::parses_dirty_status_counts ... ok
test manifest::tests::current_schema_version_is_supported ... ok
test file_ops::dry_run::tests::dry_run_previews_update_when_file_exists ... ok
test manifest::tests::default_manifest_matches_monad_runtime_shape ... ok
test manifest::tests::empty_project_name_fails_validation ... ok
test manifest::tests::future_schema_version_is_not_supported ... ok
test manifest::tests::invalid_toml_returns_invalid_input_error ... ok
test manifest::tests::missing_manifest_path_returns_not_found_error ... ok
test manifest::tests::valid_default_manifest_has_no_error_diagnostics ... ok
test manifest::tests::manifest_parses_from_toml_string ... ok
test output::tests::diagnostic_report_renders_as_text_lines ... ok
test manifest::tests::unsupported_schema_version_fails_validation ... ok
test manifest::tests::manifest_loads_from_path ... ok
test output::tests::output_format_parses_text_and_json ... ok
test manifest::tests::manifest_loads_from_workspace_context ... ok
test output::tests::repository_role_enum_is_still_available_for_future_output_work ... ok
test output::tests::repository_inspection_summary_type_defaults_policy_to_empty ... ok
test output::tests::unsupported_output_format_returns_error ... ok
test output::tests::workspace_summary_renders_like_info_command ... ok
test policy::approval::tests::approval_decision_records_actor_reason_and_result ... ok
test policy::approval::tests::approval_gate_kind_labels_are_stable ... ok
test policy::approval::tests::approval_gate_preserves_action_and_requirement ... ok
test policy::approval::tests::approval_kind_identifies_required_approval ... ok
test policy::audit::tests::action_proposed_event_records_gate_metadata ... ok
test policy::audit::tests::approval_decision_event_uses_decision_kind ... ok
test policy::audit::tests::audit_event_kind_labels_are_stable ... ok
test checks::adapters::rust::tests::rust_manifest_check_fails_when_root_manifest_is_missing ... ok
test policy::audit::tests::audit_log_detects_approval_for_gate ... ok
test policy::audit::tests::audit_log_records_events_and_filters_by_subject ... ok
test policy::audit::tests::local_write_gate_helper_creates_initial_audit_trail ... ok
test output::tests::repository_inspection_summary_includes_policy_metrics ... ok
test checks::adapters::tests::selected_adapter_checks_include_rust_when_rust_detected ... ok
test output::tests::repository_inspection_summary_renders_policy_as_text ... ok
test repo_contract::tests::contract_passes_for_valid_workspace_shape ... ok
test repo_contract::tests::initial_contract_contains_expected_paths ... ok
test repository_context_pack::tests::context_pack_default_export_dir_is_repository_local_and_deterministic ... ok
test repository_context_pack::tests::context_pack_render_format_parses_supported_formats ... ok
test output::tests::repository_inspection_summary_renders_policy_as_json ... ok
test repository_context_pack::tests::context_pack_render_format_rejects_unsupported_formats ... ok
test repository_context_pack::tests::section_kind_labels_are_stable ... ok
test repo_contract::tests::contract_reports_missing_directory ... ok
test repository_context_pack::tests::exported_file_records_capture_format_path_and_bytes ... ok
test repository_graph::tests::graph_render_format_parses_supported_formats ... ok
test repository_context_pack::tests::context_pack_contains_expected_sections ... ok
test repository_context_pack::tests::context_pack_exposes_facts_by_section_and_key ... ok
test repository_context_pack::tests::context_pack_renders_as_markdown ... ok
test repository_context_pack::tests::context_pack_reports_policy_warnings ... ok
test repository_graph::tests::graph_contains_root_and_traversed_entries ... ok
test repository_graph::tests::graph_exposes_category_and_decision_counts ... ok
test checks::run::tests::workspace_checks_report_missing_files ... ok
test repository_graph::tests::graph_output_is_deterministically_ordered ... ok
test repository_graph::tests::graph_render_format_rejects_unsupported_formats ... ok
test repository_graph::tests::graph_edges_connect_parent_child_relationships ... ok
test repository_context_pack::tests::context_pack_renders_as_json ... ok
test repository_inspection::tests::repository_entry_roles_map_to_stable_categories ... ok
test repository_graph::tests::graph_renders_as_dot ... ok
test repository_inspection::tests::traversal_guardrails_are_conservative_by_default ... ok
test repository_graph::tests::graph_renders_as_mermaid ... ok
test repository_graph::tests::graph_renders_as_text ... ok
test repository_policy::tests::policy_report_is_empty_when_no_diagnostics_are_supplied ... ok
test repository_context_pack::tests::context_pack_exports_markdown_and_json_files ... ok
test repository_graph::tests::graph_renders_as_json ... ok
test repository_policy::tests::policy_report_counts_by_severity ... ok
test repository_policy::tests::policy_reports_traversal_safety_information ... ok
test repository_graph::tests::graph_rendering_is_deterministic ... ok
test repository_policy::tests::policy_severity_labels_are_stable ... ok
test templates::model::tests::embedded_template_definition_contains_metadata_and_content ... ok
test templates::model::tests::metadata_exposes_template_fields ... ok
test templates::model::tests::source_kind_label_is_stable ... ok
test repository_policy::tests::policy_reports_missing_readme_and_license ... ok
test templates::model::tests::template_id_preserves_value ... ok
test repository_policy::tests::policy_warns_when_manifest_has_no_lockfile ... ok
test repository_inspection::tests::bounded_traversal_output_is_deterministic ... ok
test templates::registry::tests::initial_registry_contains_baseline_templates ... ok
test templates::registry::tests::registry_lists_templates_in_deterministic_order ... ok
test templates::registry::tests::registry_registers_and_retrieves_template ... ok
test templates::registry::tests::registry_rejects_duplicate_template_ids ... ok
test repository_inspection::tests::bounded_traversal_skips_generated_or_external_directories ... ok
test repository_inspection::tests::bounded_traversal_respects_simple_root_gitignore_patterns ... ok
test repository_inspection::tests::traversal_plan_is_built_from_shallow_inspection ... ok
test repository_inspection::tests::bounded_traversal_respects_max_depth ... ok
test templates::registry::tests::registry_starts_empty ... ok
test repository_inspection::tests::bounded_traversal_walks_safe_directories ... ok
test tests::checked_runtime_identity_uses_monad_result ... ok
test tests::output_format_is_exported_from_core_root ... ok
test tests::repository_contract_is_exported_from_core_root ... ok
test tests::repository_dependency_detection_types_are_exported_from_core_root ... ok
test tests::repository_entry_category_is_exported_from_core_root ... ok
test tests::repository_graph_render_format_is_exported_from_core_root ... ok
test tests::repository_graph_types_are_exported_from_core_root ... ok
test repository_inspection::tests::repository_inspection_lists_top_level_entries ... ok
test tests::repository_inspection_summary_type_is_exported_from_core_root ... ok
test tests::repository_policy_types_are_exported_from_core_root ... ok
test tests::repository_inspection_types_are_exported_from_core_root ... ok
test tests::repository_toolchain_detection_types_are_exported_from_core_root ... ok
test tests::runtime_banner_is_human_readable ... ok
test tests::runtime_identity_can_build_default_manifest ... ok
test tests::runtime_identity_can_produce_startup_diagnostic ... ok
test tests::runtime_identity_names_monad ... ok
test tests::traversal_planning_types_are_exported_from_core_root ... ok
test tests::workspace_checks_are_exported_from_core_root ... ok
test tests::workspace_context_is_exported_from_core_root ... ok
test toolchain_detection::tests::signal_kind_labels_are_stable ... ok
test toolchain_detection::tests::toolchain_kind_labels_are_stable ... ok
test toolchain_detection::tests::detection_is_empty_when_no_known_signals_exist ... ok
test workspace::tests::workspace_context_builds_standard_paths ... ok
test workspace::tests::workspace_discovery_finds_root_from_nested_directory ... ok
test workspace::tests::workspace_discovery_reports_not_found ... ok
test toolchain_detection::tests::signal_kind_counts_are_stable_and_machine_readable ... ok
test toolchain_detection::tests::signal_paths_can_be_grouped_by_toolchain ... ok
test toolchain_detection::tests::detects_common_repository_toolchains ... ok
test toolchain_detection::tests::toolchain_counts_are_stable_and_machine_readable ... ok

test result: ok. 315 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running unittests src/lib.rs (target/debug/deps/monad_mcp-8004afe4f5cd0192)

running 4 tests
test tests::capability_kind_labels_are_stable ... ok
test tests::initial_allowed_capabilities_exclude_approval_gated_and_forbidden_items ... ok
test tests::initial_capabilities_include_allowed_and_guarded_descriptors ... ok
test tests::safety_class_labels_are_stable ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests monad_core

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests monad_mcp

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

</details>


### 4. cargo-clippy

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/cargo-clippy.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s
```

</details>


### 5. monad-help

```bash
cargo run -p monad-cli -- --help
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-help.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/monad --help`
Monad

Usage:
  monad [command] [--format=<format>] [--write] [--dry-run]

Core commands:
  info                                      Show workspace summary
  check                                     Run workspace checks
  inspect                                   Inspect repository structure
  graph                                     Render repository graph
  plan "<intent>"                          Produce a supervised no-write plan
  version                                   Show runtime version
  help                                      Show this help

Context commands:
  context                                   Render AI-readable repository context pack
  context --write                           Export repository context pack files
  context generate current-state            Generate current-state artifact
  context generate handoff                  Generate latest handoff artifact
  context generate bootstrap                Generate bootstrap prompt for AI sessions
  context pack                              Assemble project-level context pack
  context verify                            Verify context files exist and are well-formed

Evolution dry-run commands:
  evolve verify-baseline --dry-run          Preview verification baseline files
  evolve context-baseline --dry-run         Preview context baseline files

General formats:
  text
  json

Graph formats:
  text
  json
  mermaid
  dot

Context formats:
  markdown
  md
  text
  json

Examples:
  monad inspect
  monad context --write
  monad check --format=json
  monad graph --format=mermaid
  monad plan "explain this repository"
  monad evolve verify-baseline --dry-run
  monad evolve context-baseline --dry-run

Safety notes:
  plan is no-write and does not run commands.
  evolve commands are dry-run only in this MVP hardening phase.
  --write is only supported for the context command.
```

</details>


### 6. monad-version

```bash
cargo run -p monad-cli -- version
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-version.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/monad version`
Monad runtime foundation ready (crate: monad-core, model: local-first)

Package:
  cli_crate: monad-cli
  version: 0.1.0

Release posture:
  channel: internal-mvp-candidate
  candidate_identifier: 0.1.0-internal-mvp-candidate
  public_release: no
  package_published: no
  installer_available: no

Safety posture:
  execution_model: local-first
  planning: no-write
  evolution: dry-run-only
  autonomous_agent_execution: no
```

</details>


### 7. monad-info

```bash
cargo run -p monad-cli -- info
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-info.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/monad info`
Monad workspace
  root: .
  project: Monad (monad)
  schema_version: 1
  core_crate: monad-core
  cli_crate: monad-cli
  execution_model: local-first
```

</details>


### 8. monad-inspect

```bash
cargo run -p monad-cli -- inspect
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-inspect.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/monad inspect`
Monad repository inspection
  root: .
  entries: 32
  files: 22
  directories: 10
  symlinks: 0
  other: 0
  policy:
    diagnostics: 2
    info: 2
    advisory: 0
    warnings: 0
    has_warnings: false
  policy_diagnostics:
    - MONAD-RI-0200 [info] 3 generated or external path(s) were identified for conservative traversal handling
    - MONAD-RI-0201 [info] bounded traversal stayed within configured max depth
  metrics:
    known_entries: 20
    unknown_entries: 12
    generated_or_external_entries: 2
    safe_for_future_traversal: 7
    inspect_shallow_only: 23
    skip_generated_or_external: 2
  future_traversal_guardrails:
    mode: future_recursive_limited
    max_depth: 3
    follow_symlinks: false
    include_generated_or_external: false
    respect_ignore_files: true
    deterministic_ordering: true
  bounded_traversal:
    mode: bounded_recursive
    entries: 749
    max_observed_depth: 3
  graph:
    nodes: 750
    edges: 749
    max_depth: 3
  toolchains:
    detected: 3
    signals: 40
  dependencies:
    toolchains: 2
    signals: 7
    manifests: 5
    lockfiles: 2
    package_manager_configs: 0
    build_files: 0
  dependency_toolchain_counts:
    javascript: 2
    rust: 5
  dependency_signal_kind_counts:
    lockfile: 2
    manifest: 5
  dependency_signals:
    - rust signals=5 manifests=4 lockfiles=1 configs=0 build_files=0
      - Cargo.lock
      - Cargo.toml
      - crates/monad-cli/Cargo.toml
      - crates/monad-core/Cargo.toml
      - crates/monad-mcp/Cargo.toml
    - javascript signals=2 manifests=1 lockfiles=1 configs=0 build_files=0
      - bun.lock
      - package.json
  toolchain_counts:
    javascript: 3
    python: 8
    rust: 29
  toolchain_signal_kind_counts:
    lockfile: 2
    manifest: 5
    source_file: 33
  detected_toolchains:
    - rust signals=29
      - Cargo.lock
      - Cargo.toml
      - crates/monad-cli/Cargo.toml
      - crates/monad-cli/src/main.rs
      - crates/monad-cli/tests/cli_smoke.rs
      - crates/monad-core/Cargo.toml
      - crates/monad-core/src/agents.rs
      - crates/monad-core/src/checks.rs
      - crates/monad-core/src/dependency_detection.rs
      - crates/monad-core/src/diagnostics.rs
      - crates/monad-core/src/error.rs
      - crates/monad-core/src/evolution.rs
      - crates/monad-core/src/exec.rs
      - crates/monad-core/src/file_ops.rs
      - crates/monad-core/src/git.rs
      - crates/monad-core/src/lib.rs
      - crates/monad-core/src/manifest.rs
      - crates/monad-core/src/output.rs
      - crates/monad-core/src/policy.rs
      - crates/monad-core/src/repo_contract.rs
      - crates/monad-core/src/repository_context_pack.rs
      - crates/monad-core/src/repository_graph.rs
      - crates/monad-core/src/repository_inspection.rs
      - crates/monad-core/src/repository_policy.rs
      - crates/monad-core/src/templates.rs
      - crates/monad-core/src/toolchain_detection.rs
      - crates/monad-core/src/workspace.rs
      - crates/monad-mcp/Cargo.toml
      - crates/monad-mcp/src/lib.rs
    - javascript signals=3
      - bun.lock
      - docs/wiki/deepwiki-dump-monad-workspace/dump-deepwiki.mjs
      - package.json
    - python signals=8
      - tools/scripts/check-adr-records.py
      - tools/scripts/check-context-records.py
      - tools/scripts/check-deliverable-records.py
      - tools/scripts/check-epic-records.py
      - tools/scripts/check-markdown-frontmatter.py
      - tools/scripts/check-required-paths.py
      - tools/scripts/check-task-records.py
      - tools/scripts/check-work-records.py
  graph_categories:
    assets: 1
    configuration: 1
    continuous_integration: 1
    documentation: 48
    generated_or_external: 3
    governance: 1
    hidden: 3
    javascript_package_management: 2
    legal: 1
    monad_control: 2
    other: 670
    rust_runtime: 7
    source: 4
    tests: 1
    tooling: 2
    version_control: 1
    work_management: 1
  graph_traversal_decisions:
    candidate_for_future_traversal: 80
    inspect_shallow_only: 662
    skip_by_default: 7
  categories:
    assets: 1
    configuration: 1
    continuous_integration: 1
    documentation: 2
    generated_or_external: 2
    hidden: 1
    javascript_package_management: 2
    legal: 1
    monad_control: 2
    other: 11
    rust_runtime: 4
    source: 1
    tooling: 1
    version_control: 1
    work_management: 1
  roles:
    asset_root: 1
    ci_root: 1
    documentation_root: 1
    editorconfig: 1
    generated_or_external: 2
    gitignore: 1
    hidden: 1
    javascript_package_config: 2
    license: 1
    monad_manifest: 1
    monad_state_root: 1
    other: 11
    readme: 1
    rust_lockfile: 1
    rust_quality_config: 1
    rust_toolchain: 1
    rust_workspace_manifest: 1
    source_root: 1
    tooling_root: 1
    work_root: 1
  traversal_policies:
    inspect_shallow_only: 23
    safe_for_future_traversal: 7
    skip_generated_or_external: 2
  top_level_entries:
    - .artifacts [directory category=hidden role=hidden traversal=inspect_shallow_only decision=inspect_shallow_only reason=hidden path is inspected shallowly unless explicitly supported]
    - .editorconfig [file category=configuration role=editorconfig traversal=inspect_shallow_only decision=inspect_shallow_only reason=hidden path is inspected shallowly unless explicitly supported]
    - .git [directory category=generated_or_external role=generated_or_external traversal=skip_generated_or_external decision=skip_by_default reason=generated or external path is skipped by default]
    - .github [directory category=continuous_integration role=ci_root traversal=safe_for_future_traversal decision=candidate_for_future_traversal reason=directory is a safe candidate for future bounded traversal]
    - .gitignore [file category=version_control role=gitignore traversal=inspect_shallow_only decision=inspect_shallow_only reason=hidden path is inspected shallowly unless explicitly supported]
    - .monad [directory category=monad_control role=monad_state_root traversal=safe_for_future_traversal decision=candidate_for_future_traversal reason=directory is a safe candidate for future bounded traversal]
    - CHANGELOG.md [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - CONTRIBUTING.md [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - Cargo.lock [file category=rust_runtime role=rust_lockfile traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - Cargo.toml [file category=rust_runtime role=rust_workspace_manifest traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - LICENSE [file category=legal role=license traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - README.md [file category=documentation role=readme traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - SECURITY.md [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - assets [directory category=assets role=asset_root traversal=safe_for_future_traversal decision=candidate_for_future_traversal reason=directory is a safe candidate for future bounded traversal]
    - bun.lock [file category=javascript_package_management role=javascript_package_config traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - clippy.toml [file category=rust_runtime role=rust_quality_config traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - complete_wp_e0_006_adr_foundation.sh [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - complete_wp_e10_001_public_claims_audit.sh [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - complete_wp_e10_002_public_prerelease_evidence.sh [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - complete_wp_e10_003_distribution_posture.sh [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - complete_wp_e10_004_public_prerelease_notes.sh [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - complete_wp_e10_005_public_prerelease_verification_audit.sh [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - crates [directory category=source role=source_root traversal=safe_for_future_traversal decision=candidate_for_future_traversal reason=directory is a safe candidate for future bounded traversal]
    - docs [directory category=documentation role=documentation_root traversal=safe_for_future_traversal decision=candidate_for_future_traversal reason=directory is a safe candidate for future bounded traversal]
    - monad.toml [file category=monad_control role=monad_manifest traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - nano [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - package.json [file category=javascript_package_management role=javascript_package_config traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - rust-toolchain.toml [file category=rust_runtime role=rust_toolchain traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - sue close 72  [file category=other role=other traversal=inspect_shallow_only decision=inspect_shallow_only reason=non-directory or shallow-only path is recorded without recursive traversal]
    - target [directory category=generated_or_external role=generated_or_external traversal=skip_generated_or_external decision=skip_by_default reason=generated or external path is skipped by default]
    - tools [directory category=tooling role=tooling_root traversal=safe_for_future_traversal decision=candidate_for_future_traversal reason=directory is a safe candidate for future bounded traversal]
    - work [directory category=work_management role=work_root traversal=safe_for_future_traversal decision=candidate_for_future_traversal reason=directory is a safe candidate for future bounded traversal]
```

</details>


### 9. monad-check

```bash
cargo run -p monad-cli -- check
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-check.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/monad check`
Monad check report

Checks run: 7
Passed: 7
Failed: 0
Warnings: 0
Skipped: 0

[PASSED] MONAD-CHECK-0001: required file exists: monad.toml
[PASSED] MONAD-CHECK-0002: required file exists: Cargo.toml
[PASSED] MONAD-CHECK-0003: cargo 1.95.0 (f2d3ce0bd 2026-03-21) (Homebrew)
[PASSED] MONAD-CHECK-RUST-0001: cargo 1.95.0 (f2d3ce0bd 2026-03-21) (Homebrew)
[PASSED] MONAD-CHECK-RUST-0002: Cargo manifest exists: Cargo.toml
[PASSED] MONAD-CHECK-JS-0001: JavaScript manifest exists: package.json
[PASSED] MONAD-CHECK-JS-0002: package-manager lockfile exists: bun.lock

Result: passed

Evidence report written: ./.monad/reports/latest-check-evidence.md
```

</details>


### 10. monad-graph

```bash
cargo run -p monad-cli -- graph
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-graph.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/monad graph`
Monad repository graph
  root: .
  nodes: 750
  edges: 749
  max_depth: 3
  category_counts:
    assets: 1
    configuration: 1
    continuous_integration: 1
    documentation: 48
    generated_or_external: 3
    governance: 1
    hidden: 3
    javascript_package_management: 2
    legal: 1
    monad_control: 2
    other: 670
    rust_runtime: 7
    source: 4
    tests: 1
    tooling: 2
    version_control: 1
    work_management: 1
  traversal_decision_counts:
    candidate_for_future_traversal: 80
    inspect_shallow_only: 662
    skip_by_default: 7
  nodes:
    - id=path:.artifacts label=.artifacts kind=repository_entry depth=0
    - id=path:.editorconfig label=.editorconfig kind=repository_entry depth=0
    - id=path:.git label=.git kind=repository_entry depth=0
    - id=path:.github label=.github kind=repository_entry depth=0
    - id=path:.github/FUNDING.yml label=.github/FUNDING.yml kind=repository_entry depth=1
    - id=path:.github/ISSUE_TEMPLATE label=.github/ISSUE_TEMPLATE kind=repository_entry depth=1
    - id=path:.github/ISSUE_TEMPLATE/adr-candidate.yml label=.github/ISSUE_TEMPLATE/adr-candidate.yml kind=repository_entry depth=2
    - id=path:.github/ISSUE_TEMPLATE/bug.yml label=.github/ISSUE_TEMPLATE/bug.yml kind=repository_entry depth=2
    - id=path:.github/ISSUE_TEMPLATE/bug_report.md label=.github/ISSUE_TEMPLATE/bug_report.md kind=repository_entry depth=2
    - id=path:.github/ISSUE_TEMPLATE/config.yml label=.github/ISSUE_TEMPLATE/config.yml kind=repository_entry depth=2
    - id=path:.github/ISSUE_TEMPLATE/epic.yml label=.github/ISSUE_TEMPLATE/epic.yml kind=repository_entry depth=2
    - id=path:.github/ISSUE_TEMPLATE/feature_request.md label=.github/ISSUE_TEMPLATE/feature_request.md kind=repository_entry depth=2
    - id=path:.github/ISSUE_TEMPLATE/research.yml label=.github/ISSUE_TEMPLATE/research.yml kind=repository_entry depth=2
    - id=path:.github/ISSUE_TEMPLATE/task.yml label=.github/ISSUE_TEMPLATE/task.yml kind=repository_entry depth=2
    - id=path:.github/ISSUE_TEMPLATE/work-packet.yml label=.github/ISSUE_TEMPLATE/work-packet.yml kind=repository_entry depth=2
    - id=path:.github/dependabot.yml label=.github/dependabot.yml kind=repository_entry depth=1
    - id=path:.github/pull_request_template.md label=.github/pull_request_template.md kind=repository_entry depth=1
    - id=path:.github/workflows label=.github/workflows kind=repository_entry depth=1
    - id=path:.github/workflows/build-binary.yml label=.github/workflows/build-binary.yml kind=repository_entry depth=2
    - id=path:.gitignore label=.gitignore kind=repository_entry depth=0
    - id=path:.monad label=.monad kind=repository_entry depth=0
    - id=path:.monad/README.md label=.monad/README.md kind=repository_entry depth=1
    - id=path:.monad/cache label=.monad/cache kind=repository_entry depth=1
    - id=path:.monad/context label=.monad/context kind=repository_entry depth=1
    - id=path:.monad/context/2026-05-23-session-001.md label=.monad/context/2026-05-23-session-001.md kind=repository_entry depth=2
    - id=path:.monad/context/README.md label=.monad/context/README.md kind=repository_entry depth=2
    - id=path:.monad/context/current-state.md label=.monad/context/current-state.md kind=repository_entry depth=2
    - id=path:.monad/context/decision-log.md label=.monad/context/decision-log.md kind=repository_entry depth=2
    - id=path:.monad/context/decision-records label=.monad/context/decision-records kind=repository_entry depth=2
    - id=path:.monad/context/decision-records/README.md label=.monad/context/decision-records/README.md kind=repository_entry depth=3
    - id=path:.monad/context/latest-context-pack.md label=.monad/context/latest-context-pack.md kind=repository_entry depth=2
    - id=path:.monad/context/latest-handoff.md label=.monad/context/latest-handoff.md kind=repository_entry depth=2
    - id=path:.monad/context/session-chronicles label=.monad/context/session-chronicles kind=repository_entry depth=2
    - id=path:.monad/context/session-chronicles/README.md label=.monad/context/session-chronicles/README.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs label=.monad/context/work-packet-handoffs kind=repository_entry depth=2
    - id=path:.monad/context/work-packet-handoffs/README.md label=.monad/context/work-packet-handoffs/README.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-001.md label=.monad/context/work-packet-handoffs/WP-E1-001.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-002.md label=.monad/context/work-packet-handoffs/WP-E1-002.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-003.md label=.monad/context/work-packet-handoffs/WP-E1-003.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-004.md label=.monad/context/work-packet-handoffs/WP-E1-004.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-005.md label=.monad/context/work-packet-handoffs/WP-E1-005.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-006.md label=.monad/context/work-packet-handoffs/WP-E1-006.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-007.md label=.monad/context/work-packet-handoffs/WP-E1-007.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-008.md label=.monad/context/work-packet-handoffs/WP-E1-008.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-009.md label=.monad/context/work-packet-handoffs/WP-E1-009.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-010.md label=.monad/context/work-packet-handoffs/WP-E1-010.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-011.md label=.monad/context/work-packet-handoffs/WP-E1-011.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E1-013.md label=.monad/context/work-packet-handoffs/WP-E1-013.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-001.md label=.monad/context/work-packet-handoffs/WP-E2-001.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-002.md label=.monad/context/work-packet-handoffs/WP-E2-002.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-003.md label=.monad/context/work-packet-handoffs/WP-E2-003.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-004.md label=.monad/context/work-packet-handoffs/WP-E2-004.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-005.md label=.monad/context/work-packet-handoffs/WP-E2-005.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-006.md label=.monad/context/work-packet-handoffs/WP-E2-006.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-007.md label=.monad/context/work-packet-handoffs/WP-E2-007.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-008.md label=.monad/context/work-packet-handoffs/WP-E2-008.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-009.md label=.monad/context/work-packet-handoffs/WP-E2-009.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-010.md label=.monad/context/work-packet-handoffs/WP-E2-010.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-011.md label=.monad/context/work-packet-handoffs/WP-E2-011.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-012.md label=.monad/context/work-packet-handoffs/WP-E2-012.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-013.md label=.monad/context/work-packet-handoffs/WP-E2-013.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-014.md label=.monad/context/work-packet-handoffs/WP-E2-014.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-015.md label=.monad/context/work-packet-handoffs/WP-E2-015.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-016.md label=.monad/context/work-packet-handoffs/WP-E2-016.md kind=repository_entry depth=3
    - id=path:.monad/context/work-packet-handoffs/WP-E2-017.md label=.monad/context/work-packet-handoffs/WP-E2-017.md kind=repository_entry depth=3
    - id=path:.monad/local label=.monad/local kind=repository_entry depth=1
    - id=path:.monad/reports label=.monad/reports kind=repository_entry depth=1
    - id=path:.monad/tmp label=.monad/tmp kind=repository_entry depth=1
    - id=path:CHANGELOG.md label=CHANGELOG.md kind=repository_entry depth=0
    - id=path:CONTRIBUTING.md label=CONTRIBUTING.md kind=repository_entry depth=0
    - id=path:Cargo.lock label=Cargo.lock kind=repository_entry depth=0
    - id=path:Cargo.toml label=Cargo.toml kind=repository_entry depth=0
    - id=path:LICENSE label=LICENSE kind=repository_entry depth=0
    - id=path:README.md label=README.md kind=repository_entry depth=0
    - id=path:SECURITY.md label=SECURITY.md kind=repository_entry depth=0
    - id=path:assets label=assets kind=repository_entry depth=0
    - id=path:assets/.keep label=assets/.keep kind=repository_entry depth=1
    - id=path:assets/softwarefordevelopers-preview.png label=assets/softwarefordevelopers-preview.png kind=repository_entry depth=1
    - id=path:bun.lock label=bun.lock kind=repository_entry depth=0
    - id=path:clippy.toml label=clippy.toml kind=repository_entry depth=0
    - id=path:complete_wp_e0_006_adr_foundation.sh label=complete_wp_e0_006_adr_foundation.sh kind=repository_entry depth=0
    - id=path:complete_wp_e10_001_public_claims_audit.sh label=complete_wp_e10_001_public_claims_audit.sh kind=repository_entry depth=0
    - id=path:complete_wp_e10_002_public_prerelease_evidence.sh label=complete_wp_e10_002_public_prerelease_evidence.sh kind=repository_entry depth=0
    - id=path:complete_wp_e10_003_distribution_posture.sh label=complete_wp_e10_003_distribution_posture.sh kind=repository_entry depth=0
    - id=path:complete_wp_e10_004_public_prerelease_notes.sh label=complete_wp_e10_004_public_prerelease_notes.sh kind=repository_entry depth=0
    - id=path:complete_wp_e10_005_public_prerelease_verification_audit.sh label=complete_wp_e10_005_public_prerelease_verification_audit.sh kind=repository_entry depth=0
    - id=path:crates label=crates kind=repository_entry depth=0
    - id=path:crates/monad-cli label=crates/monad-cli kind=repository_entry depth=1
    - id=path:crates/monad-cli/Cargo.toml label=crates/monad-cli/Cargo.toml kind=repository_entry depth=2
    - id=path:crates/monad-cli/src label=crates/monad-cli/src kind=repository_entry depth=2
    - id=path:crates/monad-cli/src/main.rs label=crates/monad-cli/src/main.rs kind=repository_entry depth=3
    - id=path:crates/monad-cli/tests label=crates/monad-cli/tests kind=repository_entry depth=2
    - id=path:crates/monad-cli/tests/cli_smoke.rs label=crates/monad-cli/tests/cli_smoke.rs kind=repository_entry depth=3
    - id=path:crates/monad-core label=crates/monad-core kind=repository_entry depth=1
    - id=path:crates/monad-core/Cargo.toml label=crates/monad-core/Cargo.toml kind=repository_entry depth=2
    - id=path:crates/monad-core/src label=crates/monad-core/src kind=repository_entry depth=2
    - id=path:crates/monad-core/src/agents label=crates/monad-core/src/agents kind=repository_entry depth=3
    - id=path:crates/monad-core/src/agents.rs label=crates/monad-core/src/agents.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/checks label=crates/monad-core/src/checks kind=repository_entry depth=3
    - id=path:crates/monad-core/src/checks.rs label=crates/monad-core/src/checks.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/context label=crates/monad-core/src/context kind=repository_entry depth=3
    - id=path:crates/monad-core/src/dependency_detection.rs label=crates/monad-core/src/dependency_detection.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/diagnostics.rs label=crates/monad-core/src/diagnostics.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/error.rs label=crates/monad-core/src/error.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/evolution label=crates/monad-core/src/evolution kind=repository_entry depth=3
    - id=path:crates/monad-core/src/evolution.rs label=crates/monad-core/src/evolution.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/exec label=crates/monad-core/src/exec kind=repository_entry depth=3
    - id=path:crates/monad-core/src/exec.rs label=crates/monad-core/src/exec.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/file_ops label=crates/monad-core/src/file_ops kind=repository_entry depth=3
    - id=path:crates/monad-core/src/file_ops.rs label=crates/monad-core/src/file_ops.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/git label=crates/monad-core/src/git kind=repository_entry depth=3
    - id=path:crates/monad-core/src/git.rs label=crates/monad-core/src/git.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/lib.rs label=crates/monad-core/src/lib.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/manifest.rs label=crates/monad-core/src/manifest.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/output.rs label=crates/monad-core/src/output.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/policy label=crates/monad-core/src/policy kind=repository_entry depth=3
    - id=path:crates/monad-core/src/policy.rs label=crates/monad-core/src/policy.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/repo_contract.rs label=crates/monad-core/src/repo_contract.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/repository_context_pack.rs label=crates/monad-core/src/repository_context_pack.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/repository_graph.rs label=crates/monad-core/src/repository_graph.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/repository_inspection.rs label=crates/monad-core/src/repository_inspection.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/repository_policy.rs label=crates/monad-core/src/repository_policy.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/templates label=crates/monad-core/src/templates kind=repository_entry depth=3
    - id=path:crates/monad-core/src/templates.rs label=crates/monad-core/src/templates.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/toolchain_detection.rs label=crates/monad-core/src/toolchain_detection.rs kind=repository_entry depth=3
    - id=path:crates/monad-core/src/workspace.rs label=crates/monad-core/src/workspace.rs kind=repository_entry depth=3
    - id=path:crates/monad-mcp label=crates/monad-mcp kind=repository_entry depth=1
    - id=path:crates/monad-mcp/Cargo.toml label=crates/monad-mcp/Cargo.toml kind=repository_entry depth=2
    - id=path:crates/monad-mcp/src label=crates/monad-mcp/src kind=repository_entry depth=2
    - id=path:crates/monad-mcp/src/lib.rs label=crates/monad-mcp/src/lib.rs kind=repository_entry depth=3
    - id=path:docs label=docs kind=repository_entry depth=0
    - id=path:docs/00-meta label=docs/00-meta kind=repository_entry depth=1
    - id=path:docs/00-meta/DOCUMENTATION-MAP.md label=docs/00-meta/DOCUMENTATION-MAP.md kind=repository_entry depth=2
    - id=path:docs/00-meta/DOCUMENTATION-STANDARD.md label=docs/00-meta/DOCUMENTATION-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/00-meta/FRONTMATTER-STANDARD.md label=docs/00-meta/FRONTMATTER-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/00-meta/GLOSSARY.md label=docs/00-meta/GLOSSARY.md kind=repository_entry depth=2
    - id=path:docs/00-meta/IDEA.md label=docs/00-meta/IDEA.md kind=repository_entry depth=2
    - id=path:docs/00-meta/NAMING-STANDARD.md label=docs/00-meta/NAMING-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/00-meta/README.md label=docs/00-meta/README.md kind=repository_entry depth=2
    - id=path:docs/00-meta/STATUS-STANDARD.md label=docs/00-meta/STATUS-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/01-project label=docs/01-project kind=repository_entry depth=1
    - id=path:docs/01-project/00-vision label=docs/01-project/00-vision kind=repository_entry depth=2
    - id=path:docs/01-project/00-vision/HOLY-GRAIL-VISION.md label=docs/01-project/00-vision/HOLY-GRAIL-VISION.md kind=repository_entry depth=3
    - id=path:docs/01-project/00-vision/PRODUCT-THESIS.md label=docs/01-project/00-vision/PRODUCT-THESIS.md kind=repository_entry depth=3
    - id=path:docs/01-project/00-vision/PRODUCT-VISION.md label=docs/01-project/00-vision/PRODUCT-VISION.md kind=repository_entry depth=3
    - id=path:docs/01-project/00-vision/README.md label=docs/01-project/00-vision/README.md kind=repository_entry depth=3
    - id=path:docs/01-project/01-charter label=docs/01-project/01-charter kind=repository_entry depth=2
    - id=path:docs/01-project/01-charter/PRODUCT-CHARTER.md label=docs/01-project/01-charter/PRODUCT-CHARTER.md kind=repository_entry depth=3
    - id=path:docs/01-project/01-charter/PROJECT-CHARTER.md label=docs/01-project/01-charter/PROJECT-CHARTER.md kind=repository_entry depth=3
    - id=path:docs/01-project/01-charter/README.md label=docs/01-project/01-charter/README.md kind=repository_entry depth=3
    - id=path:docs/01-project/02-strategy label=docs/01-project/02-strategy kind=repository_entry depth=2
    - id=path:docs/01-project/02-strategy/GO-TO-MARKET-STRATEGY.md label=docs/01-project/02-strategy/GO-TO-MARKET-STRATEGY.md kind=repository_entry depth=3
    - id=path:docs/01-project/02-strategy/MONETIZATION-STRATEGY.md label=docs/01-project/02-strategy/MONETIZATION-STRATEGY.md kind=repository_entry depth=3
    - id=path:docs/01-project/02-strategy/OPEN-CORE-STRATEGY.md label=docs/01-project/02-strategy/OPEN-CORE-STRATEGY.md kind=repository_entry depth=3
    - id=path:docs/01-project/02-strategy/PRODUCT-STRATEGY.md label=docs/01-project/02-strategy/PRODUCT-STRATEGY.md kind=repository_entry depth=3
    - id=path:docs/01-project/02-strategy/README.md label=docs/01-project/02-strategy/README.md kind=repository_entry depth=3
    - id=path:docs/01-project/03-roadmap label=docs/01-project/03-roadmap kind=repository_entry depth=2
    - id=path:docs/01-project/03-roadmap/MVP-ROADMAP.md label=docs/01-project/03-roadmap/MVP-ROADMAP.md kind=repository_entry depth=3
    - id=path:docs/01-project/03-roadmap/POST-MVP-ROADMAP.md label=docs/01-project/03-roadmap/POST-MVP-ROADMAP.md kind=repository_entry depth=3
    - id=path:docs/01-project/03-roadmap/README.md label=docs/01-project/03-roadmap/README.md kind=repository_entry depth=3
    - id=path:docs/01-project/03-roadmap/RELEASE-PLAN.md label=docs/01-project/03-roadmap/RELEASE-PLAN.md kind=repository_entry depth=3
    - id=path:docs/01-project/03-roadmap/ROADMAP.md label=docs/01-project/03-roadmap/ROADMAP.md kind=repository_entry depth=3
    - id=path:docs/01-project/04-glossary label=docs/01-project/04-glossary kind=repository_entry depth=2
    - id=path:docs/01-project/04-glossary/PRODUCT-GLOSSARY.md label=docs/01-project/04-glossary/PRODUCT-GLOSSARY.md kind=repository_entry depth=3
    - id=path:docs/01-project/04-glossary/README.md label=docs/01-project/04-glossary/README.md kind=repository_entry depth=3
    - id=path:docs/01-project/04-glossary/UBIQUITOUS-LANGUAGE.md label=docs/01-project/04-glossary/UBIQUITOUS-LANGUAGE.md kind=repository_entry depth=3
    - id=path:docs/01-project/README.md label=docs/01-project/README.md kind=repository_entry depth=2
    - id=path:docs/02-product label=docs/02-product kind=repository_entry depth=1
    - id=path:docs/02-product/COMPETITIVE-LANDSCAPE.md label=docs/02-product/COMPETITIVE-LANDSCAPE.md kind=repository_entry depth=2
    - id=path:docs/02-product/MVP-SCOPE.md label=docs/02-product/MVP-SCOPE.md kind=repository_entry depth=2
    - id=path:docs/02-product/NON-GOALS.md label=docs/02-product/NON-GOALS.md kind=repository_entry depth=2
    - id=path:docs/02-product/POSITIONING.md label=docs/02-product/POSITIONING.md kind=repository_entry depth=2
    - id=path:docs/02-product/PROBLEM-STATEMENT.md label=docs/02-product/PROBLEM-STATEMENT.md kind=repository_entry depth=2
    - id=path:docs/02-product/README.md label=docs/02-product/README.md kind=repository_entry depth=2
    - id=path:docs/02-product/SUCCESS-METRICS.md label=docs/02-product/SUCCESS-METRICS.md kind=repository_entry depth=2
    - id=path:docs/02-product/TARGET-USERS.md label=docs/02-product/TARGET-USERS.md kind=repository_entry depth=2
    - id=path:docs/02-product/USE-CASES.md label=docs/02-product/USE-CASES.md kind=repository_entry depth=2
    - id=path:docs/02-product/USER-JOURNEYS.md label=docs/02-product/USER-JOURNEYS.md kind=repository_entry depth=2
    - id=path:docs/02-product/USER-PERSONAS.md label=docs/02-product/USER-PERSONAS.md kind=repository_entry depth=2
    - id=path:docs/02-product/VALUE-PROPOSITION.md label=docs/02-product/VALUE-PROPOSITION.md kind=repository_entry depth=2
    - id=path:docs/03-requirements label=docs/03-requirements kind=repository_entry depth=1
    - id=path:docs/03-requirements/ACCEPTANCE-CRITERIA-STANDARD.md label=docs/03-requirements/ACCEPTANCE-CRITERIA-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/03-requirements/FUNCTIONAL-REQUIREMENTS.md label=docs/03-requirements/FUNCTIONAL-REQUIREMENTS.md kind=repository_entry depth=2
    - id=path:docs/03-requirements/FUTURE-REQUIREMENTS.md label=docs/03-requirements/FUTURE-REQUIREMENTS.md kind=repository_entry depth=2
    - id=path:docs/03-requirements/MVP-REQUIREMENTS.md label=docs/03-requirements/MVP-REQUIREMENTS.md kind=repository_entry depth=2
    - id=path:docs/03-requirements/NONFUNCTIONAL-REQUIREMENTS.md label=docs/03-requirements/NONFUNCTIONAL-REQUIREMENTS.md kind=repository_entry depth=2
    - id=path:docs/03-requirements/README.md label=docs/03-requirements/README.md kind=repository_entry depth=2
    - id=path:docs/03-requirements/REQUIREMENTS-TRACEABILITY-MATRIX.md label=docs/03-requirements/REQUIREMENTS-TRACEABILITY-MATRIX.md kind=repository_entry depth=2
    - id=path:docs/03-requirements/SYSTEM-QUALITIES.md label=docs/03-requirements/SYSTEM-QUALITIES.md kind=repository_entry depth=2
    - id=path:docs/04-domain label=docs/04-domain kind=repository_entry depth=1
    - id=path:docs/04-domain/BOUNDED-CONTEXTS.md label=docs/04-domain/BOUNDED-CONTEXTS.md kind=repository_entry depth=2
    - id=path:docs/04-domain/CONCEPTUAL-MODEL.md label=docs/04-domain/CONCEPTUAL-MODEL.md kind=repository_entry depth=2
    - id=path:docs/04-domain/DOMAIN-EVENTS.md label=docs/04-domain/DOMAIN-EVENTS.md kind=repository_entry depth=2
    - id=path:docs/04-domain/DOMAIN-INVARIANTS.md label=docs/04-domain/DOMAIN-INVARIANTS.md kind=repository_entry depth=2
    - id=path:docs/04-domain/DOMAIN-MODEL.md label=docs/04-domain/DOMAIN-MODEL.md kind=repository_entry depth=2
    - id=path:docs/04-domain/README.md label=docs/04-domain/README.md kind=repository_entry depth=2
    - id=path:docs/04-domain/UBIQUITOUS-LANGUAGE.md label=docs/04-domain/UBIQUITOUS-LANGUAGE.md kind=repository_entry depth=2
    - id=path:docs/05-architecture label=docs/05-architecture kind=repository_entry depth=1
    - id=path:docs/05-architecture/AGENT-SUPERVISION-ARCHITECTURE.md label=docs/05-architecture/AGENT-SUPERVISION-ARCHITECTURE.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/ARCHITECTURE-PRINCIPLES.md label=docs/05-architecture/ARCHITECTURE-PRINCIPLES.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/CONTEXT-BRIDGE-ARCHITECTURE.md label=docs/05-architecture/CONTEXT-BRIDGE-ARCHITECTURE.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/CONTROL-FLOW.md label=docs/05-architecture/CONTROL-FLOW.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/DATA-FLOW.md label=docs/05-architecture/DATA-FLOW.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/EVOLUTION-ENGINE-ARCHITECTURE.md label=docs/05-architecture/EVOLUTION-ENGINE-ARCHITECTURE.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/EXTENSION-MODEL.md label=docs/05-architecture/EXTENSION-MODEL.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/MCP-INTEGRATION-STRATEGY.md label=docs/05-architecture/MCP-INTEGRATION-STRATEGY.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/MODULE-BOUNDARIES.md label=docs/05-architecture/MODULE-BOUNDARIES.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/PLUGIN-MODEL.md label=docs/05-architecture/PLUGIN-MODEL.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/PROJECT-GRAPH-MODEL.md label=docs/05-architecture/PROJECT-GRAPH-MODEL.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/PROVIDER-MODEL.md label=docs/05-architecture/PROVIDER-MODEL.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/README.md label=docs/05-architecture/README.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/RUNTIME-ARCHITECTURE.md label=docs/05-architecture/RUNTIME-ARCHITECTURE.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/SYSTEM-OVERVIEW.md label=docs/05-architecture/SYSTEM-OVERVIEW.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/VERIFICATION-ARCHITECTURE.md label=docs/05-architecture/VERIFICATION-ARCHITECTURE.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/WORKSPACE-MODEL.md label=docs/05-architecture/WORKSPACE-MODEL.md kind=repository_entry depth=2
    - id=path:docs/05-architecture/WORKTREE-SAFETY-STRATEGY.md label=docs/05-architecture/WORKTREE-SAFETY-STRATEGY.md kind=repository_entry depth=2
    - id=path:docs/06-adrs label=docs/06-adrs kind=repository_entry depth=1
    - id=path:docs/06-adrs/ADR-0000-template.md label=docs/06-adrs/ADR-0000-template.md kind=repository_entry depth=2
    - id=path:docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md label=docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md kind=repository_entry depth=2
    - id=path:docs/06-adrs/ADR-0002-use-monad-as-unified-product-name.md label=docs/06-adrs/ADR-0002-use-monad-as-unified-product-name.md kind=repository_entry depth=2
    - id=path:docs/06-adrs/ADR-0003-use-repo-native-context-as-source-of-truth.md label=docs/06-adrs/ADR-0003-use-repo-native-context-as-source-of-truth.md kind=repository_entry depth=2
    - id=path:docs/06-adrs/ADR-0004-use-work-packets-as-primary-delivery-unit.md label=docs/06-adrs/ADR-0004-use-work-packets-as-primary-delivery-unit.md kind=repository_entry depth=2
    - id=path:docs/06-adrs/ADR-0005-use-multi-crate-rust-workspace.md label=docs/06-adrs/ADR-0005-use-multi-crate-rust-workspace.md kind=repository_entry depth=2
    - id=path:docs/06-adrs/ADR-0006-keep-cli-thin-and-core-durable.md label=docs/06-adrs/ADR-0006-keep-cli-thin-and-core-durable.md kind=repository_entry depth=2
    - id=path:docs/06-adrs/ADR-0007-use-supervised-autonomy-for-agent-workflows.md label=docs/06-adrs/ADR-0007-use-supervised-autonomy-for-agent-workflows.md kind=repository_entry depth=2
    - id=path:docs/06-adrs/ADR-0008-coordinate-native-tools-rather-than-replace-them.md label=docs/06-adrs/ADR-0008-coordinate-native-tools-rather-than-replace-them.md kind=repository_entry depth=2
    - id=path:docs/06-adrs/README.md label=docs/06-adrs/README.md kind=repository_entry depth=2
    - id=path:docs/07-workflow label=docs/07-workflow kind=repository_entry depth=1
    - id=path:docs/07-workflow/BRANCHING-STANDARD.md label=docs/07-workflow/BRANCHING-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/COMMIT-STANDARD.md label=docs/07-workflow/COMMIT-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/CONTEXT-UPDATE-STANDARD.md label=docs/07-workflow/CONTEXT-UPDATE-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/DEFINITION-OF-DONE.md label=docs/07-workflow/DEFINITION-OF-DONE.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/DEFINITION-OF-READY.md label=docs/07-workflow/DEFINITION-OF-READY.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/DELIVERABLE-STANDARD.md label=docs/07-workflow/DELIVERABLE-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/EPIC-STANDARD.md label=docs/07-workflow/EPIC-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/OPERATING-MODEL.md label=docs/07-workflow/OPERATING-MODEL.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/README.md label=docs/07-workflow/README.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/RELEASE-STANDARD.md label=docs/07-workflow/RELEASE-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/REVIEW-STANDARD.md label=docs/07-workflow/REVIEW-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/TASK-STANDARD.md label=docs/07-workflow/TASK-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/VERIFICATION-STANDARD.md label=docs/07-workflow/VERIFICATION-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/WORK-HIERARCHY.md label=docs/07-workflow/WORK-HIERARCHY.md kind=repository_entry depth=2
    - id=path:docs/07-workflow/WORK-PACKET-STANDARD.md label=docs/07-workflow/WORK-PACKET-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/08-context label=docs/08-context kind=repository_entry depth=1
    - id=path:docs/08-context/CONTEXT-ARTIFACT-SCHEMAS.md label=docs/08-context/CONTEXT-ARTIFACT-SCHEMAS.md kind=repository_entry depth=2
    - id=path:docs/08-context/CONTEXT-BRIDGE.md label=docs/08-context/CONTEXT-BRIDGE.md kind=repository_entry depth=2
    - id=path:docs/08-context/CONTEXT-PACK-STANDARD.md label=docs/08-context/CONTEXT-PACK-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/08-context/CURRENT-STATE-STANDARD.md label=docs/08-context/CURRENT-STATE-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/08-context/DECISION-LOG-STANDARD.md label=docs/08-context/DECISION-LOG-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/08-context/GENERATED-CONTEXT-STANDARD.md label=docs/08-context/GENERATED-CONTEXT-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/08-context/HANDOFF-STANDARD.md label=docs/08-context/HANDOFF-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/08-context/README.md label=docs/08-context/README.md kind=repository_entry depth=2
    - id=path:docs/08-context/REHYDRATION-STANDARD.md label=docs/08-context/REHYDRATION-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/08-context/SESSION-CHRONICLE-STANDARD.md label=docs/08-context/SESSION-CHRONICLE-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/09-ai label=docs/09-ai kind=repository_entry depth=1
    - id=path:docs/09-ai/AGENT-RUNBOOK.md label=docs/09-ai/AGENT-RUNBOOK.md kind=repository_entry depth=2
    - id=path:docs/09-ai/AGENT-SAFETY-RULES.md label=docs/09-ai/AGENT-SAFETY-RULES.md kind=repository_entry depth=2
    - id=path:docs/09-ai/AI-COLLABORATION-RULES.md label=docs/09-ai/AI-COLLABORATION-RULES.md kind=repository_entry depth=2
    - id=path:docs/09-ai/BOOTSTRAP-PROMPT.md label=docs/09-ai/BOOTSTRAP-PROMPT.md kind=repository_entry depth=2
    - id=path:docs/09-ai/CURRENT-STATE.md label=docs/09-ai/CURRENT-STATE.md kind=repository_entry depth=2
    - id=path:docs/09-ai/FRESH-CHAT-HANDOFF.md label=docs/09-ai/FRESH-CHAT-HANDOFF.md kind=repository_entry depth=2
    - id=path:docs/09-ai/MCP-TOOLING-STANDARD.md label=docs/09-ai/MCP-TOOLING-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/09-ai/MODEL-PROVIDER-STANDARD.md label=docs/09-ai/MODEL-PROVIDER-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/09-ai/PROMPTING-STANDARD.md label=docs/09-ai/PROMPTING-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/09-ai/README.md label=docs/09-ai/README.md kind=repository_entry depth=2
    - id=path:docs/10-engineering label=docs/10-engineering kind=repository_entry depth=1
    - id=path:docs/10-engineering/CLI-UX-STANDARD.md label=docs/10-engineering/CLI-UX-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/CODING-STANDARD.md label=docs/10-engineering/CODING-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/DEPENDENCY-STANDARD.md label=docs/10-engineering/DEPENDENCY-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/DIAGNOSTIC-STANDARD.md label=docs/10-engineering/DIAGNOSTIC-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/ERROR-HANDLING-STANDARD.md label=docs/10-engineering/ERROR-HANDLING-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/FIXTURE-STANDARD.md label=docs/10-engineering/FIXTURE-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/OUTPUT-FORMAT-STANDARD.md label=docs/10-engineering/OUTPUT-FORMAT-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/README.md label=docs/10-engineering/README.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/RUST-CODING-STANDARD.md label=docs/10-engineering/RUST-CODING-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/RUST-LEARNING-NOTES.md label=docs/10-engineering/RUST-LEARNING-NOTES.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/RUST-VERIFICATION.md label=docs/10-engineering/RUST-VERIFICATION.md kind=repository_entry depth=2
    - id=path:docs/10-engineering/TESTING-STANDARD.md label=docs/10-engineering/TESTING-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/11-security label=docs/11-security kind=repository_entry depth=1
    - id=path:docs/11-security/AGENT-SAFETY-MODEL.md label=docs/11-security/AGENT-SAFETY-MODEL.md kind=repository_entry depth=2
    - id=path:docs/11-security/COMMAND-EXECUTION-SAFETY.md label=docs/11-security/COMMAND-EXECUTION-SAFETY.md kind=repository_entry depth=2
    - id=path:docs/11-security/FILE-OPERATION-SAFETY.md label=docs/11-security/FILE-OPERATION-SAFETY.md kind=repository_entry depth=2
    - id=path:docs/11-security/MCP-SAFETY-BOUNDARIES.md label=docs/11-security/MCP-SAFETY-BOUNDARIES.md kind=repository_entry depth=2
    - id=path:docs/11-security/README.md label=docs/11-security/README.md kind=repository_entry depth=2
    - id=path:docs/11-security/RESPONSIBLE-DISCLOSURE.md label=docs/11-security/RESPONSIBLE-DISCLOSURE.md kind=repository_entry depth=2
    - id=path:docs/11-security/SANDBOXING-PRINCIPLES.md label=docs/11-security/SANDBOXING-PRINCIPLES.md kind=repository_entry depth=2
    - id=path:docs/11-security/SECRET-HANDLING.md label=docs/11-security/SECRET-HANDLING.md kind=repository_entry depth=2
    - id=path:docs/11-security/SECURITY-MODEL.md label=docs/11-security/SECURITY-MODEL.md kind=repository_entry depth=2
    - id=path:docs/11-security/SUPPLY-CHAIN-SECURITY.md label=docs/11-security/SUPPLY-CHAIN-SECURITY.md kind=repository_entry depth=2
    - id=path:docs/11-security/THREAT-MODEL.md label=docs/11-security/THREAT-MODEL.md kind=repository_entry depth=2
    - id=path:docs/12-verification label=docs/12-verification kind=repository_entry depth=1
    - id=path:docs/12-verification/CHECK-REGISTRY-STANDARD.md label=docs/12-verification/CHECK-REGISTRY-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/12-verification/EVIDENCE-PACKET-STANDARD.md label=docs/12-verification/EVIDENCE-PACKET-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/12-verification/EXIT-CODE-STANDARD.md label=docs/12-verification/EXIT-CODE-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/12-verification/QUALITY-GATES.md label=docs/12-verification/QUALITY-GATES.md kind=repository_entry depth=2
    - id=path:docs/12-verification/README.md label=docs/12-verification/README.md kind=repository_entry depth=2
    - id=path:docs/12-verification/REPORTING-STANDARD.md label=docs/12-verification/REPORTING-STANDARD.md kind=repository_entry depth=2
    - id=path:docs/12-verification/TEST-MATRIX.md label=docs/12-verification/TEST-MATRIX.md kind=repository_entry depth=2
    - id=path:docs/12-verification/VERIFICATION-BASELINE.md label=docs/12-verification/VERIFICATION-BASELINE.md kind=repository_entry depth=2
    - id=path:docs/12-verification/VERIFICATION-MODEL.md label=docs/12-verification/VERIFICATION-MODEL.md kind=repository_entry depth=2
    - id=path:docs/13-operations label=docs/13-operations kind=repository_entry depth=1
    - id=path:docs/13-operations/BACKUP-AND-EXPORTS.md label=docs/13-operations/BACKUP-AND-EXPORTS.md kind=repository_entry depth=2
    - id=path:docs/13-operations/LOCAL-DEVELOPMENT.md label=docs/13-operations/LOCAL-DEVELOPMENT.md kind=repository_entry depth=2
    - id=path:docs/13-operations/MAINTENANCE-MODEL.md label=docs/13-operations/MAINTENANCE-MODEL.md kind=repository_entry depth=2
    - id=path:docs/13-operations/README.md label=docs/13-operations/README.md kind=repository_entry depth=2
    - id=path:docs/13-operations/RELEASE-PROCESS.md label=docs/13-operations/RELEASE-PROCESS.md kind=repository_entry depth=2
    - id=path:docs/13-operations/REPOSITORY-SETUP.md label=docs/13-operations/REPOSITORY-SETUP.md kind=repository_entry depth=2
    - id=path:docs/13-operations/SUPPORT-MODEL.md label=docs/13-operations/SUPPORT-MODEL.md kind=repository_entry depth=2
    - id=path:docs/13-operations/TOOLCHAIN-SETUP.md label=docs/13-operations/TOOLCHAIN-SETUP.md kind=repository_entry depth=2
    - id=path:docs/13-operations/VERSIONING-POLICY.md label=docs/13-operations/VERSIONING-POLICY.md kind=repository_entry depth=2
    - id=path:docs/14-integrations label=docs/14-integrations kind=repository_entry depth=1
    - id=path:docs/14-integrations/GITHUB-INTEGRATION.md label=docs/14-integrations/GITHUB-INTEGRATION.md kind=repository_entry depth=2
    - id=path:docs/14-integrations/GITHUB-ISSUES-WORKFLOW.md label=docs/14-integrations/GITHUB-ISSUES-WORKFLOW.md kind=repository_entry depth=2
    - id=path:docs/14-integrations/GITHUB-PROJECTS-WORKFLOW.md label=docs/14-integrations/GITHUB-PROJECTS-WORKFLOW.md kind=repository_entry depth=2
    - id=path:docs/14-integrations/MCP-INTEGRATION.md label=docs/14-integrations/MCP-INTEGRATION.md kind=repository_entry depth=2
    - id=path:docs/14-integrations/MODEL-PROVIDER-INTEGRATIONS.md label=docs/14-integrations/MODEL-PROVIDER-INTEGRATIONS.md kind=repository_entry depth=2
    - id=path:docs/14-integrations/NATIVE-TOOL-ADAPTERS.md label=docs/14-integrations/NATIVE-TOOL-ADAPTERS.md kind=repository_entry depth=2
    - id=path:docs/14-integrations/README.md label=docs/14-integrations/README.md kind=repository_entry depth=2
    - id=path:docs/15-business label=docs/15-business kind=repository_entry depth=1
    - id=path:docs/15-business/BUSINESS-THESIS.md label=docs/15-business/BUSINESS-THESIS.md kind=repository_entry depth=2
    - id=path:docs/15-business/CUSTOMER-SEGMENTS.md label=docs/15-business/CUSTOMER-SEGMENTS.md kind=repository_entry depth=2
    - id=path:docs/15-business/DISTRIBUTION-STRATEGY.md label=docs/15-business/DISTRIBUTION-STRATEGY.md kind=repository_entry depth=2
    - id=path:docs/15-business/PRICING-HYPOTHESES.md label=docs/15-business/PRICING-HYPOTHESES.md kind=repository_entry depth=2
    - id=path:docs/15-business/README.md label=docs/15-business/README.md kind=repository_entry depth=2
    - id=path:docs/15-business/REPO-AUDIT-OFFER.md label=docs/15-business/REPO-AUDIT-OFFER.md kind=repository_entry depth=2
    - id=path:docs/15-business/RISKS.md label=docs/15-business/RISKS.md kind=repository_entry depth=2
    - id=path:docs/15-business/VALIDATION-PLAN.md label=docs/15-business/VALIDATION-PLAN.md kind=repository_entry depth=2
    - id=path:docs/16-reference label=docs/16-reference kind=repository_entry depth=1
    - id=path:docs/16-reference/COMMAND-CATALOG.md label=docs/16-reference/COMMAND-CATALOG.md kind=repository_entry depth=2
    - id=path:docs/16-reference/CONFIGURATION-REFERENCE.md label=docs/16-reference/CONFIGURATION-REFERENCE.md kind=repository_entry depth=2
    - id=path:docs/16-reference/FAQ.md label=docs/16-reference/FAQ.md kind=repository_entry depth=2
    - id=path:docs/16-reference/MONAD-TOML-REFERENCE.md label=docs/16-reference/MONAD-TOML-REFERENCE.md kind=repository_entry depth=2
    - id=path:docs/16-reference/README.md label=docs/16-reference/README.md kind=repository_entry depth=2
    - id=path:docs/16-reference/RESOURCES.md label=docs/16-reference/RESOURCES.md kind=repository_entry depth=2
    - id=path:docs/16-reference/TERMINOLOGY.md label=docs/16-reference/TERMINOLOGY.md kind=repository_entry depth=2
    - id=path:docs/README.md label=docs/README.md kind=repository_entry depth=1
    - id=path:docs/ai label=docs/ai kind=repository_entry depth=1
    - id=path:docs/ai/AGENT-RUNBOOK.md label=docs/ai/AGENT-RUNBOOK.md kind=repository_entry depth=2
    - id=path:docs/ai/BOOTSTRAP-PROMPT.md label=docs/ai/BOOTSTRAP-PROMPT.md kind=repository_entry depth=2
    - id=path:docs/architecture label=docs/architecture kind=repository_entry depth=1
    - id=path:docs/architecture/DRAFT-SANDBOX-WORKFLOW.md label=docs/architecture/DRAFT-SANDBOX-WORKFLOW.md kind=repository_entry depth=2
    - id=path:docs/architecture/MCP-INTEGRATION-STRATEGY.md label=docs/architecture/MCP-INTEGRATION-STRATEGY.md kind=repository_entry depth=2
    - id=path:docs/architecture/SUPERVISED-AGENT-WORKFLOW.md label=docs/architecture/SUPERVISED-AGENT-WORKFLOW.md kind=repository_entry depth=2
    - id=path:docs/architecture/WORKTREE-SAFETY-STRATEGY.md label=docs/architecture/WORKTREE-SAFETY-STRATEGY.md kind=repository_entry depth=2
    - id=path:docs/context label=docs/context kind=repository_entry depth=1
    - id=path:docs/context/CONTEXT-FRESHNESS-POLICY.md label=docs/context/CONTEXT-FRESHNESS-POLICY.md kind=repository_entry depth=2
    - id=path:docs/context/RELEASE-CONTEXT-STATE.md label=docs/context/RELEASE-CONTEXT-STATE.md kind=repository_entry depth=2
    - id=path:docs/development label=docs/development kind=repository_entry depth=1
    - id=path:docs/development/LOCAL-BUILD.md label=docs/development/LOCAL-BUILD.md kind=repository_entry depth=2
    - id=path:docs/development/LOCAL-VERIFY.md label=docs/development/LOCAL-VERIFY.md kind=repository_entry depth=2
    - id=path:docs/development/README.md label=docs/development/README.md kind=repository_entry depth=2
    - id=path:docs/development/WP-E8-004-LOCAL-BUILD-VERIFY-EVIDENCE.md label=docs/development/WP-E8-004-LOCAL-BUILD-VERIFY-EVIDENCE.md kind=repository_entry depth=2
    - id=path:docs/project label=docs/project kind=repository_entry depth=1
    - id=path:docs/project/FOUNDATION-CLOSURE-AUDIT.md label=docs/project/FOUNDATION-CLOSURE-AUDIT.md kind=repository_entry depth=2
    - id=path:docs/project/FOUNDATION-CLOSURE-REPORT.md label=docs/project/FOUNDATION-CLOSURE-REPORT.md kind=repository_entry depth=2
    - id=path:docs/project/MVP-COMMAND-REFERENCE.md label=docs/project/MVP-COMMAND-REFERENCE.md kind=repository_entry depth=2
    - id=path:docs/project/MVP-READINESS-REPORT.md label=docs/project/MVP-READINESS-REPORT.md kind=repository_entry depth=2
    - id=path:docs/project/MVP-SCOPE-FREEZE.md label=docs/project/MVP-SCOPE-FREEZE.md kind=repository_entry depth=2
    - id=path:docs/project/WP-E7-002-CLI-UX-EVIDENCE.md label=docs/project/WP-E7-002-CLI-UX-EVIDENCE.md kind=repository_entry depth=2
    - id=path:docs/project/WP-E7-003-COMMAND-SMOKE-TEST-EVIDENCE.md label=docs/project/WP-E7-003-COMMAND-SMOKE-TEST-EVIDENCE.md kind=repository_entry depth=2
    - id=path:docs/project/WP-E7-004-DOC-ALIGNMENT-EVIDENCE.md label=docs/project/WP-E7-004-DOC-ALIGNMENT-EVIDENCE.md kind=repository_entry depth=2
    - id=path:docs/project/WP-E7-005-DRY-RUN-NO-WRITE-EVIDENCE.md label=docs/project/WP-E7-005-DRY-RUN-NO-WRITE-EVIDENCE.md kind=repository_entry depth=2
    - id=path:docs/project/WP-E8-001-SCOPE-FREEZE-EVIDENCE.md label=docs/project/WP-E8-001-SCOPE-FREEZE-EVIDENCE.md kind=repository_entry depth=2
    - id=path:docs/release label=docs/release kind=repository_entry depth=1
    - id=path:docs/release/BUILD-METADATA.md label=docs/release/BUILD-METADATA.md kind=repository_entry depth=2
    - id=path:docs/release/E8-CLOSEOUT.md label=docs/release/E8-CLOSEOUT.md kind=repository_entry depth=2
    - id=path:docs/release/E9-CLOSEOUT.md label=docs/release/E9-CLOSEOUT.md kind=repository_entry depth=2
    - id=path:docs/release/E9-STABILIZATION-PLAN.md label=docs/release/E9-STABILIZATION-PLAN.md kind=repository_entry depth=2
    - id=path:docs/release/FIRST-PUBLIC-PRERELEASE-BOUNDARY.md label=docs/release/FIRST-PUBLIC-PRERELEASE-BOUNDARY.md kind=repository_entry depth=2
    - id=path:docs/release/MVP-CANDIDATE-TAG-RECORD.md label=docs/release/MVP-CANDIDATE-TAG-RECORD.md kind=repository_entry depth=2
    - id=path:docs/release/MVP-CANDIDATE-VERIFICATION-AUDIT.md label=docs/release/MVP-CANDIDATE-VERIFICATION-AUDIT.md kind=repository_entry depth=2
    - id=path:docs/release/PUBLIC-CLAIMS-AUDIT.md label=docs/release/PUBLIC-CLAIMS-AUDIT.md kind=repository_entry depth=2
    - id=path:docs/release/PUBLIC-PRERELEASE-CHECKLIST.md label=docs/release/PUBLIC-PRERELEASE-CHECKLIST.md kind=repository_entry depth=2
    - id=path:docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md label=docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md kind=repository_entry depth=2
    - id=path:docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md label=docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md kind=repository_entry depth=2
    - id=path:docs/release/PUBLIC-PRERELEASE-NOTES.md label=docs/release/PUBLIC-PRERELEASE-NOTES.md kind=repository_entry depth=2
    - id=path:docs/release/PUBLIC-READINESS-GAP-AUDIT.md label=docs/release/PUBLIC-READINESS-GAP-AUDIT.md kind=repository_entry depth=2
    - id=path:docs/release/README.md label=docs/release/README.md kind=repository_entry depth=2
    - id=path:docs/release/RELEASE-NOTES-TEMPLATE.md label=docs/release/RELEASE-NOTES-TEMPLATE.md kind=repository_entry depth=2
    - id=path:docs/release/VERSIONING.md label=docs/release/VERSIONING.md kind=repository_entry depth=2
    - id=path:docs/release/WP-E8-002-RELEASE-DOCS-EVIDENCE.md label=docs/release/WP-E8-002-RELEASE-DOCS-EVIDENCE.md kind=repository_entry depth=2
    - id=path:docs/release/WP-E8-003-VERSION-BUILD-METADATA-EVIDENCE.md label=docs/release/WP-E8-003-VERSION-BUILD-METADATA-EVIDENCE.md kind=repository_entry depth=2
    - id=path:docs/repository label=docs/repository kind=repository_entry depth=1
    - id=path:docs/repository/GENERATED-ARTIFACT-POLICY.md label=docs/repository/GENERATED-ARTIFACT-POLICY.md kind=repository_entry depth=2
    - id=path:docs/repository/REPOSITORY-HYGIENE-REVIEW.md label=docs/repository/REPOSITORY-HYGIENE-REVIEW.md kind=repository_entry depth=2
    - id=path:docs/security label=docs/security kind=repository_entry depth=1
    - id=path:docs/security/AGENT-SAFETY-MODEL.md label=docs/security/AGENT-SAFETY-MODEL.md kind=repository_entry depth=2
    - id=path:docs/security/MCP-SAFETY-BOUNDARIES.md label=docs/security/MCP-SAFETY-BOUNDARIES.md kind=repository_entry depth=2
    - id=path:docs/wiki label=docs/wiki kind=repository_entry depth=1
    - id=path:docs/wiki/.artifacts label=docs/wiki/.artifacts kind=repository_entry depth=2
    - id=path:docs/wiki/README.md label=docs/wiki/README.md kind=repository_entry depth=2
    - id=path:docs/wiki/contents.md label=docs/wiki/contents.md kind=repository_entry depth=2
    - id=path:docs/wiki/contents.raw.json label=docs/wiki/contents.raw.json kind=repository_entry depth=2
    - id=path:docs/wiki/deepwiki-dump-monad-workspace label=docs/wiki/deepwiki-dump-monad-workspace kind=repository_entry depth=2
    - id=path:docs/wiki/deepwiki-dump-monad-workspace/README.md label=docs/wiki/deepwiki-dump-monad-workspace/README.md kind=repository_entry depth=3
    - id=path:docs/wiki/deepwiki-dump-monad-workspace/dump-deepwiki.mjs label=docs/wiki/deepwiki-dump-monad-workspace/dump-deepwiki.mjs kind=repository_entry depth=3
    - id=path:docs/wiki/deepwiki-dump-monad-workspace/run-dump.sh label=docs/wiki/deepwiki-dump-monad-workspace/run-dump.sh kind=repository_entry depth=3
    - id=path:docs/wiki/structure.md label=docs/wiki/structure.md kind=repository_entry depth=2
    - id=path:docs/wiki/structure.raw.json label=docs/wiki/structure.raw.json kind=repository_entry depth=2
    - id=path:docs/wiki/tools.raw.json label=docs/wiki/tools.raw.json kind=repository_entry depth=2
    - id=path:docs/workflow label=docs/workflow kind=repository_entry depth=1
    - id=path:docs/workflow/APPROVAL-GATES.md label=docs/workflow/APPROVAL-GATES.md kind=repository_entry depth=2
    - id=path:monad.toml label=monad.toml kind=repository_entry depth=0
    - id=path:nano label=nano kind=repository_entry depth=0
    - id=path:package.json label=package.json kind=repository_entry depth=0
    - id=path:rust-toolchain.toml label=rust-toolchain.toml kind=repository_entry depth=0
    - id=path:sue close 72  label=sue close 72  kind=repository_entry depth=0
    - id=path:target label=target kind=repository_entry depth=0
    - id=path:tools label=tools kind=repository_entry depth=0
    - id=path:tools/github label=tools/github kind=repository_entry depth=1
    - id=path:tools/github/seed-e10-e11-issues.sh label=tools/github/seed-e10-e11-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e12-issues.sh label=tools/github/seed-e12-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e13-issues.sh label=tools/github/seed-e13-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e14-issues.sh label=tools/github/seed-e14-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e15-issues.sh label=tools/github/seed-e15-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e16-issues.sh label=tools/github/seed-e16-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e17-issues.sh label=tools/github/seed-e17-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e18-issues.sh label=tools/github/seed-e18-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e19-issues.sh label=tools/github/seed-e19-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e2-issues.sh label=tools/github/seed-e2-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e3-issues.sh label=tools/github/seed-e3-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e4-issues.sh label=tools/github/seed-e4-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e5-issues.sh label=tools/github/seed-e5-issues.sh kind=repository_entry depth=2
    - id=path:tools/github/seed-e6-issues.sh label=tools/github/seed-e6-issues.sh kind=repository_entry depth=2
    - id=path:tools/scripts label=tools/scripts kind=repository_entry depth=1
    - id=path:tools/scripts/audit-foundation-closure.sh label=tools/scripts/audit-foundation-closure.sh kind=repository_entry depth=2
    - id=path:tools/scripts/audit-mvp-candidate-verification.sh label=tools/scripts/audit-mvp-candidate-verification.sh kind=repository_entry depth=2
    - id=path:tools/scripts/check-adr-records.py label=tools/scripts/check-adr-records.py kind=repository_entry depth=2
    - id=path:tools/scripts/check-context-records.py label=tools/scripts/check-context-records.py kind=repository_entry depth=2
    - id=path:tools/scripts/check-deliverable-records.py label=tools/scripts/check-deliverable-records.py kind=repository_entry depth=2
    - id=path:tools/scripts/check-epic-records.py label=tools/scripts/check-epic-records.py kind=repository_entry depth=2
    - id=path:tools/scripts/check-markdown-frontmatter.py label=tools/scripts/check-markdown-frontmatter.py kind=repository_entry depth=2
    - id=path:tools/scripts/check-required-paths.py label=tools/scripts/check-required-paths.py kind=repository_entry depth=2
    - id=path:tools/scripts/check-task-records.py label=tools/scripts/check-task-records.py kind=repository_entry depth=2
    - id=path:tools/scripts/check-work-records.py label=tools/scripts/check-work-records.py kind=repository_entry depth=2
    - id=path:tools/scripts/github label=tools/scripts/github kind=repository_entry depth=2
    - id=path:tools/scripts/github/create-e7-mvp-hardening-issues.sh label=tools/scripts/github/create-e7-mvp-hardening-issues.sh kind=repository_entry depth=3
    - id=path:tools/scripts/github/create-e7-workpacket-issues.sh label=tools/scripts/github/create-e7-workpacket-issues.sh kind=repository_entry depth=3
    - id=path:tools/scripts/github/create-e8-release-prep-issues.sh label=tools/scripts/github/create-e8-release-prep-issues.sh kind=repository_entry depth=3
    - id=path:tools/scripts/github/create-e9-post-mvp-stabilization-issues.sh label=tools/scripts/github/create-e9-post-mvp-stabilization-issues.sh kind=repository_entry depth=3
    - id=path:tools/scripts/verify-no-write-commands.sh label=tools/scripts/verify-no-write-commands.sh kind=repository_entry depth=2
    - id=path:tools/scripts/verify.sh label=tools/scripts/verify.sh kind=repository_entry depth=2
    - id=path:work label=work kind=repository_entry depth=0
    - id=path:work/README.md label=work/README.md kind=repository_entry depth=1
    - id=path:work/deliverables label=work/deliverables kind=repository_entry depth=1
    - id=path:work/deliverables/E0 label=work/deliverables/E0 kind=repository_entry depth=2
    - id=path:work/deliverables/E0/D-WP-E0-010-001-deliverable-record-index.md label=work/deliverables/E0/D-WP-E0-010-001-deliverable-record-index.md kind=repository_entry depth=3
    - id=path:work/deliverables/E0/D-WP-E0-010-002-deliverable-record-verifier.md label=work/deliverables/E0/D-WP-E0-010-002-deliverable-record-verifier.md kind=repository_entry depth=3
    - id=path:work/deliverables/E0/D-WP-E0-010-003-verification-baseline-update.md label=work/deliverables/E0/D-WP-E0-010-003-verification-baseline-update.md kind=repository_entry depth=3
    - id=path:work/deliverables/E0/D-WP-E0-011-001-e0-closure-record.md label=work/deliverables/E0/D-WP-E0-011-001-e0-closure-record.md kind=repository_entry depth=3
    - id=path:work/deliverables/E0/D-WP-E0-011-002-e1-starting-point.md label=work/deliverables/E0/D-WP-E0-011-002-e1-starting-point.md kind=repository_entry depth=3
    - id=path:work/deliverables/E0/D-WP-E0-011-003-context-handoff-update.md label=work/deliverables/E0/D-WP-E0-011-003-context-handoff-update.md kind=repository_entry depth=3
    - id=path:work/deliverables/E0/README.md label=work/deliverables/E0/README.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1 label=work/deliverables/E1 kind=repository_entry depth=2
    - id=path:work/deliverables/E1/D-WP-E1-001-001-rust-workspace-manifest.md label=work/deliverables/E1/D-WP-E1-001-001-rust-workspace-manifest.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-001-002-core-runtime-library.md label=work/deliverables/E1/D-WP-E1-001-002-core-runtime-library.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-001-003-thin-cli-entrypoint.md label=work/deliverables/E1/D-WP-E1-001-003-thin-cli-entrypoint.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-002-001-diagnostics-module.md label=work/deliverables/E1/D-WP-E1-002-001-diagnostics-module.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-002-002-core-runtime-exports.md label=work/deliverables/E1/D-WP-E1-002-002-core-runtime-exports.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-002-003-diagnostics-context-handoff.md label=work/deliverables/E1/D-WP-E1-002-003-diagnostics-context-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-003-001-core-error-module.md label=work/deliverables/E1/D-WP-E1-003-001-core-error-module.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-003-002-core-error-exports.md label=work/deliverables/E1/D-WP-E1-003-002-core-error-exports.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-003-003-core-error-context-handoff.md label=work/deliverables/E1/D-WP-E1-003-003-core-error-context-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-004-001-workspace-context-module.md label=work/deliverables/E1/D-WP-E1-004-001-workspace-context-module.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-004-002-workspace-context-exports.md label=work/deliverables/E1/D-WP-E1-004-002-workspace-context-exports.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-004-003-workspace-context-handoff.md label=work/deliverables/E1/D-WP-E1-004-003-workspace-context-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-005-001-root-monad-manifest.md label=work/deliverables/E1/D-WP-E1-005-001-root-monad-manifest.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-005-002-manifest-model-module.md label=work/deliverables/E1/D-WP-E1-005-002-manifest-model-module.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-005-003-manifest-model-exports.md label=work/deliverables/E1/D-WP-E1-005-003-manifest-model-exports.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-005-004-manifest-model-handoff.md label=work/deliverables/E1/D-WP-E1-005-004-manifest-model-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-006-001-manifest-parsing-dependencies.md label=work/deliverables/E1/D-WP-E1-006-001-manifest-parsing-dependencies.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-006-002-manifest-loading-runtime.md label=work/deliverables/E1/D-WP-E1-006-002-manifest-loading-runtime.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-006-003-manifest-loading-exports.md label=work/deliverables/E1/D-WP-E1-006-003-manifest-loading-exports.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-006-004-manifest-loading-handoff.md label=work/deliverables/E1/D-WP-E1-006-004-manifest-loading-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-007-002-cli-info-verification.md label=work/deliverables/E1/D-WP-E1-007-002-cli-info-verification.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-007-003-cli-info-handoff.md label=work/deliverables/E1/D-WP-E1-007-003-cli-info-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-008-001-core-workspace-checks.md label=work/deliverables/E1/D-WP-E1-008-001-core-workspace-checks.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-008-002-cli-check-command.md label=work/deliverables/E1/D-WP-E1-008-002-cli-check-command.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-008-003-cli-check-verification.md label=work/deliverables/E1/D-WP-E1-008-003-cli-check-verification.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-008-004-cli-check-handoff.md label=work/deliverables/E1/D-WP-E1-008-004-cli-check-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-009-001-repository-contract-module.md label=work/deliverables/E1/D-WP-E1-009-001-repository-contract-module.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-009-002-checks-integration.md label=work/deliverables/E1/D-WP-E1-009-002-checks-integration.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-009-003-repository-contract-handoff.md label=work/deliverables/E1/D-WP-E1-009-003-repository-contract-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-010-001-output-formatting-module.md label=work/deliverables/E1/D-WP-E1-010-001-output-formatting-module.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-010-002-cli-output-integration.md label=work/deliverables/E1/D-WP-E1-010-002-cli-output-integration.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-010-003-output-formatting-handoff.md label=work/deliverables/E1/D-WP-E1-010-003-output-formatting-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-011-001-cli-output-format-argument.md label=work/deliverables/E1/D-WP-E1-011-001-cli-output-format-argument.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-011-002-cli-output-format-tests.md label=work/deliverables/E1/D-WP-E1-011-002-cli-output-format-tests.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-011-003-cli-output-format-handoff.md label=work/deliverables/E1/D-WP-E1-011-003-cli-output-format-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-013-001-e1-closure-record.md label=work/deliverables/E1/D-WP-E1-013-001-e1-closure-record.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-013-002-e2-starting-point.md label=work/deliverables/E1/D-WP-E1-013-002-e2-starting-point.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/D-WP-E1-013-003-e2-context-handoff.md label=work/deliverables/E1/D-WP-E1-013-003-e2-context-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E1/README.md label=work/deliverables/E1/README.md kind=repository_entry depth=3
    - id=path:work/deliverables/E10 label=work/deliverables/E10 kind=repository_entry depth=2
    - id=path:work/deliverables/E10/WP-E10-001-public-claims-audit.md label=work/deliverables/E10/WP-E10-001-public-claims-audit.md kind=repository_entry depth=3
    - id=path:work/deliverables/E10/WP-E10-002-public-prerelease-evidence-checklist.md label=work/deliverables/E10/WP-E10-002-public-prerelease-evidence-checklist.md kind=repository_entry depth=3
    - id=path:work/deliverables/E10/WP-E10-003-distribution-posture.md label=work/deliverables/E10/WP-E10-003-distribution-posture.md kind=repository_entry depth=3
    - id=path:work/deliverables/E10/WP-E10-004-public-prerelease-notes.md label=work/deliverables/E10/WP-E10-004-public-prerelease-notes.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2 label=work/deliverables/E2 kind=repository_entry depth=2
    - id=path:work/deliverables/E2/D-WP-E2-001-001-repository-inspection-module.md label=work/deliverables/E2/D-WP-E2-001-001-repository-inspection-module.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-001-002-repository-inspection-exports.md label=work/deliverables/E2/D-WP-E2-001-002-repository-inspection-exports.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-001-003-workspace-check-integration.md label=work/deliverables/E2/D-WP-E2-001-003-workspace-check-integration.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-001-004-repository-inspection-handoff.md label=work/deliverables/E2/D-WP-E2-001-004-repository-inspection-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-002-001-inspection-summary-rendering.md label=work/deliverables/E2/D-WP-E2-002-001-inspection-summary-rendering.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-002-002-cli-inspect-command.md label=work/deliverables/E2/D-WP-E2-002-002-cli-inspect-command.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-002-003-inspect-json-output.md label=work/deliverables/E2/D-WP-E2-002-003-inspect-json-output.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-002-004-inspect-verification.md label=work/deliverables/E2/D-WP-E2-002-004-inspect-verification.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-003-001-expanded-repository-entry-roles.md label=work/deliverables/E2/D-WP-E2-003-001-expanded-repository-entry-roles.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-003-002-expanded-classification-rules.md label=work/deliverables/E2/D-WP-E2-003-002-expanded-classification-rules.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-003-003-traversal-policy-hardening.md label=work/deliverables/E2/D-WP-E2-003-003-traversal-policy-hardening.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-003-004-classification-handoff.md label=work/deliverables/E2/D-WP-E2-003-004-classification-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-004-001-repository-entry-category.md label=work/deliverables/E2/D-WP-E2-004-001-repository-entry-category.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-004-002-category-summary-metrics.md label=work/deliverables/E2/D-WP-E2-004-002-category-summary-metrics.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-004-003-inspect-metrics-output.md label=work/deliverables/E2/D-WP-E2-004-003-inspect-metrics-output.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-004-004-metrics-handoff.md label=work/deliverables/E2/D-WP-E2-004-004-metrics-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-005-001-traversal-planning-types.md label=work/deliverables/E2/D-WP-E2-005-001-traversal-planning-types.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-005-002-conservative-traversal-guardrails.md label=work/deliverables/E2/D-WP-E2-005-002-conservative-traversal-guardrails.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-005-003-traversal-plan-output.md label=work/deliverables/E2/D-WP-E2-005-003-traversal-plan-output.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-005-004-traversal-guardrails-handoff.md label=work/deliverables/E2/D-WP-E2-005-004-traversal-guardrails-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-006-001-bounded-traversal-model.md label=work/deliverables/E2/D-WP-E2-006-001-bounded-traversal-model.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-006-002-bounded-traversal-implementation.md label=work/deliverables/E2/D-WP-E2-006-002-bounded-traversal-implementation.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-006-003-basic-ignore-rule-support.md label=work/deliverables/E2/D-WP-E2-006-003-basic-ignore-rule-support.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-006-004-bounded-traversal-output.md label=work/deliverables/E2/D-WP-E2-006-004-bounded-traversal-output.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-007-001-repository-graph-model.md label=work/deliverables/E2/D-WP-E2-007-001-repository-graph-model.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-007-002-graph-construction.md label=work/deliverables/E2/D-WP-E2-007-002-graph-construction.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-007-003-graph-metrics-output.md label=work/deliverables/E2/D-WP-E2-007-003-graph-metrics-output.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-007-004-graph-model-handoff.md label=work/deliverables/E2/D-WP-E2-007-004-graph-model-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-008-001-graph-render-format-type.md label=work/deliverables/E2/D-WP-E2-008-001-graph-render-format-type.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-008-002-text-json-graph-renderers.md label=work/deliverables/E2/D-WP-E2-008-002-text-json-graph-renderers.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-008-003-mermaid-dot-graph-renderers.md label=work/deliverables/E2/D-WP-E2-008-003-mermaid-dot-graph-renderers.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-008-004-graph-rendering-handoff.md label=work/deliverables/E2/D-WP-E2-008-004-graph-rendering-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-009-001-graph-cli-command.md label=work/deliverables/E2/D-WP-E2-009-001-graph-cli-command.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-009-002-graph-format-routing.md label=work/deliverables/E2/D-WP-E2-009-002-graph-format-routing.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-009-003-graph-smoke-verification.md label=work/deliverables/E2/D-WP-E2-009-003-graph-smoke-verification.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-009-004-graph-command-handoff.md label=work/deliverables/E2/D-WP-E2-009-004-graph-command-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-010-001-toolchain-detection-model.md label=work/deliverables/E2/D-WP-E2-010-001-toolchain-detection-model.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-010-002-common-toolchain-detection.md label=work/deliverables/E2/D-WP-E2-010-002-common-toolchain-detection.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-010-003-toolchain-inspect-output.md label=work/deliverables/E2/D-WP-E2-010-003-toolchain-inspect-output.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-010-004-toolchain-detection-handoff.md label=work/deliverables/E2/D-WP-E2-010-004-toolchain-detection-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-011-001-dependency-detection-model.md label=work/deliverables/E2/D-WP-E2-011-001-dependency-detection-model.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-011-002-dependency-signal-detection.md label=work/deliverables/E2/D-WP-E2-011-002-dependency-signal-detection.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-011-003-dependency-inspect-output.md label=work/deliverables/E2/D-WP-E2-011-003-dependency-inspect-output.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-011-004-dependency-detection-handoff.md label=work/deliverables/E2/D-WP-E2-011-004-dependency-detection-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-012-001-repository-policy-model.md label=work/deliverables/E2/D-WP-E2-012-001-repository-policy-model.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-012-002-advisory-policy-checks.md label=work/deliverables/E2/D-WP-E2-012-002-advisory-policy-checks.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-012-003-policy-inspect-output.md label=work/deliverables/E2/D-WP-E2-012-003-policy-inspect-output.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-012-004-policy-check-handoff.md label=work/deliverables/E2/D-WP-E2-012-004-policy-check-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-013-001-context-pack-model.md label=work/deliverables/E2/D-WP-E2-013-001-context-pack-model.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-013-002-context-pack-construction.md label=work/deliverables/E2/D-WP-E2-013-002-context-pack-construction.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-013-003-context-pack-rendering.md label=work/deliverables/E2/D-WP-E2-013-003-context-pack-rendering.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-013-004-context-pack-handoff.md label=work/deliverables/E2/D-WP-E2-013-004-context-pack-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-014-001-context-cli-command.md label=work/deliverables/E2/D-WP-E2-014-001-context-cli-command.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-014-002-context-format-routing.md label=work/deliverables/E2/D-WP-E2-014-002-context-format-routing.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-014-003-context-smoke-verification.md label=work/deliverables/E2/D-WP-E2-014-003-context-smoke-verification.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-014-004-context-command-handoff.md label=work/deliverables/E2/D-WP-E2-014-004-context-command-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-015-001-context-pack-export-model.md label=work/deliverables/E2/D-WP-E2-015-001-context-pack-export-model.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-015-002-context-pack-file-export.md label=work/deliverables/E2/D-WP-E2-015-002-context-pack-file-export.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-015-003-workspace-context-pack-export-helper.md label=work/deliverables/E2/D-WP-E2-015-003-workspace-context-pack-export-helper.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-015-004-context-export-handoff.md label=work/deliverables/E2/D-WP-E2-015-004-context-export-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-016-001-context-write-cli-flag.md label=work/deliverables/E2/D-WP-E2-016-001-context-write-cli-flag.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-016-002-context-write-validation.md label=work/deliverables/E2/D-WP-E2-016-002-context-write-validation.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-016-003-context-write-verification.md label=work/deliverables/E2/D-WP-E2-016-003-context-write-verification.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-016-004-context-write-handoff.md label=work/deliverables/E2/D-WP-E2-016-004-context-write-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-017-001-generated-context-ignore-rule.md label=work/deliverables/E2/D-WP-E2-017-001-generated-context-ignore-rule.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-017-002-generated-context-policy-diagnostics.md label=work/deliverables/E2/D-WP-E2-017-002-generated-context-policy-diagnostics.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-017-003-generated-context-policy-verification.md label=work/deliverables/E2/D-WP-E2-017-003-generated-context-policy-verification.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/D-WP-E2-017-004-generated-context-policy-handoff.md label=work/deliverables/E2/D-WP-E2-017-004-generated-context-policy-handoff.md kind=repository_entry depth=3
    - id=path:work/deliverables/E2/README.md label=work/deliverables/E2/README.md kind=repository_entry depth=3
    - id=path:work/epics label=work/epics kind=repository_entry depth=1
    - id=path:work/epics/E0-project-foundation.md label=work/epics/E0-project-foundation.md kind=repository_entry depth=2
    - id=path:work/epics/E1-runtime-foundation.md label=work/epics/E1-runtime-foundation.md kind=repository_entry depth=2
    - id=path:work/epics/E10-public-prerelease-hardening.md label=work/epics/E10-public-prerelease-hardening.md kind=repository_entry depth=2
    - id=path:work/epics/E2-repository-intelligence-foundation.md label=work/epics/E2-repository-intelligence-foundation.md kind=repository_entry depth=2
    - id=path:work/epics/E9-post-mvp-candidate-stabilization.md label=work/epics/E9-post-mvp-candidate-stabilization.md kind=repository_entry depth=2
    - id=path:work/epics/README.md label=work/epics/README.md kind=repository_entry depth=2
    - id=path:work/packets label=work/packets kind=repository_entry depth=1
    - id=path:work/packets/E0 label=work/packets/E0 kind=repository_entry depth=2
    - id=path:work/packets/E0/README.md label=work/packets/E0/README.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-001-establish-repository-foundation.md label=work/packets/E0/WP-E0-001-establish-repository-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-002-establish-documentation-architecture.md label=work/packets/E0/WP-E0-002-establish-documentation-architecture.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-003-establish-context-bridge-foundation.md label=work/packets/E0/WP-E0-003-establish-context-bridge-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-004-establish-workflow-standards.md label=work/packets/E0/WP-E0-004-establish-workflow-standards.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-005-establish-verification-baseline.md label=work/packets/E0/WP-E0-005-establish-verification-baseline.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-006-establish-work-packet-records.md label=work/packets/E0/WP-E0-006-establish-work-packet-records.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-007-establish-adr-verification.md label=work/packets/E0/WP-E0-007-establish-adr-verification.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-008-establish-epic-record-verification.md label=work/packets/E0/WP-E0-008-establish-epic-record-verification.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-009-establish-task-record-foundation.md label=work/packets/E0/WP-E0-009-establish-task-record-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-010-establish-deliverable-record-foundation.md label=work/packets/E0/WP-E0-010-establish-deliverable-record-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E0/WP-E0-011-close-e0-and-prepare-e1-handoff.md label=work/packets/E0/WP-E0-011-close-e0-and-prepare-e1-handoff.md kind=repository_entry depth=3
    - id=path:work/packets/E1 label=work/packets/E1 kind=repository_entry depth=2
    - id=path:work/packets/E1/README.md label=work/packets/E1/README.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-001-establish-rust-workspace-runtime-foundation.md label=work/packets/E1/WP-E1-001-establish-rust-workspace-runtime-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-002-establish-core-diagnostics-foundation.md label=work/packets/E1/WP-E1-002-establish-core-diagnostics-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-003-establish-core-error-foundation.md label=work/packets/E1/WP-E1-003-establish-core-error-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-004-establish-workspace-context-foundation.md label=work/packets/E1/WP-E1-004-establish-workspace-context-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-005-establish-manifest-model-foundation.md label=work/packets/E1/WP-E1-005-establish-manifest-model-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-006-establish-manifest-loading-foundation.md label=work/packets/E1/WP-E1-006-establish-manifest-loading-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-007-establish-cli-info-command-foundation.md label=work/packets/E1/WP-E1-007-establish-cli-info-command-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-008-establish-cli-check-command-foundation.md label=work/packets/E1/WP-E1-008-establish-cli-check-command-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-009-establish-repository-contract-check-foundation.md label=work/packets/E1/WP-E1-009-establish-repository-contract-check-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-010-establish-runtime-output-formatting-foundation.md label=work/packets/E1/WP-E1-010-establish-runtime-output-formatting-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-011-establish-cli-output-format-argument-foundation.md label=work/packets/E1/WP-E1-011-establish-cli-output-format-argument-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-012-establish-json-output-formatting-foundation.md label=work/packets/E1/WP-E1-012-establish-json-output-formatting-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E1/WP-E1-013-close-e1-and-prepare-e2-handoff.md label=work/packets/E1/WP-E1-013-close-e1-and-prepare-e2-handoff.md kind=repository_entry depth=3
    - id=path:work/packets/E2 label=work/packets/E2 kind=repository_entry depth=2
    - id=path:work/packets/E2/README.md label=work/packets/E2/README.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-001-establish-repository-inspection-foundation.md label=work/packets/E2/WP-E2-001-establish-repository-inspection-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-002-establish-monad-inspect-command-foundation.md label=work/packets/E2/WP-E2-002-establish-monad-inspect-command-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-003-enrich-repository-inspection-classification.md label=work/packets/E2/WP-E2-003-enrich-repository-inspection-classification.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-004-add-repository-inspection-summary-metrics.md label=work/packets/E2/WP-E2-004-add-repository-inspection-summary-metrics.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-005-add-recursive-traversal-plan-and-guardrails.md label=work/packets/E2/WP-E2-005-add-recursive-traversal-plan-and-guardrails.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-006-implement-bounded-repository-traversal-foundation.md label=work/packets/E2/WP-E2-006-implement-bounded-repository-traversal-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-007-add-repository-graph-model-foundation.md label=work/packets/E2/WP-E2-007-add-repository-graph-model-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-008-add-graph-rendering-format-foundation.md label=work/packets/E2/WP-E2-008-add-graph-rendering-format-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-009-add-monad-graph-command-foundation.md label=work/packets/E2/WP-E2-009-add-monad-graph-command-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-010-add-toolchain-detection-foundation.md label=work/packets/E2/WP-E2-010-add-toolchain-detection-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-011-add-dependency-signal-detection-foundation.md label=work/packets/E2/WP-E2-011-add-dependency-signal-detection-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-012-add-repository-intelligence-policy-check-foundation.md label=work/packets/E2/WP-E2-012-add-repository-intelligence-policy-check-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-013-add-repository-context-pack-foundation.md label=work/packets/E2/WP-E2-013-add-repository-context-pack-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-014-add-monad-context-command-foundation.md label=work/packets/E2/WP-E2-014-add-monad-context-command-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-015-add-repository-context-pack-export-foundation.md label=work/packets/E2/WP-E2-015-add-repository-context-pack-export-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-016-add-monad-context-write-foundation.md label=work/packets/E2/WP-E2-016-add-monad-context-write-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/E2/WP-E2-017-add-generated-context-artifact-policy-foundation.md label=work/packets/E2/WP-E2-017-add-generated-context-artifact-policy-foundation.md kind=repository_entry depth=3
    - id=path:work/packets/README.md label=work/packets/README.md kind=repository_entry depth=2
    - id=path:work/records label=work/records kind=repository_entry depth=1
    - id=path:work/records/README.md label=work/records/README.md kind=repository_entry depth=2
    - id=path:work/tasks label=work/tasks kind=repository_entry depth=1
    - id=path:work/tasks/E0 label=work/tasks/E0 kind=repository_entry depth=2
    - id=path:work/tasks/E0/README.md label=work/tasks/E0/README.md kind=repository_entry depth=3
    - id=path:work/tasks/E0/T-WP-E0-009-001-create-task-record-directory-and-index.md label=work/tasks/E0/T-WP-E0-009-001-create-task-record-directory-and-index.md kind=repository_entry depth=3
    - id=path:work/tasks/E0/T-WP-E0-009-002-add-task-record-verification.md label=work/tasks/E0/T-WP-E0-009-002-add-task-record-verification.md kind=repository_entry depth=3
    - id=path:work/tasks/E0/T-WP-E0-009-003-update-e0-planning-and-verification-records.md label=work/tasks/E0/T-WP-E0-009-003-update-e0-planning-and-verification-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E0/T-WP-E0-010-001-create-deliverable-record-directory-and-index.md label=work/tasks/E0/T-WP-E0-010-001-create-deliverable-record-directory-and-index.md kind=repository_entry depth=3
    - id=path:work/tasks/E0/T-WP-E0-010-002-add-deliverable-record-verification.md label=work/tasks/E0/T-WP-E0-010-002-add-deliverable-record-verification.md kind=repository_entry depth=3
    - id=path:work/tasks/E0/T-WP-E0-010-003-update-e0-planning-and-verification-records.md label=work/tasks/E0/T-WP-E0-010-003-update-e0-planning-and-verification-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E0/T-WP-E0-011-001-close-e0-records.md label=work/tasks/E0/T-WP-E0-011-001-close-e0-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E0/T-WP-E0-011-002-update-context-handoff.md label=work/tasks/E0/T-WP-E0-011-002-update-context-handoff.md kind=repository_entry depth=3
    - id=path:work/tasks/E0/T-WP-E0-011-003-create-e1-starting-point.md label=work/tasks/E0/T-WP-E0-011-003-create-e1-starting-point.md kind=repository_entry depth=3
    - id=path:work/tasks/E1 label=work/tasks/E1 kind=repository_entry depth=2
    - id=path:work/tasks/E1/README.md label=work/tasks/E1/README.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-001-001-create-rust-workspace-crates.md label=work/tasks/E1/T-WP-E1-001-001-create-rust-workspace-crates.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-001-002-add-minimal-core-runtime-identity.md label=work/tasks/E1/T-WP-E1-001-002-add-minimal-core-runtime-identity.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-001-003-add-thin-cli-entrypoint.md label=work/tasks/E1/T-WP-E1-001-003-add-thin-cli-entrypoint.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-001-004-add-rust-verification-to-baseline.md label=work/tasks/E1/T-WP-E1-001-004-add-rust-verification-to-baseline.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-002-001-add-diagnostics-module.md label=work/tasks/E1/T-WP-E1-002-001-add-diagnostics-module.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-002-002-export-diagnostics-from-core-runtime.md label=work/tasks/E1/T-WP-E1-002-002-export-diagnostics-from-core-runtime.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-002-003-update-e1-records-and-context.md label=work/tasks/E1/T-WP-E1-002-003-update-e1-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-003-001-add-core-error-module.md label=work/tasks/E1/T-WP-E1-003-001-add-core-error-module.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-003-002-export-core-error-model.md label=work/tasks/E1/T-WP-E1-003-002-export-core-error-model.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-003-003-update-e1-records-and-context.md label=work/tasks/E1/T-WP-E1-003-003-update-e1-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-004-001-add-workspace-context-module.md label=work/tasks/E1/T-WP-E1-004-001-add-workspace-context-module.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-004-002-export-workspace-context-from-core-runtime.md label=work/tasks/E1/T-WP-E1-004-002-export-workspace-context-from-core-runtime.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-004-003-update-e1-records-and-context.md label=work/tasks/E1/T-WP-E1-004-003-update-e1-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-005-001-add-root-monad-manifest.md label=work/tasks/E1/T-WP-E1-005-001-add-root-monad-manifest.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-005-002-add-manifest-model-module.md label=work/tasks/E1/T-WP-E1-005-002-add-manifest-model-module.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-005-003-export-manifest-model-from-core-runtime.md label=work/tasks/E1/T-WP-E1-005-003-export-manifest-model-from-core-runtime.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-005-004-update-e1-records-and-context.md label=work/tasks/E1/T-WP-E1-005-004-update-e1-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-006-001-add-manifest-parsing-dependencies.md label=work/tasks/E1/T-WP-E1-006-001-add-manifest-parsing-dependencies.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-006-002-add-manifest-loading.md label=work/tasks/E1/T-WP-E1-006-002-add-manifest-loading.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-006-003-update-e1-records-and-context.md label=work/tasks/E1/T-WP-E1-006-003-update-e1-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-007-001-add-cli-command-parser.md label=work/tasks/E1/T-WP-E1-007-001-add-cli-command-parser.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-007-002-add-cli-info-rendering.md label=work/tasks/E1/T-WP-E1-007-002-add-cli-info-rendering.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-007-003-update-e1-records-and-context.md label=work/tasks/E1/T-WP-E1-007-003-update-e1-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-008-001-add-core-workspace-checks.md label=work/tasks/E1/T-WP-E1-008-001-add-core-workspace-checks.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-008-002-add-cli-check-command.md label=work/tasks/E1/T-WP-E1-008-002-add-cli-check-command.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-008-003-update-e1-records-and-context.md label=work/tasks/E1/T-WP-E1-008-003-update-e1-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-009-001-add-repository-contract-module.md label=work/tasks/E1/T-WP-E1-009-001-add-repository-contract-module.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-009-002-integrate-contract-checks-with-monad-check.md label=work/tasks/E1/T-WP-E1-009-002-integrate-contract-checks-with-monad-check.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-009-003-update-e1-records-and-context.md label=work/tasks/E1/T-WP-E1-009-003-update-e1-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-010-001-add-output-formatting-module.md label=work/tasks/E1/T-WP-E1-010-001-add-output-formatting-module.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-010-002-use-output-formatting-in-cli.md label=work/tasks/E1/T-WP-E1-010-002-use-output-formatting-in-cli.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-010-003-update-e1-records-and-context.md label=work/tasks/E1/T-WP-E1-010-003-update-e1-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-011-001-add-cli-output-format-parsing.md label=work/tasks/E1/T-WP-E1-011-001-add-cli-output-format-parsing.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-011-002-wire-output-format-into-cli-commands.md label=work/tasks/E1/T-WP-E1-011-002-wire-output-format-into-cli-commands.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-011-003-update-e1-records-and-context.md label=work/tasks/E1/T-WP-E1-011-003-update-e1-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-013-001-close-e1-records.md label=work/tasks/E1/T-WP-E1-013-001-close-e1-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-013-002-update-runtime-context-handoff.md label=work/tasks/E1/T-WP-E1-013-002-update-runtime-context-handoff.md kind=repository_entry depth=3
    - id=path:work/tasks/E1/T-WP-E1-013-003-create-e2-starting-point.md label=work/tasks/E1/T-WP-E1-013-003-create-e2-starting-point.md kind=repository_entry depth=3
    - id=path:work/tasks/E2 label=work/tasks/E2 kind=repository_entry depth=2
    - id=path:work/tasks/E2/README.md label=work/tasks/E2/README.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-001-001-add-repository-inspection-module.md label=work/tasks/E2/T-WP-E2-001-001-add-repository-inspection-module.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-001-002-export-repository-inspection-types.md label=work/tasks/E2/T-WP-E2-001-002-export-repository-inspection-types.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-001-003-integrate-inspection-with-workspace-checks.md label=work/tasks/E2/T-WP-E2-001-003-integrate-inspection-with-workspace-checks.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-001-004-update-e2-records-and-context.md label=work/tasks/E2/T-WP-E2-001-004-update-e2-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-002-001-add-inspection-summary-rendering.md label=work/tasks/E2/T-WP-E2-002-001-add-inspection-summary-rendering.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-002-002-add-cli-inspect-command.md label=work/tasks/E2/T-WP-E2-002-002-add-cli-inspect-command.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-002-003-add-inspect-smoke-verification.md label=work/tasks/E2/T-WP-E2-002-003-add-inspect-smoke-verification.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-002-004-update-e2-records-and-context.md label=work/tasks/E2/T-WP-E2-002-004-update-e2-records-and-context.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-003-001-expand-repository-entry-roles.md label=work/tasks/E2/T-WP-E2-003-001-expand-repository-entry-roles.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-003-002-expand-classification-rules.md label=work/tasks/E2/T-WP-E2-003-002-expand-classification-rules.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-003-003-harden-traversal-policy.md label=work/tasks/E2/T-WP-E2-003-003-harden-traversal-policy.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-003-004-update-e2-classification-records.md label=work/tasks/E2/T-WP-E2-003-004-update-e2-classification-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-004-001-add-repository-entry-category.md label=work/tasks/E2/T-WP-E2-004-001-add-repository-entry-category.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-004-002-add-category-metrics-to-summary.md label=work/tasks/E2/T-WP-E2-004-002-add-category-metrics-to-summary.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-004-003-render-summary-metrics.md label=work/tasks/E2/T-WP-E2-004-003-render-summary-metrics.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-004-004-update-e2-metrics-records.md label=work/tasks/E2/T-WP-E2-004-004-update-e2-metrics-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-005-001-add-traversal-planning-types.md label=work/tasks/E2/T-WP-E2-005-001-add-traversal-planning-types.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-005-002-add-conservative-traversal-guardrails.md label=work/tasks/E2/T-WP-E2-005-002-add-conservative-traversal-guardrails.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-005-003-render-traversal-plan-in-inspect-summary.md label=work/tasks/E2/T-WP-E2-005-003-render-traversal-plan-in-inspect-summary.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-005-004-update-e2-traversal-records.md label=work/tasks/E2/T-WP-E2-005-004-update-e2-traversal-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-006-001-add-bounded-traversal-model.md label=work/tasks/E2/T-WP-E2-006-001-add-bounded-traversal-model.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-006-002-implement-bounded-traversal.md label=work/tasks/E2/T-WP-E2-006-002-implement-bounded-traversal.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-006-003-add-basic-ignore-rule-support.md label=work/tasks/E2/T-WP-E2-006-003-add-basic-ignore-rule-support.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-006-004-render-bounded-traversal-metrics.md label=work/tasks/E2/T-WP-E2-006-004-render-bounded-traversal-metrics.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-007-001-add-repository-graph-model.md label=work/tasks/E2/T-WP-E2-007-001-add-repository-graph-model.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-007-002-build-graph-from-bounded-traversal.md label=work/tasks/E2/T-WP-E2-007-002-build-graph-from-bounded-traversal.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-007-003-render-graph-metrics-in-inspect-summary.md label=work/tasks/E2/T-WP-E2-007-003-render-graph-metrics-in-inspect-summary.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-007-004-update-e2-graph-records.md label=work/tasks/E2/T-WP-E2-007-004-update-e2-graph-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-008-001-add-graph-render-format-type.md label=work/tasks/E2/T-WP-E2-008-001-add-graph-render-format-type.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-008-002-add-text-and-json-graph-renderers.md label=work/tasks/E2/T-WP-E2-008-002-add-text-and-json-graph-renderers.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-008-003-add-mermaid-and-dot-graph-renderers.md label=work/tasks/E2/T-WP-E2-008-003-add-mermaid-and-dot-graph-renderers.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-008-004-update-e2-graph-rendering-records.md label=work/tasks/E2/T-WP-E2-008-004-update-e2-graph-rendering-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-009-001-add-graph-cli-command.md label=work/tasks/E2/T-WP-E2-009-001-add-graph-cli-command.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-009-002-add-graph-format-routing.md label=work/tasks/E2/T-WP-E2-009-002-add-graph-format-routing.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-009-003-add-graph-smoke-verification.md label=work/tasks/E2/T-WP-E2-009-003-add-graph-smoke-verification.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-009-004-update-e2-graph-command-records.md label=work/tasks/E2/T-WP-E2-009-004-update-e2-graph-command-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-010-001-add-toolchain-detection-model.md label=work/tasks/E2/T-WP-E2-010-001-add-toolchain-detection-model.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-010-002-detect-common-toolchains.md label=work/tasks/E2/T-WP-E2-010-002-detect-common-toolchains.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-010-003-render-toolchain-metrics-in-inspect.md label=work/tasks/E2/T-WP-E2-010-003-render-toolchain-metrics-in-inspect.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-010-004-update-e2-toolchain-records.md label=work/tasks/E2/T-WP-E2-010-004-update-e2-toolchain-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-011-001-add-dependency-detection-model.md label=work/tasks/E2/T-WP-E2-011-001-add-dependency-detection-model.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-011-002-detect-dependency-signals.md label=work/tasks/E2/T-WP-E2-011-002-detect-dependency-signals.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-011-003-render-dependency-metrics-in-inspect.md label=work/tasks/E2/T-WP-E2-011-003-render-dependency-metrics-in-inspect.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-011-004-update-e2-dependency-records.md label=work/tasks/E2/T-WP-E2-011-004-update-e2-dependency-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-012-001-add-repository-policy-model.md label=work/tasks/E2/T-WP-E2-012-001-add-repository-policy-model.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-012-002-add-advisory-policy-checks.md label=work/tasks/E2/T-WP-E2-012-002-add-advisory-policy-checks.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-012-003-render-policy-metrics-in-inspect.md label=work/tasks/E2/T-WP-E2-012-003-render-policy-metrics-in-inspect.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-012-004-update-e2-policy-records.md label=work/tasks/E2/T-WP-E2-012-004-update-e2-policy-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-013-001-add-context-pack-model.md label=work/tasks/E2/T-WP-E2-013-001-add-context-pack-model.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-013-002-build-context-pack-from-repository-intelligence.md label=work/tasks/E2/T-WP-E2-013-002-build-context-pack-from-repository-intelligence.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-013-003-render-context-pack.md label=work/tasks/E2/T-WP-E2-013-003-render-context-pack.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-013-004-update-e2-context-pack-records.md label=work/tasks/E2/T-WP-E2-013-004-update-e2-context-pack-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-014-001-add-context-cli-command.md label=work/tasks/E2/T-WP-E2-014-001-add-context-cli-command.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-014-002-add-context-format-routing.md label=work/tasks/E2/T-WP-E2-014-002-add-context-format-routing.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-014-003-add-context-smoke-verification.md label=work/tasks/E2/T-WP-E2-014-003-add-context-smoke-verification.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-014-004-update-e2-context-command-records.md label=work/tasks/E2/T-WP-E2-014-004-update-e2-context-command-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-015-001-add-context-pack-export-model.md label=work/tasks/E2/T-WP-E2-015-001-add-context-pack-export-model.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-015-002-add-deterministic-context-pack-export.md label=work/tasks/E2/T-WP-E2-015-002-add-deterministic-context-pack-export.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-015-003-add-workspace-export-helper.md label=work/tasks/E2/T-WP-E2-015-003-add-workspace-export-helper.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-015-004-update-e2-context-export-records.md label=work/tasks/E2/T-WP-E2-015-004-update-e2-context-export-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-016-001-add-context-write-cli-flag.md label=work/tasks/E2/T-WP-E2-016-001-add-context-write-cli-flag.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-016-002-add-write-flag-validation.md label=work/tasks/E2/T-WP-E2-016-002-add-write-flag-validation.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-016-003-add-context-write-smoke-verification.md label=work/tasks/E2/T-WP-E2-016-003-add-context-write-smoke-verification.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-016-004-update-e2-context-write-records.md label=work/tasks/E2/T-WP-E2-016-004-update-e2-context-write-records.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-017-001-add-generated-context-ignore-rule.md label=work/tasks/E2/T-WP-E2-017-001-add-generated-context-ignore-rule.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-017-002-add-generated-context-policy-diagnostics.md label=work/tasks/E2/T-WP-E2-017-002-add-generated-context-policy-diagnostics.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-017-003-add-generated-context-policy-verification.md label=work/tasks/E2/T-WP-E2-017-003-add-generated-context-policy-verification.md kind=repository_entry depth=3
    - id=path:work/tasks/E2/T-WP-E2-017-004-update-e2-generated-context-policy-records.md label=work/tasks/E2/T-WP-E2-017-004-update-e2-generated-context-policy-records.md kind=repository_entry depth=3
    - id=path:work/tasks/README.md label=work/tasks/README.md kind=repository_entry depth=2
    - id=root label=. kind=workspace_root depth=0
  edges:
    - path:.github -[contains]-> path:.github/FUNDING.yml
    - path:.github -[contains]-> path:.github/ISSUE_TEMPLATE
    - path:.github -[contains]-> path:.github/dependabot.yml
    - path:.github -[contains]-> path:.github/pull_request_template.md
    - path:.github -[contains]-> path:.github/workflows
    - path:.github/ISSUE_TEMPLATE -[contains]-> path:.github/ISSUE_TEMPLATE/adr-candidate.yml
    - path:.github/ISSUE_TEMPLATE -[contains]-> path:.github/ISSUE_TEMPLATE/bug.yml
    - path:.github/ISSUE_TEMPLATE -[contains]-> path:.github/ISSUE_TEMPLATE/bug_report.md
    - path:.github/ISSUE_TEMPLATE -[contains]-> path:.github/ISSUE_TEMPLATE/config.yml
    - path:.github/ISSUE_TEMPLATE -[contains]-> path:.github/ISSUE_TEMPLATE/epic.yml
    - path:.github/ISSUE_TEMPLATE -[contains]-> path:.github/ISSUE_TEMPLATE/feature_request.md
    - path:.github/ISSUE_TEMPLATE -[contains]-> path:.github/ISSUE_TEMPLATE/research.yml
    - path:.github/ISSUE_TEMPLATE -[contains]-> path:.github/ISSUE_TEMPLATE/task.yml
    - path:.github/ISSUE_TEMPLATE -[contains]-> path:.github/ISSUE_TEMPLATE/work-packet.yml
    - path:.github/workflows -[contains]-> path:.github/workflows/build-binary.yml
    - path:.monad -[contains]-> path:.monad/README.md
    - path:.monad -[contains]-> path:.monad/cache
    - path:.monad -[contains]-> path:.monad/context
    - path:.monad -[contains]-> path:.monad/local
    - path:.monad -[contains]-> path:.monad/reports
    - path:.monad -[contains]-> path:.monad/tmp
    - path:.monad/context -[contains]-> path:.monad/context/2026-05-23-session-001.md
    - path:.monad/context -[contains]-> path:.monad/context/README.md
    - path:.monad/context -[contains]-> path:.monad/context/current-state.md
    - path:.monad/context -[contains]-> path:.monad/context/decision-log.md
    - path:.monad/context -[contains]-> path:.monad/context/decision-records
    - path:.monad/context -[contains]-> path:.monad/context/latest-context-pack.md
    - path:.monad/context -[contains]-> path:.monad/context/latest-handoff.md
    - path:.monad/context -[contains]-> path:.monad/context/session-chronicles
    - path:.monad/context -[contains]-> path:.monad/context/work-packet-handoffs
    - path:.monad/context/decision-records -[contains]-> path:.monad/context/decision-records/README.md
    - path:.monad/context/session-chronicles -[contains]-> path:.monad/context/session-chronicles/README.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/README.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-001.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-002.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-003.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-004.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-005.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-006.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-007.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-008.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-009.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-010.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-011.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E1-013.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-001.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-002.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-003.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-004.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-005.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-006.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-007.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-008.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-009.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-010.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-011.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-012.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-013.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-014.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-015.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-016.md
    - path:.monad/context/work-packet-handoffs -[contains]-> path:.monad/context/work-packet-handoffs/WP-E2-017.md
    - path:assets -[contains]-> path:assets/.keep
    - path:assets -[contains]-> path:assets/softwarefordevelopers-preview.png
    - path:crates -[contains]-> path:crates/monad-cli
    - path:crates -[contains]-> path:crates/monad-core
    - path:crates -[contains]-> path:crates/monad-mcp
    - path:crates/monad-cli -[contains]-> path:crates/monad-cli/Cargo.toml
    - path:crates/monad-cli -[contains]-> path:crates/monad-cli/src
    - path:crates/monad-cli -[contains]-> path:crates/monad-cli/tests
    - path:crates/monad-cli/src -[contains]-> path:crates/monad-cli/src/main.rs
    - path:crates/monad-cli/tests -[contains]-> path:crates/monad-cli/tests/cli_smoke.rs
    - path:crates/monad-core -[contains]-> path:crates/monad-core/Cargo.toml
    - path:crates/monad-core -[contains]-> path:crates/monad-core/src
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/agents
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/agents.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/checks
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/checks.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/context
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/dependency_detection.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/diagnostics.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/error.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/evolution
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/evolution.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/exec
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/exec.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/file_ops
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/file_ops.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/git
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/git.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/lib.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/manifest.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/output.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/policy
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/policy.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/repo_contract.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/repository_context_pack.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/repository_graph.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/repository_inspection.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/repository_policy.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/templates
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/templates.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/toolchain_detection.rs
    - path:crates/monad-core/src -[contains]-> path:crates/monad-core/src/workspace.rs
    - path:crates/monad-mcp -[contains]-> path:crates/monad-mcp/Cargo.toml
    - path:crates/monad-mcp -[contains]-> path:crates/monad-mcp/src
    - path:crates/monad-mcp/src -[contains]-> path:crates/monad-mcp/src/lib.rs
    - path:docs -[contains]-> path:docs/00-meta
    - path:docs -[contains]-> path:docs/01-project
    - path:docs -[contains]-> path:docs/02-product
    - path:docs -[contains]-> path:docs/03-requirements
    - path:docs -[contains]-> path:docs/04-domain
    - path:docs -[contains]-> path:docs/05-architecture
    - path:docs -[contains]-> path:docs/06-adrs
    - path:docs -[contains]-> path:docs/07-workflow
    - path:docs -[contains]-> path:docs/08-context
    - path:docs -[contains]-> path:docs/09-ai
    - path:docs -[contains]-> path:docs/10-engineering
    - path:docs -[contains]-> path:docs/11-security
    - path:docs -[contains]-> path:docs/12-verification
    - path:docs -[contains]-> path:docs/13-operations
    - path:docs -[contains]-> path:docs/14-integrations
    - path:docs -[contains]-> path:docs/15-business
    - path:docs -[contains]-> path:docs/16-reference
    - path:docs -[contains]-> path:docs/README.md
    - path:docs -[contains]-> path:docs/ai
    - path:docs -[contains]-> path:docs/architecture
    - path:docs -[contains]-> path:docs/context
    - path:docs -[contains]-> path:docs/development
    - path:docs -[contains]-> path:docs/project
    - path:docs -[contains]-> path:docs/release
    - path:docs -[contains]-> path:docs/repository
    - path:docs -[contains]-> path:docs/security
    - path:docs -[contains]-> path:docs/wiki
    - path:docs -[contains]-> path:docs/workflow
    - path:docs/00-meta -[contains]-> path:docs/00-meta/DOCUMENTATION-MAP.md
    - path:docs/00-meta -[contains]-> path:docs/00-meta/DOCUMENTATION-STANDARD.md
    - path:docs/00-meta -[contains]-> path:docs/00-meta/FRONTMATTER-STANDARD.md
    - path:docs/00-meta -[contains]-> path:docs/00-meta/GLOSSARY.md
    - path:docs/00-meta -[contains]-> path:docs/00-meta/IDEA.md
    - path:docs/00-meta -[contains]-> path:docs/00-meta/NAMING-STANDARD.md
    - path:docs/00-meta -[contains]-> path:docs/00-meta/README.md
    - path:docs/00-meta -[contains]-> path:docs/00-meta/STATUS-STANDARD.md
    - path:docs/01-project -[contains]-> path:docs/01-project/00-vision
    - path:docs/01-project -[contains]-> path:docs/01-project/01-charter
    - path:docs/01-project -[contains]-> path:docs/01-project/02-strategy
    - path:docs/01-project -[contains]-> path:docs/01-project/03-roadmap
    - path:docs/01-project -[contains]-> path:docs/01-project/04-glossary
    - path:docs/01-project -[contains]-> path:docs/01-project/README.md
    - path:docs/01-project/00-vision -[contains]-> path:docs/01-project/00-vision/HOLY-GRAIL-VISION.md
    - path:docs/01-project/00-vision -[contains]-> path:docs/01-project/00-vision/PRODUCT-THESIS.md
    - path:docs/01-project/00-vision -[contains]-> path:docs/01-project/00-vision/PRODUCT-VISION.md
    - path:docs/01-project/00-vision -[contains]-> path:docs/01-project/00-vision/README.md
    - path:docs/01-project/01-charter -[contains]-> path:docs/01-project/01-charter/PRODUCT-CHARTER.md
    - path:docs/01-project/01-charter -[contains]-> path:docs/01-project/01-charter/PROJECT-CHARTER.md
    - path:docs/01-project/01-charter -[contains]-> path:docs/01-project/01-charter/README.md
    - path:docs/01-project/02-strategy -[contains]-> path:docs/01-project/02-strategy/GO-TO-MARKET-STRATEGY.md
    - path:docs/01-project/02-strategy -[contains]-> path:docs/01-project/02-strategy/MONETIZATION-STRATEGY.md
    - path:docs/01-project/02-strategy -[contains]-> path:docs/01-project/02-strategy/OPEN-CORE-STRATEGY.md
    - path:docs/01-project/02-strategy -[contains]-> path:docs/01-project/02-strategy/PRODUCT-STRATEGY.md
    - path:docs/01-project/02-strategy -[contains]-> path:docs/01-project/02-strategy/README.md
    - path:docs/01-project/03-roadmap -[contains]-> path:docs/01-project/03-roadmap/MVP-ROADMAP.md
    - path:docs/01-project/03-roadmap -[contains]-> path:docs/01-project/03-roadmap/POST-MVP-ROADMAP.md
    - path:docs/01-project/03-roadmap -[contains]-> path:docs/01-project/03-roadmap/README.md
    - path:docs/01-project/03-roadmap -[contains]-> path:docs/01-project/03-roadmap/RELEASE-PLAN.md
    - path:docs/01-project/03-roadmap -[contains]-> path:docs/01-project/03-roadmap/ROADMAP.md
    - path:docs/01-project/04-glossary -[contains]-> path:docs/01-project/04-glossary/PRODUCT-GLOSSARY.md
    - path:docs/01-project/04-glossary -[contains]-> path:docs/01-project/04-glossary/README.md
    - path:docs/01-project/04-glossary -[contains]-> path:docs/01-project/04-glossary/UBIQUITOUS-LANGUAGE.md
    - path:docs/02-product -[contains]-> path:docs/02-product/COMPETITIVE-LANDSCAPE.md
    - path:docs/02-product -[contains]-> path:docs/02-product/MVP-SCOPE.md
    - path:docs/02-product -[contains]-> path:docs/02-product/NON-GOALS.md
    - path:docs/02-product -[contains]-> path:docs/02-product/POSITIONING.md
    - path:docs/02-product -[contains]-> path:docs/02-product/PROBLEM-STATEMENT.md
    - path:docs/02-product -[contains]-> path:docs/02-product/README.md
    - path:docs/02-product -[contains]-> path:docs/02-product/SUCCESS-METRICS.md
    - path:docs/02-product -[contains]-> path:docs/02-product/TARGET-USERS.md
    - path:docs/02-product -[contains]-> path:docs/02-product/USE-CASES.md
    - path:docs/02-product -[contains]-> path:docs/02-product/USER-JOURNEYS.md
    - path:docs/02-product -[contains]-> path:docs/02-product/USER-PERSONAS.md
    - path:docs/02-product -[contains]-> path:docs/02-product/VALUE-PROPOSITION.md
    - path:docs/03-requirements -[contains]-> path:docs/03-requirements/ACCEPTANCE-CRITERIA-STANDARD.md
    - path:docs/03-requirements -[contains]-> path:docs/03-requirements/FUNCTIONAL-REQUIREMENTS.md
    - path:docs/03-requirements -[contains]-> path:docs/03-requirements/FUTURE-REQUIREMENTS.md
    - path:docs/03-requirements -[contains]-> path:docs/03-requirements/MVP-REQUIREMENTS.md
    - path:docs/03-requirements -[contains]-> path:docs/03-requirements/NONFUNCTIONAL-REQUIREMENTS.md
    - path:docs/03-requirements -[contains]-> path:docs/03-requirements/README.md
    - path:docs/03-requirements -[contains]-> path:docs/03-requirements/REQUIREMENTS-TRACEABILITY-MATRIX.md
    - path:docs/03-requirements -[contains]-> path:docs/03-requirements/SYSTEM-QUALITIES.md
    - path:docs/04-domain -[contains]-> path:docs/04-domain/BOUNDED-CONTEXTS.md
    - path:docs/04-domain -[contains]-> path:docs/04-domain/CONCEPTUAL-MODEL.md
    - path:docs/04-domain -[contains]-> path:docs/04-domain/DOMAIN-EVENTS.md
    - path:docs/04-domain -[contains]-> path:docs/04-domain/DOMAIN-INVARIANTS.md
    - path:docs/04-domain -[contains]-> path:docs/04-domain/DOMAIN-MODEL.md
    - path:docs/04-domain -[contains]-> path:docs/04-domain/README.md
    - path:docs/04-domain -[contains]-> path:docs/04-domain/UBIQUITOUS-LANGUAGE.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/AGENT-SUPERVISION-ARCHITECTURE.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/CONTEXT-BRIDGE-ARCHITECTURE.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/CONTROL-FLOW.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/DATA-FLOW.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/EVOLUTION-ENGINE-ARCHITECTURE.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/EXTENSION-MODEL.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/MCP-INTEGRATION-STRATEGY.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/MODULE-BOUNDARIES.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/PLUGIN-MODEL.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/PROJECT-GRAPH-MODEL.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/PROVIDER-MODEL.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/README.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/RUNTIME-ARCHITECTURE.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/SYSTEM-OVERVIEW.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/VERIFICATION-ARCHITECTURE.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/WORKSPACE-MODEL.md
    - path:docs/05-architecture -[contains]-> path:docs/05-architecture/WORKTREE-SAFETY-STRATEGY.md
    - path:docs/06-adrs -[contains]-> path:docs/06-adrs/ADR-0000-template.md
    - path:docs/06-adrs -[contains]-> path:docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md
    - path:docs/06-adrs -[contains]-> path:docs/06-adrs/ADR-0002-use-monad-as-unified-product-name.md
    - path:docs/06-adrs -[contains]-> path:docs/06-adrs/ADR-0003-use-repo-native-context-as-source-of-truth.md
    - path:docs/06-adrs -[contains]-> path:docs/06-adrs/ADR-0004-use-work-packets-as-primary-delivery-unit.md
    - path:docs/06-adrs -[contains]-> path:docs/06-adrs/ADR-0005-use-multi-crate-rust-workspace.md
    - path:docs/06-adrs -[contains]-> path:docs/06-adrs/ADR-0006-keep-cli-thin-and-core-durable.md
    - path:docs/06-adrs -[contains]-> path:docs/06-adrs/ADR-0007-use-supervised-autonomy-for-agent-workflows.md
    - path:docs/06-adrs -[contains]-> path:docs/06-adrs/ADR-0008-coordinate-native-tools-rather-than-replace-them.md
    - path:docs/06-adrs -[contains]-> path:docs/06-adrs/README.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/BRANCHING-STANDARD.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/COMMIT-STANDARD.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/CONTEXT-UPDATE-STANDARD.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/DEFINITION-OF-DONE.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/DEFINITION-OF-READY.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/DELIVERABLE-STANDARD.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/EPIC-STANDARD.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/OPERATING-MODEL.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/README.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/RELEASE-STANDARD.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/REVIEW-STANDARD.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/TASK-STANDARD.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/VERIFICATION-STANDARD.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/WORK-HIERARCHY.md
    - path:docs/07-workflow -[contains]-> path:docs/07-workflow/WORK-PACKET-STANDARD.md
    - path:docs/08-context -[contains]-> path:docs/08-context/CONTEXT-ARTIFACT-SCHEMAS.md
    - path:docs/08-context -[contains]-> path:docs/08-context/CONTEXT-BRIDGE.md
    - path:docs/08-context -[contains]-> path:docs/08-context/CONTEXT-PACK-STANDARD.md
    - path:docs/08-context -[contains]-> path:docs/08-context/CURRENT-STATE-STANDARD.md
    - path:docs/08-context -[contains]-> path:docs/08-context/DECISION-LOG-STANDARD.md
    - path:docs/08-context -[contains]-> path:docs/08-context/GENERATED-CONTEXT-STANDARD.md
    - path:docs/08-context -[contains]-> path:docs/08-context/HANDOFF-STANDARD.md
    - path:docs/08-context -[contains]-> path:docs/08-context/README.md
    - path:docs/08-context -[contains]-> path:docs/08-context/REHYDRATION-STANDARD.md
    - path:docs/08-context -[contains]-> path:docs/08-context/SESSION-CHRONICLE-STANDARD.md
    - path:docs/09-ai -[contains]-> path:docs/09-ai/AGENT-RUNBOOK.md
    - path:docs/09-ai -[contains]-> path:docs/09-ai/AGENT-SAFETY-RULES.md
    - path:docs/09-ai -[contains]-> path:docs/09-ai/AI-COLLABORATION-RULES.md
    - path:docs/09-ai -[contains]-> path:docs/09-ai/BOOTSTRAP-PROMPT.md
    - path:docs/09-ai -[contains]-> path:docs/09-ai/CURRENT-STATE.md
    - path:docs/09-ai -[contains]-> path:docs/09-ai/FRESH-CHAT-HANDOFF.md
    - path:docs/09-ai -[contains]-> path:docs/09-ai/MCP-TOOLING-STANDARD.md
    - path:docs/09-ai -[contains]-> path:docs/09-ai/MODEL-PROVIDER-STANDARD.md
    - path:docs/09-ai -[contains]-> path:docs/09-ai/PROMPTING-STANDARD.md
    - path:docs/09-ai -[contains]-> path:docs/09-ai/README.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/CLI-UX-STANDARD.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/CODING-STANDARD.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/DEPENDENCY-STANDARD.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/DIAGNOSTIC-STANDARD.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/ERROR-HANDLING-STANDARD.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/FIXTURE-STANDARD.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/OUTPUT-FORMAT-STANDARD.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/README.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/RUST-CODING-STANDARD.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/RUST-LEARNING-NOTES.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/RUST-VERIFICATION.md
    - path:docs/10-engineering -[contains]-> path:docs/10-engineering/TESTING-STANDARD.md
    - path:docs/11-security -[contains]-> path:docs/11-security/AGENT-SAFETY-MODEL.md
    - path:docs/11-security -[contains]-> path:docs/11-security/COMMAND-EXECUTION-SAFETY.md
    - path:docs/11-security -[contains]-> path:docs/11-security/FILE-OPERATION-SAFETY.md
    - path:docs/11-security -[contains]-> path:docs/11-security/MCP-SAFETY-BOUNDARIES.md
    - path:docs/11-security -[contains]-> path:docs/11-security/README.md
    - path:docs/11-security -[contains]-> path:docs/11-security/RESPONSIBLE-DISCLOSURE.md
    - path:docs/11-security -[contains]-> path:docs/11-security/SANDBOXING-PRINCIPLES.md
    - path:docs/11-security -[contains]-> path:docs/11-security/SECRET-HANDLING.md
    - path:docs/11-security -[contains]-> path:docs/11-security/SECURITY-MODEL.md
    - path:docs/11-security -[contains]-> path:docs/11-security/SUPPLY-CHAIN-SECURITY.md
    - path:docs/11-security -[contains]-> path:docs/11-security/THREAT-MODEL.md
    - path:docs/12-verification -[contains]-> path:docs/12-verification/CHECK-REGISTRY-STANDARD.md
    - path:docs/12-verification -[contains]-> path:docs/12-verification/EVIDENCE-PACKET-STANDARD.md
    - path:docs/12-verification -[contains]-> path:docs/12-verification/EXIT-CODE-STANDARD.md
    - path:docs/12-verification -[contains]-> path:docs/12-verification/QUALITY-GATES.md
    - path:docs/12-verification -[contains]-> path:docs/12-verification/README.md
    - path:docs/12-verification -[contains]-> path:docs/12-verification/REPORTING-STANDARD.md
    - path:docs/12-verification -[contains]-> path:docs/12-verification/TEST-MATRIX.md
    - path:docs/12-verification -[contains]-> path:docs/12-verification/VERIFICATION-BASELINE.md
    - path:docs/12-verification -[contains]-> path:docs/12-verification/VERIFICATION-MODEL.md
    - path:docs/13-operations -[contains]-> path:docs/13-operations/BACKUP-AND-EXPORTS.md
    - path:docs/13-operations -[contains]-> path:docs/13-operations/LOCAL-DEVELOPMENT.md
    - path:docs/13-operations -[contains]-> path:docs/13-operations/MAINTENANCE-MODEL.md
    - path:docs/13-operations -[contains]-> path:docs/13-operations/README.md
    - path:docs/13-operations -[contains]-> path:docs/13-operations/RELEASE-PROCESS.md
    - path:docs/13-operations -[contains]-> path:docs/13-operations/REPOSITORY-SETUP.md
    - path:docs/13-operations -[contains]-> path:docs/13-operations/SUPPORT-MODEL.md
    - path:docs/13-operations -[contains]-> path:docs/13-operations/TOOLCHAIN-SETUP.md
    - path:docs/13-operations -[contains]-> path:docs/13-operations/VERSIONING-POLICY.md
    - path:docs/14-integrations -[contains]-> path:docs/14-integrations/GITHUB-INTEGRATION.md
    - path:docs/14-integrations -[contains]-> path:docs/14-integrations/GITHUB-ISSUES-WORKFLOW.md
    - path:docs/14-integrations -[contains]-> path:docs/14-integrations/GITHUB-PROJECTS-WORKFLOW.md
    - path:docs/14-integrations -[contains]-> path:docs/14-integrations/MCP-INTEGRATION.md
    - path:docs/14-integrations -[contains]-> path:docs/14-integrations/MODEL-PROVIDER-INTEGRATIONS.md
    - path:docs/14-integrations -[contains]-> path:docs/14-integrations/NATIVE-TOOL-ADAPTERS.md
    - path:docs/14-integrations -[contains]-> path:docs/14-integrations/README.md
    - path:docs/15-business -[contains]-> path:docs/15-business/BUSINESS-THESIS.md
    - path:docs/15-business -[contains]-> path:docs/15-business/CUSTOMER-SEGMENTS.md
    - path:docs/15-business -[contains]-> path:docs/15-business/DISTRIBUTION-STRATEGY.md
    - path:docs/15-business -[contains]-> path:docs/15-business/PRICING-HYPOTHESES.md
    - path:docs/15-business -[contains]-> path:docs/15-business/README.md
    - path:docs/15-business -[contains]-> path:docs/15-business/REPO-AUDIT-OFFER.md
    - path:docs/15-business -[contains]-> path:docs/15-business/RISKS.md
    - path:docs/15-business -[contains]-> path:docs/15-business/VALIDATION-PLAN.md
    - path:docs/16-reference -[contains]-> path:docs/16-reference/COMMAND-CATALOG.md
    - path:docs/16-reference -[contains]-> path:docs/16-reference/CONFIGURATION-REFERENCE.md
    - path:docs/16-reference -[contains]-> path:docs/16-reference/FAQ.md
    - path:docs/16-reference -[contains]-> path:docs/16-reference/MONAD-TOML-REFERENCE.md
    - path:docs/16-reference -[contains]-> path:docs/16-reference/README.md
    - path:docs/16-reference -[contains]-> path:docs/16-reference/RESOURCES.md
    - path:docs/16-reference -[contains]-> path:docs/16-reference/TERMINOLOGY.md
    - path:docs/ai -[contains]-> path:docs/ai/AGENT-RUNBOOK.md
    - path:docs/ai -[contains]-> path:docs/ai/BOOTSTRAP-PROMPT.md
    - path:docs/architecture -[contains]-> path:docs/architecture/DRAFT-SANDBOX-WORKFLOW.md
    - path:docs/architecture -[contains]-> path:docs/architecture/MCP-INTEGRATION-STRATEGY.md
    - path:docs/architecture -[contains]-> path:docs/architecture/SUPERVISED-AGENT-WORKFLOW.md
    - path:docs/architecture -[contains]-> path:docs/architecture/WORKTREE-SAFETY-STRATEGY.md
    - path:docs/context -[contains]-> path:docs/context/CONTEXT-FRESHNESS-POLICY.md
    - path:docs/context -[contains]-> path:docs/context/RELEASE-CONTEXT-STATE.md
    - path:docs/development -[contains]-> path:docs/development/LOCAL-BUILD.md
    - path:docs/development -[contains]-> path:docs/development/LOCAL-VERIFY.md
    - path:docs/development -[contains]-> path:docs/development/README.md
    - path:docs/development -[contains]-> path:docs/development/WP-E8-004-LOCAL-BUILD-VERIFY-EVIDENCE.md
    - path:docs/project -[contains]-> path:docs/project/FOUNDATION-CLOSURE-AUDIT.md
    - path:docs/project -[contains]-> path:docs/project/FOUNDATION-CLOSURE-REPORT.md
    - path:docs/project -[contains]-> path:docs/project/MVP-COMMAND-REFERENCE.md
    - path:docs/project -[contains]-> path:docs/project/MVP-READINESS-REPORT.md
    - path:docs/project -[contains]-> path:docs/project/MVP-SCOPE-FREEZE.md
    - path:docs/project -[contains]-> path:docs/project/WP-E7-002-CLI-UX-EVIDENCE.md
    - path:docs/project -[contains]-> path:docs/project/WP-E7-003-COMMAND-SMOKE-TEST-EVIDENCE.md
    - path:docs/project -[contains]-> path:docs/project/WP-E7-004-DOC-ALIGNMENT-EVIDENCE.md
    - path:docs/project -[contains]-> path:docs/project/WP-E7-005-DRY-RUN-NO-WRITE-EVIDENCE.md
    - path:docs/project -[contains]-> path:docs/project/WP-E8-001-SCOPE-FREEZE-EVIDENCE.md
    - path:docs/release -[contains]-> path:docs/release/BUILD-METADATA.md
    - path:docs/release -[contains]-> path:docs/release/E8-CLOSEOUT.md
    - path:docs/release -[contains]-> path:docs/release/E9-CLOSEOUT.md
    - path:docs/release -[contains]-> path:docs/release/E9-STABILIZATION-PLAN.md
    - path:docs/release -[contains]-> path:docs/release/FIRST-PUBLIC-PRERELEASE-BOUNDARY.md
    - path:docs/release -[contains]-> path:docs/release/MVP-CANDIDATE-TAG-RECORD.md
    - path:docs/release -[contains]-> path:docs/release/MVP-CANDIDATE-VERIFICATION-AUDIT.md
    - path:docs/release -[contains]-> path:docs/release/PUBLIC-CLAIMS-AUDIT.md
    - path:docs/release -[contains]-> path:docs/release/PUBLIC-PRERELEASE-CHECKLIST.md
    - path:docs/release -[contains]-> path:docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
    - path:docs/release -[contains]-> path:docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md
    - path:docs/release -[contains]-> path:docs/release/PUBLIC-PRERELEASE-NOTES.md
    - path:docs/release -[contains]-> path:docs/release/PUBLIC-READINESS-GAP-AUDIT.md
    - path:docs/release -[contains]-> path:docs/release/README.md
    - path:docs/release -[contains]-> path:docs/release/RELEASE-NOTES-TEMPLATE.md
    - path:docs/release -[contains]-> path:docs/release/VERSIONING.md
    - path:docs/release -[contains]-> path:docs/release/WP-E8-002-RELEASE-DOCS-EVIDENCE.md
    - path:docs/release -[contains]-> path:docs/release/WP-E8-003-VERSION-BUILD-METADATA-EVIDENCE.md
    - path:docs/repository -[contains]-> path:docs/repository/GENERATED-ARTIFACT-POLICY.md
    - path:docs/repository -[contains]-> path:docs/repository/REPOSITORY-HYGIENE-REVIEW.md
    - path:docs/security -[contains]-> path:docs/security/AGENT-SAFETY-MODEL.md
    - path:docs/security -[contains]-> path:docs/security/MCP-SAFETY-BOUNDARIES.md
    - path:docs/wiki -[contains]-> path:docs/wiki/.artifacts
    - path:docs/wiki -[contains]-> path:docs/wiki/README.md
    - path:docs/wiki -[contains]-> path:docs/wiki/contents.md
    - path:docs/wiki -[contains]-> path:docs/wiki/contents.raw.json
    - path:docs/wiki -[contains]-> path:docs/wiki/deepwiki-dump-monad-workspace
    - path:docs/wiki -[contains]-> path:docs/wiki/structure.md
    - path:docs/wiki -[contains]-> path:docs/wiki/structure.raw.json
    - path:docs/wiki -[contains]-> path:docs/wiki/tools.raw.json
    - path:docs/wiki/deepwiki-dump-monad-workspace -[contains]-> path:docs/wiki/deepwiki-dump-monad-workspace/README.md
    - path:docs/wiki/deepwiki-dump-monad-workspace -[contains]-> path:docs/wiki/deepwiki-dump-monad-workspace/dump-deepwiki.mjs
    - path:docs/wiki/deepwiki-dump-monad-workspace -[contains]-> path:docs/wiki/deepwiki-dump-monad-workspace/run-dump.sh
    - path:docs/workflow -[contains]-> path:docs/workflow/APPROVAL-GATES.md
    - path:tools -[contains]-> path:tools/github
    - path:tools -[contains]-> path:tools/scripts
    - path:tools/github -[contains]-> path:tools/github/seed-e10-e11-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e12-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e13-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e14-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e15-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e16-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e17-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e18-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e19-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e2-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e3-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e4-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e5-issues.sh
    - path:tools/github -[contains]-> path:tools/github/seed-e6-issues.sh
    - path:tools/scripts -[contains]-> path:tools/scripts/audit-foundation-closure.sh
    - path:tools/scripts -[contains]-> path:tools/scripts/audit-mvp-candidate-verification.sh
    - path:tools/scripts -[contains]-> path:tools/scripts/check-adr-records.py
    - path:tools/scripts -[contains]-> path:tools/scripts/check-context-records.py
    - path:tools/scripts -[contains]-> path:tools/scripts/check-deliverable-records.py
    - path:tools/scripts -[contains]-> path:tools/scripts/check-epic-records.py
    - path:tools/scripts -[contains]-> path:tools/scripts/check-markdown-frontmatter.py
    - path:tools/scripts -[contains]-> path:tools/scripts/check-required-paths.py
    - path:tools/scripts -[contains]-> path:tools/scripts/check-task-records.py
    - path:tools/scripts -[contains]-> path:tools/scripts/check-work-records.py
    - path:tools/scripts -[contains]-> path:tools/scripts/github
    - path:tools/scripts -[contains]-> path:tools/scripts/verify-no-write-commands.sh
    - path:tools/scripts -[contains]-> path:tools/scripts/verify.sh
    - path:tools/scripts/github -[contains]-> path:tools/scripts/github/create-e7-mvp-hardening-issues.sh
    - path:tools/scripts/github -[contains]-> path:tools/scripts/github/create-e7-workpacket-issues.sh
    - path:tools/scripts/github -[contains]-> path:tools/scripts/github/create-e8-release-prep-issues.sh
    - path:tools/scripts/github -[contains]-> path:tools/scripts/github/create-e9-post-mvp-stabilization-issues.sh
    - path:work -[contains]-> path:work/README.md
    - path:work -[contains]-> path:work/deliverables
    - path:work -[contains]-> path:work/epics
    - path:work -[contains]-> path:work/packets
    - path:work -[contains]-> path:work/records
    - path:work -[contains]-> path:work/tasks
    - path:work/deliverables -[contains]-> path:work/deliverables/E0
    - path:work/deliverables -[contains]-> path:work/deliverables/E1
    - path:work/deliverables -[contains]-> path:work/deliverables/E10
    - path:work/deliverables -[contains]-> path:work/deliverables/E2
    - path:work/deliverables/E0 -[contains]-> path:work/deliverables/E0/D-WP-E0-010-001-deliverable-record-index.md
    - path:work/deliverables/E0 -[contains]-> path:work/deliverables/E0/D-WP-E0-010-002-deliverable-record-verifier.md
    - path:work/deliverables/E0 -[contains]-> path:work/deliverables/E0/D-WP-E0-010-003-verification-baseline-update.md
    - path:work/deliverables/E0 -[contains]-> path:work/deliverables/E0/D-WP-E0-011-001-e0-closure-record.md
    - path:work/deliverables/E0 -[contains]-> path:work/deliverables/E0/D-WP-E0-011-002-e1-starting-point.md
    - path:work/deliverables/E0 -[contains]-> path:work/deliverables/E0/D-WP-E0-011-003-context-handoff-update.md
    - path:work/deliverables/E0 -[contains]-> path:work/deliverables/E0/README.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-001-001-rust-workspace-manifest.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-001-002-core-runtime-library.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-001-003-thin-cli-entrypoint.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-002-001-diagnostics-module.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-002-002-core-runtime-exports.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-002-003-diagnostics-context-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-003-001-core-error-module.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-003-002-core-error-exports.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-003-003-core-error-context-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-004-001-workspace-context-module.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-004-002-workspace-context-exports.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-004-003-workspace-context-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-005-001-root-monad-manifest.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-005-002-manifest-model-module.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-005-003-manifest-model-exports.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-005-004-manifest-model-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-006-001-manifest-parsing-dependencies.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-006-002-manifest-loading-runtime.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-006-003-manifest-loading-exports.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-006-004-manifest-loading-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-007-002-cli-info-verification.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-007-003-cli-info-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-008-001-core-workspace-checks.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-008-002-cli-check-command.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-008-003-cli-check-verification.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-008-004-cli-check-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-009-001-repository-contract-module.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-009-002-checks-integration.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-009-003-repository-contract-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-010-001-output-formatting-module.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-010-002-cli-output-integration.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-010-003-output-formatting-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-011-001-cli-output-format-argument.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-011-002-cli-output-format-tests.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-011-003-cli-output-format-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-013-001-e1-closure-record.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-013-002-e2-starting-point.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/D-WP-E1-013-003-e2-context-handoff.md
    - path:work/deliverables/E1 -[contains]-> path:work/deliverables/E1/README.md
    - path:work/deliverables/E10 -[contains]-> path:work/deliverables/E10/WP-E10-001-public-claims-audit.md
    - path:work/deliverables/E10 -[contains]-> path:work/deliverables/E10/WP-E10-002-public-prerelease-evidence-checklist.md
    - path:work/deliverables/E10 -[contains]-> path:work/deliverables/E10/WP-E10-003-distribution-posture.md
    - path:work/deliverables/E10 -[contains]-> path:work/deliverables/E10/WP-E10-004-public-prerelease-notes.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-001-001-repository-inspection-module.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-001-002-repository-inspection-exports.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-001-003-workspace-check-integration.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-001-004-repository-inspection-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-002-001-inspection-summary-rendering.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-002-002-cli-inspect-command.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-002-003-inspect-json-output.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-002-004-inspect-verification.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-003-001-expanded-repository-entry-roles.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-003-002-expanded-classification-rules.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-003-003-traversal-policy-hardening.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-003-004-classification-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-004-001-repository-entry-category.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-004-002-category-summary-metrics.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-004-003-inspect-metrics-output.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-004-004-metrics-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-005-001-traversal-planning-types.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-005-002-conservative-traversal-guardrails.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-005-003-traversal-plan-output.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-005-004-traversal-guardrails-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-006-001-bounded-traversal-model.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-006-002-bounded-traversal-implementation.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-006-003-basic-ignore-rule-support.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-006-004-bounded-traversal-output.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-007-001-repository-graph-model.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-007-002-graph-construction.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-007-003-graph-metrics-output.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-007-004-graph-model-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-008-001-graph-render-format-type.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-008-002-text-json-graph-renderers.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-008-003-mermaid-dot-graph-renderers.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-008-004-graph-rendering-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-009-001-graph-cli-command.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-009-002-graph-format-routing.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-009-003-graph-smoke-verification.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-009-004-graph-command-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-010-001-toolchain-detection-model.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-010-002-common-toolchain-detection.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-010-003-toolchain-inspect-output.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-010-004-toolchain-detection-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-011-001-dependency-detection-model.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-011-002-dependency-signal-detection.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-011-003-dependency-inspect-output.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-011-004-dependency-detection-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-012-001-repository-policy-model.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-012-002-advisory-policy-checks.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-012-003-policy-inspect-output.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-012-004-policy-check-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-013-001-context-pack-model.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-013-002-context-pack-construction.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-013-003-context-pack-rendering.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-013-004-context-pack-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-014-001-context-cli-command.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-014-002-context-format-routing.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-014-003-context-smoke-verification.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-014-004-context-command-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-015-001-context-pack-export-model.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-015-002-context-pack-file-export.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-015-003-workspace-context-pack-export-helper.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-015-004-context-export-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-016-001-context-write-cli-flag.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-016-002-context-write-validation.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-016-003-context-write-verification.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-016-004-context-write-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-017-001-generated-context-ignore-rule.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-017-002-generated-context-policy-diagnostics.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-017-003-generated-context-policy-verification.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/D-WP-E2-017-004-generated-context-policy-handoff.md
    - path:work/deliverables/E2 -[contains]-> path:work/deliverables/E2/README.md
    - path:work/epics -[contains]-> path:work/epics/E0-project-foundation.md
    - path:work/epics -[contains]-> path:work/epics/E1-runtime-foundation.md
    - path:work/epics -[contains]-> path:work/epics/E10-public-prerelease-hardening.md
    - path:work/epics -[contains]-> path:work/epics/E2-repository-intelligence-foundation.md
    - path:work/epics -[contains]-> path:work/epics/E9-post-mvp-candidate-stabilization.md
    - path:work/epics -[contains]-> path:work/epics/README.md
    - path:work/packets -[contains]-> path:work/packets/E0
    - path:work/packets -[contains]-> path:work/packets/E1
    - path:work/packets -[contains]-> path:work/packets/E2
    - path:work/packets -[contains]-> path:work/packets/README.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/README.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-001-establish-repository-foundation.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-002-establish-documentation-architecture.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-003-establish-context-bridge-foundation.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-004-establish-workflow-standards.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-005-establish-verification-baseline.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-006-establish-work-packet-records.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-007-establish-adr-verification.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-008-establish-epic-record-verification.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-009-establish-task-record-foundation.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-010-establish-deliverable-record-foundation.md
    - path:work/packets/E0 -[contains]-> path:work/packets/E0/WP-E0-011-close-e0-and-prepare-e1-handoff.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/README.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-001-establish-rust-workspace-runtime-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-002-establish-core-diagnostics-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-003-establish-core-error-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-004-establish-workspace-context-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-005-establish-manifest-model-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-006-establish-manifest-loading-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-007-establish-cli-info-command-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-008-establish-cli-check-command-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-009-establish-repository-contract-check-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-010-establish-runtime-output-formatting-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-011-establish-cli-output-format-argument-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-012-establish-json-output-formatting-foundation.md
    - path:work/packets/E1 -[contains]-> path:work/packets/E1/WP-E1-013-close-e1-and-prepare-e2-handoff.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/README.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-001-establish-repository-inspection-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-002-establish-monad-inspect-command-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-003-enrich-repository-inspection-classification.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-004-add-repository-inspection-summary-metrics.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-005-add-recursive-traversal-plan-and-guardrails.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-006-implement-bounded-repository-traversal-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-007-add-repository-graph-model-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-008-add-graph-rendering-format-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-009-add-monad-graph-command-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-010-add-toolchain-detection-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-011-add-dependency-signal-detection-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-012-add-repository-intelligence-policy-check-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-013-add-repository-context-pack-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-014-add-monad-context-command-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-015-add-repository-context-pack-export-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-016-add-monad-context-write-foundation.md
    - path:work/packets/E2 -[contains]-> path:work/packets/E2/WP-E2-017-add-generated-context-artifact-policy-foundation.md
    - path:work/records -[contains]-> path:work/records/README.md
    - path:work/tasks -[contains]-> path:work/tasks/E0
    - path:work/tasks -[contains]-> path:work/tasks/E1
    - path:work/tasks -[contains]-> path:work/tasks/E2
    - path:work/tasks -[contains]-> path:work/tasks/README.md
    - path:work/tasks/E0 -[contains]-> path:work/tasks/E0/README.md
    - path:work/tasks/E0 -[contains]-> path:work/tasks/E0/T-WP-E0-009-001-create-task-record-directory-and-index.md
    - path:work/tasks/E0 -[contains]-> path:work/tasks/E0/T-WP-E0-009-002-add-task-record-verification.md
    - path:work/tasks/E0 -[contains]-> path:work/tasks/E0/T-WP-E0-009-003-update-e0-planning-and-verification-records.md
    - path:work/tasks/E0 -[contains]-> path:work/tasks/E0/T-WP-E0-010-001-create-deliverable-record-directory-and-index.md
    - path:work/tasks/E0 -[contains]-> path:work/tasks/E0/T-WP-E0-010-002-add-deliverable-record-verification.md
    - path:work/tasks/E0 -[contains]-> path:work/tasks/E0/T-WP-E0-010-003-update-e0-planning-and-verification-records.md
    - path:work/tasks/E0 -[contains]-> path:work/tasks/E0/T-WP-E0-011-001-close-e0-records.md
    - path:work/tasks/E0 -[contains]-> path:work/tasks/E0/T-WP-E0-011-002-update-context-handoff.md
    - path:work/tasks/E0 -[contains]-> path:work/tasks/E0/T-WP-E0-011-003-create-e1-starting-point.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/README.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-001-001-create-rust-workspace-crates.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-001-002-add-minimal-core-runtime-identity.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-001-003-add-thin-cli-entrypoint.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-001-004-add-rust-verification-to-baseline.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-002-001-add-diagnostics-module.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-002-002-export-diagnostics-from-core-runtime.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-002-003-update-e1-records-and-context.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-003-001-add-core-error-module.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-003-002-export-core-error-model.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-003-003-update-e1-records-and-context.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-004-001-add-workspace-context-module.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-004-002-export-workspace-context-from-core-runtime.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-004-003-update-e1-records-and-context.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-005-001-add-root-monad-manifest.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-005-002-add-manifest-model-module.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-005-003-export-manifest-model-from-core-runtime.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-005-004-update-e1-records-and-context.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-006-001-add-manifest-parsing-dependencies.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-006-002-add-manifest-loading.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-006-003-update-e1-records-and-context.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-007-001-add-cli-command-parser.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-007-002-add-cli-info-rendering.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-007-003-update-e1-records-and-context.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-008-001-add-core-workspace-checks.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-008-002-add-cli-check-command.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-008-003-update-e1-records-and-context.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-009-001-add-repository-contract-module.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-009-002-integrate-contract-checks-with-monad-check.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-009-003-update-e1-records-and-context.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-010-001-add-output-formatting-module.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-010-002-use-output-formatting-in-cli.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-010-003-update-e1-records-and-context.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-011-001-add-cli-output-format-parsing.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-011-002-wire-output-format-into-cli-commands.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-011-003-update-e1-records-and-context.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-013-001-close-e1-records.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-013-002-update-runtime-context-handoff.md
    - path:work/tasks/E1 -[contains]-> path:work/tasks/E1/T-WP-E1-013-003-create-e2-starting-point.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/README.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-001-001-add-repository-inspection-module.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-001-002-export-repository-inspection-types.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-001-003-integrate-inspection-with-workspace-checks.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-001-004-update-e2-records-and-context.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-002-001-add-inspection-summary-rendering.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-002-002-add-cli-inspect-command.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-002-003-add-inspect-smoke-verification.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-002-004-update-e2-records-and-context.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-003-001-expand-repository-entry-roles.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-003-002-expand-classification-rules.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-003-003-harden-traversal-policy.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-003-004-update-e2-classification-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-004-001-add-repository-entry-category.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-004-002-add-category-metrics-to-summary.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-004-003-render-summary-metrics.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-004-004-update-e2-metrics-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-005-001-add-traversal-planning-types.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-005-002-add-conservative-traversal-guardrails.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-005-003-render-traversal-plan-in-inspect-summary.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-005-004-update-e2-traversal-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-006-001-add-bounded-traversal-model.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-006-002-implement-bounded-traversal.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-006-003-add-basic-ignore-rule-support.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-006-004-render-bounded-traversal-metrics.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-007-001-add-repository-graph-model.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-007-002-build-graph-from-bounded-traversal.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-007-003-render-graph-metrics-in-inspect-summary.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-007-004-update-e2-graph-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-008-001-add-graph-render-format-type.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-008-002-add-text-and-json-graph-renderers.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-008-003-add-mermaid-and-dot-graph-renderers.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-008-004-update-e2-graph-rendering-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-009-001-add-graph-cli-command.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-009-002-add-graph-format-routing.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-009-003-add-graph-smoke-verification.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-009-004-update-e2-graph-command-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-010-001-add-toolchain-detection-model.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-010-002-detect-common-toolchains.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-010-003-render-toolchain-metrics-in-inspect.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-010-004-update-e2-toolchain-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-011-001-add-dependency-detection-model.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-011-002-detect-dependency-signals.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-011-003-render-dependency-metrics-in-inspect.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-011-004-update-e2-dependency-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-012-001-add-repository-policy-model.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-012-002-add-advisory-policy-checks.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-012-003-render-policy-metrics-in-inspect.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-012-004-update-e2-policy-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-013-001-add-context-pack-model.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-013-002-build-context-pack-from-repository-intelligence.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-013-003-render-context-pack.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-013-004-update-e2-context-pack-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-014-001-add-context-cli-command.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-014-002-add-context-format-routing.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-014-003-add-context-smoke-verification.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-014-004-update-e2-context-command-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-015-001-add-context-pack-export-model.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-015-002-add-deterministic-context-pack-export.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-015-003-add-workspace-export-helper.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-015-004-update-e2-context-export-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-016-001-add-context-write-cli-flag.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-016-002-add-write-flag-validation.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-016-003-add-context-write-smoke-verification.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-016-004-update-e2-context-write-records.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-017-001-add-generated-context-ignore-rule.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-017-002-add-generated-context-policy-diagnostics.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-017-003-add-generated-context-policy-verification.md
    - path:work/tasks/E2 -[contains]-> path:work/tasks/E2/T-WP-E2-017-004-update-e2-generated-context-policy-records.md
    - root -[contains]-> path:.artifacts
    - root -[contains]-> path:.editorconfig
    - root -[contains]-> path:.git
    - root -[contains]-> path:.github
    - root -[contains]-> path:.gitignore
    - root -[contains]-> path:.monad
    - root -[contains]-> path:CHANGELOG.md
    - root -[contains]-> path:CONTRIBUTING.md
    - root -[contains]-> path:Cargo.lock
    - root -[contains]-> path:Cargo.toml
    - root -[contains]-> path:LICENSE
    - root -[contains]-> path:README.md
    - root -[contains]-> path:SECURITY.md
    - root -[contains]-> path:assets
    - root -[contains]-> path:bun.lock
    - root -[contains]-> path:clippy.toml
    - root -[contains]-> path:complete_wp_e0_006_adr_foundation.sh
    - root -[contains]-> path:complete_wp_e10_001_public_claims_audit.sh
    - root -[contains]-> path:complete_wp_e10_002_public_prerelease_evidence.sh
    - root -[contains]-> path:complete_wp_e10_003_distribution_posture.sh
    - root -[contains]-> path:complete_wp_e10_004_public_prerelease_notes.sh
    - root -[contains]-> path:complete_wp_e10_005_public_prerelease_verification_audit.sh
    - root -[contains]-> path:crates
    - root -[contains]-> path:docs
    - root -[contains]-> path:monad.toml
    - root -[contains]-> path:nano
    - root -[contains]-> path:package.json
    - root -[contains]-> path:rust-toolchain.toml
    - root -[contains]-> path:sue close 72 
    - root -[contains]-> path:target
    - root -[contains]-> path:tools
    - root -[contains]-> path:work
```

</details>


### 11. monad-context

```bash
cargo run -p monad-cli -- context
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-context.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/monad context`
# Monad Repository Context Pack

- schema_version: 1
- root: .
- sections: 7
- facts: 32

## Repository Overview

- kind: overview
- root: .
- top_level_entry_count: 32
- top_level_file_count: 22
- top_level_directory_count: 10

Notes:
- This section summarizes the shallow top-level repository inspection.
- Counts are intentionally based on the top-level inspection, not full traversal.

## Bounded Traversal

- kind: traversal
- mode: bounded_recursive
- entry_count: 749
- max_observed_depth: 3
- max_allowed_depth: 3
- follow_symlinks: false
- include_generated_or_external: false
- respect_ignore_files: true
- deterministic_ordering: true
- candidate_count: 80
- shallow_only_count: 662
- skip_count: 7

Notes:
- Bounded traversal is conservative by default.
- Generated/external paths are recorded but not descended into unless future policy explicitly allows it.

## Repository Graph

- kind: graph
- node_count: 750
- edge_count: 749
- max_depth: 3

Notes:
- graph_category:assets=1
- graph_category:configuration=1
- graph_category:continuous_integration=1
- graph_category:documentation=48
- graph_category:generated_or_external=3
- graph_category:governance=1
- graph_category:hidden=3
- graph_category:javascript_package_management=2
- graph_category:legal=1
- graph_category:monad_control=2
- graph_category:other=670
- graph_category:rust_runtime=7
- graph_category:source=4
- graph_category:tests=1
- graph_category:tooling=2
- graph_category:version_control=1
- graph_category:work_management=1
- graph_traversal_decision:candidate_for_future_traversal=80
- graph_traversal_decision:inspect_shallow_only=662
- graph_traversal_decision:skip_by_default=7

## Toolchains

- kind: toolchains
- detected_toolchain_count: 3
- signal_count: 40

Notes:
- toolchain:javascript=3
- toolchain:python=8
- toolchain:rust=29
- signal_kind:lockfile=2
- signal_kind:manifest=5
- signal_kind:source_file=33

## Dependency Signals

- kind: dependencies
- detected_toolchain_count: 2
- signal_count: 7
- manifest_count: 5
- lockfile_count: 2
- package_manager_config_count: 0
- build_file_count: 0

Notes:
- dependency_toolchain:javascript=2
- dependency_toolchain:rust=5
- dependency_signal_kind:lockfile=2
- dependency_signal_kind:manifest=5

## Repository Intelligence Policy

- kind: policy
- diagnostic_count: 2
- info_count: 2
- advisory_count: 0
- warning_count: 0
- has_warnings: false

Notes:
- MONAD-RI-0200 [info] 3 generated or external path(s) were identified for conservative traversal handling
- MONAD-RI-0201 [info] bounded traversal stayed within configured max depth

## Top-Level Entries

- kind: top_level_entries
- entry_count: 32

Notes:
- .artifacts kind=directory category=hidden role=hidden traversal=inspect_shallow_only
- .editorconfig kind=file category=configuration role=editorconfig traversal=inspect_shallow_only
- .git kind=directory category=generated_or_external role=generated_or_external traversal=skip_generated_or_external
- .github kind=directory category=continuous_integration role=ci_root traversal=safe_for_future_traversal
- .gitignore kind=file category=version_control role=gitignore traversal=inspect_shallow_only
- .monad kind=directory category=monad_control role=monad_state_root traversal=safe_for_future_traversal
- CHANGELOG.md kind=file category=other role=other traversal=inspect_shallow_only
- CONTRIBUTING.md kind=file category=other role=other traversal=inspect_shallow_only
- Cargo.lock kind=file category=rust_runtime role=rust_lockfile traversal=inspect_shallow_only
- Cargo.toml kind=file category=rust_runtime role=rust_workspace_manifest traversal=inspect_shallow_only
- LICENSE kind=file category=legal role=license traversal=inspect_shallow_only
- README.md kind=file category=documentation role=readme traversal=inspect_shallow_only
- SECURITY.md kind=file category=other role=other traversal=inspect_shallow_only
- assets kind=directory category=assets role=asset_root traversal=safe_for_future_traversal
- bun.lock kind=file category=javascript_package_management role=javascript_package_config traversal=inspect_shallow_only
- clippy.toml kind=file category=rust_runtime role=rust_quality_config traversal=inspect_shallow_only
- complete_wp_e0_006_adr_foundation.sh kind=file category=other role=other traversal=inspect_shallow_only
- complete_wp_e10_001_public_claims_audit.sh kind=file category=other role=other traversal=inspect_shallow_only
- complete_wp_e10_002_public_prerelease_evidence.sh kind=file category=other role=other traversal=inspect_shallow_only
- complete_wp_e10_003_distribution_posture.sh kind=file category=other role=other traversal=inspect_shallow_only
- complete_wp_e10_004_public_prerelease_notes.sh kind=file category=other role=other traversal=inspect_shallow_only
- complete_wp_e10_005_public_prerelease_verification_audit.sh kind=file category=other role=other traversal=inspect_shallow_only
- crates kind=directory category=source role=source_root traversal=safe_for_future_traversal
- docs kind=directory category=documentation role=documentation_root traversal=safe_for_future_traversal
- monad.toml kind=file category=monad_control role=monad_manifest traversal=inspect_shallow_only
- nano kind=file category=other role=other traversal=inspect_shallow_only
- package.json kind=file category=javascript_package_management role=javascript_package_config traversal=inspect_shallow_only
- rust-toolchain.toml kind=file category=rust_runtime role=rust_toolchain traversal=inspect_shallow_only
- sue close 72  kind=file category=other role=other traversal=inspect_shallow_only
- target kind=directory category=generated_or_external role=generated_or_external traversal=skip_generated_or_external
- tools kind=directory category=tooling role=tooling_root traversal=safe_for_future_traversal
- work kind=directory category=work_management role=work_root traversal=safe_for_future_traversal
```

</details>


### 12. monad-context-verify

```bash
cargo run -p monad-cli -- context verify
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-context-verify.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/monad context verify`
Monad context verification: PASSED
  checked: 5 files
  found: 5
  missing: 0

  ✓ .monad/context/current-state.md (Current State)
    ⚠ missing heading: '## Epics'
  ✓ .monad/context/latest-handoff.md (Latest Handoff)
  ✓ .monad/context/latest-context-pack.md (Context Pack)
    ⚠ missing heading: '# Context Pack'
  ✓ docs/ai/BOOTSTRAP-PROMPT.md (Bootstrap Prompt)
  ✓ monad.toml (Project Manifest)
```

</details>


### 13. monad-plan

```bash
cargo run -p monad-cli -- plan "explain this repository"
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-plan.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/monad plan 'explain this repository'`
Monad supervised plan

Intent: explain this repository
Provider: mock
Model: mock-model

Provider note:
- Mock provider note: this plan was produced locally without a real model API call.

Plan steps:
1. Clarify intent
   - Confirm the requested outcome: explain this repository
2. Load repo-native context
   - Use committed project documents, work packets, ADRs, and context files as source of truth.
3. Identify likely affected areas
   - List files, modules, commands, and documentation areas likely to be involved before drafting changes.
4. Draft safe file operations [approval required]
   - Represent proposed changes as create, update, delete, skip, conflict, or no-op operations.
5. Dry-run before apply [approval required]
   - Evaluate planned file operations and expose conflicts before writing any files.
6. Verify and review [approval required]
   - Run relevant verification after approved changes and review evidence before committing.

Verification commands:
- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `tools/scripts/verify.sh`

Explicit non-actions:
- No files were created, updated, or deleted.
- No shell commands were run.
- No Git state was changed.
- No commits, pushes, pull requests, or deployments were performed.
- No real model provider or external AI API was called.

Risks and cautions:
- This is a planning result, not verified truth.
- Future file changes require explicit approval and dry-run review.
- Provider output must not bypass repository policy, verification, or human review.

Status: plan only; no files were written and no commands were run.
```

</details>


### 14. monad-evolve-verify-baseline

```bash
cargo run -p monad-cli -- evolve verify-baseline --dry-run
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-evolve-verify-baseline.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/monad evolve verify-baseline --dry-run`
Monad dry-run file operation plan

Summary:
- Operations: 1
- Would create: 1
- Would update: 0
- Would delete: 0
- Would skip: 0
- Would no-op: 0
- Conflicts: 0
- Appears safe to apply: true

Operations:
- [CREATE] docs/verification/README.md: create `docs/verification/README.md` from embedded template `verify-baseline.readme`

Mode: dry-run
No files were written.
```

</details>


### 15. monad-evolve-context-baseline

```bash
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/monad-evolve-context-baseline.log`

<details>
<summary>Command output</summary>

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/monad evolve context-baseline --dry-run`
Monad dry-run file operation plan

Summary:
- Operations: 3
- Would create: 1
- Would update: 0
- Would delete: 0
- Would skip: 0
- Would no-op: 0
- Conflicts: 2
- Appears safe to apply: false

Operations:
- [CREATE] docs/ai/README.md: create `docs/ai/README.md` from embedded template `context-baseline.readme`
- [CONFLICT] .monad/context/current-state.md: create target already exists; applying would overwrite an existing path
- [CONFLICT] .monad/context/latest-handoff.md: create target already exists; applying would overwrite an existing path

Mode: dry-run
No files were written.
No AI summarization was performed.
```

</details>


### 16. root-verify

```bash
tools/scripts/verify.sh
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/root-verify.log`

<details>
<summary>Command output</summary>

```text
==> Checking git diff whitespace
==> Checking generated context artifact ignore policy
==> Checking required foundation and runtime paths
All required foundation and runtime paths exist.
==> Checking Markdown frontmatter
All first-party docs/work/.monad Markdown files have YAML frontmatter.
==> Checking context records
All context records satisfy durable continuity and current release-context discoverability.
==> Checking work records
All work packet records satisfy the required structure.
==> Checking task records
All task records satisfy the required baseline structure.
==> Checking deliverable records
All deliverable records satisfy the required baseline structure.
==> Formatting Rust code
==> Running Rust tests
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/main.rs (target/debug/deps/monad-b4e8f786dd2c8393)

running 37 tests
test tests::check_command_parses_text_and_json_formats ... ok
test tests::context_command_parses_write_flag ... ok
test tests::context_command_parses_supported_formats ... ok
test tests::context_generate_bootstrap_parses ... ok
test tests::context_command_rejects_unknown_formats ... ok
test tests::context_generate_current_state_parses ... ok
test tests::context_generate_handoff_parses ... ok
test tests::context_generate_unknown_artifact_returns_error ... ok
test tests::context_generate_without_artifact_returns_error ... ok
test tests::context_verify_parses ... ok
test tests::context_unknown_subcommand_returns_error ... ok
test tests::evolve_context_baseline_requires_dry_run_with_specific_error ... ok
test tests::evolve_dry_run_commands_parse ... ok
test tests::context_pack_parses ... ok
test tests::evolve_verify_baseline_requires_dry_run_with_specific_error ... ok
test tests::format_can_appear_before_command ... ok
test tests::graph_command_parses_supported_formats ... ok
test tests::graph_command_rejects_unknown_formats ... ok
test tests::help_command_parses ... ok
test tests::help_text_mentions_context_command_formats_and_write_mode ... ok
test tests::help_text_mentions_context_generate_bootstrap ... ok
test tests::help_text_mentions_context_generate_current_state ... ok
test tests::help_text_mentions_context_generate_handoff ... ok
test tests::help_text_mentions_context_pack ... ok
test tests::help_text_mentions_context_verify ... ok
test tests::help_text_mentions_graph_command_and_formats ... ok
test tests::info_command_parses_text_and_json_formats ... ok
test tests::help_text_mentions_plan_and_evolve_dry_run_commands ... ok
test tests::inspect_command_parses_text_and_json_formats ... ok
test tests::no_command_defaults_to_info ... ok
test tests::non_graph_commands_reject_graph_only_formats ... ok
test tests::plan_command_parses_multi_word_intent ... ok
test tests::plan_rejects_format_flags_for_now ... ok
test tests::plan_without_intent_returns_actionable_error ... ok
test tests::unknown_command_returns_error ... ok
test tests::version_command_parses ... ok
test tests::write_flag_is_rejected_for_non_context_commands ... ok

test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli_smoke.rs (target/debug/deps/cli_smoke-6557de83d18f713e)

running 13 tests
test evolve_context_baseline_requires_dry_run_smoke_test ... ok
test plan_unsupported_format_failure_smoke_test ... ok
test evolve_verify_baseline_requires_dry_run_smoke_test ... ok
test unsupported_write_flag_failure_smoke_test ... ok
test evolve_verify_baseline_dry_run_smoke_test ... ok
test unsupported_argument_failure_smoke_test ... ok
test plan_missing_intent_failure_smoke_test ... ok
test evolve_context_baseline_dry_run_smoke_test ... ok
test help_command_smoke_test ... ok
test plan_command_smoke_test ... ok
test version_command_smoke_test ... ok
test inspect_command_smoke_test ... ok
test check_command_smoke_test ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

     Running unittests src/lib.rs (target/debug/deps/monad_core-5025b4ec710d14a4)

running 315 tests
test agents::draft::tests::draft_operation_can_be_created_from_planned_operation ... ok
test agents::draft::tests::draft_can_be_created_from_file_operation_plan ... ok
test agents::draft::tests::draft_state_labels_are_stable ... ok
test agents::draft::tests::rendered_draft_is_reviewable_and_non_mutating ... ok
test agents::draft::tests::sandbox_kind_identifies_git_isolation ... ok
test agents::draft::tests::terminal_draft_state_cannot_be_reopened_by_review_or_approval ... ok
test agents::model::tests::message_role_labels_are_stable ... ok
test agents::model::tests::model_request_preserves_purpose_and_messages ... ok
test agents::model::tests::user_prompt_request_contains_one_user_message ... ok
test agents::model::tests::model_response_preserves_provider_model_and_content ... ok
test agents::plan::tests::local_plan_rejects_empty_intent ... ok
test agents::plan::tests::local_plan_contains_intent_and_no_write_non_actions ... ok
test agents::plan::tests::plan_can_use_provider_trait ... ok
test agents::provider::tests::capability_metadata_describes_mock_provider ... ok
test agents::provider::tests::mock_provider_rejects_empty_requests ... ok
test agents::provider::tests::mock_provider_returns_fixed_response_without_external_api ... ok
test agents::plan::tests::rendered_plan_is_reviewable_and_non_mutating ... ok
test agents::provider::tests::provider_metadata_exposes_stable_fields ... ok
test agents::provider::tests::provider_trait_can_be_used_through_trait_object ... ok
test checks::adapters::javascript::tests::javascript_checks_are_skipped_when_javascript_is_not_detected ... ok
test checks::adapters::rust::tests::rust_checks_are_skipped_when_rust_is_not_detected ... ok
test checks::adapters::tests::selected_adapter_checks_include_javascript_when_javascript_detected ... ok
test checks::evidence::tests::evidence_check_result_preserves_check_status ... ok
test checks::evidence::tests::evidence_command_summary_captures_first_output_lines ... ok
test checks::evidence::tests::evidence_packet_summarizes_check_report ... ok
test checks::json::tests::json_report_contains_command_summaries ... ok
test checks::json::tests::json_report_marks_failed_result ... ok
test checks::adapters::javascript::tests::javascript_checks_pass_for_package_json_and_lockfile ... ok
test checks::model::tests::check_id_preserves_stable_text ... ok
test checks::json::tests::json_report_contains_summary_and_check_results ... ok
test checks::model::tests::severity_labels_are_stable ... ok
test checks::registry::tests::registry_definitions_are_deterministically_ordered_by_id ... ok
test checks::registry::tests::registry_replaces_duplicate_check_ids ... ok
test checks::registry::tests::registry_starts_empty ... ok
test checks::model::tests::status_labels_are_stable ... ok
test checks::report::tests::markdown_report_escapes_table_pipes ... ok
test checks::model::tests::check_definition_exposes_metadata ... ok
test checks::model::tests::check_result_constructors_set_status ... ok
test checks::report::tests::markdown_report_includes_summary_and_check_results ... ok
test checks::registry::tests::registry_registers_and_finds_checks ... ok
test checks::run::tests::initial_registry_contains_expected_checks ... ok
test checks::run::tests::report_counts_statuses ... ok
test checks::run::tests::required_file_check_fails_when_file_is_missing ... ok
test checks::run::tests::report_renders_human_readable_summary ... ok
test checks::tests::check_model_exports_are_usable_from_checks_boundary ... ok
test context::bootstrap::tests::extract_description_returns_none_for_empty ... ok
test context::bootstrap::tests::extract_description_skips_frontmatter_and_headers ... ok
test context::bootstrap::tests::extract_description_skips_badges ... ok
test checks::report::tests::write_check_evidence_packet_writes_latest_report ... ok
test context::bootstrap::tests::extract_project_name_missing_falls_back ... ok
test context::bootstrap::tests::extract_project_name_from_toml ... ok
test context::bootstrap::tests::extract_project_name_no_project_section_falls_back ... ok
test context::bootstrap::tests::extract_project_name_prefers_display_name ... ok
test context::bootstrap::tests::render_epic_progress_is_included ... ok
test context::bootstrap::tests::render_includes_current_work ... ok
test context::bootstrap::tests::render_includes_project_identity ... ok
test context::bootstrap::tests::render_includes_continuation_protocol ... ok
test context::bootstrap::tests::render_includes_frontmatter ... ok
test context::bootstrap::tests::render_includes_reading_order ... ok
test context::bootstrap::tests::render_includes_source_of_truth ... ok
test context::bootstrap::tests::render_includes_workflow_rules ... ok
test context::bootstrap::tests::render_with_no_active_epic ... ok
test context::bootstrap::tests::render_is_deterministic ... ok
test context::bootstrap::tests::workflow_rules_are_non_empty ... ok
test context::bootstrap::tests::workflow_rules_mention_conventional_commits ... ok
test context::bootstrap::tests::render_includes_response_expectations ... ok
test context::current_state::tests::current_state_artifact_finds_active_epic ... ok
test context::current_state::tests::current_state_artifact_returns_none_when_no_active_epic ... ok
test context::current_state::tests::extract_frontmatter_value_handles_quoted_values ... ok
test context::current_state::tests::extract_frontmatter_value_handles_unquoted_values ... ok
test context::current_state::tests::extract_frontmatter_value_returns_none_for_empty_value ... ok
test context::current_state::tests::extract_frontmatter_value_returns_none_for_wrong_key ... ok
test context::bootstrap::tests::render_with_no_active_work_packet ... ok
test context::current_state::tests::natural_epic_sort_orders_correctly ... ok
test context::current_state::tests::parse_epic_frontmatter_extracts_fields ... ok
test context::current_state::tests::parse_epic_frontmatter_returns_none_without_epic_id ... ok
test context::current_state::tests::parse_epic_frontmatter_handles_in_progress ... ok
test context::current_state::tests::read_runtime_modules_from_content ... ok
test context::bootstrap::tests::source_of_truth_mentions_repository ... ok
test context::current_state::tests::render_current_state_handles_no_epics ... ok
test context::current_state::tests::render_current_state_includes_required_sections ... ok
test context::current_state::tests::strip_epic_prefix_preserves_title_without_prefix ... ok
test context::current_state::tests::render_current_state_is_deterministic ... ok
test context::current_state::tests::strip_epic_prefix_removes_em_dash_separator ... ok
test context::current_state::tests::strip_epic_prefix_removes_hyphen_separator ... ok
test context::handoff::tests::collapse_hyphens_collapses_multiple ... ok
test context::handoff::tests::epic_filename_generates_slug ... ok
test context::current_state::tests::generate_current_state_from_workspace ... ok
test context::handoff::tests::extract_frontmatter_field_handles_quoted_values ... ok
test context::handoff::tests::extract_frontmatter_field_handles_unquoted_values ... ok
test context::handoff::tests::extract_frontmatter_field_returns_none_for_empty_value ... ok
test context::handoff::tests::extract_frontmatter_field_returns_none_for_wrong_key ... ok
test context::handoff::tests::handoff_artifact_finds_active_work_packet ... ok
test context::handoff::tests::handoff_artifact_returns_none_when_no_active_work_packet ... ok
test context::handoff::tests::natural_wp_sort_orders_correctly ... ok
test context::handoff::tests::parse_work_packet_frontmatter_extracts_fields ... ok
test context::current_state::tests::write_and_read_current_state_artifact ... ok
test context::handoff::tests::parse_work_packet_frontmatter_handles_complete_status ... ok
test context::handoff::tests::parse_work_packet_frontmatter_returns_none_without_id ... ok
test context::handoff::tests::render_handoff_handles_no_active_work ... ok
test context::handoff::tests::render_handoff_is_deterministic ... ok
test context::handoff::tests::render_handoff_next_action_suggests_pending_when_none_active ... ok
test context::handoff::tests::render_handoff_includes_required_sections ... ok
test context::handoff::tests::strip_wp_prefix_removes_em_dash ... ok
test context::handoff::tests::strip_wp_prefix_removes_hyphen ... ok
test context::pack::tests::default_file_order_is_stable ... ok
test context::handoff::tests::strip_wp_prefix_preserves_title_without_prefix ... ok
test context::pack::tests::context_pack_work_packet_counts_are_correct ... ok
test context::pack::tests::context_pack_all_required_sections_present ... ok
test context::pack::tests::extract_adr_title_from_heading ... ok
test context::pack::tests::extract_adr_title_returns_none_for_empty ... ok
test context::pack::tests::extract_first_paragraph_handles_no_frontmatter ... ok
test context::pack::tests::extract_first_paragraph_joins_multi_line ... ok
test context::pack::tests::extract_first_paragraph_returns_empty_for_empty_content ... ok
test context::pack::tests::extract_first_paragraph_skips_frontmatter ... ok
test context::pack::tests::render_context_pack_empty_architecture_summary ... ok
test context::pack::tests::render_context_pack_empty_workflow_summary ... ok
test context::pack::tests::render_context_pack_includes_accepted_decisions ... ok
test context::pack::tests::render_context_pack_empty_decisions ... ok
test context::pack::tests::render_context_pack_includes_architecture_summary ... ok
test context::pack::tests::extract_adr_title_from_frontmatter ... ok
test context::pack::tests::render_context_pack_includes_current_status ... ok
test context::pack::tests::render_context_pack_includes_frontmatter ... ok
test context::pack::tests::render_context_pack_includes_important_documents ... ok
test context::pack::tests::render_context_pack_includes_active_work ... ok
test context::pack::tests::render_context_pack_includes_next_recommended_action ... ok
test context::pack::tests::render_context_pack_includes_project_identity ... ok
test context::pack::tests::render_context_pack_includes_risks_and_blockers ... ok
test context::handoff::tests::generate_handoff_from_workspace ... ok
test context::pack::tests::render_context_pack_includes_source_files ... ok
test context::pack::tests::render_context_pack_includes_trust_notes ... ok
test context::pack::tests::render_context_pack_includes_verification_summary ... ok
test context::pack::tests::render_context_pack_includes_workflow_summary ... ok
test context::pack::tests::render_context_pack_is_deterministic ... ok
test context::pack::tests::render_context_pack_next_action_when_all_complete ... ok
test context::verify::tests::all_headings_present_returns_empty ... ok
test context::verify::tests::expected_context_files_includes_required_and_optional ... ok
test context::pack::tests::render_context_pack_next_action_when_none_active ... ok
test context::verify::tests::expected_context_files_returns_five_entries ... ok
test context::verify::tests::frontmatter_not_detected_for_empty_content ... ok
test context::verify::tests::frontmatter_detected_when_present ... ok
test context::verify::tests::frontmatter_detected_with_leading_blank_lines ... ok
test context::verify::tests::frontmatter_not_detected_when_absent ... ok
test context::verify::tests::missing_heading_is_reported ... ok
test context::verify::tests::no_expected_headings_returns_empty ... ok
test context::verify::tests::render_summary_shows_failed_when_errors_exist ... ok
test context::verify::tests::render_summary_shows_passed_when_no_errors ... ok
test context::verify::tests::render_summary_shows_warning_indicators ... ok
test context::verify::tests::report_to_diagnostic_report_collects_all_diagnostics ... ok
test context::verify::tests::verify_context_from_cargo_manifest_dir ... ok
test context::bootstrap::tests::generate_bootstrap_prompt_from_workspace_produces_artifact ... ok
test context::verify::tests::verify_context_reports_all_missing_files ... ok
test dependency_detection::tests::dependency_detection_is_empty_without_known_signals ... ok
test dependency_detection::tests::dependency_signal_kind_labels_are_stable ... ok
test context::verify::tests::verify_context_passes_with_all_files_present ... ok
test context::verify::tests::verify_context_warns_on_missing_frontmatter ... ok
test diagnostics::tests::diagnostic_renders_as_single_line_message ... ok
test diagnostics::tests::report_knows_when_it_contains_errors ... ok
test context::verify::tests::verify_context_warns_on_missing_headings ... ok
test diagnostics::tests::severity_labels_are_stable ... ok
test error::tests::invalid_input_has_stable_code_and_message ... ok
test context::pack::tests::generate_context_pack_from_workspace_produces_artifact ... ok
test error::tests::monad_result_alias_can_return_success_or_error ... ok
test error::tests::not_found_names_missing_resource ... ok
test error::tests::verification_failed_converts_to_error_diagnostic ... ok
test context::verify::tests::verify_context_passes_with_all_required_files ... ok
test evolution::context_baseline::tests::context_baseline_plan_contains_core_context_targets ... ok
test evolution::context_baseline::tests::context_baseline_dry_run_previews_creates_when_targets_are_missing ... ok
test evolution::context_baseline::tests::context_baseline_dry_run_output_states_no_files_or_ai_work ... ok
test evolution::context_baseline::tests::context_baseline_dry_run_detects_existing_file_conflicts ... ok
test evolution::verify_baseline::tests::verify_baseline_dry_run_output_states_no_files_written ... ok
test evolution::verify_baseline::tests::verify_baseline_dry_run_detects_existing_file_conflict ... ok
test evolution::verify_baseline::tests::verify_baseline_dry_run_previews_create_when_target_is_missing ... ok
test exec::command::tests::command_runner_rejects_empty_program ... ok
test evolution::verify_baseline::tests::verify_baseline_plan_uses_embedded_template_target ... ok
test exec::command::tests::command_spec_builds_display_command ... ok
test exec::result::tests::command_result_can_represent_failure ... ok
test exec::result::tests::command_result_exposes_execution_fields ... ok
test dependency_detection::tests::dependency_signal_kind_counts_are_stable ... ok
test dependency_detection::tests::dependency_paths_can_be_grouped_by_toolchain ... ok
test dependency_detection::tests::dependency_toolchain_counts_are_stable ... ok
test file_ops::dry_run::tests::dry_run_detects_create_conflict_when_file_exists ... ok
test file_ops::model::tests::create_operation_is_content_write ... ok
test file_ops::dry_run::tests::dry_run_detects_update_conflict_when_file_is_missing ... ok
test file_ops::model::tests::delete_operation_can_require_approval ... ok
test file_ops::dry_run::tests::dry_run_previews_update_when_file_exists ... ok
test file_ops::model::tests::operation_kind_labels_are_stable ... ok
test file_ops::model::tests::skip_and_conflict_are_reviewable_states ... ok
test file_ops::model::tests::target_preserves_path ... ok
test file_ops::model::tests::update_operation_is_content_write ... ok
test file_ops::dry_run::tests::dry_run_summary_counts_outcomes ... ok
test file_ops::dry_run::tests::dry_run_previews_create_when_file_is_missing ... ok
test file_ops::plan::tests::plan_starts_empty ... ok
test file_ops::plan::tests::plan_can_be_built_incrementally ... ok
test file_ops::plan::tests::plan_preserves_operation_order ... ok
test exec::command::tests::command_runner_captures_success_stdout_and_exit_code ... ok
test file_ops::plan::tests::plan_summary_counts_operation_kinds ... ok
test file_ops::report::tests::dry_run_report_renders_operations_and_conflicts ... ok
test git::status::tests::parses_branch_without_remote_tracking ... ok
test git::status::tests::parses_clean_branch_status ... ok
test dependency_detection::tests::detects_dependency_signals_for_common_toolchains ... ok
test git::status::tests::parses_detached_head_as_isolation_required ... ok
test manifest::tests::current_schema_version_is_supported ... ok
test git::status::tests::parses_dirty_status_counts ... ok
test manifest::tests::default_manifest_matches_monad_runtime_shape ... ok
test manifest::tests::future_schema_version_is_not_supported ... ok
test file_ops::report::tests::dry_run_report_renders_empty_plan ... ok
test exec::command::tests::command_runner_captures_failure_stderr_and_exit_code ... ok
test manifest::tests::invalid_toml_returns_invalid_input_error ... ok
test manifest::tests::empty_project_name_fails_validation ... ok
test manifest::tests::manifest_parses_from_toml_string ... ok
test manifest::tests::valid_default_manifest_has_no_error_diagnostics ... ok
test manifest::tests::missing_manifest_path_returns_not_found_error ... ok
test manifest::tests::unsupported_schema_version_fails_validation ... ok
test output::tests::output_format_parses_text_and_json ... ok
test output::tests::diagnostic_report_renders_as_text_lines ... ok
test manifest::tests::manifest_loads_from_workspace_context ... ok
test manifest::tests::manifest_loads_from_path ... ok
test output::tests::repository_inspection_summary_includes_policy_metrics ... ok
test output::tests::repository_inspection_summary_type_defaults_policy_to_empty ... ok
test output::tests::repository_role_enum_is_still_available_for_future_output_work ... ok
test output::tests::unsupported_output_format_returns_error ... ok
test output::tests::workspace_summary_renders_like_info_command ... ok
test policy::approval::tests::approval_decision_records_actor_reason_and_result ... ok
test policy::approval::tests::approval_gate_kind_labels_are_stable ... ok
test output::tests::repository_inspection_summary_renders_policy_as_json ... ok
test policy::approval::tests::approval_gate_preserves_action_and_requirement ... ok
test policy::approval::tests::approval_kind_identifies_required_approval ... ok
test policy::audit::tests::action_proposed_event_records_gate_metadata ... ok
test policy::audit::tests::approval_decision_event_uses_decision_kind ... ok
test policy::audit::tests::audit_event_kind_labels_are_stable ... ok
test policy::audit::tests::audit_log_detects_approval_for_gate ... ok
test policy::audit::tests::audit_log_records_events_and_filters_by_subject ... ok
test policy::audit::tests::local_write_gate_helper_creates_initial_audit_trail ... ok
test repo_contract::tests::initial_contract_contains_expected_paths ... ok
test repository_context_pack::tests::context_pack_default_export_dir_is_repository_local_and_deterministic ... ok
test repository_context_pack::tests::context_pack_render_format_parses_supported_formats ... ok
test repository_context_pack::tests::context_pack_render_format_rejects_unsupported_formats ... ok
test output::tests::repository_inspection_summary_renders_policy_as_text ... ok
test repository_context_pack::tests::exported_file_records_capture_format_path_and_bytes ... ok
test repository_context_pack::tests::section_kind_labels_are_stable ... ok
test repo_contract::tests::contract_passes_for_valid_workspace_shape ... ok
test repo_contract::tests::contract_reports_missing_directory ... ok
test repository_context_pack::tests::context_pack_contains_expected_sections ... ok
test repository_context_pack::tests::context_pack_reports_policy_warnings ... ok
test repository_context_pack::tests::context_pack_exposes_facts_by_section_and_key ... ok
test repository_graph::tests::graph_edges_connect_parent_child_relationships ... ok
test repository_graph::tests::graph_contains_root_and_traversed_entries ... ok
test repository_context_pack::tests::context_pack_renders_as_markdown ... ok
test repository_graph::tests::graph_render_format_parses_supported_formats ... ok
test repository_graph::tests::graph_render_format_rejects_unsupported_formats ... ok
test repository_context_pack::tests::context_pack_renders_as_json ... ok
test checks::adapters::tests::selected_adapter_checks_include_rust_when_rust_detected ... ok
test checks::adapters::rust::tests::rust_manifest_check_fails_when_root_manifest_is_missing ... ok
test repository_graph::tests::graph_exposes_category_and_decision_counts ... ok
test repository_graph::tests::graph_renders_as_dot ... ok
test repository_context_pack::tests::context_pack_exports_markdown_and_json_files ... ok
test repository_graph::tests::graph_renders_as_text ... ok
test repository_graph::tests::graph_renders_as_mermaid ... ok
test repository_graph::tests::graph_renders_as_json ... ok
test repository_graph::tests::graph_rendering_is_deterministic ... ok
test repository_inspection::tests::bounded_traversal_respects_max_depth ... ok
test repository_inspection::tests::repository_entry_roles_map_to_stable_categories ... ok
test repository_graph::tests::graph_output_is_deterministically_ordered ... ok
test repository_inspection::tests::traversal_guardrails_are_conservative_by_default ... ok
test repository_policy::tests::policy_report_is_empty_when_no_diagnostics_are_supplied ... ok
test repository_inspection::tests::bounded_traversal_output_is_deterministic ... ok
test checks::run::tests::workspace_checks_report_missing_files ... ok
test repository_policy::tests::policy_severity_labels_are_stable ... ok
test templates::model::tests::embedded_template_definition_contains_metadata_and_content ... ok
test repository_inspection::tests::bounded_traversal_respects_simple_root_gitignore_patterns ... ok
test templates::model::tests::metadata_exposes_template_fields ... ok
test repository_policy::tests::policy_reports_missing_readme_and_license ... ok
test templates::model::tests::source_kind_label_is_stable ... ok
test repository_inspection::tests::bounded_traversal_walks_safe_directories ... ok
test templates::model::tests::template_id_preserves_value ... ok
test repository_policy::tests::policy_report_counts_by_severity ... ok
test repository_policy::tests::policy_reports_traversal_safety_information ... ok
test repository_policy::tests::policy_warns_when_manifest_has_no_lockfile ... ok
test templates::registry::tests::initial_registry_contains_baseline_templates ... ok
test templates::registry::tests::registry_registers_and_retrieves_template ... ok
test templates::registry::tests::registry_lists_templates_in_deterministic_order ... ok
test templates::registry::tests::registry_rejects_duplicate_template_ids ... ok
test templates::registry::tests::registry_starts_empty ... ok
test tests::checked_runtime_identity_uses_monad_result ... ok
test tests::output_format_is_exported_from_core_root ... ok
test tests::repository_contract_is_exported_from_core_root ... ok
test repository_inspection::tests::repository_inspection_lists_top_level_entries ... ok
test tests::repository_dependency_detection_types_are_exported_from_core_root ... ok
test tests::repository_entry_category_is_exported_from_core_root ... ok
test tests::repository_graph_render_format_is_exported_from_core_root ... ok
test tests::repository_graph_types_are_exported_from_core_root ... ok
test tests::repository_inspection_summary_type_is_exported_from_core_root ... ok
test tests::repository_inspection_types_are_exported_from_core_root ... ok
test tests::repository_policy_types_are_exported_from_core_root ... ok
test tests::repository_toolchain_detection_types_are_exported_from_core_root ... ok
test tests::runtime_banner_is_human_readable ... ok
test tests::runtime_identity_can_build_default_manifest ... ok
test repository_inspection::tests::bounded_traversal_skips_generated_or_external_directories ... ok
test tests::runtime_identity_names_monad ... ok
test tests::runtime_identity_can_produce_startup_diagnostic ... ok
test tests::traversal_planning_types_are_exported_from_core_root ... ok
test tests::workspace_checks_are_exported_from_core_root ... ok
test tests::workspace_context_is_exported_from_core_root ... ok
test toolchain_detection::tests::signal_kind_labels_are_stable ... ok
test repository_inspection::tests::traversal_plan_is_built_from_shallow_inspection ... ok
test toolchain_detection::tests::toolchain_kind_labels_are_stable ... ok
test workspace::tests::workspace_context_builds_standard_paths ... ok
test toolchain_detection::tests::detection_is_empty_when_no_known_signals_exist ... ok
test workspace::tests::workspace_discovery_reports_not_found ... ok
test workspace::tests::workspace_discovery_finds_root_from_nested_directory ... ok
test toolchain_detection::tests::detects_common_repository_toolchains ... ok
test toolchain_detection::tests::signal_paths_can_be_grouped_by_toolchain ... ok
test toolchain_detection::tests::signal_kind_counts_are_stable_and_machine_readable ... ok
test toolchain_detection::tests::toolchain_counts_are_stable_and_machine_readable ... ok

test result: ok. 315 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running unittests src/lib.rs (target/debug/deps/monad_mcp-8004afe4f5cd0192)

running 4 tests
test tests::capability_kind_labels_are_stable ... ok
test tests::safety_class_labels_are_stable ... ok
test tests::initial_capabilities_include_allowed_and_guarded_descriptors ... ok
test tests::initial_allowed_capabilities_exclude_approval_gated_and_forbidden_items ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests monad_core

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests monad_mcp

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

==> Running CLI info smoke test
==> Running CLI info JSON smoke test
==> Running CLI check smoke test
==> Running CLI check JSON smoke test
==> Running CLI inspect smoke test
==> Running CLI inspect JSON smoke test
==> Running CLI graph smoke test
==> Running CLI graph JSON smoke test
==> Running CLI graph Mermaid smoke test
==> Running CLI graph DOT smoke test
==> Running CLI context smoke test
==> Running CLI context Markdown smoke test
==> Running CLI context JSON smoke test
==> Running CLI context md alias smoke test
==> Running CLI context text alias smoke test
==> Running CLI context write smoke test
Verification baseline passed.

==> Verifying no-write planning and dry-run commands
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/monad plan 'explain this repository'`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/monad evolve verify-baseline --dry-run`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/monad evolve context-baseline --dry-run`
No-write command verification passed.
```

</details>


### 17. git-status-after

```bash
git status --short
```

**Result:** Pass

**Exit code:** 0

**Raw log:** `.monad/reports/e10/wp-e10-005/git-status-after.log`

<details>
<summary>Command output</summary>

```text
?? complete_wp_e10_005_public_prerelease_verification_audit.sh
```

</details>


## Public Pre-Release Boundary

The verification audit assumes the public pre-release posture remains source-only.

The following remain out of scope:

- packaged binary release;
- installer release;
- Crates.io/package-manager publication;
- hosted service;
- SaaS launch;
- autonomous agent runtime;
- production-ready platform claim.

## Outcome

The outcome of this audit must be used by WP-E10-006.

If all checks pass, WP-E10-006 may decide whether to cut the source-only public pre-release tag.

If any checks fail, WP-E10-006 should either defer the public pre-release or document an explicit blocker-resolution path.
