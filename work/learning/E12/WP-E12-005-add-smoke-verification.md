---
title: "Learning Note — WP-E12-005 Add Smoke Verification"
document_type: "learning-note"
status: active
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-005
tags:
  - learning
  - verification
  - smoke-tests
  - add
  - monad
---

# Learning Note — WP-E12-005 Add Smoke Verification

## What You Are Building

You are turning manual checks into a reusable verification script:

```text
tools/scripts/verify-add.sh
```

This is important because each future `add` change should be able to prove it did not break the safety contract.

## Mental Model

A smoke test is not an exhaustive test.

It is a fast confidence check for the most important behavior.

For `monad add`, the important behavior is:

```text
preview safely
write only after --yes
write only inside a Monad workspace
refuse conflicts
reject conflicting mode flags
```

## Why the Temp Directory Must Be Initialized

`monad add` adds a component to a Monad workspace.

A plain `mktemp -d` directory is not automatically a Monad workspace.

That means the guarded write verification must do this:

```bash
monad init --yes
monad add app web --yes
```

inside the temp directory.

## Why This Test Matters

This test protects the command's contract:

- dry-run does not write;
- `--yes` writes only after explicit approval;
- uninitialized workspaces fail safely;
- existing files are not overwritten.

## What to Inspect

Read:

```text
tools/scripts/verify-add.sh
```

Look for these phases:

```text
add dry-run
uninitialized workspace failure
init + add write
dry-run non-write
conflict refusal
mode conflict rejection
```

## What This Teaches

Good CLI verification tests:

1. create their own temporary workspace;
2. prove successful behavior;
3. prove failure behavior;
4. clean up after themselves;
5. avoid mutating the real repo unless explicitly intended.

## Verification

Run:

```bash
tools/scripts/verify-add.sh
```

Then run the full repo verification:

```bash
tools/scripts/verify.sh
```
