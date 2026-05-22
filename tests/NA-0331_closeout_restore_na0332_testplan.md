Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-22

# NA-0331 Closeout and NA-0332 Restoration Testplan

## Objective

Close NA-0331 after the bounded qshield embedded relay/demo batching
implementation/harness merged, then restore exactly one successor:
`NA-0332 -- Metadata Runtime Cover Traffic Risk Gate and Deferred
Authorization Plan`.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0332.
- NA-0331 is DONE.
- D-0644 and D-0645 each exist once.
- The closeout does not implement NA-0332.
- qshield demo batching remains bounded local/demo evidence.
- qsl-server and qsl-attachments production timing remain unproven and
  cross-repo-gated.
- No claim says batching hides timing metadata or traffic shape.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0331_closeout_restore_na0332_testplan.md`

## Forbidden Scope

- Runtime, protocol, crypto, qsc, qsp, qshield, qsl-server, qsl-attachments,
  qsc-desktop, service, formal, input, tool, script, dependency, workflow,
  website, public docs, README, START_HERE, branch-protection, and
  public-safety configuration changes.

## Required Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the closeout allowed path set.
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- Goal-lint PR proof.
- Classifier proof for the closeout changed path set.

## PR Expectations

- PR body includes `Goals: G1, G2, G3, G4, G5`.
- PR body states the closeout restores NA-0332 and does not implement NA-0332.
- Required checks complete green before merge.
- Merge uses normal merge with `--match-head-commit`.
- No branch deletion flag is used.
- Post-merge public-safety completes success.

## Successor Handoff

NA-0332 must treat cover traffic as high-risk/high-cost design work requiring
explicit risk, abuse, DoS, latency, compatibility, backup/ops, cost, and
deployment review before any implementation lane is authorized.
