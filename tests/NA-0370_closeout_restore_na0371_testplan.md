Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-27

# NA-0370 Closeout and NA-0371 Restoration Test Plan

## Objective

Close NA-0370 after the operator response intake evidence merged and restore exactly one READY successor:

`NA-0371 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Collection Request`

This test plan validates queue, decision, traceability, and boundary evidence only. It does not implement NA-0371.

## Protected Invariants

- READY_COUNT remains exactly one.
- NA-0370 is DONE after closeout.
- NA-0371 is READY after closeout.
- D-0723 exists once.
- D-0724 is absent.
- NA-0371 implementation is not performed by closeout.
- NA-0370 intake remains evidence only and is not target setup.
- `OPERATOR_RESPONSE_NOT_PRESENT` remains absence evidence, not target evidence.
- qsl-server and qsl-attachments production boundaries remain explicit.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0370_closeout_restore_na0371_testplan.md`

## Forbidden Scope

- No qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol/crypto/key-schedule, dependency, workflow, branch-protection, public-safety configuration, qsc-desktop, website, external website, README, START_HERE, docs/public, formal, input, script runtime, tools/refimpl, app runtime, service, production deployment, backup script/timer/fstab, restore state, rollback state, off-host setup, remote connection, host-key scan, known_hosts, repository init, tool installation, real key handling, credential handling, secret handling, backup, restore, deploy, rollback, or restore target creation/mount/copy changes.

## Required Local Checks

Run from the qsl-protocol repository root:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0722 --select D-0723 --select D-0724
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0370_closeout_restore_na0371_testplan.md
```

## Expected Results

- Queue helper reports READY_COUNT 1 and READY NA-0371.
- Decisions helper reports D-0722 once, D-0723 once, D-0724 absent, and duplicate count zero.
- Scope guard accepts only the five allowed closeout paths.
- Link-check reports zero missing links.
- Leak scan reports zero secret findings.
- Dependency health remains green with `rustls-webpki` v0.103.13 or newer safe version.
- qsc send_commit and formal/model checks pass.
- Classifier reports docs/governance-only scope.

## Public Claim Boundary

The closeout must preserve these public-claim limits:

- no production readiness claim.
- no public-internet readiness claim.
- no external review completion claim.
- no anonymity claim.
- no metadata-free behavior claim.
- no untraceable behavior claim.
- no hidden attachment size claim.
- no hidden timing metadata claim.
- no hidden traffic shape claim.
- no local-continuity-as-complete-disaster-recovery claim.
- no off-host backup completion claim.
- no real restore completion claim.
- no configured target claim.
- no verified host identity claim.
- no implemented real key custody claim.
- no implemented real key recovery claim.

## CI Expectations

The closeout PR must pass required qsl-protocol branch protection, including `public-safety`, before merge. The merge must use normal merge with `--match-head-commit`, no admin bypass, no squash, no rebase, no direct push, and no branch deletion command.

## Successor Handoff

The sole READY successor after merge must be:

`NA-0371 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Collection Request`

NA-0371 must remain unimplemented by this closeout.
