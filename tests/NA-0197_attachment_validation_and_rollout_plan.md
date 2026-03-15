# NA-0197 Attachment Validation and Rollout Plan

Goals: G4, G5

Status: DRAFT
Date: 2026-03-15
Owner: QSL governance
Scope: Validation and rollout definition for the signal-class attachment program. This file is evidence/planning only and authorizes no runtime changes.

## 1. Purpose

Translate the attachment architecture program into an implementation-sequencing and validation plan that can be used by the next queue items without reopening the architecture decision.

## 2. Validation ladder

Target validation should progress through these size classes:
- `4 MiB`
- `16 MiB`
- `32 MiB`
- `64 MiB`
- `100 MiB`

The ladder exists to prove the streaming/persistence model incrementally and to avoid learning about crash/restart or quota bugs only at the largest class.

## 3. Required validation categories

### 3.1 Descriptor and control-plane correctness

Must prove:
- deterministic descriptor schema validation,
- transcript/auth binding for descriptor identity and integrity fields,
- explicit reject codes for malformed or mismatched descriptors,
- and no state mutation on descriptor reject.

### 3.2 Blob integrity correctness

Must prove:
- per-part integrity failures reject before plaintext release,
- overall commitment mismatch rejects deterministically,
- truncated uploads/downloads do not produce ambiguous local state,
- and final object length mismatch is fail-closed.

### 3.3 Resume and restart behavior

Must prove:
- upload resume after process restart,
- download resume after process restart,
- stale or invalid resume token rejection with no commit mutation,
- and deterministic cleanup of abandoned local temp state.

### 3.4 Quota, expiry, and abuse ceilings

Must prove:
- oversize upload reject,
- outstanding-byte quota reject,
- expired upload-session reject,
- expired committed-object fetch reject,
- and operator-visible deterministic failure codes.

### 3.5 Honest-delivery semantics

Must prove:
- attachment-plane commit is not surfaced as `peer_confirmed`,
- descriptor relay acceptance is still only `accepted_by_relay`,
- and `peer_confirmed` occurs only after recipient retrieval, verification, local persistence, and confirmation handling.

### 3.6 Metadata and log-safety

Must prove:
- no route token, attachment capability, or resume token leakage in markers/logs,
- no plaintext filenames/MIME types on server/operator surfaces,
- and no canonical token-bearing URLs in docs/examples.

## 4. Rollout sequence

### 4.1 NA-0197A — Attachment Descriptor + Control-Plane Contract

Purpose:
- freeze the descriptor fields, transcript binding, reject reasons, and peer-confirm linkage.

Deliverables:
- implementation-grade descriptor document,
- reject-reason registry updates as needed,
- and clear mapping to existing `accepted_by_relay` / `peer_confirmed` semantics.

This is the sole READY follow-on because both service work and qsc work depend on it.

### 4.2 NA-0197B — Attachment Service Contract + Governance Promotion

Purpose:
- define the attachment-plane HTTP/API/storage contract and prepare the chosen attachment-surface repo queue.

Deliverables:
- service contract draft,
- repo-local governance promotion plan,
- and explicit operator invariants for quota, expiry, and log safety.

### 4.3 Repo-local attachment service implementation follow-on

Purpose:
- implement the opaque blob plane in the chosen attachment-surface repo after `NA-0197B` closes.

This implementation step is intentionally not authorized by `NA-0197` itself.

### 4.4 NA-0197C — qsc Streaming Attachment Client Implementation

Purpose:
- implement qsc-side streaming upload/download, local persistence, resume behavior, and user-visible state handling against the stabilized descriptor and service contracts.

Dependencies:
- `NA-0197A` complete,
- `NA-0197B` complete,
- attachment service contract stable enough for client integration.

## 5. Legacy transition gates

The current `<= 4 MiB` in-message file path remains legacy and unchanged until all of the following are true:
- size ladder through `100 MiB` passes,
- restart/resume tests pass,
- quota and expiry behavior pass,
- honest delivery semantics remain explicit,
- and metadata/log-safety checks pass.

Only after those gates may a later directive decide whether to keep the legacy path for small files, deprecate it, or migrate default behavior.

## 6. Evidence expectations for follow-on items

Every follow-on item should report:
- which validation categories were exercised,
- which size classes were covered,
- whether any reject path mutated client or service state,
- whether any log/marker/token leakage was observed,
- and how the current route-token hygiene rule was preserved.

## 7. References

- `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md`
- `qsl/qsl-client/qsc/src/store/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`
- `qsl/qsl-client/qsc/tests/aws_file_medium_boundary_na0192a.rs`
- `README.md`
- `docs/public/INDEX.md`
