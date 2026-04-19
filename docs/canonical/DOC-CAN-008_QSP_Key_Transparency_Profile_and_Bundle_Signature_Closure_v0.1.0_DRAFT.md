Goals: G4

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-04-19

# DOC-CAN-008 — QSP Key Transparency Profile and Bundle Signature Closure v0.1.0 DRAFT

## 1. Purpose and scope

This document closes the KT prerequisite ambiguities identified by `DOC-AUD-003` and the focused KT audit so a later implementation lane can add fail-closed verifier enforcement without inventing formats, coverage rules, or responder policy in code.

This document is normative for:

- canonical KT bundle profile selection;
- canonical `BundleLeafData` and `BundleTBS` meanings;
- the exact byte-level meaning of `kt_log_id`, `kt_sth`, `kt_inclusion_proof`, and `kt_consistency_proof`;
- bundle-signature coverage and verification order;
- initiator- and responder-side KT obligations for authenticated sessions; and
- fail-closed rejection classes.

This document does not change current runtime behavior on `main`. Where current placeholder code differs from this document, this document is authoritative and the later KT implementation lane MUST align runtime behavior to this document.

## 2. Canonical profile selector

The canonical KT-enabled prekey-bundle profile identifier is:

- `bundle_format = "QSP-4.3.2-KT1"`

`QSP-4.3.2-KT1` means all of the following are true:

- `bundle_fields_b64` is base64url of canonical `BundleTBS` bytes from §4;
- `leaf_data_b64` is base64url of canonical `BundleLeafData` bytes from §3;
- `kt_sth`, `kt_inclusion_proof`, and `kt_consistency_proof` use the binary encodings from §5; and
- all KT verification rules in §6 through §8 apply.

Unknown `bundle_format` values and unknown KT blob versions are fatal for authenticated sessions.

## 3. Canonical byte conventions and `BundleLeafData`

### 3.1 Common conventions

- All integers are unsigned big-endian.
- Fixed-width fields use the exact byte counts stated here.
- Variable-width octet strings in `BundleLeafData` and `BundleTBS` use `varbytes_u32 = u32 length || bytes`.
- Optional fields use the current QSP boolean convention: `u16` with `0 = absent`, `1 = present`; any other value is invalid.

### 3.2 `kt_log_id`

`kt_log_id` is exactly 32 octets.

Its meaning is:

- an opaque, client-pinned KT log identifier;
- chosen by trusted configuration, not by the peer;
- stable across STH refreshes for the same logical log; and
- mapped locally to the pinned STH verification key, freshness state, and policy for that log.

The all-zero value is not valid for authenticated sessions. It is reserved only for the explicit disabled non-production shape in §8.

### 3.3 `BundleLeafData`

`BundleLeafData` is the canonical byte sequence logged into KT and used to derive the Merkle leaf hash. It exists separately from `BundleTBS` so KT proofs do not recursively cover themselves.

Field order for `BundleLeafData` is:

1. `user_id` as `varbytes_u32`
2. `device_id` as `u32`
3. `valid_from` as `u32`
4. `valid_to` as `u32`
5. `ik_sig_ec_pub` as 32 bytes
6. `ik_sig_pq_pub` as `varbytes_u32`
7. `spk_dh_pub` as 32 bytes
8. `spk_pq_pub` as `varbytes_u32`
9. `pq_rcv_id` as `u32`
10. `pq_rcv_pub` as `varbytes_u32`
11. `opk_dh_present` as `u16`
12. if `opk_dh_present = 1`: `opk_dh_id` as `u32`, then `opk_dh_pub` as 32 bytes
13. `opk_pq_present` as `u16`
14. if `opk_pq_present = 1`: `opk_pq_id` as `u32`, then `opk_pq_pub` as `varbytes_u32`
15. `kt_log_id` as 32 bytes

`BundleLeafData` excludes:

- `sig_ec`
- `sig_pq`
- `kt_sth`
- `kt_inclusion_proof`
- `kt_consistency_proof`

The canonical KT leaf hash is:

- `leaf_hash = SHA-256(0x00 || BundleLeafData)`

The canonical internal-node hash is:

- `node_hash = SHA-256(0x01 || left_child_hash || right_child_hash)`

No other leaf prefix, node prefix, or hash algorithm is valid for `QSP-4.3.2-KT1`.

## 4. `BundleTBS` and bundle-signature coverage

`BundleTBS` is the canonical byte sequence covered by `sig_ec` and `sig_pq`.

It is:

- `BundleLeafData`
- followed by `varbytes_u32(kt_sth)`
- followed by `varbytes_u32(kt_inclusion_proof)`
- followed by `varbytes_u32(kt_consistency_proof)`

`BundleTBS` intentionally includes the KT proof artifacts so bundle signatures authenticate the exact proof set presented to the peer.

`BundleTBS` intentionally does not use the current placeholder `PrekeyBundle::encode()` KT `u16` proof-length carriage. For `QSP-4.3.2-KT1`, the authoritative signature input is the `BundleTBS` layout above.

The canonical bundle-signature input is:

- `bundle_sig_input = SHA-512(BundleTBS)`

`sig_ec` and `sig_pq` MUST both verify over the same `bundle_sig_input`.

Verification order for authenticated sessions is:

1. structurally parse `BundleLeafData`, `kt_sth`, `kt_inclusion_proof`, and `kt_consistency_proof`;
2. reconstruct `BundleTBS`;
3. verify `sig_ec` and `sig_pq` over `bundle_sig_input`;
4. verify the KT proof set using the already-authenticated bundle fields and the pinned log policy; and
5. reject the bundle if any step fails.

There is no silent fallback from `bundle_sig_fail` or `kt_fail` to unauthenticated acceptance inside authenticated mode.

## 5. Canonical KT proof-carriage blobs

### 5.1 `kt_sth`

`kt_sth` is a fixed-format binary blob:

1. `version` as `u8`; valid value for this profile is `0x01`
2. `log_id` as 32 bytes; MUST equal `kt_log_id`
3. `tree_size` as `u64`
4. `timestamp_ms` as `u64`; Unix milliseconds
5. `root_hash` as 32 bytes; the SHA-256 Merkle root for the tree
6. `signature` as 64 bytes; Ed25519 signature

The canonical STH signature input is:

- `sth_sig_input = SHA-512("QSL-KT/STH/v1" || version || log_id || tree_size || timestamp_ms || root_hash)`

The verifier MUST select the Ed25519 verification key from its pinned `kt_log_id` configuration, not from the peer and not from the bundle.

### 5.2 `kt_inclusion_proof`

`kt_inclusion_proof` is a binary blob:

1. `version` as `u8`; valid value is `0x01`
2. `leaf_index` as `u64`
3. `tree_size` as `u64`
4. `sibling_count` as `u16`
5. `siblings` as `sibling_count` consecutive 32-byte hashes, ordered from leaf level toward the root

The verifier MUST reject if:

- `leaf_index >= tree_size`;
- `tree_size != kt_sth.tree_size`;
- any sibling is not exactly 32 bytes;
- trailing bytes remain after parsing; or
- the reconstructed root does not equal `kt_sth.root_hash`.

For `QSP-4.3.2-KT1`, the verifier reconstructs the root by starting from `leaf_hash`, then consuming `siblings` in order. At each step:

- if the current node is a right child, hash `0x01 || sibling || current`;
- otherwise hash `0x01 || current || sibling`.

The left/right position is derived from `leaf_index` and the current tree layer; it is not carried separately in the proof.

### 5.3 `kt_consistency_proof`

`kt_consistency_proof` is a binary blob:

1. `version` as `u8`; valid value is `0x01`
2. `from_tree_size` as `u64`
3. `to_tree_size` as `u64`
4. `node_count` as `u16`
5. `nodes` as `node_count` consecutive 32-byte hashes ordered from the smaller-tree frontier toward the larger-tree root

The proof semantics are append-only semantics for the same SHA-256 leaf/node hash tree defined in §3.3. The verifier MUST use `from_tree_size`, `to_tree_size`, `nodes`, the last accepted root for `from_tree_size`, and the current `kt_sth.root_hash` for `to_tree_size` to prove that the larger tree is an append-only extension of the smaller tree.

The verifier MUST reject if:

- `from_tree_size == 0`;
- `from_tree_size > to_tree_size`;
- `to_tree_size != kt_sth.tree_size`;
- any node is not exactly 32 bytes; or
- the proof does not reconstruct both the prior accepted root and the current root under the append-only tree semantics above.

## 6. Freshness, monotonicity, and mandatory consistency

For authenticated sessions, KT verification is stateful per pinned `kt_log_id`.

### 6.1 Freshness

- `kt_sth.timestamp_ms` MUST be checked against the verifier wall clock.
- The maximum accepted staleness is `ktl.proof_cache_ttl_seconds * 1000`.
- If `now_ms - kt_sth.timestamp_ms` exceeds that bound, reject with `kt_fail`.

### 6.2 Monotonicity

The verifier stores the last accepted `(tree_size, root_hash, timestamp_ms)` for each pinned `kt_log_id`.

- If there is no prior accepted STH state for the log, `kt_consistency_proof` MAY be empty.
- If the new `tree_size` is smaller than the stored `tree_size`, reject.
- If the new `tree_size` equals the stored `tree_size`, then:
  - `kt_consistency_proof` MUST be empty;
  - `root_hash` MUST equal the stored `root_hash`; and
  - `timestamp_ms` MUST be greater than or equal to the stored `timestamp_ms`.
- If the new `tree_size` is greater than the stored `tree_size`, `kt_consistency_proof` is mandatory and MUST verify append-only consistency from the stored root to the new root.

If any of those conditions fail, the verifier MUST reject and MUST NOT update stored KT state.

## 7. Responder obligations for initiator identity KT

The responder MUST NOT treat bare HS1 identity keys as KT-authenticated by transcript signatures alone.

For authenticated sessions, the responder MUST obtain one of the following before accepting HS1:

- A full `QSP-4.3.2-KT1` prekey bundle for the initiator; or
- an equivalent service-authenticated bundle snapshot that contains the same `BundleLeafData`, `BundleTBS`, and KT artifacts.

The responder-side checks are:

1. verify the initiator bundle signatures over canonical `BundleTBS`;
2. verify the initiator KT proof set under the pinned log policy;
3. verify that the HS1 identity keys match the verified bundle identity keys; and
4. verify that the HS1 PQ receive key material required for later `ct3` processing matches the verified bundle snapshot.

If the responder cannot obtain and verify that bundle-equivalent evidence, the authenticated responder path MUST reject. There is no caller-discretion fallback inside authenticated mode.

## 8. Disabled/non-production shape

The only permitted disabled KT shape is:

- `kt_log_id = 0x00` repeated 32 times
- `kt_sth = empty`
- `kt_inclusion_proof = empty`
- `kt_consistency_proof = empty`

This shape:

- is never valid for production, staging, or any conformance-claiming authenticated session;
- MAY be accepted only in an explicitly enabled non-production mode such as test harnesses or demo-only unauthenticated flows; and
- MUST be rejected by default.

When that disabled shape is accepted in a non-production mode, the session MUST NOT be claimed as KT-authenticated.

## 9. Fail-closed rejection classes

The later KT implementation lane MUST distinguish internal failure classes but surface only sanitized external reason codes.

`bundle_sig_fail` covers:

- inability to reconstruct canonical `BundleTBS`;
- `sig_ec` verification failure; and
- `sig_pq` verification failure.

`kt_fail` covers:

- unknown `bundle_format` or unknown KT blob version;
- disabled/non-production KT shape used without explicit non-production enablement;
- `kt_log_id` not pinned locally;
- `kt_sth.log_id != kt_log_id`;
- STH parse failure or Ed25519 signature failure;
- stale, regressing, or inconsistent STH state;
- missing mandatory consistency proof;
- inclusion-proof parse failure or root mismatch;
- consistency-proof parse failure or append-only failure; and
- responder failure to obtain bundle-equivalent initiator KT evidence in authenticated mode.

All of the above are fatal for authenticated sessions. Implementations MUST reject without mutating durable KT state when any of those failures occur.
