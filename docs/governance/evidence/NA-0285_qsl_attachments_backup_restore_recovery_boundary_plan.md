Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0285 qsl-attachments Backup / Partial Restore / Transactional Recovery Boundary Plan

Goals: G1, G3, G4, G5

## Executive Summary

NA-0285 is a planning-only qsl-attachments boundary lane. It records the
current backup, restore, same-root restart, and transactional recovery posture
after NA-0282, NA-0283, and NA-0284 proved local single-root retention,
cleanup, quota, and capability hardening evidence.

This plan does not implement backup or restore behavior. It does not change
qsl-attachments source, tests, docs, dependencies, workflows, or deployment
posture. It does not change qsl-server, qsl-protocol runtime behavior,
protocol/wire/crypto/state-machine behavior, qsc-desktop, website, workflows,
scripts, Cargo manifests, branch protection, public-safety configuration, or
dependencies.

The current supported recovery boundaries are:

- local single-root behavior;
- same-root restart behavior;
- cold full-root backup/restore plus matching service configuration, as a
  documented boundary that still needs executable full-root copy proof;
- unsupported hot/live backup;
- unsupported partial restore unless proven otherwise by future executable
  harness work.

## Current qsl-attachments Local Single-Root Baseline

The inspected qsl-attachments main state is
`QuantumShieldLabs/qsl-attachments` at `0b7b3fcf9afc`.

The service is a single-node local-disk runtime. The local storage root contains
only session directories and object directories:

- `sessions/<session_id>/session.json`;
- `sessions/<session_id>/parts/<part_index>.part`;
- `objects/<locator_ref>/object.json`;
- `objects/<locator_ref>/ciphertext.bin`.

There is no external database, durable WAL, durable audit index, object-store
backend, distributed lock, cross-node replication layer, or multi-root recovery
surface. The service persists opaque ciphertext bytes, non-secret resource
references, lifecycle metadata, and hashed capability material.

The current request paths use a process-local mutation lock for in-process
serialization. That lock does not create a durable cross-file transaction and
does not prove crash or power-loss durability.

## Current Same-Root Restart Baseline

Same-root restart is the strongest currently executable recovery baseline.
Startup creates the storage layout and reconciles the same local storage root
before serving requests.

Current restart behavior:

- coherent open sessions are re-exposed only when `session.json` is readable,
  shape-valid, names the same session directory, and every journaled
  `stored_parts` entry has a matching part file with the expected length;
- orphan staged artifacts are discarded rather than adopted into the session
  journal;
- missing journaled part files make the session incoherent and the session
  directory is discarded fail-closed;
- aborted or expired sessions are not resumed, staged parts are cleared, and
  resume-token hashes are cleared;
- committed objects are re-exposed only when `object.json` is readable,
  shape-valid, names the same object directory, and `ciphertext.bin` exists as
  a file with the exact declared ciphertext length;
- object directories without coherent metadata/bytes are discarded
  fail-closed;
- expired object metadata is preserved as expired state while any remaining
  `ciphertext.bin` is removed.

Existing tests prove same-root restart preserves coherent open sessions and
committed objects while discarding missing-part sessions, orphan part files,
object metadata without matching bytes, and ciphertext-only object directories.

## Current Cold Full-Root Backup / Restore Support or Gap

The current docs and service-contract test state that the only supported
backup/restore shape is a cold or quiesced full copy or snapshot of the entire
storage root plus matching operator-managed service configuration.

That statement is a boundary, not a completed executable backup/restore proof.
The current test suite checks that docs and validation evidence state the
boundary truthfully, and same-root startup recovery exercises the reconciliation
rules. It does not yet execute a tempdir full-root copy, restore into a new
storage root, restart the service on that restored root, and prove resulting
API behavior.

For NA-0286, cold full-root restore should become an executable harness:
stop/quiesce the service, copy the whole root and matching config, start a new
state on the copied root, then prove coherent committed objects remain
fetchable, coherent open sessions are treated according to the selected
best-effort boundary, and incoherent artifacts remain fail-closed.

## Unsupported Hot / Live Backup Boundary

Hot/live backup while mutations continue is unsupported.

The current runtime does not provide a durable cross-file transaction for:

- staged part bytes plus `session.json`;
- `ciphertext.bin` plus `object.json` plus session-directory removal.

A live copy can observe a mixed-time state such as a staged part without the
matching journal update, an object byte file without metadata, metadata without
bytes, or an old session directory alongside a newly written committed object.
The current reconciliation logic intentionally treats those artifacts as
incoherent and discards them rather than reconstructing a stronger state.

Future work must not imply hot/live backup support unless a later directive
adds a transaction or snapshot design and executable tests that prove it.

## Partial Restore Boundary

Partial restore is unsupported unless a future lane proves a narrower
fail-closed policy.

Unsupported partial restore shapes include:

- sessions without their whole session directory;
- only `session.json`;
- only part files;
- objects without paired `object.json` and `ciphertext.bin`;
- only object metadata;
- only object ciphertext bytes;
- mixed-time subsets copied from different roots or different points in time.

The recommended policy for NA-0286 is fail-closed partial restore: if the
service is presented with a partial or mixed fixture, startup must discard or
refuse to re-expose incoherent resources and must not reconstruct missing
journals, missing parts, missing object metadata, missing object bytes, or
capability material.

## Transactional Recovery Boundary

The current service has atomic per-file write helpers for JSON metadata and
staged part files, but it does not provide cross-file transactional durability.

Current write ordering:

- session metadata writes go through a temporary JSON file and rename into
  `session.json`;
- staged ciphertext part writes go through a temporary file and rename into
  the part path;
- object metadata writes go through a temporary JSON file and rename into
  `object.json`;
- commit writes `ciphertext.bin`, flushes that file handle, writes
  `object.json`, then removes the old session directory.

There is no current fsync discipline for files plus parent directories and no
durable transaction spanning the object bytes, object metadata, and session
removal. The truthful transactional recovery boundary is therefore
fail-closed reconciliation, not exactly-once commit promotion or open-session
survival across abrupt crash/power loss.

Future implementation may add stronger durability only if it remains bounded,
test-backed, and secret-safe. Until then, startup must prefer discarding
incoherent artifacts over inventing state.

## Failure-Case Matrix

| Failure case | Current baseline | Recommended NA-0286 expectation |
| --- | --- | --- |
| missing `session.json` | Session directory is treated as orphaned and removed. | Prove partial session restore fails closed and does not create a resumable session. |
| orphan part | Part not named by coherent `session.json` is removed. | Prove orphan staged bytes are discarded and not adopted into session state. |
| missing part | Journaled part without matching file makes the session incoherent. | Prove missing-part restore fails closed and cannot commit by reconstructing bytes. |
| `object.json` without ciphertext | Committed object is incoherent and discarded. | Prove metadata-only object restore is not fetchable. |
| ciphertext without `object.json` | Object directory is orphaned and discarded. | Prove ciphertext-only object restore is not fetchable and does not leak bytes. |
| mismatched descriptor/object metadata | Shape-invalid or directory-mismatched metadata is discarded; commit request shape mismatches reject. | Prove mismatched metadata or declared ciphertext length fails closed. |
| expired state | Request-path cleanup marks expired sessions/objects, clears capability hashes, and removes staged/object bytes; startup preserves expired metadata without re-exposing bytes. | Prove restore does not resurrect expired sessions or expired object bytes. |
| deleted state | Aborted sessions clear staged parts and invalidate resume tokens; committed sessions are removed after commit. | Prove restore does not resurrect aborted session access or old resume tokens. |
| rejected write residue | Existing reject/quota/capability tests prove malformed, unauthorized, quota, and pressure rejects do not create recoverable state or do not resurrect after same-root restart. | Prove restored fixtures containing reject residue stay fail-closed and do not become sessions/objects. |

## Security Boundaries

### Opaque Ciphertext

qsl-attachments remains an opaque ciphertext service. It stores and returns
ciphertext bytes only. It must not decrypt, inspect, parse, infer, repair, or
reconstruct client plaintext during backup, restore, startup reconciliation, or
future recovery automation.

### No Plaintext

No backup/restore/recovery lane may require plaintext attachment material,
plaintext filenames, media types, message-plane transcript content, or
decryption context. Operators are not expected to recover plaintext from local
state.

### Capability / Hash Handling

Stored capability material is hashed. Future restore/recovery logic must not
log, export, reconstruct, or mint resume tokens or fetch capabilities from
local artifacts. It may preserve already stored hashed capability material only
when the whole coherent resource boundary survives and remains valid under the
chosen semantics.

### Logging

Recovery summaries may include counts and redacted handles. Logs, audit
snapshots, error bodies, docs, and evidence must not expose resume tokens,
fetch capabilities, wrong-capability sentinels, descriptors, ciphertext bytes,
plaintext markers, secret-bearing URLs, or long stable identifiers when short
handles are sufficient.

## Recommended Backup / Restore Policy

Recommended policy:

- support only stopped/quiesced full-root backup plus matching service
  configuration;
- require the entire storage root as the recovery unit;
- treat restore into a new root as valid only when the copied root is a
  coherent full-root snapshot;
- re-expose committed objects only when `object.json` and `ciphertext.bin`
  both survive coherently;
- allow coherent open sessions to be best-effort only if `session.json` and
  all journaled parts survive coherently;
- fail closed for missing, orphaned, malformed, expired, aborted, rejected, or
  mixed-time artifacts;
- keep unsupported hot/live backup, unsupported partial restore, cross-node replication, and
  production backup automation out of scope until separately designed and
  proved.

## Recommended Future NA-0286 Executable Harness Plan

NA-0286 should add executable qsl-attachments harness evidence in the sibling
repository.

First harness shape:

- full-root tempdir copy test: create coherent open session and committed
  object, quiesce the state, copy the entire root to a new root, construct a
  new `AppState` on the copied root, and assert the selected recovery behavior;
- committed-object restore test: prove only paired `object.json` plus
  `ciphertext.bin` with matching length is fetchable;
- open-session restore test: prove a coherent open session is resumable only
  when `session.json` and journaled parts match exactly;
- partial restore fixtures: metadata-only session, part-only session, missing
  part, object metadata only, object bytes only, bad object length, mismatched
  locator/session identifiers, and mixed expired/aborted state;
- reject-residue restore fixtures: malformed, unauthorized, quota/pressure, and
  capability-abuse residues must not become recoverable sessions or objects;
- expiration/deletion restore fixtures: expired objects/sessions and aborted
  sessions must not regain usable capabilities or bytes;
- no-secret logging test: recovery logs, audit snapshots, error bodies, and
  evidence omit capabilities, descriptors, ciphertext, and plaintext sentinels;
- no-production posture check: README/docs/evidence keep unsupported hot/live
  backup, partial restore, cross-node replication, and production operation
  explicit.

## Alternatives Rejected

- Jumping straight to implementation without first freezing the backup,
  partial-restore, and transactional recovery boundary.
- Treating same-root restart proof as completed backup/restore proof.
- Claiming operator backup safety without full-root copy and restore tests.
- Supporting hot/live backup without a durable snapshot/transaction design.
- Best-effort partial restore that reconstructs missing journals, parts, object
  metadata, object bytes, or capability material.
- Adding production backup, production restore automation, cross-node
  replication, object-store backend, or public deployment scope in this
  planning lane.

## Non-Production / No-Production-Readiness Boundary

This plan does not claim production readiness, deployment readiness, public
internet readiness, external review completion, metadata elimination, strong
identity-hiding guarantees, completed backup implementation, completed restore
implementation, unsupported hot/live backup support, unsupported partial restore support, cross-node
replication support, or a completed release-security proof.

The current evidence remains local/loopback/single-node and operator-scoped.

## Not Implemented in NA-0285

NA-0285 does not implement:

- qsl-attachments backup behavior;
- qsl-attachments restore behavior;
- qsl-attachments partial restore behavior;
- qsl-attachments transactional recovery behavior;
- qsl-attachments source changes;
- qsl-attachments test/harness changes;
- qsl-attachments docs changes;
- qsl-server changes;
- qsl-protocol runtime/protocol/crypto/state-machine changes;
- production deployment, public exposure, branch protection, public-safety, or
  dependency changes.
