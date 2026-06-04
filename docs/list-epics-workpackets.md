---

---

# List of Epics and Workpackets

| Epic Name                                                                    | Description                                                                                                                                                    |
| -----------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|   E0 — Project Foundation                                                    | Establish the repository, documentation, workflow, context bridge, product canon, ADRs, GitHub workflow, and verification baseline.                            |
|   E1 — Rust Core Foundation                                                  | Create the Rust workspace, CLI shell, core library boundaries, diagnostics, manifest foundation, and Rust learning/verification baseline.                      |
|   E2 — Repo Intelligence                                                     | Teach Monad to inspect repositories, detect toolchains/manifests, and build the first project graph.                                                           |
|   E3 — Context Bridge                                                        | Generate durable repo-native context artifacts: current state, handoffs, context packs, bootstrap prompts, and context verification.                           |
|   E4 — Verification Engine                                                   | Add repeatable checks, command execution, evidence packets, adapter-specific checks, and JSON verification output.                                             |
|   E5 — Evolution Engine                                                      | Establish safe file operations, dry-run planning, template registry, and baseline evolution commands.                                                          |
|   E6 — Agent Supervision                                                     | Define supervised agent workflows, provider abstraction, planning, draft sandboxes, approval gates, audit logs, and MCP foundations.                           |
|   E7 — MVP Hardening                                                         | Stabilize E0–E6 into a coherent local-first CLI MVP with accurate docs, smoke tests, and dry-run guarantees.                                                   |
|   E8 — MVP Candidate Cut and Release Preparation                             | Freeze MVP candidate scope, prepare changelog/release metadata, document installation, audit verification, and cut an internal candidate tag.                  |
|   E9 — Post-MVP Candidate Stabilization and Public-Readiness Gap Closure     | Close public-readiness gaps, harden generated artifact policy, stabilize context freshness, review repository hygiene, and decide public pre-release boundary. |
|  E10 — Public Pre-Release Hardening and Boundary Enforcement                 | Audit claims, create public release evidence, decide distribution posture, and make the go/no-go public pre-release decision.                                  |
|  E11 — Init Command and Monorepo Scaffold Foundation                         | Implement the first safe `monad init` capability for creating a minimal Monad-managed monorepo skeleton.                                                       |
|  E12 — Component Add and Polyglot Scaffold Foundation                        | Implement `monad add` for safely adding apps, packages, services, libraries, and minimal polyglot components.                                                  |
|  E13 — Task Execution and Native Tool Orchestration Foundation               | Implement `monad run` to discover, plan, and execute native-tool tasks safely.                                                                                 |
|  E14 — Manifest Sync and Repository Contract Foundation                      | Implement `monad sync` to compare declared repository intent against actual repository state and produce safe sync plans.                                      |
|  E15 — Doctor and Environment Diagnostics Foundation                         | Implement `monad doctor` to diagnose local tools, repo health, manifests, context state, and environment readiness.                                            |
|  E16 — Release and Distribution Foundation                                   | Implement `monad release` planning, release readiness checks, tag/version validation, artifacts, checksums, and release evidence.                              |
|  E17 — Upgrade and Repository Evolution Foundation                           | Implement `monad upgrade` to safely evolve existing Monad-managed repos through versioned, non-destructive upgrade plans.                                      |
|  E18 — AI Context Memory and Provider-Agnostic Assistant Workflow Foundation | Add repo-native memory, provider-agnostic AI configuration, context snapshots, and assistant handoff/export workflows.                                         |
|  E19 — Policy, Safety, and Approval Gate Foundation                          | Add operation classification, policy checks, approval evidence, and gated write/apply foundations.                                                             |
|  E20 — Patch Planning and Supervised Apply Foundation                        | Add safe patch/change planning and supervised apply under E19 policy gates.                                                                                    |
|  E21 — Work Packet Execution Workflow Foundation                             | Teach Monad to understand, plan, verify, and close work packets as executable repo-native work units.                                                          |
|  E22 — Repo Contract Schema and Validation Foundation                        | Formalize `monad.toml`, `monad.lock`, repo contract schemas, validation, and migration paths.                                                                  |
|  E23 — Language Adapter Foundation                                           | Add first-class adapter boundaries for Rust, Node/Bun, Python, Go, Java, and future ecosystems.                                                                |
|  E24 — LSP and Static Analysis Foundation                                    | Introduce language-aware code understanding through LSP, parsers, symbols, and static analysis.                                                                |
|  E25 — Dependency Graph and Impact Analysis Foundation                       | Expand graphing into real dependency and change-impact analysis.                                                                                               |
|  E26 — Test Intelligence and Verification Planning Foundation                | Map components to tests and recommend targeted verification plans.                                                                                             |
|  E27 — Build Cache and Incremental Execution Foundation                      | Add cache-aware task planning and incremental execution metadata.                                                                                              |
|  E28 — Local Artifact and Report Store Foundation                            | Standardize `.monad/reports`, `.monad/artifacts`, evidence logs, summaries, and generated outputs.                                                             |
|  E29 — Template Registry and Preset Evolution Foundation                     | Expand safe built-in templates and presets while preserving deterministic generation.                                                                          |
|  E30 — Plugin and Extension System Foundation                                | Add governed plugin interfaces for adapters, templates, providers, policies, and future commands.                                                              |
|  E31 — MCP and External Tool Integration Foundation                          | Add MCP-style integration surfaces for exposing/consuming external tools safely.                                                                               |
|  E32 — Local AI Retrieval and Vector Memory Foundation                       | Add optional local retrieval/vector memory over repo context and memory records.                                                                               |
|  E33 — Agent Workflow Sandbox Foundation                                     | Prepare a local sandbox model for future supervised agent actions.                                                                                             |
|  E34 — Interactive Workbench / TUI Foundation                                | Add terminal UI navigation for plans, checks, reports, context, and approvals.                                                                                 |
|  E35 — Web Workbench Foundation                                              | Add a local web UI for repo visualization, work packets, reports, approvals, and context.                                                                      |
|  E36 — GitHub Integration and PR Workflow Foundation                         | Generate issues, branches, PR descriptions, review packs, closeout evidence, and GitHub workflow helpers.                                                      |
|  E37 — Remote Execution and CI Parity Foundation                             | Align local Monad execution with CI runners and reproducible remote execution.                                                                                 |
|  E38 — Release Channel and Installer Ecosystem Foundation                    | Add cross-platform releases, installers, checksums, signing, and distribution channels.                                                                        |
|  E39 — Security and Supply Chain Hardening Foundation                        | Add SBOM, signing, dependency audit, secret checks, provenance, and supply-chain policy.                                                                       |
|  E40 — Governance and Compliance Evidence Foundation                         | Add governance-grade audit trails, ADR traceability, release attestations, and compliance evidence.                                                            |
|  E41 — Team and Multi-User Workflow Foundation                               | Add team roles, shared approvals, assignment, review flows, and collaboration metadata.                                                                        |
|  E42 — Cloud/Hosted Control Plane Exploration                                | Explore optional hosted services without undermining local-first operation.                                                                                    |
|  E43 — Enterprise Policy and Organization Standards Foundation               | Add organization-level templates, standards, policy packs, and reusable governance baselines.                                                                  |
|  E44 — Marketplace and Community Template Ecosystem                          | Enable shared templates, adapters, presets, policies, and workflows.                                                                                           |
|  E45 — Self-Seeding Monad Evolution Foundation                               | Make Monad able to describe, regenerate, verify, and evolve its own architecture and repo standards.                                                           |

## 2. Work Packets by Epic, E0–E45

### E0 — Project Foundation

1. WP-E0-001 — Establish repository foundation
2. WP-E0-002 — Establish documentation architecture
3. WP-E0-003 — Establish context bridge foundation
4. WP-E0-004 — Establish workflow standards
5. WP-E0-005 — Establish initial product canon
6. WP-E0-006 — Establish ADR foundation
7. WP-E0-007 — Establish GitHub project and issue workflow
8. WP-E0-008 — Establish verification baseline

### E1 — Rust Core Foundation

1. WP-E1-001 — Create Rust workspace crates
2. WP-E1-002 — Add CLI shell
3. WP-E1-003 — Add core error and diagnostic model
4. WP-E1-004 — Add workspace context resolver
5. WP-E1-005 — Add `monad.toml` manifest foundation
6. WP-E1-006 — Add Rust verification and learning baseline

### E2 — Repo Intelligence

1. WP-E2-001 — Add toolchain detection model
2. WP-E2-002 — Detect Node and JavaScript package managers
3. WP-E2-003 — Detect Rust Cargo workspaces
4. WP-E2-004 — Add inspect command report
5. WP-E2-005 — Add basic project graph model
6. WP-E2-006 — Add graph output formats

### E3 — Context Bridge

1. WP-E3-001 — Define context artifact schemas
2. WP-E3-002 — Implement current-state generator
3. WP-E3-003 — Implement handoff generator
4. WP-E3-004 — Implement context pack assembler
5. WP-E3-005 — Implement bootstrap prompt generator
6. WP-E3-006 — Add context verification checks

### E4 — Verification Engine

1. WP-E4-001 — Define check registry and result model
2. WP-E4-002 — Add command runner
3. WP-E4-003 — Add `monad-workspace` check command
4. WP-E4-004 — Add evidence packet report
5. WP-E4-005 — Add adapter-specific checks
6. WP-E4-006 — Add JSON verification output

### E5 — Evolution Engine

1. WP-E5-001 — Define safe file operation model
2. WP-E5-002 — Add dry-run and diff planner
3. WP-E5-003 — Add template registry foundation
4. WP-E5-004 — Add evolve verify-baseline command
5. WP-E5-005 — Add evolve context-baseline command
6. WP-E5-006 — Add worktree and branch safety strategy

### E6 — Agent Supervision

1. WP-E6-001 — Define supervised agent workflow
2. WP-E6-002 — Add model provider abstraction
3. WP-E6-003 — Add plan command
4. WP-E6-004 — Add draft sandbox workflow
5. WP-E6-005 — Add approval gates and audit log
6. WP-E6-006 — Add MCP integration foundation

### E7 — MVP Hardening

1. WP-E7-001 — Run foundation closure audit
2. WP-E7-002 — Normalize CLI help and command UX
3. WP-E7-003 — Harden command smoke tests
4. WP-E7-004 — Align documentation with implemented behavior
5. WP-E7-005 — Harden dry-run and no-write guarantees
6. WP-E7-006 — Create MVP readiness report

### E8 — MVP Candidate Cut and Release Preparation

1. WP-E8-001 — Freeze MVP candidate scope
2. WP-E8-002 — Add changelog and release notes foundation
3. WP-E8-003 — Harden version and build metadata
4. WP-E8-004 — Add installation and local build documentation
5. WP-E8-005 — Run release-candidate verification audit
6. WP-E8-006 — Cut internal MVP candidate tag

### E9 — Post-MVP Candidate Stabilization and Public-Readiness Gap Closure

1. WP-E9-001 — Audit MVP candidate gaps against public-readiness criteria
2. WP-E9-002 — Harden generated artifact and ignore policies
3. WP-E9-003 — Stabilize context-generation freshness and release metadata
4. WP-E9-004 — Add public pre-release readiness checklist
5. WP-E9-005 — Review licensing, contribution, and repository hygiene
6. WP-E9-006 — Decide first public pre-release boundary

### E10 — Public Pre-Release Hardening and Boundary Enforcement

1. WP-E10-001 — Audit README and public claims against implemented capability
2. WP-E10-002 — Convert public pre-release checklist into pass/fail evidence
3. WP-E10-003 — Decide source-only versus packaged pre-release posture
4. WP-E10-004 — Draft public pre-release notes
5. WP-E10-005 — Run final public pre-release verification audit
6. WP-E10-006 — Decide and cut first public pre-release tag, if approved

### E11 — Init Command and Monorepo Scaffold Foundation

1. WP-E11-001 — Define `monad init` UX and safety contract
2. WP-E11-002 — Add init dry-run plan
3. WP-E11-003 — Add minimal embedded scaffold templates
4. WP-E11-004 — Add guarded init write path
5. WP-E11-005 — Add basic/polyglot-minimal preset
6. WP-E11-006 — Add init smoke tests and verification evidence

### E12 — Component Add and Polyglot Scaffold Foundation

1. WP-E12-001 — Define `monad add` UX and component taxonomy
2. WP-E12-002 — Add component scaffold plan model
3. WP-E12-003 — Add app/package/service/library scaffold templates
4. WP-E12-004 — Add `monad add --dry-run` command path
5. WP-E12-005 — Add guarded component scaffold write path
6. WP-E12-006 — Add polyglot-minimal component presets and smoke tests

### E13 — Task Execution and Native Tool Orchestration Foundation

1. WP-E13-001 — Define task model and native-tool coordination contract
2. WP-E13-002 — Add task discovery from manifests and `monad.toml`
3. WP-E13-003 — Add `monad run --dry-run` plan output
4. WP-E13-004 — Add guarded command execution runner
5. WP-E13-005 — Add package/component filtering and graph-aware ordering
6. WP-E13-006 — Add task evidence reports and smoke tests

### E14 — Manifest Sync and Repository Contract Foundation

1. WP-E14-001 — Define `monad sync` contract and repo intent model
2. WP-E14-002 — Add repository contract diff model
3. WP-E14-003 — Add `monad sync --dry-run` plan output
4. WP-E14-004 — Add non-destructive manifest/context sync writes
5. WP-E14-005 — Add native manifest reconciliation checks
6. WP-E14-006 — Add sync evidence reports and smoke tests

### E15 — Doctor and Environment Diagnostics Foundation

1. WP-E15-001 — Define `monad doctor` diagnostic contract
2. WP-E15-002 — Add local tool detection foundation
3. WP-E15-003 — Add Rust, Git, and repository readiness diagnostics
4. WP-E15-004 — Add ecosystem diagnostics for Node/Bun/Python/Go/Java
5. WP-E15-005 — Add Monad context and repo contract diagnostics
6. WP-E15-006 — Add doctor report output and smoke tests

### E16 — Release and Distribution Foundation

1. WP-E16-001 — Define `monad release` contract and release boundary
2. WP-E16-002 — Add release readiness model and go/no-go plan
3. WP-E16-003 — Add version and tag validation
4. WP-E16-004 — Add binary artifact packaging and checksums
5. WP-E16-005 — Add release notes and changelog validation
6. WP-E16-006 — Add GitHub release draft workflow and release evidence tests

### E17 — Upgrade and Repository Evolution Foundation

1. WP-E17-001 — Define `monad upgrade` contract and safety model
2. WP-E17-002 — Add repository version and upgrade target model
3. WP-E17-003 — Add upgrade dry-run plan output
4. WP-E17-004 — Add upgrade step registry foundation
5. WP-E17-005 — Add guarded non-destructive upgrade writes
6. WP-E17-006 — Add upgrade evidence reports and smoke tests

### E18 — AI Context Memory and Provider-Agnostic Assistant Workflow Foundation

1. WP-E18-001 — Define provider-agnostic AI workflow and memory contract
2. WP-E18-002 — Add AI provider configuration model
3. WP-E18-003 — Add repo-native memory record schema
4. WP-E18-004 — Add context snapshot and work-packet planning artifacts
5. WP-E18-005 — Add supervised assistant handoff/export workflow
6. WP-E18-006 — Add AI context verification and smoke tests

### E19 — Policy, Safety, and Approval Gate Foundation

1. WP-E19-001 — Define policy and approval-gate contract
2. WP-E19-002 — Add operation classification and risk model
3. WP-E19-003 — Add approval plan and approval evidence model
4. WP-E19-004 — Add policy checks for file operations and command execution
5. WP-E19-005 — Add gated write/apply foundation
6. WP-E19-006 — Add policy reports and smoke tests

### E20 — Patch Planning and Supervised Apply Foundation

1. WP-E20-001 — Define patch planning and supervised apply contract
2. WP-E20-002 — Add change-set and patch diff model
3. WP-E20-003 — Add patch dry-run plan output
4. WP-E20-004 — Add patch validation and conflict checks
5. WP-E20-005 — Add supervised apply under approval gates
6. WP-E20-006 — Add patch evidence reports and smoke tests

### E21 — Work Packet Execution Workflow Foundation

1. WP-E21-001 — Define work-packet execution model
2. WP-E21-002 — Add work-packet metadata parser
3. WP-E21-003 — Add work-packet implementation plan generator
4. WP-E21-004 — Add verification and evidence checklist automation
5. WP-E21-005 — Add closeout and handoff record generation
6. WP-E21-006 — Add work-packet workflow smoke tests

### E22 — Repo Contract Schema and Validation Foundation

1. WP-E22-001 — Define repository contract schema boundary
2. WP-E22-002 — Add `monad.toml` schema validation
3. WP-E22-003 — Add `monad.lock` / generated state model
4. WP-E22-004 — Add schema migration planning model
5. WP-E22-005 — Add contract validation fixtures
6. WP-E22-006 — Add contract validation reports and smoke tests

### E23 — Language Adapter Foundation

1. WP-E23-001 — Define language adapter interface contract
2. WP-E23-002 — Add Rust adapter foundation
3. WP-E23-003 — Add Node/Bun adapter foundation
4. WP-E23-004 — Add Python adapter foundation
5. WP-E23-005 — Add Go and Java adapter foundations
6. WP-E23-006 — Add adapter registry tests and documentation

### E24 — LSP and Static Analysis Foundation

1. WP-E24-001 — Define static-analysis and symbol model
2. WP-E24-002 — Add parser abstraction foundation
3. WP-E24-003 — Add LSP discovery and capability model
4. WP-E24-004 — Add symbol extraction proof of concept
5. WP-E24-005 — Add source map and ownership metadata
6. WP-E24-006 — Add static-analysis report tests

### E25 — Dependency Graph and Impact Analysis Foundation

1. WP-E25-001 — Define dependency and impact graph model
2. WP-E25-002 — Add component dependency edge detection
3. WP-E25-003 — Add task-to-component graph linkage
4. WP-E25-004 — Add changed-file impact analysis
5. WP-E25-005 — Add impacted verification recommendation output
6. WP-E25-006 — Add graph/impact fixtures and smoke tests

### E26 — Test Intelligence and Verification Planning Foundation

1. WP-E26-001 — Define test intelligence model
2. WP-E26-002 — Discover test commands from manifests
3. WP-E26-003 — Map tests to components/packages
4. WP-E26-004 — Generate targeted verification plans
5. WP-E26-005 — Add verification confidence/evidence model
6. WP-E26-006 — Add verification planner smoke tests

### E27 — Build Cache and Incremental Execution Foundation

1. WP-E27-001 — Define cache and incremental execution contract
2. WP-E27-002 — Add task fingerprint model
3. WP-E27-003 — Add local execution metadata store
4. WP-E27-004 — Add cache-aware dry-run planning
5. WP-E27-005 — Add incremental execution proof of concept
6. WP-E27-006 — Add cache evidence and invalidation tests

### E28 — Local Artifact and Report Store Foundation

1. WP-E28-001 — Define `.monad/reports` and `.monad/artifacts` contract
2. WP-E28-002 — Add report metadata schema
3. WP-E28-003 — Add artifact metadata schema
4. WP-E28-004 — Add report writing and retention policy
5. WP-E28-005 — Add report index and lookup command foundation
6. WP-E28-006 — Add artifact/report store smoke tests

### E29 — Template Registry and Preset Evolution Foundation

1. WP-E29-001 — Define template registry evolution contract
2. WP-E29-002 — Add template metadata schema
3. WP-E29-003 — Add preset metadata schema
4. WP-E29-004 — Add template compatibility validation
5. WP-E29-005 — Add preset upgrade planning
6. WP-E29-006 — Add template registry fixtures and tests

### E30 — Plugin and Extension System Foundation

1. WP-E30-001 — Define plugin boundary and trust model
2. WP-E30-002 — Add plugin manifest schema
3. WP-E30-003 — Add extension point registry foundation
4. WP-E30-004 — Add adapter/plugin loading plan model
5. WP-E30-005 — Add disabled-by-default plugin safety checks
6. WP-E30-006 — Add plugin contract tests and documentation

### E31 — MCP and External Tool Integration Foundation

1. WP-E31-001 — Define MCP integration boundary
2. WP-E31-002 — Add MCP tool capability model
3. WP-E31-003 — Add MCP context export proof of concept
4. WP-E31-004 — Add external tool invocation policy checks
5. WP-E31-005 — Add MCP/local tool documentation
6. WP-E31-006 — Add MCP integration smoke tests

### E32 — Local AI Retrieval and Vector Memory Foundation

1. WP-E32-001 — Define local retrieval and vector memory contract
2. WP-E32-002 — Add document chunking model
3. WP-E32-003 — Add embedding provider abstraction
4. WP-E32-004 — Add local index storage proof of concept
5. WP-E32-005 — Add retrieval query and context assembly model
6. WP-E32-006 — Add retrieval fixtures and smoke tests

### E33 — Agent Workflow Sandbox Foundation

1. WP-E33-001 — Define local sandbox and agent action boundary
2. WP-E33-002 — Add sandbox workspace model
3. WP-E33-003 — Add isolated draft operation planner
4. WP-E33-004 — Add sandbox verification command path
5. WP-E33-005 — Add sandbox promotion/approval model
6. WP-E33-006 — Add sandbox safety tests

### E34 — Interactive Workbench / TUI Foundation

1. WP-E34-001 — Define TUI navigation model
2. WP-E34-002 — Add TUI shell proof of concept
3. WP-E34-003 — Add issue/work-packet view
4. WP-E34-004 — Add plan/report/context viewer
5. WP-E34-005 — Add approval review screen foundation
6. WP-E34-006 — Add TUI smoke tests

### E35 — Web Workbench Foundation

1. WP-E35-001 — Define local web workbench architecture
2. WP-E35-002 — Add local server/API foundation
3. WP-E35-003 — Add repository graph view
4. WP-E35-004 — Add work-packet and report views
5. WP-E35-005 — Add approval/context viewer foundation
6. WP-E35-006 — Add web workbench smoke tests

### E36 — GitHub Integration and PR Workflow Foundation

1. WP-E36-001 — Define GitHub integration boundary
2. WP-E36-002 — Add GitHub issue sync/export model
3. WP-E36-003 — Add branch and PR planning model
4. WP-E36-004 — Add PR description and review-pack generation
5. WP-E36-005 — Add issue closeout/evidence helpers
6. WP-E36-006 — Add GitHub workflow smoke tests

### E37 — Remote Execution and CI Parity Foundation

1. WP-E37-001 — Define CI parity and remote execution contract
2. WP-E37-002 — Add CI environment detection model
3. WP-E37-003 — Add local-vs-CI command mapping
4. WP-E37-004 — Add reproducibility evidence reports
5. WP-E37-005 — Add remote-runner planning model
6. WP-E37-006 — Add CI parity tests and documentation

### E38 — Release Channel and Installer Ecosystem Foundation

1. WP-E38-001 — Define release channel strategy
2. WP-E38-002 — Add multi-platform build matrix
3. WP-E38-003 — Add installer/package strategy docs
4. WP-E38-004 — Add checksum/signing preparation
5. WP-E38-005 — Add GitHub release automation hardening
6. WP-E38-006 — Add release-channel verification tests

### E39 — Security and Supply Chain Hardening Foundation

1. WP-E39-001 — Define security and supply-chain baseline
2. WP-E39-002 — Add dependency audit integration
3. WP-E39-003 — Add secret/check hygiene foundation
4. WP-E39-004 — Add SBOM/provenance preparation
5. WP-E39-005 — Add signing/attestation preparation
6. WP-E39-006 — Add security evidence reports and tests

### E40 — Governance and Compliance Evidence Foundation

1. WP-E40-001 — Define governance evidence model
2. WP-E40-002 — Add ADR traceability checks
3. WP-E40-003 — Add requirement-to-work-packet traceability
4. WP-E40-004 — Add release attestation evidence model
5. WP-E40-005 — Add audit trail report generation
6. WP-E40-006 — Add governance evidence smoke tests

### E41 — Team and Multi-User Workflow Foundation

1. WP-E41-001 — Define team workflow and role model
2. WP-E41-002 — Add assignment/reviewer metadata model
3. WP-E41-003 — Add shared approval workflow foundation
4. WP-E41-004 — Add collaboration evidence records
5. WP-E41-005 — Add contributor handoff/report workflow
6. WP-E41-006 — Add team workflow smoke tests

### E42 — Cloud/Hosted Control Plane Exploration

1. WP-E42-001 — Define hosted control-plane exploration boundary
2. WP-E42-002 — Add local-first/cloud-optional architecture note
3. WP-E42-003 — Add hosted sync/use-case analysis
4. WP-E42-004 — Add tenant/org model exploration
5. WP-E42-005 — Add cost/security/risk analysis
6. WP-E42-006 — Add hosted-control-plane go/no-go report

### E43 — Enterprise Policy and Organization Standards Foundation

1. WP-E43-001 — Define organization standards pack model
2. WP-E43-002 — Add org-level policy schema draft
3. WP-E43-003 — Add reusable repo baseline pack model
4. WP-E43-004 — Add enterprise verification profile model
5. WP-E43-005 — Add org template/preset governance model
6. WP-E43-006 — Add enterprise standards smoke tests

### E44 — Marketplace and Community Template Ecosystem

1. WP-E44-001 — Define marketplace trust and publishing model
2. WP-E44-002 — Add community template metadata schema
3. WP-E44-003 — Add template validation and signing requirements
4. WP-E44-004 — Add adapter/preset discovery model
5. WP-E44-005 — Add marketplace contribution workflow docs
6. WP-E44-006 — Add marketplace safety tests

### E45 — Self-Seeding Monad Evolution Foundation

1. WP-E45-001 — Define Monad self-seeding architecture
2. WP-E45-002 — Add canonical repo blueprint manifest
3. WP-E45-003 — Add self-regeneration planning model
4. WP-E45-004 — Add self-verification and drift detection
5. WP-E45-005 — Add self-upgrade/evolution evidence workflow
6. WP-E45-006 — Add self-seeding closeout and roadmap reset report
