# E21 Work-Packet Workflow Closeout Handoff

Epic: E21 — Work Packet Execution Workflow Foundation

## Work packets
- WP-E21-001 — Define work-packet execution model
- WP-E21-002 — Add work-packet metadata parser
- WP-E21-003 — Add work-packet implementation plan generator
- WP-E21-004 — Add verification and evidence checklist automation
- WP-E21-005 — Add closeout and handoff record generation
- WP-E21-006 — Add work-packet workflow smoke tests

## Verification
- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `tools/scripts/verify-work-packet.sh`
- `tools/scripts/verify-e21.sh`

## Handoff notes
- Record verification commands before closing a work packet.
- Attach or reference generated evidence reports during closeout.
- Capture follow-up risks before proceeding to the next work packet.
- Do not close GitHub issues automatically from the local CLI.

## Safety
- No autonomous work-packet execution.
- No GitHub issue mutation or closeout automation.
- No arbitrary command execution.
- No user-owned source rewrites.
- No remote service calls.
- Generated writes require explicit --yes and E19 approval gates.
