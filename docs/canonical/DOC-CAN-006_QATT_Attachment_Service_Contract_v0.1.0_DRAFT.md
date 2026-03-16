# DOC-CAN-006 — QATT Attachment Service Contract
Goals: G4, G5
**Version:** v0.1.0 (DRAFT)
**Status:** Draft (normative intent; implementable)
**Last Updated:** 2026-03-15
**Authority:** Canonical attachment-plane service/session/storage contract for the signal-class attachment path frozen by `NA-0197B`
**Goals:** G4, G5

---

## 0. Scope, goals, and non-goals

This document freezes the implementation-grade service contract for the separate opaque encrypted attachment plane chosen by `NA-0197` and bound to the control-plane descriptor frozen by `DOC-CAN-005`.

It defines:
- the canonical endpoint family and path structure,
- which URL/path elements may be non-secret references,
- exact carriage rules for secret-bearing material,
- session creation, upload, status, commit, abort, and retrieval semantics,
- the session/object state machine,
- deterministic reject classes and no-mutation rules,
- and operator/logging/metadata invariants for the future runtime.

This document does not authorize:
- service runtime implementation,
- qsc streaming attachment implementation,
- qsl-server changes,
- workflow changes,
- or any plaintext attachment handling on service surfaces.

## 1. Contract relationship to existing canonical documents

`DOC-ATT-001` remains the architecture/program source of truth for the control-plane/data-plane split, qsl-server transport-only posture, and validation ladder.

`DOC-CAN-005` remains the canonical source of truth for:
- descriptor payload identity and version,
- descriptor field meanings,
- transcript-bound control-plane data,
- peer-confirm linkage,
- and legacy coexistence rules.

This document defines the service-plane behavior that produces or consumes the following `DOC-CAN-005` fields:
- `attachment_id`,
- `ciphertext_len`,
- `part_size_class`,
- `part_count`,
- `integrity_alg`,
- `integrity_root`,
- `locator_kind`,
- `locator_ref`,
- `fetch_capability`,
- `retention_class`,
- and `expires_at_unix_s`.

No later runtime item may redefine those fields locally.

## 2. Canonical endpoint family and URL rules

### 2.1 Canonical v1 endpoint family

The canonical v1 endpoint family is:
- `POST /v1/attachments/sessions`
- `PUT /v1/attachments/sessions/{session_id}/parts/{part_index}`
- `GET /v1/attachments/sessions/{session_id}`
- `POST /v1/attachments/sessions/{session_id}/commit`
- `DELETE /v1/attachments/sessions/{session_id}`
- `GET /v1/attachments/objects/{locator_ref}`

No other v1 endpoint family is canonical.

### 2.2 Non-secret path elements

The following path elements are canonical and non-secret:
- `session_id`
- `part_index`
- `locator_ref`

Normative rules:
- `session_id` is a non-secret service reference for one upload session only.
- `locator_ref` is a non-secret committed-object reference only.
- `part_index` is a zero-based integer identifying one ciphertext part position.
- None of these values authorizes read, resume, commit, or abort by itself.

### 2.3 Canonical URL secrecy rule

Canonical URLs MUST NOT contain:
- `fetch_capability`,
- `resume_token`,
- any write/commit capability,
- any future token that directly authorizes object access or session continuation,
- route tokens,
- plaintext filenames,
- plaintext media types,
- or any other secret-bearing or transcript-bearing attachment metadata.

Query-string carriage of secret-bearing material is forbidden in v1.

## 3. Secret carriage rules

### 3.1 Request carriage

When a request needs secret-bearing material, v1 uses dedicated request headers only:
- `X-QATT-Resume-Token`
- `X-QATT-Fetch-Capability`

`Authorization` is reserved for any future repo-local authentication layer and is not defined by this canonical contract.

### 3.2 Response carriage

When the service issues a new secret-bearing value, v1 returns it in the JSON response body only:
- session creation returns `resume_token`
- commit returns `fetch_capability`

Services MUST NOT reflect those values into canonical URLs, redirect locations, or operator-facing logs.

### 3.3 Secret-carriage binding rules

Normative rules:
- `X-QATT-Resume-Token` authorizes status, upload, commit, and abort operations for exactly one `session_id`.
- `X-QATT-Fetch-Capability` authorizes retrieval for exactly one committed object addressed by one `locator_ref`.
- Secret-bearing headers are exact-match values; partial, transformed, or URL-decoded variants are invalid.
- Invalid or stale secret-bearing values MUST reject deterministically with no durable state mutation.

## 4. Session creation contract

### 4.1 Create-session request

`POST /v1/attachments/sessions`

Request body fields:
- `attachment_id`
- `ciphertext_len`
- `part_size_class`
- `part_count`
- `integrity_alg`
- `integrity_root`
- `retention_class`

Normative rules:
- These fields MUST already satisfy the shape rules from `DOC-CAN-005`.
- `attachment_id` is a client-supplied stable ciphertext-object identifier and MUST be a lower-case hex string of 64 chars.
- `ciphertext_len`, `part_size_class`, `part_count`, `integrity_alg`, `integrity_root`, and `retention_class` MUST match the values later emitted in the control-plane descriptor.
- The service MUST reject session creation if an active session or committed object already exists for that `attachment_id`.

### 4.2 Create-session response

Successful response body fields:
- `session_id`
- `resume_token`
- `session_state`
- `ciphertext_len`
- `part_size_class`
- `part_count`
- `retention_class`
- `session_expires_at_unix_s`

Normative rules:
- `session_state` MUST be `created` on successful creation.
- `session_id` MUST be non-secret and unique within the service namespace.
- `resume_token` MUST be secret-bearing and unguessable.
- `session_expires_at_unix_s` defines incomplete-upload expiry, not committed-object expiry.

## 5. Part upload contract

### 5.1 Upload-part request

`PUT /v1/attachments/sessions/{session_id}/parts/{part_index}`

Required request properties:
- header `X-QATT-Resume-Token`
- raw ciphertext-part request body
- `Content-Length`

Normative rules:
- `part_index` MUST be in `[0, part_count - 1]`.
- Expected part length is `part_size_bytes(part_size_class)` except for the final part, which MAY be shorter but MUST satisfy the `DOC-CAN-005` ciphertext-length consistency rules.
- The service MUST NOT parse plaintext or any control-plane metadata from part bytes.
- The service MUST bind uploaded part bytes to the session's `attachment_id` and expected shape.

### 5.2 Idempotence and replay rule

Re-upload of an already present `part_index` is permitted only if the ciphertext bytes are identical to the staged bytes already associated with that part.

Normative rules:
- identical replay MAY return success with unchanged state
- mismatched replay MUST reject with no mutation of the existing staged part

### 5.3 Upload-part response

Successful response body fields:
- `session_id`
- `session_state`
- `received_part_index`
- `stored_part_count`
- `missing_part_ranges`

`session_state` MUST be:
- `uploading` while required parts are still missing
- `committable` once all required parts are present and shape-valid

## 6. Session status / resume contract

### 6.1 Status request

`GET /v1/attachments/sessions/{session_id}`

Required request property:
- header `X-QATT-Resume-Token`

### 6.2 Status response

Successful response body fields:
- `session_id`
- `session_state`
- `attachment_id`
- `ciphertext_len`
- `part_size_class`
- `part_count`
- `stored_part_count`
- `missing_part_ranges`
- `retention_class`
- `session_expires_at_unix_s`

Normative rules:
- `missing_part_ranges` is the canonical v1 resume summary.
- The service MAY return a compressed range summary, but it MUST be deterministic for the same stored-part set.
- Status MUST never return secret-bearing material other than the caller-supplied success of the session itself.
- `resumable` is an externally visible open-session condition, not a distinct terminal record; a session in `created`, `uploading`, or `committable` is resumable while not expired or aborted.

## 7. Commit contract

### 7.1 Commit request

`POST /v1/attachments/sessions/{session_id}/commit`

Required request properties:
- header `X-QATT-Resume-Token`
- JSON body fields:
  - `attachment_id`
  - `ciphertext_len`
  - `part_count`
  - `integrity_alg`
  - `integrity_root`
  - `retention_class`

Normative rules:
- The commit body MUST exactly match the session's creation-time shape.
- Commit MUST reject if any required part is absent, any stored part length is invalid, the session is expired, or the session is aborted.
- The service MUST validate complete part presence and final ciphertext-length consistency before commit succeeds.

### 7.2 Commit success semantics

Commit succeeds only when:
- all required parts are present,
- every stored part length is valid for the declared shape,
- session/body shape matches exactly,
- retention policy is still permitted,
- quota/abuse ceilings permit commit,
- and the session is not expired or aborted.

Successful commit response body fields:
- `attachment_id`
- `locator_kind`
- `locator_ref`
- `fetch_capability`
- `ciphertext_len`
- `part_size_class`
- `part_count`
- `integrity_alg`
- `integrity_root`
- `retention_class`
- `expires_at_unix_s`
- `object_state`

Normative rules:
- `locator_kind` MUST be `service_ref_v1`.
- `object_state` MUST be `committed_object`.
- `expires_at_unix_s` is the committed-object retrieval expiry and MUST be the value later carried in the control-plane descriptor.
- Partial or uncommitted objects are never retrievable.
- The committed object is immutable with respect to `attachment_id`, `ciphertext_len`, `part_size_class`, `part_count`, `integrity_alg`, `integrity_root`, `retention_class`, and `expires_at_unix_s`.

## 8. Abort and expiry contract

### 8.1 Abort request

`DELETE /v1/attachments/sessions/{session_id}`

Required request property:
- header `X-QATT-Resume-Token`

Normative rules:
- Abort is valid only for non-committed open sessions.
- Abort MUST invalidate any future use of that `session_id` and its `resume_token`.
- Abort MUST discard staged ciphertext parts for that session.

### 8.2 Abort success semantics

Successful abort response body fields:
- `session_id`
- `session_state`

`session_state` MUST be `aborted_session`.

### 8.3 Expiry semantics

Normative rules:
- incomplete sessions expire at `session_expires_at_unix_s`
- committed objects expire at `expires_at_unix_s`
- expiry MUST invalidate the relevant secret-bearing capability for future use
- expired sessions and expired objects MAY be garbage-collected, but expiry semantics apply before physical deletion

## 9. Retrieval and range/resume contract

### 9.1 Retrieval request

`GET /v1/attachments/objects/{locator_ref}`

Required request property:
- header `X-QATT-Fetch-Capability`

Optional request property:
- single-range `Range: bytes=start-end`

Normative rules:
- retrieval is valid only for committed, unexpired objects
- `locator_ref` by itself is insufficient to authorize retrieval
- multiple byte ranges are not canonical in v1 and MUST reject
- the service returns ciphertext bytes only

### 9.2 Retrieval preconditions

Bytes may be returned only if:
- `locator_ref` identifies one committed object
- `X-QATT-Fetch-Capability` matches exactly for that object
- the object is not expired
- the requested range, if present, is syntactically and semantically valid within `[0, ciphertext_len - 1]`

### 9.3 Retrieval success semantics

Normative rules:
- full retrieval returns the complete ciphertext object
- range retrieval returns only the requested ciphertext byte span
- retrieval MUST preserve ciphertext byte order exactly as committed
- the service MUST NOT synthesize, pad, transform, or recompress ciphertext bytes
- retrieval responses MUST expose enough length metadata for the caller to validate range progress safely

## 10. Session/object state machine

### 10.1 Canonical states

Session states:
- `created`
- `uploading`
- `committable`
- `aborted_session`
- `expired_session`

Object states:
- `committed_object`
- `expired_object`

Derived open-session condition:
- `resumable` means the session is in `created`, `uploading`, or `committable` and is neither aborted nor expired

Invalid secret usage condition:
- invalid or stale `resume_token` / `fetch_capability` is a reject condition only; it is not a durable object/session state transition by itself

### 10.2 Canonical transitions

Allowed transitions:
- `created -> uploading`
- `created -> committable` if all parts are uploaded in one uninterrupted flow
- `uploading -> uploading`
- `uploading -> committable`
- `created|uploading|committable -> aborted_session`
- `created|uploading|committable -> expired_session`
- `committable -> committed_object`
- `committed_object -> expired_object`

Forbidden transitions:
- any transition out of `aborted_session`
- any transition out of `expired_session`
- any transition from `committed_object` back to session states
- any transition from `expired_object` back to active retrieval

## 11. Reject taxonomy and no-mutation rules

All non-successful operations MUST return a deterministic canonical `reason_code` in an operator-safe error body.

Canonical reject classes:
- `REJECT_QATTSVC_SECRET_URL_PLACEMENT`
- `REJECT_QATTSVC_SESSION_SHAPE`
- `REJECT_QATTSVC_SESSION_STATE`
- `REJECT_QATTSVC_RESUME_TOKEN`
- `REJECT_QATTSVC_PART_INDEX`
- `REJECT_QATTSVC_PART_LENGTH`
- `REJECT_QATTSVC_PART_REPLAY_MISMATCH`
- `REJECT_QATTSVC_COMMIT_INCOMPLETE`
- `REJECT_QATTSVC_COMMIT_MISMATCH`
- `REJECT_QATTSVC_LOCATOR_UNKNOWN`
- `REJECT_QATTSVC_FETCH_CAPABILITY`
- `REJECT_QATTSVC_RANGE`
- `REJECT_QATTSVC_EXPIRED`
- `REJECT_QATTSVC_POLICY`
- `REJECT_QATTSVC_QUOTA`
- `REJECT_QATTSVC_ABUSE`

| Reason code | Trigger | Durable mutation allowed? | Temp staging | Notes |
|---|---|---|---|---|
| `REJECT_QATTSVC_SECRET_URL_PLACEMENT` | secret-bearing material appears in path/query/canonical URL form | no | discard request parse state only | configuration/client bug |
| `REJECT_QATTSVC_SESSION_SHAPE` | malformed create/commit body or mismatch with `DOC-CAN-005` shape rules | no | discard request parse state only | no session/object creation |
| `REJECT_QATTSVC_SESSION_STATE` | operation invalid for current canonical state | no | unchanged | state remains truthful |
| `REJECT_QATTSVC_RESUME_TOKEN` | missing, stale, malformed, or wrong `X-QATT-Resume-Token` | no | unchanged | no state transition |
| `REJECT_QATTSVC_PART_INDEX` | invalid `part_index` or disallowed duplicate index semantics | no new durable part state | unchanged existing staged parts | no mutation of valid prior parts |
| `REJECT_QATTSVC_PART_LENGTH` | part length inconsistent with declared shape | no new durable part state | discard request body only | no mutation of existing valid staged parts |
| `REJECT_QATTSVC_PART_REPLAY_MISMATCH` | re-uploaded part index bytes differ from existing staged bytes | no | preserve prior staged part | fail-closed idempotence |
| `REJECT_QATTSVC_COMMIT_INCOMPLETE` | commit attempted while parts missing | no commit/object mutation | preserve staged parts | session remains truthful/open if not expired |
| `REJECT_QATTSVC_COMMIT_MISMATCH` | commit body mismatches session shape | no commit/object mutation | preserve staged parts | session remains truthful/open if not expired |
| `REJECT_QATTSVC_LOCATOR_UNKNOWN` | `locator_ref` does not identify a committed object | no | unchanged | no object creation |
| `REJECT_QATTSVC_FETCH_CAPABILITY` | missing, stale, malformed, or wrong `X-QATT-Fetch-Capability` | no | unchanged | no retrieval mutation |
| `REJECT_QATTSVC_RANGE` | invalid or multi-range request | no | unchanged | no partial-byte mutation |
| `REJECT_QATTSVC_EXPIRED` | expired session or object | no new active-state mutation | cleanup MAY proceed under expiry handling | expiry state remains truthful |
| `REJECT_QATTSVC_POLICY` | retention class or other policy violation | no | unchanged | no session/object creation or promotion |
| `REJECT_QATTSVC_QUOTA` | size/outstanding-byte/object ceiling violation | no | unchanged | deterministic quota reject |
| `REJECT_QATTSVC_ABUSE` | concurrency, retry, or range abuse ceiling hit | no | unchanged | deterministic abuse reject |

## 12. Operator, logging, and metadata invariants

### 12.1 Logging rules

The future runtime MUST NOT log:
- plaintext attachment bytes,
- plaintext filenames or media types,
- `resume_token`,
- `fetch_capability`,
- any future write capability,
- raw ciphertext bytes,
- or canonical URLs containing secret-bearing material.

The future runtime MAY log, at full-value granularity:
- `session_id`
- `locator_ref`
- `attachment_id`
- `part_index`
- `ciphertext_len`
- `part_count`
- `retention_class`
- canonical `reason_code`
- state transitions

### 12.2 Metadata minimization rules

Unavoidable metadata:
- `session_id`
- `locator_ref`
- `attachment_id`
- ciphertext length
- part count / part size class
- retention class
- session/object expiry timestamps
- upload/download timing
- service-side access events

Prohibited metadata:
- plaintext filenames
- plaintext media types
- transcript contents
- peer confirmation handles
- route tokens
- secret-bearing capability values
- raw plaintext bytes

Long-term service storage is limited to committed-object bytes plus the minimum object metadata needed to enforce retrieval and expiry.

Session-lifetime-only state includes:
- `resume_token`
- upload progress
- staged part presence
- abort eligibility
- and open-session expiry tracking

### 12.3 Quota and abuse ceilings

The canonical contract requires deterministic enforcement of:
- per-session byte ceiling equal to declared `ciphertext_len`
- per-object byte ceiling equal to committed `ciphertext_len`
- one active session per `attachment_id`
- incomplete-upload expiry
- committed-object expiry
- bounded concurrent open sessions per principal or equivalent service policy subject
- bounded repeated invalid-token attempts
- bounded repeated invalid-range attempts

The runtime MAY tighten these ceilings, but MUST NOT relax them below the contract minimums.

## 13. Source-of-truth mapping for later runtime work

The future `qsl-attachments` runtime item MUST implement exactly:
- the endpoint family from §2,
- the secret carriage rules from §3,
- the create/upload/status/commit/abort/retrieval semantics from §§4-9,
- the state machine from §10,
- the reject taxonomy from §11,
- and the operator/logging/metadata invariants from §12.

`DOC-CAN-005` remains authoritative for any field that later appears in the control-plane descriptor.

`DOC-ATT-001` remains authoritative for:
- qsl-server staying transport-only,
- no plaintext attachment storage on service surfaces,
- the validation ladder,
- and the later qsc/client rollout sequence.

## 14. Safe examples

### 14.1 Create-session request

```json
{
  "attachment_id": "<ATTACHMENT_ID_HEX_64>",
  "ciphertext_len": 10485760,
  "part_size_class": "p256k",
  "part_count": 40,
  "integrity_alg": "sha512_merkle_v1",
  "integrity_root": "<INTEGRITY_ROOT_HEX_128>",
  "retention_class": "standard"
}
```

### 14.2 Commit success response

```json
{
  "attachment_id": "<ATTACHMENT_ID_HEX_64>",
  "locator_kind": "service_ref_v1",
  "locator_ref": "<LOCATOR_REF>",
  "fetch_capability": "<FETCH_CAPABILITY>",
  "ciphertext_len": 10485760,
  "part_size_class": "p256k",
  "part_count": 40,
  "integrity_alg": "sha512_merkle_v1",
  "integrity_root": "<INTEGRITY_ROOT_HEX_128>",
  "retention_class": "standard",
  "expires_at_unix_s": 1778880000,
  "object_state": "committed_object"
}
```

## 15. References

- `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- `tests/NA-0197_attachment_validation_and_rollout_plan.md`
