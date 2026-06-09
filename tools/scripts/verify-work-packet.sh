#!/usr/bin/env bash
set -euo pipefail

echo "[verify-work-packet] cargo test -p monad-core --lib work_packet"
cargo test -p monad-core --lib work_packet

echo "[verify-work-packet] monad work-packet --dry-run"
cargo run -p monad-cli -- work-packet --dry-run >/tmp/monad-work-packet-dry-run.txt
grep -q "Monad work-packet execution workflow plan" /tmp/monad-work-packet-dry-run.txt
grep -q "Verification checklist" /tmp/monad-work-packet-dry-run.txt
grep -q "No GitHub issues were modified" /tmp/monad-work-packet-dry-run.txt

echo "[verify-work-packet] monad work-packet --dry-run --format=json"
cargo run -p monad-cli -- work-packet --dry-run --format=json >/tmp/monad-work-packet-dry-run.json
grep -q '"command":"work-packet"' /tmp/monad-work-packet-dry-run.json
grep -q '"mode":"dry-run"' /tmp/monad-work-packet-dry-run.json
grep -q '"work_packet_count":6' /tmp/monad-work-packet-dry-run.json

echo "[verify-work-packet] monad work-packet --yes"
cargo run -p monad-cli -- work-packet --yes >/tmp/monad-work-packet-apply.txt
grep -q "Monad work-packet workflow evidence write result" /tmp/monad-work-packet-apply.txt

test -f .monad/reports/work-packet-plan.md
test -f .monad/reports/work-packet-plan.json
test -f .monad/work-packets/e21-closeout-handoff.md

grep -q "Monad work-packet execution workflow plan" .monad/reports/work-packet-plan.md
grep -q '"command":"work-packet"' .monad/reports/work-packet-plan.json
grep -q "E21 Work-Packet Workflow Closeout Handoff" .monad/work-packets/e21-closeout-handoff.md

echo "[verify-work-packet] ok"
