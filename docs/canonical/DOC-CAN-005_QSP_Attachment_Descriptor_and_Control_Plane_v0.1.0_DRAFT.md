# DOC-CAN-005 — QSP Attachment Descriptor + Control-Plane Contract
Goals: G4, G5
**Version:** v0.1.0 (DRAFT)  
**Status:** Draft (normative intent; implementable)  
**Last Updated:** 2026-03-15  
**Authority:** Canonical control-plane contract for the signal-class attachment path chosen by NA-0197  
**Goals:** G4, G5

---

## 0. Scope, goals, and non-goals

This document freezes the implementation-grade message-plane contract for signal-class attachments.

It defines:
- the canonical descriptor payload identity and version,
- the exact transmitted field set and field classes,
- transcript/authentication binding rules,
- descriptor-carried attachment encryption-context rules,
- peer-confirm linkage rules,
- deterministic reject classes and no-mutation behavior,
- and coexistence rules with the current `<= 4 MiB` legacy message-plane file path.

This document does not authorize:
- attachment-service runtime implementation,
- qsc streaming attachment implementation,
- qsl-server changes,
- workflow changes,
- or legacy-path runtime changes.

## 1. Type identity, versioning, and path selection

### 1.1 Canonical control-plane payload identity

The new attachment control-plane payload is identified by:
- `t = "attachment_descriptor"`
- `v = 1`

Normative rule:
- Any payload claiming the signal-class attachment path MUST use exactly this type/version pair.
- Any payload with `t = "attachment_descriptor"` and unsupported `v` MUST be rejected fail-closed.

### 1.2 Path families

There are exactly two attachment/file-transfer path families during coexistence:
- legacy message-plane path: `file_chunk` and `file_manifest`
- signal-class attachment path: `attachment_descriptor`

A single logical send attempt MUST choose exactly one family before any payloads are emitted.

### 1.3 Message-plane role

The message plane carries only the peer-visible descriptor contract for the new path. Encrypted blob parts, upload sessions, resume cursors, and storage/runtime state remain off the message plane.

## 2. Descriptor field set and field classes

### 2.1 Transmitted peer-visible descriptor fields

The canonical transmitted descriptor payload is:

| Field | Type / domain | Required | Transcript-bound | Notes |
|---|---|---|---|---|
| `v` | `u8`, fixed `1` | yes | yes | descriptor version |
| `t` | fixed string `"attachment_descriptor"` | yes | yes | payload identity |
| `attachment_id` | lower-case hex, 64 chars | yes | yes | stable identifier for the committed ciphertext object; not a session id |
| `plaintext_len` | `u64`, `> 0` | yes | yes | final plaintext length in bytes |
| `ciphertext_len` | `u64`, `> 0` | yes | yes | committed ciphertext-object length in bytes |
| `part_size_class` | enum: `p64k`, `p256k`, `p1024k` | yes | yes | fixed byte-size registry in §2.2 |
| `part_count` | `u32`, `> 0` | yes | yes | MUST equal `ceil(ciphertext_len / part_size_bytes(part_size_class))` |
| `integrity_alg` | fixed string `"sha512_merkle_v1"` | yes | yes | exact commitment scheme in §2.3 |
| `integrity_root` | lower-case hex, 128 chars | yes | yes | full 64-byte Merkle root digest |
| `locator_kind` | fixed string `"service_ref_v1"` | yes | yes | peer-visible retrieval shape selector |
| `locator_ref` | base64url token, 1..128 chars | yes | yes | non-secret service/object reference |
| `fetch_capability` | base64url token, 32..255 chars | yes | yes | peer-visible secret retrieval capability; MUST NOT appear in canonical URLs |
| `enc_ctx_alg` | fixed string `"chacha20poly1305_part_v1"` | yes | yes | attachment encryption-context selector; exact semantics in `DOC-CAN-007` |
| `enc_ctx_b64u` | base64url token, exactly 55 chars | yes | yes | secret-bearing descriptor-carried attachment context package; exact decode rules in `DOC-CAN-007`; MUST NOT appear in canonical URLs or service APIs |
| `retention_class` | enum: `short`, `standard`, `extended` | yes | yes | policy class, not a raw TTL |
| `expires_at_unix_s` | `u64`, `> 0` | yes | yes | absolute expiry for committed-object retrieval |
| `confirm_requested` | boolean | yes | yes | whether recipient confirmation is requested |
| `confirm_handle` | lower-case hex, 24 chars | conditional | yes | required iff `confirm_requested = true`; omitted otherwise |
| `filename_hint` | UTF-8 basename, 1..255 bytes, no path separators | no | yes if present | peer-visible post-decryption hint |
| `media_type` | lower-case ASCII `type/subtype`, 3..127 chars | no | yes if present | peer-visible post-decryption hint |

Normative rules:
- Unknown transmitted fields are not permitted in v1.
- `filename_hint` and `media_type` are the only optional peer-visible metadata fields in v1.
- `enc_ctx_alg` and `enc_ctx_b64u` are required in every v1 descriptor and MUST satisfy `DOC-CAN-007`.
- If `confirm_requested = false`, `confirm_handle` MUST be absent.
- If `confirm_requested = true`, `confirm_handle` MUST be present and valid per §4.2.

### 2.2 Fixed registries used by the descriptor

Part-size-class registry:
- `p64k = 65,536` bytes
- `p256k = 262,144` bytes
- `p1024k = 1,048,576` bytes

Retention-class registry:
- `short`
- `standard`
- `extended`

Unknown enum values MUST reject.

### 2.3 Integrity commitment semantics

`integrity_alg = "sha512_merkle_v1"` means:
- leaf hash for part `i`: `SHA-512(0x00 || u32be(i) || u64be(part_len) || ciphertext_part_bytes)`
- parent hash: `SHA-512(0x01 || left_child || right_child)`
- odd-node rule: duplicate the final child when a level has odd width
- part ordering is ascending zero-based part index
- `integrity_root` is the lower-case hex encoding of the final 64-byte root digest

All commitment material is over ciphertext parts exactly as stored/fetched from the attachment plane.

### 2.4 Local-only fields that MUST NOT be transmitted

The following are local-only and MUST NOT appear in the peer-visible descriptor:
- local attachment journal id,
- source file path,
- destination temp/final local paths,
- decoded attachment content-encryption key,
- decoded attachment nonce prefix,
- local UI correlation id,
- rate estimates and retry counters,
- target-device local marker / routing cache,
- sender-side staging timestamps,
- and any local cleanup eligibility flags.

### 2.5 Service-only/session fields that stay off the message plane

The following are service-plane or local/session state and MUST stay off the message plane:
- upload session id,
- upload resume token,
- download resume token,
- service-side progress cursor,
- per-part commit receipts or storage etags,
- storage bucket/path coordinates,
- abuse/quota counters,
- and any service-internal object-location metadata.

`locator_ref`, `fetch_capability`, `enc_ctx_alg`, and `enc_ctx_b64u` are the only v1 retrieval/decrypt-context fields that cross into the peer-visible descriptor.

## 3. Transcript binding and receiver comparison rules

### 3.1 Authentication rule

The entire transmitted descriptor payload is authenticated by the existing message plane. No attachment fetch, attachment journal promotion, or confirmation processing may proceed from an unauthenticated or partially parsed descriptor.

### 3.2 Exact compare set

The receiver MUST parse and compare exactly the following fields before any attachment-plane fetch begins:
- `v`
- `t`
- `attachment_id`
- `plaintext_len`
- `ciphertext_len`
- `part_size_class`
- `part_count`
- `integrity_alg`
- `integrity_root`
- `locator_kind`
- `locator_ref`
- `fetch_capability`
- `enc_ctx_alg`
- `enc_ctx_b64u`
- `retention_class`
- `expires_at_unix_s`
- `confirm_requested`
- `confirm_handle` if present
- `filename_hint` if present
- `media_type` if present

Fail-closed rule:
- any malformed field,
- any missing required field,
- any invalid field-domain value,
- any inconsistency between size/count/commitment fields,
- or any descriptor/confirmation mismatch later defined in §4 and §6
MUST reject before plaintext release or final delivery-state transition.

### 3.3 Size/count consistency rule

For `part_size_bytes = registry(part_size_class)`:
- `part_count MUST equal ceil(ciphertext_len / part_size_bytes)`
- `ciphertext_len MUST be >= 1`
- `ciphertext_len MUST be <= part_count * part_size_bytes`
- `ciphertext_len MUST be > (part_count - 1) * part_size_bytes`

The receiver and later service/client implementations MUST treat any violation as a deterministic descriptor reject.

### 3.4 Encryption-context and ciphertext-shape consistency

For `enc_ctx_alg = "chacha20poly1305_part_v1"`, `DOC-CAN-007` additionally fixes:
- `part_plaintext_capacity = part_size_bytes(part_size_class) - 16`
- `part_count MUST equal ceil(plaintext_len / part_plaintext_capacity)`
- `ciphertext_len MUST equal plaintext_len + (16 * part_count)`

Fail-closed rule:
- any descriptor whose `enc_ctx_*`, `plaintext_len`, `ciphertext_len`, `part_size_class`, or `part_count` values cannot simultaneously satisfy `DOC-CAN-007`
MUST reject before fetch, decryption, plaintext release, or final delivery-state transition.

## 4. Peer-confirm linkage and delivery semantics

### 4.1 Distinct delivery milestones

The contract preserves three distinct milestones:
- attachment-plane commit: encrypted blob durably committed on the attachment plane; not message delivery
- `accepted_by_relay`: descriptor message accepted by the relay/message plane
- `peer_confirmed`: recipient completed descriptor validation, blob retrieval, integrity verification, local persistence, and valid confirmation handling

No later item may collapse these meanings.

### 4.2 Confirmation-handle derivation

If `confirm_requested = true`, the sender MUST derive `confirm_handle` as:
- `confirm_material = "QATT-CONFIRM-V1|" || attachment_id || "|" || dec(plaintext_len) || "|" || dec(ciphertext_len) || "|" || part_size_class || "|" || dec(part_count) || "|" || integrity_alg || "|" || integrity_root || "|" || retention_class || "|" || dec(expires_at_unix_s)`
- `confirm_handle = lower_hex(first_12_bytes(SHA-512(confirm_material)))`

This yields the required 24-char lower-case hex handle.

Canonical encoding rule:
- `confirm_material` is ASCII/UTF-8 text.
- `dec(...)` is the minimal base-10 ASCII encoding with no leading zeroes except for the value `0`.
- enum fields use their exact canonical identifier strings from this document.

Normative note:
- `NA-0197CA` adds no extra confirmation payload field for `enc_ctx_*`; the transcript-bound descriptor plus `integrity_root` already bind the attachment encryption context to the confirmation handle inputs.

### 4.3 Confirmation payload linkage rule

The later confirmation payload MUST echo exactly:
- `attachment_id`
- `confirm_handle`

Sender-side confirmation processing MUST require:
- an existing outstanding descriptor record for that `attachment_id`
- `confirm_requested = true`
- exact `confirm_handle` match
- the sender-side state already recorded at least descriptor send completion and relay acceptance

Invalid, replayed, early, or mismatched confirmation MUST NOT set `peer_confirmed`.

## 5. Legacy coexistence and path-selection rules

### 5.1 Sender path choice

A sender MUST choose exactly one path family per logical attachment send attempt:
- legacy path only: use existing `file_chunk` / `file_manifest` semantics
- attachment path only: use `attachment_descriptor` plus off-message-plane blob transfer

The sender MUST NOT emit both path families for the same logical attachment send attempt.

### 5.2 Receiver path distinction

Receivers distinguish the path family by payload type identity:
- `file_chunk` / `file_manifest` => legacy path
- `attachment_descriptor` => new attachment-plane path

A receiver MUST NOT infer the new attachment path from legacy payload fields, and MUST NOT infer legacy chunk semantics from an attachment descriptor.

### 5.3 Mixed/ambiguous cases are invalid

The following are invalid coexistence cases:
- a payload claiming `t = "attachment_descriptor"` while also carrying legacy-only fields such as inline chunk bytes or `chunk_hashes`
- a legacy `file_chunk` or `file_manifest` payload carrying attachment-plane-only fields such as `locator_ref`, `fetch_capability`, or `part_size_class`
- any sender-side attempt to represent one logical attachment send as both a legacy manifest/chunk sequence and an attachment descriptor

Such cases MUST reject fail-closed.

### 5.4 Coexistence matrix

| Mode | Message-plane shape | Blob-plane shape | Valid? | Owned by later item(s) |
|---|---|---|---|---|
| Legacy path only | `file_chunk` + `file_manifest` | none | yes | existing legacy runtime; untouched by `NA-0197A`, `NA-0197B`, and `NA-0197C` until later migration work is separately authorized |
| Attachment descriptor path only | `attachment_descriptor` | separate opaque attachment plane | yes | `NA-0197B`, repo-local attachment-service implementation, `NA-0197C` |
| Mixed same-attempt path | any legacy payload plus `attachment_descriptor` for one logical send attempt | ambiguous | no | reject via `REJECT_ATT_DESC_COEXISTENCE_MODE` |
| Attachment payload with legacy-inline blob fields | `attachment_descriptor` plus legacy-only chunk/manifest content | ambiguous | no | reject via `REJECT_ATT_DESC_COEXISTENCE_MODE` |
| Legacy payload with attachment-plane fields | `file_chunk` / `file_manifest` plus locator/capability/part-class fields | ambiguous | no | reject via `REJECT_ATT_DESC_COEXISTENCE_MODE` |

### 5.5 Legacy deprecation trigger summary

The current `<= 4 MiB` legacy path remains unchanged until a later item proves:
- the size ladder through `100 MiB`
- restart/resume behavior
- quota/expiry behavior
- honest-delivery semantics
- and metadata/log-safety

This summary is a coexistence rule only. It authorizes no runtime migration by itself.

## 6. Reject matrix and no-mutation rules

| Reason code | Trigger | Durable mutation allowed? | Temp/local staging | `peer_confirmed` | `accepted_by_relay` |
|---|---|---|---|---|---|
| `REJECT_ATT_DESC_UNKNOWN_VERSION` | unknown `v` for `t = "attachment_descriptor"` | no | discard parse state only | false | unchanged/truthful |
| `REJECT_ATT_DESC_MISSING_REQUIRED_FIELD` | missing required descriptor field | no | discard parse state only | false | unchanged/truthful |
| `REJECT_ATT_DESC_FIELD_DOMAIN` | invalid enum/domain/format value | no | discard parse state only | false | unchanged/truthful |
| `REJECT_ATT_DESC_ENC_CTX` | missing/unknown `enc_ctx_*`, malformed `enc_ctx_b64u`, unsupported encryption-context algorithm, or package decode/version failure | no durable completion state | discard parse state only | false | unchanged/truthful |
| `REJECT_ATT_DESC_INCONSISTENT_SHAPE` | size/count/commitment inconsistency | no durable completion state | discard any temp download staging | false | unchanged/truthful |
| `REJECT_ATT_DESC_COEXISTENCE_MODE` | legacy/attachment ambiguity or mixed-mode violation | no | discard parse state only | false | unchanged/truthful |
| `REJECT_ATT_DESC_LOCATOR_PLACEMENT` | malformed locator, malformed capability, or capability placed in disallowed canonical-URL form | no | discard any temp fetch state | false | unchanged/truthful |
| `REJECT_ATT_DESC_EXPIRED` | descriptor expired at processing/fetch time | no durable completion state | discard temp staging | false | unchanged/truthful |
| `REJECT_ATT_DESC_POLICY` | policy-violating size, retention class, or descriptor shape | no | discard parse/fetch staging | false | unchanged/truthful |
| `REJECT_ATT_DECRYPT_CTX_MISMATCH` | decoded attachment context conflicts with transcript-bound descriptor shape or local outstanding record state | no durable completion state | discard any temp fetch/decrypt staging | false | unchanged/truthful |
| `REJECT_ATT_CIPHERTEXT_PRECHECK` | ciphertext part ordering, ciphertext shape, or `integrity_root` check fails before plaintext promotion | no durable completion state | discard any temp fetch/decrypt staging | false | unchanged/truthful |
| `REJECT_ATT_DECRYPT_AUTH` | per-part decryption authentication failure, nonce mismatch, or AAD mismatch under `DOC-CAN-007` | no durable completion state | discard temp decrypt staging | false | unchanged/truthful |
| `REJECT_ATT_PLAINTEXT_SHAPE` | post-decrypt plaintext length mismatch or local persistence cannot be completed truthfully | no durable completion state | discard temp decrypt staging | false | unchanged/truthful |
| `REJECT_ATT_CONFIRM_LINKAGE` | confirmation missing outstanding record, not requested, replayed, early, or wrong `attachment_id`/`confirm_handle` pair | no confirmation-state mutation | no special staging requirement | false | unchanged/truthful |
| `REJECT_ATT_CONFIRM_TRANSCRIPT_MISMATCH` | confirmation conflicts with transcript-bound descriptor data for the outstanding record | no confirmation-state mutation | no special staging requirement | false | unchanged/truthful |
| `REJECT_ATT_CONFIRM_EARLY` | any attempt to emit completion confirmation before verified local persistence and successful decrypt/integrity checks | no confirmation-state mutation | discard any temp decrypt staging if present | false | unchanged/truthful |

Normative rule:
- temp/local staging may be discarded on reject,
- durable completion state MUST NOT advance,
- and sender-side relay acceptance history MUST NOT be rewritten by any attachment reject.

## 7. Canonical URL and secret-handling rules

### 7.1 No capability-like secrets in canonical URLs

The following MUST NOT appear in canonical URLs, query strings, or loggable operator-facing URLs:
- `fetch_capability`
- `enc_ctx_b64u`
- upload/download resume tokens
- route tokens
- any future token that directly authorizes blob access or session continuation

### 7.2 Peer-visible versus service-visible retrieval material

Field classification:
- peer-visible and transcript-bound retrieval material: `locator_kind`, `locator_ref`, `fetch_capability`
- peer-visible and transcript-bound decrypt-context material: `enc_ctx_alg`, `enc_ctx_b64u`
- service-visible only: session ids, resume tokens, service-internal object coordinates
- local-only: decoded attachment keys, decoded nonce prefixes, UI correlation ids, local file paths, local cleanup markers

Later service APIs MUST carry `fetch_capability` and resume tokens in headers or request bodies when transmission outside the descriptor itself is required.
`enc_ctx_alg` and `enc_ctx_b64u` MUST remain off the service plane entirely.

### 7.3 Documentation and example hygiene

Canonical docs and examples MUST use placeholders such as:
- `<ATTACHMENT_ID_HEX_64>`
- `<ATTACHMENT_REF>`
- `<ATTACHMENT_CAPABILITY>`
- `<ATTACHMENT_ENC_CTX_B64U>`
- `<ATTACHMENT_EXPIRES_AT_UNIX_S>`

## 8. Source-of-truth mapping for later items

This canonical document is now the source of truth for:
- descriptor field names and domains,
- transcript-bound compare rules,
- attachment encryption-context field placement,
- peer-confirm linkage,
- reject classes,
- and legacy coexistence semantics.

Later ownership split:
- `NA-0197B` consumes `locator_kind`, `locator_ref`, `fetch_capability`, `retention_class`, `expires_at_unix_s`, and the reject matrix to define the attachment-service contract.
- `NA-0197CA` and `DOC-CAN-007` freeze the attachment encryption context, part-cipher shape, and decrypt-order rules that stay in the control plane and client surfaces only.
- repo-local attachment-service implementation consumes the service-facing parts of `NA-0197B` and must honor this document’s control-plane invariants.
- `NA-0197C` consumes the local-only/state-mapping rules, delivery semantics, confirmation rules, and `DOC-CAN-007` decrypt-order contract to implement qsc streaming/journaling behavior.

## 9. Safe descriptor example

```json
{
  "v": 1,
  "t": "attachment_descriptor",
  "attachment_id": "<ATTACHMENT_ID_HEX_64>",
  "plaintext_len": 10485120,
  "ciphertext_len": 10485760,
  "part_size_class": "p256k",
  "part_count": 40,
  "integrity_alg": "sha512_merkle_v1",
  "integrity_root": "<INTEGRITY_ROOT_HEX_128>",
  "locator_kind": "service_ref_v1",
  "locator_ref": "<ATTACHMENT_REF>",
  "fetch_capability": "<ATTACHMENT_CAPABILITY>",
  "enc_ctx_alg": "chacha20poly1305_part_v1",
  "enc_ctx_b64u": "<ATTACHMENT_ENC_CTX_B64U>",
  "retention_class": "standard",
  "expires_at_unix_s": <ATTACHMENT_EXPIRES_AT_UNIX_S>,
  "confirm_requested": true,
  "confirm_handle": "<CONFIRM_HANDLE_HEX_24>",
  "filename_hint": "example.pdf",
  "media_type": "application/pdf"
}
```

## 10. References

- `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`
- `tests/NA-0197_attachment_validation_and_rollout_plan.md`
- `qsl/qsl-client/qsc/src/store/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`
- `qsl/qsl-client/qsc/tests/aws_file_medium_boundary_na0192a.rs`
- `README.md`
- `docs/public/INDEX.md`
