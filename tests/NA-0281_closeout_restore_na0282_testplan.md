Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-13

# NA-0281 Closeout / NA-0282 Restoration Test Plan

## Objective

Close NA-0281 after the qsl-server route lifecycle / TTL / retention harness
and qsl-protocol evidence PRs have merged, then restore NA-0282 as the sole
READY successor for qsl-attachments retention / cleanup / recovery harness
work.

## Protected Invariants

- Exactly one READY item exists after the closeout.
- NA-0281 is DONE and records qsl-server PR #54 plus qsl-protocol PR #815.
- NA-0282 is READY and remains future executable qsl-attachments harness work.
- D-0533 exists once and D-0534 is absent.
- qsl-protocol remains governance/testplan only in this closeout.
- qsl-server, qsl-attachments, qsc-desktop, protocol/crypto state-machine,
  website/external repo, workflow, script, Cargo, branch-protection,
  public-safety configuration, dependency, and branch state paths are not
  changed.
- No production-readiness claim is introduced.

## qsl-server Evidence Carried Forward

- qsl-server PR #54 merged the executable route lifecycle / TTL / retention
  harness.
- qsl-server head `d5e6e5213a52` merged as `3f28d7433e74`.
- Chosen semantics: `ROUTE_IDLE_TTL_MS` defaults to 300000 ms, rejects
  non-numeric and zero values, caps above 86400000 ms, applies to live route
  state including non-empty routes, and runs deterministic access-triggered
  cleanup on canonical push/pull after request validation.

## qsl-protocol Evidence Carried Forward

- qsl-protocol PR #815 merged the NA-0281 evidence/governance record.
- qsl-protocol evidence head `a058f8c3350f` merged as `38b7b7572ad5`.
- D-0532 records the NA-0281 qsl-server route lifecycle / TTL / retention
  harness decision.

## Closeout Scope

Allowed files:
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0281_closeout_restore_na0282_testplan.md`

Forbidden files:
- `.github/**`
- `scripts/**`
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
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- external website repo paths
- runtime/protocol/crypto/demo/service code

## Validation Expectations

- `git diff --check` passes.
- `python3 scripts/ci/qsl_evidence_helper.py queue` reports
  `READY_COUNT 1` and READY `NA-0282`.
- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports D-0533
  latest, no duplicate decision IDs, and no D-0534.
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the closeout allowed paths reports no forbidden paths.
- `python3 scripts/ci/qsl_evidence_helper.py link-check` passes.
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base
  origin/main` reports no findings.
- `cargo audit --deny warnings` passes.
- `cargo tree -i rustls-webpki --locked` remains at v0.103.13.
- `cargo +stable test -p qsc --locked --test send_commit --
  --test-threads=1` passes.
- `python3 formal/run_model_checks.py` passes when present.
- goal-lint accepts the PR body with standalone `Goals: G1, G3, G4, G5`.
- Required GitHub checks complete successfully or are accepted skipped checks
  for docs/governance scope.

## Successor Handoff

NA-0282 must remain an executable qsl-attachments retention / cleanup /
recovery harness lane. It must not be implemented in this closeout PR.
