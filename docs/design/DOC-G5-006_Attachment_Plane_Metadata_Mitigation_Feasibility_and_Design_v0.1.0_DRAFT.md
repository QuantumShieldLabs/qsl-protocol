Goals: G5 (primary), supports G1, G2, G3, G4

Status: DRAFT
Owner: QSL Governance
Doc-ID: DOC-G5-006
Version: v0.1.0
Last-Updated: 2026-07-07

# DOC-G5-006 — Attachment-Plane Metadata Mitigation Feasibility and Design

> **CORRECTION (NA-0614, D-1224):** This doc's original conclusion that M1
> (object-size padding) is **client-only with "no contract change"** was wrong. The
> receiver enforces an exact `sum(decrypted) == plaintext_len` check and the descriptor
> is `#[serde(deny_unknown_fields)]`, so a true-length field cannot be added without a
> coordinated format change — padding is **not** achievable as a sender-only change.
> Because the project is pre-release (no installed base), that coordinated change is
> cheap and needs no negotiation/versioning machinery, so padding is made a **mandatory
> baseline format** with an additive authenticated `content_len` (the exact-length check
> is preserved, not weakened). The corrected, locked design is in **DOC-G5-007**. In the
> tables below, read M1's "contract change = None" as **"format change: yes; negotiation:
> no (pre-release mandatory baseline)"**.

## 1. Purpose and scope

This is a read-only feasibility+design study (DOC-G5-005 §9 rank 4 / ledger ENG-0007)
of mitigations for the attachment-plane residual metadata that NA-0608 identified as
EXPOSED: ciphertext-object size, object/part count, and upload/fetch timing. It
extends the message-plane padding/bucketing philosophy (already implemented via
`meta_bucket_for_len` / `bucket_max` in `qsc/src/transport`) to the attachment plane.
It proposes designs and a cost/benefit matrix; it changes no source, wire format, or
attachment contract. Every mitigation is deferred to its own implementation lane.

Non-goals (DOC-G5-001 stands): no claim of metadata elimination, anonymity,
unlinkability, or traffic-analysis resistance. This study reduces and buckets residual
metadata; it does not eliminate it.

## 2. Threat model and observation separation (the key distinction)

Two observers must be separated:

- The **peer** (intended recipient) receives the attachment descriptor **inside the
  end-to-end-encrypted QSP envelope**. The descriptor carries the true `plaintext_len`.
  The peer is trusted with the plaintext; descriptor contents are not a hostile-analyst
  leak.
- The **attachment service / network observer** never sees the descriptor. It sees
  only the stored **opaque ciphertext object** (its byte length and part count) and the
  **upload/fetch access pattern and timing**. This is the hostile-analyst surface for
  the attachment plane.

Therefore mitigations that pad the ciphertext *object* reduce what the service/network
observer learns, even though the descriptor (peer-only) still carries the true size.

## 3. Current attachment-plane facts (grounded in code)

- Fixed part-size ladder (`attachments/mod.rs::attachment_part_size_bytes`):
  `p64k` = 65,536 B, `p256k` = 262,144 B, `p1024k` = 1,048,576 B.
- Class selection (`choose_attachment_part_size_class`) is a deterministic function of
  `plaintext_len`: `<= 16 MiB -> p64k`, `<= 64 MiB -> p256k`, else `p1024k`.
- `part_count = ceil(plaintext_len / (part_size - tag))`;
  `ciphertext_len = plaintext_len + part_count * ATTACHMENT_CIPHER_TAG_LEN`.
  There is **no object-level size padding today**: `ciphertext_len` reveals
  `plaintext_len` to within the per-part tag overhead.
- The size fields (`plaintext_len`, `ciphertext_len`, `part_size_class`, `part_count`)
  are cryptographically bound into the per-part AAD (`QATT-PART-V1|...`) and the confirm
  MAC (`QATT-CONFIRM-V1|...`). Any mitigation must keep these internally consistent and
  bound.

## 4. Residual-metadata channel inventory

- **C1 — Object ciphertext size.** The stored object length (`ciphertext_len`) is
  visible to the service and reveals the plaintext size almost exactly. Highest-signal
  channel.
- **C2 — Object/part count.** The number of uploaded parts reveals size to within one
  part granularity (64 KiB / 256 KiB / 1 MiB by class). Correlated with C1.
- **C3 — Part-size-class selection.** Because the class is a deterministic function of
  `plaintext_len` (16 MiB / 64 MiB thresholds), the object's part sizing reveals a
  coarse 3-way size band to any observer who can infer the class from stored part
  sizes.
- **C4 — Upload/fetch timing and access pattern.** When an object is uploaded/fetched,
  by which capability, and the request cadence are visible to the service/network.
  Largely a qsl-attachments/deployment property, not a qsc client property.

## 5. Candidate mitigations (extending the message-plane bucket model)

- **M1 — Object-size padding to a fixed ladder (client-side).** Before chunking and
  per-part AEAD, pad the plaintext to the next size on a defined ladder; keep the
  descriptor's `plaintext_len` set to the **true** length (so the peer's decrypt
  truncates to it) while `ciphertext_len` / `part_count` / integrity root reflect the
  **padded** object. The service/network then observe only the bucketed size (C1/C2
  collapse to a ladder). Feasible client-side because the descriptor is peer-only and
  the service stores opaque objects and only checks `stored_size == ciphertext_len`
  (which becomes the padded length). Requires no attachment-contract change.
- **M2 — Part-count bucketing.** A consequence of M1: once the object is padded to a
  ladder, `part_count` takes only the ladder's discrete values. No separate mechanism
  needed beyond M1's ladder choice.
- **M3 — Decouple part-size-class from plaintext.** Fold into M1: choose the class from
  the **padded** (bucketed) size or a fixed policy so the class no longer encodes the
  raw plaintext band (mitigates C3).
- **M4 — Upload/fetch timing normalization / cover traffic.** Jitter or rate-shape
  uploads/fetches, or add cover objects. Mostly a qsl-attachments/deployment concern
  (the service observes timing); the qsc client can add send/fetch jitter but cannot
  hide access existence. Higher cost, cross-repo.

## 6. Cost/benefit matrix

| Mitigation | Benefit | Cost | Placement | Contract change |
|---|---|---|---|---|
| M1 object-size padding | High: collapses C1 (exact size) to a bucket | Bandwidth/storage overhead (ladder-dependent; ~up to 2x for power-of-two, less for a finer ladder) | qsc client | None (service stores opaque padded object) |
| M2 part-count bucketing | Medium: collapses C2 | Included in M1 | qsc client | None |
| M3 class decoupling | Medium: removes C3 band leak | Small (larger parts for small objects) | qsc client | None |
| M4 timing/cover | Medium-High: addresses C4 | High: latency, cover-traffic bandwidth; cross-repo | qsl-attachments/deployment (+ optional qsc jitter) | Likely yes (service/deployment) |

## 7. Ranked recommendations

- **Recommended now (top implementation lane): M1 + M2 + M3 combined** — a
  client-side object-size/part-count bucketing pass with class decoupling, using a
  tunable size ladder that mirrors the message-plane `bucket_max` philosophy. It is
  the highest-value, lowest-friction mitigation: client-only, no attachment-contract
  change, collapses the exact-size and coarse-band channels to a documented ladder. It
  must carry deterministic vectors proving bucketed sizes, keep the AAD/confirm binding
  consistent, preserve fail-closed decrypt/truncation, and explicitly account for the
  bandwidth/storage overhead. This is the proposed successor lane (NA-0614) and is
  recorded as ENG-0010.
- **Deferred (separate, cross-repo): M4 timing/cover** — belongs primarily to
  qsl-attachments/deployment; the client-side jitter portion is a smaller optional
  follow-up. Recorded as ENG-0011.
- **Out of immediate scope:** eliminating access-existence/pattern metadata (the
  service inherently learns that some object was stored/fetched); this is bounded by
  the DOC-G5-001 non-goals and is not a qsc-only problem.

## 8. Honest residual (even with M1–M3)

Even with object-size/part-count bucketing and class decoupling, the following remain
and are **not** claimed eliminated: the bucket still discloses a size *range*; the
number of distinct objects and their access timing/pattern remain visible to the
service; correlation across objects/sessions is not addressed; and cover-traffic is
out of scope. This study reduces and buckets residual metadata; it does not make the
attachment plane metadata-free, anonymous, unlinkable, or traffic-analysis-resistant.

## 9. Interfaces to existing docs

Extends DOC-G5-005 §2/§6/§9 (rank 4) and the DOC-G5-002/004 leakage inventory and
logging contract to the attachment plane; consistent with DOC-ATT-001/002/003
(attachment architecture/deployment/promotion). The message-plane precedent is
`qsc/src/transport::meta_bucket_for_len` / `bucket_max`.
