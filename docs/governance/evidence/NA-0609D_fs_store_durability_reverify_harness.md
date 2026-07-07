Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

# NA-0609D — fs_store Persistence Durability Re-Verification and ENG-0004 False-Positive Correction

## Summary

NA-0609D is a read-only re-verification lane executed under directive
QSL-DIR-2026-07-06-545 (D545) as a LITE-CEREMONY lane (single PR, single decision
D-1216). It corrects a false positive from the NA-0609B audit: ledger ENG-0004
reported the fs_store directory fsync as a no-op, but that no-op is only the
`#[cfg(not(unix))]` fallback. On the deployment target (x86_64-linux) the directory
fsync is implemented and the persistence path is fully crash-durable.

Result classification: `FS_STORE_DURABILITY_SOUND_ON_UNIX_ENG0004_FALSE_POSITIVE`.

This lane changes no source; it corrects the record honestly. It is not a
public-readiness, production-readiness, security-completion, crypto-complete,
vulnerability-free, or bug-free claim.

## Required Markers

- NA0609D_D1214_CONSUMED_OK
- NA0609D_D1215_CONSUMED_OK
- NA0609D_FRESH_QWORK_PROOF_OK
- NA0609D_CURRENT_MAIN_HEALTH_OK
- NA0609D_D1216_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0609D_LITE_CEREMONY_CERTIFIED_OK
- NA0609D_READ_ONLY_NO_SOURCE_MUTATION_OK
- NA0609D_WRITE_ATOMIC_DURABLE_SEQUENCE_VERIFIED_OK
- NA0609D_UNIX_DIR_FSYNC_CONFIRMED_OK
- NA0609D_ENG0004_MARKED_WONTFIX_OK
- NA0609D_WF0005_FILED_OK
- NA0609D_NA0609B_EVIDENCE_ADDENDUM_OK
- NA0609D_PRIVATE_MATERIAL_SCAN_OK
- NA0609D_RESULT_CLASSIFICATION_SELECTED_OK
- NA0609D_NA0609_SOLE_READY_OK

## Qwork, Queue, And Main Gates

Fresh operator-run qwork proof for lane NA-0609 from `2026-07-06T23:51:32Z`
(regenerated via the WF-0004 drop-first workflow) verified before mutation; HEAD ==
origin/main == main == `4ebb81665e50`; worktree clean; READY_COUNT 1 with READY
NA-0609; D-1214 once, D-1215 once, D-1216 absent.

## Inheritance

D-1214 (NA-0609C implementation) and D-1215 (NA-0609C closeout) consumed once each
and Accepted; ENG-0004 was open in the ledger.

## Re-Verification (read-only)

`qsl/qsl-client/qsc/src/fs_store/mod.rs`:

- `write_atomic` (lines 94-124) performs, on Unix: enforce safe parents and dir
  perms; create an exclusive temp file `<name>.tmp.<pid>`; enforce file perms;
  `write_all`; `f.sync_all()` (fsync the file content); `fs::rename(tmp, path)`
  (atomic rename); then `fsync_dir_best_effort(dir)`.
- `fsync_dir_best_effort` has two cfg-gated definitions: `#[cfg(not(unix))]` (line
  359) is a no-op, and `#[cfg(unix)]` (line 362) is
  `let _ = File::open(dir).and_then(|d| d.sync_all());` — a real directory fsync.

Conclusion: on the x86_64-linux deployment target the full durable-write sequence
(content fsync -> atomic rename -> directory fsync) is present, so a completed
config/session write survives a power-loss crash. G2 crash-durability is sound on
Unix; the non-unix no-op is a documented best-effort degradation. ENG-0004 was a
false positive caused by the NA-0609B audit reading only the `not(unix)` stub.

## Corrections

- Ledger ENG-0004 set to wontfix with the correction explanation.
- Ledger WF-0005 added: audits must enumerate and read cfg-gated variants of a
  symbol (compiled for the deployment target) before recording a function as a
  no-op/stub.
- A corrective addendum was added to the NA-0609B audit evidence doc pointing to
  this re-verification.

## Boundary And Claim

This lane mutated only docs/evidence/ledger/governance paths; it changed no `.rs`,
test, Cargo, workflow, spec, `.claude`, or hook file, and applied no source fix. No
runtime/LAN action occurred. No endpoint, port, token, capability, key, seed,
plaintext, ciphertext body, or raw private material is published. No public-
readiness, production-readiness, security-completion, crypto-complete,
vulnerability-free, or bug-free claim is made; the durability statement is scoped
to Unix and to the read code path.
