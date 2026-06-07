# NA-0436 Closeout and NA-0437 Restoration Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0436 closes only after PR #1141 remains merged and post-merge
public-safety completed success, and that NA-0437 is restored as the sole READY
successor without implementing NA-0437.

## Protected Invariants

- PR #1141 remains merged at `37362dc82fce`.
- `pq_decap_failed` no-mutation evidence remains test-backed.
- `pq_encap_failed` remains caveated as a defensive branch with no executable
  coverage overclaim.
- Root and nested dependency health remain green.
- NA-0434 remains BLOCKED.
- Exactly one READY item remains.
- No backup or restore is run.
- No public-claim expansion is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0436_closeout_restore_na0437_testplan.md`

## Forbidden Scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, script, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback,
and `/backup/qsl` paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
generated operator scripts, cargo update, cargo generate-lockfile, and
dependency remediation commands.

## PR #1141 Merge / Public-Safety Checks

- Confirm PR #1141 state is MERGED.
- Confirm PR #1141 merge commit begins with `37362dc82fce`.
- Confirm `origin/main` equals or descends from `37362dc82fce`.
- Confirm post-merge public-safety completed success on `37362dc82fce`.
- If public-safety is missing, red, ambiguous, or still in progress after the
  bounded wait, stop before governance patching.

## qsc-Adversarial Success Check

- Confirm qsc-adversarial-smoke completed success on the PR #1141 head or merge
  commit according to repo check shape.
- For this closeout, merge-commit qsc-adversarial-smoke success is acceptable
  proof.

## qsc-Adversarial-Miri Success Check

- Confirm qsc-adversarial-miri completed success on the PR #1141 head or merge
  commit according to repo check shape.
- For this closeout, merge-commit qsc-adversarial-miri success is acceptable
  proof.

## pq_decap_failed No-Mutation Test Check

- Confirm `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
  exists.
- Confirm the test
  `pq_decap_failed_reject_does_not_mutate_sessions_or_pending_state` exists.
- Run:

```bash
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
```

- Required markers:
  - `NA0436_PQ_DECAP_FAILED_MARKER_OK`
  - `NA0436_NO_SESSION_MUTATION_ON_DECAP_REJECT_OK`
  - `NA0436_PENDING_STORE_NO_MUTATION_ON_DECAP_REJECT_OK`
  - `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`
  - `NA0436_NO_RUNTIME_HOOK_USED_OK`

## pq_encap_failed Caveat Preservation Check

- Confirm the test source or output preserves the `pq_encap_failed` caveat.
- Confirm no executable coverage claim is made for `pq_encap_failed`.
- Confirm NA-0437 is framed as documentation/evidence planning for that
  defensive branch, not as implementation.

## Root Cargo Audit Green Check

Run:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
```

Required:

- root cargo audit passes;
- `rustls-webpki` is `v0.103.13` or newer safe version;
- root workspace pqcrypto unmaintained RustSec blocker is not active.

## Nested Fuzz Lock Audit Green Check

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required:

- nested qsc fuzz lock audit passes;
- pqcrypto residual scan returns zero matches.

## NA-0436 DONE / NA-0437 READY Check

- Confirm NA-0436 is marked DONE.
- Confirm NA-0437 is marked READY.
- Confirm NA-0434 remains BLOCKED.
- Confirm READY_COUNT is exactly 1.
- Confirm D-0860 exists once.
- Confirm D-0861 is absent.
- Confirm duplicate decision count is zero.

## qsl-protocol Closeout Scope Guard

Changed paths must be exactly:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0436_closeout_restore_na0437_testplan.md`

Run an exact path guard before PR creation and before merge.

## No Runtime / Dependency / Workflow / Test / Vector Mutation

- Confirm no runtime, crypto, dependency, Cargo, lockfile, workflow, script,
  executable test, fuzz target, vector, qsl-server, qsl-attachments, qshield
  runtime, website, public doc, README, START_HERE, qwork/qstart/qresume/qshell,
  qsl-backup, backup status, backup plan, rollback, or `/backup/qsl` path is
  changed.
- Confirm the existing NA-0436 test file is not changed by this closeout.

## No Public Overclaim

- Confirm no public-readiness, production-readiness, or
  public-internet-readiness claim is introduced.
- Confirm no external-review-completion, crypto-complete, side-channel-free,
  bug-free, vulnerability-free, or perfect-crypto claim is introduced.
- Confirm no metadata-free, anonymity, untraceability,
  off-host-backup-completion, disaster-recovery-completion, restore-proof, or
  backup-completion claim is introduced.
- Confirm cargo audit green is described only as dependency-health evidence.
- Confirm NA-0437 does not imply executable coverage for `pq_encap_failed`.
