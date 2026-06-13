Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-13

# NA-0472 Closeout and NA-0473 Restoration Testplan

## Objective

Close out `NA-0472 -- QSL qsc TUI Account Bootstrap Pre-Generation
Transactionality Implementation Harness` after implementation PR #1214 merged
and post-merge public-safety completed success, then restore
`NA-0473 -- QSL Identity / Provider RNG Assurance Gap Review Plan` as the sole
READY successor without implementing NA-0473.

## Protected Invariants

- NA-0472 is DONE only after PR #1214 merged at `0eb8ceb3229c` and
  post-merge public-safety completed success.
- NA-0473 is READY and governance-review scoped.
- Exactly one READY item remains.
- Closeout changes only `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this testplan.
- No NA-0473 implementation occurs.
- No runtime mutation occurs.
- No crypto mutation occurs.
- No dependency mutation occurs.
- No Cargo manifest mutation occurs.
- No lockfile mutation occurs.
- No workflow mutation occurs.
- No qsc source mutation occurs.
- No executable test mutation occurs.
- No fuzz target mutation occurs.
- No vector mutation occurs.
- No formal model mutation occurs.
- No refimpl mutation occurs.
- No qsl-server mutation occurs.
- No qsl-attachments mutation occurs.
- No qshield runtime mutation occurs.
- No qshield-cli mutation occurs.
- No website mutation occurs.
- No public docs mutation occurs.
- No README mutation occurs.
- No START_HERE mutation occurs.
- No qwork/qstart/qresume/qshell mutation occurs.
- No backup is run.
- No restore is run.
- No qsl-backup, backup status, backup plan, rollback subtree, or backup tree
  path is mutated.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No RNG-failure-complete claim is introduced.
- No provider-RNG-complete claim is introduced.
- No secret-material-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.

## NA-0473 Restored Scope

Future NA-0473 may mutate only:

- governance evidence/testplan paths for NA-0473.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Future NA-0473 may inspect read-only:

- qsc source and tests.
- refimpl source and tests.
- formal model files.
- fuzz and vector surfaces.
- governance evidence and testplans.
- CI and public-safety evidence.

Future NA-0473 must not mutate implementation, runtime, crypto,
dependencies, Cargo manifests, lockfiles, workflows, executable tests, fuzz
targets, vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, public docs, website, README, START_HERE, qwork/qstart/qresume/
qshell, backup/restore/local-ops paths, qsl-backup, backup status files,
backup plan files, rollback subtree paths, or `/backup/qsl`.

## Validation Commands

Run from the qsl-protocol repo root:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py classifier --changed-files <changed-files>
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
```

Also run the directive closeout scope guard, link check, leak scan, overclaim
scan, PR body preflight, and goal-lint.

## Expected Results

- READY_COUNT is 1.
- The sole READY item is NA-0473.
- NA-0472 is DONE.
- NA-0471 is DONE.
- D-0932 exists exactly once.
- D-0933 exists exactly once.
- D-0934 is absent.
- Duplicate decision count is zero.
- Changed paths are limited to the five closeout paths.
- The CI classifier reports docs-only scope.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Public-safety is green before merge and after merge.
- qsl-backup source hash and source-list proof remain read-only boundary
  evidence only.

## Closeout Markers

- `NA0472_CLOSEOUT_PR1214_MERGED_OK`
- `NA0472_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0472_CLOSEOUT_D0932_CONSUMED_OK`
- `NA0472_CLOSEOUT_NA0472_DONE_OK`
- `NA0472_CLOSEOUT_NA0473_READY_OK`
- `NA0472_CLOSEOUT_ASSURANCE_GAP_REVIEW_SUCCESSOR_OK`
- `NA0472_CLOSEOUT_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0472_CLOSEOUT_NO_RUNTIME_MUTATION_OK`
- `NA0472_CLOSEOUT_NO_CRYPTO_MUTATION_OK`
- `NA0472_CLOSEOUT_NO_DEPENDENCY_CHANGE_OK`
- `NA0472_CLOSEOUT_NO_CARGO_OR_LOCKFILE_CHANGE_OK`
- `NA0472_CLOSEOUT_NO_WORKFLOW_CHANGE_OK`
- `NA0472_CLOSEOUT_NO_PUBLIC_OVERCLAIM_OK`
- `NA0472_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Public Claim Boundary

This closeout makes no public-readiness claim, no production-readiness claim,
no public-internet-readiness claim, no external-review-complete claim, no
crypto-complete claim, no KEM-complete claim, no signature-complete claim, no
identity-complete claim, no RNG-failure-complete claim, no provider-RNG-complete
claim, no secret-material-complete claim, no side-channel-free claim, no
vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

Cargo audit green remains dependency-health evidence only.

## Acceptance Criteria

- PR #1214 is merged.
- Post-merge public-safety on PR #1214 merge commit is success.
- NA-0472 is DONE.
- NA-0473 is the only READY item.
- D-0933 records closeout and restoration.
- Changed paths are limited to the closeout scope guard.
- No implementation mutation occurs.
- Root cargo audit remains green.
- Nested qsc fuzz lock audit remains green.
- Public-safety is green before merge and after merge.
