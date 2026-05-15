Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0290A Insert Public Attention Visibility Testplan

## Objective

Verify that NA-0290A is inserted as a bounded public attention and visibility
strategy lane before NA-0290, without authorizing implementation or public copy
changes.

## Protected Invariants

- Exactly one READY item exists after insertion.
- NA-0290A is the sole READY item.
- NA-0290 remains preserved as the successor after NA-0290A closeout, but is not
  READY during NA-0290A execution.
- D-0550 exists exactly once.
- No duplicate decision IDs are introduced.
- Public-safety remains required and green.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0290A_insert_public_attention_visibility_testplan.md`

## Forbidden Scope

- README or START_HERE changes.
- Website or external website changes.
- Protocol, crypto, state-machine, runtime, qsl-server, qsl-attachments, or
  qsc-desktop changes.
- `.github/**`, `scripts/**`, `Cargo.toml`, `Cargo.lock`, dependency, formal,
  inputs, tools, or implementation-path changes.
- Branch-protection or public-safety configuration changes.
- Branch deletion commands.

## Queue Checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Expected:

- `READY_COUNT 1`
- `READY NA-0290A Public Attention and Visibility Strategy Audit`
- NA-0290 is not READY.

## Decision Checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected:

- latest decision includes D-0550
- D-0550 count is one
- duplicate decision count is zero
- D-0551 is absent during the insertion PR

## Scope Guard

Run the scope guard against `origin/main` with only the allowed insertion paths.

Expected:

- all changed paths are inside the allowed scope.
- no website, README, START_HERE, implementation, workflow, script, Cargo, or
  dependency paths are present.

## Claim-Boundary Checks

Scan changed lines for high-risk public claims.

Allowed findings are limited to explicit prohibited-language, negated, or
future-unproven examples. No affirmative production-readiness, external review
completion, anonymity, metadata-free, untraceable, quantum-proof, unbreakable,
military-grade, or guaranteed-secure claim may be introduced.

## CI Expectations

- public-safety is required before work.
- public-safety is green before PR creation.
- docs/governance-only scope should qualify for cost-controlled CI behavior.
- PR required checks must pass before merge.

## Future Gate

NA-0290A implementation must create the strategy, audit, and strategy testplan
in a separate PR before closeout restores NA-0290.
