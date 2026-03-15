# DOC-ATT-001 — Signal-Class Attachment Architecture Program v0.1.0 DRAFT

Goals: G4, G5

Status: DRAFT
Date: 2026-03-15
Owner: QSL governance
Scope: Attachment architecture and program definition only. No qsc, qsl-server, website, workflow, or wire/runtime changes are authorized by this document.

## 1. Purpose

Define the smallest truthful attachment architecture that can reach the program target class of about 100 MiB without mutating the current message-plane file path or pretending that a constant bump is sufficient.

This document is design/program-definition only. It does not authorize:
- changing the current qsc runtime behavior,
- enlarging the current relay inbox/body limits,
- embedding large encrypted blobs in the current message plane,
- changing route-token carriage,
- or weakening the current `accepted_by_relay` versus `peer_confirmed` distinction.

## 2. Current baseline and architectural gap

Current qsc file transfer is intentionally bounded and message-plane oriented.

### 2.1 Current hard limits

Current qsc store constants define:
- default max file size: `256 KiB`
- hard file-size ceiling: `4 MiB`
- default chunk size: `16 KiB`
- hard chunk-size ceiling: `16 KiB`
- default max chunks: `64`
- hard max chunks: `256`

The current 4 MiB ceiling is the product of the chunk ceiling and chunk-count ceiling.

### 2.2 Current sender behavior

Current sender behavior reads the entire local file into memory, rejects above the configured bound, chunks the payload into `file_chunk` control payloads, and sends both chunk payloads and the final `file_manifest` through the same relay inbox/message plane used for ordinary messages.

### 2.3 Current persistence and receive behavior

Current partial transfer state is persisted inside the existing encrypted timeline store as transfer metadata plus hex-encoded chunk bodies. On receive, qsc reconstructs the entire file in memory before the final timeline file entry is committed.

### 2.4 Honest-delivery semantics that already exist

Current file delivery is deliberately split:
- `accepted_by_relay` means the relay accepted the transport operation.
- `peer_confirmed` means a valid end-to-end completion acknowledgement was processed.

That distinction is already exercised by the file-transfer regression tests and must survive any attachment redesign.

### 2.5 Why a constant bump is rejected

A 100 MiB target is about twenty-five times the current 4 MiB ceiling.

A constant bump is not acceptable because it would preserve the wrong shape:
- the current message plane would still carry blob data instead of compact control metadata,
- the current sender would still assume whole-file reads,
- the current receive path would still rebuild the full file in memory,
- the encrypted timeline store would still accumulate transfer chunks instead of durable streaming state,
- and the relay inbox would still be misused as a blob queue rather than a small-message/control surface.

Increasing chunk size is also not a safe escape hatch because the current ceiling is explicitly bound to the existing Suite-2 wire body-length considerations and already has fail-closed boundary tests that reject larger chunk sizes.

## 3. Design requirements

The follow-on implementation program must preserve all of the following:
- fail-closed integrity semantics,
- deterministic reject behavior with no mutation on invalid or stale resume state,
- no plaintext attachment storage on relay/server surfaces,
- honest delivery semantics (`accepted_by_relay` remains distinct from `peer_confirmed`),
- current route-token migration posture (no capability-like secrets in canonical URLs),
- legacy file-transfer compatibility until the new path is proven,
- and client restart/resume semantics that do not depend on timeline-embedded blob persistence.

## 4. Candidate architecture set

| Option | Summary | 100 MiB viability | Integrity / delivery fit | Resume / quota / retention fit | Decision |
|---|---|---|---|---|---|
| `A0` | Raise current qsc file-size / chunk-count limits only | Poor | Preserves the wrong control/data-plane coupling | Poor | Reject |
| `A1` | Keep current relay inbox as blob plane, just bigger | Poor | Weak separation; relay acceptance becomes blob-queue acceptance | Poor | Reject |
| `A2` | Add a distinct opaque attachment surface adjacent to qsl-server in the same trust domain | Good | Good | Good | Plausible but not chosen |
| `A3` | Separate attachment service boundary with opaque blob plane and message-plane descriptors | Good | Best | Best | Choose |

### 4.1 Rejection of A0

`A0` is rejected because it keeps all current scaling bottlenecks in place and only enlarges their blast radius.

### 4.2 Rejection of A1

`A1` is rejected because the current relay inbox contract is optimized for bounded opaque messages and queue semantics, not resumable large-object storage. Reusing the inbox as the blob plane would collapse two distinct delivery meanings into one queue surface and would force qsl-server to absorb retention, resume, quota, and large-payload operational concerns that are outside its current transport-only boundary.

### 4.3 Why A3 is the smallest truthful choice

`A3` is chosen because it preserves the current relay/message plane as the control and transcript-bearing surface while moving large encrypted blob carriage into a purpose-built opaque attachment plane. It is smaller and truer than redesigning qsl-server into a mixed relay/object service, and it leaves a clean repo-local queue seam for the attachment surface without reopening current qsc or qsl-server behavior in this item.

Public Signal clean-room reference supports the split between a mailbox/control surface and separately stored encrypted attachments with bounded retention, but QSL must define its own contract and not copy Signal implementation details. See: https://signal.org/blog/a-synchronized-start-for-linked-devices/ and https://signal.org/blog/signal-is-expensive/.

## 5. Chosen architecture and boundary model

### 5.1 Control plane

The existing QSP/QSC message plane remains the control plane.

It carries an end-to-end protected attachment descriptor that is small enough to fit ordinary message transport expectations. The descriptor is the only peer-visible attachment contract inside the transcript-bearing message path.

### 5.2 Data plane

A new opaque attachment plane carries encrypted blob parts only.

The attachment plane is responsible for:
- upload session creation,
- resumable part transfer,
- object commit/finalization,
- bounded retention/expiry,
- download by capability-bearing client,
- and operator-safe quota/abuse ceilings.

It does not parse plaintext attachment content or QSP control semantics.

### 5.3 Boundary and ownership decision

The chosen boundary is a separate attachment service boundary.

This is expected to be a logically distinct and later separately deployable surface, even if an operator chooses to co-locate it with qsl-server in the same operational environment. qsl-server remains transport-only and is not promoted into the blob service boundary by this design.

Ownership split:
- qsl-protocol owns the descriptor/control-plane contract and the cross-boundary invariants.
- the future attachment-surface repo owns the service contract and runtime behavior.
- qsc owns streaming upload/download, local persistence, resume UX, and final confirmation behavior.

### 5.4 Legacy compatibility decision

The current `<= 4 MiB` in-message file path is retained temporarily as a legacy path during the transition.

This design does not deprecate or remove it immediately. Removal criteria are deferred until the new streaming attachment path proves:
- descriptor correctness,
- blob integrity,
- restart/resume behavior,
- quota/retention enforcement,
- and honest delivery semantics across the target size ladder up to the 100 MiB class.

## 6. Attachment descriptor responsibilities

The message-plane descriptor must carry enough information for the recipient to fetch, verify, and persist the blob without trusting the attachment service.

Minimum responsibilities:
- stable `attachment_id`,
- protocol/schema version,
- sender-selected local metadata that the peer needs after decryption (for example `filename_hint` and `media_type`),
- plaintext length,
- ciphertext length,
- part size class and part count,
- blob integrity commitment (root commitment over the ciphertext object/parts),
- download locator/capability reference or equivalent retrieval material,
- retention/expiry class identifier,
- optional sender-generated transfer correlation id for UI-only local bookkeeping,
- and peer confirmation binding material.

Fields that MUST be transcript-bound or otherwise authenticated in the message plane:
- `attachment_id`,
- descriptor version,
- plaintext length,
- ciphertext length,
- part count / size class,
- integrity commitment,
- retention class,
- any retrieval capability version/identifier,
- and the message-level confirmation binding handle.

Resume session identifiers used only for upload recovery are service-plane state and do not need to be permanently transcript-bound, but any committed object identity derived from them must not diverge from the descriptor once sent.

## 7. Security, integrity, and abuse-control model

### 7.1 Integrity model

Integrity composes in two layers:
- the descriptor is authenticated by the existing message plane,
- the blob is verified against descriptor-bound commitments before the recipient releases plaintext.

Required properties:
- no descriptor means no attachment fetch,
- no committed blob means no descriptor send,
- any mismatch in size, part count, commitment, or final object length is a deterministic fail-closed reject,
- invalid or replayed peer confirmation must not mutate attachment completion state,
- and the attachment service must not be trusted for end-to-end integrity beyond bounded storage semantics.

### 7.2 Confidentiality model

Plaintext attachment bytes must exist only on client surfaces.

The attachment plane stores opaque encrypted blob parts and bounded operational metadata only. Plaintext filenames, MIME hints, and any other peer-visible metadata stay inside the end-to-end protected descriptor/control plane. Raw attachment plaintext on server surfaces is not justified by this design.

### 7.3 Resume model

Resumable upload/download is mandatory at the architecture level.

Client-side persistent state must include at minimum:
- local attachment journal id,
- source file path or destination temp path,
- declared lengths and part size class,
- uploaded/downloaded part bitmap or equivalent progress cursor,
- attachment service session id,
- resume token or equivalent capability material,
- integrity commitment state,
- and whether descriptor send / peer confirmation is still pending.

Service-side session state must be immutable with respect to attachment identity parameters once created. Stale, malformed, or mismatched resume tokens must reject deterministically with no partial commit mutation.

### 7.4 Quota, retention, and abuse control

The service contract must define:
- per-attachment hard size caps,
- per-principal outstanding byte ceilings,
- incomplete-upload expiry,
- committed-object retention expiry,
- maximum concurrent sessions,
- and deterministic operator-visible failure codes for quota, expiry, and abuse rejection.

The design target is the 100 MiB class, but rollout validation should use a size ladder (`4 MiB`, `16 MiB`, `32 MiB`, `64 MiB`, `100 MiB`) rather than jumping directly to the top size.

### 7.5 Metadata minimization target

The attachment plane cannot hide all metadata. The bounded target is:
- unavoidable: upload/download timing, ciphertext length, part count, expiry class, object/session ids, and service-side access events,
- avoidable and therefore excluded from the attachment service: plaintext filenames, plaintext MIME types, message transcript contents, route tokens in URLs, and raw attachment plaintext.

Capability-bearing attachment identifiers or access tokens must follow the same hygiene rule as migrated route tokens: canonical APIs and docs must not place them in URLs when a header/body carriage shape is available.

## 8. Client streaming, persistence, and UX model

### 8.1 Upload path

1. qsc opens the local file as a stream; no whole-file-in-memory assumption.
2. qsc derives/loads the attachment encryption context and computes ciphertext parts incrementally.
3. qsc persists upload journal state before and during transfer.
4. qsc uploads encrypted parts to the attachment plane using resumable session state.
5. qsc finalizes the object and receives a committed attachment identity/capability package.
6. only after successful object commit does qsc send the message-plane descriptor.
7. relay acceptance of the descriptor remains `accepted_by_relay`, not peer receipt.

### 8.2 Download path

1. qsc receives the message-plane descriptor.
2. qsc persists a download journal and begins attachment-plane fetch.
3. qsc streams ciphertext parts through integrity verification and decrypts to a temp file.
4. qsc fsyncs and atomically promotes the completed local file only after full verification succeeds.
5. only then may qsc emit recipient-side completion logic that can lead to sender `peer_confirmed`.

### 8.3 Local persistence model

The current encrypted timeline store must not remain the blob persistence surface for the new path.

The follow-on qsc design should use a dedicated encrypted attachment journal/staging area for:
- resumable upload state,
- resumable download state,
- temp ciphertext/plaintext staging,
- final local attachment metadata,
- and deterministic cleanup of abandoned or expired local artifacts.

What must survive restart:
- session ids / resume tokens,
- part progress,
- integrity commitments,
- pending descriptor-send state,
- pending peer-confirm state,
- and cleanup eligibility.

What may be recomputed:
- transient UI progress projections,
- non-authoritative rate estimates,
- and derived human-readable status summaries.

### 8.4 Delivery semantics

The redesign must preserve the current semantic split and extend it rather than collapsing it:
- attachment-surface acceptance means the encrypted blob is durably staged or committed on the attachment plane; it is not message delivery,
- `accepted_by_relay` means the descriptor message was accepted into the relay/message plane,
- `peer_confirmed` means the peer completed descriptor processing, blob retrieval, integrity verification, local persistence, and valid confirmation handling.

### 8.5 UX and logging boundaries

User-visible progress states should distinguish at least:
- encrypting,
- uploading,
- upload paused/resumable,
- upload committed,
- descriptor sent / `accepted_by_relay`,
- peer pending retrieval,
- download verifying,
- completed / `peer_confirmed`,
- failed deterministic reject,
- expired / quota rejected.

Logs and machine-readable markers must never print:
- raw attachment plaintext,
- access tokens or resume tokens,
- route tokens,
- decrypted metadata not already user-visible locally,
- or operator-unsafe URLs containing capability material.

## 9. Validation and rollout plan

### 9.1 Validation categories

Required validation categories for follow-on items:
- descriptor schema and transcript-binding correctness,
- blob integrity correctness (good path and tamper path),
- resumable upload/download across restart,
- deterministic stale/invalid resume reject behavior,
- quota and retention enforcement,
- honest delivery semantics (`accepted_by_relay` vs `peer_confirmed`),
- metadata/log-safety checks,
- operator-safe expiry/cleanup behavior,
- and staged large-size cases through the target ladder.

### 9.2 Rollout order

Phase 0: current legacy path remains unchanged.

Phase 1 (`NA-0197A`): define the descriptor and control-plane contract to implementation-grade precision.

Phase 2 (`NA-0197B`): define the attachment service contract and prepare the chosen attachment-surface repo queue/governance lane.

Phase 3 (repo-local follow-on after `NA-0197B`): implement the attachment service boundary using the committed service contract.

Phase 4 (`NA-0197C`): implement qsc streaming upload/download, local persistence, resume, and UI/logging behavior against the stabilized descriptor and service contract.

Phase 5: transition policy decision on the legacy `<= 4 MiB` path after the new path proves size-ladder, crash/restart, and honest-delivery invariants.

### 9.3 Legacy-path transition policy

The legacy path remains available during the transition.

The later implementation program may choose one of two operational modes once the new path exists:
- explicit opt-in attachment-plane mode, or
- automatic selection for sizes above a conservative threshold.

This program item does not authorize that choice yet. It only records that legacy removal/deprecation must wait for evidence, not enthusiasm.

## 10. Queue implications

This architecture is explicit enough to spawn the next implementation sequence:
- `NA-0197A — Attachment Descriptor + Control-Plane Contract`
- `NA-0197B — Attachment Service Contract + Governance Promotion`
- `NA-0197C — qsc Streaming Attachment Client Implementation`

`NA-0197A` is the smallest and safest first step because both the service boundary and the client depend on an implementation-grade descriptor contract, transcript binding rules, and reject semantics before runtime work should start.

## 11. References

- `qsl/qsl-client/qsc/src/store/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`
- `qsl/qsl-client/qsc/tests/aws_file_medium_boundary_na0192a.rs`
- `qsl/qsl-client/qsc/tests/two_client_local_runbook_na0182.rs`
- `README.md`
- `docs/public/INDEX.md`
- `docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md`
- `docs/archive/testplans/NA-0119_file_transfer_mvp_plan.md`
- `docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md`
- qsl-server `README.md` (boundary posture reference only; no server changes authorized here)
- Signal blog references (clean-room inspiration only):
  - https://signal.org/blog/a-synchronized-start-for-linked-devices/
  - https://signal.org/blog/signal-is-expensive/
