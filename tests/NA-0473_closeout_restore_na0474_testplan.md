Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-13

# NA-0473 Closeout and NA-0474 Restoration Testplan

## Objective

Close out `NA-0473 -- QSL Identity / Provider RNG Assurance Gap Review Plan`
after evidence PR #1216 merged and post-merge public-safety completed success,
then restore `NA-0474 -- QSL KEM / Signature / Transcript Binding Read-Only
Audit Plan` as the sole READY successor without implementing NA-0474.

## Protected Invariants

- NA-0473 is DONE only after PR #1216 merged at `91e05e7b089b` and
  post-merge public-safety completed success.
- Post-merge qsc-adversarial-smoke on `91e05e7b089b` completed success.
- NA-0474 is READY and read-only audit scoped.
- Exactly one READY item remains.
- Closeout changes only `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this testplan.
- No NA-0474 implementation occurs.
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
- No metadata-free claim is introduced.
- No anonymity claim is introduced.
- No untraceability claim is introduced.
- No backup-complete claim is introduced.
- No restore-proof claim is introduced.

## NA-0474 Restored Scope

Future NA-0474 may mutate only:

- governance evidence/testplan paths for NA-0474.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Future NA-0474 may inspect read-only:

- qsc KEM, signature, transcript, identity, suite binding, and
  state-transition surfaces.
- refimpl KEM, signature, transcript, identity, suite binding, and
  state-transition surfaces.
- formal model files.
- fuzz and vector surfaces.
- governance evidence and testplans.
- CI and public-safety evidence.

Future NA-0474 must not mutate implementation, runtime, crypto, dependencies,
Cargo manifests, lockfiles, workflows, executable tests, fuzz targets, vectors,
formal models, qsl-server, qsl-attachments, qshield runtime, qshield-cli,
public docs, website, README, START_HERE, qwork/qstart/qresume/qshell,
backup/restore/local-ops paths, qsl-backup, backup status files, backup plan
files, rollback subtree paths, or `/backup/qsl`.

## Validation Commands

Run from the qsl-protocol repo root:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
```

Also run the directive closeout scope guard, link check, leak scan, overclaim
scan, PR body preflight, and goal-lint.

## Expected Results

- READY_COUNT is 1.
- The sole READY item is NA-0474.
- NA-0473 is DONE.
- NA-0472 is DONE.
- D-0934 exists exactly once.
- D-0935 exists exactly once.
- D-0936 is absent.
- Duplicate decision count is zero.
- Changed paths are limited to the five closeout paths.
- The local classifier reports governance-only closeout scope.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Public-safety is green before merge and after merge.
- qsl-backup source hash and source-list proof remain read-only boundary
  evidence only.

## Closeout Markers

- `NA0473_CLOSEOUT_PR1216_MERGED_OK`
- `NA0473_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0473_CLOSEOUT_QSC_ADVERSARIAL_GREEN_OK`
- `NA0473_CLOSEOUT_D0934_CONSUMED_OK`
- `NA0473_CLOSEOUT_NA0473_DONE_OK`
- `NA0473_CLOSEOUT_NA0474_READY_OK`
- `NA0473_CLOSEOUT_KEM_SIGNATURE_TRANSCRIPT_AUDIT_SUCCESSOR_OK`
- `NA0473_CLOSEOUT_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0473_CLOSEOUT_NO_RUNTIME_MUTATION_OK`
- `NA0473_CLOSEOUT_NO_CRYPTO_MUTATION_OK`
- `NA0473_CLOSEOUT_NO_DEPENDENCY_CHANGE_OK`
- `NA0473_CLOSEOUT_NO_CARGO_OR_LOCKFILE_CHANGE_OK`
- `NA0473_CLOSEOUT_NO_WORKFLOW_CHANGE_OK`
- `NA0473_CLOSEOUT_NO_PUBLIC_OVERCLAIM_OK`
- `NA0473_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Public Claim Boundary

This closeout makes no public-readiness claim, no production-readiness claim,
no public-internet-readiness claim, no external-review-complete claim, no
crypto-complete claim, no KEM-complete claim, no signature-complete claim, no
identity-complete claim, no RNG-failure-complete claim, no
provider-RNG-complete claim, no secret-material-complete claim, no
side-channel-free claim, no vulnerability-free claim, no bug-free claim, no
perfect-crypto claim, no metadata-free claim, no anonymity claim, no
untraceability claim, no backup-complete claim, and no restore-proof claim.

Cargo audit green remains dependency-health evidence only.

## Acceptance Criteria

- PR #1216 is merged.
- Post-merge public-safety on PR #1216 merge commit is success.
- Post-merge qsc-adversarial-smoke on PR #1216 merge commit is success.
- NA-0473 is DONE.
- NA-0474 is the only READY item.
- NA-0474 is read-only audit scoped.
- D-0935 exists once.
- D-0936 is absent.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, service, qshield, qsl-server,
  qsl-attachments, backup, restore, qwork, qstart, qresume, or qshell path is
  mutated.
- No public overclaim is introduced.
