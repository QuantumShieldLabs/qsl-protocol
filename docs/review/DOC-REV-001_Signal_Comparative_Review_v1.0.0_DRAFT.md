# DOC-REV-001 — Signal Comparative Review (Clean-Room) v1.0.0 DRAFT

Goals: G4, G5

Status: DRAFT
Date: 2026-01-04
Owner: QSL governance
Scope: Comparative review of Signal protocol/transport hardening patterns for QSL roadmap (clean-room only).

## 1. Document control

- Purpose: Provide a bounded, decision-grade comparative review that maps Signal hardening patterns to QSL gaps and candidate actions.
- Audience: QSL governance and engineering leads.
- Output: Delta Matrix (bounded) + Top-5 actionable upgrades + explicit non-goals.

## 2. Clean-room / licensing hygiene (non-negotiable)

- Do not copy Signal code or implementation details (AGPL risk).
- Paraphrase only; cite sources by repo/spec + file path + line ranges.
- Prefer prose over snippets; any excerpt must be short and for reference only.
- No derived artifacts may include code lifted from Signal repositories.

## 3. Sources to review (initial set)

Signal specifications (public):
- X3DH Key Agreement Protocol: https://signal.org/docs/specifications/x3dh/ (Revision 1, 2016-11-04)
- PQXDH Key Agreement Protocol: https://signal.org/docs/specifications/pqxdh/ (Revision 3, 2023-05-24; updated 2024-01-23)
- Double Ratchet Algorithm: https://signal.org/docs/specifications/doubleratchet/ (Revision 4, 2025-11-04)

Optional future sources (not yet reviewed in this PR):
- ML-KEM Braid: https://signal.org/docs/specifications/mlkembraid/
- SPQR/Signal PQ ratchet blog posts and engineering notes
- Signal repos: libsignal, Signal-Server (clean-room paraphrase with SHA + line ranges)

## 4. Method

### 4.1 Citation format

- Spec citation format: "Signal <Spec Name> §<section>" + URL
  Example: "Signal PQXDH §3.3 (https://signal.org/docs/specifications/pqxdh/)"
- Repo citation format (not used in this PR):
  "Signal <repo>@<sha> <path>:Lx-Ly" with a clean-room paraphrase.

### 4.2 Mapping to QSL goals and queue

- Each observation is mapped to QSL Goals (G1–G5) and a concrete candidate NA item.
- Each candidate action must include a CI-gating idea to keep G4 enforceable.

## 5. Delta Matrix — bounded "decision-grade" core

Target: keep the main matrix to <= 25 rows. Additional items go to the Appendix.

| Area | Signal approach (paraphrase + cite) | QSL posture (cite) | Gap/risk | Proposed action | Queue mapping (NA + Goals) | CI gating idea |
|---|---|---|---|---|---|---|
| Asynchronous prekey publishing | Server stores identity + signed prekey + optional one-time prekeys; initiator fetches a prekey bundle (X3DH §3.2–3.3; PQXDH §3.2–3.3). | Demo relay has register/send/poll; no explicit one-time prekey semantics (NA-0015). | Demo relay does not prevent prekey reuse or enforce at-most-once OPK semantics. | Define OPK lifecycle and server-side consumption rules for demo relay. | NA-0018 (G3/G5) | CI: register OPK, consume once, reject reuse (fail-closed). |
| Identity binding in initial AD | Initial message includes AD that binds both parties’ identity keys (X3DH §3.3, §4.8; PQXDH §3.3, §4.10). | Suite-2 establishment binds session_id/protocol_version/suite_id; identity binding is a system-layer prerequisite (DOC-CAN-003 §6). | Identity binding is not explicit in demo relay, creating misbinding risk in UI. | Add explicit identity binding policy in demo establish path and document in DOC-CAN-003 appendix. | NA-0019 (G3/G5) | CI: establishment vector fails if identity binding missing. |
| Initial message replay and key reuse | X3DH/PQXDH warn about replay of initial messages and key reuse; post-X3DH protocols must re-randomize (X3DH §4.2–4.3; PQXDH §4.2–4.3). | Suite-2 has OOO/replay defenses and durable replay store for ratchet messages (DOC-CAN-003 §8; CAT-S2-OOO-001; CAT-S2-CRASH-001). | No explicit replay cache for establishment messages in demo relay/harness. | Add establishment replay cache keyed by session_id + bundle ids. | NA-0020 (G3/G4) | CI: establishment replay vector must reject second use. |
| Server trust and rate limiting | Server can refuse or replay bundles; rate limiting and OPK depletion handling are recommended (X3DH §4.7; PQXDH §4.9). | Demo relay enforces queue caps and token auth (NA-0016). | Register/poll not rate-limited; high-rate probing could leak metadata and exhaust resources. | Add simple rate limiting and backoff in demo relay. | NA-0021 (G5) | CI: enforce 429 on excessive register/poll. |
| Key identifiers and collision risk | PQXDH allows smaller identifiers but warns collisions cause decrypt failures (PQXDH §4.13). | session_id is 16 random bytes (DOC-CAN-003 §4.2). | Demo IDs for relay are stable strings without collision handling. | Introduce relay id format guidance and collision detection in demo CLI. | NA-0022 (G5) | CI: duplicate id registration must reject. |
| Skipped message key deletion | Double Ratchet recommends deleting skipped message keys after use (Double Ratchet §2.6; §8.4). | Suite-2 OOO path deletes consumed MKSKIPPED entries (DOC-CAN-003 §8.1; CAT-S2-OOO-001). | Need explicit policy for eviction when bounds reached and after successful decrypt. | Add explicit deletion/eviction invariants to DOC-CAN-003 and vectors. | NA-0023 (G2/G4) | CI: negative vectors ensure eviction and no reuse. |
| Post-quantum handshake integration | Double Ratchet spec integrates PQXDH with SPQR/Triple Ratchet (Double Ratchet §7.1, §5). | Suite-2 uses pq_init_ss + SCKA epochs; establishment mapping clarifies base handshake outputs (DOC-CAN-003 §6). | No explicit mapping from PQXDH-style bundles to SCKA epoch lifecycle. | Add a mapping appendix: PQXDH outputs -> SCKA initial epoch rules. | NA-0024 (G2/G3) | CI: establishment + first boundary vectors enforce mapping. |
| KEM re-encapsulation mitigation | PQXDH highlights re-encapsulation attack; mitigation: bind KEM public keys in AD or use contributory KEM (PQXDH §4.12). | Suite-2 pq_bind includes PQ_PREFIX; establishment contract does not explicitly bind pqkem public key. | Potential gap: base handshake may not bind PQ public key into transcript. | Require binding PQ prekey identifier or public key into establishment transcript/AD. | NA-0025 (G3) | CI: establish vectors reject if PQ binding missing. |
| Forward secrecy and key deletion | Double Ratchet emphasizes deleting old chain keys and secure deletion of past epoch state (Double Ratchet §8.1, §8.10; SPQR §5.7). | Suite-2 uses transactional commit and SCKA tombstones; no explicit secure deletion wording for demo state. | Memory/on-disk deletion policy not fully specified for demo store. | Add explicit demo store secure deletion guidance + best-effort deletion semantics. | NA-0026 (G2/G5) | CI: store files not world-readable; removal on rotate. |
| Identity misbinding and UI binding | X3DH/PQXDH warn about identity misbinding; recommend binding identity info in AD or UX verification (X3DH §4.8; PQXDH §4.10). | QSL relies on system-layer authentication before Suite-2 commit (DOC-CAN-003 §0.2, §6.3). | Demo UX does not surface identity verification steps. | Add explicit demo UX warnings and optional identity fingerprint display. | NA-0027 (G5) | CI: CLI warns on first establish unless override. |
| Denial-of-service protection | Server can be abused by repeated bundle requests or OPK exhaustion (X3DH §4.7; PQXDH §4.9). | Demo relay enforces queue caps, token auth (NA-0016). | Register/poll endpoints still allow high-rate scans within caps. | Add per-token quota and simple timing backoff. | NA-0028 (G5) | CI: conformance smoke verifies backoff or rejection. |

## 6. Top actionable upgrades (ranked)

### Upgrade #1 — Establishment replay cache and bundle consumption

- Summary: Add replay protection for establishment messages and enforce one-time prekey consumption at the relay.
- Scope guard (what must NOT change): No Suite-1/1B behavior; no new wire formats.
- Acceptance criteria (objective/testable): establishment replay and OPK reuse rejected with explicit reason codes; relay records consumption.
- CI gate (what enforces it): CAT-S2-ESTABLISH-002 vectors + relay conformance smoke.
- Evidence (what proves completion): vector runs in suite2-ci and relay smoke artifacts.
- Proposed queue item: NA-0020 (G3/G4) + NA-0018 (G3/G5).

### Upgrade #2 — Explicit identity binding for demo establish

- Summary: Bind peer identity identifiers into establishment transcript/AD (demo layer) to reduce misbinding risk.
- Scope guard: Do not alter Suite-2 wire; identity binding is a demo-layer check.
- Acceptance criteria: establish fails if identity binding is absent or mismatched.
- CI gate: CAT-S2-ESTABLISH-IDENTITY-001 vectors.
- Evidence: suite2-ci vectors + demo CLI warnings.
- Proposed queue item: NA-0019 (G3/G5).

### Upgrade #3 — PQ KEM public key binding

- Summary: Require binding of PQ prekey identifier or public key into establishment transcript to prevent re-encapsulation attacks.
- Scope guard: No changes to PQ KEM wire formats; binding only in establishment contract.
- Acceptance criteria: establish rejects if PQ binding missing; documented in DOC-CAN-003.
- CI gate: establishment vectors with missing PQ binding must reject.
- Evidence: suite2-ci establish vectors updated.
- Proposed queue item: NA-0025 (G3).

### Upgrade #4 — Relay rate limiting + abuse backoff

- Summary: Add simple rate limiting for register/poll to reduce metadata probing and OPK exhaustion.
- Scope guard: Demo relay only; local-only by default.
- Acceptance criteria: repeated register/poll above threshold returns 429; token-based quotas enforced.
- CI gate: metadata-conformance-smoke verifies 429 on excess.
- Evidence: CI logs + updated DOC-G5-002 leakage row.
- Proposed queue item: NA-0021 (G5).

### Upgrade #5 — Secure deletion policy for skipped keys and store artifacts

- Summary: Add explicit deletion/rotation policy for skipped keys and demo store artifacts.
- Scope guard: No changes to Suite-1/1B behavior.
- Acceptance criteria: explicit deletion rules in DOC-CAN-003/004; demo store files are 0600/0700; stale keys removed on rotate.
- CI gate: metadata-conformance-smoke checks perms + unit tests for skipped key eviction.
- Evidence: CI passing + traceability links.
- Proposed queue item: NA-0026 (G2/G5).

## 7. Non-goals / do-not-chase list

- Build or integrate an anonymity network (mixnets, PIR, Tor/I2P).
- Copy Signal code or reproduce Signal client/server architecture.
- Provide “metadata eliminated” claims; only measurable minimization.
- Redesign Suite-1/1B or change their wire formats.
- Mobile UX parity or production deployment guidance.

## 8. Appendix — additional observations (unbounded, not prioritized)

- Double Ratchet emphasizes secure deletion and public key rotation policies; QSL can align on terminology and explicit memory hygiene.
- PQXDH notes KEM contributory properties and re-encapsulation risks; QSL should decide whether to bind PQ public keys or identifiers.
- Identity misbinding is a recurring issue; QSL demo UX should make identity verification visible and explicit.
