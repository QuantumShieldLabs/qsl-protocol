Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-25

# NA-0360 Closeout and NA-0361 Restoration Testplan

## Objective

Close NA-0360 after the key custody / key recovery implementation
authorization plan merged, and restore exactly one READY successor:
`NA-0361 -- Metadata Runtime Key Custody / Key Recovery No-Secret
Implementation Harness`.

## Protected Invariants

- NA-0360 is DONE after closeout.
- NA-0361 is the only READY item.
- D-0702 exists once.
- D-0703 exists once.
- D-0704 is absent.
- No NA-0361 implementation is performed by closeout.
- No real key generation, key upload, passphrase collection, private-key
  inspection, recovery-envelope handling, or secret material handling occurs.
- No backup, restore, deploy, rollback, off-host operation, restore target
  creation/mount/copy, backup script/timer/fstab mutation, service mutation,
  runtime mutation, dependency change, workflow change, website/public-doc
  change, README change, START_HERE change, or public-safety mutation occurs.
- qsl-server PR #56 remains bounded end-to-end harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.
- No production readiness, public-internet readiness, completed external
  review, anonymous-operation, metadata-elimination, untraceable-behavior,
  hidden-size, hidden-timing, hidden-traffic-shape, complete-disaster-recovery,
  complete-off-host-backup, real-restore-complete, real-key-custody-
  implemented, or real-key-recovery-implemented claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0360_closeout_restore_na0361_testplan.md`

## Forbidden Scope

- README, START_HERE, docs/public, workflows, dependencies, runtime source,
  protocol/crypto/qsc/qsp/qshield implementation, qsl-server, qsl-attachments,
  qsc-desktop, website, backup scripts, timers, fstab, system services,
  restore targets, off-host destinations, key material, passphrase paths,
  recovery envelopes, deployment scripts, rollback scripts, and branch
  protection/public-safety configuration.

## Queue Requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Expected:

- `READY_COUNT 1`
- `READY NA-0361 Metadata Runtime Key Custody / Key Recovery No-Secret Implementation Harness`
- NA-0360 DONE

## Decision Requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected:

- latest decision D-0703
- duplicate count zero
- D-0702 exists once
- D-0703 exists once
- D-0704 absent

## Scope Requirements

Run scope guard with only the allowed closeout paths. The changed path set must
contain no runtime, service, workflow, dependency, website, backup, restore,
key, or off-host paths.

## Link and Leak Requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- `TOTAL_MISSING 0`
- `SECRET_FINDING_COUNT 0`

## Claim-Boundary Requirements

The closeout must state that NA-0361 remains future work and must not claim:

- production readiness
- public-internet readiness
- completed external review
- anonymous operation
- metadata elimination
- untraceable behavior
- hidden attachment size
- hidden timing metadata
- hidden traffic shape
- complete disaster recovery
- complete off-host backup
- real restore completion
- real key custody implementation
- real key recovery implementation

## Required Local Checks

Run:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0360_closeout_restore_na0361_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0360_closeout_restore_na0361_testplan.md
```

## CI Expectations

- PR goal-lint passes with `Goals: G1, G2, G3, G4, G5`.
- Required checks complete normally.
- `public-safety` remains required and green.
- Merge uses a normal merge commit with `--match-head-commit`.
- No admin bypass, squash, rebase, direct push, or branch deletion command is
  used.

## Successor Handoff

NA-0361 may implement only the no-secret qsl-protocol fixture/harness in a
future directive. Real key custody, real key recovery, real recovery
envelopes, restore-drill execution, off-host target/tool implementation,
backup-plan updates, and local-ops workflow support remain explicitly
authorization gated.
