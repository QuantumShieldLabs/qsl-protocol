Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0295 Closeout and NA-0296 Restoration Testplan

## Objective

Validate the governance-only closeout that marks NA-0295 DONE and restores
NA-0296 as the sole READY successor for website source verification and
claim-safe implementation readiness audit.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0295 is DONE.
- NA-0296 is READY.
- D-0567 exists once and D-0568 is absent.
- NA-0296 is not implemented by this closeout.
- No website or external website repository is mutated.
- No qsl-protocol runtime, protocol, wire, crypto, state-machine, demo,
  service, qsc-desktop, qsl-server, or qsl-attachments implementation changes
  occur.
- No `.github/**`, `scripts/**`, Cargo, dependency, branch-protection, or
  public-safety configuration changes occur.
- No production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable,
  quantum-proof, unbreakable, guaranteed-secure, or website-updated claim is
  introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` only if consistent
- `tests/NA-0295_closeout_restore_na0296_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `tools/**`,
  `inputs/**`, `formal/**`, `qsc-desktop/**`, `qsl-server/**`,
  `qsl-attachments/**`
- website or external website repository paths
- branch protection, public-safety configuration, branch deletion, or
  implementation paths

## Validation Requirements

Required local validation:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- targeted decision count proof for D-0567 once and D-0568 absent
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exact closeout allowed paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- direct changed-line overclaim scan
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py` if present
- goal-lint for the PR body
- classifier proof for the changed path set

## Acceptance Criteria

- Queue helper reports READY_COUNT 1 and READY NA-0296.
- NA-0295 is DONE in NEXT_ACTIONS.
- D-0567 exists once and no duplicate decision IDs exist.
- Scope guard reports only allowed closeout paths.
- Link-check and added-line leak scan pass.
- Overclaim scan matches are only negated, prohibited, or future-gated
  boundary language.
- Required CI is green before merge and post-merge public-safety is green.
- The closeout records PR #849, D-0566, D-0567, and the NA-0296 successor
  without implementing NA-0296.
