# NA-0391 Closeout and NA-0392 Restoration Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-30
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

## Objective

Close out NA-0391 after the external standards / threat / technology watch
authorization evidence merges, then restore exactly one READY successor:

`NA-0392 -- QSL External Standards / Threat / Technology Watch First Source-Cited Sweep`

This closeout does not implement NA-0392 and does not perform the first sweep.

## Protected invariants

- Exactly one READY item exists after closeout.
- NA-0391 is DONE.
- NA-0392 is READY.
- D-0764 exists once.
- D-0765 exists once.
- D-0766 is absent.
- No runtime, service, protocol, crypto, dependency, workflow, helper, public
  docs, website, qsl-server, qsl-attachments, qshield runtime, backup script,
  timer, fstab, local tool, or response archive path is changed.
- No secret handling, target setup, remote/off-host setup, real backup, real
  restore, deploy, or rollback operation is performed.
- No public, readiness, privacy, external-review-complete, production-ready,
  public-internet-ready, metadata-free, anonymity, untraceable, bug-free, or
  perfect-crypto claim is expanded.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0391_closeout_restore_na0392_testplan.md`

## Forbidden scope

- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `scripts/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- `docs/public/**`
- `README.md`
- `START_HERE.md`
- `/home/victor/work/qsl/codex/**`
- `/srv/qbuild/tools/**`
- backup scripts, timers, fstab entries, service units, keys, passphrases,
  restore paths, recovery envelopes, remote destinations, and monitoring
  configuration.

## NA-0391 evidence requirements

- PR #1045 merged normally.
- PR #1045 head and merge SHAs are recorded.
- Post-merge public-safety for the PR #1045 merge SHA completed success.
- D-0764 records the authorization decision.
- The NA-0391 evidence and authorization testplan remain linked from
  TRACEABILITY.
- The selected successor is exact.

## Queue requirements

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT 1.
- The sole READY item is NA-0392.
- NA-0391 is marked DONE.
- No NA-0392 implementation language is added to the queue.

## Decision requirements

- D-0765 records NA-0391 closeout and NA-0392 restoration.
- D-0765 states that no NA-0392 implementation is authorized by closeout.
- D-0765 preserves runtime, security, public-claim, backup, qsl-server, and
  qsl-attachments boundaries.
- Decision IDs have no duplicates.

## Traceability requirements

- TRACEABILITY links D-0765, D-0764, the NA-0391 evidence, this closeout
  testplan, PR #1045, and the selected successor.
- TRACEABILITY records backup-impact classification and sibling-repo
  boundaries.

## Source and report boundary requirements

- Closeout does not perform the first external watch sweep.
- Closeout creates no durable external-watch report.
- Closeout creates no source inventory beyond governance closeout references.
- Future durable watch report storage remains gated on explicit live scope and
  backup-impact review.

## Public-claim boundary requirements

- Closeout does not present source discovery as external review.
- Closeout does not present standards-watch authorization as production
  readiness, public-internet readiness, metadata-free proof, anonymity proof,
  untraceability proof, external-review completion, bug-free behavior, or
  perfect crypto.
- Future public technical paper work remains gated on fresh source-cited watch
  evidence and separate readiness audits.

## Required local checks

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard` with exact allowed
  closeout paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check --root .`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base
  origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- qshield-cli build/test if feasible
- classifier proof for the changed path set
- PR-body preflight and goal-lint

## CI expectations

Required GitHub checks, including public-safety, must pass normally before
merge. No admin bypass, direct push, squash merge, rebase merge, force push,
amend after PR creation, or branch deletion command is allowed.

## Successor handoff

After merge and post-merge public-safety success, the next lane is:

`NA-0392 -- QSL External Standards / Threat / Technology Watch First Source-Cited Sweep`

NA-0392 must start from its live queue scope and must not assume authority to
mutate runtime, workflow, dependency, public docs, backup, sibling-repo, local
tool, response archive, or durable watch-report storage paths unless that live
scope explicitly authorizes exact files.
