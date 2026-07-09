Goals: G4 (primary), supports G1–G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-06

# DOC-OPS-007 — Improvement & Findings Ledger

Purpose: this is the single, committed, cross-lane backlog of engineering
findings and workflow/process recommendations. Because every lane runs as a
fresh assistant in a different qwork workspace path, per-session assistant
memory does NOT carry across lanes — **the committed repo is the only reliable
continuity channel**, so durable findings and recommendations live here.

This ledger is subordinate to the governance spine and does not reorder or
override `NEXT_ACTIONS.md`. It informs successor selection (see the Director
triage discipline in `docs/ops/DIRECTOR_OPERATIONS.md`); it does not authorize
work by itself. Entries are class-only: never record secrets, endpoints, ports,
tokens, capabilities, keys, plaintext, or ciphertext bodies here.

## How every lane uses this ledger (binding)

- READ this ledger during the Director phase of every lane (it is in the
  read-first lists of `CLAUDE.md`, `START_HERE.md`, and `AGENTS.md`).
- FILE or UPDATE an entry before closeout whenever a lane discovers an
  engineering finding or a workflow/process issue, or advances/resolves one.
- Update `status`, `originating/last lane`, and `last-updated` on any change.
- Promotion of a stable recurring lesson into canon still follows
  `docs/ops/DOC-OPS-004`. This ledger is the staging list; canon is the durable
  home once a rule is load-bearing.

## Status lifecycle

`open` → `queued` (accepted for a future lane) → `in-lane` (an active NA item is
addressing it) → `promoted` (turned into canon/decision) or `done` (resolved with
evidence) — or `wontfix` (closed with a recorded reason). Never delete entries;
close them.

## Entry ID convention

`ENG-####` for engineering findings, `WF-####` for workflow/process items;
monotonic per prefix, never reused.

## Engineering-finding schema (per DOC-AUD-001 §6)

Severity (`P0`/`P1`/`P2`/`P3`); Title; Exact surfaces (file/function/test/spec);
Claim violated; Why it matters (exploit or correctness-failure path); Minimal fix
direction (not a full design); Proof gap (missing regression/vector/property/fuzz);
Recommended directive shape (implementation-only / docs-evidence-only /
audit follow-on). Reject anti-patterns: "needs refactor", "crypto should be
reviewed", "tests insufficient" without naming the exact missing proof, or broad
severity with no failure path.

## Workflow-item schema

Title; Problem; Recommended change; Status; Originating/last lane; Last-updated.

---

## Engineering findings

### ENG-0001 — qsc identity/handshake verification-fingerprint semantics unclear
- Severity: P3 (robustness/UX footgun; not an identity-binding defect)
- Status: done — the self-label footgun remediated by NA-0616 (D-1227); last-updated 2026-07-07
- Resolution (NA-0616): the self-label divergence footgun is fixed fail-closed. The
  auto-create branch of `identity_self_kem_keypair` now refuses to mint a SECOND,
  divergent self-identity when the config dir already holds one (emitting
  `identity_self_ambiguous` / `ErrorCode::IdentitySelfAmbiguous`); first-run auto-create
  and explicit `identity rotate` are preserved. The handshake `--as` default is aligned
  with `identity show`'s `"self"`. The verification-fingerprint model itself was already
  coherent (NA-0609B); this closes the residual footgun. See the NA-0616 evidence doc.
- Resolution (NA-0609B): the verification-fingerprint model is COHERENT — the
  primary pin is checked against the KEM identity fingerprint that `identity show`
  displays, with the ML-DSA signing-key fingerprint as a separate optional pin;
  there is no KEM-vs-SIG binding flaw. The residual is a P3 footgun: an
  inconsistent `--as <label>` self-label silently operates a divergent
  lazily-created identity instead of failing loud. Minimal fix: fail loud on an
  unknown self label, and/or document the single-self-label convention. See the
  NA-0609B evidence doc.
- Exact surfaces: `qsl/qsl-client/qsc/src/identity/mod.rs`,
  `qsl/qsl-client/qsc/src/handshake/mod.rs` (identity-show fingerprint vs the
  handshake peer-verify path; `--as <label>` self-identity selection)
- Claim potentially at stake: G3-adjacent operator-verifiable identity binding
- Why it matters: while driving the NA-0608 harness, the fingerprint an operator
  would compare out of band vs. the fingerprint the handshake authenticates, and
  the effect of inconsistent `--as <label>` values (which selected divergent
  lazily-created identities and produced `peer_mismatch`), were error-prone and
  ambiguous. Recorded as audit-needed, NOT a confirmed defect.
- Minimal fix direction: clarify/document the single canonical verification
  fingerprint and the self-label model; make inconsistent-label use fail loud
  rather than silently diverge.
- Proof gap: no test asserts which fingerprint an operator verifies, or that
  identity-show and handshake-verify fingerprints are consistent/clearly distinct.
- Recommended directive shape: read-only handshake/identity audit (DOC-AUD-001 §4.1),
  then a bounded remediation lane only if the audit substantiates a concrete root cause.

### ENG-0002 — qsc attachment upload session single-use per qsc session
- Severity: P3 (clarity/documentation → resolved with a fix)
- Status: done — resolved (fixed) by NA-0617 (D-1229); originating lane NA-0608
  (D-1209); last-updated 2026-07-07
- Resolution (NA-0617): disambiguated into a two-layer session model and fixed a
  client footgun. L1 = the qsl-attachments SERVICE upload session (single-object BY
  DESIGN: create → upload → commit → session consumed/removed, object persists);
  reuse fails closed with `REJECT_QATTSVC_SESSION_STATE`. L2 = the qsc CLIENT session
  (config dir/identity across `file send` invocations), which is NOT limited to one
  attachment — distinct sends each mint their own L1 session. The reject was L1
  fail-closed behavior on session reuse, not an L2 cap. Footgun fixed:
  `attachment_find_outbound_by_source` now excludes consumed-session states
  (`COMMITTED`/`ACCEPTED_BY_RELAY`, in addition to `PEER_CONFIRMED`) from reuse, so a
  re-send of an already-delivered file mints a fresh session and succeeds; resumable
  (`SESSION_CREATED`/`UPLOADING`) and in-flight (`AWAITING_CONFIRMATION`) states are
  preserved. Client journal logic only; no protocol/wire/crypto/state-machine or
  attachment-format change. Pinned by `na_0617_attachment_single_send_per_session`
  (4/4) with a negative control and full `attachment_streaming_na0197c` regression.
  See the NA-0617 evidence doc.
- Exact surfaces: `qsl/qsl-client/qsc/src/attachments/mod.rs`; qsl-attachments
  service session-state path (`REJECT_QATTSVC_SESSION_STATE`)
- Claim potentially at stake: none security-critical; operability/predictability
- Why it matters: a second `file send` in the same qsc session returns
  `REJECT_QATTSVC_SESSION_STATE`; unclear whether this is intended (one attachment
  per session) or a client session-reuse limitation. Made multi-send harnessing
  require a fresh session per send.
- Minimal fix direction: document the intended per-session send semantics and, if
  by design, assert it with a test; if a limitation, file a bounded fix lane.
- Proof gap: no documented/tested statement of multi-send-per-session behavior.
- Recommended directive shape: docs/evidence-only clarification, or a small audit
  follow-on within the attachment hardening track (NA-0609).

### ENG-0003 — Non-constant-time keyed-MAC comparisons in the handshake accept path
- Severity: P3 (implementation-attack surface; low current exploitability)
- Status: done — remediated by NA-0609C (D-1214); last-updated 2026-07-06
- Resolution (NA-0609C): added a dependency-free constant-time 32-byte helper
  `hs_ct_eq_32` in handshake/mod.rs and used it at both MAC-comparison sites
  (:1458 B1 transcript MAC, :1665 A2 confirm MAC); timing-only, accept/reject
  semantics bit-for-bit unchanged; proven by a co-located unit test (equivalence
  to `==`) and the existing handshake suites passing. Residual: other tag/MAC
  comparison sites outside the handshake seam are not in scope and remain future
  work if a review finds them.
- Exact surfaces: `qsl/qsl-client/qsc/src/handshake/mod.rs:1458` (B1 transcript
  MAC) and `:1665` (A2 confirm MAC); no constant-time equality helper exists in the
  qsc or refimpl crypto stack.
- Claim potentially at stake: defense-in-depth constant-time MAC/tag verification
  (implementation-attack resistance; G5-adjacent hardening).
- Why it matters: array `!=` short-circuits and is not constant-time; a precise
  timing oracle could in principle aid MAC forgery. Exploitability is LOW here
  (acceptance also requires a valid ML-DSA signature; keys are fresh per handshake),
  but the pattern is systematic and constant-time comparison is standard hygiene.
- Minimal fix direction: add a constant-time fixed-length comparison helper and use
  it at both sites; audit for other tag comparisons.
- Proof gap: no test asserts constant-time comparison for handshake MAC/tag paths.
- Recommended directive shape: implementation-only; natural first item for the
  NA-0609 implementation-attack hardening batch.

### ENG-0004 — Directory fsync is a no-op; atomic-rename durability not guaranteed
- Severity: P3 (crash-durability; fail-closed-safe direction)
- Status: wontfix — FALSE POSITIVE, re-verified by NA-0609D (D-1216); last-updated 2026-07-06
- Correction (NA-0609D): NOT a defect. `fsync_dir_best_effort` has two cfg-gated
  definitions in `fs_store/mod.rs`: the `#[cfg(not(unix))]` variant (line 359) is a
  no-op, but the `#[cfg(unix)]` variant (line 362) does the real directory fsync
  `File::open(dir).and_then(|d| d.sync_all())`. On the deployment target
  (x86_64-linux) `write_atomic` performs a full durable sequence: content
  `sync_all` -> atomic `rename` -> directory fsync. G2 crash-durability is sound on
  Unix; the non-unix no-op is a documented best-effort degradation. The NA-0609B
  audit erred by grepping only the `not(unix)` stub. See WF-0005.
- Exact surfaces: `qsl/qsl-client/qsc/src/fs_store/mod.rs:359`
  (`fsync_dir_best_effort` is empty), called after the rename in `write_atomic`.
- Claim potentially at stake: G2 crash-safe state persistence.
- Why it matters: file content is written atomically (temp + `sync_all` + rename),
  so no partial/corrupt file is possible, but the directory entry from the rename
  is not fsync'd, so a power-loss crash right after a store can revert to the prior
  state. Direction is fail-closed-safe (revert → re-handshake), but it is a real
  gap against the G2 durability gate.
- Minimal fix direction: implement a real directory fsync, or document the
  durability boundary explicitly against G2 if deferred.
- Proof gap: no crash/durability test exercises loss of the directory entry.
- Recommended directive shape: implementation-only (or docs boundary statement),
  scoped to fs_store.

### ENG-0005 — Constant-time comparison sweep beyond the handshake seam
- Severity: P3 (implementation-attack; defense-in-depth)
- Status: resolved-into-findings — swept by NA-0611 (D-1221); last-updated 2026-07-07
- Resolution (NA-0611): the sweep found NO keyed-secret/MAC comparison outside the
  already-fixed handshake seam. `kmac_out` uses are key derivations (not compares);
  AEAD tag verification is in-primitive constant-time; protocol_state has no secret
  compares; integrity-hash (SHA-512) and route-token compares are verified-acceptable
  (timing not exploitable). Constant-time posture sound. One optional P3
  defense-in-depth item recorded as ENG-0008. See the NA-0611 evidence doc.
- Surfaces: qsc tag/MAC/secret comparison sites outside handshake/mod.rs (e.g.
  attachment capability/token checks, vault unlock).
- Why it matters: ENG-0003 fixed the handshake MAC comparisons; a sweep should
  enumerate and confirm/convert any remaining secret comparisons. DOC-G5-005 §3,9
  (rank 2).
- Recommended directive shape: read-only audit, then a bounded fix lane per finding.

### ENG-0006 — Error/retry normalization review
- Severity: P3 (implementation-attack; distinguishability)
- Status: resolved-into-findings — reviewed by NA-0612 (D-1222); last-updated 2026-07-07
- Resolution (NA-0612): within qsc, the reject taxonomy (recv_reject_*, REJECT_ATT_*,
  REJECT_QSC_HS_*) is LOCAL-ONLY (operator stdout/log markers; not transmitted on the
  wire on reject), the retry path (`bounded_retry`) is cause-agnostic (unit error;
  attempt-count-derived backoff), and no reason-carrying NACK is sent on the wire.
  No remotely-observable failure-cause oracle beyond the send/fetch timing/size
  metadata already documented (NA-0608 / DOC-G5-005). Residual: optional P3 ENG-0009
  (deterministic retry jitter) plus a service-side scope note (qsl-server /
  qsl-attachments error normalization is out of qsc-repo scope). See the NA-0612
  evidence doc.
- Surfaces: qsc/qsl-server/qsl-attachments reject-code, timing, and retry/backoff
  behavior.
- Why it matters: distinct internal failure causes should not be externally
  distinguishable beyond the deterministic reject taxonomy. DOC-G5-005 §7,9 (rank 3).
- Recommended directive shape: audit/docs review, then a bounded normalization fix.

### ENG-0007 — Attachment-plane metadata mitigation feasibility (size/count/timing)
- Severity: P3 (metadata; highest-value residual)
- Status: resolved-into-findings — studied by NA-0613 (D-1223); last-updated 2026-07-07
- Resolution (NA-0613): DOC-G5-006 inventories the residual channels (C1 object size,
  C2 part count, C3 part-size-class-by-plaintext, C4 upload/fetch timing) and shows
  object-size/part-count bucketing (M1) is client-side feasible against the
  service/network observer without an attachment-contract change, because the
  descriptor (true plaintext_len) is peer-only inside the encrypted envelope while the
  service sees only the opaque padded object. Recommended mitigation filed as ENG-0010;
  timing/cover deferred as ENG-0011. Honest residual documented (no metadata
  elimination). See DOC-G5-006.
- Surfaces: qsl-attachments object storage/service contract; qsc attachment path.
- Why it matters: NA-0608 showed ciphertext-object size, object/part count, and
  upload/fetch timing are EXPOSED residual metadata on the attachment plane (the
  message plane already has padding/bucketing). DOC-G5-005 §2,6,9 (rank 4). Touches
  the attachment contract; needs feasibility+design before any behavior change.
- Recommended directive shape: read-only feasibility+design (cost/benefit matrix),
  then a separate implementation lane if justified.

### ENG-0008 — Verification-code equality is not constant-time (optional defense-in-depth)
- Severity: P3 (defense-in-depth; NOT exploitable)
- Status: open — originating lane NA-0611 (D-1221); last-updated 2026-07-07
- Surfaces: `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs:1194` and
  `qsl/qsl-client/qsc/src/contacts/mod.rs:1237` (`if expected == provided`).
- Why it matters: the trust-promotion gate compares the pinned identity fingerprint
  against the operator-provided code with `==` (not constant-time). This is NOT a
  keyed-secret comparison: the fingerprint is public (safety-number-style, derived
  from the peer's public key), and any local attacker able to build a timing oracle
  already has direct read access to it — so there is no practical timing advantage.
- Minimal fix direction: use a constant-time fixed-length comparison at these two
  sites if a future lane elects the hardening.
- Recommended directive shape: optional small implementation-only lane; low priority.

### ENG-0009 — Deterministic retry backoff jitter (optional defense-in-depth)
- Severity: P3 (defense-in-depth; NOT a failure-cause oracle)
- Status: open — originating lane NA-0612 (D-1222); last-updated 2026-07-07
- Surface: `qsl/qsl-client/qsc/src/main.rs` `bounded_retry` — the backoff jitter is
  deterministic (attempt-count-derived), not randomized.
- Why it matters: retry is cause-agnostic and retry attempts are not remotely
  observable in the current model, so this leaks no failure cause. It is recorded only
  because a deterministic backoff is more predictable than a randomized one in a
  hypothetical live send-retry-to-relay scenario; any concern there ties to the
  send-timing metadata already tracked (NA-0608 / DOC-G5-005), not to distinguishability.
- Minimal fix direction: randomize the retry jitter only if send-retry-to-relay timing
  is ever made a live mitigation target; otherwise no action.
- Recommended directive shape: optional small implementation-only lane; low priority.

### ENG-0010 — Attachment-plane object-size/part-count bucketing (recommended mitigation)
- Severity: P3 (metadata; highest-value residual — the top NA-0613 recommendation)
- Status: resolved (implemented) — shipped by NA-0614 (D-1224); last-updated 2026-07-07
- Resolution (NA-0614): mandatory baseline attachment-object padding (DOC-G5-007):
  additive authenticated `content_len` vs padded `plaintext_len`, receiver truncation
  with the exact-length check preserved, AAD/confirm binding, a sender size ladder with
  a ladder-agnostic receiver. The service/network observer now sees only a bucketed
  object size. Part-count is bucketed as a consequence. Access existence/timing residual
  remains ENG-0011.
- Surface: `qsl/qsl-client/qsc/src/attachments/mod.rs` object sizing/chunking path.
- Why it matters: today `ciphertext_len = plaintext_len + part_count*tag` (no object
  padding), so the service/network observer learns the plaintext size almost exactly
  (C1), plus a coarse count (C2) and a 3-way class band (C3). DOC-G5-006 M1/M2/M3.
- Design (client-only, no contract change): pad the plaintext to a defined size ladder
  before chunk/AEAD; keep descriptor `plaintext_len` true (peer decrypt truncates);
  `ciphertext_len`/`part_count`/integrity root reflect the padded object; choose the
  part-size-class from the padded size. Keep all size fields consistent in the per-part
  AAD and confirm MAC.
- Recommended directive shape: full-ritual implementation lane (NA-0614) with
  deterministic bucketed-size vectors, fail-closed decrypt/truncation preserved, and
  explicit bandwidth/storage-overhead accounting. No metadata-free claim.

### ENG-0011 — Attachment upload/fetch timing and cover-traffic (deferred, cross-repo)
- Severity: P3 (metadata; deferred)
- Status: open — originating lane NA-0613 (D-1223); last-updated 2026-07-07
- Surface: qsl-attachments service/deployment (primary); optional qsc send/fetch jitter.
- Why it matters: upload/fetch timing and access pattern (C4) are observable by the
  service/network and are largely a qsl-attachments/deployment property, not a qsc-only
  concern; cover traffic is high-cost.
- Recommended directive shape: separate cross-repo design/implementation in
  qsl-attachments; optional small qsc-side jitter follow-up. Lower priority than ENG-0010.

### ENG-0012 — Suite-2 send-side ratchet liveness gap (no DH ratchet + no boundary/PQ-reseed sender)
- Severity: P1 (blocks the G1/G2 release gates; top-priority engineering finding)
- Status: CLOSED (the P1 resolved at NA-0624, D-1243) — design-complete (NA-0619, D-1234); Stage 1a
  (DH-ratchet state plumbing) DONE (NA-0620, D-1235); Stage 1b-i (DH-ratchet SEND+RECEIVE behavior
  + NHK header keys, in refimpl) DONE (NA-0621, D-1237); Stage 1b-ii (qsc trigger + static-`rk`
  removal — the classical DH ratchet now runs on the REAL client send path) DONE (NA-0622, D-1239).
  The CLASSICAL half of the P1 closed there — classical post-compromise security on live qsc
  traffic (ratchet-on-reply + N=4/T=15min fallback), proven end-to-end over a real handshake
  (round-trip + PCS-healing).
  Stage 2 (PQ-reseed sender) was sub-staged: Stage 2a (the refimpl SCKA sender core — advertisement
  + PQ reseed — plus the both-sides RK advance so the PQ hardening survives a DH ratchet) DONE
  (NA-0623, D-1241). Stage 2b (qsc SCKA wiring = NA-0624, D-1243) DONE — the P1 is CLOSED:
  post-quantum forward secrecy now runs on live qsc traffic, proven end-to-end over a real
  handshake (advertise -> reseed -> both decrypt; PQ-PCS healing that survives a subsequent DH
  ratchet; G2 rollback fails closed). NO post-quantum / Triple-Ratchet / post-compromise CLAIM
  until the DH+PQ composition is independently analyzed (the standing claim boundary). Filed
  NA-0617 (D-1230) from the external Suite-2 code/crypto review;
- Stage 1a (NA-0620): added a session-level `Suite2DhRatchetState` (`dhs_priv`/`dhs_pub`/`dhr`/
  `rk`) to `Suite2SessionState`, populated at establishment (the qsc handshake threads its
  retained X25519 ephemeral private key via `set_dh_self_priv`), and persisted via a snapshot
  format bump to v2 (fail-closed on any non-v2 version). PLUMBING ONLY — no message-path/wire/
  nonce/KDF/AEAD change; the static-`rk` bootstrap is untouched (removed in Stage 1b). Proven
  by DH round-trip + non-v2 fail-closed unit tests and the full suite2/qsc regression
  (including the runtime-equivalence test) passing byte-for-byte. See the NA-0620 evidence doc.
- Stage 1b-i (NA-0621): implemented the classical DH ratchet in refimpl — `KDF_RK_DH` (§3.3.2,
  `KMAC256(RK,"QSP5.0/RKDH",dh_out,64)`), on-demand `HK/NHK` header keys (§3.4/§8.1),
  `send_boundary` (DH-ratchet send, §8.5.2: fresh X25519 keypair, `KDF_RK_DH`, PQ send-chain
  reinit, `HK_s` recompute, header under the pre-boundary `NHK_s`), and `recv_dh_boundary`
  (DH-ratchet receive + §8.5.1 CURRENT_NHK anti-spoof, no state mutation on reject). No
  wire-format change (the `DH_pub[32]` per §4.3 is already on the wire), no non-boundary-path
  change, no PQ-reseed (`apply_pq_reseed`) change, no snapshot change (NHK derived on demand).
  Proven by co-located refimpl tests: two-party round-trip (the ratchet fires both directions and
  messages decrypt), PCS-healing (a pre-ratchet state snapshot cannot decrypt post-ratchet
  messages once both parties have advanced), and no-mutation-on-reject; the full suite2/qsc
  regression stays green. NOT wired into qsc and NOT a post-compromise claim yet (Stage 1b-ii).
  See the NA-0621 evidence doc. last-updated 2026-07-08
- Stage 1b-ii (NA-0622): wired the classical DH ratchet into the REAL qsc send/receive path and
  removed the static-`rk` bootstrap (`qsp_activate_responder_send_chain_if_needed`,
  `qsp_activate_initiator_recv_chain_if_needed`). `qsp_pack` originates a DH boundary via the
  refimpl `send_boundary` when the trigger fires — RATCHET-ON-REPLY (first send after any receive)
  plus a bounded fallback of N=4 messages / T=15 min; `qsp_unpack` routes incoming DH boundaries
  to the refimpl `recv_dh_boundary`. The reply-driven trigger (a `pending` bit + N/T counters) is
  persisted in a new qsc session-blob v2 plaintext (`b"QTRG"` + trigger + snapshot), keeping the
  refimpl Suite2SessionState / QS2S snapshot FROZEN; legacy blobs migrate transparently. No
  wire-format change (DH_pub already on the wire), no refimpl change, no PQ-reseed change. The
  ratchet is gated OFF for a degenerate self-DH session (`dhr == dhs`, the symmetric both-role-A
  seed-fallback TEST model that cannot round-trip the directional ratchet) — a SESSION-STATE check,
  not the seed-permitted flag (real-handshake tests set that flag too), so the pre-ratchet
  seed-model regression suite stays byte-for-byte green while REAL handshake sessions (dhr != dhs)
  always ratchet. Proven
  end-to-end over a REAL A/B handshake: `dh_ratchet_e2e_roundtrip_over_real_handshake` (the ratchet
  fires both directions and messages decrypt) and `dh_ratchet_e2e_pcs_healing_over_real_handshake`
  (a pre-ratchet session snapshot cannot decrypt a post-ratchet message), plus the updated
  runtime-equivalence test (deterministic path byte-for-byte equivalent + ratchet-on-reply fires)
  and the full qsc regression. The DH-boundary observable is recorded in DOC-G5-004; cover-traffic
  obfuscation is deferred to ENG-0022. See the NA-0622 evidence doc. last-updated 2026-07-08
- Stage 2a (NA-0623): implemented the Suite-2 SCKA sender core IN REFIMPL and — per the D560
  AMENDMENT — the both-sides ROOT ADVANCE. Adds `KDF_RK_PQ` (§3.3.3,
  `KMAC32(RK,"QSP5.0/RKPQ",pq_ss||[0x01])`); the SCKA advertisement sender (`send_pq_advertise`),
  peer-ADV monotonicity tracking (`track_peer_adv`), and the PQ-reseed sender (`send_pq_reseed`,
  §8.5.3/§8.5.4 + DOC-CAN-004 §3.1–§3.3). The AMENDMENT fix: the receiver (`recv_boundary_in_order`)
  now advances `RK := KDF_RK_PQ` and recomputes `HK_r` after `apply_pq_reseed` (it previously
  absorbed `pq_epoch_ss` into the PQ chains only — §8.5.3 steps 5+7 were unimplemented — so the next
  DH ratchet reinitialised `CK_pq` from the un-hardened root and WIPED the PQ protection), and the
  new sender mirrors it, writing the advanced root to BOTH root slots so the classical DH ratchet
  carries the PQ hardening forward permanently. The advertised-key store / ML-KEM KeyGen+Encap are
  CALLER-side (the refimpl sender is pure functions); the SCKA target sets already persist (snapshot
  v2 — no bump); `parse.rs` already parses `FLAG_PQ_ADV`/`FLAG_PQ_CTXT` (no wire change). The
  `KDF_PQ_RESEED` seeds are reused from `apply_pq_reseed` (its CTXT-validation semantics unchanged;
  its vectors byte-identical). Proven by co-located refimpl integration tests — round-trip
  (advertise -> encapsulate -> `apply_pq_reseed` decrypts + converges) and, the headline,
  `pq_pcs_healing_survives_dh_ratchet` (a pre-reseed snapshot cannot open the post-reseed DH
  boundary) — plus fail-closed sender rejects / one-time / peer-ADV monotonicity, harness ops, and 6
  byte-pinned CAT-SCKA-LOGIC-001 vectors; the frozen `apply_pq_reseed`/boundary/SCKA-KEM/KDF vectors
  and the full refimpl suite stay green. NOT wired into qsc and NOT a post-quantum claim on live
  traffic yet (Stage 2b). NHK note: the refimpl PQ-CTXT boundary header uses `HK` (the frozen
  receiver), not the §8.5.1 `NHK` — flagged for Stage 2b / a spec-alignment lane. See the NA-0623
  evidence doc. last-updated 2026-07-08
- Stage 2b (NA-0624): wired the Stage-2a SCKA sender into the REAL qsc send/receive path, reusing
  the frozen refimpl semantics exactly (no refimpl change; the seed-model runtime-equivalence test
  stays byte-for-byte). `qsp_pack` originates SCKA advertisements as separate CONTROL envelopes
  pushed before the main message (the frozen receiver has no ADV body decrypt path) — on
  establishment, on consumption of the local advertised key, and on rotation — and originates PQ
  reseeds via the frozen `send_pq_reseed` on the operator-approved sparse cadence (first reseed as
  soon as a fresh unconsumed peer advertisement is available, then every N_pq=8 sent DH boundaries
  or T_pq=3600 s, evaluated on non-boundary sends so reseeds co-schedule after DH boundaries).
  `qsp_unpack` intercepts `FLAG_PQ_ADV` before `recv_wire` (validating via `track_peer_adv`) and
  routes `FLAG_PQ_CTXT` through ML-KEM decapsulation into the frozen `apply_pq_reseed`, first
  INJECTING the canonical session root (`recv.rk := dh.rk` when live — the frozen reseed sender
  derives from `session_root` while a DH boundary advances only `dh.rk`; the NA-0623 dh.rk-sync
  carry-over, resolved caller-side) and then ADOPTING the advanced root into the DH-ratchet slot
  (`dh.rk := recv.rk`) so a later classical DH ratchet carries the PQ hardening. SCKA state
  (bounded advertised-key store, CAP=4 with deterministic eviction; peer advertisement; cadence
  counters) persists inside the AEAD session blob as a length-delimited v3 plaintext section
  (QS2S snapshot FROZEN; v2/v1 migrate; ML-KEM secrets only inside the encrypted blob) with a G2
  monotonic side-record (incl. `peer_adv_consumed_max`, so a rolled-back store can never
  re-consume a one-time peer target) — a rolled-back blob FAILS CLOSED
  (`session_rollback_detected`). An advertisement never shares a pack with a reseed (the control
  envelope consumes a chain slot only a normal message's OOO skip or a DH epoch reset can absorb;
  the frozen reseed receiver is strict-in-order), so a due advertisement defers to the next send.
  Enabling fix: the transport deliver path now persists the qsp_pack trigger (the NA-0622
  cleared-flag/fallback counters previously never landed there, so every post-receive send
  ratcheted and a non-boundary reseed send could never fire). Proven end-to-end over a REAL A/B
  handshake: `scka_e2e_advertise_reseed_roundtrip_over_real_handshake` (advertise -> reseed
  mid-conversation in both directions -> both decrypt, with a DH boundary riding the PQ-advanced
  root); `scka_e2e_pq_pcs_healing_survives_dh_ratchet_over_real_handshake` (THE HEADLINE — a
  pre-reseed snapshot holding every CLASSICAL secret, including the DH private key, cannot decrypt
  the post-reseed-post-DH message; only the ML-KEM shared secret encapsulated to the peer's key is
  missing); `scka_e2e_rolled_back_session_blob_fails_closed` (G2), plus fail-closed
  codec/rollback/eviction unit tests. Flagged deviations deferred to the spec-alignment successor
  candidate (with the §8.5.1 NHK item): ADV tracking is UNAUTHENTICATED (the frozen receiver has
  no ADV path — length+monotonicity only; a relay-level injector can plant an advertisement;
  bounded: the reseed still mixes into RK, so classical security is unaffected and the PQ layer
  degrades at worst to "no reseed", plus a tracking-DoS via a max adv_id); a lost ADV/reseed
  envelope degrades to the classical status quo until rotation. The PQ-ADV/PQ-CTXT wire
  observables are recorded in DOC-G5-004 §3.1 (Operator Decision 4); cover traffic stays deferred
  to ENG-0022. See the NA-0624 evidence doc. last-updated 2026-07-08
- Design (NA-0619): `docs/design/DOC-G5-008_Suite2_Send_Side_Ratchet_Liveness_Feasibility_and_Design_v0.1.0_DRAFT.md`
  establishes feasibility (receiver machinery + `qsp::dh_ratchet_send` reference + complete
  DOC-CAN-003 §8.5 spec) and a staged plan: Stage 1 classical DH ratchet on the real send path
  (remove the static-`rk` bootstrap; two-party vectors) → NA-0620; Stage 2 PQ reseed sender;
  Stage 3 spec + claim reconciliation. Requires adding DH keypair / `DHr` / live `RK` to the
  send/recv state (currently absent). Corrected an audit imprecision: parse permits DH-only
  boundaries. Binding claim boundary: no Triple-Ratchet / post-compromise / quantum-secure
  claim until Stages 1–2 land and vectors pass.
  (findings C-1 + C-2); last-updated 2026-07-07
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/suite2/{ratchet.rs,establish.rs,
  scka.rs}`; `qsl/qsl-client/qsc/src/main.rs` send path (`send_wire_canon(..., 0, ...)`).
- Claim at stake: G1 (per-message hybrid keys with a live classical DH ratchet + sparse PQ
  reseed) and the "Triple Ratchet" description; DOC-CAN-003 §8.5.2 (DH boundary).
- Why it matters: the shipped `suite2` module never executes a classical X25519 DH ratchet
  (`rk`/`hk_s`/`hk_r` are assigned once in `init_from_base_handshake` and never updated; no
  X25519 use in the module), AND there is no sender-side path for boundary/PQ-reseed
  messages (`send_wire` rejects any nonzero `flags`; no `send_boundary`/`send_pq_*`
  anywhere), so SCKA epoch advancement and `apply_pq_reseed` are receive-only and
  unreachable from the real client. Net delivered property: forward secrecy by chain-key
  deletion only, for the session lifetime, with NO post-compromise self-healing — weaker
  than plain Signal and contradicting the spec/name. A live state snapshot compromises all
  future messages in the session.
- Minimal fix direction: design first (trigger policy — e.g. every N messages / T seconds;
  DH-only vs co-scheduled DH+PQ boundary reconciled with `parse_pq_prefix` and DOC-CAN-003
  §8.5.2; metadata/traffic-shape/G5 implications), then implement `send_boundary`/
  `send_pq_ctxt` mirroring the existing receive side, wired to the client send path.
- Proof gap: no conformance vector exercises a full two-party session where the DH ratchet
  and PQ reseed fire mid-conversation through the real client send path and messages still
  decrypt.
- Cross-repo note: primarily qsl-protocol (refimpl + qsc), but reconciliation touches the
  canonical spec (DOC-CAN-003). Driving queue/operator for the multi-repo implementation
  is TBD (operator to confirm).
- Recommended directive shape: docs-only feasibility+design lane first, then staged
  implementation lane(s) with conformance vectors. Blocking for any production /
  quantum-secure / Triple-Ratchet / post-compromise claim.
- Addendum (D-1231, from the Comprehensive Audit — sharpens, does not change severity): the
  client does not merely leave the reverse direction static — it MANUFACTURES both chains
  from the same static root key. `qsp_activate_responder_send_chain_if_needed` /
  `_initiator_recv_chain_if_needed` (`qsl/qsl-client/qsc/src/main.rs` ~:2130/:2153) set
  `ck_ec`/`ck_pq` via `kmac_out(&st.recv.rk, "QSP5.0/CK0/B->A", ...)`. Net: the ENTIRE
  bidirectional key schedule is a deterministic function of one `rk` fixed at establishment;
  no fresh entropy is ever injected in either direction. Confirms "no PCS" and that the fix
  must also remove this static-`rk` bootstrap in favour of real ratchet steps.

### ENG-0013 — Suite-2 symmetric counter (ns/nr) overflow hard-stop missing
- Severity: P2 (nonce-reuse-class at saturation; bounded precondition)
- Status: done — resolved by NA-0618 (D-1232); filed NA-0617 (D-1230) from the Suite-2
  review (H-1); last-updated 2026-07-07
- Resolution (NA-0618): added a `checked_counter_inc` helper (fail-closed `u32::MAX`
  increment) used at all three ns/nr advance sites in `suite2/ratchet.rs` (`send_wire`,
  `recv_nonboundary_ooo`, `recv_boundary_in_order`) in place of `saturating_add`; on
  saturation the send returns `Err(REJECT_S2_COUNTER_OVERFLOW)` and the recv paths return a
  reject with that reason and NO state mutation (the transactional no-mutation-on-reject rule
  holds). New reject code `REJECT_S2_COUNTER_OVERFLOW` registered in DOC-CAN-003 §10 (local
  reason code; not wire-transmitted). Pinned by `checked_counter_inc_boundary_and_normal` and
  `send_wire_rejects_counter_overflow_at_ns_max_and_no_mutation`; the receive-side guards use
  the same helper (unreachable via a compliant sender). Full refimpl suite green (no
  regression). See the NA-0618 evidence doc.
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` `send_wire`
  (`ns`), `recv_nonboundary_ooo`/`recv_boundary_in_order` (`nr`) — all `saturating_add`
  with no `u32::MAX` guard; the sibling `qsp/ratchet.rs` has the guard.
- Claim at stake: fail-closed message-counter monotonicity; header-nonce uniqueness.
- Why it matters: at `ns==u32::MAX` in one direction `saturating_add` freezes the counter;
  with static header keys (see ENG-0012) the header ciphertext then repeats byte-for-byte —
  a nonce-reuse-class failure. Bounded behind ~4.29e9 messages/direction, but a real defect
  with the fix pattern already in the same crate.
- Minimal fix direction: add the identical `u32::MAX` guard to the three sites, forcing a
  re-handshake on hit rather than saturating.
- Proof gap: no vector drives a counter to saturation and asserts fail-closed.
- Recommended directive shape: small source/test lane (NA-0618).

### ENG-0014 — qsl-server non-constant-time bearer/route-token comparison
- Severity: P2 (impl-attack; cross-repo)
- Status: open — filed NA-0617 (D-1230) from the Suite-2 review (H-3); last-updated
  2026-07-07
- Exact surfaces: qsl-server `src/lib.rs` `auth_ok` (`provided == token`) and per-channel
  route-token resolution (ordinary HashMap lookup); `relay_token: None` disables auth.
- Claim at stake: constant-time credential comparison (consistent with the qsc ENG-0003
  fix and the ML-DSA timing-oracle audit posture).
- Why it matters: ordinary `&str` equality short-circuits and is not constant-time; the one
  place timing-side-channel hygiene was missed. The `None` relay token disabling auth is a
  dev-only posture that must never be a production default.
- Minimal fix direction: `subtle::ConstantTimeEq` (or manual byte-accumulate) for the
  bearer token and the per-channel route token; document the `None`-token dev-only posture.
- Precedent (Signal comparison study, 2026-07-09): Signal-Server compares all credential
  material via `MessageDigest.isEqual` (constant-time) — `SaltedTokenHash` /
  `UnidentifiedAccessUtil` / `HmacUtils`; it also derives time-limited downstream credentials
  via HMAC (`ExternalServiceCredentialsGenerator`) so services store no long-term secrets —
  a candidate pattern for relay route tokens.
- Proof gap: no test asserts constant-time comparison for the server token paths.
- Cross-repo note: **qsl-server**, NOT this repo. Driving queue/operator is TBD (operator
  to confirm whether this NA queue drives qsl-server).
- Recommended directive shape: small source/test lane in qsl-server.

### ENG-0015 — Suite-2 header trial-decryption is not constant-time (ordering leak)
- Severity: P3 (impl-attack; timing)
- Status: open — filed NA-0617 (D-1230) from the Suite-2 review (H-2); last-updated
  2026-07-07
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`
  `recv_nonboundary_ooo` (fixed-priority candidate order, returns on first AEAD success).
- Why it matters: the number of AEAD `open()` attempts — and thus processing time — depends
  on which bucket the true `header_n` falls into, leaking coarse ordering/gap info to a
  local timing observer. NA-0611's sweep scoped qsc secret compares, not this refimpl loop.
- Minimal fix direction: run a constant number of AEAD attempts regardless of early
  success, OR document an accepted residual bounded by network jitter.
- Proof gap: no test bounds the attempt-count variance across header positions.
- Recommended directive shape: source/test normalization or a documented residual decision.

### ENG-0016 — Suite-2 skip-window key-derivation amplification
- Severity: P3 (bounded DoS amplification)
- Status: open — filed NA-0617 (D-1230) from the Suite-2 review (M-1); last-updated
  2026-07-07
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`
  `recv_nonboundary_ooo` skip loop (`MAX_SKIP = 1000`).
- Why it matters: one crafted message from an authenticated peer forces up to ~3000 KMAC
  ops (3 per skipped counter), repeatable per message.
- Minimal fix direction: per-peer skip-rate limit, or reassess whether `MAX_SKIP = 1000`
  exceeds real need.
- Proof gap: no test bounds forced KMAC work per inbound message.
- Recommended directive shape: small source/test lane; low priority.

### ENG-0017 — Pre-1.0 PQ crates and ml-dsa version skew in the interop actor
- Severity: P3 (dependency maturity / interop hygiene)
- Status: open — filed NA-0617 (D-1230) from the Suite-2 review (M-2); last-updated
  2026-07-07
- Exact surfaces: `Cargo.lock` (`ml-kem 0.2.1`, `ml-dsa 0.1.0-rc.7` in the client; a
  second `ml-dsa 0.0.4` pulled only by `tools/actors/refimpl_actor_rs`).
- Why it matters: any "quantum-secure" claim rests on pre-1.0 PQ crates; and the
  interop/vector actor generates against a different ML-DSA draft (`0.0.4`) than the client
  uses (`0.1.0-rc.7`), a quiet version-skew footgun for conformance vectors.
- Minimal fix direction: caveat PQ-maturity in claims; align the interop actor's ML-DSA
  version with the client; monitor upstream 1.0 releases.
- Proof gap: no CI check flags actor-vs-client PQ-crate version divergence.
- Recommended directive shape: dependency/docs lane (touches Cargo — full ritual, its own
  authorization).

### ENG-0018 — Legacy plaintext-session migration deletion not verified
- Severity: P3 (secret-at-rest hygiene)
- Status: open — filed NA-0617 (D-1230) from the Suite-2 review (LOW note); last-updated
  2026-07-07
- Exact surfaces: `qsl/qsl-client/qsc/src/protocol_state/mod.rs:338` (reads an old
  plaintext session blob and re-encrypts via the vault).
- Why it matters: the encrypted-at-rest path is otherwise sound (all three snapshot paths
  route through the vault), but the legacy migration should provably delete the plaintext
  original so no unencrypted session snapshot lingers.
- Minimal fix direction: confirm/assert deletion of the plaintext source after migration.
- Proof gap: no test asserts the pre-encryption plaintext blob is removed post-migration.
- Recommended directive shape: small audit + deletion-assertion lane.

### ENG-0019 — `qsp::handshake` X3DH skeleton is auth-unsafe dead code
- Severity: P3 (latent; unreachable in the shipped client, so NOT currently exploitable)
- Status: open — filed D-1231 from the Comprehensive Audit (H-4); last-updated 2026-07-07
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`
  (`responder_process` ~:216; `InitiatorState.pq_rcv_a_priv` ~:196/:206).
- Claim at stake: peer authentication of any deployment that wires in this skeleton.
- Why it matters: `responder_process` defers KT identity verification to the caller ("expects
  the caller to have performed KT pinning for A out-of-band"), and `pq_rcv_a_priv` is left
  `Vec::new()` ("not populated here") so the initiator cannot decapsulate ct3 — the skeleton
  cannot complete and, if wired into a real deployment, peer authentication would be
  MITM-able. The shipped `qsc` client uses its own `QSC.HS.*` handshake, so this is NOT
  reachable today; the risk is an integrator mistaking this plausibly-named `QSP4.3` code for
  the production handshake.
- Minimal fix direction: feature-gate / mark non-functional / remove the skeleton so it
  cannot be mistaken for the real path; OR complete it (KT verification inside
  `responder_process`; retain `pq_rcv_a_priv`) with conformance tests.
- Proof gap: no test asserts the skeleton is unreachable/gated.
- Recommended directive shape: small source lane (gate/remove). Recommended near-term
  despite P3 severity — the fix is cheap and it aligns with the "eliminate attack surfaces by
  construction" design tenet (PROJECT_CHARTER).

### ENG-0020 — Attachment Merkle root duplicates the last node on odd levels (malleability shape)
- Severity: P3 (defense-in-depth; doubly mitigated — not currently exploitable)
- Status: open — filed D-1231 from the Comprehensive Audit (L-1); last-updated 2026-07-07
- Exact surfaces: `qsl/qsl-client/qsc/src/attachments/mod.rs` `attachment_merkle_root`
  (~:249) AND qsl-attachments `sha512_merkle_root` (both duplicate the last node on odd
  levels — the CVE-2012-2459 shape).
- Claim at stake: attachment integrity-root uniqueness (no two distinct part-lists → one root).
- Why it matters: classic Merkle odd-duplication malleability. Doubly mitigated here: each
  leaf binds its index + length (`0x00` prefix), and the protocol independently binds
  `part_count` (commit validates `parts.len() == part_count`), so the standard substitution
  attack is already neutralized. Recorded as defense-in-depth hardening, not a live exploit.
- Minimal fix direction: reject odd duplication, or bind the total leaf count into the root.
  NOTE: the root is a shared wire-level integrity commitment, so this is a COORDINATED
  cross-repo change — the qsc client and the qsl-attachments service must change identically.
- Proof gap: no test rejects the odd-duplication collision shape.
- Recommended directive shape: coordinated qsc + qsl-attachments source lane; low priority.

### ENG-0021 — `hash_secret` is unsalted SHA-512
- Severity: P3 (latent; current callers are high-entropy, so NOT currently exploitable)
- Status: open — filed D-1231 from the Comprehensive Audit (L-2); last-updated 2026-07-07
- Exact surfaces: qsl-attachments `hash_secret` (`SHA-512(secret)`, unsalted), called on
  `resume_token` and `fetch_capability`; the qsl-server has an analogous pattern.
- Claim at stake: resistance to brute-force of any low-entropy value passed to the hash.
- Why it matters: unsalted SHA-512 is cheap to brute-force for low-entropy inputs. Current
  callers pass only high-entropy random tokens (`random_token(18)`/`random_token(32)`), so
  there is no realized risk; the concern is purely latent (a future low-entropy caller).
- Minimal fix direction: add/enforce a caller-invariant that only high-entropy random tokens
  are hashed (assertion + doc); route through a salted KDF only if a low-entropy caller is
  ever introduced.
- Proof gap: no invariant/test constrains what may be passed to `hash_secret`.
- Precedent (Signal comparison study, 2026-07-09): Signal-Server stores credentials only as
  `SaltedTokenHash` (salt + hash, constant-time verify) — the reference shape if a salted KDF
  path is ever needed here.
- Cross-repo note: qsl-attachments (+ qsl-server); driving queue TBD. Low priority.
- Recommended directive shape: small caller-invariant/docs lane; low priority.

### ENG-0022 — DH-boundary cadence is an observable metadata distinguisher (G5)
- Severity: P3 (metadata; no confidentiality/integrity impact) — filed D-1239 from the NA-0622
  (ENG-0012 Stage 1b-ii) metadata decision
- Problem: with ratchet-on-reply, a Suite-2 DH boundary (FLAG_BOUNDARY + a fresh on-wire DH_pub)
  is observable and correlates with conversation turn-taking; these are the first boundary
  messages on the wire (PQ-reseed boundaries are Stage 2). The NA-0622 operator decision was
  ACCEPT + DOCUMENT (the leak is minor beyond what message timing/direction already exposes, and
  the bounded fallback prevents long silent gaps); the observable is recorded in DOC-G5-004.
- Recommended change: boundary-cadence obfuscation / cover traffic to blur the reply-correlation —
  e.g., decouple some ratchets from replies, or emit occasional cover boundaries. This is a
  protocol-wide G5 decision best made AFTER Stage 2 (PQ reseed) lands, alongside a holistic
  metadata pass; premature to bolt onto the ratchet lane.
- Recommended directive shape: G5 design lane (DOC-G5-004/DOC-G5-005 family) + a scoped
  qsc/refimpl source lane; sequence after ENG-0012 Stage 2. Deferred (consciously), tracked here.

### ENG-0023 — Spec-alignment: PQ-CTXT boundary header under HK (not §8.5.1 NHK) + no authenticated ADV receive path
- Severity: P2 (spec deviation + an unauthenticated control-plane input; bounded, no classical
  confidentiality/integrity impact) — filed 2026-07-08 from the NA-0623 deviation note (D-1241,
  Operator Decision 5 at D561) and the NA-0624 flagged deviation (D-1243)
- Problem: two header-authentication gaps live in the same frozen-receiver work area. (1) The
  frozen PQ-CTXT boundary receiver (`recv_boundary_in_order`) opens the boundary header under the
  ordinary `HK_r`, not the §8.5.1 `NHK` anti-spoof rule; the Stage-2a sender mirrors `HK_s` so the
  round-trip holds, but the deviation is normative. (2) The frozen receiver has no ADV receive
  path at all (`recv_wire` rejects `FLAG_PQ_ADV`), so the NA-0624 qsc wiring can validate an
  incoming advertisement only by length + monotonicity (`track_peer_adv`) — ADV TRACKING IS
  UNAUTHENTICATED. A relay-inbox injector can plant an advertisement: a reseed to a planted key
  still mixes into `RK` via `KDF_RK_PQ` (classical security unaffected; the PQ layer degrades at
  worst to "no reseed"), and a max-`adv_id` injection is a tracking DoS for future advertisements.
- Recommended change: reconcile the PQ-CTXT boundary header to `NHK` AND add an authenticated ADV
  receive path in one lane — both need the same receiver-semantics change + conformance-vector
  regeneration, and the qsc intercept then upgrades to authenticated tracking. Design options for
  the ADV path (resolve at design-lock): (a) header trial-open under the receive header key with
  the `pq_bind` AD; (b) SPQR precedent (Signal's production PQ ratchet) — a dedicated
  control-plane MAC under a session-derived `auth_key` (e.g. KMAC over the ADV bytes keyed from
  `RK`), which avoids trial-decryption entirely and also lets the receiver CONSUME the ADV chain
  slot in-order, retiring the NA-0624 ADV/reseed pack-exclusion rule and the mkskipped growth.
- Recommended directive shape: a delicate refimpl+qsc source lane (frozen-receiver semantics
  change; regenerate byte-pinned vectors; runtime-equivalence must still pass); the leading
  successor candidate at the NA-0624 closeout triage. Tees up the independent DH+PQ composition
  analysis the standing claim boundary requires. last-updated 2026-07-08

### ENG-0024 — Root-key duality: `RK` stored redundantly in `recv.rk` and `dh.rk` with caller-owned coherence
- Severity: P2 (architecture debt with a demonstrated desync failure class; currently mitigated
  caller-side) — filed 2026-07-08 from the NA-0624 findings (D-1243)
- Problem: DOC-CAN-003 §8.1 defines ONE session root, but `Suite2SessionState` stores it twice
  (`recv.rk`, read by the PQ path; `dh.rk`, read/advanced by the DH ratchet) and keeping them
  coherent is a CALLER obligation nothing type-enforces. This duality caused the D560 amendment
  (a DH reply wiped the PQ hardening) and the NA-0624 dh.rk-sync desync (a DH boundary advances
  only `dh.rk`, so a following reseed derived `KDF_RK_PQ` from different roots on the two
  parties); qsc now compensates with an inject-before/adopt-after dance at the CTXT receive,
  regression-pinned by the scka_e2e vectors.
- Recommended change: unify to one canonical `RK` slot in the session state (the sub-states read
  it; no redundant copies), retiring the inject/adopt obligations. Requires a QS2S snapshot
  format migration — sequence opportunistically with the NEXT lane that already needs a snapshot
  bump (pre-1.0: eliminate, do not carry, per the PROJECT_CHARTER design tenet).
- Recommended directive shape: refimpl state-model lane with a snapshot migration + full vector
  regeneration; pairs naturally with ENG-0023 (same frozen-surface unfreeze). last-updated 2026-07-08

### ENG-0025 — qsc session façade: seam obligations are scattered across the message path
- Severity: P3 (maintainability/assurance debt; all current obligations are regression-pinned)
  — filed 2026-07-08 from the NA-0624 findings (D-1243)
- Problem: the qsc↔refimpl seam carries an informal contract list enforced only by convention and
  tests: inject the canonical root before a CTXT receive and adopt it after (ENG-0024), never pair
  an ADV with a reseed in one pack (the control chain-slot / strict-in-order reseed interaction),
  persist the trigger on EVERY send path (the NA-0622 gap sat dormant on main because one of five
  store call-sites used the trigger-preserving variant), and preserve the SCKA section on every
  store. The persistence choreography also performs several AEAD decrypt passes per message
  (session + trigger + SCKA loads, plus read-modify-write stores).
- Recommended change: a single qsc session façade owning load→mutate→store for (snapshot, trigger,
  SCKA, monotonic record) with one decrypt/encrypt cycle and the coherence rules in one place;
  `qsp_pack`/`qsp_unpack` become pure policy over it. Also a natural home for extracting the
  protocol path out of the 3k-line `main.rs`.
- Recommended directive shape: a qsc-only refactor lane (no wire/crypto change; the full suite +
  runtime-equivalence are the safety net). last-updated 2026-07-08

### ENG-0026 — Combined DH+PQ boundary (single-message hybrid ratchet) in the refimpl receiver
- Severity: P3 (optimization/spec-recommended shape; the PQ-only reseed composition is proven)
  — filed 2026-07-08 from Operator Decision 1 at D561 (D-1243)
- Problem: DOC-G5-008 §4 recommends PQ reseeds RIDE ON DH boundaries (one combined boundary
  applying `KDF_RK_DH` + `KDF_RK_PQ`), but the frozen receiver has no combined path, so NA-0624
  ships PQ-only reseeds co-scheduled AFTER DH boundaries (two wire messages where one could do,
  with the ADV/reseed pack-exclusion rule as a consequence of the split).
- Recommended change: a combined DH+PQ boundary receive path (and sender mirror) in refimpl,
  collapsing the reseed into the boundary message and simplifying the qsc cadence policy.
- Recommended directive shape: refimpl lane with new conformance vectors; sequence after (or
  with) ENG-0023/ENG-0024 since it touches the same receiver surface. last-updated 2026-07-08

### ENG-0027 — Chunked / erasure-coded PQ control-plane transport (SPQR-style) with an always-progress state machine
- Severity: P3 (robustness + metadata; supersedes part of ENG-0022's scope) — filed 2026-07-09
  from the operator-directed Signal comparison study at the NA-0624 closeout (D-1244)
- Problem: our SCKA control plane ships MONOLITHIC envelopes (~1184 B FLAG_PQ_ADV, ~1088 B
  FLAG_PQ_CTXT). Consequences accepted at NA-0624: a lost/dropped ADV or reseed degrades to the
  classical status quo until the T_pq rotation; PQ control messages are size-distinguishable on
  the wire (the DOC-G5-004 §3.1 observable); cadence has idle gaps. Signal's production PQ
  ratchet (SPQR, signalapp/SparsePostQuantumRatchet) instead ERASURE-CODES the ML-KEM key and
  ciphertext into small chunks piggybacked on EVERY message header — any sufficient subset
  reconstructs, so an attacker must drop ALL traffic to suppress an epoch (loss-suppression
  becomes full DoS), per-message overhead is near-uniform (the distinguisher shrinks toward
  timing-only), and an explicit per-epoch state machine (SendingEK/ReceivingCT analogues) keeps
  both parties always making progress.
- Recommended change: a chunked PQ-transport design for the SCKA plane — polynomial/erasure
  encoding of ADV pubkeys + reseed ciphertexts across ratchet-message headers, an epoch state
  machine replacing the timer-only cadence, and (per SPQR's `SecretOutput::{Send,Recv}` shape)
  an API that tells the caller which chain the epoch secret mixes into. Wire-format change —
  a major design lane (DOC-CAN-004 §3 revision + refimpl + qsc + vectors), NOT a bolt-on.
- Recommended directive shape: a design lane first (DOC-G5-008/DOC-CAN-004 family, folding in
  what remains of ENG-0022's cadence-obfuscation scope), then staged implementation lanes;
  sequence after ENG-0023 (the frozen-receiver unfreeze it depends on). last-updated 2026-07-09

### ENG-0028 — ProVerif model of the DH+PQ composition (+ root-composition slice for the bounded explorer)
- Severity: P2 (assurance; the standing claim boundary REQUIRES independent analysis of the
  DH+PQ composition before any post-quantum claim) — filed 2026-07-09 from the Signal comparison
  study at the NA-0624 closeout (D-1244)
- Problem: `formal/` covers the SCKA LOGIC invariants (monotonicity/one-time/no-mutation-on-
  reject) but NOT the root-composition layer (recv.rk/dh.rk coherence, KDF_RK_PQ convergence,
  trigger cadence) — exactly where all four NA-0624 findings lived; the dh.rk-sync desync would
  have been caught pre-implementation by a two-party model asserting root convergence. Signal
  modeled SPQR in ProVerif BEFORE implementation and runs continuous machine-checked proofs
  (hax→F*) in CI.
- Recommended change: (1) near-term — extend the bounded Python explorer with a two-party
  root-composition slice over {DH boundary, PQ reseed, ADV} events asserting root convergence +
  PCS/healing properties (guards the ENG-0023/0024 receiver changes); (2) the substantive lane —
  a ProVerif model of the Suite-2 DH+PQ composition (secrecy + healing under compromise),
  which doubles as the on-ramp for the independent analysis the claim boundary demands.
- Recommended directive shape: a formal/ lane (G4) — the bounded-explorer slice is LITE-adjacent;
  the ProVerif model is its own full lane, ideally sequenced alongside ENG-0023 so the model
  covers the NHK-corrected receiver. last-updated 2026-07-09

### ENG-0029 — Evaluate migrating ML-KEM to a formally verified implementation (libcrux-ml-kem)
- Severity: P3 (assurance hardening; no known defect in the current dependency) — filed
  2026-07-09 from the Signal comparison study at the NA-0624 closeout (D-1244)
- Problem: we use the RustCrypto `ml-kem` crate; Signal's libsignal uses Cryspen's
  `libcrux-ml-kem`, whose ML-KEM implementation carries machine-checked functional-correctness
  and secret-independence proofs. Our KEM sits under every PQ epoch secret.
- Recommended change: an evaluation lane — API/feature fit (encap/decap/keygen surfaces used by
  `PqKem768` + `runtime_pq_kem_keypair`), maturity/audit trail, build/lockfile impact, and a
  byte-compatibility check against the existing SCKA-KEM conformance vectors; migrate only if
  the evaluation is clean (dependency mutation requires its own operator-approved lane under
  the standing rules).
- Recommended directive shape: a bounded dependency-evaluation lane (read/evaluate + report,
  then a migration lane on operator approval). last-updated 2026-07-09

---

## Workflow / process items

### WF-0001 — Cross-lane continuity requires an in-repo ledger
- Status: done — lane NA-0609A (D-1211); last-updated 2026-07-06
- Problem: each lane is a fresh assistant in a different qwork workspace path, so
  per-session assistant memory does not propagate; findings/recommendations were
  lost or buried in append-only journal narrative.
- Recommended change: an in-repo, committed, triageable ledger (this document)
  wired into the mandatory reads. Resolved by NA-0609A.

### WF-0002 — Reduce micro-lane ceremony without weakening rails
- Status: done — lane NA-0609A (D-1211); last-updated 2026-07-06
- Problem: the queue advanced through many high-ceremony micro-lanes; the cost is
  per-lane ritual, not lane count.
- Recommended change: define a WAVE lane class (one directive, several bounded
  sub-items, shared evidence) and a LITE-CEREMONY class (single PR/decision for
  genuinely low-risk docs/process/read-only-audit work only), with a hard
  fail-closed boundary excluding anything touching protocol/wire/crypto/auth/
  state-machine/security/dependencies/lockfiles/workflows/branch-protection/
  public-safety/runtime. Resolved by NA-0609A in `docs/ops/DIRECTOR_OPERATIONS.md`.

### WF-0003 — Director must triage the ledger and roadmap gates each turn
- Status: done — lane NA-0609A (D-1211); last-updated 2026-07-06
- Problem: no explicit obligation tied successor selection to the ledger or the
  DOC-PROG-001 release gates, risking queue drift.
- Recommended change: a Director triage discipline (in `docs/ops/DIRECTOR_OPERATIONS.md`)
  requiring each Director turn to read this ledger and the DOC-PROG-001 gates and
  justify successor selection against them. Resolved by NA-0609A.

### WF-0004 — Consecutive lanes in the same NA workspace get a stale qwork proof
- Status: open — originating lane NA-0609C (D-1214); last-updated 2026-07-06
- Problem: when a second lane reuses an existing NA workspace, re-running the
  operator startup gate (`qwork`) returns the cached proof from the earlier run
  rather than regenerating it, so the proof `head`/timestamp name a superseded
  commit while live `origin/main` has advanced. Observed twice in one session
  (before NA-0609B and before NA-0609C); each time the executor caught it fail-
  closed by verifying the proof against live state and stopped.
- Recommended change: for a new lane after any merge in the same NA workspace,
  drop the disposable checkout before re-running the startup gate
  (`drop_checkout.sh <lane> <repo>` then the startup gate), so a fresh checkout at
  current `origin/main` with a fresh proof is minted. Add a one-line note to
  `docs/ops/DIRECTOR_OPERATIONS.md` §5 (verified-state) capturing this, and/or a
  startup-gate enhancement to refresh an existing checkout's proof.
- Recommended directive shape: docs/process (a LITE lane) to add the runbook note.

### WF-0005 — Audits must check for cfg-gated alternate definitions before calling a function a no-op
- Status: done — lane NA-0609D (D-1216); last-updated 2026-07-06
- Problem: the NA-0609B audit reported ENG-0004 (a directory-fsync no-op) as a
  finding by grepping and seeing only the `#[cfg(not(unix))]` stub of
  `fsync_dir_best_effort`; it missed the `#[cfg(unix)]` variant that does the real
  fsync. The finding was a false positive (see ENG-0004 correction).
- Recommended change: when a read-only audit concludes a function is a no-op or
  stub, first grep for all definitions of that symbol (including `#[cfg(...)]`,
  `#[cfg(not(...))]`, target-gated, and feature-gated variants) and read the one
  that applies to the deployment target before recording a finding. Lesson
  recorded here for future audit lanes (DOC-AUD-001 methodology); resolved by
  NA-0609D re-verification and this note.

### WF-0006 — Operator startup wrapper failed silently; qnext helper added
- Type: workflow; Status: resolved (operator-applied); recorded NA-0615 (D-1226)
- Problem: the sourced startup wrapper captured the startup script's output then
  returned on failure without printing it, so a wrong-lane startup (e.g. requesting a
  DONE lane) failed silently with no diagnostic.
- Change (operator infra, outside repo): the wrapper now prints the failure output to
  stderr and returns the code; and a `qnext <current-lane> <repo>` helper derives the
  new sole-READY lane from the queue and runs drop-then-startup for it, so the operator
  never re-types the (changed) lane number. Refines the WF-0004 drop-first workflow.
- Residual: none material. Recorded so future lanes rely on the fail-visible behavior.

### WF-0007 — gov-append helper for anchor-free governance appends (with a limitation)
- Type: workflow; Status: partial (operator-applied); recorded NA-0615 (D-1226)
- Problem: governance appends done via the Edit tool require a unique last-line anchor,
  which is fragile (duplicate-match errors).
- Change (operator infra): a `gov-append <file>` helper appends stdin verbatim (no
  anchor). Use it for governance appends.
- Limitation: `gov-append` is invoked via the Bash tool, and the PreToolUse guardrail
  hook scans Bash text; standard governance boilerplate contains operator-startup words,
  which (before WF-0008) blocked the Bash call. After the WF-0008 narrowing, gov-append
  works when the text has no such word in command position; when in doubt use Write/Edit.

### WF-0008 — Guardrail hook over-broad word-matching narrowed to command position
- Type: workflow; Status: resolved (operator-applied); recorded NA-0615 (D-1226)
- Problem: the PreToolUse guardrail hook matched operator-only/privileged command names
  (startup commands, sudo, systemctl, firewall, package managers) anywhere in the Bash
  text, so prose/attestations/PR-bodies/heredocs merely mentioning those words were
  false-blocked (a specific instance of the known blunt-word-matching issue).
- Change (operator-amended hook, outside repo; the executor must never edit the hook):
  the matchers now require a real command boundary (line start, `;`/`&`/`|`/`(`/backtick)
  with an optional path/`source` prefix, instead of "any whitespace." Verified with
  three test harnesses (regex + against the installed hook + live tool calls): all real
  invocations still block (and several forms the old regex missed now block too), and
  prose is allowed.
- Residual (accepted): contrived indirect execution (`eval "..."`, `xargs`, `doas`/`env`
  prefixes) is not caught; those are deliberate-evasion forms, and the settings.json deny
  rules remain as the second defense layer. The hook is a defense-in-depth aid.

### WF-0009 — Docs-only CI path-filter (deferred to its own authorized workflow lane)
- Type: workflow; Status: open (deferred); recorded NA-0615 (D-1226)
- Problem: docs-only PRs run the full CI suite (qsc-adversarial, ci-4a..4d, CodeQL),
  costing minutes and bounded-poll cycles per lane.
- Proposed change: a `.github` path-filter so documentation-only PRs run only
  public-safety/advisories/goal-lint/link-check. This mutates workflows and interacts
  with branch-protection required checks, so it needs explicit lane authorization and the
  full two-PR ritual — NOT a docs/LITE lane. Filed for prioritization.

### WF-0010 — No reachability/liveness audit class (spec-mandated transitions can ship unreachable)
- Type: workflow; Status: open — filed NA-0617 (D-1230) from the external Suite-2 review
- Problem: the audit program has deep parse/reject/state-machine coverage but no class that
  asks "is every spec-mandated state transition actually reachable from the real client
  entry points?" The Suite-2 send-side ratchet gap (ENG-0012: DH ratchet and PQ reseed
  unreachable from the real send path) coexisted for months with hundreds of green evidence
  docs and was surfaced only by an external code review, not by the in-repo audit trail.
- Recommended change: add a DOC-AUD-001 reachability/liveness audit class that, for each
  spec-mandated transition, traces a path from a real client entry point to that transition
  (or records it as deliberately unimplemented). Require it before any "implemented" or
  release-gate (DOC-PROG-001 G1/G2) claim for a protocol feature.
- Recommended directive shape: docs/process (add the audit class to DOC-AUD-001 and wire it
  into the Director triage + release-gate checklist).

### WF-0011 — Split the DONE archive out of NEXT_ACTIONS.md (CI-script-aware)
- Type: workflow; Status: open — filed D-1231 (queue-header restructure)
- Problem: `NEXT_ACTIONS.md` is ~34k lines because it holds every completed lane block. The
  live queue is now surfaced by a `## LIVE QUEUE` header (D-1231), but the DONE blocks still
  bloat the file. A physical split into a live file + `docs/ops/NA_ARCHIVE.md` was deferred
  because three in-repo CI scripts read DONE blocks from `NEXT_ACTIONS.md` and would break:
  `scripts/ci/post_merge_verify.sh` (asserts the merged lane's `Status: DONE` block is in
  NEXT_ACTIONS.md), `scripts/ci/qsl_director_state_index.py` (computes `last_done` from DONE
  blocks), and `scripts/ci/public_safety_gate.py` (reads NEXT_ACTIONS.md content). The READY
  parser (`scripts/ci/qsl_evidence_helper.py queue`) is already tolerant (filters by
  `Status: READY`).
- Recommended change: a dedicated lane that (1) moves all `Status: DONE` lane blocks to
  `docs/ops/NA_ARCHIVE.md` (append-only), (2) updates the three CI scripts to read the archive
  where they currently read DONE blocks (or relaxes the post-merge DONE assertion to check
  TRACEABILITY.md, which already records every closeout), and (3) verifies the full CI gate
  set green before/after on a no-op lane. Keep exactly one `Status: READY` in the live file.
- Recommended directive shape: source/CI lane (touches CI scripts) — full ritual, its own
  authorization; NOT a docs/LITE lane. Medium priority (readability/maintainability).

### WF-0012 — Structured, tool-backed findings ledger (`ledger.py`)
- Type: workflow; Status: open — proposed at NA-0619 (design), recorded D-1236
- Problem: triage of the flat markdown ledger is manual and O(n); severity/status/repo live in
  prose (not filterable); no dedup on intake (two external audits re-reported known findings and
  had to be hand-cross-mapped); IDs/fields/status integrity is hand-maintained; ON DECK can drift
  from the ledger.
- Recommended change: keep the markdown, add one machine-readable `@meta sev=… status=… repo=…
  related=… updated=…` line per entry, plus a `scripts/ci/ledger.py` tool (mirroring the existing
  `qsl_evidence_helper.py queue`): `list` (filter/sort), `validate` (CI gate on monotonic/unique
  IDs, required fields, valid status, live cross-refs), `dedup` (fuzzy-match a new finding against
  existing by surface/keyword — the audit-intake killer feature), `ondeck` (generate the ON DECK
  view from the ledger), `new` (scaffold the next ID). Backfill `@meta` on existing entries.
  Optional follow-up: a CI check that the LIVE QUEUE ON DECK equals `ledger ondeck`.
- Recommended directive shape: small source (`scripts/`) + docs lane; full ritual (touches CI
  preflight). Pays for itself the next time an audit lands.

### WF-0013 — Build the full workspace (`--workspace --all-targets`) before pushing a shared-struct change
- Type: workflow; Status: open — recorded D-1236 from the NA-0620 recovered failure
- Problem: NA-0620 added a field to `Suite2SessionState`; local validation built only
  `-p quantumshield_refimpl -p qsc`, which missed two direct-construction sites in
  `tools/actors/refimpl_actor_rs` (a workspace member CI builds with `--all-targets`). The first
  CI run failed the ci-4*/demo/metadata build checks; a corrective commit added the field and it
  went green. No bad merge resulted, but a CI cycle was wasted.
- Recommended change: when a change adds/removes a field on, or changes the signature of, a
  shared type or a widely-used function, run `cargo build --workspace --all-targets` (and, where
  cheap, `cargo test --workspace`) locally BEFORE pushing — not just the directly-edited crates.
  Add this to the executor's Phase-5 build-gate checklist in `docs/ops/DIRECTOR_OPERATIONS.md`.
- Recommended directive shape: docs/process (a LITE note in DOC-OPS-006), or fold into the next
  source lane's checklist.
