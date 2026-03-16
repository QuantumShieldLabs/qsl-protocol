# DOC-CAN-007 — QATT Attachment Encryption Context + Part-Cipher Contract
Goals: G4, G5
**Version:** v0.1.0 (DRAFT)
**Status:** Draft (normative intent; implementable)
**Last Updated:** 2026-03-16
**Authority:** Canonical attachment encryption/decryption context and part-cipher contract for the signal-class attachment path frozen by `NA-0197CA`
**Goals:** G4, G5

---

## 0. Scope, goals, and non-goals

This document freezes the implementation-grade attachment encryption/decryption context for the signal-class attachment path.

It defines:
- how the sender generates the attachment encryption context,
- how the receiver obtains the attachment decryption context,
- which encryption-context fields are peer-visible, local-only, or service-only,
- the exact part-cipher and ciphertext-shape rules,
- the sender-side and receiver-side order of operations,
- and deterministic reject/no-mutation rules for missing, malformed, mismatched, or invalid encryption-context and decrypt-order conditions.

This document does not authorize:
- qsc runtime implementation,
- qsl-attachments runtime changes,
- qsl-server changes,
- workflow changes,
- or any plaintext attachment handling on service surfaces.

## 1. Relationship to existing canonical documents

`DOC-ATT-001` remains the architecture/program source of truth for:
- the control-plane / data-plane split,
- qsl-server staying transport-only,
- the requirement that the attachment plane remains opaque and plaintext-free,
- and the later qsc streaming/persistence rollout.

`DOC-CAN-005` remains authoritative for:
- descriptor payload identity and version,
- peer-visible descriptor field meanings,
- transcript-bound comparison rules,
- peer-confirm linkage,
- and legacy coexistence semantics.

`DOC-CAN-006` remains authoritative for:
- service/session/object semantics,
- secret carriage for service operations,
- session/object state transitions,
- and service-side reject/no-mutation rules.

This document is authoritative for:
- the v1 attachment encryption-context strategy,
- the exact v1 `enc_ctx_*` descriptor field semantics,
- plaintext-to-ciphertext part-shape derivation,
- the part-cipher algorithm and nonce/AAD rules,
- and the sender/receiver decrypt-order contract that later qsc work must implement.

## 2. Chosen encryption-context strategy

### 2.1 Canonical v1 strategy

The canonical v1 strategy is:
- sender-generated per-attachment encryption context,
- carried to the peer only inside the authenticated message-plane descriptor payload,
- never stored or served by the attachment plane,
- and never placed into canonical URLs.

Normative rules:
- The sender MUST generate fresh encryption-context material for each logical attachment send attempt.
- The receiver MUST obtain the decryption context only from the authenticated descriptor fields defined here and `DOC-CAN-005`.
- The service plane MUST remain blind to the attachment encryption context.

### 2.2 Canonical v1 algorithm identifier

The only canonical v1 attachment encryption-context algorithm is:
- `enc_ctx_alg = "chacha20poly1305_part_v1"`

Unknown `enc_ctx_alg` values MUST reject fail-closed.

### 2.3 No extra service-visible key wrap in v1

The v1 descriptor carries exactly one secret-bearing context package field:
- `enc_ctx_b64u`

Normative rules:
- `enc_ctx_b64u` is an opaque descriptor-carried context package.
- v1 does not define a second independent service-visible key-wrap layer.
- The confidentiality/authentication envelope for `enc_ctx_b64u` is the existing authenticated message-plane payload itself.

## 3. Canonical v1 context package

### 3.1 Descriptor-carried fields

The v1 descriptor additions are:
- `enc_ctx_alg`
- `enc_ctx_b64u`

Field-class rules:
- `enc_ctx_alg` is peer-visible and transcript-bound.
- `enc_ctx_b64u` is peer-visible, transcript-bound, and secret-bearing.
- Neither field is service-visible.

### 3.2 `enc_ctx_b64u` decoded package shape

For `enc_ctx_alg = "chacha20poly1305_part_v1"`, `enc_ctx_b64u` MUST decode from unpadded base64url into exactly 41 bytes:
- byte `0`: package version `0x01`
- bytes `1..32`: 32-byte attachment content-encryption key `att_cek`
- bytes `33..40`: 8-byte nonce prefix `nonce_prefix`

Normative rules:
- The encoded `enc_ctx_b64u` token MUST be exactly 55 base64url characters with no padding.
- Any decode failure, wrong decoded length, or wrong package version MUST reject fail-closed.
- `att_cek` and `nonce_prefix` MUST be generated with a cryptographically secure random source.

### 3.3 Excluded surfaces

The following MUST NOT contain `enc_ctx_alg`, `enc_ctx_b64u`, `att_cek`, `nonce_prefix`, or any future equivalent attachment decrypt-context material:
- canonical URLs,
- query strings,
- qsl-attachments service requests/responses,
- qsl-attachments logs,
- qsl-server messages outside the authenticated control-plane payload,
- and any operator-facing URL-like diagnostic output.

## 4. Part-cipher and ciphertext-shape contract

### 4.1 Plaintext splitting rule

Let:
- `part_size_bytes = registry(part_size_class)` from `DOC-CAN-005`
- `part_tag_len = 16`
- `part_plaintext_capacity = part_size_bytes - part_tag_len`

Normative rules:
- `part_plaintext_capacity` MUST be greater than zero.
- Plaintext is split into ascending zero-based parts with maximum plaintext size `part_plaintext_capacity`.
- Every non-final plaintext part MUST be exactly `part_plaintext_capacity` bytes.
- The final plaintext part MUST be in `[1, part_plaintext_capacity]` bytes.

### 4.2 Shape equations

For `enc_ctx_alg = "chacha20poly1305_part_v1"`:
- `part_count = ceil(plaintext_len / part_plaintext_capacity)`
- `ciphertext_len = plaintext_len + (part_tag_len * part_count)`

Equivalent ciphertext-part rules:
- every non-final ciphertext part MUST be exactly `part_size_bytes`
- the final ciphertext part MUST be `(final_plaintext_part_len + part_tag_len)` bytes
- `part_count` MUST also satisfy `ceil(ciphertext_len / part_size_bytes)`

### 4.3 Per-part nonce derivation

For part index `i`:
- `nonce_i = nonce_prefix || u32be(i)`

Normative rules:
- `nonce_i` is the canonical 12-byte nonce for part `i`.
- `i` MUST be the exact zero-based logical part index carried by the ciphertext object order.
- Reuse of the same `(att_cek, nonce_prefix)` pair across two logical attachment send attempts is forbidden.

### 4.4 Per-part additional authenticated data

For part index `i`, the canonical AAD material is the UTF-8 encoding of:
- `"QATT-PART-V1|" || attachment_id || "|" || enc_ctx_alg || "|" || dec(plaintext_len) || "|" || dec(ciphertext_len) || "|" || part_size_class || "|" || dec(part_count) || "|" || dec(i)`

Normative rules:
- `dec(...)` is the minimal base-10 ASCII encoding with no leading zeroes except for the value `0`.
- The AAD is transcript-bound in the sense that all inputs are descriptor fields that must already match the authenticated control-plane payload.
- `integrity_root` is intentionally not an AAD input because it is computed over the ciphertext parts produced by this contract.

### 4.5 Per-part encryption rule

For each plaintext part `pt_i`:
- instantiate `ChaCha20Poly1305(att_cek)`
- encrypt `pt_i` with nonce `nonce_i` and AAD from §4.4
- emit ciphertext part `ct_i`

Normative rules:
- `len(ct_i) = len(pt_i) + 16`
- the ciphertext object is the bytewise concatenation `ct_0 || ct_1 || ... || ct_(part_count-1)`
- `integrity_root` from `DOC-CAN-005` and `DOC-CAN-006` is computed over these ciphertext parts exactly as stored/fetched from the service plane

## 5. Field split and source-of-truth boundaries

### 5.1 Peer-visible and transcript-bound

The following attachment encryption-context fields are peer-visible and transcript-bound:
- `enc_ctx_alg`
- `enc_ctx_b64u`

### 5.2 Local-only

The following attachment encryption-context material is local-only and MUST NOT be transmitted:
- decoded `att_cek`
- decoded `nonce_prefix`
- local key-object handles
- sender-side plaintext source path
- sender-side staged ciphertext path
- receiver-side temp plaintext path
- local progress/resume markers
- local cleanup eligibility markers

### 5.3 Service-only

The following remain service-only/session state and MUST stay off the message plane:
- `session_id`
- `resume_token`
- service-side object coordinates
- service-side quota/abuse counters

Normative rule:
- qsl-attachments MUST NOT require, store, log, or derive `enc_ctx_alg`, `enc_ctx_b64u`, `att_cek`, or `nonce_prefix`.
- `locator_ref` and `fetch_capability` remain peer-visible retrieval fields per `DOC-CAN-005`, but neither field conveys or substitutes for the attachment decrypt context.

## 6. Sender-side order of operations

The sender MUST follow this canonical order:
1. choose `part_size_class` and generate fresh `att_cek` plus `nonce_prefix`
2. construct `enc_ctx_alg` and `enc_ctx_b64u`
3. compute `part_count` and `ciphertext_len` from `plaintext_len` using §4.2
4. stream plaintext through the part cipher from §4.5 into local staged ciphertext while computing the ciphertext-part Merkle leaves/root required by `integrity_root`
5. persist enough local state to resume upload truthfully, including at minimum the staged ciphertext location, descriptor shape fields, `enc_ctx_*`, and staged-part progress metadata
6. create the attachment-plane session using the already known ciphertext-shape and `integrity_root`
7. upload ciphertext parts from local staging to the attachment plane and commit the object
8. emit the authenticated message-plane descriptor only after attachment-plane commit succeeds and returns `locator_ref`, `fetch_capability`, and `expires_at_unix_s`

Fail-closed rule:
- the sender MUST NOT emit the descriptor before local ciphertext staging and `integrity_root` are known and the attachment-plane commit has succeeded.

## 7. Receiver-side order of operations

The receiver MUST follow this canonical order:
1. authenticate and parse the descriptor per `DOC-CAN-005`
2. validate `enc_ctx_alg`, decode `enc_ctx_b64u`, and reject on any malformed or unsupported context package
3. validate ciphertext-shape and fetch-eligibility fields before any attachment-plane retrieval begins
4. fetch ciphertext object bytes from qsl-attachments using `locator_ref` and `fetch_capability`
5. validate ciphertext-part ordering, part lengths, and full-object `integrity_root` over the fetched ciphertext bytes
6. decrypt each ciphertext part using the decoded context package and the exact nonce/AAD rules from §4 into local temp plaintext staging only
7. validate final plaintext length exactly equals `plaintext_len`
8. atomically promote local plaintext staging to its final local persisted form only after all prior checks succeed
9. emit completion confirmation only after final local persistence succeeds and all `DOC-CAN-005` confirmation-linkage rules are satisfied

Normative rules:
- plaintext release to the user or final durable attachment record MUST NOT precede the successful completion of steps 1 through 8
- `peer_confirmed` MUST NOT be emitted before step 9

## 8. Reject and no-mutation rules

| Reason code | Trigger | Durable mutation allowed? | Temp/local staging | `peer_confirmed` | `accepted_by_relay` |
|---|---|---|---|---|---|
| `REJECT_ATT_DESC_ENC_CTX` | missing `enc_ctx_*`, unknown `enc_ctx_alg`, malformed `enc_ctx_b64u`, wrong package version, or decode failure | no durable completion state | discard parse state only | false | unchanged/truthful |
| `REJECT_ATT_DECRYPT_CTX_MISMATCH` | decoded context cannot satisfy the transcript-bound shape rules from §4 or local record conflicts with the descriptor-carried context | no durable completion state | discard temp download/decrypt staging | false | unchanged/truthful |
| `REJECT_ATT_CIPHERTEXT_PRECHECK` | ciphertext part ordering, per-part length, `ciphertext_len`, `part_count`, or `integrity_root` check fails before plaintext promotion | no durable completion state | discard temp fetch/decrypt staging | false | unchanged/truthful |
| `REJECT_ATT_DECRYPT_AUTH` | per-part ChaCha20-Poly1305 authentication failure or nonce/AAD mismatch during decryption | no durable completion state | discard temp decrypt staging | false | unchanged/truthful |
| `REJECT_ATT_PLAINTEXT_SHAPE` | post-decrypt plaintext length mismatch or local persistence cannot be completed truthfully | no durable completion state | discard temp decrypt staging | false | unchanged/truthful |
| `REJECT_ATT_CONFIRM_EARLY` | any attempt to emit completion confirmation before verified local persistence and successful decrypt/integrity checks | no confirmation-state mutation | no special staging requirement beyond existing temp cleanup | false | unchanged/truthful |

Normative rule:
- temp/local staging may be discarded on reject,
- durable attachment completion state MUST NOT advance,
- and sender-side relay acceptance history MUST NOT be rewritten by any encryption-context or decrypt-order reject.

## 9. Safe placeholders and examples

Canonical docs and examples MUST use placeholders such as:
- `<ATTACHMENT_ENC_CTX_B64U>`
- `<ATTACHMENT_ID_HEX_64>`
- `<INTEGRITY_ROOT_HEX_128>`

Safe descriptor fragment:

```json
{
  "enc_ctx_alg": "chacha20poly1305_part_v1",
  "enc_ctx_b64u": "<ATTACHMENT_ENC_CTX_B64U>"
}
```

## 10. References

- `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`
- `tests/NA-0197_attachment_validation_and_rollout_plan.md`
