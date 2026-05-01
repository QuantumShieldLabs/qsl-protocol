Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-01

# NA-0240 Closeout and NA-0241 Restoration Test Plan

## Objective

Close `NA-0240` only from already-merged PR `#727` evidence, record D-0446, and restore `NA-0241 — Demo Negative Acceptance and Downgrade / No-Mutation Hardening` as the sole READY successor.

## Protected Invariant

- `NA-0240` closeout is governance-only and derives from merged PR `#727` evidence.
- `NEXT_ACTIONS.md` has exactly one READY item after the patch: `NA-0241`.
- `public-safety` remains a required protected check and remains green on latest `main`.
- PR `#722` remains closed and unmerged.
- No branch-protection exception, admin bypass, direct push, check spoofing, squash merge, or rebase merge is used.
- No qsl-server, qsl-attachments, qsc-desktop, website, Cargo, script, workflow, runtime, protocol, crypto, demo implementation, or service paths change in this closeout.

## Scope Guard

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0240_closeout_restore_na0241_testplan.md`

Forbidden closeout paths include `.github/**`, `scripts/**`, Cargo files, qsc/qsl runtime paths, qsl-server, qsl-attachments, qsc-desktop, website, runtime/protocol/crypto/demo/service code, branch-protection settings, public-safety configuration, and PR `#722` / `#708` / `#727` branches.

## PR #727 Merge Proof

Required proof commands:

```bash
git rev-parse origin/main
gh pr view 727 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
```

Expected:

- `origin/main` is `dc5e9755822c7e4c63cea2a8c71ae1023b8987fc`.
- PR `#727` is `MERGED`.
- PR `#727` head is `69479e8a5241395c3662d54479dd90c1d0947655`.
- PR `#727` merge commit is `dc5e9755822c7e4c63cea2a8c71ae1023b8987fc`.

## Public-Safety Required/Green Proof

Required proof commands:

```bash
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
gh api "/repos/QuantumShieldLabs/qsl-protocol/commits/dc5e9755822c7e4c63cea2a8c71ae1023b8987fc/check-runs?per_page=100" -H "Accept: application/vnd.github+json"
```

Expected:

- Required contexts include `public-safety`.
- Latest `main` `public-safety` check is `completed` with conclusion `success`.

## PR #722 Closed/Unmerged Proof

Required proof command:

```bash
gh pr view 722 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
```

Expected:

- PR `#722` is `CLOSED`.
- `mergedAt` is `null`.
- `mergeCommit` is `null`.

## Queue Parser Proof

Required parser semantics:

- Count `Status: READY` entries in `NEXT_ACTIONS.md`.
- Verify exactly one READY item: `NA-0241`.
- Verify `NA-0240`, `NA-0239`, `NA-0238`, `NA-0237`, `NA-0237A`, `NA-0237B`, `NA-0237C`, and `NA-0237D` are `DONE`.

## Decision Parser Proof

Required parser semantics:

- Count decision entries from `- **ID:** D-XXXX` lines only.
- Verify D-0446 exists exactly once.
- Verify D-0439 through D-0445 still exist exactly once.
- Verify no duplicate decision-entry IDs exist.

## Local Validation Commands

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --check
python3 tools/goal_lint.py <synthetic-pr-event-json>
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Also run the repository markdown inventory/link-integrity and leak-safe evidence scans when established by local governance practice.

## Required CI Expectations

The closeout PR must pass required protected contexts normally:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

`public-safety` must conclude `success`. CodeQL may be neutral only if GitHub accepts it for branch protection.
