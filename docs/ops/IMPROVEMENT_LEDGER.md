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
- Status: open — filed NA-0617 (D-1230) from the external Suite-2 code/crypto review
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

### ENG-0013 — Suite-2 symmetric counter (ns/nr) overflow hard-stop missing
- Severity: P2 (nonce-reuse-class at saturation; bounded precondition)
- Status: queued — filed NA-0617 (D-1230) from the Suite-2 review (H-1); selected as the
  NA-0618 successor; last-updated 2026-07-07
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
