Goals: G5 (primary), supports G1, G2, G3, G4

Status: DRAFT
Owner: QSL Governance
Doc-ID: DOC-G5-007
Version: v0.1.0
Last-Updated: 2026-07-07

# DOC-G5-007 — Attachment Object Padding: Format and Ladder (design-lock)

This is the design-lock for NA-0614 (D-1224). It fixes the on-object format and the
padding policy before any code. It supersedes DOC-G5-006's incorrect "client-only, no
format change" M1 conclusion (corrected there): the receiver enforces an exact
decrypted-length check and the descriptor is `deny_unknown_fields`, so bucketing is a
coordinated format change — cheap and mandatory pre-release.

## 1. Threat and goal

Reduce the attachment-plane object-size metadata visible to the attachment
service/network observer (NA-0608 C1/C2). The descriptor is peer-only inside the
encrypted envelope; the service sees only the opaque ciphertext object's size and part
count. Padding the object collapses exact size to a bucket. This is not metadata
elimination (access existence/timing remain; see ENG-0011); no anonymity/unlinkability
claim.

## 2. Field model (additive; exact-check preserved)

- `plaintext_len` (existing, semantics tightened): the **padded/encrypted** plaintext
  length. It drives `part_size_class`, `part_count`, `ciphertext_len`, the merkle root,
  and the AAD. It is always a ladder-bucket value.
- `content_len` (NEW, required): the **true** file length delivered to the user.
  Invariant: `0 < content_len <= plaintext_len` and `content_len <= ATTACHMENT_DEFAULT_MAX_FILE_SIZE`.
- The receiver keeps its existing exact check `sum(decrypted part plaintext) ==
  plaintext_len` (unweakened — now verifying the padded length) and additionally
  truncates the written output file to `content_len`.

## 3. Authentication (integrity of the true length)

`content_len` is security-relevant (it controls truncation), so it is authenticated:
- Bound into the per-part AAD: `QATT-PART-V1|{attachment_id}|{enc_ctx_alg}|{content_len}|{plaintext_len}|{ciphertext_len}|{part_size_class}|{part_count}|{part_index}`.
- Bound into the confirm MAC material: `QATT-CONFIRM-V1|{attachment_id}|{content_len}|{plaintext_len}|{ciphertext_len}|{part_size_class}|{part_count}|{integrity_alg}|{integrity_root}|{retention_class}|{expires_at_unix_s}`.
Tampering `content_len` therefore fails AEAD decryption / confirm verification —
fail-closed. (Domain tags stay `V1`: this is version 1 of the shipped format, which
includes `content_len` from first release; see §7.)

## 4. Size ladder (sender policy; receiver-agnostic)

`ATTACHMENT_PAD_LADDER` (bytes), round `content_len` UP to the smallest bucket
`>= content_len`:
`4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304,
8388608, 16777216, 33554432, 67108864,` then the **cap bucket** `ATTACHMENT_DEFAULT_MAX_FILE_SIZE`
(= 100 MiB = 104857600).
- Because `content_len <= 100 MiB` (enforced pre-pad), the padded length always fits a
  bucket, and the top bucket == the max file size (padding never exceeds the max).
- `part_size_class` is chosen from the **padded** length via the existing
  `choose_attachment_part_size_class`; `part_count`/`ciphertext_len` computed over the
  padded length. Worst-case `part_count` at the cap bucket is ~100 (p1024k), far under
  `ATTACHMENT_DEFAULT_MAX_PARTS` (4096).
- The ladder is **sender policy only**. The receiver honors any valid `content_len`
  (§2 invariant); the ladder may be retuned in a future build with no format/receiver
  change (forward plumbing).

## 5. Sender algorithm

1. `content_len = file size`; reject if `content_len == 0` or `content_len >
   ATTACHMENT_DEFAULT_MAX_FILE_SIZE` (the "too-big" gate now uses `content_len`, not the
   padded length).
2. `plaintext_len = pad_to_ladder(content_len)`.
3. `part_size_class = choose_attachment_part_size_class(plaintext_len)`;
   `part_count`/`ciphertext_len` over `plaintext_len`.
4. Stream the source file; after EOF, emit deterministic zero bytes to reach
   `plaintext_len`; AEAD-encrypt each part with the §3 AAD; build merkle leaves over
   the (padded) parts.
5. Descriptor/record carry both `content_len` and `plaintext_len`.

## 6. Receiver algorithm (fail-closed)

1. Validate descriptor shape incl. `0 < content_len <= plaintext_len` and
   `content_len <= max_file` (reject `REJECT_ATT_*` otherwise).
2. Decrypt each part with the §3 AAD (tamper of any bound field, incl. `content_len`,
   fails AEAD → `REJECT_ATT_DECRYPT_AUTH`).
3. Keep the exact check: total decrypted plaintext == `plaintext_len` else
   `REJECT_ATT_PLAINTEXT_SHAPE`.
4. Write only the first `content_len` bytes to the output file (truncate the pad).

## 7. Versioned-evolution convention (forward plumbing, not loosening)

- The descriptor keeps `#[serde(deny_unknown_fields)]` and `v == ATTACHMENT_DESCRIPTOR_VERSION`
  (1). `content_len` is a required field of the v1 format from first release.
- Future format changes bump `v` (and the `QATT-*` domain tags) and the receiver
  dispatches on `v`; unknown fields are never silently accepted. Extensibility is an
  explicit versioned upgrade path plus a policy-agnostic receiver — not permissive
  parsing. (Project design tenet.)

## 8. Fail-closed test matrix (PHASE 5)

Deterministic vectors: bucket-boundary `content_len` (no pad), just-over-boundary
(pads to next), tiny file (pads to 4 KiB), cap bucket, round-trip byte-exactness.
Adversarial (must reject): tampered `content_len` (AAD fail), `content_len > plaintext_len`,
`content_len == 0`, inconsistent `part_count`/`ciphertext_len`, truncated object,
padded total != `plaintext_len`.

## 9. Out of scope

Attachment upload/fetch timing and access-pattern metadata (ENG-0011, cross-repo);
part-count is bucketed only as a consequence of object bucketing. The `fuzz/corpus`
descriptor seed may become a stale (rejected) sample — harmless for a fuzz seed and
outside this lane's `tests/**` scope.
