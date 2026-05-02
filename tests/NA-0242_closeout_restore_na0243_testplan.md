Goals: G3, G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-02

# NA-0242 Closeout and NA-0243 Restoration Test Plan

## Objective

Close `NA-0242` only after PR `#731` is merged and post-merge `public-safety` is green, then restore exactly one successor: `NA-0243 — Skipped-Key and Receive-Decryption Reject No-Mutation Hardening`.

## Protected Invariant

- Queue discipline remains single-READY.
- `public-safety` remains a required, green protected check.
- NA-0243 is promoted only as a bounded successor; it is not implemented in this closeout.
- No branch-protection exception, check spoofing, direct push, squash, or rebase merge is used.
- No qsl-server, qsl-attachments, qsc-desktop, website, qsc/qsl app, Cargo, `.github`, scripts, runtime/service, public-safety, or branch-protection path changes are introduced by this closeout.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0242_closeout_restore_na0243_testplan.md`

Forbidden paths include `.github/**`, `scripts/**`, Cargo files, qsp/qsc/qsl runtime paths, qsl-server, qsl-attachments, qsc-desktop, website, branch-protection settings, public-safety configuration, and any NA-0243 implementation paths.

## PR #731 Merge Proof

Required evidence:

- `gh pr view 731 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url`
- `state` is `MERGED`.
- `headRefOid` begins with `4c11dbdcda6c`.
- `mergeCommit.oid` begins with `51c478d8111b`.

## Public-Safety Required/Green Proof

Required evidence:

- `gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks`
- The required context list includes `public-safety`.
- PR `#731` `public-safety` conclusion is `success`.
- Post-merge main `public-safety` conclusion is `success`.

## Queue Parser Proof

Run a deterministic parser over `NEXT_ACTIONS.md` using `### NA-*` headings and `Status:` lines.

Required result after the patch:

- `READY_COUNT 1`
- `READY NA-0243`
- `NA-0242 DONE`
- `NA-0241 DONE`
- `NA-0240 DONE`
- `NA-0239 DONE`
- `NA-0238 DONE`
- `NA-0237 DONE`

## Decision Parser Proof

Run a deterministic parser over `DECISIONS.md` using only `- **ID:** D-XXXX` entry lines.

Required result after the patch:

- D-0449 exists once.
- D-0450 exists once.
- D-0450 is the next closeout/restoration decision ID after D-0449.
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

Also run markdown inventory/link validation, leak-safe added-line scan, queue parser, decision parser, and scope guard using established repository patterns.

## Required CI Expectations

The closeout PR must attach and pass the protected contexts required by `main`, including `public-safety`. CodeQL may be accepted as neutral only if GitHub accepts it for the protected context. Merge must use a normal merge commit with a validated head SHA and no branch-protection exception.
