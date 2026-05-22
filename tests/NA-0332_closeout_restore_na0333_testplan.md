Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0332 Closeout and NA-0333 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0332 after the cover-traffic risk gate and deferred authorization
plan merged, then restore exactly one successor:

`NA-0333 -- Metadata Runtime Cover Traffic Cost / Quota / Retention Prerequisite Plan`

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0333.
- NA-0332 is DONE.
- D-0646 and D-0647 each exist once.
- The closeout does not implement NA-0333.
- Cover traffic remains deferred pending cost/quota/retention prerequisite
  proof.
- qshield embedded relay/demo evidence remains local/demo only.
- qsl-server and qsl-attachments production timing remain unproven and
  cross-repo-gated.
- No claim says cover traffic is implemented.
- No claim says timing metadata or traffic shape is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0332_closeout_restore_na0333_testplan.md`

## Forbidden Scope

- Runtime, protocol, crypto, qsc, qsp, qshield, qsl-server, qsl-attachments,
  qsc-desktop, service, formal, input, tool, script, dependency, workflow,
  website, public docs, README, START_HERE, branch-protection, and
  public-safety configuration changes.
- Cover traffic implementation.
- NA-0333 implementation.

## Required Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0646 --select D-0647 --select D-0648`
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
- PR body states the closeout restores NA-0333 and does not implement NA-0333.
- Required checks complete green before merge.
- Merge uses normal merge with `--match-head-commit`.
- No branch deletion flag is used.
- Post-merge public-safety completes success.

## Cost-Control Proof

The closeout changed paths are governance/testplan/journal only. The expected
CI classification is docs-only, allowing cost-controlled full-suite skips
where branch protection permits them while preserving `public-safety`.

## Successor Handoff

NA-0333 must define cover-traffic cost, bandwidth, storage, quota, retention,
purge, backup, abuse/DoS, deployment, qshield demo, and service-production
boundaries before any cover-traffic prototype can be authorized.
