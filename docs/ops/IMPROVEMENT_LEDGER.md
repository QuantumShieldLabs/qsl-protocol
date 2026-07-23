Goals: G4 (primary), supports G1–G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-22

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

### ENG-0019 — `qsp::handshake` is auth-unsafe reference code — **PARTIALLY REMEDIATED at NA-0630 (D-1254): de-attested + labeled + CI-guarded; re-rated P2 → P3. Full retirement (b type-extraction + c Suite-1/1B conformance) OPEN at P3.**
- Severity: **P3** (lowered from P2 at NA-0630, 2026-07-10, after partial remediation: `refimpl_actor` is no longer built/attested/shipped by `release-auth.yml`, the `qsp` modules carry a `//! NOT PRODUCTION — auth-unsafe` banner, and `cargo test -p quantumshield_refimpl` — incl. the NA-0628 anti-regression scan — now runs on every code PR via the required `ci-4a` job. Residual: the code still compiles and runs in `ci-4b`/`ci-4d-dur` as the Suite-1/1B reference; full retirement (b + c below) remains OPEN.). It is NOT remotely exploitable
  and NOT reachable from the shipped `qsc` client. It is raised because the original P3 rested on
  "auth-unsafe DEAD code", and the code is **not dead**: two REQUIRED CI checks certify it green as
  the Suite-1/1B reference, and a published release attests and ships the binary that embeds it.
  Authority and provenance, not reachability, are the harm.
- Status: open — filed D-1231 (Comprehensive Audit H-4); folded into NA-0628 by D565 and **UNFOLDED
  by D565-A1.2 when its "dead code" premise was falsified**; last-updated 2026-07-10.
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`
  (`responder_process` ~:216; `InitiatorState.pq_rcv_a_priv` ~:196/:206) and `src/qsp/ratchet.rs`.
- Claim at stake: peer authentication of any deployment or integrator that wires in this skeleton.
- Why it matters (rewritten 2026-07-10): `responder_process` defers KT identity verification to the
  caller ("expects the caller to have performed KT pinning for A out-of-band"), and `pq_rcv_a_priv`
  is left `Vec::new()`, so **peer authentication would be MITM-able if wired into a real deployment**.
  The shipped `qsc` client uses its own `QSC.HS.*` handshake, so this is not reachable today. What
  changed is the *authority* the code carries:
  1. **Two REQUIRED status checks execute it on every PR.** `ci-4b` and `ci-4d-dur` drive
     `refimpl_actor`'s Suite-1/Suite-1B `handshake_init/respond/finish` ops straight into
     `qsp::handshake` + `qsp::ratchet`.
  2. **The green does not mean what a reader assumes.** The actor performs NO KT verification: it
     reconstructs the peer bundle locally from a shared deterministic seed
     (`build_prekey_bundle_for`, `main.rs:1495/:1558`) and its verifier is
     `CanonicalKtVerifier::new([], KtTimeSource::Fixed(0), true)` — an **empty pinned log**
     (`main.rs:1358/:1396`). The checks validate transcript and ratchet MECHANICS and say nothing
     about peer authentication. **ENG-0019's MITM-able finding stands, un-contradicted by the green.**
  3. **`release-auth.yml` builds, sha256s, provenance-ATTESTS (`attest-build-provenance@v3`) and
     uploads `refimpl_actor` alongside `qsc` and `qshield`** on every published release. The binary
     advertises `"suites": ["Suite-1","Suite-1B","Suite-2"]`.
  So the risk is no longer merely "an integrator might mistake this plausibly-named `QSP4.3` code for
  the production handshake" — it is that CI and the release provenance chain actively invite them to.
- **Retirement is NOT cheap** (this is why the fold failed): deleting `qsp/ratchet.rs` breaks
  `refimpl_error.rs` (`RatchetError`); gating the module breaks `kt/` (`HandshakeInit`,
  `PrekeyBundle` live in `qsp/types.rs`); and either removes the subject matter of two required
  checks. See the ENG-0034 entry for the full consumer map.
- Fix menu for the successor directive (operator chooses; none of these belong in NA-0628):
  (a) module banner `//! NOT PRODUCTION — auth-unsafe (ENG-0019)` + docs, keep compiling;
  (b) extract `RatchetError` / `HandshakeInit` / `PrekeyBundle` into a neutral module so `qsp` CAN be
      gated at all;
  (c) retire Suite-1/1B conformance — a **product** decision touching branch protection, the
      `4b`/`4d-dur` harnesses, `tests/harness/4b/actor_contract.md`, and the actor's advertised suites;
  (d) **stop shipping `refimpl_actor` in `release_artifacts/`** — one line in `release-auth.yml`.
      **Cheapest real risk reduction on the list, and independent of (a)-(c).** It is release-only, so
      it cannot affect the required PR checks. `.github/**` — needs operator authorization.
- Proof gap: no test asserts the skeleton is unreachable/gated; no test asserts the conformance actor
  performs no KT verification (it does not, by construction).
- Recommended directive shape: its own lane (D566 candidate), carrying the consumer map above. (d) can
  stand alone as a one-line LITE lane.

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
- **STATUS: DONE (NA-0625; D-1245 impl, D-1246 closeout; PR #1528, merge `4b3e4fda`).** Both gaps closed. (1) The PQ-CTXT boundary header now seals
  and opens under the §8.5.1 `NHK`, derived on the fly from the pre-reseed root (receiver
  `recv_boundary_in_order` + sender mirror `send_pq_reseed`); NHK-only open, so a pre-NHK
  (HK-sealed) frame fails generically with `REJECT_S2_HDR_AUTH_FAIL`. The design-lock settled the
  crux from DOC-CAN-003 exact text (§8.5 defines a boundary as any `FLAG_BOUNDARY=1` message;
  §8.5.3 step 1 verbatim: "Require `hdr_source == CURRENT_NHK`") — the NA-0623 deviation was real.
  (2) An authenticated ADV receive path (`recv_pq_adv`, routed from `recv_wire`) binds a tracked
  advertisement to the session BEFORE it is persisted: header AEAD under the session receive
  header key, then an SPQR-style control-plane MAC
  `adv_mac = KMAC32(RK, "QSP5.0/ADVAUTH", u32be(pq_adv_id) || pq_adv_pub || [0x01])` carried as
  the first 32 bytes of the sealed body (DOC-CAN-004 §1.1/§1.3 fixes the prefix normatively, so
  the MAC cannot ride there; parse.rs took no hook). qsc's intercept fails closed: an
  unauthenticated advertisement is REJECTED, never tracked. The ADV receive consumes its chain
  slot in-order (Operator Decision 2), retiring BOTH NA-0624 workarounds — the ADV/reseed
  pack-exclusion rule and the mkskipped control-slot growth. No new primitive, no new reason code,
  no QS2S snapshot bump (NHK is derived, never stored). Two follow-ups filed below (ENG-0030,
  ENG-0031). See DOC-G5-008 (ENG-0023 note) and DOC-G5-004 §3.1 (+32B ADV observable).
- Severity: P2 (spec deviation + an unauthenticated control-plane input; bounded, no classical
  confidentiality/integrity impact) — filed 2026-07-08 from the NA-0623 deviation note (D-1241,
  Operator Decision 5 at D561) and the NA-0624 flagged deviation (D-1243); RESOLVED 2026-07-09
  (NA-0625, D-1245)
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
- **STATUS: DONE (NA-0626; D-1247 impl, D-1248 closeout; impl PR #1530, merge `fb2f1c21`).**
  `Suite2SessionState` now carries exactly ONE `rk` (DOC-CAN-003 §8.1); `recv.rk` and `dh.rk` are
  REMOVED and `session_root()` is deleted. The wire-level ops are root-EXPLICIT (`recv_wire` /
  `recv_wire_canon` take the root and return the possibly-advanced root in the outcome;
  `recv_pq_adv` takes it for the ADVAUTH verify); session-level fns read/write the single slot.
  The qsc INJECT/ADOPT dances became UNREPRESENTABLE (fields gone; deleted in the same
  workspace-atomic commit, compiler-enforced). QS2S bumped v2 -> v3 (root leads the layout; net
  -32 B); `restore_bytes` rejects any non-v3 version fail-closed with the DISTINCT static marker
  `unsupported suite2 snapshot version` (Operator Decision 1: no migration — diverged v2 roots
  are not soundly collapsible); qsc treats a pre-v3 stored session as UNRECOVERABLE
  (`session_unsupported_version`, nothing mutated on disk, session re-established) and the three
  dead legacy-migration branches are removed with a test each. Zero pinned vector bytes were
  invalidated (the WF-0014 byte-scan proved no vector pins QS2S bytes); exactly ONE vector JSON
  member changed (`S2-SEND-PQRESEED-ACCEPT-0001` lost the duplicate `dh_rk` output). The
  runtime-equivalence gate was restated per Operator Decision 3 (v3 state bytes; wire half
  STRENGTHENED with fixed golden SHA-256 pins). Severity/problem/history below are retained
  as filed.
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
- RE-TRIAGE (NA-0626, D-1247): the seam contract SHRANK — the root INJECT/ADOPT and the ENG-0030
  send-half refresh no longer exist (structural at the refimpl; ENG-0024/ENG-0030 DONE), and the
  ADV/reseed pack-exclusion rule was already retired at NA-0625. REMAINING scope: (1) the
  persistence choreography (multiple AEAD decrypt passes per message; one façade owning
  load->mutate->store) and the `main.rs` extraction; (2) `recv.ck_pq_send` — the wire-level ops'
  transport slot for the send-direction reseed seed. Same caller-owned-coherence CLASS as
  ENG-0024/0030 but at the wire-op level only; the session entry points moot the qsc-seam hazard
  (the seed lands directly in `send.ck_pq`), and the vectors pin it as input AND expectation, so
  removing it means another vector-touching lane — deliberately not widened into NA-0626;
  (3) the qsc combined-send CADENCE switch (send combined DH+PQ boundaries instead of PQ-only
  reseeds co-scheduled after DH boundaries): a live-behavior policy change, D561 operator-set,
  explicitly out of NA-0626's scope — decide alongside this façade lane or its own LITE lane.
  last-updated 2026-07-09

### ENG-0026 — Combined DH+PQ boundary (single-message hybrid ratchet) in the refimpl receiver
- **STATUS: DONE (NA-0626; D-1247 impl, D-1248 closeout; impl PR #1530, merge `fb2f1c21`).**
  A single `FLAG_BOUNDARY|FLAG_PQ_CTXT` (0x0006) frame carrying a FRESH `DH_pub` now applies the
  DH ratchet AND the SCKA reseed in one hybrid epoch transition: pure sender
  `send_combined_boundary` (caller-supplied keypair — vector-deterministic) and the combined arm
  of the session-level `recv_pq_reseed` (discrimination = `parsed.dh_pub != dh.dhr`; the AD binds
  `DH_pub`). Composition order design-locked DH-FIRST-THEN-PQ
  (`RK_final = KDF_RK_PQ(KDF_RK_DH(RK_pre, dh_out), ss)`; anchors §8.2 / DOC-G5-008 §4 / §3.3.6;
  the PQ-first order would clobber the §8.5.3 step-6 ct-bound seeds via §8.5.2 step 6 — pinned as
  a model counterfactual). The combined frame is n=0 of the new DH epoch under the pre-boundary
  NHK (§8.5.1); NO wire FORMAT change (byte-layout identical to the existing reseed frame; §4.3
  already carries `DH_pub`; parse.rs untouched — the D563 Decision-2 re-present clause did not
  fire); 0x0007 stays `REJECT_S2_LOCAL_UNSUPPORTED`. New vectors: `S2-SEND-COMBINED-ACCEPT-0001`
  (pinned wire) + 4 constructed receiver vectors. qsc RECEIVES combined frames via the same entry
  point; the SEND cadence stays the D561 operator-set policy (explicitly out of scope; re-triage
  with ENG-0025). Severity/problem/history below are retained as filed.
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

### ENG-0028 — ProVerif model of the DH+PQ composition (+ root-composition slice for the bounded explorer) — **CLOSED (NA-0627, D-1249/D-1250; PR #1533, merge `a43c0af2`)**
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

### ENG-0030 — Reseed RECEIVE leaves the receiver's SEND key schedule stale (caller-owned coherence)
- **STATUS: DONE (NA-0626; D-1247 impl, D-1248 closeout; impl PR #1530, merge `fb2f1c21`).**
  STRUCTURAL, as recommended: the session-level `recv_pq_reseed` (mirroring `send_pq_reseed`
  field-for-field) returns a fully updated `Suite2SessionState` INCLUDING the send half
  (`send.hk_s` from the advanced root, `send.ck_pq` from the send-direction seed), with a
  companion `recv_pq_adv_session` for a uniform ADV arm. The qsc caller-side mitigation was
  removed IN THE SAME COMMIT that landed the replacement (the duplicated root fields are gone,
  so the compiler enforces the no-window rule). The regression test
  `reseed_receiver_send_schedule_must_be_refreshed_from_advanced_root` was INVERTED
  (`assert_ne!` -> `assert_eq!`; name kept) and the model's invariant 4 is now asserted OF the
  entry point. Severity/problem/history below are retained as filed.
- Severity: P2 (a demonstrated desync class, currently mitigated caller-side in qsc) — filed
  2026-07-09 from an NA-0625 implementation finding (D-1245)
- Problem: `send_pq_reseed` writes BOTH directional header keys and the new send PQ chain into the
  SENDER's session state (§8.5.3 steps 6+7). The receive path (`recv_wire` -> `recv_boundary_in_order`)
  operates on `Suite2RecvWireState` and can only return recv-side state, so after a party RECEIVES
  a reseed its `send.hk_s` and `send.ck_pq` are still on the PRE-reseed schedule while the peer's
  receive schedule has moved. (The receiver's correct post-reseed send PQ chain is the one
  `apply_pq_reseed` derived into `recv.ck_pq_send`.) This was LATENT before NA-0625 — the
  reply-driven trigger makes any send after a receive a DH boundary, which reinitialises both —
  but an SCKA advertisement rides the CURRENT send chain as a control pre-envelope, and NA-0625's
  authenticated ADV receiver actually opens that header and body: the peer rejected the
  advertisement with `REJECT_S2_HDR_AUTH_FAIL`. Same class as the NA-0624 dh.rk-sync bug, and the
  same root cause as ENG-0024 (caller-owned coherence nothing type-enforces).
- Mitigation in place (NA-0625): qsc's CTXT intercept arm now mirrors the send half beside the
  dh.rk ADOPT (`send.hk_s := HK(new_rk, send_dir)`, `send.ck_pq := recv.ck_pq_send`). Pinned by
  `reseed_receiver_send_schedule_must_be_refreshed_from_advanced_root` (refimpl), by the
  `scka_e2e_*` proofs, and at model level by invariant 4 of
  `formal/model_suite2_root_composition_bounded.py`.
- Recommended change: make the coherence structural rather than caller-owned — a session-level
  reseed RECEIVE entry point in the refimpl (mirroring `send_pq_reseed`) that returns a fully
  updated `Suite2SessionState`, so no caller can hold half a schedule. Natural co-scope for
  ENG-0024 (RK unification) and ENG-0025 (qsc session façade); until then the qsc mitigation is
  load-bearing and must not be dropped.
- Recommended directive shape: fold into the ENG-0024 + ENG-0026 same-surface lane (the snapshot
  migration amortizes it), or ENG-0025. last-updated 2026-07-09

### ENG-0031 — DOC-CAN-003 §8.5.1 vs §8.5.4: is an ADV boundary header NHK or HK?
- **STATUS: DONE (NA-0626; D-1247 impl, D-1248 closeout; impl PR #1530, merge `fb2f1c21`;
  Operator Decision 4 at D563 selected the one-sentence clarification).** DOC-CAN-003 §8.5.1's sender bullet now scopes the NHK rule to boundary
  headers "that apply an epoch transition (DH ratchet advancement and/or an SCKA reseed event)"
  and states that an advertisement-only boundary advances no root and keeps the sender's current
  `HK_s` — matching §8.5.4's silence, §8.5.1's own receiver sentence, and the shipped
  implementation. EXACTLY ONE SENTENCE changed in `docs/canonical/**` (the bounded unfreeze D563
  granted); the NHK-flip alternative stays rejected (an ADV advances no root, so NHK confers zero
  attacker advantage, and the flip would churn the ADV vectors for nothing). The combined
  boundary (ENG-0026) is an epoch transition and is unambiguously NHK under either wording.
  Severity/problem/history below are retained as filed.
- Severity: P3 (spec text ambiguity; no implementation defect, no security delta) — filed
  2026-07-09 from the NA-0625 design-lock residual (D-1245)
- Problem: §8.5.1's SENDER sentence is unconditional over `FLAG_BOUNDARY = 1` messages ("A boundary
  message header MUST be encrypted under the sender's `NHK_s` derived from the pre-boundary `RK`"),
  which literally also covers `FLAG_PQ_ADV` boundaries. But §8.5.4 (advertisement) conspicuously
  omits the "Require `hdr_source == CURRENT_NHK`" step that §8.5.2 and §8.5.3 both state, and
  §8.5.1's RECEIVER sentence scopes itself to "a boundary **epoch transition**" — which an ADV is
  not (it advances no root). Both readings are defensible.
- Decision taken at the NA-0625 design-lock (bounded, deliberate): the ADV header stays under
  `HK`. An ADV advances no root, so HK-vs-NHK confers zero attacker advantage — both prove
  possession of a key derived from the same `RK` — and flipping `send_pq_advertise`'s header key
  was outside the lane's two named gaps. The ADV is separately authenticated by the ADVAUTH MAC
  under the root (ENG-0023).
- Recommended change: a one-line normative clarification in DOC-CAN-003 (scope §8.5.1's sender
  sentence to epoch-creating boundaries, matching §8.5.4's silence and §8.5.1's own receiver
  sentence), OR a bounded NHK flip for the ADV header riding ENG-0026. Pick one; do not leave the
  tension unrecorded in the spec.
- Recommended directive shape: a docs/canonical LITE lane (clarification), or a rider on ENG-0026.
  last-updated 2026-07-09

### ENG-0032 — apps hygiene: qsl-tui demo bypasses the session-level API; qshield-cli lint debt
- Severity: P3 (maintainability/coupling debt; zero runtime impact) — filed 2026-07-09,
  operator-directed, from the NA-0626 D-1247 reported boundary deviation and the D-1245 reported
  lint carry-over
- Problem: `apps/qsl-tui/src/demo.rs` (the NA-0051-era demo) calls the refimpl WIRE-LEVEL ops
  (`send_wire`/`recv_wire`) directly, so every internal refimpl signature change leaks into
  `apps/**` — which the standing directive boundaries FORBID. NA-0626's root-explicit `recv_wire`
  forced a three-line mechanical fallout there (reported at D-1247): the boundary, the WF-0013
  workspace-build gate, and the design-locked signature could not all hold. Separately,
  `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs:150` carries a pre-existing
  `needless_borrow` lint (clippy 1.95.0) that fails `-D warnings` workspace-wide, reported at
  D-1245 and again untouched at D-1247.
- Recommended change: ONE LITE apps-hygiene lane that (a) points the qsl-tui demo at the stable
  session-level entry points (`recv_pq_reseed`-style; they exist since NA-0626) or retires the
  demo outright per the pre-release eliminate-legacy tenet, and (b) fixes the qshield-cli lint.
  Alternatively fold (a) into the ENG-0025 façade lane (same theme: one stable seam for callers).
- Recommended directive shape: apps-only LITE lane (no refimpl/qsc change; workspace build +
  clippy are the gates). last-updated 2026-07-09

### ENG-0033 — public-safety PR gate: broken "unless" fallback (403), cancelled-vs-failed conflation, cancellable main runs
- Severity: P3 (CI availability/process; no security delta — the gate fails CLOSED) — filed
  2026-07-09, operator-directed, from the NA-0626 Phase-5 finding (D-1248 records the recovery)
- Problem (three defects, one gate): the public-ci "block relevant PRs when latest main
  public-safety is red unless the PR clears live advisories" step (1) can never take its
  documented "unless" branch — its fallback queries the branch-protection required-checks API
  with the default `GITHUB_TOKEN`, which 403s ("Resource not accessible by integration"), so the
  step errors instead of evaluating the PR's advisories; (2) treats a CANCELLED latest-main run
  the same as a FAILED one — but cancelled is indeterminate, not red: main@`842f6757`'s
  public-safety JOB was cancelled (siblings succeeded) and every subsequent PR was blocked with
  nothing actually failing; (3) main-push public-ci runs are cancellable in the first place —
  the root cause. NA-0626 recovered by re-running the cancelled MAIN-side job once (completing
  an interrupted run; no failing PR check was re-run to green), disclosed at D-1247/D-1248.
- Recommended change: (1) rewrite the fallback to read the PR's own `advisories` job result (no
  extra token scope needed) or grant a token that can read branch protection; (2) in the block
  step, distinguish `conclusion == failure` (block) from `cancelled`/`skipped` (warn + require a
  completed rerun); (3) set `concurrency: cancel-in-progress: false` for main-push public-ci so
  the gate's own precondition cannot be cancelled out from under it. RECOVERY PLAYBOOK (until
  fixed): if the gate is red with main's public-safety job `cancelled`, re-run that MAIN-side
  job (`gh run rerun <main-run-id> --failed`), then re-run the PR's public-ci run; never re-run
  a FAILING PR check to green.
- Recommended directive shape: `.github/**` is outside standard lane mutation paths — an
  operator-authorized CI LITE lane (workflow YAML + a runbook paragraph), or operator-side edit.
  last-updated 2026-07-09

### ENG-0034 — X25519 DH accepts non-contributory (low-order) peer keys: the DH output is never checked — **CLOSED at NA-0628 (impl D-1251 / PR #1536 / merge `e9439df7`; closeout D-1252). Every LIVE DH output now fails closed on the all-zero value (RFC 7748 §6.1); `REJECT_S2_DH_NONCONTRIBUTORY` registered in DOC-CAN-003; additive negative vectors + WF-0014 byte-scan; anti-regression scan. ENG-0019 was UNFOLDED (D565-A1) — see its entry.**
- Severity: P2 (security-relevant correctness gap; NOT remotely exploitable against an honest
  pair — see the exposure bound below — but it silently voids the CLASSICAL half of
  post-compromise security and therefore blocks the Triple-Ratchet/PCS claim language) — filed
  2026-07-09 from NA-0627 (ENG-0028), discharging **Operator Decision 5 of D564, option (c)**.
  Full evidence: `docs/governance/evidence/NA-0627_decision5_contributory_code_inspection.md`.
- Problem: `x25519-dalek 2.0.1`'s `StaticSecret::diffie_hellman` is deliberately
  NON-CONTRIBUTORY per RFC 7748 — a low-order peer point yields an all-zero shared secret rather
  than an error. RFC 7748 §6.1 requires protocols needing contributory behaviour to check the
  all-zero DH OUTPUT. This repo never does: `was_contributory()` is called nowhere, and
  `X25519Dh::dh` (`crypto/traits.rs:36`) returns a bare `[u8; 32]`, so the flag is discarded at
  the trait boundary. All four Suite-2 DH outputs (`ratchet.rs:1306` send_boundary, `:1475`
  recv_dh_boundary, `:1885` send_combined_boundary, `:2390` recv_combined_boundary) and the QSP
  handshake's `dh1`/`dh2` (`qsp/handshake.rs:134`, `:144`, `:285`, `:297`) feed straight into
  `KDF_RK_DH`/`derive_rk0`. The only related guard, `is_zero32(&parsed.dh_pub)`
  (`ratchet.rs:1420`, `:2317`), rejects exactly ONE of Curve25519's eight small-order encodings
  (the all-zero one); every other low-order encoding passes and drives `dh_out = [0u8; 32]`.
  Effect: `RK' = KMAC(RK, "QSP5.0/RKDH", 0…0)` — the DH ratchet contributes NO fresh entropy for
  that epoch, silently (no reject, no reason code, both parties converge, no vector observes it).
- Exposure bound (stated honestly, this is why it is P2 and not a STOP): a network/Dolev-Yao
  attacker CANNOT reach it — a boundary header is AEAD-sealed under `NHK_r` from the current root
  and `DH_pub` is bound into `ad_hdr`, so injection needs the root. NA-0627's Q1/Q2 prove that
  envelope (`is true.`); no modeled query is disproved. The AUTHENTICATED PEER (or malware
  steering its key selection) can reach it, and thereby void classical PCS: an attacker who once
  learned `RK` stays synchronized across every boundary it forces non-contributory. NA-0627's Q5
  (classical healing across a DH boundary) holds ONLY because the modeled honest sender always
  contributes a fresh exponent — exactly the property a low-order point removes. The PQ half
  still heals (Q3/Q4 hold independently), so the hybrid degrades to PQ-only healing rather than
  collapsing. Note: **the symbolic model cannot decide this question at all** (abstraction A4;
  ProVerif's DH theory idealizes the group and would return "secure" either way — the
  Decision-5 re-presentation), which is why it is answered by code inspection.
- Prior art, now closed: the 2026-04-09 incoming security audit
  (`docs/audit/incoming/2026-04-09_security_batch/…Security Audit.md:138`) stated the same fact
  for the QSP-4.3-era code and was never converted into a tracked item. This filing closes that
  gap and adds the post-compromise consequence the audit did not analyze.
- Recommended change: fail closed on a non-contributory DH. Either (a) surface
  `was_contributory()` through `X25519Dh::dh` (`Result`/`Option` return) and reject at all call
  sites — the version a new call site cannot forget; or (b) keep the trait shape and add an
  `is_zero32(&dh_out)` fail-closed check immediately after each `dh()` call, plus a small-order
  screen on `DH_pub` ingress — the smaller diff. Either needs a new reason code
  (`REJECT_S2_DH_NONCONTRIBUTORY`) and negative conformance vectors.
- **Operator direction (2026-07-09, at the NA-0627 closeout): FIX IT, as the sole READY successor
  lane (NA-0628), with its own design-lock before code.** The alternative — amending D564 to
  authorize an in-lane fix — was presented and DECLINED on the executor's recommendation: it would
  have falsified D-1249/TRACEABILITY/the NA-0627 testplan/DOC-G4-002 (each asserts "no source
  change") and landed a crypto-path change with no design-lock, no WF-0014 byte-claim vector regen,
  and no WF-0015 caller-surface enumeration. The analysis-lane rule held: **filed, not fixed.**
- **⚠ SURFACE CORRECTED 2026-07-10 (Director turn, verified read-only before D565 was drafted). The
  original filing was BOTH understated and overstated. Recorded, not silently fixed:**
  - **UNDERSTATED — the shipped client's ESTABLISHMENT DH was missing.** `qsc` does not use
    `qsp::handshake`; it has its own `QSC.HS.*` handshake, whose DH helper
    `qsl/qsl-client/qsc/src/handshake/mod.rs:801 hs_dh_shared` validates LENGTHS ONLY and returns the
    raw shared secret. Live call sites: `:1449` (initiator) and `:1877` (responder). **This is the
    establishment DH of the shipped client and the most important surface in the item.** It already
    returns `Result<[u8;32], &'static str>`, so the guard is a two-line change in ONE function.
  - **~~OVERSTATED — `qsp/**` is DEAD CODE~~ — THIS CORRECTION WAS ITSELF FALSE.** Corrected again at
    NA-0628 Phase 0 (2026-07-10), which STOPPED the lane; see D565 AMENDMENT 1 (D565-A1.1).
    `qsp::handshake` and `qsp::ratchet` have LIVE CONSUMERS. `qsp/mod.rs` flattens its submodules
    (`pub use handshake::*; pub use ratchet::*;`), so consumers write `qsp::initiator_build`, and a
    path-qualified grep for `qsp::handshake` / `qsp::ratchet` returns zero while callers exist.
    True consumer map: **`tools/actors/refimpl_actor_rs/src/main.rs`** calls `initiator_build`
    (`:1508`), `responder_process` (`:1561`), `initiator_finalize` (`:1614`), `ratchet_encrypt`
    (`:1654`), `ratchet_decrypt` (`:1718`) behind its `handshake_init/respond/finish`, `encrypt` and
    `decrypt` ops — these ARE Suite-1/Suite-1B, and **they run on EVERY pull request as the REQUIRED
    checks `ci-4b` and `ci-4d-dur`** (`ci.yml` → `scripts/ci/run_4b.sh` → `tests/harness/4b/runner.py`,
    suites `["Suite-1","Suite-1B"]`; `ci.yml` → `scripts/ci/run_4d_dur.sh` → `scripts/ci/durability_4d.py`,
    `suite = "Suite-1"`). Library-internal consumers outside the module: `src/kt/mod.rs:6` and
    `src/kt/canonical.rs:3,567` (`HandshakeInit`, `PrekeyBundle`), `src/refimpl_error.rs:4,63`
    (`RatchetError`), `src/lib.rs:13,18`. Tests: `tests/na_0071_header_key_derivation.rs`,
    `tests/kt_verifier_vectors.rs`. (The repo-root `main.rs` also calls them but is ORPHANED — the
    root `Cargo.toml` is a virtual manifest with no `[package]`, so it never compiles.)
    **Consequence: deleting or feature-gating `qsp` would break core library code
    (`refimpl_error.rs`, `kt/`) and would remove the subject matter of two REQUIRED status checks.**
    ENG-0019 was therefore **UNFOLDED from NA-0628** (D565-A1.2) and re-rated — see its entry.
    `qsc` references neither `kt` nor `qsp`, so none of this reaches the shipped client.
    **Process lesson filed as WF-0017.**
  - **CORRECTED FRAMING (NA-0628 design-lock, 2026-07-10): the four Suite-2 sites are SHIPPED-CLIENT
    paths, not merely reference-implementation paths.** `qsl/qsl-client/qsc/src/main.rs:23` imports
    the refimpl ratchet directly and calls `send_boundary` (`:2320` → `ratchet.rs:1306`),
    `recv_pq_reseed` (`:2657` → `ratchet.rs:2390` via `recv_combined_boundary`) and
    `recv_dh_boundary` (`:2683` → `ratchet.rs:1475`). This RAISES the item's importance.
  - **OVERSTATED — a "small-order ingress screen" is NOT required.** X25519 clamps the scalar to a
    multiple of 8, so any small-order peer point maps to the identity and yields an all-zero output.
    Therefore `dh_out == 0` **iff** the peer point is in the small subgroup: the all-zero OUTPUT check
    alone catches all eight low-order encodings, and it is exactly what RFC 7748 §6.1 prescribes. An
    ingress screen is optional defence-in-depth. The earlier "plus a small-order screen" phrasing is
    superseded by D565.
- **LIVE SURFACE (authoritative, as of D565):** (i) `qsc` establishment — `hs_dh_shared`
  (`qsl/qsl-client/qsc/src/handshake/mod.rs:801`), covering call sites `:1449` and `:1877`; and
  (ii) refimpl Suite-2 ratchet — `ratchet.rs:1306` `send_boundary`, `:1475` `recv_dh_boundary`,
  `:1885` `send_combined_boundary`, `:2390` `recv_combined_boundary`. Everything in `qsp/**` is dead
  and is handled by ENG-0019, not by hardening.
- **WF-0015 caller surface, enumerated before design-lock:** changing the trait `X25519Dh::dh` to
  return `Result`/`Option` would touch **7 trait impls and ~20 call sites**, INCLUDING the
  boundary-FORBIDDEN `apps/qsl-tui/src/demo.rs:378-379` — the same `apps/**` leak ENG-0032 was filed
  for. D565 therefore recommends the contained post-hoc guard PLUS a mandatory anti-regression scan
  that fails if a new `dh()` call site appears without an adjacent zero check.
- Recommended directive shape: refimpl + vectors lane (`tools/refimpl/**` suite2 + qsp, `inputs/**`
  negative vectors, a DOC-CAN-003 §8.5.2 note). Note for its design-lock: the handshake arm touches
  the `qsc` handshake caller surface, so **WF-0015's caller-surface enumeration binds regardless of
  which fix shape is chosen**, and the bundle-ingress screen is the natural home for the small-order
  check on that arm. NOT done in NA-0627: D564 is an ANALYSIS lane ("the FIX, if warranted, stays
  out of scope"). last-updated 2026-07-09

### ENG-0035 — ProVerif does not terminate on the 2-boundary unrolling of the Suite-2 composition — **OPEN; filed NA-0627 (PR #1533, merge `a43c0af2`)**
- Severity: P3 (assurance-coverage limit; no security delta — the reduced-scope model proves the
  same queries, and nothing was weakened) — filed 2026-07-09 from NA-0627 (ENG-0028), per D564
  Decision 1's standing instruction and the design-lock §6 non-termination protocol.
- Problem: the design-lock bound was "unroll 2 boundaries per direction" (abstraction A6). At
  that bound `formal/proverif/suite2_dhpq_main.pv` DOES NOT TERMINATE: with A's combined DH+PQ
  boundary following B's DH boundary, the session root carries TWO nested `exp` terms under the
  commutativity equation and ProVerif's saturation diverges (>102 000 rules inserted, no `RESULT`
  line, capped at 2400 s; a single secrecy query in isolation also diverges, so the cost is the
  PROCESS, not the query count). Raw evidence in the proof root:
  `nonterm_main_v1_full_2400s.out`, `nonterm_main_v1_q1only_2400s.out`, `nonterm_main_v1.pv.txt`.
  This is the exact risk Decision 1 recorded when ProVerif was selected over Tamarin.
- What was done instead (recorded, not silent): the main model was reduced to ONE DH boundary +
  ONE PQ reseed + both advertisements, and the reduction is documented in the model header. **No
  query text was weakened** — Q1/Q2/Q6/Q7 are stated over the full reduced schedule and all pass.
  The combined boundary is NOT unmodeled: it is verified with its own compromise scenario and its
  own guard-form query in `suite2_dhpq_q4_combined_healing.pv`, which terminates in ~1 min.
  Q3/Q4/Q5 (the healing queries, one boundary each) all terminate.
- Residual gap: no single model exercises TWO consecutive root-advancing DH epochs, so an attack
  requiring a second DH epoch would not be found. Nothing suggests one exists; the gap is stated,
  not papered over (abstraction A6, as reduced).
- Recommended change: RE-PRESENT THE TAMARIN OPTION for this query shape (D564 Decision 1's
  documented fallback). Tamarin's multiset rewriting handles unbounded ratchet state and PCS
  lemmas natively; the cost is hand-written oracles/lemmas and a much larger lane. Alternative,
  cheaper: keep ProVerif and try `set attacker = passive` variants, `nounif` hints, or an
  axiomatized `rkdh_rk` over an opaque `dh_out` type (dropping the `exp` equation and modeling
  the DH share as an abstract fresh value per epoch) — the last of which would trade the DH
  algebra for a stated abstraction and should be design-locked, not improvised.
- Recommended directive shape: an operator decision at D-1249 (accept the stated A6 reduction) +
  an optional successor formal lane if the 2-epoch unrolling is judged load-bearing.
  last-updated 2026-07-09

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

### WF-0014 — A vector-freeze scope claim MUST be verified against the vector BYTES, not a prose note
- Status: filed 2026-07-09 from the NA-0625 STOP (D-1245 / D562 Operator Decision 5)
- Problem: the NA-0625 forward study asserted "e2e_recv/interop/crash_restart embed NO reseed
  frames", and the NA-0625 design-lock §5 promoted that to "verified against live files" without
  ever decoding the pinned bytes. It was wrong: `qshield_suite2_e2e_recv_vectors_v1.json` ->
  `S2-E2E-ACCEPT-BOUNDARY-0001` pins a `flags = 0x0006 (PQ_CTXT|BOUNDARY)` frame whose header was
  sealed under `HK`. The §8.5.1 NHK change therefore invalidated a frozen vector set OUTSIDE the two
  files the directive named, which surfaced only at the Phase-4/5 merge boundary — as a STOP, after
  the whole implementation and gate stack had run — instead of at the Phase-2 design-lock, where the
  operator could have scoped the lane correctly from the start.
- Recommended change: whenever a lane's design-lock claims a set of conformance-vector files is
  unaffected, it MUST prove it by decoding every pinned byte string in `inputs/**/vectors/*.json`
  that parses as a wire envelope and reporting the frames whose flags/shape intersect the semantics
  being changed. The scan is ~30 lines of Python and runs in well under a minute; the NA-0625
  version is archived at `docs/governance/evidence/NA-0625_suite2_spec_alignment_harness.md` §8 and
  can be lifted verbatim. Add the obligation to the design-lock checklist in
  `docs/ops/DIRECTOR_OPERATIONS.md` (and to DOC-OPS-006's design-lock section): "a vector-freeze
  claim is a BYTE claim; cite the scan, not a forward-study note."
- Cheaper generalization worth considering: a `scripts/ci/scan_pinned_wire_frames.py` that any lane
  can run, and which CI could optionally assert against a checked-in inventory so that a frame's
  appearance in a new vector file is itself reviewable.
- Companion gap, same lane, same root cause (assumption instead of the real artifact/tooling): the
  executor ran all 15 suite2 vector RUNNERS locally but not `scripts/ci/validate_suite2_vectors.py`,
  so a JSON-schema violation in the 5 appended ADV-receive vectors (`input.role.data` must be an
  object, not the bare string `"A"`) reached CI instead of being caught locally. The executor's
  Phase-5 gate checklist should be derived MECHANICALLY from the workflows a change touches — i.e.
  run every `scripts/ci/*.py` invoked by the affected `.github/workflows/*.yml`, not a remembered
  subset. (`goal-lint` additionally requires a `Goals: G1, ...` line in the PR body; it cannot run
  locally, so it belongs on a PR-creation checklist.)
- Recommended directive shape: docs/process LITE lane, or fold into the next source lane's
  design-lock checklist (it costs one command). last-updated 2026-07-09

### WF-0015 — A signature/shape change's scope claim MUST enumerate its CALLER surface at design-lock
- Status: filed 2026-07-09, operator-directed, from the NA-0626 D-1247 reported boundary
  deviation (the caller-surface sibling of WF-0014's byte-claim rule)
- Problem: NA-0626's design-lock §10 boundary audit asserted "apps/** untouched", checking the
  lane's MUTATION INTENT but not the CALLER SURFACE of the design-locked signature change —
  `apps/qsl-tui`'s demo calls `recv_wire` directly, so the root-explicit signature + the binding
  WF-0013 workspace-build gate forced a three-line edit in a boundary-FORBIDDEN path, discovered
  at Phase 3 instead of Phase 2 (where the operator could have pre-authorized it in the
  directive). Same failure shape as WF-0014: a scope claim asserted from intent rather than
  from the artifact.
- Rule: a design-lock that pins a change to any public refimpl signature, public struct shape,
  or serialized format MUST verify its boundary audit MECHANICALLY against the caller surface —
  `cargo build --workspace --all-targets` (WF-0013, run at DESIGN-LOCK time against a spike or
  by grepping every caller of the changed item) and an explicit list of every crate/path the
  change forces edits in, each checked against the directive's Result boundary. A forced caller
  outside the boundary is design-lock output for the operator (pre-authorize or re-scope), not
  a Phase-3 surprise.
- Companion standing-directive suggestion (operator's call): a boundary clause distinguishing
  discretionary mutation (FORBIDDEN stays forbidden) from signature-forced mechanical compile
  fallout (bounded, reported, not a STOP) — NA-0626's D-1247 records the precedent resolution.
- Recommended directive shape: docs/process LITE (design-lock checklist edit in
  DIRECTOR_OPERATIONS/DOC-OPS-006), or fold into the next source lane's design-lock like
  WF-0014 was. last-updated 2026-07-09

### WF-0016 — Session handoff has no single artifact and no machine-checkable contract
- Severity: P2 (process/assurance; a lost or stale handoff artifact can silently drop a
  design-lock, and nothing fails closed when it does) — filed 2026-07-09 from NA-0627, at the
  operator's request after the NA-0626→NA-0627 handoff proved rough in practice.
- Problem: a handoff is currently **five artifacts with five different lifetimes**, two of them
  outside version control:
  (1) the archived directive + its appended DESIGN-LOCK CONCLUSIONS — durable, but lives in
      `/srv/qbuild/operator/directives/`, OUTSIDE git;
  (2) `docs/governance/evidence/NA-####_design_lock.md` — the single most load-bearing document
      for the incoming chat, and it is **GITIGNORED** (`.gitignore:65` `**/evidence/`). It survives
      only because the convention "commit it with `git add -f`" is itself remembered. Forget once
      and a fresh checkout silently has no design-lock;
      **⚠ THE "REMEMBERED CONVENTION" MITIGATION HAS EMPIRICALLY FAILED — MEASURED, WITH THE
      MEASUREMENT'S LIMITS STATED (added 2026-07-21 by NA-0664/D-1290).**
      `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` records this footgun **actually biting** — a new
      evidence document **hidden, skipped, rejected, or requiring `-f`** on a failed attempt — in
      **AT LEAST 31 DISTINCT PRIOR INSTANCES, each hand-verified at the line level, spanning
      NA-0245 through NA-0580.** **31 IS A FLOOR, NOT A POINT ESTIMATE, AND MUST BE CITED AS
      ONE:** successive keyword searches returned **41, 42, 175 and 35** depending on marker
      framing — loose patterns swept in unrelated recovery notes merely because the word
      "evidence" appeared on the line, narrow ones missed real variants. **The true count is
      higher than 31; the recurrence is unambiguous, its exact magnitude is not.** **It bit TWICE
      MORE in NA-0664 alone** — on the initial closeout commit and again on the amend, because
      the ignore rule applies to **every** commit carrying the file, not only the first.
      **THREE CORRECTIONS TO THIS ITEM FOLLOW.** (i) **The hazard is NOT specific to design-lock
      handoffs**, which is how this item frames it — NA-0664's instance was an **as-built**
      (`docs/governance/evidence/NA-0664_as_built.md`), and the rule catches **every** artifact
      under `evidence/`, so recommendation (a)'s "put handoffs outside `evidence/`" fixes only
      the slice this item happens to name. (ii) **A convention forgotten at least 31 recorded
      times is not a mitigation, it is a defect with a workaround** — and **⚠ THE ARGUMENT IS
      UNCHANGED BY THE NUMBER: it was never rate-dependent.** A defect recovered flawlessly
      every time generates no pressure to fix it; that holds at 31 as it did at the erroneous
      41, and it would hold at 5. **The mechanism is the SILENCE of the failure mode, not the
      frequency. A reader who sees the figure corrected downward must not discount the finding
      with it.** (iii) **⚠ THE REFRAMING THAT
      MAKES THIS CHEAPER THAN "ADD TOOLING": IF THE CONVENTION IS "ALWAYS `git add -f`", THEN THE
      IGNORE RULE IS WRONG.** A path that is **unconditionally force-added is not a path anyone
      intends to ignore.** Exactly one of two things is true and **both are one-line answers**:
      either **`.gitignore:65` (`**/evidence/`) is too broad** and wants a negation for the
      subpaths that must be tracked, or **evidence genuinely should not be tracked** and the
      standing force-add convention contradicts that intent. **This item currently frames the
      problem as REMEMBERING; it is a MISCALIBRATED RULE** — which moves the fix from "build a
      staging assertion" to "fix the line." **RECORDING ONLY — nothing about WF-0016 was
      executed in NA-0664, and `.gitignore` was NOT in this lane's scope.**
      **⚠ AMENDED 2026-07-23 by NA-0668 (D-1294) — EXTENDED, NOT REWRITTEN, because the original
      census was accurate when it was made.** Artifact (1) is recorded above as living
      *"OUTSIDE git."* **It was also OUTSIDE THE BACKUP, and nobody knew.** `/srv/qbuild/operator`
      — the entire directive corpus, plus `responses/` (575 files, 2026-05-06 → 2026-07-22),
      `relay/` (69 files), and the 2026-07-22 independent audit report — appeared in **no**
      `daily_sources` entry of `/usr/local/sbin/qsl-backup`, and was verified **absent** from
      three consecutive daily snapshots. **So "outside git" understated it: for artifact (1)
      there was exactly one copy on exactly one disk.** The spine's references to directives by
      **sha256 + line count only** (e.g. `NA-0667_as_built.md:3`) are an *integrity check, not a
      copy* — they can prove a directive was altered, and can reconstruct nothing.
      **CLOSED 2026-07-23 by D604's B0**: `/srv/qbuild/operator` and `/home/victor/work/qsl/codex`
      are now `daily_sources` entries, verified present in a post-install checkpoint (807 files).
      **The "outside version control" half of this item stands unchanged and unfixed.**
  (3) the auto-memory resume note — per-user, per-machine, not in the repo, not reviewable in a PR;
  (4) the proof root under `/srv/qbuild/tmp/NA####_...` — holds the RAW query/probe outputs, and
      `qbuild-ssd-maintenance.timer` runs nightly. The only copy of the evidence sits somewhere a
      timer is entitled to delete;
  (5) a ~1,500-character resume prompt typed by the operator by hand, carrying paths and rules.
  The workaround for (1)-(5) has been to make the directive "self-sufficient" by **duplicating**
  the design-lock conclusions into it — which creates two sources of truth that can disagree.
  **Nothing verifies any of it.** The incoming chat is told, in prose, to "re-verify Phase 0 live."
  It works when the chat is conscientious; nothing objects when it is not.
- Evidence that this is real, not theoretical (all from the NA-0627 resume, 2026-07-09): the
  auto-memory index pointed at `~/qsl-handoff-packet.md`; the operator's shell history records
  `~/work/qsl-handoff-packet.md`; **neither exists** — verified by bounded `find / -xdev`, the repo
  and its git history, and `/backup/qsl` manifests. Two recorded paths, zero files, and the
  incoming chat spent real time proving the absence. Nothing in the lane depended on it *only*
  because the directive happened to carry the duplicated conclusions.
- Recommended change: **ONE artifact, ONE path, machine-verified.**
  (a) `docs/governance/handoff/NA-####_handoff.md` — TRACKED (deliberately NOT under `evidence/`,
      so no `git add -f` footgun), generated by `scripts/ops/make_handoff.py` so every handoff has
      an identical shape. Fixed schema: lane/directive/decision identity + phase to resume at; base
      SHA with the exact commands that verify each claim; the DESIGN-LOCK CONCLUSIONS **once**
      (the directive REFERENCES them instead of copying them); decisions RE-PRESENTED and still
      owed an operator answer; a proof-root inventory **with a sha256 per file**; an explicit
      DO-NOT list; a mechanical phase checklist the incoming chat converts 1:1 into its task list
      (task lists do not cross chats); and the resume prompt emitted VERBATIM, never hand-composed.
  (b) `scripts/ops/verify_handoff.py NA-####` — read-only, FAIL-CLOSED, the incoming chat's FIRST
      Phase-0 duty. Asserts: manifest present + schema-complete; live `HEAD` == recorded base;
      worktree clean; the anchored `^Status: READY` count is exactly what the manifest declares;
      DECISIONS counters correct (successor ID absent); every proof-root file present with a
      MATCHING sha256 — so a nightly tmp sweep STOPS the lane instead of letting it proceed on
      missing evidence; and each declared tool version is invocable and matches (e.g.
      `proverif -help` -> 2.05). This converts "the incoming chat was careful" into "the gate
      refused."
  (c) Durability: copy the load-bearing raw outputs (small text files) into the tracked handoff dir
      rather than leaving the only copy in a swept tmp directory; and store REPO paths in
      auto-memory, never home-directory paths — repo paths are versioned, reviewable, and cannot
      quietly evaporate the way the packet did.
- Non-goals: this does NOT change the ONE-handoff-per-lane cap (delicate lanes only, at design-lock
  completion) recorded from the 2026-07-08 operator pushback. It does not add handoffs; it makes
  the single permitted handoff cheap and verifiable.
- Recommended directive shape: docs/process + tooling LITE lane (`docs/governance/handoff/**` +
  `scripts/ops/**` + a DOC-OPS-006/AGENTS.md section). **Must NOT be ad-hoc-edited from an
  unrelated executor lane** — that is precisely why NA-0627 filed this rather than fixing it.
  Adjacent: WF-0012 (`ledger.py`) is the same "stop hand-maintaining structured state in markdown"
  theme and could share the lane. last-updated 2026-07-09

### WF-0017 — A NEGATIVE reachability claim MUST be established by a mechanism proved able to find a POSITIVE
- Severity: P2 (process; it has already produced two wrong directives in 24 hours)
- Status: open — filed at NA-0628 Phase 0 (2026-07-10, D565-A1.6); last-updated 2026-07-10
- Rule: a claim of the form **"zero callers", "dead code", "nothing runs this", "not wired into CI"**
  must be established by a search mechanism that has been **demonstrated capable of finding a
  counterexample**. A single-pattern grep is not such a mechanism when the reference can sit one
  indirection away.
- The two errors that motivated it, both from this lane, both the same shape:
  1. **D565's Director turn** claimed `qsp::handshake` / `qsp::ratchet` had zero external callers.
     `qsp/mod.rs` re-exports flattened (`pub use handshake::*`), so every real consumer writes
     `qsp::initiator_build`. The grep searched a module path that **no caller can ever contain**:
     `grep -c 'qsp::handshake\|qsp::ratchet'` over the actor is 0 while `grep -c 'qsp::'` is 1.
  2. **NA-0628's Phase-0 executor** claimed the `4b`/`4d-dur` harnesses were "not wired into any
     current `.github` workflow", from `grep -rn 'harness/4b|durability_4d' .github/workflows/` →
     empty. The workflows call **wrapper scripts** (`scripts/ci/run_4b.sh`,
     `scripts/ci/run_4d_dur.sh`) which call the harnesses. Both are REQUIRED checks.
- Accepted mechanisms, in ascending order of decisiveness:
  1. grep the **flattened symbol names**, not the module path;
  2. read `mod.rs` for `pub use <sub>::*` **before** trusting any path-qualified grep;
  3. trace the **job → script → harness** chain rather than grepping the workflow directory;
  4. read the **required-checks list** (`gh api .../protection/required_status_checks/contexts`) and
     follow each job to what it actually executes;
  5. **decisive:** delete or `#[cfg(any())]` the item and let the **compiler enumerate the consumers**.
     A `cargo check` cannot be fooled by a re-export.
- Relationship to WF-0015: this is its **dual**. WF-0015 governs *positive* caller enumeration when a
  signature changes. WF-0017 governs *negative* claims, which are strictly harder — a positive claim
  is proved by one example; a negative claim requires proof that the search **could have found one**.
- Proof gap: no lint or checklist item forces a negative claim to name its search mechanism.
- Recommended directive shape: docs/process (DOC-OPS-006 directive template gains a "negative claims"
  box; AGENTS.md gains the mechanism list). Cheap. Pairs naturally with WF-0016.

### ENG-0036 — Token-gated relay access for a private/self-hosted deployment (closed-network authorization) — **NEW; filed 2026-07-10 (operator product direction)**
- Severity: P3 (feature / deployment-hardening; NOT a confidentiality/integrity gap in the shipped E2EE — it is an ACCESS-CONTROL layer at the relay, orthogonal to message security)
- Status: open — filed 2026-07-10 from an operator product-direction note during NA-0628; last-updated 2026-07-10
- Idea (operator, verbatim intent): the relay/server generates an access token at install/setup; only apps that hold the token AND the server address can connect. Target: a niche "run-your-own highly-secure server" product for a small trusted group, paired with the forthcoming TUI/GUI.
- **Setup-time mode toggle (operator refinement, 2026-07-10):** server setup asks *"will this be a PUBLIC accessible server or PRIVATE?"* and drives the config from the answer. PRIVATE ⇒ token-gating ON (this ENG); PUBLIC ⇒ open relay (the default Signal-like posture, no token wall). The operator judges this "codes easy enough for both" — plausible, since it is a config branch over the EXISTING `relay_auth_header` path rather than a new transport. This toggle is also the natural seam for the metadata posture: it is where a deployment decides which protections apply (see ENG-0037 sealed-sender — high-value in PUBLIC mode, nice-to-have in PRIVATE where the operator is trusted). The UX MUST make each mode's security implications explicit so neither is mistaken for the other's guarantees. **These are options to weigh WHEN we reach that point, not a committed design.**
- **Grounding — this EXTENDS existing architecture, it does not start from zero.** The client/relay already carry a route-token / relay-auth-header mechanism (`qsl/qsl-client/qsc/tests/relay_auth_header.rs`; `route-token/header discipline`; auth-token resolution in the transport subsystem; a fail-closed `relay_unauthorized` state). ENG-0036 is the formalization of that into an install-time PROVISIONED, closed-network access credential with a specified lifecycle.
- **What it buys (state honestly):** a closed relay — unauthorized clients cannot connect/enqueue; reduced spam/DoS/enumeration surface; a "private network" property analogous to a WireGuard pre-shared key or a self-hosted-server registration token. Strong fit for the self-hosted niche where the operator runs the relay.
- **What it does NOT buy (must be stated in any spec/UX so it creates no false security):** it is NOT end-to-end security — the Suite-2 E2EE already protects message confidentiality/integrity against the relay. It does NOT hide who-talks-to-whom from the relay operator (that is the sealed-sender gap, still unfiled). It is a BEARER credential: whoever holds it can connect, so distribution, rotation, and revocation are load-bearing, and a leaked token opens the network until rotated.
- **Threat-model discipline (per the metadata roadmap rule "name the adversary"):** ENG-0036 answers "outsiders connecting to my private relay." It does NOT answer "the relay operator is the adversary" (in a self-hosted deployment the operator IS the relay, so that is an accepted posture) or "a global passive adversary" (mixnet territory, out of scope). It must not be marketed as more than closed-network authorization.
- Design questions for a future directive: token generation + entropy + storage at rest on the server; provisioning/enrollment UX (QR / paste / file) into the app alongside the server address; rotation + revocation + multi-token (per-device) support; interaction with the existing `relay_auth_header` path (extend vs replace); rate-limiting / lockout; and the exact "no false security" wording for the claim matrix.
- Recommended directive shape: a design-lock-first lane (threat model + token lifecycle spec before code), sequenced AFTER the crypto core is at its completion point and alongside the TUI/GUI work it serves. Cross-repo (qsl-server + qsc client).

### WF-0018 — Strategic/program/review-facing docs drift behind live truth — **DONE at NA-0629 (D-1253, 2026-07-10); directive D566**
- Severity: P2 (process/assurance; the external-review package understates the project's own evidence, and stale "current posture" is where a claim can silently move)
- Status: **DONE at NA-0629 (D-1253, PR #1539, merge `6809906d`, 2026-07-10)** — directive `QSL-DIR-2026-07-10-566` (D566) executed; the strategic/program/public-review docs were refreshed to live truth and the external-review package now records the ProVerif analysis; claim boundary unchanged; last-updated 2026-07-10
- Problem: the governance spine (NEXT_ACTIONS/DECISIONS/TRACEABILITY/this ledger) is current, but the strategic narrative is 6–18 weeks stale. `STATUS.md` (2026-03-02) still lists NA-0177 READY; `ROADMAP.md` (2026-04-30) and `DOC-PROG-001` (2026-04-03) predate the crypto-core arc; the whole `docs/public/**` review corpus (2026-06-25) predates NA-0619..0628. **Highest-value gap: `docs/public/EXTERNAL_REVIEW_PACKAGE.md` omits the CI-gated ProVerif analysis (NA-0627) — the project's single strongest assurance artifact.**
- Claim-safety: the audit's finding is that closing ENG-0034 moves NO claim (post-compromise language still blocked by the A1–A8 abstractions, ENG-0035, and independent review). The fix STRENGTHENS the evidence base while holding the claim boundary; D566 is built fail-closed around any claim-status/sentence change.
- Fix: execute D566 — the [SAFE] bulk (formal-plan ENG-0034 update, review-package/evidence-map/progress additions, STATUS.md deprecation-to-stub per the QSL_PUBLIC_RELEASE_PLAN.md precedent, superseded-by pointers) plus the two [CLAIM-ADJ] posture edits (ROADMAP, DOC-PROG-001) presented as exact before/after for operator approval. Optional: a lightweight doc-staleness lint so this does not silently recur (WF-0012 theme).
- Recommended directive shape: docs/governance single-PR lane, claim-adjacent, fail-closed on any claim movement. Strong successor candidate — arguably BEFORE commissioning external review, since you do not hand a reviewer a package that omits your formal analysis.

### ENG-0037 — Sealed-sender: hide sender↔recipient (the social graph) from the relay/qsl-server — **NEW; filed 2026-07-10 (was owed since NA-0622, never filed)**
- Severity: P3 (metadata; deferred post-Stage-2) — but it is the **flagship metadata item**: it is the concrete mechanism behind the operator's standing "eventually beat Signal on metadata" goal. Message content is already fully protected by Suite-2 E2EE; this closes a WHO-TALKS-TO-WHOM exposure to the relay operator, not a content break.
- Status: open — **filed 2026-07-10**; previously owed off a promised relay/sender-metadata audit since NA-0622 and never converted into a tracked item (the gap my own ENG-0036 entry flagged as "still unfiled"); last-updated 2026-07-10.
- The gap: today the relay/qsl-server observes enough to reconstruct the sender↔recipient social graph (route tokens, delivery routing, timing). Suite-2 hides message CONTENT from the relay; it does not hide the communicants' relationship from it. Signal's Sealed Sender is the precedent (studied — see the ROLLING_OPERATIONS_JOURNAL source-verification entry).
- **Prerequisite (operator's own stated plan): a relay/sender-metadata audit FIRST.** Enumerate exactly what qsl-server currently learns about who talks to whom (extend `docs/design/DOC-G5-004` the metadata-leakage surface review), THEN design sealed-sender off concrete findings rather than assuming the mechanism. "Prove; do not assume" applies to the threat surface too.
- Threat model it answers (name the adversary): **the relay operator / a party with relay logs.** This is EXACTLY the adversary that ENG-0036's access token does NOT address — the two are complementary, not substitutes. It does NOT answer a global passive network adversary (mixnet/Loopix territory, ENG-0022, far higher cost).
- **Interaction with the public/private server mode (ENG-0036):** in a PRIVATE self-hosted deployment for a trusted group, the operator IS the relay, so sealed-sender is lower-value (nice-to-have). In a PUBLIC deployment it is HIGH-value, because untrusted users + an untrusted operator see the full graph. The public/private setup toggle is the natural seam at which "which metadata protections apply" is decided.
- Recommended directive shape: analysis-first — the relay-metadata audit as its own lane (findings + severity), then a sealed-sender DESIGN lane (cross-repo: qsl-server routing + qsc client), then implementation. Do NOT collapse these; the audit may reshape the design. Sequenced post-crypto-core, alongside/after the metadata batch (ENG-0022/0027) and the private-server work (ENG-0036).

### ENG-0038 — `qsc` handshake: the responder is NOT authenticated to the initiator (asymmetric auth; active-MITM responder impersonation) — **NEW; filed 2026-07-11 by NA-0632 (D-1256)**
- Severity: **P1** — an authentication bypass in the SHIPPED establishment path. An active on-path attacker (the relay is a natural position; the product's self-hosted-relay niche puts it on path) can impersonate the responder to the initiator, and the out-of-band verification code a user checks does NOT prevent it. Remote-reachable, deterministic, no secret knowledge required. **Fix BEFORE the GUI** (report §6). Rated P1 as an analysis finding; the operator sets the final priority and picks the fix.
- Status: **REMEDIATED at NA-0633 (D-1257, directive D570) — construction C1.** Filed 2026-07-11 by the NA-0632 internal adversarial re-analysis (D569; finding FILED not fixed, analysis-lane rule); fixed by NA-0633. The initiator now encapsulates to the responder's PINNED identity KEM key (carried in the contact, verified against the human code) and mixes the shared secret into `pq_init_ss`, so a responder that cannot decapsulate (does not hold the pinned identity KEM secret) fails the initiator's transcript-MAC check → explicit reject at B1, no committed session. Proven end-to-end (`tests/NA_0633_eng0038_reproduction.rs`: a wrong responder is REJECTED, the genuine responder still establishes; no regression of the initiator→responder direction). Closes the DOC-CAN-003 §6.3 gap DOC-AUD-002 §178 recorded. As-built: `docs/governance/evidence/NA-0633_design_lock.md` (C1 + AS-BUILT). Original report: `docs/governance/evidence/NA-0632_adversarial_reanalysis.md` §2. **NA-0634 (D-1258, directive D571 REV 4) COMPLETED the interim — the SIGNING half:** the single verification code now binds `fingerprint(kem_pk, sig_pk)`, `sig_fp` is finally populated at provisioning, and the initiator REQUIRES the responder's signing key to match it at B1 (fail-closed `responder_sig_mismatch`/`responder_sig_unpinned`) — retiring the whole authentication-asymmetry class (not just the KEM half C1 fixed) and closing the never-populated-`sig_fp` weakness. A canonical KDF combiner (`hs_root_combine`) replaced C1's incremental append. As-built: `docs/governance/evidence/NA-0634_as_built.md`. The Signal-shaped prekey end-state remains D571 Decision 3 (NA-0635, GATED). Last-updated 2026-07-11.
- **✅ DISCHARGED verification obligation (was ⚠ OPEN; paid by NA-0636 at D-1259, 2026-07-12).** The obligation: NA-0634 left the **responder→initiator** sig-pin OPTIONAL on a REDUNDANCY argument — the responder's primary pin recomputes the combined `fingerprint(kem_pk, sig_pk)`, which already binds `init.sig_pk`, so the separate signing-key pin was judged redundant — and that redundancy was **asserted on REASONING, not proof**. The `QSC.HS.*` formal-model lane (D571 Decision 4 / directive D572) was required to VERIFY it rather than assume it. **VERDICT: the reverse sig-pin IS REDUNDANT — the obligation is discharged AFFIRMATIVELY by the model, not by re-argument.** `formal/model_qsc_handshake_authentication_bounded.py` (registered in `run_model_checks.py`; runs in the REQUIRED `formal-ci` job) exhaustively enumerates 10,800 responder configurations — every reachable contact-pin state **including the `sig_fp`-absent ones where the optional reverse pin SKIPS**, every mid-run re-pin between A1 and A2, all 16 adversary compromise subsets of the honest identity secrets, and every presented key pair — and finds **0 reachable responder-commits in which the initiator's presented signing key escapes binding to the verified code** (80 commits judged). There is no run the reverse pin would have caught that the required primary combined pin does not.
  - **The verdict is CONTINGENT, and the dependency is machine-checked:** redundancy holds *because and only because* the combined code covers the signing key **injectively** (collision-resistance is ASSUMED — a crypto-agnostic model cannot prove it). The model's non-vacuity counterfactual rewinds that one property to the pre-NA-0634 KEM-only code and immediately finds **128 unbound-signing-key commits** a required reverse pin *would* have caught. **⚠ REGRESSION GUARD: if the verification-code format is ever narrowed back to the KEM half, the reverse pin becomes LOAD-BEARING again and this discharge is VOID.** That obligation now sits on the code format, not on the pin.
  - **Non-vacuity / faithfulness (WF-0017 — a negative claim must show the search could have found a positive):** with the landed defences rewound, the same model **reproduces the real ENG-0038 flaw** — 54 impersonation traces, canonically an adversary that has stolen **nothing**, signing B1 with its own generated keypair, making the initiator commit `authenticated=true` to a peer holding neither of the responder's identity secrets (verbatim the NA-0632 §2.2 scenario). Under the **landed** rules: **0** such commits, and **0** for an adversary holding the responder's KEM identity secret but NOT its signing secret (the NA-0634 half). The fix chain is confirmed closed in model form. The formal gate **fails closed**: a P3 disproof raises `QSC_HS_HANDSHAKE_AUTH_MODEL_GAP_FOUND` and turns the check RED (verified by re-running the P3 machinery against the pre-fix rules).
  - **Secondary result — do NOT make the reverse pin required "for symmetry".** 60 enumerated configurations are commits that are correctly bound *and* that a REQUIRED reverse pin would reject (`responder_sig_unpinned`): the **S-BARE** contacts provisioned by `contacts add --fp <code>` (code only, no keys ⇒ no `sig_fp`), which cannot initiate but can legitimately respond. Requiring the pin would convert sound handshakes into rejects and catch nothing. This answers the open question NA-0634 recorded ("a strict reading … might want it required too"): **the model says no.**
  - **Claim boundary UNCHANGED.** A PASS substantiates a bounded authentication-BINDING property over an abstract state machine — NOT cryptographic security, NOT a side-channel property, NOT a post-compromise/PQ guarantee, NOT refimpl equivalence. Independent external review remains an open prerequisite.
  - **⚠ KNOWN UNMODELED SLICES of the NA-0636 model (recorded at operator direction, 2026-07-12, BEFORE merge — so the discharge is read with its limits attached, not without them).** ENG-0038 hid *because* the handshake-authentication slice was unmodeled; the model that closes it has its own unmodeled slices, and naming them is what keeps a green result from becoming the next false assurance. The model does NOT cover:
    1. **The contact-store DEVICE INDIRECTION (the substantive one).** The model represents the pin store as a **single coherent triple** `(pin_code, kem_stored, sig_fp)` — an ABSTRACTION, not a proved invariant. The real code resolves the three pin reads through a **primary-device indirection**: `identity_read_pin` (`identity/mod.rs:635`) returns the primary device's `fp`, while `identity_read_sig_pin` (`:649`) and `identity_read_peer_kem_pk` (`:661`) return the primary device's value **`.or(` the contact record's `)`**. Read in isolation those could resolve from *different* records. **They do not** — `contacts_entry_read` (`contacts/mod.rs:464`) → `contacts_store_load` (`:429`) runs `normalize_contact_record` (`:144`) over every record on every load, force-syncing the record's `fp`/`sig_fp`/`kem_pk` **from the primary device** (`:221-232`) and guaranteeing a device exists (`:146-158`); `contacts_store_save` normalizes on write (`:451-455`). So the fallbacks are **inert** and the store is coherent at read time. **BUT that justification was established by READING THE CODE — it is NOT model-verified.** Also unmodeled: the primary-device **selection rule** (`primary_device_id` → first `TRUSTED` → first device, `:191-206`) and a **change of primary device mid-handshake** (promotion/removal/newly-trusted) — a re-pin channel the model represents only abstractly via its `pin_a1`/`pin_a2` dimension; and the fact that `contacts_device_add` (`:1139`) writes an operator-supplied `fp` with `sig_fp: None`, `kem_pk: None` and **no key verification** (`:1165-1166`).
       **⚠ The P3 verdict is ARGUED to survive this, and that argument is REASONED — NOT MODEL-VERIFIED.** The reasoning: the signing-key binding flows entirely from the REQUIRED primary pin (which recomputes the combined code from the presented pair), so a stale or mismatched `sig_fp` can only make the OPTIONAL reverse check *stricter* — a false reject — and can never admit a commit the primary pin would refuse. Fail-closed. **This is precisely the shape of claim ("it's fine, and here is why") that this lane exists to distrust**, and it is recorded as a claim to be discharged, not as a result. **Candidate follow-up lane: extend the bounded model to the device indirection + primary-device selection** and convert this argument into a search result.
    2. **Cross-session replay.** One bounded handshake per configuration; no multi-session or cross-session token replay.
    3. **Concurrent pendings.** A single pending handshake per party; no interleaved or competing pending records.
    4. **Composition with suite negotiation / downgrade.** Modeled separately (`model_suite2_negotiation_bounded`, `model_qsc_handshake_suite_id_bounded`); the **composition** of negotiation with authentication is covered by neither.
    5. **Fingerprint collision-resistance** — ASSUMED (codes are injective structured tokens), never proved; see the contingency bullet above, which is the load-bearing case of this.
  - Ref: NA-0636; D-1259; directive `QSL-DIR-2026-07-11-572` (D572); `docs/governance/evidence/NA-0636_as_built.md` (§1 the read-only semantics extraction incl. §1.2's explicit abstraction boundary, §4 the verdict); `tests/NA-0636_qsc_hs_handshake_auth_model_testplan.md`; `formal/README.md` §2/§4 (P13–P16)/§5. Prior: `docs/governance/evidence/NA-0634_as_built.md` §Design decisions (2); D-1258.
- **⚠ RE-TESTS AND CONTRADICTS a prior "verified" conclusion.** ENG-0001 / NA-0609B concluded "the verification-fingerprint model is COHERENT … there is no KEM-vs-SIG binding flaw." D569 mandated re-testing exactly such claims. It does not hold on current code for the **initiator→responder** direction: the KEM fingerprint a user verifies out-of-band authenticates the initiator TO the responder, but NOT the responder to the initiator.
- The defect (each step a verified code fact; report §2.1):
  1. The responder's only identity credential in `B1` is its ML-DSA `sig_pk`; `HsResp` carries NO KEM public key of the responder (it sends only `kem_ct`, an encapsulation to the *initiator's* public key). `qsc/src/handshake/mod.rs:138-148,1885,1938-1944`.
  2. The initiator verifies the signature under `resp.sig_pk` — the key the responder SENT — which is self-consistent for ANY key. `handshake/mod.rs:1509`.
  3. The responder's signing key is pinned only via the OPTIONAL `sig_fp`, which is structurally always `None`: `contacts_add`/`contacts_device_add` set `sig_fp: None` (even with `verify=true`), and no path writes a learned `sig_fp` back. `contacts/mod.rs:1047,1053,1110`; the optional check skips on `None` (`handshake/mod.rs:1532`, `identity/mod.rs:634-641`, `handshake/mod.rs:1001`).
  4. The initiator's REQUIRED "primary" (KEM) pin is inert here: the responder's KEM key is never sent/used B→A, and the check is TAUTOLOGICAL (`pending.peer_fp` = `identity_read_pin(peer)` at initiate, re-compared to the same pin at B1). `handshake/mod.rs:1241,1295,1527`.
  5. ⇒ the initiator commits a Suite-2 session with `authenticated=true` for any responder signing key. `handshake/mod.rs:1550-1551`. (The responder→initiator direction is sound — the initiator's KEM key IS pinned and used, and KEM-secret possession is proven by the A2 confirm MAC.)
- Failure scenario: on-path M intercepts A1, encapsulates to A's public `kem_pk`, generates its OWN ML-DSA keypair, signs the B1 transcript, sends B1. A accepts (MAC ok, sig verifies under M's key, KEM pin tautological, sig pin skipped) and commits a session with M as "B". Not first-contact-only (`sig_fp` never populated). Report §2.2.
- Claim at stake: DOC-CAN-003 §6.3/§0.2 make authenticated peer identity a PRECONDITION the Suite-2 core rests on ("MUST authenticate peer identity before Suite-2 state is committed"). On the shipped initiator path that precondition is not met. Same shape as ENG-0019 (`authenticated=true` asserted; real auth absent) but on the SHIPPED `qsc` path, not the `qsp` reference actor.
- Exact surfaces: `qsc/src/handshake/mod.rs` (initiator B1 handling `:1507-1551`; responder A1 handling `:1867-1984`; `hs_check_optional_identity_pin` `:971-1011`; `hs_require_primary_identity_pin` `:920-969`); `qsc/src/contacts/mod.rs` (`contacts_add` `:1019-1072`, `contacts_device_add` `:1084`); `qsc/src/identity/mod.rs` (`identity_read_pin`/`identity_read_sig_pin` `:620-641`).
- Proof gap (why it was not caught): `src/adversarial/binding_fuzz.rs` covers only frame decoding + pin-string comparison; `tests/kem_signature_transcript_binding_negative.rs` must hand-inject `sig_fp` (no product path sets it) and only tests a wrong-pinned value; the ProVerif model (DOC-G4-002) covers the ratchet composition, NOT the `qsc` handshake authentication (UNMODELED). Report §2.3.
- Minimal fix direction (design-lock-first; operator chooses): (a) wire `sig_fp` into contact provisioning so the responder's signing key IS pinned and the optional check becomes effective/required; AND/OR (b) cryptographically bind the responder's identity into the B→A direction (carry+pin the responder's identity KEM key so the KEM handshake authenticates BOTH directions, or certify the responder's signing key under the pinned identity); (c) make the initiator's primary pin non-tautological. Add the report §B proof-of-issue test (un-`#[ignore]` on fix) + a regression guard that the shipped path yields a non-empty responder `sig_fp` (or otherwise binds the responder) before committing a session.
- Recommended directive shape: a before-GUI remediation lane (design-lock-first: the authentication model for BOTH directions, then the minimal wiring), plus — to decide the residual — a ProVerif/Tamarin model of `QSC.HS.*` (extends the ENG-0035 formal track). Honest caveat: this is an internal code-trace corroborated by the test infrastructure, NOT a running PoC; independently confirm before acting, though the trace is unambiguous.

### WF-0019 — An audit "no P0/P1"/"verified sound" certification is only as strong as its EXERCISED coverage — NA-0609B certified sound the seam that carried ENG-0038 — **NEW; filed 2026-07-12 by NA-0637 (D-1260; directive D573, paying D571 Decision 4)**
- Severity: P2 (process/audit-methodology; the false assurance was load-bearing — ENG-0001/NA-0609B's "no KEM-vs-SIG binding flaw" stood as grounds not to suspect the seam until D569 mandated re-testing exactly such claims, and NA-0632 then contradicted it)
- Status: done — closed by NA-0638 (D-1261, directive D574, 2026-07-12): the scoped re-examination executed; per-claim verdicts = 6 EXERCISED / 2 INSPECTED-ONLY (→ WF-0021 closed-as-paid, WF-0022 open) / 1 CONTRADICTED (claim 5, the calibration point). The per-claim exercised-coverage box + non-vacuity question for the DOC-AUD-001 template remain a recommended successor (unchanged below); the constructive standard stands
- The claim: NA-0609B ("qsc Handshake and Identity Read-Only Security Audit", D543, D-1213, 2026-07-06) closed with result `QSC_HANDSHAKE_IDENTITY_AUDIT_COMPLETE_NO_P0_P1_THREE_P3_HARDENING` — "No P0 or P1 finding was substantiated. The handshake seam is well-constructed and fail-closed." — and enumerated the seam "verified sound on transcript binding, hybrid handshake + all-zero DH guard, fail-closed ML-DSA verify, downgrade/suite-context binding, dual-pin identity model, no-mutation-on-reject, atomic writes, and replay rejection." Specifically certified: "Identity binding: a dual-pin model — the primary pin is checked against the KEM identity fingerprint …, with the ML-DSA signing-key fingerprint (`hs_sig_fingerprint`) as a separate optional pin. A mismatch fails closed …" and "`hs_sig_verify` … is fail-closed on both invalid and error" (`docs/governance/evidence/NA-0609B_qsc_handshake_identity_security_audit_harness.md` §Verified Sound).
- The reality: ENG-0038 — a P1 responder-impersonation authentication bypass — lived INSIDE the certified mechanisms, on the shipped initiator path: the "optional pin" (`sig_fp`) was structurally always `None` (no product path populated it — the check always SKIPPED); the "primary pin" was TAUTOLOGICAL for the initiator (`pending.peer_fp` = `identity_read_pin(peer)` at initiate, re-compared to the same stored value at B1 — the responder's KEM key is never sent or used B→A); the fail-closed ML-DSA verify was self-referential for identity (verified under the key the responder itself sent). Found by NA-0632 (D-1256, 2026-07-11), fixed by NA-0633/NA-0634 (D-1257/D-1258), modeled by NA-0636 (D-1259) — which also named five slices of the same seam STILL unmodeled (see the ENG-0038 entry). Every local observation the audit recorded was accurate; the soundness CONCLUSION was not supported by the coverage that produced it.
- The methodology lesson (the actual finding): a "no P0/P1" or "verified sound" verdict is a claim about COVERAGE, not just about code. An audit that reaches a soundness verdict WITHOUT an end-to-end adversarial exercise of the seam can certify sound a seam that is not: inspection verifies that mechanisms exist and fire; it does not verify that they BIND. NA-0609B had no wrong-responder end-to-end exercise — that test did not exist until the fix lane (NA-0633, `tests/NA_0633_eng0038_reproduction.rs`). Soundness verdicts MUST be backed by NAMED, EXERCISED coverage (a test/vector/model run demonstrably capable of finding a counterexample), not inspection alone; per-claim, the audit must state WHICH mechanism exercised it.
- The pattern: this is the SECOND methodology miss from the same audit. WF-0005 (done at NA-0609D, D-1216) recorded a false POSITIVE from incomplete inspection (ENG-0004: the cfg-gated fsync variant was missed). WF-0019 records the inverse and more serious false NEGATIVE from inspection that could not see a vacuous binding. Same root cause: the conclusion outran the mechanism that produced it.
- The constructive standard (the fix precedent): NA-0636's WF-0017 non-vacuity anchor — a negative/"no flaw" claim must demonstrate the search COULD have found a positive (the NA-0636 model reproduces the real ENG-0038 flaw when the defences are rewound; that is what makes its green verdict meaningful). Audits should carry the analogous check: "could this audit have detected the flaw class it certifies absent?" — answered per certified claim, with the detecting mechanism named. Recommended landing spots (successor lanes, not here): the DOC-AUD-001 audit template gains a per-claim exercised-coverage box + the non-vacuity question; the re-examination lane (ON DECK 0a) applies it retroactively to NA-0609B's remaining claims.
- Ref: NA-0637; D-1260; directive `QSL-DIR-2026-07-12-573` (D573); D571 (REV 4) Decision 4 + Phase 5 (the origin of this filing); ENG-0038 (incl. the "RE-TESTS AND CONTRADICTS" note and the five unmodeled slices); WF-0005; WF-0017; WF-0020 (the sibling process item — this filing itself was dropped once); DOC-AUD-001; `docs/governance/evidence/NA-0637_as_built.md` §1–§5; `docs/governance/evidence/NA-0609B_qsc_handshake_identity_security_audit_harness.md`; `docs/governance/evidence/NA-0632_adversarial_reanalysis.md` §2.

### WF-0020 — An approved-directive instruction can close a lane UNEXECUTED with no automatic detection — the D571 Decision-4 filing was dropped at NA-0634 closeout — **NEW; filed 2026-07-12 by NA-0637 (D-1260; directive D573)**
- Severity: P3 (process; one observed instance, but the class is silent by construction — a dropped instruction leaves NO artifact, so every existing closeout check passes)
- Status: open — recorded for tracking; the corrective (a closeout directive-instruction completeness check) is deliberately NOT implemented in this lane (D573 explicit non-goal) and awaits its own directive
- The instance: D571 (REV 4) Decision 4 required "File an AUDIT-METHODOLOGY finding … (file now, re-scoped)"; the D571 phase table assigned it: "Phase 5 — Audit-methodology finding: file the coverage blind spot (not just the missing test) per Decision 4; schedule the bounded re-examination of NA-0609B's coverage claims." NA-0634 closed 2026-07-11 (D-1258) WITHOUT filing it: `docs/governance/evidence/NA-0634_as_built.md` contains zero mention (grep-verified); no WF item was added to this ledger; D-1258 references the finding only under "Successor — PROPOSED, NOT PROMOTED" as a candidate lane — future work, where the directive had ordered an executed in-lane filing. The omission went undetected through the NA-0634 closeout, the NA-0636 promotion (PR #1551), and the whole NA-0636 lane (D-1259), surfacing only at the 2026-07-12 ad-hoc read-only live-state check.
- Why it was silent: closeout verifies queue invariants (sole-READY, decision counters), governance artifacts (DECISIONS/TRACEABILITY/journal/evidence), and scope — every one of which PASSES when an instruction is simply never executed. Nothing walks the approved directive's numbered decisions and phase table and demands, per instruction, evidence of executed-or-explicitly-deferred. The discovery mechanism (a live-state check the operator happened to commission) is not guaranteed to run.
- Second-order note (recorded honestly, per D573): the dropped instruction was ITSELF the audit-coverage finding — a coverage gap fell into a coverage gap. The failure mode WF-0019 records for audits (conclusion recorded, backing mechanism absent) recurred at the process level (closeout recorded, instruction-completeness never checked). The two findings are one lesson at two altitudes.
- Recommended change (NOT implemented here): a closeout checklist step — enumerate the governing directive's numbered Operator Decisions and phase-table rows; map each to either (a) evidence of execution (file/PR/ledger anchor) or (b) an explicit deferral recorded in the closeout DECISIONS entry with the operator's acknowledgment. A dropped instruction then fails closeout instead of vanishing. Cheap; pairs with WF-0016 (session-handoff contract) and the WF-0003 closeout triage. Candidate lane: batch with WF-0016 as one docs/process LITE lane.
- Ref: NA-0637; D-1260; directive `QSL-DIR-2026-07-12-573` (D573); D571 (REV 4) Decision 4 + Phase 5; D-1258 (the closeout that dropped it); WF-0019 (the sibling finding — the dropped content itself); WF-0016; WF-0003; `docs/governance/evidence/NA-0637_as_built.md` §6.

### WF-0021 — The all-zero/degenerate-DH guard had ZERO exercised coverage at NA-0609B audit time; the settling exercise landed post-audit (NA-0628/ENG-0034) — **NEW; filed 2026-07-12 by NA-0638 (D-1261; directive D574); CLOSED-AS-PAID on filing**
- Severity: P3 (coverage-history record; the code gap it describes is already remediated and tested)
- Status: done — closed-as-paid on filing. Filed so the coverage history is on the ledger rather than implicit in a diff: the NA-0638 re-examination classified NA-0609B claim 2 ("hybrid handshake + all-zero DH guard", audit doc :48) **INSPECTED-ONLY** and this entry records both the audit-time gap and its post-audit payment
- The audit-time gap (at `c0b30265`, the NA-0609B merge): `all_zero` appeared ONLY in `qsl/qsl-client/qsc/src/handshake/mod.rs` (the guard and its call sites). No test, no model, no fuzz target referenced the guard or fed a degenerate public key on any path; deleting the guard would have turned nothing red. The hybrid combine was exercised only by happy-path establishment — proving it works, not that it fails closed.
- The payment (post-audit): NA-0628/ENG-0034 (directive D565 as amended, PR #1536, 2026-07-10) added `establishment_dh_rejects_every_low_order_peer_key`, `seven_of_eight_low_order_keys_evade_the_encoding_check` — which also demonstrates the audit-time defence was NARROW: 7 of 8 low-order keys evade the encoding check the all-zero guard sat behind — `establishment_dh_accepts_an_honest_peer_key`, `length_errors_keep_their_distinct_marker` (in `handshake/mod.rs` unit tests) plus four ratchet-boundary noncontributory-rejection tests and `noncontributory_guard_is_not_shadowed_by_the_dh_pub_encoding_check` (in `refimpl` `suite2/ratchet.rs`). These are exactly the settling exercise this finding would otherwise have named; no successor lane is needed.
- The lesson (WF-0019 corollary): the narrow defence (all-zero only) plus zero coverage coexisted with a "verified sound" certification for two+ months; the claim as WORDED was accurate (the guard existed and fired on all-zero) — the certification format just could not distinguish an exercised claim from an inspected one. That distinction is the DOC-AUD-001 template recommendation on WF-0019.
- Ref: NA-0638; D-1261; directive `QSL-DIR-2026-07-12-574` (D574); WF-0019; ENG-0034 (closed); NA-0628 (D-1251/D-1252, PR #1536); `docs/governance/evidence/NA-0638_as_built.md` §2 (claim-2 row), §4; NA-0609B claim 2 (`docs/governance/evidence/NA-0609B_qsc_handshake_identity_security_audit_harness.md` :48).

### WF-0022 — `write_atomic` crash-window atomicity ("file content is never partially written") has NEVER been exercised by any mechanism — **NEW; filed 2026-07-12 by NA-0638 (D-1261; directive D574)**
- Severity: P3 (defense-in-depth/coverage; the failure direction is fail-closed — a torn write yields a vault/store that fails to decrypt or parse, a lockout/availability event, not a confidentiality or auth break; the code was re-verified sound by inspection at NA-0609D)
- Status: done — closed by NA-0639 (D-1262, directive D575, 2026-07-12): the settling exercise now exists — `qsl/qsl-client/qsc/tests/NA_0639_wf0022_atomic_write_crash_window.rs` EXERCISES the real `write_atomic` through the compiled binary (`config set policy-profile`, exactly one `write_atomic` per invocation once the layout exists). Crash window simulated at the NAMED point — after temp-write+`sync_all`, at `fs::rename` (`fs_store/mod.rs:120-122`) — by denying directory write permission once the deterministic tmp (`config.txt.tmp.<pid>`) appears; assertions: subsequent reader sees target byte-identical OLD, tmp residue holds complete NEW and was never the live target, recovery write lands exactly NEW; plus a concurrent-reader test sampling across repeated real writes (every sample exactly-OLD-xor-exactly-NEW). WF-0017 non-vacuity DEMONSTRATED: two negative controls (test-local truncate-then-write in-place path) trip the SAME classifier (half-written target ⇒ Torn; concurrent reader catches the held-open torn window), and a temporary red-run demo showed the positive test failing against the non-atomic writer (36,961 torn samples of 45,033). The test FAILS if no trial lands in-window (no silent vacuity). Simulation limits stated: rename denied at the directory-permission seam, not a kernel kill — power-loss/page-cache/fsync-lying semantics remain out of scope. RESIDUE (stays on NEXT_ACTIONS ON-DECK 0c, per the D575 scope note): the claim-7 handshake_complete-marker fault case and the per-seam kill/fault matrix between each of temp-write/`sync_all`/rename were NOT in D575's scope and remain unexercised
- The gap: NA-0609B claim 8 (audit doc :69) certified `write_atomic` (`qsl/qsl-client/qsc/src/fs_store/mod.rs`) — temp-file write, permission enforcement, `sync_all`, atomic rename, "so file content is never partially written." The NA-0638 re-examination classified the claim **INSPECTED-ONLY**: exercised neighbors existed at audit time (`tests/fs_store_contract_na0217b.rs` — permission modes, symlink reject, lock contention, leftover-tmp cleanup; `tests/session_state_at_rest.rs` — at-rest tamper fails closed) but the CORE property, atomicity across the temp-write → `sync_all` → rename window, had and has NO mechanism: no crash or fault injection exists on any store path, so a reordering or partial-write regression (e.g. rename-before-sync) would turn nothing red.
- History rhyme (recorded per WF-0019): this same bullet's neighborhood produced ENG-0004 (the audit's directory-fsync finding) and then WF-0005 (ENG-0004 was a false positive from incomplete inspection — the cfg-gated fsync variant was missed; NA-0609D corrected it by MORE inspection). Both directions of error on this seam came from reading; neither direction has ever been decided by an exercise.
- The settling exercise (names, per the WF-0019 standard): a fault-injection harness on `write_atomic` — kill/fault the process (or interpose a failing/reordering filesystem shim at the provider seam) between each of temp-write, `sync_all`, and rename; assert on recovery that the target file content is old-XOR-new, never mixed and never absent-when-old-existed; include the claim-7 residue (a store-stage fault while a `handshake_complete` marker is pending — assert no marker without a committed session even under store failure). Non-vacuity check per WF-0017: the harness must turn RED when run against a deliberately weakened `write_atomic` (e.g. rename before sync, or direct in-place write).
- Ref: NA-0638; D-1261; directive `QSL-DIR-2026-07-12-574` (D574); WF-0019; WF-0005; ENG-0004 (closed, stays closed); NA-0609D (D-1216); `docs/governance/evidence/NA-0638_as_built.md` §2 (claims 7/8 rows), §4, §5; NEXT_ACTIONS ON DECK 0c. Closure: NA-0639; D-1262; directive `QSL-DIR-2026-07-12-575` (D575); `qsl/qsl-client/qsc/tests/NA_0639_wf0022_atomic_write_crash_window.rs`; `docs/governance/evidence/NA-0639_as_built.md`; `tests/NA-0639_wf0022_atomic_write_crash_harness_testplan.md`.

### WF-0023 — NO test exercised the real product stack (qsc <-> qsl-server <-> qsl-attachments) together; the only real-qsl-server coverage was schedule-only + unpinned and rotted silently for ~5 months — **NEW; filed 2026-07-12 by NA-0640 (D-1263; directive D576); CLOSED-AS-PAID on filing**
- Severity: P2 (coverage/process; the gap hid a real 5-month CI rot and left cross-component interop entirely unverified — but the failure it enabled was silent-coverage-loss, not a demonstrated product defect: the NA-0640 e2e round-trip PASSED against unmodified product source at the pinned revs)
- Status: done — closed by the same lane (NA-0640, D-1263, directive D576): the settling coverage now exists IN THE STANDARD SUITE.
- The gap (verified by the 2026-07-12 read-only investigations): every "integration" test swapped in a mock for at least one leg — `two_client_local_runbook_na0182.rs` uses qsc's own embedded `relay serve` (transport/mod.rs:835), the attachment tests (`attachment_streaming_na0197c.rs`, `na_0617_*`) use the test-local inbox in `tests/common/mod.rs:293`; the ONLY coverage that touched a real qsl-server binary was `relay-ui-integration.yml` — schedule-only (unwatched), cloning qsl-server at UNPINNED default-branch HEAD. When qsl-server's na-0012 (`3897ca1`, 2026-03-30) retired the legacy `/v1/push/:channel` routes, the workflow's health probes (`POST /v1/push/health`, `GET /v1/pull/health` — channel-in-path pushes under the OLD API) began to 404 and the step died BEFORE `cargo test` ran. Last green: 2026-02-11. Nobody noticed until the 2026-07-12 investigation. Root cause, both halves: (a) schedule-only coverage can silently stop running; (b) an unpinned cross-repo dependency lets the covered surface drift out from under the covering test.
- The payment (this lane): (1) `qsl/qsl-client/qsc/tests/NA_0640_full_stack_e2e.rs` — two isolated qsc clients + the REAL qsl-server IN-PROCESS (dev-only git dependency pinned at `19b9b02dbe1f2ae9bc246ff3a16890e56c073c3e`; `tests/common/mod.rs` `start_qsl_server`, mirroring `start_attachment_server`) + the REAL qsl-attachments in-process: message round-trip with plaintext byte-match + receipt round-trip, AND a >4 MiB attachment round-trip on the REAL attachment path (upload sessions on the real service, descriptor through the real relay, download byte-verified); auth modes covered: open relay (message+attachment) and bearer-token relay (message + a wrong-token rejection negative). Runs in the standard `cargo test -p qsc` suite — it cannot silently stop running, and qsl-server drift is now a deliberate, visible pin bump. (2) The two UNIQUE TUI focus-routing assertions (unfocused ⇒ `mode=buffer`+`unread=1`; focused ⇒ `mode=append`+`unread=0`) — previously guarded ONLY by the dead workflow — ported in-suite as `tests/NA_0640_tui_focus_semantics.rs` (na0177 mock-inbox idiom; the focus KEY model changed since na-0127 — `/key tab` now toggles INTO the thread — the port drives current controls, the asserted semantics are unchanged, verified against `tui/controller/state/account.rs:440-443`). (3) `.github/workflows/relay-ui-integration.yml` RETIRED (deleted) — transport leg superseded by (1), TUI value preserved by (2), order preserved-then-retire.
- Coverage limits (stated per D576 — a PASS asserts interop under the tested scenarios at the pinned revs, NOT production-readiness): auth = open + bearer-token (token rotation/revocation untested); sizes = one small message + one 6 MiB attachment (no boundary sweep here — the 4 MiB threshold boundary is covered by na0197c against the mock inbox); paths = happy-path round-trips + one auth negative (no drop/reorder/fault injection against the real relay — the retired remote smokes' scenario territory remains a candidate successor); the qsc `#[ignore]`d `relay_ui_integration.rs` test FILE remains in-tree (dormant, no runner) — deleting it was outside D576 scope, candidate for a hygiene sweep.
- Residual (recorded, not in scope): the two REMOTE scheduled workflows (`remote-relay-tests.yml`, `remote-handshake-tests.yml`) are STILL red daily against the deployed relay (last green 2026-02-09) and remain gated on NA-0564/NA-0565 (operator); this lane fixes the LOCAL coverage class only.
- Ref: NA-0640; D-1263; directive `QSL-DIR-2026-07-12-576` (D576); `qsl/qsl-client/qsc/tests/NA_0640_full_stack_e2e.rs`; `qsl/qsl-client/qsc/tests/NA_0640_tui_focus_semantics.rs`; `tests/common/mod.rs` (`start_qsl_server`); `qsl/qsl-client/qsc/Cargo.toml:36-39` (both pinned service dev-deps); `docs/governance/evidence/NA-0640_as_built.md`; `tests/NA-0640_e2e_integration_full_stack_testplan.md`; NA-0564/NA-0565 (the remote-relay operator track); WF-0019 (the exercised-coverage standard this filing follows).

### ENG-0039 — qsl-server deferred hardening bundle (release/deploy/doc debt found by the 2026-07-13 review; D578 rule: fix ONE, file the REST) — **NEW; filed 2026-07-13 by NA-0642 (D-1265; directive D578)**
- Severity: P3 (deployment/process hygiene; none of it moves a security claim — but (b) leaves the documented release-based deploy path BROKEN, which matters for the self-host operator-path this program is now building)
- Status: open — filed 2026-07-13 at the NA-0642 closeout per the operator's explicit fix-one/file-the-rest decision (the ONE fix taken in-lane was relay.env.example MAX_QUEUE_DEPTH 256→257); last-updated 2026-07-13
- The bundle (all qsl-server repo; verified against pin `19b9b02d`, pre-NA-0642):
  - (a) **ENG-0014 cross-reference** — the non-constant-time bearer compare (`src/lib.rs:338` at review time) is ALREADY filed here as ENG-0014 but has NO in-repo trace in qsl-server itself; the fix lane should land a qsl-server-side pointer or the fix.
  - (b) **Stale/broken release path** — latest tag v0.0.4 predates the current routes (release-based deploy broken until a v0.0.5+ tag is cut); runbook references v0.0.3; Cargo.toml says 0.1.0 vs v0.0.x tags. A coherent version/tag/release story is owed.
  - (c) **Three orphaned scripts/ci guards** — packaging-alignment, deploy-compat, route-token-migration are referenced by nothing.
  - (d) **All five DOC-SRV contracts still DRAFT** — two were behind the code at review time, and **NA-0642 adds known drift**: README + DOC-SRV-003 still describe the retired ROUTE_IDLE_TTL_MS idle-discard and delete-on-pull-only delivery; neither documents STORE_PATH/RETENTION_TTL_SECS/PULL_LEASE_SECS or the acknowledged-pull mode (out of D578 scope by design; the retired `route_ttl_config_and_docs_are_explicit` doc-currency test went with its contract).
  - (e) **Dead tower-http dependency** (still present post-NA-0642).
  - (f) **Stale Caddyfile.example comment** about legacy path clients (paths retired at D-0010).
- Recommended directive shape: ONE qsl-server hardening lane (qsl-protocol-governed, satellite model per D578): cut the release/tag story, delete the orphaned guards, bring README/DOC-SRV-003 (and the other DRAFT DOC-SRVs) to post-NA-0642 truth, drop tower-http, fix the Caddy comment, and take ENG-0014 while the file is open. Low risk, high doc-truth value; sequence before or alongside the next self-host operator-path step.

### ENG-0040 — qsc client: adopt the acknowledged-pull contract (ack after local persistence; dedupe by msg_id) — **DONE at NA-0644 (D-1267, directive D580, this-lane PR; 2026-07-14)**
- Severity: P2 (reliability; the client's pull→persist crash window stays open until this lands — the server-side mechanism that closes it shipped at NA-0642 but the CURRENT client does not use it)
- Status: **DONE at NA-0644 (D-1267, directive D580, 2026-07-14)** — qsc gained the OPT-IN lease mode (`receive --ack-mode lease`): pull with `?ack=lease`, persist durably at the existing per-item commit points, THEN batch-ack (`POST /v1/pull/ack`, ≤4096 ids, after the pull loop and BEFORE attachment resume) — the server deletes only after the ack. NEW durable per-mailbox msg_id dedup (`src/dedup/mod.rs`, `relay_seen_ids_v1_<hash>.json` via `write_atomic`, 31-day prune > the 30-day retention ceiling + 65,536 cap) checks BEFORE unpack: a redelivered id is acked-and-skipped, never reprocessed, never process-exited. The invariant held per item: an id becomes ack-eligible only after BOTH its item's durable commit AND its seen-entry are on disk. Old-server tolerance: ack-404 = "legacy-complete" (info marker, no error, no retry). DEFAULT IS LEGACY, byte-identical (proven: the recorded pull URL is the exact pre-lane string, zero ack POSTs, no new markers; the NA-0640 e2e green UNCHANGED locally, zero edits). Proven by `tests/NA_0644_ack_client.rs` (6/6 green first run vs the REAL pinned server incl. real 1s-lease expiry/redelivery): lease happy path deletes server-side; **the LOST-ACK redelivery deduped (non-vacuous per WF-0017: the reverted red-run with dedup neutered fails with today's `qsp_replay_reject` process-exit)**; SIGKILL between persist and ack (the payload was on disk WHILE the ack was stalled — persist-before-ack observed) → clean dedup; the commit-before-write seam handled loudly (ENG-0042). LIMITS: lease is NOT the default (ENG-0043 owed); the pre-existing commit-before-write seam is HANDLED and FILED, not fixed (ENG-0042). As-built: `docs/governance/evidence/NA-0644_as_built.md`. Filed 2026-07-13; last-updated 2026-07-14
- The gap (honest statement, per D578 "do not fake robustness"): qsl-server now offers lease-mode delivery (`GET /v1/pull?ack=lease` + `POST /v1/pull/ack`, qsl-server D-0011), under which a message is deleted only after the client acknowledges local persistence — a crash between pull and local-store no longer loses it. The shipped qsc client still uses the legacy delete-on-pull path (`qsl/qsl-client/qsc/src/transport/mod.rs` `relay_inbox_pull`), so ITS crash window is status quo. Full end-to-end delivery durability requires this client lane.
- Scope sketch for the future directive: switch `relay_inbox_pull` to `?ack=lease`; send the ack ONLY after the pulled message is durably persisted locally (the NA-0639-exercised `write_atomic` path is the natural seam); dedupe redeliveries by msg_id (ids are already on the wire; at-least-once delivery means duplicates are expected after a lost ack); keep a legacy-pull fallback for talking to pre-NA-0642 relays (the server change is backward-compatible in one direction only — new client must tolerate old server).
- Cross-reference: qsl-server D-0011; NA-0642 tests `na0642_ack_contract.rs` (the server-side contract this client must speak); ENG-0041 (the pin bump that makes the new server visible to the qsc dev-dep e2e).

### ENG-0041 — qsl-protocol: bump the qsl-server dev-dep pin past NA-0642 and re-run the NA-0640 full-stack e2e locally — **DONE at NA-0643 (D-1266, directive D579, this-lane PR; 2026-07-13)**
- Severity: P2 (assurance currency; until the bump, the STANDARD suite's only real-server coverage exercises a PRE-durability relay — the pin is now knowingly STALE)
- Status: **DONE at NA-0643 (D-1266, directive D579, 2026-07-13)** — the pin advanced `19b9b02d` → `8e4ea27877db46a2b660b46c36ba60f3db73b38c` (confirmed qsl-server main HEAD by fresh `git ls-remote` at Phase 0) with a scoped mechanical `Cargo.lock` regeneration, and the NA-0640 e2e was RUN LOCALLY and passed **UNCHANGED** on the first post-bump invocation (2 passed / 0 failed, 115.57s: message + >4 MiB attachment round-trips byte-verified, open + bearer-token auth, wrong-bearer negative rejected) — the NA-0642 legacy-pull backward-compat guarantee held end-to-end, converted from analysis to artifact. Full `cargo test -p qsc` green (603 passed / 0 failed / 3 pre-existing ignored, exit 0). Dev-edge-only PROVEN (`cargo tree -p qsc -e normal` byte-identical before/after; the lock adds only the qsl-server SQLite stack on the dev edge). ZERO test-file or source change. The caveat below held exactly as filed (library constructor → `:memory:`; STORE_PATH never applied). NOTE: this PASS does NOT exercise the new durability/ack features — that coverage is ENG-0040 (still OWED, now unblocked: the durable server is in the dev-dep). As-built: `docs/governance/evidence/NA-0643_as_built.md`. Filed 2026-07-13; D578 forbade the bump in-lane (a pin bump is a DELIBERATE, SEPARATE step per the NA-0640 discipline); last-updated 2026-07-13
- The fact: `qsl/qsl-client/qsc/Cargo.toml` pins qsl-server at rev `19b9b02d` (dev-only, for `NA_0640_full_stack_e2e.rs`). NA-0642 merged qsl-server PR #61 (merge `8e4ea278`), so the pin no longer reflects qsl-server main.
- What the bump must prove: the e2e passes UNCHANGED against the new server — this is exactly the NA-0642 backward-compat guarantee (legacy pull byte-identical; guarded server-side by `na0642_backward_compat.rs`). One caveat the bump lane must handle: the NA-0642 server's LIBRARY constructors default to an in-memory store, so the in-process `start_qsl_server` harness needs NO change; if the harness ever moves to the binary, STORE_PATH becomes required.
- Recommended shape: a LITE qsl-protocol lane — one-line rev bump + `cargo test -p qsc` locally including the e2e (it does not run on PRs) + lock-delta proof (dev-edge only, per the NA-0640 discipline). Natural pairing: run it BEFORE or WITH ENG-0040 (the ack-client lane needs the new server in the dev-dep anyway).

### ENG-0042 — qsc receive: the commit-before-write seam — ratchet state commits durably BEFORE the app-payload write; a crash in that gap consumes the message key without persisting the plaintext — **NEW; filed 2026-07-14 by NA-0644 (D-1267; directive D580)**
- Severity: P3 (reliability edge; PRE-EXISTING — present in legacy delete-on-pull today, where the server has also already deleted the copy; bounded at ONE message per crash; NA-0644's lease mode narrows the whole client crash window down to exactly this seam and bounds its blast radius with a loud backstop)
- Status: open — filed 2026-07-14 (the HANDLING shipped with NA-0644; the seam itself did not move)
- The seam: in `receive_pull_and_write` a plain application message commits ratchet/session state fail-closed (`commit_unpack_state`) and THEN writes the plaintext via `write_atomic` (adjacent statements in `qsl/qsl-client/qsc/src/transport/mod.rs`). A crash between the two durably consumes the message key (forward secrecy: the envelope can never be decrypted again) while the payload never lands on disk. In LEASE mode the redelivered envelope hits `qsp_replay_reject` and the NA-0644 backstop ACKS it with the loud `ack_replay_unrecoverable` marker — the redelivery loop ends, bounded and visible (proven by the NA-0644 seam test). In LEGACY mode the behavior is unchanged (process-exit on the replay; the old server had already deleted, so the loss shape is identical to the pre-NA-0642 status quo).
- The real fix is NOT the backstop: it is reordering payload-write-before-state-commit (or a two-phase per-item persist), which interacts with the audited no-mutation-on-reject discipline (reject paths must not mutate state) and with the skipped-key store semantics — its own analysis + lane. Do NOT patch it casually inside an unrelated lane.
- Cross-reference: NA-0644 `tests/NA_0644_ack_client.rs::commit_before_write_seam_acked_loudly_no_poison_loop`; NA-0639/WF-0022 (`write_atomic` itself is crash-exercised and held); ENG-0040 (the lane that filed this); D-1267.

### ENG-0043 — qsc: flip the receive default to ack-lease once proven in operation — **NEW; filed 2026-07-14 by NA-0644 (D-1267; directive D580) — OWED FOLLOW-UP**
- Severity: P3 (reliability completion; the mechanism shipped OPT-IN at NA-0644 BY DESIGN — D580 explicitly forbade the default flip in-lane)
- Status: open — filed 2026-07-14
- The gap: delivery durability is only ambient once lease is the DEFAULT. NA-0644 shipped `--ack-mode lease` opt-in with the legacy path byte-identical. The flip is a deliberate later lane: flip the default (likely adding a config key for a persistent per-install choice), keep `--ack-mode legacy` as the explicit escape hatch, migrate the NA-0640 e2e consciously (exercise lease as the default while RETAINING an explicit legacy-path guard — never silently repurpose the compat proof), and restate the old-server fallback story for the flipped default.
- Cross-reference: ENG-0040 (DONE — the mechanism this flip turns on by default); D580 non-goals (the flip recorded as owed); the NA-0640 e2e (the compat guard that must be consciously migrated); ENG-0042 (the seam stays open regardless of the default).

### ENG-0044 — GUI phase: restore the three TUI-only vault/account-protection features co-deleted with the TUI (failed-unlock attempt-limit, idle autolock, account-destroy) — **DONE at NA-0658 (D-1281, directive D594, 2026-07-19)**
- Severity: P2 (security/account-protection UX; the features are GONE from the shipped surface until the GUI restores them — filed as ONE coherent item BY DESIGN so the obligation is hard to lose)
- Status: **DONE at NA-0658 (D-1281, directive D594, 2026-07-19)** — all three features restored as qsc LIBRARY surface per the operator-approved 2026-07-17 design refinement (DOC-PROG-004 step 4), in the new `src/vault/protection.rs` submodule the GUI consumes (the CLI does not re-expose them; its ingresses byte-identical, spot-check-proven): **(a)** attempt limiting returns REFINED — the guarded unlock (`unlock_guarded`/`unlock_guarded_at`) counts every wrong attempt into the restored persisted pair (`vault_security.txt` + `vault_unlock_failures.txt`, historical names/bounds, one additive last-failure-timestamp field) and enforces the accepted escalating-delay schedule DEFAULT-ON (failures 1–2 free, 5 s doubling capped at 300 s; restart-proof; clock-rollback fails safe), while wipe-at-N is a SEPARATE explicit opt-in (`wipe_after_failed_unlocks_arm/disarm/limit`, bounds 1..=100 restored; unarmed default proven safe by test; the historical tombstone wipe + the restored `QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS` marker on trigger); **(b)** the library half of idle autolock lands as the one-call `lock()` (R3 postconditions: process passphrase + unlocked flag + live `VaultSession` cleared/zeroized as ONE idempotent operation; the timer + minutes setting stay GUI-side per step 5; `tui.autolock.minutes` NOT restored); **(c)** `destroy_with_passphrase` returns with the historical validate-by-decrypt + keychain-removal + zero-overwrite-then-remove-then-fsync machinery PLUS the refined required `DestroyConfirmToken` (no single plain call can destroy) and the refined post-state (protection files cleared, process left locked). The tui_* event vocabulary stays deleted; the eight restore-vs-redesign deltas are enumerated in D-1281 and the as-built. See D-1281, `docs/governance/evidence/NA-0658_as_built.md`, `tests/NA-0658_eng0044_vault_protections_testplan.md`.
- The fact: NA-0645 retired the qsc TUI (D581, operator product decision: the GUI is the only end-user UI; the CLI stays a thin test-harness/operator surface). Three security/account-protection features whose LOGIC was TUI-only were co-deleted with it rather than re-homed (re-homing them would have mixed a deletion lane with a preserve-and-unit-test effort — D581 explicitly deletes + files): **(a) vault failed-unlock ATTEMPT-LIMIT** (wipe-on-repeated-failure: `parse_vault_attempt_limit_config`, `vault_security_state_load/store/clear_files`, `wipe_vault_file_best_effort`, the `vault_security.txt`/`vault_unlock_failures.txt` store files, the `QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS` marker; test `vault_attempt_limit.rs`); **(b) vault idle AUTOLOCK** (the TUI autolock timer + `tui.autolock.minutes` key; test `tui_autolock.rs`); **(c) ACCOUNT-DESTROY** (`vault::destroy_with_passphrase` — passphrase-gated best-effort cryptographic erase + keychain removal; test `tui_system_account_destroy.rs`).
- Git history preserves the implementations (deleted at NA-0645, base `9018ae4f` — recover with `git log --diff-filter=D` / the NA-0645 lane diff). The deleted tests document the contracts (wipe threshold parsing + counter semantics; autolock timing; destroy-refuses-wrong-passphrase + erase-then-remove ordering).
- The obligation: **the GUI phase must not close without restoring all three (or a recorded operator decision to drop each, feature by feature).** These are account-protection features for the DOC-PROG-003 §2 target user (self-hosted, high-trust orgs); silently losing them past the GUI phase would be a capability regression the docs still advertise.
- Cross-reference: D581 (the FILE decision + the KEEP/DELETE split); NA-0645 as-built (`docs/governance/evidence/NA-0645_as_built.md`); DOC-PROG-003 §3 T1 (the GUI build order this rides on); ENG-0037 (GUI-adjacent sealed-sender, same phase).

### ENG-0045 — demo: `scripts/demo/qsc_demo_local.sh` cannot deliver at current main (no vault unlock) and masks the failure (exit 0 + DEMO DONE with zero deliveries) — **NEW; filed 2026-07-15 by NA-0647 (D-1270; directive D583) — OWED FOLLOW-UP (the website RunDemos page links these instructions)**
- Severity: P2 (public-facing: the website's RunDemos "Local Demo" section instructs visitors to clone and run this script bare; at current main it silently produces nothing).
- Status: open — filed 2026-07-15
- The fact: the NA-0647 smoke-run at `ac7e850c` (the FIRST run since the TUI retirement + core extraction) showed every qsc invocation failing with `event=error code=vault_locked reason=explicit_unlock_required` — the script pre-dates the qsc explicit vault-unlock requirement (`--unlock-passphrase-file` / `--unlock-passphrase-env`; `bootstrap_unlock` in `qsl/qsl-client/qsc/src/main.rs`) and performs no vault/identity setup. Worse, the script MASKS the failure: every qsc call is `|| true`, the trailer prints `DEMO DONE` unconditionally, `normalized_counts.txt` records `status=ok`, and the exit code is 0 with `deliver_count=0` and empty receive directories. Full record: `docs/governance/evidence/NA-0647_as_built.md` §2; claim-matrix row WCM-110 (OUTDATED, MUST_FIX).
- The fix (its own lane; NOT the NA-0647 docs lane, which was run-only on this script): add a vault/identity bootstrap (or an explicit documented unlock step) to the script; make it FAIL LOUDLY when delivery does not occur (assert `deliver_count>0` / received-payload byte-match; propagate a nonzero exit); re-run at main; then re-touch WCM-110 and, if the instructions change shape, hand the RunDemos wording delta to the website lane (Phase B or later).
- Cross-reference: D583/NA-0647 (found + flagged); NA-0645/NA-0646 (the retirement/extraction the script was unrun across — NOT the cause; the CLI was proven byte-identical, and the failure is a pre-existing vault-gate mismatch); the website WEB-0006 Phase-A closeout (which requested this protocol-side verification).

### ENG-0046 — qsl-protocol: bump the qsl-server dev-dep pin past the NA-0652 server-info merge and re-run the NA-0640 full-stack e2e locally — **DONE at NA-0654 (D-1277, directive D590, 2026-07-17)**
- Severity: P2 (assurance currency; once qsl-server PR #62 merges, the pin at `8e4ea278` is knowingly STALE — the STANDARD suite's only real-server coverage exercises a pre-server-info relay)
- Status: **DONE at NA-0654 (D-1277, directive D590, 2026-07-17)** — the pin advanced `8e4ea278` → `3cc551a8d9cfd8f8f53d51e0b98d10a5dc62c944` (re-confirmed CURRENT qsl-server main HEAD at Phase 0 by fresh `git ls-remote`), with the landed lock delta EXACTLY the qsl_server rev advance (1/1 + 1/1 — the scoped-update method's five windows-sys edge flips were PROVEN pre-existing resolver drift by a zero-rev-change control and operator-ruled OUT of the lane; `cargo metadata --locked` exit 0). THE PROOF: the NA-0640 e2e GREEN UNCHANGED on the FIRST post-bump invocation (2/0, 118.47s, compile line proving `3cc551a8` built; zero test edits) and within the head-side full suite (2/0, 117.49s); full `cargo test -p qsc` = 412/0/1 across all 108 result sets exit 0 on BOTH SIDES of the bump (base-derived baseline at `e8bf93cc` = the repo-truth NA-0649 record EXACTLY; per-set normalized results sha256-IDENTICAL across the bump); `cargo tree -p qsc -e normal` byte-identical before/after. The optional server-info e2e probe was DECLINED per the operator scope line (client consumption = the GUI skeleton, DOC-PROG-004 step 5). Originally filed 2026-07-17 at the NA-0652 closeout; D588 forbade the bump in-lane (a pin bump is a DELIBERATE, SEPARATE step per the NA-0640 discipline). See D-1277, `docs/governance/evidence/NA-0654_as_built.md`.
- The fact: `qsl/qsl-client/qsc/Cargo.toml:34` pins qsl-server at rev `8e4ea278` (dev-only, for `NA_0640_full_stack_e2e.rs`). NA-0652 lands `GET /v1/server-info` as qsl-server PR #62 (branch `na-0652-server-info`, commit `8c5627e3`); at its merge the pin no longer reflects qsl-server main.
- What the bump must prove: the e2e passes UNCHANGED against the new server. The risk is lower than ENG-0041's bump: the NA-0652 route is ADDITIVE ONLY with `/v1/push`, `/v1/pull`, `/v1/pull/ack` handlers byte-untouched (proven by diff census + the 108/0 suite incl. every NA-0642 contract test), so the existing e2e surface is untouched by construction — the bump converts that from analysis to artifact. Same caveat as ENG-0041, unchanged: the library constructors default to an in-memory store, so the in-process `start_qsl_server` harness needs no change.
- Recommended shape: a LITE qsl-protocol lane — one-line rev bump to the PR #62 merge SHA + `cargo test -p qsc` locally including the e2e (it does not run on PRs) + lock-delta proof (dev-edge only). Optional cheap enrichment at bump time: the e2e MAY additionally probe `/v1/server-info` (the contract now exists in the pinned server), but that is a lane-scoping decision, not owed.
- Cross-reference: D588/NA-0652 (the lane that created the staleness; D-1275); qsl-server PR #62 + D-0012 + DOC-SRV-006 (the contract the bumped pin picks up); ENG-0041 (the executed precedent this repeats, incl. the dev-edge lock proof); ENG-0040/NA-0644 (the ack-client precedent for enriching e2e coverage after a pin bump); DOC-PROG-004 step 5 (the GUI skeleton, the eventual real consumer of server-info).
### ENG-0047 — qsl-desktop launch-state probe couples to the qsc vault store filename
- Severity: P3 (coupling/hygiene; no security delta — the probe is app-local, read-only, and fails toward the wizard whose vault-create path is refusal-guarded)
- Exact surfaces: qsl-desktop `src-tauri/src/paths.rs` (`QSC_VAULT_FILE_NAME = "vault.qsv"`, `vault_file()`); qsl-desktop `src-tauri/src/state.rs` `resolve_launch_state` (the S0/S1 discriminator); qsc `src/vault/mod.rs` `vault_path_resolved` (PRIVATE; resolves `$QSC_CONFIG_DIR/vault.qsv`)
- Description: the D595 F2 approval ruling (2026-07-19) made the GUI's S0-vs-S1 launch discriminator an app-level existence probe of `<app data dir>/qsc/vault.qsv`. The probe is deterministic (the GUI itself sets QSC_CONFIG_DIR before any qsc call), but the `vault.qsv` filename is qsc-format-internal and pub-invisible: if qsc ever renames or relocates its store, the GUI silently resolves S0 on a machine that HAS a vault — the wizard appears, and vault creation then refuses via the existing NA-0649 vault-exists error contract (fail-closed downstream, but a confusing surface).
- Remedy: a pub `vault_exists()` (or config-dir/store-path) probe in qsc — investigation residue R11's clean form — landed via a normal qsc surface lane and consumed by qsl-desktop at its next deliberate pin advance (the ENG-0041/D-1266 + ENG-0046/D-1277 bump-lane pattern). Per the operator's F2 ruling this successor is UNBLOCKED AND UNSCHEDULED.
- Status: open — filed 2026-07-19 at the NA-0659 closeout per the D595 F2 approval ruling

### ENG-0048 — qsl-desktop `destroy_vault` leaves app-level settings.json across the destroy boundary
- Severity: P3 (hygiene/consistency; NO secret involved — settings.json carries only autolock_minutes and the optional self_alias display label, both non-secret by the D-0003 skip-when-empty ruling)
- Exact surfaces: qsl-desktop `src-tauri/src/commands.rs` `destroy_vault` (the tokened core destroy only — touches no app file) vs `erase_all_impl` (removes the qsc dir AND `settings_file(data_dir)`); the D597 item-13 binding rule "no secret or prior-vault value may cross a destroy/erase boundary"
- Description: found by the NA-0661 item-13 trace. After a tokened DESTROY the app's `settings.json` persists on disk by landed D-0002 semantics, so the previous profile's alias and autolock preference resurface in the NEXT vault's session (Settings shows the old alias for a brand-new identity). The webview-side pathway is CLOSED by NA-0661 (full reload on completion; the wizard never pre-fills the alias), so what remains is disk persistence only. ERASE already removes the file — the two boundaries are inconsistent. Changing what destroy removes was out of NA-0661's scope (the "NO change to what destroy/erase DO" boundary).
- Remedy: the operator decides the semantics — either destroy also clears the app-level settings.json (one `fs::remove_file` in the destroy path, mirroring erase; a deliberate small lane since it changes what destroy DOES), or the persistence is declared intended (a fresh profile inheriting local display preferences) and recorded in the destroy copy. Either way the boundaries become consistent and documented.
- Status: open — filed 2026-07-19 by NA-0661 (D-1284; directive D597 item 13)

### WF-0024 — DOC-PROG-004 v0.2.0's "~15 min" autolock wording is SUPERSEDED by the D598 operator decision (default 60; 0 = never-auto-lock) — the roadmap-revision micro-lane is OWED — **NEW; filed 2026-07-20 by NA-0662 (D-1285; directive D598) — the REQUIRED closeout filing**
- Problem: DOC-PROG-004 v0.2.0 (the GUI phase roadmap) describes idle autolock as "~15 min". The operator's recorded round-3 decision (D598, approved 2026-07-19; landed by NA-0662 as the sanctioned settings.rs item-2 set) sets the default to 60 minutes with 0 VALID and meaning never-auto-lock (danger banner; the idle timer never fires at 0 — the BINDING never-fire guard). The roadmap doc now trails the landed, operator-decided behavior — exactly the WF-0018 docs-drift class, caught at the moment of divergence.
- Recommended change: a DOC-PROG-004 revision micro-lane (docs-only; the D592 rename/fidelity pattern) folding the autolock 60/0-never decision into the roadmap text (and any other status folds due at that revision). Explicitly OUT of NA-0662's scope by directive ("the DOC-PROG-004 edit itself is NOT this lane").
- Status: open — the doc edit awaits its own micro-lane; the SEMANTICS are landed and test-pinned in qsl-desktop (default-60 + zero-valid; D-0005).
- Originating/last lane: NA-0662 (D-1285; directive D598).
- Last-updated: 2026-07-20.

### ENG-0049 — qsc client TLS trust: no OS trust store, no explicit CA option, and certificate-verification failure indistinguishable from any other transport failure — **CLOSED AS PAID at NA-0663 (D-1286, directive D599, 2026-07-20)**
- Severity: P2 (deployability/diagnosability for the DOC-PROG-002 self-hosted niche; NO cryptographic weakness — the client failed CLOSED throughout, it simply could not be told what to trust)
- Exact surfaces: `qsl/qsl-client/qsc/Cargo.toml` reqwest feature line (baked-in webpki roots only); the eight `HttpClient::new()` sites (`transport/mod.rs` push/pull/ack + `attachments/mod.rs` ×5); the push/pull/ack error arms collapsing every send failure into `relay_inbox_push_failed` / `relay_inbox_pull_failed` / `relay_ack_failed`
- Description: qsc trusted ONLY the certificate authorities compiled into the binary. Installing a private CA on the client machine the standard way could not make qsc trust a self-hosted relay, `SSL_CERT_FILE`/`SSL_CERT_DIR` had no effect, and there was no explicit CA-file option. The observable was the opaque `relay_inbox_push_failed`, byte-identical to "relay unreachable" and "relay rejected the request", so an operator could not tell a trust problem from a connectivity problem. Found operationally on 2026-07-19 against the inspiron LAN relay (private CA); the working stock path at the time was an ssh tunnel. The gap blocked DOC-PROG-004 step 5's acceptance ("a fresh machine onboards to a live relay") and DOC-PROG-004:182's required "certificate not trusted" taxonomy entry, which GUI slice B's Server pane cannot render truthfully without a typed value.
- Remedy: PAID IN-LANE by NA-0663 in three parts — (1) the OS trust store honored via the reqwest `rustls-tls-native-roots` feature held in UNION with the retained `rustls-tls` webpki roots, so nothing previously trusted stops being trusted and `SSL_CERT_FILE`/`SSL_CERT_DIR` behave as standard; (2) an ADDITIVE explicit CA-file option resolving env `QSC_RELAY_CA_FILE` → env `RELAY_CA_FILE` → vault secret `tui.relay.ca_file`, fail-closed on a configured-but-unusable CA, reachable as pub GUI-facing library surface and via the additive CLI verbs `relay ca-set` / `ca-clear` / `ca-show`; (3) a DISTINGUISHABLE typed outcome `relay_tls_untrusted`, detected BY VALUE on `rustls::Error::InvalidCertificate` rather than by matched substring, distinct from unreachable, DNS failure, timeout and `relay_unauthorized`, plus the `relay_ca_file_missing`/`_unreadable`/`_invalid` configuration trio. No insecure-skip-verify / accept-any-certificate / accept-invalid-certificate path was added in any form, including tests; a needle scan is pinned as an executable test with no exemption list.
- Status: **CLOSED AS PAID at NA-0663** (D-1286 implementation, merge `b2dc23bf`; D-1287 closeout) — filed and paid in the same lane per the WF-0021/WF-0023 precedent. Provenance chain: the 2026-07-19 inspiron private-CA deployment finding → QSL-DIR-2026-07-20-599 (D599, approved 2026-07-20, sha256 `925b56cd…`) → D-1286. See `docs/governance/evidence/NA-0663_as_built.md`, `tests/NA-0663_qsc_client_tls_trust_testplan.md`.

### ENG-0050 — the fmt/clippy validation defaults are a standing trap: `cargo fmt --all` rewrites 45 files including FORBIDDEN paths, and `clippy -D warnings` is RED at base
- Severity: P2 (process/scope-safety; NO runtime or security impact — but the failure mode is a silent scope breach and byte-identity violation in any lane that follows the stated validation defaults)
- Exact surfaces: the validation-default line carried by lane directives ("fmt; clippy per the validation defaults"); `.github/workflows/**` (which contains NO `cargo fmt` job and NO `cargo clippy` job — grep-proven); the base-tree rustfmt drift, e.g. `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs:320` and `qsl/qsl-client/qsc/src/attachments/mod.rs:1721`; the base-tree clippy findings `result_unit_err` on `qsl/qsl-client/qsc/src/lib.rs` `push` and `bounded_retry`
- Description: two independent defects in the same default, both hit by NA-0663. (a) The repository is NOT rustfmt-clean at base, so running `cargo fmt --all` — the literal validation default — reformatted **45 files** in one command, including FORBIDDEN paths (`lib.rs`, `vault/`, `handshake/`, `contacts/`, `timeline/`, `adversarial/`) and `qsl/qsl-client/qsc/tests/NA_0640_full_stack_e2e.rs`. That is simultaneously a scope breach and a byte-identity STOP, produced by following the directive exactly. NA-0663 caught it before commit and fully reverted it (restoring the in-scope files to base and re-applying the lane's edits surgically, with byte-identity re-verified), but detection was luck of ordering, not a gate — no CI check would have caught it, because there is no fmt gate at all. (b) `cargo clippy -- -D warnings` FAILS AT BASE with pre-existing `result_unit_err` findings in a file NA-0663 leaves byte-identical, so a lane cannot distinguish "my change broke clippy" from "clippy was already red"; this is related in kind to ENG-0032, which records a different pre-existing clippy-1.95 `needless_borrow` failure in `apps/qshield-cli`. Directives continue to list both tools as validation defaults, so every future lane runs the same trap.
- Recommended change: pick ONE of two coherent end states rather than leaving the default aspirational — either (i) fix the drift and ADD the gates (`cargo fmt --all -- --check` and `cargo clippy --locked -- -D warnings` as REQUIRED checks), so the defaults become enforceable and meaningful; or (ii) STRIKE both from the validation defaults and replace them with a narrower, honest instruction (format only the touched region; no repo-wide clippy claim). Until either lands, directives should say "format only the touched region" so no lane repeats the 45-file incident. NA-0651's owed rustfmt-drift micro-lane is the natural home for (i). Cross-reference ENG-0032 (the sibling pre-existing clippy-1.95 lint debt in `apps/**`).
- Status: open — filed 2026-07-20 by NA-0663 (D-1287; provenance: the NA-0663 response §16.2 and §16.3). FILING ONLY; not executed in-lane.

### ENG-0051 — qsc sends the relay bearer token unconditionally, including to a relay advertising OPEN auth mode
- Severity: P2 (credential hygiene / unnecessary secret exposure; NO authentication bypass and NO confidentiality break in transit — the token still travels under TLS, and an open relay grants no more access with it than without)
- Exact surfaces: `qsl/qsl-client/qsc/src/transport/mod.rs` — the three `Authorization: Bearer` attachment points in `relay_inbox_push`, `relay_inbox_pull_mode` and `relay_inbox_ack`, each gated ONLY on `relay_auth_token()` returning `Some`; the `GET /v1/server-info` capability landed server-side at NA-0652 (D-1275; qsl-server D-0012, pin `3cc551a8`), which qsc does NOT consume
- Description: operational verification against the inspiron relay on 2026-07-20 established that on an OPEN relay the auth gate returns true unconditionally, so a WRONG bearer token produces a response byte-identical to sending no token — the relay silently ignores the credential. Independently source-proven on the client side at NA-0663: all three relay operations attach the `Authorization` header whenever a token is configured, with ZERO consultation of any advertised auth mode (grep for `server_info`/`auth_mode` across `qsc/src` returns 0). qsc therefore hands a secret to a server that has no use for it and will not tell it so. On self-hosted deployments — the DOC-PROG-002 niche — that token may be logged by a reverse proxy, retained in server-side request logs, or reused across services by an operator who assumes it was needed. The mechanism to know better already exists: the server-info capability was landed at NA-0652, and ENG-0046's closeout records that client consumption of it was DECLINED at the time and deferred ("client consumption = the GUI skeleton, DOC-PROG-004 step 5"), which is exactly why the token is attached blind today.
- Recommended change: when the relay advertises `auth.mode` of `"open"`, SUPPRESS the `Authorization` header rather than sending a credential the server ignores. Client-side auth-header suppression ONLY — no relay change, no protocol/wire change, no change to how the token is stored or resolved. **DELIBERATELY LEFT OPEN for the future directive, as design rulings rather than filing decisions:** (a) how the mode is learned — a `GET /v1/server-info` probe, cached per endpoint or performed per call, and the cost/staleness trade-off between them; (b) what happens when the mode is UNKNOWN or unprobed (send, suppress, or fail closed) — noting that "suppress on unknown" risks breaking authenticated relays that are merely unreachable for probing, while "send on unknown" preserves today's behavior and therefore today's exposure. Scope note: adjacent to but NOT inside D599/NA-0663, which changed only what the client TRUSTS, never what it SENDS.
- Sequencing: intended to execute BEFORE GUI slice B is drafted, so the Server pane is designed against the settled send-side behavior rather than retrofitted to it.
- Status: open — filed 2026-07-20 by NA-0663 (D-1287; provenance: operator operational verification against the inspiron relay 2026-07-20, plus the NA-0663 source verification of the three unconditional attachment points). FILING ONLY; not executed in-lane. Cross-reference ENG-0046 (where client consumption of server-info was deferred) and D-1275/NA-0652 (where the capability landed).

### ENG-0052 — the push-only full suites go unexercised across governance-heavy windows, then fire on the first code push — and the macOS variant does not fit its own 120-minute ceiling
- Severity: P2 (CI assurance currency + release-gate reliability; NO runtime or security impact — but main can go RED on a change whose own acceptance is fully green, and the timeout ceiling is currently unfalsifiable because no recent baseline exists)
- Exact surfaces: `.github/workflows/ci.yml:372` `qsc-linux-full-suite`, gated `if: github.event_name != 'pull_request' && needs.classify.outputs.docs_only != 'true'`, with **NO `timeout-minutes` configured** (so it inherits the 360-minute GitHub default); `.github/workflows/macos-build.yml:100` `macos-qsc-full-serial`, same gating shape, `timeout-minutes: 120` at `macos-build.yml:104`; `.github/workflows/public-ci.yml:517` `public-safety`, the watchdog that fails main when either push-only full suite is red
- Description: both full suites are excluded from pull requests BY DESIGN (they sit outside the PR critical path) and are additionally skipped on any docs-only push. In a governance-heavy window — promotions, closeouts, docs lanes — neither suite executes at all, so the CI configuration that guards main goes untested for many consecutive pushes. The first CODE push then exercises them cold. That happened at NA-0663 (merge `b2dc23bf`, 2026-07-20): `macos-qsc-full-serial` ran 20:16:56 → 22:17:14 = **2h00m18s** and was CANCELLED by its 120-minute ceiling — NOT a designed supersession, since `origin/main` had not advanced past the merge commit — and `public-safety` then failed as its designed consequence at ITER=339/390 on observing `conclusion=cancelled`, turning main RED. Corroborating evidence that the suites were genuinely dormant: recent `macos-build` runs on main completed in **24s / 21s / 29s / 22s / 27s** (`e5313fa3`, `83b6b4a4`, `17bba8bc`, `1a3d4d48`, `a5af1b49`), durations only possible with the full-serial job skipped; the same three most recent pushes show `qsc-linux-full-suite` skipped as well.
- Consequence: **no recent macOS baseline exists**, so the 120-minute ceiling cannot be set from evidence without a deliberate measurement — a manual `macos-build` run on a pre-change parent commit is required to learn what the suite actually costs. Until that measurement exists, any adjustment to the ceiling is a guess, and the attribution question ("did the change tip it, or was it already over?") is unanswerable from repository history alone.
- Recommended change: (a) obtain the measurement first — run `macos-build` manually on a parent commit to establish the pre-change duration; (b) then set `timeout-minutes` on `macos-qsc-full-serial` from that measured reality, or narrow what the macOS serial job runs, rather than raising the ceiling blindly; (c) give `qsc-linux-full-suite` an explicit `timeout-minutes` instead of inheriting the 360-minute default, so an overrun there surfaces as a bounded failure rather than a six-hour hang; (d) consider a periodic (scheduled) exercise of both push-only suites so the configuration cannot rot silently across a governance-heavy window — the cost is a known-good cadence rather than discovering the ceiling on a merge. NOT recommended: re-running `public-safety` to chase green; it is not flaky and correctly reports a red upstream suite.
- **Masking note (added 2026-07-21, NA-0663 closeout merge `dd2918e7`):** the `docs_only` gating means **governance merges look GREEN regardless of ceiling state**. Demonstrated on the closeout merge for this very filing: `macos-build` completed SUCCESS in **32 seconds** with `macos-qsc-full-serial` **skipped**, and `public-ci` passed — because `public-safety` fails on push-only suites that are RED, not on suites that are SKIPPED. The ceiling was no less broken after that merge than before it; it simply was not exercised. **Consequence: the red reappears only on the next push touching `src/`, and anyone reading main's recent history would conclude the problem is fixed. It is not.** Green main-push history is therefore NOT evidence that this item is resolved, and must not be read as such.
- **Ordering problem — RESOLVED (added 2026-07-21; resolution recorded 2026-07-21 by NA-0664/D600):** the next code-touching lane would INHERIT THE RED, whatever that lane was about, and the likely candidate was the ENG-0053 instrumentation lane — which would have hit the 120-minute macOS ceiling BEFORE it could report on the per-operation cost that may be causing the overrun. That ordering was planned for at directive time rather than discovered mid-lane. **The resolution chosen, of the three options originally enumerated, was the first: raise the ceiling as a deliberate, separately-justified PRECONDITION of the instrumentation lane.** NA-0664 (D600) is CEILING-FIRST — its PR-1 carries the ceiling change alone and, because `.github/workflows/*` classifies `docs_only=false` (`scripts/ci/classify_ci_scope.sh:20,61-62`), **that PR's own merge is the first genuine exercise of `macos-qsc-full-serial` under the corrected ceiling** — the capacity correction is tested by the very merge that lands it. The masking effect was demonstrated a SECOND time on the NA-0664 seating merge `32215b75` (`macos-build` SUCCESS in **26 s**, `macos-qsc-full-serial` `completed/skipped`, start == end), which is why the ordering had to be deliberate. Cross-reference ENG-0053.
- **THERE WERE THREE UNMEASURED BUDGETS, NOT TWO — the strongest available evidence for this item's thesis (added 2026-07-21 by NA-0664/D-1289):** this filing named **two** fixed CI budgets nobody had re-derived from measurement — `macos-qsc-full-serial`'s 120-minute ceiling and `qsc-linux-full-suite`'s inherited 360-minute default. **A THIRD existed and was named nowhere**: `public-safety`'s `--max-iterations 390` poll cap in `.github/workflows/public-ci.yml` (~130 min at a 20 s interval). It sat outside this filing, outside the D600 directive, and outside the drafter's analysis — and it **became the binding constraint the moment the first two were corrected**. On the NA-0664 PR-1 merge `ca6897fc` both full suites PASSED (`macos-qsc-full-serial` 132m15s, `qsc-linux-full-suite` 157m45s) while `public-safety` exhausted **390/390** iterations at 19:13:03Z and failed with `qsc-linux-full-suite` still `in_progress` — that suite then succeeded **20m52s later**. **Main went RED on a merge where nothing was broken.** Correcting two budgets simply moved the failure to the one nobody had counted. It surfaced **only** because the suites had been dormant behind the `docs_only` gate: the contradiction between a 240-minute Linux ceiling and a ~130-minute watchdog had been latent and unobservable for the whole governance-heavy window this item describes. **Remedied at D-1289 by DERIVING the watchdog budget from the suite ceilings at run time** rather than bumping the literal — a hardcoded count kept in manual sync with two independent ceilings is this same defect one more time.
- **OPEN DESIGN QUESTION — RECORDED, NOT ACTED ON (added 2026-07-21 by NA-0664/D-1289, operator-directed):** when the watchdog exhausts its budget **without reaching a verdict**, is FAILING the right terminal behaviour? It cannot confirm green, so failing conservative is defensible — but it produces a **FALSE RED when the suites actually pass**, which is exactly what happened at `ca6897fc`. Whether budget exhaustion should be **distinguishable** from a genuine suite failure — a distinct conclusion, a distinct marker, or a distinct check — is a design question **beyond the D-1289 fix**, which only ensures the budget is large enough that exhaustion should not occur. **Not acted on in NA-0664.** Carry to a successor lane.
- **DISPOSITION AT NA-0664 (added 2026-07-21 by D-1290) — CLAUSES (a), (b), (c) DISCHARGED; CLAUSE (d) REMAINS OWED.**
  - **(a) obtain the measurement first — DISCHARGED.** The pre-lane macOS baseline was measured on a parent commit by `workflow_dispatch` (run 29785894339 on `e5313fa3`): **105m52s** job / 103m36s step, i.e. **88.2% of the old 120-minute ceiling**. That figure is what made the ceiling falsifiable for the first time.
  - **(b) set the ceiling from measured reality — DISCHARGED at D-1288.** `macos-qsc-full-serial` `timeout-minutes` 120 → **180** (1.70x the measured pre-lane runtime). **VERIFIED IN FORCE BY OBSERVATION:** on the D-1288 merge `ca6897fc` the job ran **132m15s** (step 130m00s) and **PASSED** — **73.5% of the 180 ceiling, 47m45s headroom**. Against the old 120 ceiling that same run would have been **110.2% utilised** (13m45s over) and cancelled again; against a 150 ceiling it would have been **88.2% utilised** — numerically the identical thin margin this item was filed about, which is why 150 was rejected at F1.
  - **(c) give `qsc-linux-full-suite` an explicit `timeout-minutes` — DISCHARGED at D-1288.** `timeout-minutes: 240` replaces the inherited 360-minute GitHub default; observed **157m45s** on the same merge = **65.7% utilised**.
  - **(d) a periodic (scheduled) exercise of both push-only suites — NOT DONE, EXPLICITLY OWED.** It was ruled out of NA-0664's scope at directive time (any scheduled/periodic trigger was on the D600 FORBIDDEN list). **The masking note above is unretired by this lane's work:** governance merges still skip both suites, so main's green history still is not evidence about ceiling health. This clause is the only remaining defense against that, and it is still absent. Carry to a successor lane.
  - **The extrapolation that justified 180 UNDERSHOT, and that is recorded as a result, not a footnote.** Applying Linux's +17.3% to the macOS baseline projected a 121m31s step; the observed step was **130m00s** — the projection was **8m29s low**. The cross-platform extrapolation was directionally right and quantitatively optimistic. **Absolute added cost was close on both platforms** — macOS step +26m24s vs Linux job +23m13s, **ratio 1.14** — consistent with a compute-bound, platform-invariant regression rather than an I/O-bound one. **A future reader must not treat a cross-platform percentage extrapolation as a measurement; this lane's own use of one was off by 8m29s in the unsafe direction.**
- **THE DEFECT CLASS HAS FIVE INSTANCES, NOT TWO AND NOT THREE (added 2026-07-21 by D-1290, operator-ruled).** This filing named **two** fixed CI budgets that nothing re-derives. The lane found **five**, and the count is recorded in full because the thesis is broader than the filing states:
  1. **`macos-qsc-full-serial`'s 120-minute ceiling** — a literal, set once, never re-derived from a measurement (the original filing).
  2. **`qsc-linux-full-suite`'s inherited 360-minute GitHub default** — a budget nobody chose at all (the original filing).
  3. **`public-safety`'s `--max-iterations 390`** — a literal in a THIRD workflow file, logically derived from the ceilings it waits on but expressed as a bare constant, with nothing computing or asserting it. **It became binding the instant the first two were corrected** and turned main RED on a merge where nothing was broken (D-1289).
  4. **The red-main repair profile `send_commit_vault_mock_provider_retired`** — the only encoded profile in `scripts/ci/public_safety_gate.py`, an incident-specific hardcoded value that does not generalize, so a red main from any other cause has no matching profile (filed as **ENG-0059**).
  5. **The sanctioned self-repair bootstrap's advisories-only trigger** — `validate_self_repair_bootstrap_pr` requires main to be red **because `advisories` is failing**. It is not a numeric constant, which is why it was flagged as a candidate rather than counted silently; **the operator RULED it IN as the fifth instance**. It is the same shape as the other four: a hardcoded assumption encoding the one incident that motivated it — here, an assumption about *why* main would be red (filed as **ENG-0059**).
  **What the five have in common:** each is a FIXED VALUE ENCODING A MOMENT IN TIME THAT NOTHING RE-DERIVES. Instances 4 and 5 show the class is not confined to numbers or to CI timing — an encoded assumption about a failure's *cause* rots exactly the same way a timeout literal does. **Only instance 3 has been given a derivation (D-1289); instances 1 and 2 were corrected to better literals, not to derived values, and remain literals today.** That is a deliberate, recorded limitation of this lane's remedy, not an oversight.
- Status: **PARTIALLY DISCHARGED** — filed 2026-07-20 by NA-0663 (D-1287; provenance: the NA-0663 post-merge STOP response §6, and the operator ruling of 2026-07-20 directing this filing). **Clauses (a), (b), (c) discharged at NA-0664 (D-1288); clause (d) — the periodic scheduled exercise — REMAINS OPEN and is the reason this item is not closed.** The open design question below (exhaustion vs genuine failure) is also unresolved. Cross-reference **ENG-0050** (the fmt/clippy validation-default hazard): both are CI-configuration debt surfaced by this same lane — one where the stated gates do not exist, one where the gates that do exist had not run. Cross-reference **ENG-0059** (instances 4 and 5 of this defect class, and the two unusable red-main escape hatches).

### ENG-0053 — `relay_ca_file()` performs a vault read on EVERY client construction: ~350-400 ms per relay operation, compounding as the vault store grows — **⚠ TITLE FIGURE CORRECTED AT NA-0664: the ~350-400 ms is a DEBUG-PROFILE cost. RELEASE is ~18 ms per `secret_get`, ~95-97% of it Argon2id. See the MEASURED VERDICT below before citing this item.**
- Severity: P2 (PRODUCT-FACING client efficiency; NO security or correctness impact — the trust decision itself is unaffected, and the cost is paid in latency, not in safety)
- Exact surfaces: `qsl/qsl-client/qsc/src/transport/mod.rs` — `relay_ca_file()` falling through to `relay_ca_file_from_account_secret()` -> `vault::secret_get(TUI_RELAY_CA_FILE_SECRET_KEY)`, invoked from `relay_http_client()`, which replaced all EIGHT former `HttpClient::new()` sites (transport push/pull/ack + attachments x5); `qsl/qsl-client/qsc/src/vault/mod.rs` `secret_get` (the suspected true locus, NOT modified by NA-0663)
- Description: `relay_ca_file()` falls through to `relay_ca_file_from_account_secret()` -> `vault::secret_get` on EVERY `relay_http_client()` construction whenever `QSC_RELAY_CA_FILE` and `RELAY_CA_FILE` are unset — which is the default. Measured cost: **+311 / +329 / +372 ms per relay operation** across three interleaved samples, 180 operations, with a confound that runs CONSERVATIVE (side B additionally parses a CA file, making the delta an UNDER-statement). Scaling is linear trending SUPER-LINEAR, per-op delta rising monotonically with N and no meaningful fixed offset, consistent with a full store read-and-decrypt per lookup rather than a keyed access — meaning the cost COMPOUNDS as the vault store grows. This is a PRODUCT-FACING efficiency question, not only a CI one: every relay operation in the shipped client pays it, and it worsens over a user's lifetime.
- Status of the finding: **ESTABLISHED as the LOCATION of the cost. NOT established as the root cause** — a `vault::secret_get` should cost tens of milliseconds, not ~350-400, so it stands as a PROXY for something below it. Untested candidates, none claimed: Argon2id re-derivation per `secret_get`; full store read plus AEAD decrypt per lookup rather than a cached handle; lock contention or a per-access file re-open.
- Recommended change / scope for the future lane: **INSTRUMENT FIRST** — time `relay_http_client()` and `vault::secret_get` directly and determine whether a key derivation or full-store decrypt occurs per access — **THEN** choose a remedy. Do NOT assume the fix is caching the CA path: if the root cause is in `vault::secret_get`, that cost is being paid by every other secret accessor too, and the CA path is merely where this lane made it visible.
- Method note (why the finding is trustworthy): the discriminator used the SAME head binary on both sides so the vault read was the only intended variable; both sides were verified to reach `relay_http_client()` and emit `send_attempt ok=true` + `send_commit` BEFORE any timing was recorded; base/head were interleaved iteration-by-iteration because per-send state growth would otherwise favour whichever side ran first; three N values (15/30/45) were used so the per-op term and any fixed offset could be separated. Two earlier hypotheses were tested and REFUTED by the same method (Argon2id-per-subprocess under serial execution; trust-store loading per construction), which is what narrowed the search to this path.
- Cross-reference: **ENG-0052** (the push-only full-suite exercise gap / macOS 120-minute ceiling). The two are related but DISTINCT — one is CI configuration debt, the other is a client efficiency regression. Neither remedy should be chosen before the other is understood.
- **Inherited-red warning — CLOSED (added 2026-07-21; closed 2026-07-21 by NA-0664/D600):** the instrumentation lane contemplated here WAS the lane that would inherit the unresolved macOS ceiling from ENG-0052, hitting the 120-minute timeout BEFORE it could report on the per-operation cost. That was a genuine circular dependency — ENG-0052's remedy was being deferred pending ENG-0053's result, while ENG-0053's lane could not get a green main-push until ENG-0052's ceiling was addressed. **The dependency was broken deliberately at directive time, as required: NA-0664 is CEILING-FIRST**, raising the ceiling in a PR-1 that carries no source change at all, so the corrected ceiling is in force before any measurement work depends on it. The instrumentation half was additionally ruled SHIP-NOTHING and runs entirely locally, so its measurement never depended on a green main-push in the first place. Cross-reference ENG-0052's ordering note.
- **Independent CI corroboration on a second platform (added 2026-07-21 by NA-0664/D600):** this finding rested on local probes. The Linux full suite provides an INDEPENDENT pre/post pair from CI, and it is the ONLY complete pre/post measurement of the regression that exists — Linux completed on BOTH sides precisely because it has no `timeout-minutes` ceiling, where macOS was cancelled and so could not produce one. `qsc-linux-full-suite` ran **133m55s** at `ba4099bd` (NA-0658, pre-regression; run 29678251282) and **157m08s** at `b2dc23bf` (NA-0663, post-regression; run 29775446132): **+23m13s = +17.3%**. Applying that factor to the macOS pre-lane baseline of 105m52s projects a post-regression macOS duration of **~124m**, coherent with the observed cancellation while still running at 118m18s. **This projection is a cross-platform EXTRAPOLATION, not a measurement**, and must not be cited as one.
- **⚠ MEASURED VERDICT — NA-0664 (D-1290, 2026-07-21). READ THIS BEFORE CITING ANY FIGURE ABOVE.**
  - **THE HEADLINE CORRECTION: `~350-400 ms` IS A DEBUG-PROFILE MEASUREMENT. IT IS NOT THE PRODUCT COST.** The release cost of one `vault::secret_get` is **~18 ms**, of which **~95-97% is Argon2id**. The filing's product-facing severity was **OVERSTATED — by the filing, by the operator's ranking, and by the measuring executor's own hypothesis. NO READER MAY CARRY 350 ms INTO A PRODUCT DECISION.** The debug figure remains the CORRECT figure for CI cost, because CI runs tests in debug (see below) — the number is not wrong, its ATTRIBUTION to the shipped client was.
  - **How the debug profile was confirmed — three independent checks.** (1) The NA-0663 probe binary `qsc.BASE.e5313fa3` is **89,332,792 bytes with `.debug_info` present**; on the same box a debug `qsc` is 92,227,392 B and a release `qsc` is 8,850,832 B — a **10x** separation, and the probe sits with debug. (2) **Both CI suites run their TESTS in debug:** `macos-qsc-full-serial` runs `cargo test -p qsc --locked` with **no `--release`**; `qsc-linux-full-suite` *builds* release then runs `cargo test -p qsc --locked` — **the test is debug**. (3) The ratio closes the gap with nothing left over: debug/release wall-clock = **401.310 / 18.137 = 22.1x**, against an observed gap of ~375/18.1 ≈ **21x**. **Debug alone accounts for it; no second explanation is required.**
  - **THIS CLOSES THE 20x GAP that the apportionment run had left open.** The apportionment artifact recorded an unresolved discrepancy — a release `secret_get` at ~18 ms could not explain NA-0663's measured +311/+329/+372 ms per relay operation — and explicitly withheld a verdict pending its resolution. **That gap is now CLOSED by the debug-profile finding, and the apportionment artifact's "OPEN PROBLEM" section is SUPERSEDED.** Of its three candidate explanations, candidate 1 (non-release probe binary) is CONFIRMED; candidates 2 and 3 are rendered UNNECESSARY, **not refuted** — see the owed measurement below.
  - **NO CEILING CHANGE FOLLOWS FROM THIS VERDICT.** The 180/240 ceilings were derived from CI wall-clock measurements (105m52s pre-lane, 132m15s observed, 157m45s Linux), all of which are debug-profile realities. They remain correct as set.
  - **FOUR-BUCKET APPORTIONMENT — RELEASE, `key_source = 1` (passphrase), REPS=12, median.** Two store sizes so the fixed floor separates from the growth term:

    | bucket | Regime A (153 B, near-empty) | Regime B (414,592 B, 4000 msgs) |
    |---|---|---|
    | wall-clock `secret_get` | **18.137 ms** | **18.554 ms** |
    | (a) file read | 0.014 ms (0.1%) | 0.054 ms (0.3%) |
    | **(b) Argon2id** | **17.675 ms (97.4%)** | **17.703 ms (95.4%)** |
    | (c) AEAD decrypt | 0.004 ms (0.0%) | 0.289 ms (1.6%) |
    | (d-outer) payload parse | 0.001 ms (0.0%) | 0.759 ms (4.1%) |
    | **attributed** | **97.6%** | **101.4%** |
    | (d-inner) timeline parse | 0.000 ms | **2.009 ms** — paid ADDITIONALLY by `timeline_store_load`, outside `secret_get` |

  - **FIXED FLOOR vs GROWTH TERM, STATED SEPARATELY (the acceptance requirement).** **FLOOR = Argon2id ≈ 17.7 ms, and it is FLAT** — 17.675 → 17.703 ms across a **2700x** store-size increase. **GROWTH = (c)+(d-outer) = 0.005 → 1.048 ms** across that same 2700x. At 4000 messages the growth term is **~6% of the call**. The filing's "compounding as the vault store grows" framing is **real but small at today's sizes** and is the second-order term, not the first.
  - **THE (c)/(d) SPLIT — REQUIRED, AND IT IS NOT EVEN.** At Regime B, release: **(c) AEAD 0.289 ms vs (d-outer) 0.759 ms — (d) outweighs (c) by ~2.6x.** The double-JSON finding is confirmed: deserialization, not decryption, is the larger half of the growth term. **(d-inner) at 2.009 ms exceeds (c)+(d-outer) combined** and has a DIFFERENT remedy — only removing the timeline from the vault addresses it.
  - **`key_source` STATED (required):** all figures are **`key_source = 1`** (passphrase, full Argon2id). **They do not generalize to `key_source = 2`** (keychain), where `derive_runtime_key` calls `keychain_load_key` and runs **no KDF at all** — bucket (b) ≈ 0 there, and with it ~95-97% of the measured cost. **The successor remedy differs between the two machines and must not be scoped from these numbers alone.**
  - **COUNTER VALIDATION (required before any counter-derived claim) — PASSED.** 12 `secret_get` calls yielded `kdf=+12 reads=+12 decrypts=+12` in every run, both regimes, both profiles. Counters agreed exactly with call count; **timing never had to override them.** Recorded because `perf_snapshot()` had **zero prior consumers** in `src/` or `tests/`, so its counters were plausible rather than proven before this lane, and because `PERF_VAULT_FILE_READS` increments **before** its `fs::read` and therefore counts **attempts, not successes** — a distinction that did not bite here (no read failed) but will if a future consumer measures an error path.
  - **A HYPOTHESIS WAS REFUTED, AND IT WAS THE MEASURING EXECUTOR'S OWN.** The prediction was that the store-size-dependent (c)+(d) terms would already dominate. **They do not — Argon2id dominates in both regimes and both profiles.** Recorded because the successor lane would otherwise inherit the wrong framing from this item's original text.
  - **WRITE PATH, MEASURED (it had never been measured; buckets (e)/(f)/(g)).** Release, `key_source = 1`: `secret_set` costs **20.014 ms** (empty) / **22.142 ms** (414 KB) — i.e. the write adds **~2.0-2.6 ms** over a read, **dominated by the atomic whole-file write at ~1.9-2.0 ms, which is FLAT with store size** (2.001 ms at 153 B → 1.899 ms at 414 KB): it is **fsync-bound, not size-bound**. **An operator prediction that the whole-file write would become significant at scale is CORRECTED: store size does not punish the write path** at these scales. The undercount concern behind that prediction was RIGHT; its ATTRIBUTION was wrong — the missing cost was **(e) serialize, (f) encrypt, and the inner timeline serialize**. **ASSUMPTION, NOT MEASUREMENT: extrapolating (g) as flat out to 5 MB is UNVERIFIED** — a successor wanting a precise 5 MB figure must measure it there rather than inherit this.
  - **CORRECTED APPEND ARITHMETIC.** `timeline_append_entry` = `timeline_store_load` + `timeline_store_save`, and it performs **TWO Argon2id derivations**, 2 outer parses, 1 inner parse, 1 inner serialize, and one each of (e)/(f)/(g). **Measured at 414 KB: ~43.2 ms** (load 20.563 + save 22.682) — an earlier ~38-40 ms estimate was an undercount. **Extrapolated to ~5 MB / ~50,000 messages: ~103-105 ms**, revised up from an earlier ~85 ms.
  - **⚠ ARITHMETIC CORRECTION THAT CHANGES THE CONCLUSION — Argon2id is 81.9% of an append at 414 KB, NOT ~42%.** The erroneous figure divided **ONE** derivation (17.703) by the append total (43.245) = 40.9%; **an append performs TWO** (17.703 x 2 = 35.406 / 43.245 = **81.9%**). At ~5 MB the share falls to **~34%** (that figure was always correct). **At today's realistic store sizes Argon2id does not merely lead an append — it DOMINATES it at ~82%.** This materially strengthens the envelope-encryption option below and must not be carried forward in its erroneous form.
- **AMENDED FRAMING — WHAT THIS ITEM IS ACTUALLY ABOUT (added 2026-07-21 by D-1290, operator-ruled).**
  - **(i) THE FILING NAMED THE CALLER, NOT THE DEFECT.** The defect is **`vault::secret_get`'s cold-start-as-hot-path architecture**, and **every secret accessor in the tree pays it** — not only `relay_ca_file()`. There are **16 `secret_get` call sites tree-wide** (transport 4, contacts 3, identity 2, protocol_state 2, timeline 1, attachments 1, handshake 1, lib 1). **THIS COUNT IS STATIC INSPECTION, NOT MEASUREMENT** — see the owed item below. **NA-0663 did not CAUSE this cost; it EXPOSED it** by being the first caller to put a cold-start API on a hot path.
  - **(ii) The four-bucket apportionment above, with the (c)/(d) split and the floor-vs-growth separation, IS the deliverable the successor needs** — because the two costs have **different remedies** (a cached derived key vs a cached parsed store) with **different ENG-0055 consequences**.
  - **(iii) IF the dominant cost had been (c)+(d), the cached-parsed-store remedy would NOT be an ENG-0055 census line-item — it would be the whole secret corpus resident in the clear, making zeroization scope a REDESIGN QUESTION rather than an audit item.** **The measurement says the dominant cost is (b), so the cheapest effective remedy does NOT require a resident plaintext corpus at all** — but this consequence is recorded because any remedy that caches the PARSED STORE, rather than the derived key, still triggers it. **Scope the fix lane honestly from the start.**
- **REMEDY OPTIONS — RECORDED AS OPTIONS. NOTHING IS DECIDED HERE, AND NO REMEDY WORK HAPPENED IN NA-0664.** The choice belongs to the successor lane and its directive.
  1. **ENVELOPE ENCRYPTION / KEY WRAPPING — LEADS ON THE MERITS.** At ~95-97% of a `secret_get` and ~82% of an append, Argon2id is the dominant term, and this option targets it directly: derive once at unlock to unwrap a data key, and ~17.7 of the ~18.1 ms disappears. It also makes passphrase changes cheap. **Its real cost is a vault FORMAT change with migration — that, not the performance case, is the tradeoff to argue.** **The prior objection that it "targets the wrong term" is REFUTED by this lane's data and must not be carried forward.**
     - **⚠ ITS DEADLINE IS SET BY THE FORMAT CHANGE, NOT BY THE MILLISECONDS — AND THAT MAKES IT A PRE-RELEASE ITEM, NOT AN INDEFINITE ONE.** **Migration is FREE TODAY: there are no users and no vaults in the field**, so the change is a format decision made once, against nothing. **After public release the identical change requires version detection, a migration path, testing against old vaults, and a failure mode in which a user's vault will not open — on a product whose premise is that there is no recovery.** The cost of this remedy therefore does not stay flat; **it steps up sharply and permanently at first release**, and it does so for reasons that have nothing to do with the performance case.
     - **STATED EXPLICITLY BECAUSE THE PERFORMANCE FRAMING IS ACTIVELY MISLEADING HERE: "~18 ms, low priority" WILL READ AS "NEVER" to a future reader.** On the milliseconds alone this item is easy to defer forever, and each deferral looks individually reasonable. **The format argument is what makes it urgent: the window in which this is cheap CLOSES AT PUBLIC RELEASE.** A successor scoping this item must weigh the migration deadline, not the latency saving, or it will correctly conclude the latency does not justify the work — and reach the wrong answer.
  2. **CLIENT REUSE — cheap, no ENG-0055 entanglement, payoff exactly 1:1.** Tier 1 measured a multiplicity of **exactly 1** `secret_get` per `relay_http_client()` construction, perfectly linear at N=10 — so one avoided construction = one avoided cold open. **But that is now ~18 ms saved per avoided construction, not ~350 ms.** Independently justified: `reqwest::Client` is designed to be cloned and holds a connection pool, so per-operation construction is wasteful regardless of the vault. **Sizing this option fully requires the owed per-operation measurement below.**
  3. **HISTORY OUT OF THE VAULT — DOWNGRADED; no longer performance-urgent.** Still right for design reasons and still banked in DOC-PROG-004 for lane 3. **Recorded explicitly so that lane is not scoped against a phantom emergency.**
  4. **PAYLOAD CACHING — NOT RECOMMENDED.** At ~6% of the call it buys little for a real security cost (a plaintext corpus resident in memory).
  5. **A TEST-ONLY REDUCED-COST KDF PROFILE — FLAGGED, NOT RECOMMENDED, NOT ACTED ON.** If CI cost is Argon2id x call-count in **debug**, a reduced-cost KDF under `cfg(test)` would collapse CI time directly. **TRADEOFF STATED: CI would no longer exercise the production KDF path. That is a real loss of coverage, not a free win.**
- **⚠ OWED MEASUREMENT — NOT TAKEN IN NA-0664. IT SIZES THE REMEDY, AND IT MAY BE THE LARGER NUMBER.** The **MEASURED `secret_get` count per FULL relay operation** (send / pull / ack).
  - **TWO DIFFERENT QUESTIONS, AND ONLY ONE IS ANSWERED.** Tier 1 measured multiplicity per **`relay_http_client()` CONSTRUCTION** (= 1), and the debug-profile verdict explains the **MARGINAL** cost of the ONE call NA-0663 added — **that pair settles the REGRESSION question, which is why CI got slower.** Neither answers the **TOTAL** vault cost of a relay operation, which is what **sizes the remedy**. A full operation may perform several `secret_get` calls; the **16 call sites are STATIC INSPECTION, NOT a measured per-operation count.**
  - **⚠ INFERENCE — ARITHMETIC SHOWN, EXPLICITLY NOT MEASURED, AND NOT TO BE CITED AS A RESULT.** *If* a relay operation touches even **5–10** of the 16 sites, then at the measured release cost of **~18.1 ms** per `secret_get` (95–97% Argon2id) the total is **5 x 18.1 ≈ 90 ms to 10 x 18.1 ≈ 181 ms of vault cost per relay operation**, of which **~86–172 ms is Argon2id**. **The 5–10 figure is an assumption about call-graph reachability, NOT an observation** — no measurement in this lane bounds how many of the 16 sites a single operation actually reaches, and the true number may be lower (short-circuits, cached higher-level state) or higher (loops, retries, per-contact iteration).
  - **WHY IT MATTERS ENOUGH TO BUDGET FOR: if that range holds, it MATERIALLY CHANGES REMEDY SIZING, and it strengthens BOTH leading options at once.** **Client reuse (option 2)** stops being an ~18 ms saving and becomes a saving proportional to the constructions avoided per operation. **Envelope encryption (option 1)** stops removing ~17.7 ms from one call and removes the Argon2id term from **every** call in the operation — the ~86–172 ms above. **A remedy chosen against the per-call figure alone would be scoped against the smaller number.**
  - **WHY IT WAS NOT TAKEN — THREE INDEPENDENT BLOCKERS, recorded so a successor budgets for it rather than rediscovering the cost:** **(1)** `perf_snapshot()` has **zero consumers** anywhere in `src/` or `tests/`, so reading the counters at all requires a **temporary source bridge**; **(2)** the counters are **process-global**, while the only working full-operation harness (`tests/same_host_client_to_client_e2e.rs`) drives the `qsc` CLI as a **SUBPROCESS** — the test process cannot see them, so a temporary `src/` edit dumping counters at CLI exit would be needed; **(3)** `relay_send` requires an **established two-party protocol session** (it enforces contact trust, peer-not-blocked, and `protocol_active` before any relay work), so an in-process measurement means reproducing multi-step two-party setup that currently exists only as CLI choreography. **Judged NOT CHEAP and deliberately carried as owed rather than guessed.** **It does not fall out of a counter read.**
- Sequencing: **(added 2026-07-21 by NA-0664/D-1290; migrated to the canonical field shape 2026-07-22 by NA-0666/D-1292, content unchanged) the envelope-encryption option should land BEFORE public release.** Its deadline is set by the vault **FORMAT change**, not by the latency: **migration is free while no vaults exist in the field, and becomes permanently harder at first release** — version detection, a migration path, testing against old vaults, and a failure mode where a user's vault will not open, on a product whose premise is that there is no recovery. **Latency alone does not justify the work; the deadline does.** **A successor weighing latency instead of the deadline will correctly conclude the latency does not justify the work — and reach the wrong answer.** This is a SEQUENCING constraint, not a severity claim: **P2 remains correct on an impact scale**, and the deadline is recorded here rather than by inflating severity.
- Status: **open — LOCATED AND APPORTIONED, NOT REMEDIED.** Filed 2026-07-21 by NA-0663 (D-1287; provenance: that lane's five measurement responses, 2026-07-20/21); **measured and re-framed 2026-07-21 by NA-0664 (D-1290), which shipped NO fix by design.** The remedy is a SUCCESSOR lane, deliberately sequenced against **ENG-0055** so that item's zeroization census sees the final shape of any cached secret handle rather than auditing a structure about to change. **What is now settled: the location, the four-bucket split, the floor/growth separation, the `key_source=1` regime, and the debug-vs-release correction. What is NOT settled: the per-operation call count, the `key_source=2` profile, and the choice of remedy.**

### ENG-0054 — the refimpl crate is labeled "non-production" in its own metadata but is the shipping crypto core of qsc
- Severity: P2 (external-review credibility + provenance clarity; NO cryptographic defect — the review found the construction SOUND. The defect is that the crate's own metadata contradicts its role)
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/Cargo.toml:6` — `description = "QuantumShield (QSP/QSE) reference implementation skeleton (audit-friendly, non-production)."`; `:12` — `pqcrypto = ["pqkem", "dep:ml-dsa"] # historical optional integration feature name; verify mappings before production`; the dependency edge from `qsl/qsl-client/qsc` onto that crate
- Claim potentially at stake: that the shipping client's cryptographic core is production-qualified — precisely the release-gate claim the commissioned INDEPENDENT EXTERNAL REVIEW exists to test.
- Description: qsc — the shipping client — takes its ML-KEM-768, ML-DSA-65, AEAD, KMAC and hash primitives from `quantumshield_refimpl`, whose own package description calls it a "reference implementation skeleton (audit-friendly, non-production)" and whose `pqcrypto` feature carries the comment "historical optional integration feature name; verify mappings before production". The review found NO defect in the construction itself. The problem is that the crate's self-description contradicts its actual role, and it is the FIRST artifact an external reviewer encounters — `Cargo.toml` is read before any source file. Exactly one of two things is true: either the verification implied by "verify mappings before production" HAS happened and both strings are stale, or it has NOT happened and the crate is accurately labeled — in which case the label is itself a release-gate finding. The repository does not currently record which.
- Recommended change / scope for the future lane: establish which of the two is true. If the mappings WERE verified, record where and when (lane, decision number, evidence path) and update the description and the feature comment so the metadata matches the crate's role. If they were NOT, that verification IS the work of the lane and the labels stay until it is done. NO cryptographic change is anticipated on either branch. A metadata edit made WITHOUT first establishing the underlying fact would be the wrong outcome and is explicitly NOT what this item asks for — the strings are a symptom, and editing a symptom to look production-ready is the failure mode to avoid.
- Proof gap: no test, evidence document, or decision record asserts that the pqcrypto algorithm mappings were verified against their specifications.
- Recommended directive shape: docs-evidence-only IF the verification is found on record; audit follow-on IF it is not.
- Sequencing: resolve BEFORE reviewer outreach (Track C). If a reviewer finds this first it undercuts the credibility of everything else in the tree, independent of the tree's actual quality.
- Status: open — filed 2026-07-21 from the crypto and protocol review of 2026-07-20, conducted against the qsl-protocol tree at the NA-0663 merge commit `b2dc23bf`. NOT a lane finding — no NA item produced it; filed directly against the ledger by operator instruction. FILING ONLY; not executed.

### ENG-0055 — zeroization coverage is thin on session key material: one derive site tree-wide, and the Suite-2 session state wipes nothing
- Severity: P2 (key-material lifetime / memory-disclosure exposure; NO break in the protocol and NO weakening of at-rest protection — the vault's Argon2id defense is unaffected. This is about what survives in freed heap AFTER the keys are derived from it)
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs:39` — the ONLY `ZeroizeOnDrop` derive in either crate, and it is `cfg_attr`-gated on the `stdcrypto` feature (covers `X25519Priv`); `tools/refimpl/quantumshield_refimpl/src/suite2/state.rs:26-27` — `#[derive(Clone)] pub struct Suite2SessionState`, with NO `Drop` impl and NO `ZeroizeOnDrop`, holding the session root `rk`, the chain keys `ck_ec`/`ck_pq`, the header keys `hk_s`/`hk_r`, the DH ratchet private key `dhs_priv`, and up to `MAX_MKSKIPPED_RESTORE` = 1000 skipped message keys (`state.rs:9`); `snapshot_bytes()` at `suite2/state.rs:48` **and** at `qsp/state.rs:171` — BOTH construct a `Vec<u8>` containing root and chain key material, and NEITHER is wiped after use
- Claim potentially at stake: that key material is protected across its LIFETIME, not only at rest. The vault is defended with Argon2id; the keys derived from it are currently left in freed heap.
- Why it matters: against memory forensics, a core dump, or swap, this is the softest surface in the system, and it is INCONSISTENT with the protection the project already applies elsewhere — `vault/protection.rs`'s `lock()` (ENG-0044, DONE at NA-0658) deliberately zeroizes the live `VaultSession` as one of its stated R3 postconditions. The session ratchet state receives no equivalent treatment. The 1000 retained skipped message keys are the largest single concentration of unwiped key material in the tree.
- Recommended change / scope for the future lane: audit key-bearing structs across `qsc` and the refimpl; apply `ZeroizeOnDrop` where appropriate; wipe BOTH `snapshot_bytes` buffers after use.
- **BINDING NOTE FOR THE DRAFTER — THIS IS NOT A MECHANICAL EDIT.** `ZeroizeOnDrop` interacts with `Clone`: `Suite2SessionState` derives `Clone` today, and every clone site is a potential surviving copy that dropping the original will never reach. Separately, `Vec` REALLOCATION can leave the old allocation's contents in freed heap where no `Drop` impl can reach them at all — so `Vec`-held key material is not fully covered by `ZeroizeOnDrop` alone. The directive MUST require (a) a CENSUS of every clone site with a per-site ruling on whether that clone should exist at all, and (b) an EXPLICIT written statement of what IS and IS NOT covered by the resulting change. A directive that claims blanket wiping would be asserting something the mechanism cannot deliver, and should be rejected at draft.
- Proof gap: no test asserts that any session key material reads as zero after drop, and nothing bounds how many copies of the root/chain keys exist at any moment.
- Recommended directive shape: implementation-only, with a mandatory census artifact and an explicit coverage statement as named deliverables.
- Cross-reference: **ENG-0044** (vault protections restore, DONE at NA-0658) — establishes that the house DOES zeroize at the vault seam, which is what makes the session-state gap an INCONSISTENCY rather than a uniform accepted decision. **ENG-0053** (the per-construction `vault::secret_get` cost) — if that item's remedy introduces a cached secret handle, that cache becomes NEW long-lived key-bearing state and falls inside this item's census; the two should be sequenced so the census sees the final shape rather than auditing a structure that is about to change.
- **⚠ CENSUS SCOPE WIDENS — A TRANSIENT VAULT-SIDE GAP ON THE `secret_get` HOT PATH (added 2026-07-21 by NA-0664/D-1290; RECORDING ONLY, nothing about this item was executed).** Verified by inspection at `ca6897fc`:
  - **(i) THE GAP, WITH ITS EVIDENCE.** `vault/mod.rs` contains **exactly ONE `impl Drop`** — on **`VaultSession` (`:725`, impl at `:733`)**, which zeroizes the key and wipes every secret value. **`secret_get` DOES NOT USE IT.** It uses **`VaultRuntime` (`:717`)**, which holds the 32-byte derived key with **NO `Drop`**, and **`VaultPayload` (`:74`)**, which holds the full `BTreeMap<String,String>` secret corpus with **NO `Drop`**. There is **ZERO `ZeroizeOnDrop`** anywhere in the module. **Every `secret_get` therefore materializes the entire plaintext secret corpus plus a derived key and abandons both unwiped — on the hot path.**
  - **(ii) THIS ITEM'S OWN "the house DOES zeroize at the vault seam" CLAIM IS `VaultSession`-ONLY AND DOES NOT HOLD FOR THE `secret_get` PATH.** Stated explicitly because it is the sentence in this entry most likely to mislead whoever scopes the census: it is true of the WRITE path and of `lock()`, and false of the READ path. **The filing's named surfaces (`traits.rs`, `suite2/state.rs`, both `snapshot_bytes` sites) do not include this vault-side gap.**
  - **(iii) THE SCOPE WIDENS INDEPENDENTLY OF WHETHER ANY CACHE EVER LANDS.** This **SUPERSEDES the conditional, future-tense framing in the ENG-0053 cross-reference immediately above** ("*if* that item's remedy *introduces* a cached secret handle…"), which would otherwise tell a drafter there is nothing here to audit until a cache exists. **There is something to audit today, at every `secret_get`.**
  - **(iv) THE TRADEOFF IS THE REVERSE OF WHAT WAS ASSUMED.** Today: **N plaintext corpus copies created and abandoned unzeroized**, where N = the `secret_get` call count. With a cache: **ONE longer-lived copy with a single known owner that CAN be wiped.** **Caching may therefore IMPROVE memory hygiene rather than degrade it**, and the earlier **"cache = corpus in the clear" framing is WITHDRAWN.**
  - **THE FRAMING THAT MATTERS: THE HARDENED PATH AND THE HOT PATH HAVE DIVERGED.** `lock()` wipes deliberately and was hardened on purpose at ENG-0044/NA-0658; `secret_get` wipes nothing — **in the same module.** The redesign this implies is **owed whether or not any cache ever lands.**
  - **ENG-0055 remains a SEPARATE FUTURE LANE. Nothing about it executed in NA-0664** — this is a recording touch only.
- Status: open — filed 2026-07-21 from the crypto and protocol review of 2026-07-20, conducted against the qsl-protocol tree at the NA-0663 merge commit `b2dc23bf`. NOT a lane finding — no NA item produced it; filed directly against the ledger by operator instruction. FILING ONLY; not executed.

### ENG-0056 — the identity fingerprint truncates to 128 bits and the reasoning is nowhere recorded
- Severity: P2 (documentation of a load-bearing security parameter; the width is assessed DEFENSIBLE on the merits — see the assessment below — but it is UNDOCUMENTED, and that is the finding)
- Exact surfaces: `qsl/qsl-client/qsc/src/identity/mod.rs:138` — `identity_fingerprint_from_identity(kem_pk, sig_pk)` hashes `kem_pk || sig_pk` with SHA-512 and truncates to `hash[..16]`; `:123` — `identity_fingerprint_from_pk(pk)` performs the same truncation for the single-key form. **BOTH** fingerprint constructions are 128-bit
- Claim potentially at stake: the authenticity of an out-of-band verified identity — the DOC-PROG-002 trust model in full.
- Why it matters: the identity fingerprint is the SOLE authenticity mechanism in the QSL trust model. There is no directory, no server vouching, and no phone-number anchor, so a second-preimage on the fingerprint is a FULL impersonation, not a degraded-confidence state. 128 bits gives roughly 2^64 generic collision work. Signal's comparable safety number is ~200 bits, and a reviewer who knows that figure will reach for it immediately.
- Assessment (recorded here so it is not re-derived by the next reader): **DEFENSIBLE but UNDOCUMENTED.** The practical attack cost is well ABOVE the 2^64 generic figure, because a collision must be realized as a VALID ML-KEM/ML-DSA identity pair — not merely as a hash collision on arbitrary bytes. The grinding cost is therefore dominated by candidate identity GENERATION, not by hashing. That argument is sound, and it is the reason the width is acceptable. It is written down here for the first time; it belongs in the claim-boundary documentation where a reviewer will actually look for it.
- Recommended change / scope for the future lane: **DOCUMENTATION FIRST.** State the fingerprint's collision resistance and the realized-identity argument explicitly in the claim-boundary docs, covering BOTH constructions named above. ONLY if that analysis fails to hold under scrutiny should a width change be considered — and a width change is a BREAKING change to every verification code any user has already compared out of band, so it must be treated as one (migration story, dual-display period, or an explicitly accepted break), never as a quiet constant edit.
- Proof gap: no document states the intended collision-resistance TARGET for the verification code, so there is nothing against which the 128-bit choice can be judged correct or incorrect.
- Recommended directive shape: docs-evidence-only.
- Cross-reference: **ENG-0001** (qsc identity/handshake verification-fingerprint SEMANTICS — which fingerprint an operator compares out of band vs. which the handshake authenticates; RESOLVED at NA-0609B, D-1214). That item settled WHICH fingerprint is authoritative and established the dual-pin model; it does NOT touch the WIDTH of either. The two are adjacent and non-overlapping. Note that ENG-0001's dual-pin outcome is what makes BOTH constructions named above user-facing, so a documentation pass must cover both.
- Status: open — filed 2026-07-21 from the crypto and protocol review of 2026-07-20, conducted against the qsl-protocol tree at the NA-0663 merge commit `b2dc23bf`. NOT a lane finding — no NA item produced it; filed directly against the ledger by operator instruction. FILING ONLY; not executed.

### ENG-0057 — two AEAD primitives in the trust base (AES-256-GCM in refimpl, ChaCha20-Poly1305 in the vault) with no recorded rationale
- Severity: P3 (documentation / trust-base surface area; NO defect — both primitives are sound choices, and the review recommends NO change)
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs:9,50,160` — `Aes256Gcm`; `qsl/qsl-client/qsc/src/vault/mod.rs:27-28,65,231` — `ChaCha20Poly1305`
- Description: the protocol side and the vault side use DIFFERENT AEADs. The split is most likely DELIBERATE and well-motivated — GCM for the protocol per spec, ChaCha20-Poly1305 for the vault because it is constant-time in software WITHOUT AES-NI hardware support, which matters for a vault unlocked on arbitrary user hardware. But that rationale is not recorded anywhere in the repository, and the observable consequence is that the trust base contains TWO constant-time AEAD implementations to depend on and to audit rather than one.
- Why it matters: this is not a weakness — it is a surface-area and reviewability question. An external reviewer must audit both implementations, will ask why there are two, and the repository currently has no answer to hand them.
- Recommended change / scope for the future lane: **DOCUMENTATION ONLY** — record the rationale for the split (including the AES-NI / software-constant-time argument, if that is in fact the reason) in the crypto claim-boundary documentation. Consolidate to a single AEAD ONLY IF that review finds the split was UNINTENTIONAL rather than reasoned. **The review recommends NO code change**, and this item must not be read as licence for one.
- Proof gap: no document states which AEAD is used where, or why.
- Recommended directive shape: docs-evidence-only.
- Cross-reference: **ENG-0015** (Suite-2 header trial-decryption is not constant-time — an AEAD *timing* finding in `suite2/ratchet.rs` `recv_nonboundary_ooo`; P3, open, filed NA-0617/D-1230). Same primitive family, DIFFERENT concern: that item is about how many `open()` attempts run and what the count leaks; this one is about which primitives exist in the trust base at all. They should not be merged into one lane.
- Status: open — filed 2026-07-21 from the crypto and protocol review of 2026-07-20, conducted against the qsl-protocol tree at the NA-0663 merge commit `b2dc23bf`. NOT a lane finding — no NA item produced it; filed directly against the ledger by operator instruction. FILING ONLY; not executed.

### ENG-0058 — the dedup store silently resets to empty on parse failure, and the replay backstop behind it is a process-exit
- Severity: P2 (replay-window visibility; the DEFAULT behavior is assessed CORRECT — the finding is that the operator-visible signal it depends on is unproven)
- Exact surfaces: `qsl/qsl-client/qsc/src/dedup/mod.rs:67-68` (the in-code availability-over-strictness rationale), `:74`/`:81`/`:84` (`reset` set on a wrong-version, unparseable, or unreadable file), `:48`/`:105` (the `reset` field as returned to the caller); the `qsp_replay_reject` process-exit path that serves as the backstop in lease mode
- Description: `RelaySeenIds` treats an unreadable, unparseable, or wrong-version seen-ids file as an EMPTY store, setting `reset = true` so the caller can warn. The in-code justification is availability over strictness, with the lease-mode replay-reject as the backstop. **The review AGREES this is the right default** — failing closed on a corrupt dedup file would brick message receipt over a recoverable condition. The finding is what sits BEHIND it: the backstop is a PROCESS-EXIT, and a silently reset dedup store reopens a replay window that only that hard-stop path closes. The whole arrangement therefore rests on the `reset` flag actually reaching a human.
- Why it matters: if `reset = true` is emitted only into a marker stream nobody reads, the warning is not doing its job, and the first observable symptom of a reset store is a process exit on a redelivered message — a hard failure presented with no trace of the corrupt-file event that caused it.
- Recommended change / scope for the future lane: CONFIRM that the reset marker is genuinely surfaced somewhere an operator can see it. This ties directly to the planned **Settings > Logs pane (GUI slice B)**, which is both the natural home for the signal and the reason to settle this before that pane is designed rather than after. **NO change to the reset behavior itself is recommended**; the availability-over-strictness default stands.
- Proof gap: no test asserts that a `reset = true` load produces an operator-visible signal, and nothing pins the relationship between a reset store and the `qsp_replay_reject` exit that backstops it.
- Recommended directive shape: audit follow-on to establish where the marker surfaces, then docs-evidence-only or a small GUI-side lane depending on what is found.
- Sequencing: settle BEFORE GUI slice B's Logs pane is drafted, so the pane is designed against a known signal rather than retrofitted to one.
- Cross-reference: **ENG-0042** (the commit-before-write seam) — that item covers the crash window between `commit_unpack_state` and `write_atomic`, and records that in LEASE mode the redelivered envelope hits `qsp_replay_reject` with the loud `ack_replay_unrecoverable` marker. That is the SAME backstop path this item depends on, approached from the other side: ENG-0042 asks what happens when a MESSAGE is lost, this asks what happens when the dedup RECORD of it is. Directly related; neither should be scoped without the other. Also **ENG-0040** / NA-0644 (D-1267), where the durable dedup store and lease mode landed and where the 31-day prune and 65,536 cap are defined.
- Status: open — filed 2026-07-21 from the crypto and protocol review of 2026-07-20, conducted against the qsl-protocol tree at the NA-0663 merge commit `b2dc23bf`. NOT a lane finding — no NA item produced it; filed directly against the ledger by operator instruction. FILING ONLY; not executed.

### ENG-0059 — BOTH sanctioned routes from a red main back to green are unusable, for two unrelated reasons: the repair path is non-functional and the bootstrap is scoped to a single incident
- Severity: P2 (release-process availability + governance-process integrity; NO runtime, protocol, or security impact — but when main goes red, the repository has NO working sanctioned automated recovery, and every remaining route requires an ad-hoc operator sanction recorded by hand)
- Exact surfaces: `scripts/ci/public_safety_gate.py` — `branch_required_checks()` at **`:599`** (the unhandled 403 on `branches/main/protection/required_status_checks`); the repair-profile table whose only encoded entry is **`send_commit_vault_mock_provider_retired`**; `validate_self_repair_bootstrap_pr()` at **`:1034-1160`** (the bootstrap eligibility conditions), with `--main-advisories-check` defaulting to `advisories` at **`:2553`**; `.github/workflows/public-ci.yml` (the `public-safety` job, which grants only `contents: read`)
- Description — **TWO INDEPENDENT BLOCKERS, DISCOVERED TOGETHER BECAUSE NA-0664 PR-1b HIT BOTH:**
  - **BLOCKER 1 — the red-main repair path is non-functional, and latent.** `branch_required_checks()` (`:599`) queries `branches/main/protection/required_status_checks`; **an unhandled 403 there is fatal**, and the `public-safety` workflow grants only `contents: read` — which cannot read branch protection. Separately, the **only encoded repair profile is `send_commit_vault_mock_provider_retired`**, an **incident-specific hardcoded value that does not generalize**, so a main that is red **for any other cause has no matching profile**. This path is reached only when main is red AND the PR is `workflow_security` or `runtime_critical`, so it is **LATENT — it may never have executed successfully at any point in this repository's history.**
  - **BLOCKER 2 — the sanctioned self-repair bootstrap is scoped exclusively to advisories-driven reds.** `validate_self_repair_bootstrap_pr` imposes **THREE conditions beyond path shape**, all verified unmet for PR-1b by reading the function end to end: **(1)** main must be red **BECAUSE `advisories` is failing** — on `ca6897fc`, `advisories` was **SUCCESS** and main was red because of the *watchdog*, so this condition **could only have been satisfied by making `advisories` fail**; **(2)** the PR must modify **`scripts/ci/public_safety_gate.py`**, which PR-1b did not and should not have; **(3)** exactly **ONE** `tests/NA-*public_safety*.md` testplan stub must be present, and PR-1b had **zero**. Satisfying (2) or (3) would have meant widening scope or improvising to fit the gate — both were refused under the operator's standing hard condition, and **nothing was re-scoped, re-pushed, or altered.**
- **⚠ CORRECTION TO THIS ITEM'S ORIGINATING TEXT — RECORDED BECAUSE IT WOULD MISLEAD.** The dictated filing text stated that NA-0664 PR-1b **"was routed via the sanctioned self-repair bootstrap instead."** **That is FALSE, and it was not merely not done — it was NOT POSSIBLE**, for the three reasons above. **What actually unblocked PR-1b was an explicit, ad-hoc OPERATOR SANCTION** to re-run a `public-safety` check that had never reached a verdict (see `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, 2026-07-21) — a human ruling recorded by hand, **not** an automated sanctioned route. **The distinction is the whole point of this filing: the automated routes did not work, and a person had to decide.**
- Consequence: **the repository currently has NO working sanctioned automated route from a non-advisories red main back to green.** Both designed escape hatches are unavailable, for unrelated reasons, and **neither had ever been exercised** — which is why the gap surfaced only when a lane happened to need it. The remaining routes are all worse: an admin merge or a branch-protection adjustment (a strictly LARGER bypass — merging past a red required check by removing the requirement), or a hand-sanctioned re-run.
- Recommended change / scope for the future lane: **decide whether the repair path should be FIXED or REMOVED as dead machinery.** If fixed, it needs **all** of: the permission to read branch protection, **403 tolerance** so a permissions gap degrades instead of aborting, and a **profile mechanism that generalizes** beyond one hardcoded incident. If removed, say so explicitly and document what replaces it. **DO NOT FIX ONE BLOCKER WITHOUT THE OTHER — a working 403 path with a non-generalizing profile is still unusable**, and a generalizing profile behind a fatal 403 never runs. **The bootstrap's advisories-only trigger must be settled in the same lane:** a repository whose only self-repair route assumes one specific cause of redness has not solved self-repair, it has solved one incident.
- **CROSS-REFERENCE — THIS IS THE SAME DEFECT CLASS AS ENG-0052, INSTANCES 4 AND 5.** The repair profile (a hardcoded incident-specific value) and the bootstrap's advisories-only trigger (a hardcoded assumption about *why* main would be red) are both **FIXED VALUES ENCODING A MOMENT IN TIME THAT NOTHING RE-DERIVES** — the same shape as the macOS 120 ceiling, Linux's inherited 360 default, and the watchdog's 390 budget. **These two show the class is not confined to numbers or to CI timing.** See ENG-0052's five-instance enumeration.
- Proof gap: no test exercises either recovery route. **Both are reached only when main is already red**, which is precisely the state in which nobody wants to be discovering that the recovery machinery does not work. A successor should consider whether either route can be exercised deliberately — on a scratch ref or in a rehearsal — rather than only in a real incident.
- Recommended directive shape: CI-tooling lane against `scripts/ci/public_safety_gate.py` + `.github/workflows/public-ci.yml`. **Not docs-only; touches workflow-security surfaces, so FULL ritual.**
- Status: open — filed 2026-07-21 by NA-0664 (D-1290; operator-dictated during PR-1b, with the bootstrap-ineligibility correction applied at filing). FILING ONLY; not executed in-lane — NA-0664's scope admitted only the `public-safety` polling budget, and any remediation here is a separate, deliberate lane.

### ENG-0060 — the native GTK menubar cannot be themed or hidden from the frontend, and the fix is platform-specific work that no frontend lane can do
- Severity: P3 (cosmetic; NO runtime, protocol, or security impact — a light-themed menu strip renders against the dark app on the surfaces where the menu is deliberately shown)
- Exact surfaces: `qsl-desktop src-tauri/src/lib.rs` — the menu construction at **`:173-217`** (`MenuBuilder` / `SubmenuBuilder` / `PredefinedMenuItem`, the pinned tauri 2 core menu API) and the attachment at **`:218`** (`app.set_menu(menu)`); the per-mode attach/remove at **`:91-100`**
- Description: the File/Edit/View/Help bar is a **Tao/GTK NATIVE widget**, not DOM. On Linux `app.set_menu` produces a GTK menubar packed inside the tao window, **outside the webview**. **No rule in `ui/style.css` can reach it and no frontend change of any kind can recolor it.** The measured ~RGB(202,222,233) is consistent with the **ambient GTK light theme's** menubar, which is precisely why it reads as a bright strip against the dark app: **the app is not choosing that colour, the desktop theme is.**
- **⚠ SCOPE NOTE THAT MUST NOT BE LOST — this is why the item is filed rather than fixed.** Work item C of D601 was **DROPPED by the operator's own scope gate** and the drop **CONFIRMED at approval**. Theming or hiding a native menubar requires **platform-specific** work: a `GtkCssProvider` against the menubar widget on Linux, **plus** the corresponding path on macOS and Windows, where the same code produces a system menu that is themed differently again or is not a strip at all. **The eventual Appearance-pane / dark-frame story OWNS THIS. A FRONTEND LANE CANNOT PICK IT UP, AND NO FILING OR ROADMAP MAY IMPLY THAT IT CAN.**
- **RELATED FINDING, SETTLED EMPIRICALLY AND RECORDED SO THE SCOPE IS NOT RE-LITIGATED (FINDING C2):** NA-0662 already removes the menu on **every compact mode** (`remove_menu()` at `lib.rs:98-100`, asserted by `design_round3.rs`). NA-0665's Phase-1 before-shots, built from **unmodified main at `8db2b2a5`**, confirm it: the top rows of both compact screens measure **exactly RGB(29,29,31) = `--bg` #1D1D1F** — **there is no white strip on the pre-main screens at all.** The screenshots that motivated item C therefore **predate NA-0662 and are SUPERSEDED**. **The remaining exposure is the MAIN WINDOW and Settings only**, where the menu is deliberately attached (`Full` mode, `menu_visible == true`).
- Consequence: on the main window and Settings, a GTK-light-themed strip sits above a dark app on desktops using a light GTK theme. Cosmetic, and confined to two surfaces.
- Recommended change / scope for the future lane: fold into the **Appearance-pane / dark-frame** work. Decide there whether the menubar is **themed** (per-platform) or **removed entirely** in favour of an in-app menu affordance — the latter being the only option that is uniformly solvable across the three platforms. **Do not scope this as a CSS task.**
- Proof gap: no test asserts menubar COLOUR anywhere, and none can from the frontend — the widget is not in the DOM and is not reachable by the existing CSS-text assertions. Menu **visibility** is asserted (`design_round3.rs`), colour is not.
- Status: open — filed 2026-07-22 by NA-0665 (D-1291) as the recorded disposition of D601 work item C. **FILING ONLY; deliberately not executed in-lane — the drop was ruled at approval and this lane was DESIGN/FRONTEND only.**

### ENG-0061 — the wiped / "Vault erased" screen ships with no danger colour, because its heading was never red and the round-4a chrome strip removed its only signal
- Severity: P3 (cosmetic / signalling; NO runtime, protocol, or security impact — and **nothing can be mis-triggered by it**, see the acceptance ruling below)
- Exact surfaces: `qsl-desktop ui/index.html` — the wiped screen at **`:89-95`**, whose heading is a **plain** `<h1>Vault erased</h1>` carrying no danger class; `qsl-desktop ui/style.css` — `.card h1` sets size and weight but **no colour**, so it inherits `--fg` (#E8E8E8); the round-4a chrome strip that removed `.danger-card`'s border on that screen
- Description: **the F2 override's own stated rationale is factually false on this one screen, and that is why the item exists.** The override reasoned that stripping the danger border was safe because *"the 'Erase everything' / 'Vault erased' headings and the warning copy are already red."* **That premise holds for erase and NOT for wiped.** Erase is `<h1 class="ceremony-head">`, coloured `var(--danger-text)` by `.ceremony-card .ceremony-head` — a rule NA-0665 left untouched, and the operator's after-shot confirms it renders red. **Wiped's heading was NEVER red**; its only danger signal was the **`.danger-card` BORDER**, which the override instructed be stripped. The result, visible in the operator's own 11-44-34 after-shot: **a white heading and no border — no red anything.**
- **RULED ACCEPTED AS-IS by the operator (2026-07-22), and the reasoning is recorded because it bounds the severity:** the wiped screen is a **calm post-hoc NOTICE**, not an armed destructive gate. The data is **already gone**, nothing is being confirmed, and the only control is a primary "Start over". **Nothing can be mis-triggered by under-signalling it.** The operator's acceptance rested on **seeing the actual rendered pixels**, so what shipped is what was approved — **the error was in the verbal rationale, not in the approval.**
- Consequence: one pre-main screen conveys a destructive OUTCOME with no colour cue. Arguably correct for a neutral notice; arguably a gap against the override's stated intent. **The point of the filing is that the two readings differ and the choice should be deliberate rather than accidental.**
- Recommended change / scope for the future lane: **round 4c, revisited with the Settings-pane pass.** If red is wanted it is **one attribute plus one selector** — give the wiped `<h1>` the danger class and add the matching rule; if the calm-notice reading is preferred, **say so explicitly in Appendix E** so a later reader does not "fix" it back. **Do not fix it in isolation** — decide it alongside the rest of the danger-signalling vocabulary, so the app has one answer rather than per-screen accidents.
- Proof gap: no test asserts danger COLOUR on the wiped screen, in either direction. Whichever way round 4c rules, **the ruling should land as an assertion**, because this defect arrived precisely by a rule being changed with no test to notice the consequence on one screen.
- Status: open — filed 2026-07-22 by NA-0665 (D-1291), **ACCEPTED AS-IS by operator ruling and DELIBERATELY NOT FIXED FORWARD in-lane** (fixing it would have been scope the lane was not given). Deferred to round 4c.

### WF-0025 — the queue helper cannot distinguish "READY_COUNT 0, correct at closeout" from "the queue is malformed" — both are exit 2, so closeout verification cannot be a hard gate
- Severity: P3 (governance-process/tooling; NO runtime, protocol, or security impact — but it makes a class of closeout instruction unsatisfiable as written, and invites the one workaround that reports a flag's behavior instead of the queue's)
- Exact surface: `scripts/ci/qsl_evidence_helper.py` — `queue_command()` at **`:249-251`**:
  ```python
  if len(ready) != 1 and not args.allow_nonready_count:
      return 2
  return 0
  ```
- Description: the subcommand's contract is the **`qwork` PRECONDITION** — *exactly one lane is ready to start* — so it returns **0 only at `READY_COUNT == 1`**. Every other count returns **2**. That conflates three genuinely different states behind one exit code: **(a)** `READY_COUNT 0` at a **closeout**, which is the CORRECT terminal state; **(b)** `READY_COUNT 0` because the queue is **malformed** (a STATE-vs-section mismatch, a dropped `Status:` line, a section-heading typo that makes the regex miss an item); and **(c)** `READY_COUNT >= 2`, an over-promoted queue. **(a) is success and (b) is the exact failure that broke `qwork`**, and the helper reports them identically.
- **HOW IT SURFACED, recorded because the near-miss is the point.** NA-0665's closeout instruction required verification "the way `qwork` reads it — **`READY_COUNT 0, exit 0`**, not a STATE grep." **That is unsatisfiable:** `READY_COUNT 0` and `exit 0` cannot co-occur without `--allow-nonready-count`. The executor verified the behavior in both directions (**exit 2 without the flag, exit 0 with it**), **declined to pass the flag** — since doing so would have reported the flag's behavior rather than the queue's — and reported the contradiction instead of manufacturing a green exit code. **The operator confirmed the instruction was in error and that refusing the flag was correct.** The substance the instruction was protecting (both layers agreeing at `READY=NONE` / `Status: DONE`) was proven separately and held.
- Consequence: **any lane closing to `READY=NONE` hits this**, so it is not specific to NA-0665 and will recur at every closeout. The failure mode is social rather than technical: a closeout instruction phrased as "verify exit 0" can only be met by passing `--allow-nonready-count` (**which suppresses the very check being invoked**, turning a gate into a formality) or by quietly not checking. **A gate that cannot pass teaches people to bypass it.**
- Recommended change / scope for the future lane: give the subcommand a mode that **separates the three states** — e.g. an `--expect closeout|ready|any` selector, or distinct exit codes for "no lane ready" versus "queue malformed" — so a closeout can assert **success** rather than assert a documented failure. **Whatever shape is chosen, `--allow-nonready-count` must stop being the only route to a green closeout**, because it green-lights (b) and (c) exactly as readily as (a). Pair it with the both-directions queue lint below, which is the check that would actually catch (b).
- **⚠ CLUSTER NOTE — the qwork / workspace-hardening cluster this belongs to IS NOT YET IN THIS LEDGER, and that is itself the finding's companion.** The operator tracks this item alongside three others: **(1)** a **both-directions queue lint** (STATE and the `### NA-xxxx` section must be asserted to agree, in both directions — the mismatch that broke `qwork` on 2026-07-21 and required the #1617 correction merge); **(2)** the **FAIL-artifact** problem (cancelled/superseded workflow runs litter a PR's checks as `fail` and cannot be re-run — see the NA-0662 GitHub-outage experience, where cancel + close/reopen was the only route to fresh runs); **(3)** the **workspace `user.email`** hazard (this workspace's git config reads a non-GH007 address, so every commit must override explicitly or carry the wrong identity — a standing trap paid per-commit, per-lane, forever, and never fixed at source). **A search of `docs/ops/IMPROVEMENT_LEDGER.md` at NA-0665 closeout found NO ledger entry for any of the three** — they exist in operator/relay context only. **This entry therefore OPENS the cluster rather than joining one**, and a successor consolidating it should file the other three rather than assume they are already recorded. — **AMENDED 2026-07-22 by NA-0666 (D-1292): the cluster IS NOW FILED, and the note above is preserved as written because its census was accurate when made.** Member **(1)** the both-directions queue lint = **WF-0026**, which also carries the `parse_queue` two-letter-suffix widening as part of the same item (they are one surface: repairing this entry's exit codes without that widening leaves a live trap behind a newly-trusted gate). Member **(2)** the FAIL-artifact problem = **WF-0028** (GitHub Actions cancelled-run litter). Member **(3)** the workspace `user.email` hazard = **WF-0029**. **⚠ AND ONE MEMBER THE CENSUS DID NOT CAPTURE: `qwork`'s own missing FAIL artifact = WF-0027**, a genuinely different finding that had been tracked together with member (2) under the shared word "FAIL" — one is GitHub Actions run state, the other is local `qwork` shell tooling. Also filed with the cluster: **WF-0030**, **WF-0031**, **WF-0032**, and **ENG-0062**. **This entry's own Status and findings are unchanged.**
- Proof gap: nothing asserts the helper's exit code for any queue shape. **The behavior above was established by reading `:249-251` and running the command both with and without the flag** — the first exercise of that path either way, as far as this ledger records. A successor should pin the intended exit semantics as a test **before** changing them, so the fix cannot silently re-conflate the states.
- Status: open — filed 2026-07-22 by NA-0665 (D-1291) at operator direction. **FILING ONLY, explicitly NOT this lane** — NA-0665 was a qsl-desktop DESIGN/FRONTEND lane and touching `scripts/ci/` would have been far outside its scope.

### WF-0026 — the queue parser cannot see two-letter-suffix `NA-` headings, and repairing WF-0025's exit codes WITHOUT widening it leaves a live trap behind a newly-trusted gate
- Severity: P2 (governance-process/tooling; NO runtime, protocol, or security impact). **Rated above WF-0025's P3 for one reason: the defect is LATENT today and becomes LIVE the moment WF-0025's fix makes the gate worth trusting.**
- Exact surface: `scripts/ci/qsl_evidence_helper.py` — `parse_queue()` at **`:222-234`**, the heading regex:
  ```python
  pattern = re.compile(
      r"^### (NA-\d+[A-Z]?) — ([^\n]+)\n(?P<body>.*?)(?=^### |\Z)",
      re.M | re.S,
  )
  ```
  `[A-Z]?` admits **at most one** suffix letter. The fix is the one-character widening to `[A-Z]{0,2}`.
- Description: measured live on this lane's branch by applying the regex above, read-only and unmodified, to `NEXT_ACTIONS.md`: **718 `^### NA-` headings present, 711 matched by the parser, 7 invisible** — `NA-0197BA`, `NA-0197CA`, `NA-0200AA`, `NA-0215BA`, `NA-0215BB`, `NA-0215BC`, `NA-0216AA`, every one a two-letter suffix. (D602 recorded **717 / 710 / 7** at draft time; the totals moved by one when the NA-0666 promotion added a heading. **The seven are identical and the count of invisible headings did not change.**) **All seven carry `Status: DONE` — verified individually — so the defect is LATENT, not live:** no promotable lane is currently hidden, and no queue reading has yet been wrong because of it. **Say that plainly rather than implying a live outage.**
- Consequence — **the trap, which is the reason this is one item with WF-0025 and not a downstream note.** WF-0025's finding is that `READY_COUNT 0` returns exit 2 for three different states, so a closeout cannot assert exit 0. The obvious repair is to separate those states so a closeout can assert success. **But a two-letter lane promoted to `Status: READY` yields `READY_COUNT 0` — which, after that repair, reports as the CORRECT terminal "no lane ready" state, byte-identical to WF-0025's case (a), while actually being case (b), a queue the parser cannot read.** Today that misreading is caught because everything returns exit 2 and every exit 2 is investigated. **Repairing the exit codes first would convert a loud, universally-distrusted failure into a quiet, newly-trusted success.** The widening is therefore not an optimisation of the same surface; it is the precondition that keeps the repair honest.
- Recommended change: **(1)** widen the suffix class to `[A-Z]{0,2}`; **(2)** add a **both-directions queue lint** asserting that STATE `READY=NA-X` ⟺ exactly one `### NA-X` section with `Status: READY`, **and** that every `^### NA-` heading in the file is parser-visible — the second half is what catches this defect and the first half is what catches the STATE-vs-section mismatch that broke `qwork` on 2026-07-21 and required the #1617 correction merge. A heading the parser cannot see must be a hard error, not a silent omission.
- Proof gap: nothing asserts the parser's heading coverage, in either direction. No test pins the regex against a two-letter-suffix heading, and no check compares the raw heading count to the parsed item count — the exact comparison that makes this defect visible in one line. **Pin the coverage assertion BEFORE widening**, so the widening is demonstrated to change something.
- Sequencing: **pairs with WF-0025; neither should land alone.** Landing WF-0025's exit-code separation without this widening is the trap described above. Landing this widening alone is harmless but leaves the closeout gate unsatisfiable. **They are one surface and one lane.**
- Status: open — filed 2026-07-22 by NA-0666 (D-1292) at operator direction. **FILING ONLY — the fix is lane (b).**

### WF-0027 — `qwork` failure produces no artifact: `qwork_fail()` prints to stdout and exits, so a FAILED startup leaves nothing on disk while a successful one is durably proved
- Severity: P2 (governance-evidence/tooling; NO runtime, protocol, or security impact — but it makes a failure unprovable in exactly the evidence model the house runs on)
- Exact surface: `/srv/qbuild/tools/qwork.sh` — `qwork_fail()` at **`:38-52`**: it `printf`s `startup_result=FAIL`, `reason=`, `lane=`, and optionally `repo=`/`path=` **to stdout**, then `exit 2`. **No file is written on any failure path.** Contrast the success path at **`:214-215, 228`**, which writes `startup.<repo>.kv` and `startup.<repo>.json` into both `/srv/qbuild/logs/<lane>/` and the lane workspace's `.qwork/` directory.
- Description: the asymmetry is total. A startup that SUCCEEDS produces two durable, timestamped, sha-stamped proof files that the executor is required to verify and the Director can re-read at any later date. A startup that FAILS produces terminal text that exists only in the operator's scrollback.
- Consequence: **a successful startup is durably provable and a failed one is not.** The Director can only ever learn about a failure as pasted terminal text — **which is asserted state, the precise thing `DIRECTOR_OPERATIONS.md §5` ("Verified state replaces asserted state") exists to forbid.** The failure case is also the case where evidence matters most: a lane that will not start is a lane about to be diagnosed remotely, from a description, by someone who cannot see the terminal.
- Recommended change: write `startup.<repo>.FAIL.json` on every `qwork_fail()` path, carrying the reason, the lane, the repo, the path, and the helper output that produced the decision — into the same two locations the success path uses, so the proof directory is the single place to look for either outcome. The success/failure asymmetry, not the format, is the defect.
- Proof gap: no test exercises any `qwork_fail()` path, and nothing asserts that a failed startup leaves any artifact. Because `qwork.sh` is unversioned (**WF-0031**), there is also no test harness that could hold such an assertion.
- **⚠ NAME COLLISION, RECORDED DELIBERATELY (OBS-A): this is NOT the item WF-0025's cluster note calls (2).** That member is the GitHub Actions cancelled-run problem, filed here as **WF-0028**. The two were tracked as a single item because both were described with the word "FAIL". **They are unrelated: this one is local `qwork` shell tooling, WF-0028 is GitHub Actions run state.** A future directive drafted from either description alone will silently drop the other — which is why both entries carry this note.
- Sequencing: **blocked by WF-0031.** The file is unversioned, so this fix cannot be diff-reviewed, reverted, or carried by any PR until the custody question is answered.
- Status: open — filed 2026-07-22 by NA-0666 (D-1292) at operator direction. **FILING ONLY — the fix is lane (b).**

### WF-0028 — cancelled and superseded GitHub Actions runs litter a PR's checks as `fail` and cannot be re-run
- Severity: P2 (CI-evidence legibility; NO runtime, protocol, or security impact — but it degrades the check surface that merge decisions are read from)
- Exact surface: GitHub Actions run state as presented on a PR's checks list. Not a repository file — the defect is in how cancelled runs persist and what can be done with them.
- Description: two independent instances are on record. **(1) NA-0662, during a critical GitHub Actions outage:** queued runs could not be re-run at all; the only route to fresh runs was to CANCEL them and then CLOSE and REOPEN the PR. The cancelled runs then remained attached to the PR as `fail`-presenting artifacts alongside the successor runs that actually governed the merge. **(2) NA-0660, `qsc-adversarial`:** a run cancelled by DESIGNED supersession — the workflow sets concurrency `cancel-in-progress` per-ref, so a newer push deliberately cancels the older run — presented identically to a genuine failure. In both cases the successor run was green and governed; in both cases the checks list carried a red artifact that no action could clear.
- Consequence: a PR's check list stops being a reliable summary of its state. A reader must reconstruct, by timestamp and by knowledge of the concurrency configuration, which red entries are real and which are superseded corpses — and the two most consequential moments for that reading (an outage, a rapid re-push) are exactly when the litter appears. **The failure mode is that a genuine red hides among cancelled ones, or that a reviewer learns to discount red entries generally.**
- Recommended change: no repository fix is currently known — this is substantially GitHub-side behaviour. What the house CAN do is record the disambiguation rule it already applies by hand: a cancelled run is superseded (not failing) when a later run of the same context on the same ref concluded, and the merge decision follows the successor. Capture that as a written rule and, if possible, as a check-summary helper, so the reasoning is not re-derived under time pressure during the next outage.
- Proof gap: nothing distinguishes "cancelled by supersession" from "cancelled after failure" mechanically; the distinction currently lives in the operator's and executor's reading of run timestamps and concurrency settings.
- **⚠ NAME COLLISION, RECORDED DELIBERATELY (OBS-A): this IS the item WF-0025's cluster note calls (2), and it is NOT the same finding as WF-0027.** Both were tracked under the shared word "FAIL" — this one is GitHub Actions run state, **WF-0027** is `qwork`'s local failure artifact. **They are unrelated and neither substitutes for the other.** A future directive drafted from either description alone will silently drop the other; both entries carry this note so that cannot happen quietly.
- Status: open — filed 2026-07-22 by NA-0666 (D-1292) at operator direction. **FILING ONLY — the fix is lane (b).**

### WF-0029 — the lane-checkout git identity is the personal address, so the GH007 noreply identity is re-paid by hand on every commit, in every lane, forever
- Severity: P2 (release-hygiene/process; NO runtime, protocol, or security impact — the failure is a rejected push or a leaked personal address in commit metadata, not a break)
- Exact surface: measured live in this lane's workspace. `git config --show-origin user.email` resolves to **`file:/home/victor/.gitconfig`**, carrying **the personal, non-GH007 address** (**the literal value is deliberately NOT recorded here — see the redaction note in this entry's Status**); `git config --local --get user.email` → **no local override**. `/srv/qbuild/tools/new_checkout.sh` is **87 lines** and contains **zero** `git config` calls — a grep for `user.email`, `user.name`, or `git config` across `new_checkout.sh`, `qwork.sh`, and `qwork` returns **zero hits in all three files**. Nothing in the checkout or startup path ever sets the repo-local identity.
- Description: GH007 requires the noreply identity `238594419+Tebbens4832@users.noreply.github.com` on every commit. The workspace supplies the personal address by inheritance. Every commit in every lane must therefore carry an explicit `git -c user.email=…` override, applied by hand, by the executor, every time — and a single forgotten override either rejects the push or writes the personal address into permanent history.
- Consequence: **this is the canonical instance of the meta-finding this cluster exists to record.** The hazard has a recovery that ALWAYS WORKS: the override is one flag, it is well known, it is in every directive's standing rules, and a diligent executor applies it every time. **Because the recovery never fails, the hazard never generates pressure to fix itself**, and the cost is paid forever in small, invisible increments by whoever is most careful. The defect is not that anything breaks; it is that **a permanent per-commit tax is indistinguishable, from the outside, from a solved problem.**
- Recommended change: set the repo-local identity at checkout creation, in `new_checkout.sh` — one `git config user.email` and one `git config user.name` per created checkout. **This is the fix that removes the tax rather than continuing to pay it.**
- Proof gap: nothing asserts the identity of any checkout, and nothing would notice a checkout created without it. A post-creation assertion (checkout has the noreply identity locally set) is the missing check, and it is cheap.
- Sequencing: **blocked by WF-0031.** `new_checkout.sh` is unversioned, so this fix cannot be carried by a PR, reviewed as a diff, or reverted.
- **⚠ REDACTION, at operator ruling 2026-07-22 (D602/OBS-4): the literal personal address is NOT recorded in this repository, and the one pre-existing instance elsewhere in the tree was redacted by this lane in the same pass.** The finding is about the **MECHANISM** — `new_checkout.sh` sets no identity, so the wrong identity is paid by hand on every commit — and **the literal value adds nothing the description does not already carry.** Writing a personal email into a public-bound repository in order to document that it leaks is self-defeating. The address is recoverable from `git config --show-origin user.email` on the build host by anyone who needs it; it does not need to be committed.
- Status: open — filed 2026-07-22 by NA-0666 (D-1292) at operator direction. **FILING ONLY — the fix is lane (b).** **The override remains mandatory on every commit until it lands, including the commits of this very lane** — which is itself the finding, demonstrated.

### WF-0030 — `qsl-desktop` is not a qbuild-known repo, so `qwork` cannot check it out and the MANDATORY qwork proof is structurally unavailable for every desktop lane
- Severity: P2 (governance-coverage; NO runtime, protocol, or security impact — but it means a mandatory gate has been silently inapplicable for nine consecutive lanes)
- Exact surface: `/srv/qbuild/tools/env_qbuild.sh` — `qbuild_all_repos()` returns exactly `qsl-protocol qsl-server qsl-attachments` (verified live), and `qbuild_require_known_repo()` hard-rejects anything else. A grep for `qsl-desktop` across **all** of `/srv/qbuild/tools/` returns **zero hits** (verified live).
- Description: `CLAUDE.md:35` makes the operator-run `qwork` proof mandatory for a lane to begin. `qwork` can only operate on repos `qbuild_all_repos()` knows. `qsl-desktop` is not among them, so no desktop lane can ever produce that proof. Lanes **NA-0657 through NA-0665** — every desktop lane that has ever run — therefore ran from hand-made `/srv/qbuild/tmp/` directories, with **no `startup.<repo>.json` proof, no lock directory, and no shared cargo target.**
- Consequence: **a mandatory gate is not merely being skipped, it is structurally unavailable** — the lanes were not careless, the tooling cannot serve them. That distinction matters for how it gets fixed: no amount of lane discipline closes it. It also means the qwork proof requirement, as written, is unsatisfiable for a whole repo, and a reader comparing desktop lanes against `CLAUDE.md:35` would conclude nine lanes were run improperly. **This is a governance-coverage gap affecting every desktop lane. It is NOT a `measure.py` problem** — ENG-0062 is downstream of it, not the same finding.
- Recommended change: make `qsl-desktop` a qbuild-known repo — add it to `qbuild_all_repos()` with its checkout, lock, and target conventions — so desktop lanes obtain the same startup proof as spine lanes.
- Proof gap: nothing asserts that every repo a lane may target is qwork-reachable, so the gap was invisible until a desktop lane needed the proof. The missing check is a census: the set of repos lanes actually run in versus the set `qbuild_all_repos()` returns.
- Sequencing: **blocked by WF-0031** for the same custody reason as WF-0027 and WF-0029. **⚠ AND IT MAY REORDER THE CLUSTER: fixing this may be what makes ENG-0062's durable home and clean desktop lanes possible at all.** If `qsl-desktop` becomes qbuild-known first, both the ENG-0062 move and every future desktop lane gain a proper workspace — which is an argument for taking this AHEAD of the helper work. **The fork is named here, not resolved; the operator promotes.** **RESOLVED IN THE ORDER FILED: the operator ruled WF-0031 then WF-0030 in one lane, and the fork closed the way this line anticipated.**
- **RESOLVED 2026-07-22 by NA-0667 (D-1293, directive D603).** `qsl-desktop` is now qbuild-known. Six one-line sites: `env_qbuild.sh` (`qbuild_all_repos`, `qbuild_require_known_repo`, `qbuild_repo_remote`), `qwork.sh:68`, `qshell.sh:12`, and the `cache/targets/qsl-desktop` entry in `bootstrap_qbuild.sh` + `preflight_clean.sh`. `/srv/qbuild/mirrors/qsl-desktop.git` (296 KB, bare, `main` = `02cc9b96`) was created **in the same change and is not separable from it** — `preflight_clean.sh:57` reports an issue for any repo in `qbuild_all_repos()` lacking a mirror, and `capture_evidence.sh:96` reads each mirror's remotes.
- **⚠ THE FILING OVERSTATED THE WORK, AND THE REASON IS WORTH KEEPING: `qwork` never assumed spine structure for non-spine repos.** All three spine-specific branches — `qwork.sh:142` (queue verification), `qwork.sh:445` (queue proof fields), `qshell.sh:104` (the fast-forward guard) — already early-return for any repo that is not `qsl-protocol`, and were **deliberately left byte-unchanged**. `qsl-server` and `qsl-attachments` were the live precedent throughout: both qwork-known, neither with a `NEXT_ACTIONS.md`, a queue, or an evidence helper. **The gap was a missing list entry, not missing machinery.** The filing's "no amount of lane discipline closes it" was correct; its implied difficulty was not.
- **Proof gap CLOSED:** `/srv/qbuild/tools/check_repo_registration.sh` asserts — for every repo `qbuild_all_repos()` returns — that the remote resolves, the mirror exists and is bare, **both** known-repo predicates accept it, the target root exists, and `--print`/`source` agree. It lives in the tools repo, not the spine, because GitHub runners have no `/srv/qbuild`: a spine CI test asserting this would fail on every runner or be a silent no-op.
- Status: **RESOLVED** — registration and mirror landed together; before/after `qwork` evidence in `docs/governance/evidence/NA-0667_as_built.md`. **⚠ One residual, which is NOT a defect in this fix and does not reopen this item:** `/srv/qbuild/cache/targets` is root-owned and not group-writable, so `cache/targets/qsl-desktop` could not be created by the lane. Filed as **WF-0033**; one privileged command completes it.

### WF-0031 — `/srv/qbuild/tools/` is not under version control, so the tooling the house treats as operational has no diff, no revert, and no backup
- Severity: **Reporting-only** per operator ruling (recorded, deliberately not severity-ranked — the custody decision is the operator's, and a severity number would imply a queue position this item is not asking for)
- Exact surface: `cd /srv/qbuild/tools && git rev-parse --show-toplevel` → **`fatal: not a git repository`** (verified live). A `find` across the spine for `qwork.sh`, `qwork`, `new_checkout.sh`, or `env_qbuild.sh` returns **nothing** — **no copy of any of these files exists anywhere in any repo.**
- Description: `DIRECTOR_OPERATIONS.md §5a` describes this directory as operational infrastructure. It is the code that starts every lane, creates every checkout, and writes every startup proof. It has no history, no diff, no review path, no revert, and no backup.
- Consequence: **this is the custody problem that makes lane (b) hard, and it is why it is filed alongside the items it blocks rather than beneath them.** **WF-0027**, **WF-0029**, and **WF-0030** all target files that no PR can carry. Their fixes cannot be proposed as a diff, reviewed by anyone, tested by anything, or reverted if wrong — and a mistake in `qwork.sh` breaks the startup path for every lane in every repo simultaneously, with no previous version to return to. **The three blocked entries are not blocked by difficulty; they are blocked by the absence of a place to put the change.**
- Recommended change: bring `/srv/qbuild/tools/` under version control. The shape is genuinely an open question — its own repository, a directory in the spine, or a satellite — and it interacts with what the host-local tooling is allowed to assume about its environment. **Recorded as a question for the operator, not answered here.**
- Proof gap: not applicable in the usual sense — there is no test to add, because there is no repository to add it to. **That is the finding.**
- **⚠⚠ CORRECTION OF RECORD, 2026-07-22 by NA-0667 (D-1293) at operator direction — THIS ENTRY'S TITLE AND FRAMING OVERSTATED ITS OWN SEVERITY, AND THE OVERSTATEMENT WAS LOAD-BEARING.** The title says "no diff, no revert, **and no backup**", and the Consequence rests its blocker status on *"a mistake in `qwork.sh` breaks the startup path for every lane in every repo simultaneously, **with no previous version to return to**."* **The backup clause was FALSE WHEN FILED.** Measured live at NA-0667 draft: `/srv/qbuild/tools` is **source #33 in `/usr/local/sbin/qsl-backup`**, run daily by `qsl-backup-daily.timer` with **`DAILY_KEEP=30`**, and the run immediately preceding the filing succeeded — `Jul 22 02:36:07 ideacentre qsl-backup[410135]: qsl-backup complete: /backup/qsl/snapshots/daily/daily-20260722T023405-0500`. **Up to thirty recoverable prior versions of every file in `tools/` existed the whole time.**
- **WHAT REMAINS TRUE, AND IS THE ACTUAL FINDING — this correction narrows the entry, it does not retire it.** There was **no diff, no history, no review path, and no revert-by-commit.** That was the real gap, it was worth fixing, and it was fixed. **The correction is recorded because an item that overstates its own severity distorts every sequencing decision that reads it** — this one was filed as the blocker gating WF-0027, WF-0029 and WF-0030, and the census that disproved the backup clause is what let all four be sequenced into a single lane instead of a custody negotiation. **Operator ruling: correct the claim, do not erase the finding.**
- **RESOLVED 2026-07-22 by NA-0667 (D-1293, directive D603).** `/srv/qbuild/tools/` is now a git repository (F1 = option (a): `git init` in place, no remote). 21 tracked files plus `.gitignore`; `backups/` excluded as an rsync-era artifact this history supersedes; a **baseline commit imported the tree byte-exactly BEFORE any edit**, so the WF-0030 registration and the OBS-2 fix each land as reviewable diffs against the state that ran every lane through NA-0666. Repo-local identity is set to the GH007 noreply address **deliberately**, so the new repo does not inherit the WF-0029 hazard. A pre-commit secret scan over the full tree returned three hits, **all false positives** — detector regexes inside `na0607-proof-tools/added_line_publication_scan.py` and a type name in a readiness checker.
- **⚠ Git is NOT a permissions restore path, and the two mechanisms are complementary, not redundant.** `tools/` is `drwxrwxr-x` (not setgid, unlike the rest of `/srv/qbuild`); `backups/` and `na0607-proof-tools/` are 0700 and `na0607-proof-tools/README.md` is 0600. Git preserves only the executable bit, so a git-based restore returns 0755/0644. **`qsl-backup` remains the permissions-faithful restore path; git is the diff/review/revert path. Keep both.**
- Status: **RESOLVED** — with the severity correction above standing as part of the record. **F1 = option (a) forecloses nothing:** promoting this to a satellite later is `git remote add` + `push`, history intact. **WF-0027 and WF-0029 are hereby UNBLOCKED** — this entry was their stated blocker; neither is fixed here.

### WF-0032 — `CLAUDE.md` is not a docs path, so editing the repo's own operating-instructions file costs a full-suite run on merge
- Severity: P2 (CI-economics/governance-maintenance; NO runtime, protocol, or security impact — the harm is a standing disincentive, not a break)
- Exact surface: `scripts/ci/classify_ci_scope.sh` — `is_docs_path()` at **`:4-16`**. The allowlist admits `tests/*.md` by regex, then case-matches `NEXT_ACTIONS.md`, `TRACEABILITY.md`, `DECISIONS.md`, `STATUS.md`, `README.md`, `START_HERE.md`, `SECURITY.md`, `SUPPORT.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `THIRD_PARTY_NOTICES.md`, `LICENSE`, and `docs/*`. **`CLAUDE.md` is absent.**
- Description: measured live, read-only, on this lane's branch — `bash scripts/ci/classify_ci_scope.sh docs/ops/DIRECTOR_OPERATIONS.md CLAUDE.md` → **`docs_only=false / runtime_critical=true / scope_class=runtime_critical`**, against the control `bash scripts/ci/classify_ci_scope.sh docs/ops/DIRECTOR_OPERATIONS.md` → **`docs_only=true / scope_class=docs_only`**. **Adding `CLAUDE.md` to an otherwise docs-only change set flips the whole set to `runtime_critical`.** `.github/workflows/macos-build.yml:102` gates `macos-qsc-full-serial` on `needs.classify.outputs.docs_only != 'true'`, and `qsc-linux-full-suite` is gated the same way, so **a one-line pointer edit to `CLAUDE.md` fires BOTH full suites on merge — approximately 132 min macOS + 158 min Linux at NA-0664's measured durations.**
- Consequence: **a structural disincentive to maintaining the very file that tells each lane how to operate.** Every correction to the operating instructions — including correcting instructions that are already wrong — costs roughly five hours of CI and a materially longer merge window than the edit deserves. **Inference, labelled as inference: this is plausibly part of why the operator-relay convention ended up buried in a `NEXT_ACTIONS.md` archive block rather than in `CLAUDE.md`, where a reader would look for it.** The cheap place to write it down was not the correct place, and the correct place was expensive.
- Recommended change: add `CLAUDE.md` to the `is_docs_path()` allowlist. It is a one-line change to a file this lane may not touch — the same shape as WF-0026, and for the same reason.
- Proof gap: nothing asserts which root-level governance files are docs paths, so the allowlist and the set of files that are actually documentation drift apart silently. A test pinning the intended set would catch both this omission and the next one.
- Sequencing: **this gates two deferred `CLAUDE.md` edits, and both ride the same carrier.** **(1)** The read-first pointer to the new `DIRECTOR_OPERATIONS.md §4` conventions, deferred out of NA-0666 by operator ruling at F1. **(2)** The correction of the now-superseded **`CLAUDE.md:47-50` step-6 response-file text** — see `DIRECTOR_OPERATIONS.md §4`, "Relay file versus response file", where the reconciliation is settled authoritatively. **Both land free once this item is fixed, or ride the first future lane already paying `docs_only=false` at zero marginal cost.** Until then the §4 text is authoritative and `CLAUDE.md:47-50` is known-stale.
- Status: open — filed 2026-07-22 by NA-0666 (D-1292) at operator direction. **FILING ONLY — the fix is lane (b).** **Filed at operator ruling at F1, and it is the reason the `CLAUDE.md` pointer is deferred out of the lane that wrote the rule the pointer would point at.**

### WF-0033 — `/srv/qbuild/cache/targets` is root-owned and not group-writable, so registering a NEW repo cannot provision its shared target root
- Severity: P3 (build-infrastructure provisioning; NO runtime, protocol, or security impact — but it silently withholds the shared-cache benefit that repo registration exists to deliver)
- Exact surface: `/srv/qbuild/cache/targets` is `drwxr-sr-x root victor` (verified live 2026-07-22). Its three existing children — `qsl-protocol`, `qsl-server`, `qsl-attachments` — are `drwxrwsr-x victor victor`, i.e. created when someone held root. The operating account `victor` (groups: `victor adm cdrom sudo dip plugdev users lpadmin`) has `r-x` on the parent and therefore **cannot create a fourth child.**
- Description: found by NA-0667 while registering `qsl-desktop`. `install -d /srv/qbuild/cache/targets/qsl-desktop` fails with `Permission denied`. `bootstrap_qbuild.sh`, which is the script whose job is creating exactly these directories, would fail the same way for the same reason.
- Consequence, stated precisely because the failure is quiet in one direction and loud in the other: **`qwork` still succeeds.** `qbuild_select_cargo_target` only *computes* the path and probes it — it does not create it — so a seat prints `startup_result=OK` with the correct keyed `cargo_target_dir` and merely reports `shared_target_ready=no`. **But `preflight_clean.sh` exits 1** (`Missing required qbuild path: /srv/qbuild/cache/targets/qsl-desktop`), and the first `cargo` build into that path would fail on `mkdir`. **So the registration looks complete and the shared-cache benefit — the entire point — is not actually available.** A reader checking only the qwork proof would not notice.
- Recommended change: one privileged command, matching the mode and ownership of the three siblings:
  `sudo install -d -o victor -g victor -m 2775 /srv/qbuild/cache/targets/qsl-desktop`
  The broader question — whether `/srv/qbuild/cache/targets` should be group-writable so that registering a repo is not a privileged operation at all — is **left to the operator and deliberately not answered here.**
- Proof gap: nothing asserted that a registered repo's target root exists. `check_repo_registration.sh` (added by NA-0667) now does, and **it is what surfaced this** — the check failed on its first real run and the failure was true.
- Sequencing: **blocks nothing in NA-0667** — registration, mirror, and the dispatch fix all landed. It blocks only the realisation of the shared-cache benefit for `qsl-desktop`, and it is the one acceptance item NA-0667 could not complete itself.
- Status: open — filed 2026-07-22 by NA-0667 (D-1293). **NA-0667 did NOT run any privileged command and did not attempt to work around the permission.**

### WF-0034 — 17.2 GB of stale flat-era cargo output sits beside the live keyed target trees
- Severity: P3 (disk hygiene; NO runtime, protocol, or security impact)
- Exact surfaces, measured live 2026-07-22: `/srv/qbuild/cache/targets/qsl-protocol/debug` — **15 G**, last modified **2026-05-26**; `/srv/qbuild/cache/targets/qsl-protocol/release` — **2.2 G**, last modified **2026-05-11**; `/srv/qbuild/cache/targets/qsl-protocol/.rustc_info.json` (4.0 K). Beside them, the LIVE keyed tree `rustc-1.95.0-x86_64-unknown-linux-gnu/` — 61 G, current. Also present: lane-named directories under `/srv/qbuild/cache/targets/qsl-server/` (`na0587`, `NA-0591`, `NA-0591-github-main`, `NA-0591-scratch`, …) consistent with old explicit `CARGO_TARGET_DIR` overrides.
- Description: residue of the NA-0543 migration from flat `cache/targets/<repo>` to toolchain-keyed `cache/targets/<repo>/<toolchain>/<class>`, compounded by the WF-0035 dispatch divergence — anyone using `env_qbuild.sh --print` built into the flat path, which is the parent of the keyed one. The dates bracket the migration.
- Consequence: ~17.2 GB unavailable for reuse, and a directory layout in which the live tree is nested inside a stale one — confusing to read and easy to misinterpret when reasoning about cache behaviour.
- Recommended change: delete the stale flat-level `debug/`, `release/` and `.rustc_info.json` under `qsl-protocol`, and triage the lane-named directories under `qsl-server`. **Verify nothing references them before deleting.**
- Proof gap: nothing asserts that `cache/targets/<repo>` contains only keyed subtrees.
- Sequencing: independent; do it whenever convenient.
- Status: open — filed 2026-07-22 by NA-0667 (D-1293) at operator direction. **⚠ FILING ONLY — NOTHING WAS DELETED.** Operator ruling, recorded because it is the reason this is a filing and not a cleanup: *"A registration lane does not get to silently delete 17 GB as a side effect."* This was **STOP condition 8** of D603.

### WF-0035 — `env_qbuild.sh` returned two different `CARGO_TARGET_DIR` values depending on how it was invoked
- Severity: P2 (build-cache correctness; NO runtime, protocol, or security impact — but it silently defeated the shared cache for anyone who used the documented `--print` interface)
- Exact surface (pre-fix): the direct-execution dispatch block sat at `env_qbuild.sh:140-160`, **above** the NA-0543 redefinitions of `qbuild_target_dir()` (`:188`) and `qbuild_export_repo_env()` (`:239`). Bash binds whichever definition exists at call time, so direct execution bound the earlier flat definitions and sourcing bound the later keyed ones. Measured live before the fix:
  `env_qbuild.sh --print qsl-protocol` → `/srv/qbuild/cache/targets/qsl-protocol` (flat)
  `source …; qbuild_export_repo_env qsl-protocol` → `/srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default` (keyed)
- Description: the file's own documented usage (`env_qbuild.sh --print [repo]`, per its `--help`) produced a target directory that no lane actually builds into. There was no warning and no divergence between the two paths other than the value itself.
- Consequence: anyone setting up a shell via `--print` built into an unshared, un-toolchain-keyed tree — a full cold rebuild that neither benefits from nor contributes to the shared cache. **WF-0034 is the measured residue of exactly this.**
- **RESOLVED 2026-07-22 by NA-0667 (D-1293, directive D603 F2).** Both dead flat definitions deleted; the dispatch block relocated to end-of-file with a comment stating that it must stay there and why. Verified: `--print` and `source` now agree for **all four** repos.
- Proof gap CLOSED: `check_repo_registration.sh` asserts invocation parity per repo, so moving the dispatch block back above the redefinitions fails loudly instead of silently.
- Status: **RESOLVED.** Filed and fixed in the same lane, as a correctness precondition of the registration it accompanied — the lane was adding a fourth repo to a function whose two definitions disagreed. **Recorded rather than fixed silently, because the 17.2 GB in WF-0034 is what a silent version of this bug already cost.**

### WF-0036 — the Claude Code guardrail hook is present, executable, and wired to nothing
- Severity: **Reporting-only** — deliberate-vs-drift is UNRESOLVED and the answer changes what, if anything, should be done
- Exact surface: `/srv/qbuild/tools/claude/qsl_guardrails_hook.sh` exists and is executable (`-rwxr-xr-x`, 2026-07-07). It self-documents as *"Claude Code PreToolUse hook (Bash tool matcher)"* that *"mechanically blocks Tier 5 forbidden command classes before execution"*, and its own installation note reads *"Wire up via .claude/settings.json (see companion settings file)."* `/home/victor/.claude/settings.json` (387 bytes, read live 2026-07-22) contains keys `permissions`, `model`, `statusLine`, `enabledPlugins`, `effortLevel`, `skipDangerousModePermissionPrompt`, `theme`, `switchModelsOnFlag` — and **no `hooks` key at all.**
- Description: found incidentally by the NA-0667 WF-0031 census while inventorying `/srv/qbuild/tools/`. The hook is a real, complete script that is not invoked by anything.
- Consequence: **whichever way it resolves, something is wrong and it is worth knowing which.** If the wiring was intended, a mechanical Tier-5 guardrail the governance may believe is active is silently absent. If it was deliberately retired, an executable script advertising itself as an enforcement aid is misleading to every future reader of `tools/`.
- Recommended change: **the operator determines intent first.** Then either wire it and verify it fires, or retire it explicitly with a note recording why. **Do neither on inference.**
- Proof gap: nothing asserts that hooks referenced by tooling are actually installed.
- Sequencing: independent.
- Status: open — filed 2026-07-22 by NA-0667 (D-1293) at operator direction. **REPORT-ONLY — NA-0667 neither wired nor removed it, and did not modify `settings.json`.**

### ENG-0062 — the NA-0665 GUI measurement harness has no durable home, and its `fitCode` replication is a silent-drift seam
- Severity: P2 (engineering-tooling/measurement integrity; NO runtime, protocol, or security impact — but a wrong number from a trusted harness is worse than no harness)
- Exact surfaces: `/srv/qbuild/tmp/NA0665_gui_round4a_20260722T051031Z/measure.py` — **5,708 bytes, 151 lines** (`wc -l`; D602 recorded 152, which counts the final terminated line — the byte count is exact and authoritative). The `fitCode` replication is at **`:73-84`**. Companions sharing the same disposable directory and the same hard-coded-`ROOT` pattern: `rig.sh` (**1,943 B**), `analyze.py` (**2,371 B**), `analyze2.py` (**2,427 B**) — all four verified live.
- Description: the harness drives a real GTK/WebKit window and measures rendered geometry. Dependencies: `python3`, PyGObject, GTK 3.0, and **WebKit2 4.1** — the last pinned to tauri's Linux engine, so the harness only tells the truth on the engine the product actually ships. Track record on NA-0665: it predicted **620** against the operator's hand-measured **621**, and matched all five after-shots to the pixel, net of the constant 66 px GNOME titlebar offset. **It works, it was decisive, and it currently lives in a `/srv/qbuild/tmp/` directory whose entire purpose is to be deleted.**
- **⚠ THE DESIGN CONSTRAINT, per operator ruling — the reason this is an engineering finding and not a filing chore.** `measure.py:73-84` replicates `fitCode` from `ui/main.js` **BY HAND**, in Python, as a transliteration of the JavaScript. **If `fitCode` changes and the copy does not, the harness keeps running, keeps printing numbers, and the numbers are silently wrong.** There is no version check, no shared source, and no assertion tying the two together. A measurement harness that fails loudly is an inconvenience; **one that drifts quietly is a source of confident wrong answers**, and this one has already been trusted to a one-pixel verdict. **The durable version MUST close this seam or fail loudly — it may not simply be moved as-is.**
- Also hard-coded and requiring attention in any move: the absolute `UI` path at `:21`, the six screen ids, and the backend strings lifted from `commands.rs:27-31`.
- Consequence: the harness is one `rm -rf /srv/qbuild/tmp/*` from gone, and its replication seam means a future GUI lane could re-derive it, trust it, and be wrong. Both risks are cheap to close and neither closes itself.
- Recommended change: **agreed home `qsl-desktop/tools/gui-measure/`, companions alongside.** It reads `ui/index.html` and replicates `ui/main.js`, so it must version **with** them — a spine home would guarantee the drift it needs to prevent. The move must also address the `fitCode` seam (share the source, generate the replication, or assert equivalence and fail loudly) and de-hard-code the `UI` path.
- Proof gap: nothing asserts that the harness's `fitCode` behaviour matches `ui/main.js`'s, and nothing asserts the harness's own predictions against a known-good fixture. Both are missing, and the second is what would make a silent drift loud.
- Sequencing: **blocked by WF-0030.** Until `qsl-desktop` is qbuild-known there is no clean lane in which to land the move — a desktop lane today runs without a startup proof, from a hand-made directory, which is precisely the condition this item is trying to escape.
- **⚠⚠ SEQUENCING CORRECTED 2026-07-22 by NA-0667 (D-1293) at operator direction — "blocked by WF-0030" WAS OVER-STATED, AND WAITING ON IT WOULD HAVE BEEN A MISTAKE.** This entry's Sequencing line reads *"blocked by WF-0030. Until `qsl-desktop` is qbuild-known there is no clean lane in which to land the move."* **That is a quality preference, not a technical dependency.** The agreed home is `qsl-desktop/tools/gui-measure/`, and landing it requires **a desktop PR — nothing more.** Nine desktop lanes (NA-0657 through NA-0665) landed desktop PRs from hand-made `/srv/qbuild/tmp/` directories; PR #6 merged `02cc9b96` on 2026-07-22. **The easy half was never blocked.** What actually flowed from WF-0030 is narrower and should be stated as such: a desktop lane landing the move now runs from a proper qwork workspace with a shared target cache instead of a tmp-dir cold rebuild — **a quality-of-execution benefit, not a precondition.**
- **⚠ THE HARD HALF IS INDEPENDENT OF WF-0030 IN BOTH DIRECTIONS.** The `measure.py:73-84` `fitCode` hand-replication of `ui/main.js` — the silent-drift seam this entry exists to close — is neither caused nor eased by qwork registration. **Un-blocking this item does not shrink it.**
- **⚠⚠ TIME-CRITICAL — THE ONLY COPY IS ON A DELETION CLOCK. MOVE IT BEFORE 2026-07-29 OR IT IS LOST.** Discovered by the NA-0667 census, not previously known: `qbuild-ssd-maintenance` runs daily at ~03:38 CDT and deletes `/srv/qbuild/tmp/NA*` directories whose **newest descendant** exceeds `TMP_DAYS=7` (`--tmp-days 7` in the live unit). `/srv/qbuild/tmp/NA0665_gui_round4a_20260722T051031Z/` holds the **only** copy of `measure.py` and its three companions; its newest-descendant mtime is **2026-07-22 11:20:24 CDT**, making it **deletion-eligible on or after 2026-07-29**, first eligible sweep **2026-07-29 ~03:38 CDT**. **And `/srv/qbuild/tmp` is in `qsl-backup`'s EXCLUDES list — there is no snapshot and no restore path.** This entry's "one `rm -rf /srv/qbuild/tmp/*` from gone" is not hypothetical: **it has a date.** **A copy placed anywhere outside `/srv/qbuild/tmp/` defuses the deadline immediately and independently of any lane.**
- **⚠⚠ THE DEADLINE IS DEAD — CORRECTED 2026-07-23 by NA-0668 (D-1294). DO NOT CARRY 2026-07-29 FORWARD AS URGENCY.** The preceding paragraph's *"MOVE IT BEFORE 2026-07-29 OR IT IS LOST"* **no longer holds, and leaving it unqualified would overstate a deadline and distort the next sequencing decision — the same failure mode the WF-0031 severity correction was made to prevent.** On 2026-07-22 17:44 the operator copied all four harness files to `/srv/qbuild/operator/preserved/gui-measure/`, and **D604's B0 brought `/srv/qbuild/operator` under daily backup on 2026-07-23.** Verified byte-identical in the post-install checkpoint: `measure.py` `7f84199e1ba0f1f7…`, `rig.sh` `18f59a8435ada4ad…`, `analyze.py` `96b4771918d8b616…`, `analyze2.py` `8d6dcc0088725de9…`. **The irreversible-loss risk is CLOSED.**
- **What remains true, stated precisely:** the **originals** are still at `/srv/qbuild/tmp/NA0665_gui_round4a_20260722T051031Z/`, still in `qsl-backup`'s EXCLUDES, and still deletion-eligible on/after 2026-07-29 — so that *path* will very likely stop existing. **That is now a path change, not a data loss.** **⚠ AND THE `fitCode` SILENT-DRIFT SEAM IS COMPLETELY UNTOUCHED BY ANY OF THIS** — a backup preserves a harness that can still produce confident wrong numbers. It is, and always was, the hard half.
- **⚠ CONSEQUENCE FOR SEQUENCING: the durable `qsl-desktop/tools/gui-measure/` home is owed AS GOOD PRACTICE, NOT AS RISK MITIGATION.** It should be scheduled on its merits — closing the drift seam — and **not** on a deadline that no longer exists.
- Status: open — filed 2026-07-22 by NA-0666 (D-1292); **sequencing corrected and the deadline recorded 2026-07-22 by NA-0667 (D-1293); the deadline RETIRED and the loss risk closed 2026-07-23 by NA-0668 (D-1294).** **The move is still a desktop PR, and NA-0668 did NOT make it.** **The four files were NOT moved or edited by NA-0666, NA-0667, or NA-0668** — NA-0668 only verified the operator's `preserved/` copies into the backup.

### WF-0037 — `new_checkout.sh` resolves an explicit `ref` from the mirror before origin is ever fetched
- Severity: P2 (checkout correctness; a live trap, **not** a demonstrated incident)
- Exact surface: `/srv/qbuild/tools/new_checkout.sh`
  `:58  git clone --origin mirror "$mirror" "$dest"`
  `:59  git remote add origin "$remote_url"` — added, **never fetched in this path**
  `:63  if git rev-parse --verify --quiet "mirror/$ref"` → `checkout -B "$ref" "mirror/$ref"`
  `:65  elif git rev-parse --verify --quiet "$ref"` → `checkout "$ref"`
- Description: when an explicit `ref` is supplied, it is resolved **entirely from the bare mirror**. If the mirror is stale, the resulting checkout is silently at an older commit than the same `ref` on origin.
- **RULED FILE-ONLY by D604 §3c**, on the operator's own instruction (*"if fetching origin before resolving an explicit ref is genuinely cheap, fix it here too; if it is a behavior change with any ambiguity, file only and say why"*). It is a behaviour change with ambiguity on **four independent axes**:
  1. **`$ref` is overloaded.** The `mirror/$ref` arm takes a branch; the bare-`$ref` arm takes a tag or a raw sha. `git fetch origin "$ref"` behaves differently for each and **fails outright for most raw shas** (servers reject unadvertised objects absent `uploadpack.allowReachableSHA1InWant`). A blanket fetch would break the sha case that works today.
  2. **It changes which commit a given input resolves to.** That *is* the fix, but it rewrites the contract of a helper whose own usage text says *"from the persistent bare mirror."*
  3. **It breaks an offline path.** Today this command works with zero network when the mirror holds the ref.
  4. **`--refresh` already exists** as the sanctioned opt-in for exactly this. The real question is whether the **default should flip** — and a default flip is a decision, not a side effect of a freshness lane.
- **Reachability, so the filing is not inflated:** `qwork` never passes a `ref`, so this is reachable **only by direct manual invocation** of `new_checkout.sh`, and the D604 census found **no evidence it has ever produced a wrong checkout.**
- Mitigation already delivered: D604 §3b bounds how stale any mirror can become, which shrinks this trap without touching the contract.
- Proof gap: nothing asserts that an explicitly-requested `ref` resolves to the same commit the origin advertises.
- Sequencing: independent. The decision to make is the `--refresh` default flip.
- Status: open — filed 2026-07-23 by NA-0668 (D-1294). **`new_checkout.sh` is byte-unchanged by that lane.**

### WF-0038 — mirror freshness was never asserted anywhere, and the class is "an artifact that reads as safe and isn't"
- Severity: P2 (tooling correctness and, more importantly, **assurance integrity** — several checks reported health while meaning something weaker)
- Exact surfaces (pre-fix): `check_repo_registration.sh:51`, `qshell.sh:167`, `qwork.sh:106` — all three tested that the mirror **directory existed** and treated that as sufficient. **The word "commit" appeared nowhere in `check_repo_registration.sh`.** Live run at D604 drafting: `4 repos checked, 0 issue(s)` against a mirror set that had been stale for months at a time.
- **RESOLVED IN PART 2026-07-23 by NA-0668 (D-1294, directive D604 §3a/§3b).** `check_repo_registration.sh` gained a three-state freshness assertion (CURRENT/ok/0, STALE/FAIL/1 naming both shas, UNREACHABLE/"could not verify"/2, with exit 1 outranking exit 2 so a network failure cannot mask a real staleness finding); `qshell.sh` and `qwork.sh` now refresh the mirror on **every** worktree creation rather than only when the directory is missing.
- **⚠ Honest limit, recorded so this is not over-valued:** neither change repaired a live incident path. `qwork.sh:372` and `qshell.sh:139` already fetch origin and hard-assert `head == origin/main`. §3a is detection; §3b is a structural staleness bound. `preflight_clean.sh:54` is existence-only **by design** and was deliberately left alone.
- **⚠ WHY IT SURVIVED — this is the substance of the entry, not a footnote.** The defect was **not** undetected. It was observed by name **four times** and worked around every time:
  - **2026-06-03** `DIRECTOR_QWORK_STARTUP_RECOMMENDATION.md` — reports `## main...mirror/main [ahead 778]` and asks that a startup command *"normalize or explicitly report this state."* **That document is the recommendation that produced `qwork`.**
  - **2026-07-11** `DIRECTOR_LIVESTATE_NA0636….md:5` — *"the local `/srv/qbuild/mirrors` copy is stale (2026-04-28)"*, worked around with `gh` API queries and a disposable shallow clone.
  - **2026-07-12** `DIRECTOR_READONLY_INVESTIGATION_E2E_INTEGRATION….md:111` — *"the local mirror … is stale (HEAD Mar 29 vs GitHub Jul 6)."*
  - **2026-07-13** `DIRECTOR_READONLY_INVESTIGATION_SELF_HOST….md:229`, under *Unknowns / flagged* — *"~14 PRs behind, missing the pin."*
  - Plus **46 lines** in `ROLLING_OPERATIONS_JOURNAL.md` matching stale-`mirror/main` phrasing, recurring as *"Initial worktree was clean but still checked out at stale local `mirror/main` state"* — each recorded as a **recovered** condition. **46 is a count of matching journal lines, hand-inspected for phrasing, NOT a count of distinct lanes, and is cited as a floor** (NA-0664 rule).
- **The named principle: *"a recovery convention that always works removes pressure to fix"* (NA-0664), recurring on a second surface.** The 2026-06-03 sighting is the sharpest form of it: the stale mirror was a **named motivation for building `qwork`**, and `qwork` was built to normalize *around* the staleness (fetch origin, ff-merge, assert) rather than to fix it. **The workaround was institutionalised in tooling** — which is precisely why `qwork.sh:106` still carried the existence-only check until NA-0668. **It is the reason the fix had to be an assertion that FAILS rather than a note a human reads.**
- **⚠ THE CLASS, IN SIX COSTUMES — one finding, not six.** *An artifact that reads as safe and isn't.*
  1. **An existence-only check that reads as health and means presence** (the three sites above).
  2. **A workaround that always worked and therefore removed the pressure to fix** (the four sightings, the 46-line floor).
  3. **A documented, correct-to-follow procedure that would have silently regressed the very thing the lane exists to protect** — the backup packet's own *Change Procedure* pointed at a workspace copy **21 diff lines stale**; following it literally would have dropped `/home/victor/work/qsl/codex/ops`, `/home/victor/work/qsl/claude` and **`/home/victor/.claude`** from `daily_sources`, plus four excludes and five manifest sections. **A lane opened to close a backup gap would have opened three larger ones.**
  4. **⚠ A VERIFICATION INSTRUCTION THAT READS AS AUTHORITATIVE AND IS WRONG** — and it is the strongest evidence here precisely because it is **self-implicating** and was caught **by testing rather than by an incident.** The Director's own B0 package shipped **four wrong `EXPECT` lines**, found only because each was executed against a scratch copy of the real script instead of asserted. The worst promised diff output `7a8` / `>   /srv/qbuild/operator` where the truth is `39d38` / `<   /srv/qbuild/operator` — **wrong in both direction and line numbers, at the one step whose entire stated purpose is "anything else ⇒ STOP."** A wrong halt condition does not merely fail; **it trains the operator to distrust halt conditions**, which is this class turned on the lane itself. Also wrong: `wc -l` vs `grep -c` for the staleness count, and a dry-run grep against a stream that contains no file paths at all, because the script runs rsync without `-v`.
  5. **⚠ A FIFTH INSTANCE, IN THE SAME PACKAGE, AFTER THE STANDING METHOD WAS ADOPTED.** Step 7.3 shipped `EXPECT: 582+` for the response count; the operator's real run returned **576**, because the census had counted with `ls dir/*` and the glob **expanded the `director_handoff/` subdirectory**, absorbing its 5 entries plus `ls`'s header and blank line. Ground truth: **576 top-level entries = 575 response files + 1 subdirectory; 580 files recursively.** The subtree size was corrected the same way: **37 MB → 48 MB**, the 37 being a `du` multi-argument deduplication artifact (true total **48,390,098 bytes across 800 files**). **The wrong expectations survived because the standing method was applied only to the commands whose output was a *diff*, and not to the ones whose output was a *count*.** Both were corrected in place with their explanations rather than quietly dropped.
  6. **⚠ A SIXTH, INSIDE NA-0668, BY THE EXECUTOR, WHILE WRITING UP THE OTHER FIVE.** Verifying B0's codex coverage, the executor referenced the checkpoint as `checkpoint-20260723T083238-0500`, **dropping the `-after-operator-source-added` label suffix.** The path did not exist. Every `test -f "$CP/…" && echo COVERED || echo MISSING` printed **MISSING**, and `find … 2>/dev/null | wc -l` printed **0** — a confident, fully-formatted, **entirely false** report that B0's second source line had silently failed, which was nearly recorded as a material finding against the operator's completed work. **`test -f` cannot distinguish "file absent" from "parent directory absent," and `2>/dev/null` erased the one signal that would have exposed it.** Caught only by cross-checking an earlier run that used the full path and returned 807. Re-verified correctly: **10/10 codex files covered, packet byte-identical.**
- **⚠ COSTUMES 6 AND 7 SHARE ONE SHAPE, AND NAMING IT IS WHAT MAKES THEM ACTIONABLE** (operator ruling, 2026-07-23 closeout). **In both cases the tool answered a question ADJACENT to the one intended, and the answer was shaped like an answer to the intended question.**
  - `test -f` against a path missing its label suffix answered *"does this nonexistent path exist?"* — **no** — not *"did B0 back up these files?"*
  - `git check-ignore` against a **tracked** file answered *"is this tracked file ignored?"* — **no, tracked files are exempt** — not *"is this directory ignored?"*
  **They failed in OPPOSITE DIRECTIONS from the same defect:** one produced a **false alarm against completed work**, nearly relayed to the operator as a finding; the other a **false all-clear** that nearly shipped a PR with no as-built. **The instrument was pointed slightly off the question.** That is why neither was caught by re-reading the output — the output was *well-formed and internally consistent*; only the question was wrong.
- **THE STANDING METHOD, narrowed by instances 5 and 6 and recorded as this entry's remedy:**
  **EXECUTE EVERY OPERATOR-FACING EXPECTATION BEFORE SHIPPING IT — INCLUDING THE ARITHMETIC ONES.** A shell glob is not an inventory, exactly as a grep is not a measurement (NA-0664).
  **⚠ AND THE GENERALIZED FORM, WHICH SUPERSEDES THE FIRST DRAFT OF THIS CLAUSE** (operator ruling, 2026-07-23 closeout):
  > **A NEGATIVE RESULT IS ONLY EVIDENCE IF THE INSTRUMENT COULD HAVE RETURNED POSITIVE.**
  The first draft read *"…only evidence if the PATH it was measured against exists,"* which is **too narrow — it covers costume 6 and misses costume 7 entirely**, since `check-ignore`'s path existed perfectly well and the instrument was still incapable of answering the question asked. The general rule covers both, and is discharged in practice by running a **positive control**: point the same instrument at a case known to be positive, and only trust the negative if the control comes back positive. **WF-0041 was answered that way deliberately** — the same greps returned 8 and 3 real references for other scripts, and 3 and 2 mentions in the read-first docs, before its zero was accepted as a finding.
- **The entry's first draft closed by predicting a fourth instance; it arrived within the hour, inside the package written to fix the first three. A fifth followed after the remedy was adopted, and a sixth inside the lane that wrote the remedy down.** That progression is the argument for making the method mechanical rather than intentional.
- Proof gap CLOSED for `main`: `check_repo_registration.sh` now fails loudly on a stale mirror. **NOT closed for `refs/pull/*` or tags** — the assertion compares `refs/heads/main` only, and that limit is stated in the check's own comments.
- **⚠ THE EVIDENCE PAIR IS COMPLETE ON A REAL MIRROR, NOT A FIXTURE — recorded 2026-07-23 at closeout.** The fail half was captured unprompted on the check's **first live run**: `qsl-protocol`'s mirror two commits behind origin, and **the two commits were NA-0668's own queue promotion and its merge.** The pass half was captured by the operator after refreshing: **`4 repos checked, 0 issue(s), 0 unverified`, exit 0, each repo printing `mirror CURRENT at <sha>`.** **Acceptance §5.B.6 is therefore CLOSED at 4/4**, and the lane holds a complete FAIL→refresh→PASS pair against production mirrors rather than only against the throwaway fixture.
- **⚠ AND THE MIRROR WENT STALE A THIRD TIME WITHIN THE HOUR, WHICH IS DATA AND NOT A DEFECT.** Merging PR #1624 advanced origin `8a05c1a3` → `565d480c`, so the same check now reports `qsl-protocol` STALE again. **Every spine merge re-stales the spine mirror.** This is not a regression and needs no chasing — §3b self-heals it at the next lane seat — but it is the measured decay rate behind **WF-0041**, and it means *"is the mirror set 4/4 right now?"* is a question with a shelf life of one merge.
- **NON-FATAL REFRESH RULING AFFIRMED** (operator, 2026-07-23), reasoning recorded because the alternative is superficially the safer-looking choice: `qwork.sh:372` and `qshell.sh:137` already fetch origin and hard-assert `head_equals_origin_main` **after** the mirror seeds the clone, **so a stale mirror CANNOT produce a wrong checkout — the refresh is hygiene, not a correctness gate.** Making it fatal would block work on a network blip for **zero correctness gain**, which is *"a gate that cannot pass teaches bypass"* **arriving inside the fix for it.** Keeping the **absent**-mirror case fatal preserves the real invariant, since there the refresh *is* the clone.
- Status: **RESOLVED IN PART** — the three existence-only sites are fixed and the fail/pass evidence pair is complete on real mirrors. **The DETECTION half has no owner: see WF-0041.** The class remains open as a review discipline, and `WF-0037` is the nearest un-fixed instance.

### WF-0039 — `DOC-OPS-002` enumerates three repos and predates `qsl-desktop`
- Severity: P3 (documentation currency; same shape as the defect WF-0030 fixed in `env_qbuild.sh`)
- Exact surface: `docs/ops/DOC-OPS-002_Continuity_Snapshot_Manifest_and_Offhost_Procedure_v0.1.0_DRAFT.md` §3, which fixes snapshot contents over **three** repos. `qsl-desktop` became a governed satellite at NA-0656 / D-1279 and a qbuild-known repo at NA-0667 / D-1293.
- Description: a hard-coded repo enumeration in a procedure document, drifting exactly as the hard-coded repo lists in `env_qbuild.sh` did.
- Consequence: any snapshot built to this document's §3 would omit `qsl-desktop` and be believed complete.
- Recommended change: **v0.2.0.** Owed regardless of whether any off-host archive is ever built.
- Sequencing: this is B1's subject matter. **D604 §7 explicitly forbids revising it in-lane.**
- Status: open — filed 2026-07-23 by NA-0668 (D-1294). Not revised by that lane.

### WF-0040 — `prune_snapshots()` never prunes failed runs, so `.incomplete-*` directories accumulate permanently
- Severity: P3 (backup housekeeping; no data-loss risk)
- Exact surface: `/usr/local/sbin/qsl-backup`, `prune_snapshots()` — it filters `! -name '.incomplete-*'` from **both** its keep list and its prune list, so a failed run's residue is excluded from consideration in both directions and is never removed.
- Description: two are present, `.incomplete-daily-20260603…` and `.incomplete-daily-20260604…`.
- **Deliberately NOT fixed by NA-0668.** It touches the same root-owned script as D604's B0, and B0's privileged diff was required to be exactly two source lines. **A housekeeping fix must not ride along on a privileged step whose diff is meant to be reviewable at a glance.**
- Recommended change: include `.incomplete-*` in the prune list only, on its own deliberate change with its own checkpoint/preflight/dry-run/install cycle.
- Sequencing: independent; pairs naturally with tidying the now-redundant `codex/logs`, `codex/responses`, `codex/ops` entries left in place by B0's bounded amendment.
- Status: open — filed 2026-07-23 by NA-0668 (D-1294). **Nothing deleted** (D604 STOP-7).

### WF-0041 — the mirror-freshness DETECTOR has no owner: nothing runs `check_repo_registration.sh`, and nothing tells anyone to
- Severity: P2 (assurance ownership). **The check itself is correct and proven — see WF-0038. This entry is about whether it is ever RUN.**
- **THE ANSWER, ASKED DIRECTLY BY THE OPERATOR AT NA-0668 CLOSEOUT AND MEASURED RATHER THAN ASSUMED: NOTHING RUNS IT AUTOMATICALLY, AND NOTHING EVEN ASKS ANYONE TO.**

| candidate mechanism | result |
|---|---|
| systemd **system** timers | 14 present; the only two project timers are `qsl-backup-daily` (`ExecStart=/usr/local/sbin/qsl-backup daily`) and `qbuild-ssd-maintenance`. **Neither invokes it.** |
| systemd **user** timers | 2 present, both OS/snap. **No.** |
| `crontab -l` (victor) | `no crontab for victor` |
| `/etc/cron.d`, `/etc/cron.daily` | OS defaults only (`anacron`, `e2scrub_all`, `sysstat`, `apport`, `apt-compat`, `dpkg`, `logrotate`, `man-db`) |
| Claude Code hooks | `/home/victor/.claude/settings.json` has **no `hooks` key at all** — the same wiring gap as **WF-0036** |
| spine CI workflows | no `.github/**` reference |
| `qwork` / `qshell` / `preflight_clean.sh` / `bootstrap_qbuild.sh` / any of the 15 tools scripts | **ZERO non-comment references.** The only two mentions anywhere in `/srv/qbuild/tools/` are **comments written by NA-0668 itself** at `qwork.sh:111` and `qshell.sh:177` |
| `CLAUDE.md`, `START_HERE.md`, `AGENTS.md`, `DIRECTOR_OPERATIONS.md`, DOC-OPS-003 | **zero mentions.** Every spine reference to the script is *narrative* — governance records describing what it does — and **none is a procedural step in any checklist, runbook, or read-first list.** |

- **⚠ THE NEGATIVE RESULT WAS VALIDATED WITH POSITIVE CONTROLS, per WF-0038's remedy.** A sweep that finds nothing proves nothing unless the instrument could have found something. The identical `grep` over the identical file set returned **8** references for `refresh_mirrors.sh` and **3** for `new_checkout.sh`, including real invocation lines; the identical read-first-doc sweep returned **3** `qwork` mentions in `CLAUDE.md` and **2** in `START_HERE.md`. **The instrument was demonstrably capable of returning positive and did not.**
- **Consequence, stated in the operator's own framing: the detector inherits the exact shape of the artifact it replaced.** WF-0038's defect was a check that *read as health and meant presence*. Its replacement is a check that reads as health **whenever nobody looks — and nobody looks until something is already wrong.** §3a made the assertion correct; it did not give it an owner. **A correct detector that runs only when a human remembers it exists is a memory dependency wearing a check's clothing.**
- **⚠ THE DECAY RATE IS MEASURED, NOT ESTIMATED — AND IT IS FASTER THAN THE ONLY EXISTING CADENCE.** `/srv/qbuild/mirrors/qsl-protocol.git` went stale **three times on 2026-07-23 alone**: current after the operator's 02:13Z refresh → **stale** when PR #1623 merged (caught unprompted by the check's first live run) → current after the operator's refresh, verified 4/4 → **stale again** when PR #1624 merged (`8a05c1a3` → `565d480c`). **Every spine merge staleness the spine mirror.** The nearest automated cadence on the box is daily; the actual decay event is per-merge.
- **⚠ HONEST BOUNDING, so this is not inflated into an emergency.** D604 §3b substantially defuses the *operational* consequence: every new worktree creation now refreshes the mirror it clones from, so a stale mirror **self-heals at the next lane seat** and cannot produce a wrong checkout in any case (`qwork.sh:372` and `qshell.sh:137` fetch origin and hard-assert `head == origin/main`). **The residual risk is narrower and should be stated as such:** a repo in which **no lane is ever seated** — `qsl-attachments` is the live candidate — receives no §3b refresh at all, so **only §3a would ever notice its drift, and only if somebody runs it.** That is the gap this entry names.
- Recommended change (**NOT implemented here** — filed at operator instruction, and D604's scope forbids it): give the detector an owner. Options, in ascending cost, **none chosen**: (a) a systemd timer alongside `qsl-backup-daily`, cheapest and matches the existing pattern but a daily cadence under-samples a per-merge decay; (b) invoke it from `qwork` at lane seat, which samples exactly when it matters but couples a read-only census to the seat path and would have to be non-fatal (D604's non-fatal ruling applies with equal force); (c) wire it as the Claude Code `PreToolUse`/session hook that `settings.json` has no key for — **which would also resolve WF-0036, and the two should probably be decided together**; (d) add it to a read-first checklist, which is the cheapest and is **explicitly the weakest**, since it re-implements the memory dependency this entry exists to remove.
- Proof gap: nothing asserts that any assurance check in `/srv/qbuild/tools/` is actually reachable from an automated trigger. **This entry is one instance; the general form is unmeasured** — `preflight_clean.sh` and `capture_evidence.sh` were not audited for the same property by NA-0668.
- Sequencing: independent of everything NA-0668 landed. **Pairs naturally with WF-0036** (a hook that exists and is wired to nothing) — the two are the same question from opposite ends: one is wiring with no script anymore, the other a script with no wiring.
- Status: open — filed 2026-07-23 by NA-0668 closeout (D-1294) **at operator instruction, answered and filed, deliberately NOT fixed in-lane.**
