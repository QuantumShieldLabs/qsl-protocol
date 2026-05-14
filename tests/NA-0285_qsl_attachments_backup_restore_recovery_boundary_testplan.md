Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0285 qsl-attachments Backup / Partial Restore / Transactional Recovery Boundary Testplan

Goals: G1, G3, G4, G5

## Objective

Validate the planning-only `NA-0285 — qsl-attachments Backup / Partial
Restore / Transactional Recovery Boundary Plan`.

NA-0285 must record the current qsl-attachments backup, restore, same-root
restart, partial restore, and transactional recovery boundaries and define the
future NA-0286 executable harness shape. It must not implement qsl-attachments
behavior.

## Protected Invariants

- qsl-attachments remains read-only for NA-0285.
- qsl-attachments source, tests/harness, docs, dependencies, and workflows are
  untouched.
- qsl-server remains untouched.
- qsl-protocol remains implementation-clean.
- Protocol, wire, crypto, auth, and state-machine behavior are untouched.
- The current supported recovery boundaries stay explicit:
  - local single-root behavior;
  - same-root restart behavior;
  - cold full-root backup/restore plus matching service configuration as a
    documented boundary needing executable full-root copy proof;
  - unsupported hot/live backup;
  - unsupported partial restore unless proven otherwise.
- Backup/restore/transactional recovery are not claimed implemented.
- Production readiness, deployment readiness, public exposure, and production
  backup/restore are not claimed.
- Future implementation remains bounded and test-backed.
- Public-safety remains required and green.

## Allowed / Forbidden Scope

Allowed qsl-protocol paths:

- `docs/governance/evidence/NA-0285_qsl_attachments_backup_restore_recovery_boundary_plan.md`
- `tests/NA-0285_qsl_attachments_backup_restore_recovery_boundary_testplan.md`
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md` only for
  handoff references
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope:

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
- external website repository paths
- runtime/protocol/crypto/demo/service code
- branch-protection or public-safety configuration
- branch deletion

## Read-Only qsl-attachments Baseline Proof

Required baseline audit:

- inspect `src/main.rs`, `src/lib.rs`, relevant integration tests,
  `README.md`, and `docs/**` in the sibling qsl-attachments repository;
- confirm qsl-attachments worktree is clean before and after inspection;
- confirm `origin/main` is the expected qsl-attachments merge state;
- record that qsl-attachments remains unchanged.

Baseline facts to preserve:

- startup recovery reconciles a single local storage root;
- coherent open sessions require `session.json` and all journaled part files;
- committed objects require `object.json` plus `ciphertext.bin`;
- orphan and incoherent artifacts are discarded fail-closed;
- no external journal/index/WAL exists beyond the JSON metadata and file tree;
- hot/live backup is unsupported;
- partial restore is unsupported;
- cross-file transactional recovery is not claimed;
- current cold full-root backup/restore is a documented boundary and still
  needs executable full-root copy proof.

## Backup / Partial Restore / Transactional Recovery Design Requirements

The design document must cover:

- current backup baseline;
- current restore baseline;
- current partial restore baseline;
- current transactional recovery baseline;
- current same-root restart baseline;
- existing proof coverage;
- missing proof/design gaps;
- semantic decisions needed before implementation;
- recommended future semantic policy;
- rejected alternatives;
- implementation prerequisites;
- explicit items not implemented in NA-0285.

The design must include a failure-case matrix for:

- missing `session.json`;
- orphan part;
- missing part;
- `object.json` without ciphertext;
- ciphertext without `object.json`;
- mismatched descriptor/object metadata;
- expired state;
- deleted state;
- rejected write residue.

## Future NA-0286 Executable Harness Requirements

The NA-0286 plan should require qsl-attachments executable tests for:

- cold/quiesced full-root copy and restore into a new storage root;
- coherent committed-object recovery;
- coherent open-session best-effort recovery under the selected boundary;
- partial restore fixtures that fail closed;
- transactional recovery boundary fixtures for object/session write windows;
- no resurrection of rejected writes;
- no resurrection of expired or deleted state;
- no reconstruction of missing capability material;
- no plaintext/ciphertext/capability/descriptor leakage in logs, audit
  snapshots, error bodies, or evidence;
- no public or production backup/restore claim.

## No Implementation Proof

Success for NA-0285 requires proving that no implementation happened:

- qsl-attachments changed files: none;
- qsl-server changed files: none;
- qsl-protocol implementation/runtime changed files: none;
- Cargo/dependency/workflow/public-safety changed files: none;
- qsl-attachments source/test/docs/workflows/dependencies remain unchanged.

## No Production-Readiness Claim

The evidence must not claim production readiness, deployment readiness, public
internet readiness, external review completion, metadata elimination, strong
identity-hiding guarantees, completed backup implementation, completed restore
implementation, unsupported hot/live backup support, unsupported partial restore support, cross-node
replication support, or a completed release-security proof.

Unsupported boundaries and known gaps must remain visible.

## Link / Leak / Goal-Lint Expectations

Expected qsl-protocol validation:

- queue parser reports `READY_COUNT 1` and `READY NA-0285`;
- decisions parser reports D-0540 exactly once and no duplicate IDs;
- D-0541 remains absent before closeout;
- scope guard accepts only allowed paths;
- link check passes;
- leak scan passes;
- overclaim scan has no affirmative prohibited claims;
- PR body has a standalone `Goals: G1, G3, G4, G5` line near the top.

## CI Expectations

- qsl-protocol public-safety is required and green before PR creation.
- qsl-protocol evidence PR required checks attach and pass normally.
- qsl-protocol public-safety remains required and green after merge.
- Docs/governance-only classification should avoid unnecessary full-suite cost
  under NA-0262A.
- qsl-attachments read-only audit/test evidence may be recorded, but no
  qsl-attachments file may be changed.

## Successor Handoff

Recommended successor:

`NA-0286 — qsl-attachments Executable Backup / Partial Restore / Transactional Recovery Harness`

The successor must implement the executable qsl-attachments backup / partial
restore / transactional recovery harness designed in NA-0285 or stop with a
clear prerequisite. It must not claim production backup/restore posture without
the executable proof and required CI.
