Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0458 Closeout and NA-0459 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate closeout of NA-0458 after the KEM provider RNG seam implementation PR
merged and restore the selected NA-0459 qsc signature / identity provider RNG
failure scope authorization lane as the sole READY item without implementing
NA-0459.

## Protected invariants

- NA-0458 implementation evidence remains bounded to qsc KEM forced-seam tests.
- NA-0459 is authorization-only until a later exact directive changes scope.
- Production semantics remain unchanged by this closeout.
- No qsc source, test, refimpl, dependency, workflow, fuzz, vector, formal,
  service, public-surface, backup, restore, or qwork mutation occurs.
- Exactly one READY item remains.

## Allowed scope

Changed paths must be limited to:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0458_closeout_restore_na0459_testplan.md`

## Forbidden scope

- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, refimpl, qsl-server, qsl-attachments,
  qshield runtime, qshield-cli, website, public docs, README, START_HERE,
  qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status,
  backup plan, rollback, or backup tree mutation is allowed.
- No NA-0459 implementation mutation is allowed.

## Queue transition

Required queue result:

- NA-0458 is DONE.
- NA-0459 is READY.
- READY_COUNT is exactly one.

## Decision and traceability

Required decision result:

- D-0903 exists once.
- D-0904 exists once.
- D-0905 is absent before a future NA-0459 directive.
- Duplicate decision count is zero.

Traceability must contain a closeout row tying NA-0458 closeout, PR #1185,
NA-0459 restoration, and the closeout validation bundle to Goals G1 through G5.

## Public-safety prerequisite

Before closeout, PR #1185 must be merged and post-merge public-safety must have
completed success on merge commit `614923e9cb2e`.

## Validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

## Scope guard

The changed path set must contain only the five allowed closeout paths. Any
runtime, crypto, dependency, workflow, executable test, fuzz, vector, formal,
refimpl, service, public-surface, backup, restore, qsl-backup, status, plan,
rollback, qwork, qstart, qresume, or qshell mutation fails the closeout.

## Public claim boundary

NA-0458 closeout and NA-0459 restoration are bounded internal governance
evidence only.

No public-readiness claim is allowed.

No production-readiness claim is allowed.

No public-internet-readiness claim is allowed.

No external-review-complete claim is allowed.

No crypto-complete claim is allowed.

No KEM-complete claim is allowed.

No RNG-failure-complete claim is allowed.

No provider-RNG-complete claim is allowed.

No side-channel-free claim is allowed.

No vulnerability-free claim is allowed.

No bug-free claim is allowed.

No perfect-crypto claim is allowed.

Cargo audit green remains dependency-health evidence only.

## Closeout acceptance

NA-0458 is DONE, NA-0459 is READY, D-0904 exists once, D-0905 is absent, and
exactly one READY item remains.
