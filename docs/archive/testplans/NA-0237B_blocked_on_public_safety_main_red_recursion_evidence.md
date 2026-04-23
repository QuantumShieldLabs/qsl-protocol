Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-23

# NA-0237B Blocked on public-safety Main-Red Recursion Evidence

Goals: G4

## Purpose

This archive evidence records why `NA-0237B` must now be blocked on `public-safety` recursion rather than on remaining dependency ambiguity.

## PR #713 live blocker truth

- PR: `#713`
- State: `OPEN`
- Head SHA: `e4032d3906f594b9ca931bb7fe7f3e6f3db9c357`
- Mergeable: `true`
- `mergeable_state`: `blocked`

Required protected contexts on PR `#713` at capture time:

- Green: `CodeQL`, `ci-4a`, `ci-4b`, `ci-4c`, `ci-4d`, `ci-4d-dur`, `demo-cli-build`, `demo-cli-smoke`, `formal-scka-model`, `goal-lint`, `macos-qsc-qshield-build`, `metadata-conformance-smoke`, `suite2-vectors`
- Pending: none
- Failing: `public-safety`

PR changed-path proof confirms `#713` is the bounded advisory-remediation branch:

- `Cargo.lock`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0237B_dependency_advisory_remediation_testplan.md`
- `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`

No `.github/**`, `scripts/ci/**`, Cargo manifest, qsc runtime/test, qsc-desktop, qsl-server, qsl-attachments, or website/public-runtime path is part of PR `#713`.

## Exact recursion proof

The failing `public-safety` job on PR `#713` reports:

- `ERROR: latest main public safety is not green; relevant PRs stay blocked`
- `main sha=ed1b44236d94cc6ee2146409b0afb8844bdac53c check=public-safety status=completed conclusion=failure`

That proves the live blocker is recursive:

- PR `#713` is the bounded branch meant to clear the advisory
- PR `#713` itself is green on every other required protected context
- `public-safety` still fails only because latest `main` is already red

## Main-side blocker truth

Latest `main` SHA at capture time:

- `ed1b44236d94cc6ee2146409b0afb8844bdac53c`

Required protected contexts on that `main` SHA:

- Green: `ci-4a`, `ci-4b`, `ci-4c`, `ci-4d`, `ci-4d-dur`, `demo-cli-build`, `demo-cli-smoke`, `formal-scka-model`, `macos-qsc-qshield-build`, `metadata-conformance-smoke`, `suite2-vectors`
- Pending: none
- Failing: `public-safety`

Other live red main-side contexts on the same SHA:

- `advisories`
- `qsc-linux-full-suite`
- `macos-qsc-full-serial`

The `advisories` failure on latest `main` still reports:

- crate `rustls-webpki`
- version `0.103.12`
- advisory `RUSTSEC-2026-0104`
- patched floor `>= 0.103.13`
- dependency reachability through `qshield-cli`, `qsl-tui`, and `qsc`

## Proof the dependency remediation work is preserved and resumable

- Dirty/local implementation worktree path: `/srv/qbuild/work/NA-0237B/qsl-protocol`
- Local implementation branch: `na-0237b-rustls-webpki-remediation-v2`
- Local implementation head: `e4032d3906f5`
- Off-repo preservation bundle: `/srv/qbuild/tmp/na0237b_blocked_on_public_safety_preservation/`

The preservation bundle contains:

- `status.txt`
- `changed_paths.txt`
- `diffstat.txt`
- `tracked.patch`
- `untracked.zlist`
- `untracked.tgz`
- `head_sha.txt`

Those artifacts preserve the current local implementation state without mutating PR `#713`.

## Why this blocker is outside live NA-0237B code scope

`NA-0237B` authorizes the bounded dependency-remediation seam plus the already-authorized clippy-only `qsp/state.rs` touch.

The live blocker is instead:

- required-check behavior in `public-safety`
- latest-`main` health recursion
- workflow/CI governance truth

Those are outside the active `NA-0237B` runtime/dependency code scope and require a separate queue item.

## Governance-only statement

This queue-repair PR is governance-only.

It does not:

- change runtime semantics
- change protocol, wire, crypto, auth, or state-machine behavior
- change `.github/**`, `Cargo.toml`, or `Cargo.lock`
- modify PR `#713` or PR `#708`

It only repairs live queue truth on `main` so the actual `public-safety` recursion blocker can be addressed first.
