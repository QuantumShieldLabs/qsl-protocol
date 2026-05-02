Goals: G3, G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-02

# NA-0241 Closeout and NA-0242 Restoration Test Plan

## Objective

Close `NA-0241` only after the already-merged PR `#729` evidence is verified, then restore exactly one successor: `NA-0242 — KT Consistency Reject No-Mutation Hardening`.

## Protected Invariant

- Queue discipline remains single-READY.
- `public-safety` remains a required, green protected check.
- PR `#722` remains closed/superseded and unmerged.
- No branch-protection exception, check spoofing, direct push, squash, or rebase merge is used.
- No qsl-server, qsl-attachments, qsc-desktop, website, Cargo, `.github`, scripts, runtime, protocol, demo implementation, or public-safety path changes are introduced by this closeout.

## Scope Guard

Allowed changed paths for this closeout are:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0241_closeout_restore_na0242_testplan.md`

Forbidden paths include `.github/**`, `scripts/**`, Cargo files, qsp/qsc/qsl runtime paths, qsl-server, qsl-attachments, qsc-desktop, website, branch-protection settings, public-safety configuration, PR `#722`, PR `#708`, and PR `#729` branches.

## PR #729 Merge Proof

Required evidence:

- `gh pr view 729 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url`
- `state` is `MERGED`.
- `headRefOid` begins with `88728707a007`.
- `mergeCommit.oid` begins with `3d9474eff375`.
- `git rev-parse --short=12 origin/main` equals `3d9474eff375` before this closeout branch starts.

## Public-Safety Required/Green Proof

Required evidence:

- `gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks`
- The required context list includes `public-safety`.
- The latest `public-safety` check run for `origin/main` conclusion is `success`.
- PR `#729` and post-merge main `public-safety` passed normally.
- The pre-NA-0241 stale/flaky public-safety recovery was bounded-rerun evidence and did not use a branch-protection exception for PR `#729`.

## PR #722 Closed/Unmerged Proof

Required evidence:

- `gh pr view 722 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url`
- `state` is `CLOSED`.
- `mergedAt` is `null`.
- `mergeCommit` is `null`.

## Queue Parser Proof

Run a deterministic parser over `NEXT_ACTIONS.md` using `### NA-*` headings and `Status:` lines.

Required result after the patch:

- `READY_COUNT 1`
- `READY NA-0242`
- `NA-0241 DONE`
- `NA-0240 DONE`
- `NA-0239 DONE`
- `NA-0238 DONE`
- `NA-0237 DONE`

## Decision Parser Proof

Run a deterministic parser over `DECISIONS.md` using only `- **ID:** D-XXXX` entry lines.

Required result after the patch:

- D-0439 through D-0448 each exist once.
- D-0448 is the next unused decision ID after D-0447.
- No duplicate decision entry IDs exist.

## Local Validation Commands

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --check
python3 tools/goal_lint.py
cargo audit --deny warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Also run the repository markdown inventory/link validation and leak-safe added-line scan if established in current governance closeout practice.

## Required CI Expectations

The closeout PR must attach and pass the protected contexts required by `main`, including `public-safety`. CodeQL may be accepted as neutral only if GitHub accepts it for the protected context. Merge must use a normal merge commit with a validated head SHA and no branch-protection exception.
