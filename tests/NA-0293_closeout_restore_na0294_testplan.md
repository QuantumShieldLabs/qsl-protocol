Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0293 Closeout and NA-0294 Restoration Test Plan

## Objective

Validate that NA-0293 closes only after its executable sanitized-error and
retention/purge metadata harness PR has merged and post-merge public-safety is
green, then restore NA-0294 as exactly one READY successor.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0293 is DONE.
- NA-0294 is READY.
- D-0560 exists once.
- D-0561 remains absent.
- Metadata phase-2 remains evidence-bound and incomplete beyond bounded
  NA-0291 and NA-0293 harness proof.
- No anonymity, metadata-free, untraceable, external-review-complete,
  production-readiness, or public-internet-readiness claim is introduced.
- No public website implementation is authorized.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0293_closeout_restore_na0294_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
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
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime, protocol, crypto, demo, service, dependency, workflow, and public
  website implementation paths.

## Closeout Preconditions

Before editing closeout files:

- Packet F PR `#842` is merged.
- Packet F merge commit `ea74be7338a9` has post-merge `public-safety`
  completed success.
- READY_COUNT is `1`.
- READY is `NA-0293`.
- D-0559 exists once.
- D-0560 is absent.
- The live NA-0293 block does not name a successor conflicting with NA-0294.

## Queue Requirements

Validation must prove:

- READY_COUNT `1`.
- READY `NA-0294`.
- NA-0293 `DONE`.
- NA-0292 remains `DONE`.
- NA-0291 remains `DONE`.
- NA-0290A remains `DONE`.

## Decision Requirements

Validation must prove:

- D-0559 exists once.
- D-0560 exists once.
- D-0561 is absent.
- duplicate decision count is `0`.

## Scope Requirements

Run scope guard against `origin/main...HEAD` with the allowed paths listed in
this plan. The forbidden-path check must show no README, START_HERE, workflow,
Cargo, qsp, protocol/crypto, service, website, app, tool, input, formal, or
runtime implementation changes.

## Claim Boundary Requirements

Scan changed lines for unsupported readiness and metadata claims. Any matches
must be negated, prohibited, NOT_READY, future-gated, or explicitly bounded to
the NA-0293 harness proof.

## Validation Commands

Run:

- `git diff --check origin/main...HEAD`
- `python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0294 --select NA-0293 --select NA-0292 --select NA-0291 --select NA-0290A`
- `python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0559 --select D-0560 --select D-0561`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main ...`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- synthetic-event `tools/goal_lint.py`

## CI Expectations

- Required PR checks attach and complete normally.
- `public-safety` remains required and green.
- Closeout is governance/testplan-only, so full-suite skip behavior may apply
  where CI cost-control classifies the change as docs/governance-only.
- No admin bypass, direct push, squash, rebase, branch-protection mutation, or
  branch deletion is used.

## Successor Handoff

After closeout merge and post-merge public-safety success, NA-0294 is the sole
READY item. NA-0294 may implement README/START_HERE/public evidence navigation
refresh within its own scope and must preserve all claim boundaries.
