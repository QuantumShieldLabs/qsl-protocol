Goals: G4

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-12
Replaces: n/a
Superseded-By: n/a

# NA-0233 Blocked On PR Critical-Path CI Evidence

## Summary

This governance-only lane records why `NA-0233` is now truthfully `BLOCKED` even though PR #688 already carries the bounded MockProvider implementation/evidence work.

No runtime code, runtime tests, workflows, branch protection, or PR #688 contents change in this PR.

## PR #688 State

- PR #688 is `OPEN`.
- Current head SHA: `d9a0d3260ae0`.
- Mergeable: `MERGEABLE`.
- Merge state: `BLOCKED`.

## Current Required-Context Conclusions

- `public-safety`: `success`
- `ci-4a`: `failure`
- `ci-4b`: `success`
- `ci-4c`: `success`
- `ci-4d`: `success`
- `ci-4d-dur`: `success`
- `demo-cli-build`: `success`
- `demo-cli-smoke`: `success`
- `formal-scka-model`: `success`
- `goal-lint`: `success`
- `metadata-conformance-smoke`: `success`
- `suite2-vectors`: `success`
- `CodeQL`: `success`
- `macos-qsc-qshield-build`: `cancelled`

## Live Blocker Proof

- `ci-4a` completed `failure` at `2026-04-12T03:44:56Z`.
- `macos-qsc-qshield-build` completed `cancelled` at `2026-04-12T03:25:27Z`.
- PR #688 therefore remains blocked on required critical-path contexts rather than on queue ambiguity.

## Workflow Proof

### Broad Linux Gate

- `.github/workflows/ci.yml` still defines required context `ci-4a`.
- The runtime step still executes the whole-package qsc lane:

```bash
cargo +stable build -p qsc --release --locked
cargo +stable test -p qsc --locked
```

### Timed Serial macOS Gate

- `.github/workflows/macos-build.yml` still defines required context `macos-qsc-qshield-build`.
- The workflow still fixes `timeout-minutes: 45`.
- The test step still executes the full serial qsc suite:

```bash
cargo test -p qsc --locked --jobs 1 -- --test-threads=1
```

## Why Queue Repair Is Truthful

- PR #688 already carries the bounded MockProvider implementation/evidence lane and remains open.
- The current blocker is not queue ambiguity, stale scope, or missing runtime proof.
- The current blocker is the required PR critical-path CI design itself: a broad whole-package Linux gate plus a timed full-serial macOS gate.
- The next truthful successor is therefore `NA-0233A — qsc PR Critical-Path CI Rebalance`.

## Governance-Only Note

This PR repairs queue truth only. It does not modify runtime code, runtime tests, workflows, branch protection, or PR #688 contents.
