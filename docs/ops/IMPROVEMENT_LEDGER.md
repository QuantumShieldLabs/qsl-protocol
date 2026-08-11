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

### Recording closure: the `Resolution:` line (adopted 2026-07-29 by NA-0687 / D-1326)

**Closure is recorded by APPENDING a `Resolution:` line to the entry. Original `Status:`
lines are never edited.** A grep for open items therefore keys on the **absence** of a
`Resolution:` line, never on the text of `Status:`.

⚠ **Why this exists.** Because the house rule is *mark-don't-rewrite*, closures were being
recorded as prose annotations while the original `Status: open` stayed put — so the field
that looks authoritative was wrong for every closed entry. It was worse than a stale field:
NA-0686's seven closures opened with **three different verbs** — `CLOSED` (ENG-0075, 0082,
0084, 0088, 0089, 0090), `DISPOSITIONED … FIXED, not deferred` (ENG-0085) and
`RE-ENUMERATED AND RESOLVED` (ENG-0087) — so no grep could find them and a reader had to
read all 91 entries in full to learn which were live. A backlog whose own state cannot be
queried is an instrument that does not instrument, which is the defect family
ENG-0077/0078/0091 belong to. This convention makes the state machine above **greppable**
without rewriting a single historical claim.

⚠ **THE PARTIAL-CLOSURE RULE, and it is the load-bearing half: an entry with ANY part still
open NEVER receives a `Resolution:` line, however emphatic its annotation.** Two live cases
decided this at adoption. **ENG-0087** is annotated *"RE-ENUMERATED AND RESOLVED"* but its
~60-scrape annex is still owed, so it gets an annotation and **no** `Resolution:`.
**ENG-0091** had its pattern fixed at all twelve sites by the very lane that adopted this
convention, and still gets **no** `Resolution:` because one of its two recorded instances
still fails by a second, separately filed mechanism (ENG-0094). *Had the rule been the
other way, the convention's first two uses would both have reported open work as closed —
failing at exactly the job it was adopted to do.*

Closing an entry therefore means: append `Resolution:` naming the lane, the decision id and
what was measured; leave `Status:` and every prior annotation byte-identical.

### The NA-0709 triage (D-1346, 2026-08-10) — how every entry got its status

**Every ENG entry in this file carries a status established by MEASUREMENT at spine main
`b845e678`.** The method, so it can be re-run rather than trusted:

**Three closure instruments exist in this file, and they are three ERAS, not three fields.**
**I1** a bold closure verb in the `###` heading (pre-convention) · **I2** an appended
`Resolution:` line (the documented convention, NA-0687/D-1326) · **I3** a closure verb opening
the `- Status:` bullet. Measured totals **I1=10 · I2=27 · I3=30**, and the agreement matrix is
`none` 118 · `I2` 19 · `I3` 14 · `I2+I3` 8 · `I1+I3` 8 · `I1` 2 — ⚠ **`I1+I2+I3` = 0. No entry
is closed by all three, and I1 and I2 never co-occur.** An entry is CLOSED if ANY fires ⇒
**CLOSED 51 · OPEN 118** of 169 ENG entries.

⚠ **Every instrument used in the direction it supports and no further.** The partial-closure
rule below means **I2 firing is sound evidence of CLOSED, while I2 NOT firing is not evidence
of open.**

⚠ **THE DOCUMENTED QUERY AT `:41-43` OVER-REPORTS OPEN BY 24.** Keying on the absence of a
`Resolution:` line returns **142** open; the three-instrument union returns **118**. The
difference is **pre-convention closure plus an unaudited residue**: 24 entries were closed by
instruments the document's own query rejects, and **they have not been individually re-read.**
The convention was adopted 2026-07-29 and never back-applied. ⚠ **Recorded, deliberately NOT
repaired** — back-applying `Resolution:` lines is itself a closure act, and a lane that both
performs and adjudicates its own closures has no gate.

⚠ **A calibration note for anyone re-deriving the matrix:** I1 and I3 do not use the same
vocabulary. I1 needs four verbs; **I3 needs eight** lead tokens (adding `wontfix`,
`remediated`, `superseded`, `resolved-into-findings`). With I1's four-verb list, I3 measures
24, not 30 — and a reader would wrongly conclude this record is broken.

**Result of the triage:** 82 open entries were cold-read against source; 36 filed the previous
day were statused from their own filing text. **LIVE 99 · moved to DOC-OPS-008 13 ·
UNDECIDABLE 7 · SUPERSEDED 1 · CLOSED 0.**

⚠ **THE CLOSURE COLUMN IS EMPTY, AND THAT IS THE RESULT.** The triage produced exactly one
closure and its own adversarial read withdrew it. **Nothing in this ledger closes by
re-reading; the open set is real.**

⚠ **A STANDING FORM THIS TRIAGE ENTERED — residue survives only if it has an ID.** A paragraph
inside a closed entry is not a queue item, and a defect that is not in the queue is not looked
for. Two entries had orphaned residue and it now has ids: **ENG-0084 → ENG-0171** and
**ENG-0038 → ENG-0172**.

⚠ **AND CITATION ROT IS THE NORM, NOT THE EXCEPTION.** Line numbers in these entries drift and
some are now wrong by hundreds of lines or point at deleted files. **Any lane acting on an
entry must re-derive its surfaces from the entry's DESCRIPTION, never from its citation.**


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

### ENG-0011 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0011`.

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

### ENG-0022 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0022`.

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

### ENG-0027 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0027`.

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

### ENG-0029 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0029`.

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

### ENG-0088 — the claim-discipline guard family covers neither Cargo metadata nor module docs, and a retired claim survives in both — **NEW; filed 2026-07-29 by NA-0683 (D-1321; directive D618 F5, operator-ruled LEAVE-and-FILE)**
- Severity: P3 (claim discipline / assurance; a published crate description states something the product outgrew)
- Status: open — filed 2026-07-29. **Deliberately not fixed in-lane**: it needs a wording decision NA-0683 was not authorised to make, and the naming lane changed exactly one word per line.
- Exact surfaces: `qsl-desktop` `src-tauri/Cargo.toml:6` (`description = "QSL desktop client — slice A: serverless skeleton …"`) and `src-tauri/src/lib.rs:1` (module doc, same phrase). The guard that retired this phrase is `src-tauri/tests/server_pane.rs::claim_discipline_five_surfaces_swept` (`:403-422`).
- Claim violated: slice B shipped relay connectivity; "serverless skeleton" is no longer true of the crate it describes.
- ⚠ **Why it matters, and why it is a needle gap rather than a typo:** the guard asserts the phrase is absent from `ui/index.html` and `src-tauri/src/commands.rs` **and nowhere else**. Cargo metadata is published (it reaches package registries and any bundle manifest) and module docs reach `cargo doc`. **The defect is the gap in the needle set, not the word** — a claim-discipline sweep that stops at the two files a previous lane happened to name will keep missing the surfaces nobody thought of.
- Minimal fix direction: extend the claim-discipline needles to `Cargo.toml` and to `src/**.rs` module docs, then correct both strings in the same lane. Do not fix the strings without extending the needles, or the next stale claim lands in the same blind spot.
- Proof gap: no test reads `Cargo.toml` or any module doc for claim discipline.
- Recommended shape: **CI/tooling lane**, implementation-only, alongside ENG-0089.
- ⚠ It is visible rather than hidden today: NA-0683's naming gate prints it as its single `RULED-LEAVE` entry on every run, by operator ruling F5.
- Cross-reference: D-1320; D-1321; D618 §4 F5; NA-0683 as-built §2.1 and §9.

- ⚠ **CLOSED 2026-07-29 by NA-0686 (D-1325).** Both surfaces corrected to the
  operator-pre-approved wording, and — per this entry's own instruction — **the needles were
  extended FIRST**: `claim_discipline_covers_cargo_metadata_and_module_docs` covers
  `src-tauri/Cargo.toml` and `src-tauri/src/lib.rs`, asserting both the absence of the retired
  claim and the PRESENCE of the research-stage / no-security-assurance boundary.
- ⚠ **This entry's description of the gap was slightly wrong, and the correction sharpens the
  lesson.** It recorded that the old guard asserts absence from `ui/index.html` and
  `src-tauri/src/commands.rs` *"and nowhere else"*. Measured: the guard reads **five** files —
  `index.html`, `main.js`, `lib.rs`, `commands.rs`, `README.md`. **`lib.rs` WAS in the needle
  set**, but only for the phrase *"makes no network connections"*, never for the retired
  slice-A phrase. So the gap was not "a file nobody looked at"; it was **a file looked at for
  the wrong needle** — harder to spot, and a better argument for this entry's own conclusion
  that the defect is the needle SET rather than the word.
- Red control, both directions: reintroducing the retired phrase into either surface turns the
  new guard red, naming the surface; both restored byte-identical (`cmp`).
- ⚠ **The guard caught the lane writing the defect back in.** The first draft of the
  explanatory comment in `lib.rs` QUOTED the retired phrase, and the new guard failed on it —
  the same trap the older guard's comment warns about. Recorded because it is direct evidence
  the needle works on live content, not just on the case it was written for.
- Resolution: CLOSED 2026-07-29 by NA-0686 (D-1325) — the claim-discipline needles now cover Cargo metadata and module docs, and both stale strings were corrected; the needle was proved against live content, not only the case it was written for. (This `Resolution:` line was applied 2026-07-29 by **NA-0687 / D-1326** when the convention was adopted; the closure itself is NA-0686's and its annotation above is unchanged.)

### ENG-0089 — `host_retired_rig` fires on ADDED LINES ONLY, so the tree is only as clean as its last edit — **NEW; filed 2026-07-29 by NA-0683 (D-1321; OBS-8)**
- Severity: P2 (gate design; it blocks correct work and its clean signal is weaker than it reads)
- Status: open — filed 2026-07-29, with an operator question attached (below).
- Exact surfaces: `scripts/ci/infra_literal_scan.py` (byte-identical across the four repos) in `--mode diff` / `--mode staged`; the pattern class `host_retired_rig`; the Tier-1 `--mode tree` class set that omits it. Observed instance: `qsl-desktop` `docs/DESIGN_SPEC_AppendixF.md:239`.
- ⚠ **The failure shape, measured:** NA-0683's one-word edit to that line was refused by the pre-commit gate for a literal **already present on `main`**. `--mode tree` reports the same tree **clean**, because the class applies only to added lines. **Any lane that touches a legacy line inherits a gate failure it did not create**, and is then pushed toward either an out-of-scope redaction or a bypass.
- Why it matters: a gate whose clean signal means "nobody has edited the dirty lines yet" is weaker than it reads, and the incentive it creates on a correct change is to work around it.
- ⚠ **OPERATOR QUESTION FOR THE CI/TOOLING LANE (ruled 2026-07-29):** **promote `host_retired_rig` to Tier-1 tree-wide once the known instances are zeroed.** Sequencing matters — promoting first would turn every repo red on content already published.
- Minimal fix direction: (1) the approved sanitization micro-lane zeroes the known instances, taking `AppendixF:239` **whole** — hostname → placeholder **and** NA-0683's deferred one-word relay fix (D-1320); (2) then Tier-1 adopts the class tree-wide; (3) the scanner reports which tier caught a hit so the two signals are never conflated.
- Proof gap: no control proves the diff-mode class can fire on a line the tree-mode class ignores — NA-0683 produced that evidence by accident rather than by design.
- Recommended shape: **CI/tooling lane**, implementation-only, sequenced **after** the sanitization micro-lane.
- Cross-reference: D-1320 (F1's deferred 14th line); D-1321; NA-0683 as-built §3; D613 (the gate's origin).
- ⚠ **SHARPENED 2026-07-29 by NA-0684 (D-1323), and the question changed.** This finding was filed as *"promote `host_retired_rig` to Tier-1 or not"*. **NA-0684 met the failure twice in one lane and the real question is: WHAT DOES A LANE DO WHEN IT MUST RE-ADD A LINE IT IS NOT ALLOWED TO CHANGE?** The sanitization lane's ruled edits landed on lines that also carried a **remote account name** — a Tier-2b class, clean on `main` for months, which became an *added line* the moment the lane touched it — while a flag of that same lane had ruled the account name **stays**. **A gate refused a correct, ruled change, and no amount of Tier-1/Tier-2 tuning answers that.**
- ⚠ **THE ANSWER IS RECORDED (operator, 2026-07-29, "Option B"), and the CI/tooling lane implements against it rather than re-deriving it:** **(a)** any line a lane re-adds carries **no Tier-2b literal** — the literal is **placeholdered as part of that edit**; **(b)** **grandfathered lines stay** — untouched legacy content remains under report-don't-touch, which is the tier's *designed migration semantics*, not an exemption; **(c)** the gate and the redaction rule (*name the field, never the value*) are **one policy**, and on an added line **the gate wins**. Measured consequence in NA-0684: one authority doc now carries the token on one line and a placeholder on the four beside it — **grandfathering is visible in the tree, and that is correct**.
- ⚠ **ALLOWLIST INPUT, so this lane meets known exceptions rather than discoveries:** NA-0684's census is the starting set — the historical **proof labels** whose text names a past artifact, the **10 tracked file paths** whose names carry the rig token (renaming them is history churn and breaks every pointer), and the **796-occurrence record class**. See `docs/governance/evidence/NA-0684_as_built.md` §10.
- ⚠ **A SECOND CLASS THIS GATE CANNOT SEE, found by NA-0684:** **CGNAT tailnet addresses** — 40 occurrences across 12 files and two distinct addresses, ~14 of them in live reproduction commands. The structural patterns cover RFC1918, the tailnet *hostname* form and the mail domain; **`100.64/10` matches nothing**. The durable fix is a **CGNAT Tier-2b class** so that future edits to such lines ship clean. **Sequencing (ruled): the tailnet sanitization micro-lane runs FIRST, then this lane's promotions** — otherwise a promoted gate goes red on already-published content, which is the same hazard this finding was filed about.
- ⚠ **THE SEQUENCING PRECONDITION IS NOW SATISFIED — NA-0685 ran and the known instances are ZEROED (D-1324, 2026-07-29).** Re-censused fresh at `b31730ea`: **39 occurrences / 11 files / one repo — 0 script defaults, 16 live instructions, 23 dated records** (the predecessor's "40 / 12 files / ~14 B-shaped" was measured at an older head, and its **"about 14" was an estimate: the true live count is 16**). The 16 are placeholdered; **`--mode tree` is clean and this lane's own gate reads `A+B=0`**, so **the Tier-1 promotion and a new CGNAT structural class can now land without turning a gate red on published content.**
- ⚠ **SECOND ALLOWLIST INPUT, so this lane meets known exceptions rather than discoveries:** NA-0685's **23 record-class occurrences across 8 named files** — the append-only journal (6), three dated `evidence/` audits (8), two dated `tests/` testplans (5), the queue archive (2) and traceability (2). They are **ruled LEAVE** under D-1322's property and are printed by that lane's gate in every run. See `docs/governance/evidence/NA-0685_as_built.md` §10.
- ⚠ **A VOCABULARY NOTE THIS LANE MUST NOT RE-DERIVE (D-1324):** the tailnet class now speaks with **one pair of placeholder tokens** across the tree — NA-0685 adopted the tokens the tree already used rather than minting new ones, and **derived the host-A/host-B mapping from the runbook's own usage**. A scanner class matching the literals will therefore never fire on the fixed lines; a lane that must re-add such a line takes the placeholder as part of the edit, per Option B. **Standing rule recorded at D-1324: a lane adopts the vocabulary the tree already uses, and derives its mapping from that usage** — the converse of NA-0684's F2, and the reason "which token?" is a measurement rather than a matter of taste.
- ⚠ **A PROOF GAP THIS LANE SHOULD CLOSE, observed by NA-0685:** running the scan in `--mode diff` over an empty input made it **refuse a vacuous pass** (*"NOTHING EXAMINED — refusing to report a pass over an empty input"*, exit 2). That behaviour is correct and valuable, and **it is currently unguarded** — no test proves the scan refuses an empty input rather than reporting clean. Cheap to add alongside the promotion work.

- ⚠ **CLOSED 2026-07-29 by NA-0686 (D-1325).** Three classes landed, each proven fail-closed
  with a synthetic added line and green again on removal:
  **(i) `host_retired_rig` PROMOTED to Tier-1 tree-wide.** Its pre-existing population is met
  as a **per-path expected-count baseline**, not a per-path exemption: a file may KEEP the
  occurrences it has and may LOSE them, but may not GAIN one. ⚠ A path-only allowlist would
  have let a new occurrence into an already-listed file silently, which is the very hole this
  finding was filed about. **Option B is now a tree invariant rather than a habit.**
  **(ii) NEW `tailnet_cgnat` (100.64/10), Tier-2b** — the class NA-0684 found the gate could
  not see. **(iii) NEW `public_ddns_host`, Tier-2b** — the retired names' provider domain.
- **The census reconciled EXACTLY, and correcting it corrected the record.** Measured at
  `d2bf480e`: **771** rig occurrences (79 files, `qsl-protocol` only) + **25** public-DDNS
  (11 files across three repos) = **796**, byte-identical to NA-0684's recorded final-gate
  total. ⚠ The "796-occurrence record class" in this entry is **two needles summed**, not the
  rig alone — a first pass measuring only the rig read 771 and looked like drift. CGNAT
  measured **23 occurrences / 8 files** and the tracked token-carrying paths measured
  **10**, both exactly as censused. **NA-0685 added zero new occurrences of either token**
  even though the tree grew by 12 files and 3 508 lines between the two measurements.
- ⚠ **Allowlist keys are salted digests of `<repo>:<path>`, and that is not decoration:**
  **ten of the allowlisted paths carry the token IN THE PATH ITSELF**, so a plaintext list
  would have republished in this file exactly what two sanitization lanes removed — and the
  Tier-1 scan would then have hit its own allowlist. The scan **prints the real paths it met
  at run time** (771 occurrences across 79 files, listed), because an exception you cannot
  see is not an exception.
- **The gate file stays BYTE-IDENTICAL across all four repositories** (md5 verified); the
  repo is derived at run time from the origin remote, so the allowlist is repo-aware without
  a per-repo copy.
- **THE VACUOUS-PASS GUARD NOW EXISTS** — `scripts/ci/infra_literal_scan_selftest.py`, 13
  checks, wired into CI in all four repos AHEAD of the scan itself so a broken instrument
  fails before it can report clean. It pins the `NOTHING EXAMINED` / exit-2 refusal in `tree`
  and `diff` mode **and the deliberate asymmetry that `staged` mode does NOT refuse** (a
  deletion-only commit legitimately has no added lines). ⚠ The self-test carries **no
  operator literal**: every needle is assembled at run time from fragments that never appear
  contiguously in its source, so the file cannot fail the gate it tests.
- **Phase 4d:** the remediation help now prints Option B (a)/(b)/(c) and the
  adopt-the-tree's-vocabulary rule at the moment the gate fires — a ruling that lives only in
  `DECISIONS.md` gets re-derived by whoever trips the gate at 2am.
- Resolution: CLOSED 2026-07-29 by NA-0686 (D-1325) — `host_retired_rig` promoted Tier-1 tree-wide with a per-path expected-count BUDGET (not an exemption), two new Tier-2b classes each proven fail-closed, and the vacuous-pass refusal guarded by a 13-check selftest that runs before the scan in all four repos. (This `Resolution:` line was applied 2026-07-29 by **NA-0687 / D-1326** when the convention was adopted; the closure itself is NA-0686's and its annotation above is unchanged.)

### ENG-0090 — the naming ruling's cross-repo remainder: five user-facing "Server" surfaces outside qsl-desktop — **NEW; filed 2026-07-29 by NA-0683 (D-1320's map)**
- Severity: P3 (product vocabulary consistency; no correctness or security impact)
- Status: open — filed 2026-07-29. NA-0683 was **fix-in-qsl-desktop, enumerate-only elsewhere**, by directive.
- Exact surfaces, measured read-only against the bare mirrors: `qsl-protocol` `qsl/qsl-client/qsc/src/cmd/mod.rs:601` (⚠ **CLI help text** — clap derives `--help` from the doc comment, so a user reads it), `apps/qsl-tui/README.md:4`, ~10 prose lines under `docs/public/**` using "Server" for the service; `qsl-server` `README.md:7`; org `.github` `profile/README.md:70`.
- Claim violated: none. This is the D-1320 ruling's remainder, not a defect.
- ⚠ **Boundaries that are NOT in scope and must never be swept:** `"server": "qsl-server"` in the `/v1/server-info` body (**wire field**), the `relay_server_info=` CLI markers, `GET /v1/server-info`, and `qsl-server` the repo/crate/service name. `profile/README.md:33-34`'s "server-side" is **ruled LEAVE** — it is doing security-model work, not naming our pane.
- Minimal fix direction: one small docs rider; the org profile line rides the operator's manual org-README rewrite instead.
- Proof gap: none of these surfaces has a naming guard; `qsl-desktop`'s `relay_naming.rs` is the only one.
- Recommended shape: **docs-evidence-only rider**, likely attached to the CI/tooling lane.
- Cross-reference: D-1320 (the map — cite it, do not re-derive the enumeration); NA-0683 as-built §8.

- ⚠ **CLOSED 2026-07-29 by NA-0686 (D-1325) — as THREE edits, not five.** Applied:
  the `qsc` CLI help doc-comment ("Run a local relay **server** …" → "Run a local relay …"),
  `apps/qsl-tui/README.md:4`, and `qsl-server/README.md:7`. The org `.github` line remains
  with the operator's manual org-README rewrite, as filed.
- ⚠ **The "~10 `docs/public` prose lines" measured TWO, and both are LEAVE.** Of 88 raw
  `server` hits under `docs/public/**`, essentially all are `qsl-server` — the proper noun
  D-1320 itself rules stays. The two survivors (`PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md`
  `:110`, `:139`) use "Server" for the **qsl-server workstream**, paired with "attachment",
  which is the repo sense and not our pane. Operator-confirmed: **the docs/public remainder
  is ZERO.**
- ⚠ **D-1320's map was keyed on LINE NUMBERS and had already drifted** — the CLI help cited
  at `cmd/mod.rs:601` measured at `:641`. The map is annotated in place (mark-don't-rewrite)
  with content needles for all three surfaces, since it is the artifact later lanes were told
  to cite rather than re-derive.
- Resolution: CLOSED 2026-07-29 by NA-0686 (D-1325) — as THREE one-word edits, not the five filed; the `docs/public` remainder measured ZERO, not the estimated ~10. (This `Resolution:` line was applied 2026-07-29 by **NA-0687 / D-1326** when the convention was adopted; the closure itself is NA-0686's and its annotation above is unchanged.)

### ENG-0091 — qsl-server log-capture assertions read the buffer without synchronising on the write, and they now flake ON THE RUNNER THAT DECIDES MERGES — **NEW; filed 2026-07-29 by NA-0686A (D-1325), at operator instruction**
- Severity: **P2** (test-synchronisation correctness; **no runtime, protocol or security impact** — but it blocks merges non-deterministically, and a gate that fails at random teaches reviewers to disbelieve reds, which is the expensive part)
- Status: open — filed 2026-07-29. **Nothing was fixed.** Filed at operator instruction after the second instance blocked an in-flight PR.
- Exact surfaces, both in `qsl-server`, both the *capture-then-assert-immediately* shape:

| # | date | test | site | assertion that failed |
|---|---|---|---|---|
| 1 | 2026-07-26 | `bundle_is_opaque_bytes_in_bytes_out_and_never_logged` | `tests/na0678_invite_slots.rs:562` | (log-capture assertion, run on `main` at `131d63f4`) |
| 2 | 2026-07-29 | `na0347_secret_env_public_ingress_and_log_redaction_boundaries_hold` | `tests/qsl_attachments_integration_contract.rs:362` | `assertion failed: text.contains("channel_id=")` |

- ⚠ **THE DISCRIMINATING EXPERIMENT, AND IT IS THE WHOLE EVIDENCE.** Instance 2 was resolved by **one predicted experiment, not by repetition**: the PR was closed and reopened, producing a fresh `pull_request` run **on a byte-identical head — the same SHA, no new commit**. **Run 1 RED (`30483439679`), run 2 GREEN 4/4 (`30484212181`).** Same tree, same command, different outcome. That is a flake demonstrated rather than assumed, and it is what distinguishes this from a real regression.
- ⚠ **WHY THE ASSERTION IS THE POSITIVE ONE, EVERY TIME.** In instance 2 the *negative* assertions in the same block — that the route token, the auth token and the payload sentinel are **absent** from the log — all passed. Only the assertion that a line is **present** failed. A missing-flush race can only break the positive direction, which is both the signature of the defect and the reason it is not a redaction regression.
- **Remedy sketch (NOT implemented):** replace **capture-then-assert** with either
  **(a) poll-with-deadline** — await the expected line for a bounded interval and fail with a named timeout, or
  **(b) flush-then-read** — synchronise on the subscriber/server having emitted before reading the buffer.
  ⚠ **Red-capable, and the control is the point:** race it deliberately — assert immediately with no synchronisation, or run at high `--test-threads` — and the fixed form must still pass while the unfixed form fails. **A fix whose control cannot be made to fire has not been shown to fix anything.**
- ⚠ **THIS IS ENG-0065's DEFECT, AND IT DISPROVES ENG-0065's REASSURING HALF.** ENG-0065 named this exact pattern (down to the `channel_id=` assertion) in `src/lib.rs::tests::logs_do_not_contain_raw_channel`, and argued it was low-pressure **because** it *"always passes on the 2-vCPU runner that decides merges"*. It has now failed on that runner twice. **The population is at least three tests across three files, not one**, so the fix is a pattern sweep rather than a one-line change.
- **Family: ENG-0065 (same defect, now with runner evidence), ENG-0077, ENG-0078** — *instruments that do not instrument*. This is the family's shape on the synchronisation surface: **an assertion whose truth depends on TIMING rather than on the property it names.** A test that passes because the log happened to flush in time is measuring the scheduler, exactly as a test that scrapes a redacted marker measures redaction policy (ENG-0087).
- ⚠ **DISCIPLINE NOTE, recorded because the handling is as reusable as the finding.** The lane **STOPPED** on the red rather than reruning it; **diagnosed from evidence** (branch differs from `main` in one workflow file, +59/−4; `main` green on the same tree eight minutes earlier; prior flake history in the same suite; only the positive assertion failing); **predicted flake**; and the operator then **resolved it with one experiment that could have falsified the prediction**. ⚠ **Never rerun into silence** — a green obtained by repetition until the noise stops is indistinguishable from a green obtained by fixing something, and it destroys the evidence that a flake existed at all.
- Proof gap: nothing asserts these tests synchronise before reading their capture buffer; nothing runs the suite at a thread count where the race is visible; and **nothing tracks flake recurrence**, so instance 1 was forgotten until instance 2 made someone look.
- Sequencing: independent; a `qsl-server` test-hardening micro-lane, implementation-only (test-only), naturally taken **together with ENG-0065** since they are one defect.
- Cross-reference: ENG-0065; ENG-0077; ENG-0078; ENG-0087 (the same "asserting a proxy rather than the property" shape, one surface over); NA-0686 / D-1325.
- ⚠ **PATTERN FIXED AT ALL TWELVE SITES, BUT THIS ENTRY STAYS OPEN — annotated 2026-07-29 by NA-0687 (D-1326), NO `Resolution:` line, per the partial-closure rule this lane wrote into the ledger header.** The census measured the population at **12 sites, not "at least three"** — every log-capture assertion site in `qsl-server`, reconciled across three independent mechanism-keyed searches — and all 12 now synchronise before asserting, with every absence assertion anchored on a server-emitted positive sentinel. **Instance 2's site (`na0347_secret_env_public_ingress_and_log_redaction_boundaries_hold`, the merge-blocker) is clean in every measurement NA-0687 took after the fix.** ⚠ **Instance 1's site (`bundle_is_opaque_bytes_in_bytes_out_and_never_logged`) STILL FAILS**, at a lower rate, by a **different mechanism** now separately filed as **ENG-0094**: it times out with `buffer 0 bytes, 0 lines` after the full 5 s deadline, which falsifies slow-emit outright — nothing was ever captured — and it reproduces only with sibling tests in the same process (1 of 20 whole-binary runs; 0 of 20 for the test alone).
- ⚠ **THE PART THAT MATTERS FOR THIS ENTRY'S OWN EVIDENCE: THE PRE-FIX INSTRUMENT COULD NOT DISTINGUISH THE TWO MECHANISMS.** Both produced the identical text `assertion failed: ...contains("channel_id=")`, whether the buffer held the wrong lines or **no** lines. **So the two data points recorded in this entry's table may include the ENG-0094 mechanism rather than the missing-flush race, and there is no way to tell retrospectively.** The distinction only became visible because NA-0687's remedy reports the size of the buffer it examined — the fix is what revealed that the diagnosis was incomplete. **This entry's `Status:` line and its table are unchanged; nothing above is rewritten.**
- Sequencing after NA-0687: this entry closes when **ENG-0094** closes. The synchronisation half needs no further work.
- ⚠ **SUPERSEDED 2026-07-30 — ENG-0094 WAS FIXED IN NA-0687 AFTER ALL, so the condition above is met.** The annotation above is left byte-identical: it was correct when written, when the operator had ruled the second mechanism to its own later lane. **That ruling was superseded when the mechanism blocked this lane's own merge** (PR #69's required check went red at a third site), the fix was admitted as a scoped extension, and its design was **ruled from a five-arm experiment rather than chosen**.
- Resolution: CLOSED 2026-07-30 by **NA-0687 (D-1326)**, result class `LOG_CAPTURE_SYNC_SWEEP_PASS`. Both halves are done: the **synchronisation** half at all **12** sites (the population this entry recorded as "at least three"), and the **capture-visibility** half via ENG-0094's fix. Measured after both: M5 **135 passed / 0 failed / exit 0** at the house thread count (exact match to prediction), M6 **5 of 5 exit 0** at full parallelism, and **0 of 20 red** on each of the two binaries that had failed — including instance 1's `na0678_invite_slots` and the site-5 binary whose red blocked the merge. ⚠ **`Status:` above and every prior annotation are unchanged, per the convention this lane adopted; this entry's original two-instance table remains legible and its caveat stands — the pre-fix instrument could not distinguish the two mechanisms, so those data points may include ENG-0094's.**

### ENG-0092 — `qsl-server` CI runs `cargo test -q`, so "the totals match" is not evidence that the binaries it should have run did run — **NEW; filed 2026-07-29 by NA-0687 (D-1326), at operator instruction (rider R-c)**
- Severity: **P3** (assurance legibility; **no runtime, protocol or security impact** — the tests that run are unaffected. The defect is that a MISSING binary is invisible)
- Status: open — filed 2026-07-29. **Nothing was changed**: `.github/workflows/**` is a FORBIDDEN path in NA-0687's directive (§6), so this is a filing only.
- Exact surface: `qsl-server` `.github/workflows/ci.yml`, the `rust` job's `test` step — `run: cargo test -q`.
- Claim violated: that a green `rust` check demonstrates the suite ran. Under `-q`, libtest prints no per-binary `Running …` / `test result:` lines, so a binary that silently ran **zero** tests, or a test target that vanished from the build, produces the same output as one that passed. **A silent skip is a vacuous pass**, and `-q` is what makes the skip silent.
- ⚠ **THIS IS ENG-0075 ONE REPO OVER, WITH THE REMEDY ALREADY WORKED.** ENG-0075 was filed against `qsl-desktop` for this exact invocation and closed by NA-0686 (D-1325) with a two-part fix: drop `-q`, and **pin the test inventory by NAME** so a matching total cannot hide a missing binary. Nothing about that remedy is specific to the desktop repo.
- How it surfaced: NA-0687 measured the `qsl-server` suite locally and needed per-binary counts to derive a point prediction. **The prediction (115) missed the measurement (129) because `src/main.rs` carries 14 CLI tests that the executor's static count had not included — and the per-binary breakdown is what made that diagnosable in one read.** CI, running `-q`, would have shown neither number.
- Proof gap: nothing asserts the set of `qsl-server` test binaries CI executes; nothing would fail if one disappeared.
- Recommended change (**NOT implemented**): `cargo test` without `-q`, plus a name-pinned inventory in the ENG-0075 shape (see `qsl-desktop`'s `test_inventory.sh`, which pins `LC_ALL=C` because test names containing `::` sort differently under other locales, and treats an unsorted `comm` warning as fatal).
- Sequencing: independent; a `qsl-server` CI micro-lane. **Fix home: a later ruled lane** — a workflow edit was outside NA-0687's scope by directive.
- Cross-reference: **ENG-0075** (the same defect, fixed in `qsl-desktop`, remedy reusable); ENG-0077/0078/0091 (the *instruments that do not instrument* family); NA-0687 / D-1326 (OBS-2).

### ENG-0093 — the infra-literal scanner leaves an UNTRACKED, NOT-GITIGNORED `scripts/ci/__pycache__/`, so a lane that runs its own gate and stages with `git add -A` commits Python bytecode — **NEW; filed 2026-07-29 by NA-0687 (D-1326), at operator instruction (rider R-c/OBS-11)**
- Severity: **P3** (repo hygiene / accidental-commit hazard; **no runtime, protocol or security impact**, and the gate's verdict is unaffected)
- Status: open — filed 2026-07-29. **Nothing was changed**: `.gitignore` was not touched (out of NA-0687's scope); the executor deleted the directory to restore a clean tree and recorded the fact.
- Exact surface: running `python3 scripts/ci/infra_literal_scan.py` (or `infra_literal_scan_selftest.py`) from a repo checkout writes `scripts/ci/__pycache__/*.pyc`. Measured in `qsl-server` at NA-0687 Phase 0.3: `git status --porcelain` reported `?? scripts/ci/__pycache__/`, and `git check-ignore -v` confirmed **no ignore rule matches it**.
- Why it matters: the local pre-commit call site (`scripts/hooks/pre-commit`, D-0015) and every executor verifying the gate before a PR run this scanner **inside the tree it measures**. The hazard is the ordinary one — `git add -A` after a local gate run — and the failure is silent: bytecode in a public repository, in a directory named after the CI scripts.
- ⚠ **TREE-WIDE, not one repo.** The scanner file is **byte-identical in all four repositories** and must stay so (D-1325), so the same untracked directory appears wherever the gate is run locally. Any fix should be applied in the same four places, in the same shape.
- Proof gap: nothing asserts that running the repo's own gates leaves the working tree clean.
- Recommended change (**NOT implemented**): add `__pycache__/` (and `*.pyc`) to `.gitignore` in all four repositories — or set `PYTHONDONTWRITEBYTECODE=1` in the hook and the workflow steps, which leaves no artifact to ignore. ⚠ Prefer the `.gitignore` route only after checking `git check-ignore` behaviour on **tracked** files (it skips them — NA-0668's gotcha).
- Sequencing: independent; a four-repo hygiene micro-lane, or a free rider on any lane already touching `.gitignore`.
- Cross-reference: D-0015 / NA-0677 (the gate and its pre-commit call site); D-1325 / ENG-0089 (the scanner's Tier-1 promotion and its selftest); NA-0687 / D-1326 (OBS-11).

### ENG-0094 — a log-capture buffer can stay COMPLETELY EMPTY for the whole deadline, and it only happens with sibling tests in the same process: a second mechanism, distinct from the missing-flush race — **NEW; filed 2026-07-29 by NA-0687 (D-1326)**
- Severity: **P2** (test-instrument correctness; **no runtime, protocol or security impact** — but it makes a redaction assertion unfailable-or-unpassable for reasons unrelated to redaction, and it is the surviving half of the flake that blocks merges)
- Status: open — filed 2026-07-29. **Deliberately NOT fixed**: the remedy requires a test-harness design decision (below) that NA-0687 was not authorised to make; the operator ruled it to its own lane rather than let an unreviewed design land late in a long lane.
- Exact surface: `qsl-server` `tests/na0678_invite_slots.rs` — `bundle_is_opaque_bytes_in_bytes_out_and_never_logged` (ENG-0091's instance 1). One capture site in a **16-test** binary.
- **The measurement, and it is the whole finding.** After NA-0687 synchronised all twelve capture sites, this one still fails — but now it fails *legibly*:
  `LOG_SYNC_TIMEOUT: needle "channel_id=" not observed within 5027ms (buffer 0 bytes, 0 lines)`.
  ⚠ **`0 bytes` after the full 5 s deadline FALSIFIES the slow-emit hypothesis outright.** A lost race yields either the needle arriving inside the deadline or a **populated** buffer missing it; it cannot yield a buffer still empty after 100 reads at 50 ms. **Nothing was ever captured.**
- **ONE DISCRIMINATING EXPERIMENT, prediction written first, both arms confirmed:**

| arm | scope | runs | predicted | measured |
|---|---|---|---|---|
| A | whole binary, full parallelism | 20 | ≥1 red with `0 bytes` | **1 red**, `…within 5022ms (buffer 0 bytes, 0 lines)` |
| B | that test ALONE (`--exact`) | 20 | 0 red | **0 red** (`15 filtered out` confirms the filter matched exactly 1 test — not a silent skip) |

  **The failure REQUIRES sibling tests in the same process** — consistent with process-global state, **inconsistent with a per-emit race**, which would not care what else is in the binary.
- **HYPOTHESIS — LABELLED AS INFERENCE, NOT MEASURED.** `tracing` caches callsite `Interest` **globally per process**, while `set_default` installs a **thread-local** dispatcher. The other 15 tests drive the same relay paths with **no** subscriber on their threads; if one reaches the relay's `push channel_id=` callsite first, `NoSubscriber`'s `Interest::never()` can be cached process-wide, after which the event is skipped **without consulting any dispatcher** — including the real one on the capture test's thread.
- **Supporting code evidence (measured, read-only):** 16 tests in the binary, **15 of which drive relay pushes/redeems** (37 call sites), **exactly one** installs a subscriber, and the relay logs through **one** `info!("push channel_id={} id={} bytes={}", …)` callsite in `src/lib.rs` shared by all of them. In the failing run all 15 siblings passed.
- ⚠ **HONEST LIMIT, and the reason this is filed rather than fixed: the experiment that would CONFIRM the mechanism — install a process-global subscriber and watch the failure vanish — IS the candidate fix.** Running it would have been implementing an unruled remedy, so NA-0687 stopped instead. The behaviour is measured; the mechanism is inference.
- ⚠ **THIS IS WHY ENG-0091 STAYS OPEN, AND IT IS ALSO A LESSON ABOUT EVIDENCE.** The pre-fix instrument printed `assertion failed: ...contains("channel_id=")` for **both** mechanisms, so ENG-0091's two recorded runner instances **may include this one** and there is no way to tell retrospectively. **A fix that improves an error message can reveal that the diagnosis behind it was incomplete** — which is what happened here, and it is the strongest argument available for naming what an instrument examined rather than only whether it liked what it saw.
- Recommended change (**NOT implemented — design required**): make the capture subscriber visible to the emitting task regardless of global interest caching. Candidates, none ruled: a process-global subscriber installed once per test binary with a per-test switchable writer; `tracing_subscriber`'s test-writer support; or routing the emit through a subscriber attached to the spawned task (`WithSubscriber`) rather than the thread. ⚠ Whichever is chosen must keep the twelve sites' assertions byte-identical and must arrive with its own red-capable control — a version of NA-0687's gated writer that fails when the capture is never wired up at all.
- Proof gap: nothing asserts that a capture site's subscriber actually receives the relay's events; a site whose buffer stays empty is indistinguishable, to every assertion in the population, from a site whose relay logged nothing.
- Sequencing: independent of the synchronisation work, which is done. **ENG-0091 closes when this closes.** Natural pairing with ENG-0077/0078 (same family) and with ENG-0092, since both concern what a test instrument can be trusted to have examined.
- Cross-reference: **ENG-0091** (closed once this was fixed); ENG-0065 (closed — its own site was unaffected); ENG-0077, ENG-0078, ENG-0087 (the family); NA-0687 / D-1326 and `STOP_NA0687_002`, `STOP_NA0687_004`, `STOP_NA0687_005`.
- ⚠ **FIXED IN NA-0687 AFTER ALL — 2026-07-30. Everything above is left byte-identical**, including the "deliberately NOT fixed" status line and the hypothesis this experiment went on to falsify. **What changed: a third observation, and then this defect blocked the fixing lane's own merge.** PR #69's required `rust` check went red at census **site 5** with `LOG_SYNC_TIMEOUT … (buffer 83 bytes, 1 lines)` — a **POPULATED** buffer, where this entry's own instance reported `0 bytes`.
- ⚠ **THE MECHANISM RECORDED ABOVE IS WRONG, AND THE EXPERIMENT IS WHAT SHOWED IT.** A scratch reproducer of the exposure pattern (15 sibling tests driving the shared callsite with no subscriber + 1 capture test) failed **16 of 20**. Four candidates, 20 runs each: **`rebuild_interest_cache()` after `set_default` → 19/20 RED**, so it is **not** stale per-callsite `Interest` that a rebuild repairs; **`WithSubscriber` on the emitting future → 20/20 RED**, so it is **not** thread-local dispatcher visibility either (OBS-10's family); **a global default carrying data + thread-local routing → 0/20**; **a permissive global default writing to `io::sink` → 0/20**. ⚠ **BOTH HYPOTHESES WRITTEN DOWN IN ADVANCE WERE FALSIFIED.** The `io::sink` arm is decisive **because it discards everything**: it cannot be doing any capturing, so the only thing it can have changed is **process-global filter state**. That account of the internals remains **INFERENCE**; the five outcomes are the claims.
- **The remedy as shipped (D4):** `install_permissive_global_once()` — a permissive process-global default that discards every event — called once at each capture site, **with not one assertion changed at any of the twelve**. The data-carrying global (D1) measured identically and was **rejected on blast radius**: it would route every event in the binary through one writer and depend on per-thread bookkeeping to keep tests apart, while this one **cannot capture, leak or misroute**, and if it ever stops working the flake returns **loudly** as `LOG_SYNC_TIMEOUT`. Shipped with **`control_d4_the_permissive_global_is_installed_and_permissive`**, a control for **the fix's own failure mode** (RED if no global default is set, or if the global max level would drop the relay's INFO lines) — because without it that regression would reappear only as the original defect returning at random.
- **Also shipped, operator-authorised:** the timeout message now carries a **bounded excerpt** of the buffer's content (240 bytes, newlines flattened, test-data surface only). ⚠ **This entry is the argument for it:** reporting the buffer's SIZE is what separated "nothing captured" from "the wrong thing captured", but only the CONTENT names which line arrived — and its absence is why identifying this mechanism took a five-arm experiment instead of one CI log.
- Resolution: FIXED 2026-07-30 by **NA-0687 (D-1326)** as a ruled scoped extension. Measured after the fix: M5 **29 binaries / 135 passed / 0 failed / exit 0** (exact match to prediction), M6 **5 of 5 exit 0** at full parallelism, and **0 of 20 RED** on each of the two exposed binaries — `hardening_auth_reject_logging` (site 5) and `na0678_invite_slots` (site 10, this entry's own instance). ⚠ The confirmatory arms were **load-bearing, not ceremonial**: the reproducer never produced the populated-buffer presentation, so it modelled the mechanism but not both of its faces.

## Workflow / process items

### ENG-0170 — `packaging/caddy/Caddyfile.example` 404s EVERY request as documented and never proxies to the relay; `caddy validate` passes it — **NEW; filed 2026-08-11 by NA-0710 (D-1347; R225B §2.1)**

- Severity: **P2** (a documented example that silently does nothing; ⚠ **worse than one that fails loudly, because every user debugs their own configuration first**)
- Status: open — filed 2026-08-11. **FILING ONLY; nothing in `qsl-server` was edited.**
- Exact surface: `packaging/caddy/Caddyfile.example` — a bare `handle { respond 404 }` written **after** `reverse_proxy 127.0.0.1:8080`.
- The defect: **caddy sorts by DIRECTIVE ORDER, not file order.** The bare `handle` is ordered first, matches everything, and terminates. `reverse_proxy` is never reached.
- ⚠ **EVIDENCE — static, and reproducible WITHOUT a running server:** `caddy adapt --config <file> --adapter caddyfile` expands the shipped example and NA-0710's pre-fix copy to the **IDENTICAL** route tree — `… encode -> [route match=None -> static_response] -> reverse_proxy`, with `reverse_proxy` unreachable. ⚠ **That identity is what makes this the product's defect rather than the lane's.**
- Measured symptom in production: every request over TLS returned `HTTP/1.1 404 Not Found`, including `/v1/server-info` and `/v1/pull`.
- ⚠ **`caddy validate` returns `Valid configuration` on the broken file.** It proves the config PARSES and ADAPTS; **it says nothing about ROUTING.** The consumer that matters is a request.
- Remedy, **verified in production by NA-0710**: mutually-exclusive handles — `handle @relay_api { reverse_proxy 127.0.0.1:8080 }` followed by `handle { respond 404 }` — which preserves the example's intent and fixes the ordering.
- ⚠ Related, same file, different failure: a site keyed by an **IP** serves **no certificate** to a client that sends no SNI (RFC 6066 forbids an IP literal in SNI). `default_sni` is required. Not a defect in the example, which uses a hostname — but it bites any IP-addressed deployment.
- Cross-reference: `docs/ops/RIG_PROVISION_RUNBOOK.md` §2. Originating/last lane: NA-0710 (D-1347). Last-updated: 2026-08-11.

### ENG-0173 — ⚠⚠ THE INVITE PATH DID NOT COMPLETE FOR THE PARTY WHO CREATED THE INVITE — **RE-CHARACTERIZED then CLOSED 2026-08-11 by NA-0711 (D-1348; R231–R244)**

- Severity: **P1** (ruled the program's top defect at R229). **Status: CLOSED 2026-08-11 — fixed, instrumented RED-first, and proven on the producer it was found on.**
- ⚠ **RE-CHARACTERIZED BEFORE IT WAS FIXED (R232 §2), because the filed symptom was not the defect:** the mechanism is **two flag names for one key component, and a client that hid the mismatch.** `invite accept --self-label` and `handshake poll --as` took the same component of the pending-record key (`handshake.pending.{self_label}.{peer}`, `handshake/mod.rs:1158`) under **different names, both silently defaulting to `self`** — and when they disagreed the client emitted `present=false role=none` (naming nothing) and `decode_failed` **for a frame it had already successfully decoded**.
- ⚠ **The concrete form, and it is the one-line version of the whole defect:** `--as` was documented as *"the canonical single self-identity"*; **`--self-label` had NO help text at all**, no literal pin anywhere in the tree, and a third spelling (`QSC_SELF_LABEL`) survived in a demo script read by zero Rust source. **A flag that decided a key, documented nowhere and pinned nowhere.**
- ⚠ **NOT what the original filing said:** the record's *"the state that would let it decode the reply is never persisted"* was an inference from markers that **the source contradicts** — see **ENG-0176**. The state was there all along; the lookup asked for it under a different name.
- **THE FIX (S3-lite, ruled R235 §3 as amended R237 §2):** one vocabulary — **`--as` on all four invite commands, `--self-label` dropped with no alias** (measured unused in the tree **and** by the only downstream consumer, `qsl-desktop` at `c52fd51b`) · the label is **DERIVED** from the config dir's single self identity when no flag is given · an **explicit inconsistent** label **fails closed BEFORE the relay pull**, at both sites (`handshake/mod.rs` beside `enforce_peer_not_blocked`, and `invite/mod.rs`'s own site) · the marker now names **the whole key and which of three states** it found. ⚠ **NA-0616/ENG-0001's ratified predicate was EXTRACTED, not re-authored** (`identity/mod.rs`, `identity_resolved_self_label`).
- ⚠ **AND IT WAS WRITTEN AGAINST ERROR SUPPRESSION (R238 §5.1):** NA-0616's refusal was **already present on the poll path and downgraded** — `identity_self_kem_keypair` is called at `handshake/mod.rs:2127`/`:2183` and its error discarded, where the init path at `:1417` propagates. A gate in the local idiom (22 `return Ok(())` against one `return Err`) would have been swallowed identically. **This one returns `Err`.**
- **DISCHARGED IN BOTH HALVES, and neither substitutes for the other:**
  - **in-process** — `tests/na0711_invite_label_resolution.rs`, three rows, **each watched RED before its green**: the success row (RED at base: `identity_self_ambiguous existing=station requested=self`), the refusal row (⚠ **RED at base by SUCCEEDING silently, rc 0, with the exact `present=false` + `decode_failed` pair — the defect reproduced in a unit test for the first time**), and the slot row (⚠ **passed at base for the WRONG reason — clap rejected an unknown flag — so it was made to fail by moving the check after the pull, where it reddens because the frame is already consumed**);
  - **on the rig** — the two-party invite walk against the live relay, **identities under non-default labels and no label flag on any command**, 2026-08-11:
    ```
    handshake_pending peer=h-from-g present=true role=responder key=handshake.pending.G.h-from-g state=present
    handshake_recv msg=A2 ok=true
    handshake_complete peer=h-from-g role=responder peer_confirmed=yes
    ```
    ⚠ **NA-0710 got `present=false role=none` and no key, three walks running, across two client revs.**
- **Gates:** full suite **132/132 targets reconciled BY NAME against the shard manifest in both directions, 615 passed, 0 failed** · clippy **65 warnings at base, 65 after — zero new** · fmt drift zero measured against base · `na_0616_self_label_footgun` **4/4**, so ⚠ **the over-broad risk the SR-15 read named did not materialise**.
- ⚠ **WHAT IS NOT CLOSED BY THIS, STATED IN THE ENTRY RATHER THAN DISCOVERED LATER:** in a config dir that legitimately holds **two or more** identities, an explicit **wrong-but-existing** label passes the gate and the lookup can still miss silently — the residual hole of the ruled definition, narrowed to multi-identity dirs but **not closed**. The whole-key marker is the compensating control.
- Cross-reference: **ENG-0174**, **ENG-0175** (partial, not closed), **ENG-0176**, **ENG-0177**, **ENG-0178**; NA-0616/ENG-0001 (the ratified property this reuses); NA-0704's *"asymmetric establishing window"*, ⚠ **reproduced on this walk and now EXPLAINED — it closes on first inbound traffic**. Originating lane: NA-0710 (D-1347). **Closed by: NA-0711 (D-1348).** Last-updated: 2026-08-11.
### ENG-0174 — ⚠ **CORRECTED 2026-08-11 by NA-0711 (D-1348; R233 §2): a handshake frame that fails to decode is RETAINED UNDER A 60-SECOND LEASE, NOT DESTROYED** — filed 2026-08-11 by NA-0710 as *"a handshake frame that fails to decode is DESTROYED, not quarantined"*

- Severity: ⚠ **re-argued down from P1** — the loss is **temporary and self-healing at lease expiry**, not permanent. It remains a defect: the client discards a frame it has already decoded.
- Status: **open (corrected).**
- ⚠ **THE CORRECTION, FROM SOURCE, WITH THE SITES:** `PullMode::Lease` → **`UPDATE messages SET leased_until = now + pull_lease_secs`**; only `PullMode::Legacy` → `DELETE FROM messages` (`qsl-server` `src/store.rs:744-757` at `37ec8207`) · the pull admits rows `WHERE leased_until IS NULL OR leased_until <= ?` (`:722`), so **an expired lease makes the message visible again** · `PULL_LEASE_SECS_DEFAULT = 60` (`:7`), `RETENTION_TTL_SECS_DEFAULT = 604_800` (`:5`), **and the env this deployment pushed carries `PULL_LEASE_SECS=60`, `RETENTION_TTL_SECS=604800`** · the client pulls in **`AckMode::Lease` by default** (`qsc` `lib.rs:937`) and ⚠ **the handshake path never acks** — `relay_inbox_ack` has exactly ONE call site, `transport/mod.rs:1452`, in the receive path.
- ⚠⚠ **AN INDEPENDENT SECOND BASIS, FROM A DIRECTION THIS LANE DID NOT ARRANGE (R242 §2):** `tests/na0689_p3_a2_stranding.rs` — a test that **predates this lane** — carries the arm `a2_collateral_pull_by_the_acking_path_strands_under_legacy_and_survives_under_lease`, **and it passes.** ⚠ **The evidence was already in the tree.** That is worth more than the confirming poll R233 declined to spend a credential on.
- ⚠ **THE LIMIT, WRITTEN INTO THE ENTRY RATHER THAN OMITTED FROM IT (R233 §2):** *"'Destroyed' is refuted from SOURCE and remains unrefuted ON THE WIRE. No poll was run to observe the frame's return. The first lane with an authorized bearer and a reason to look should confirm it."*
- ⚠ **THE ORIGINAL MISREADING, AS ITS OWN ROW (R233 §2.1):** *"the frame is destroyed"* was read from **`msg=none` on an IMMEDIATE second poll — exactly what an unexpired 60-second lease looks like.** ⚠ **An absence standing in for a state. The instrument was fine; the interval was wrong.**
- ⚠ **THE QUARANTINE HALF IS ALSO CORRECTED:** NA-0689's quarantine did not *fail* to catch this frame — it is **unreachable from this path** (`grep -n quarantine` over `qsc/src/handshake/mod.rs` and `qsc/src/invite/mod.rs` returns **zero**). ⚠ **A correct instrument pointed at the wrong path, not a broken one.**
- Cross-reference: **ENG-0173**, **ENG-0177**, NA-0689 (quarantine), NA-0708 F-1. Originating lane: NA-0710 (D-1347). **Corrected by: NA-0711 (D-1348).** Last-updated: 2026-08-11.
### ENG-0175 — `invite accept` is not re-runnable, so the accepter has no retry even in principle — ⚠ **PARTIAL FIX 2026-08-11 by NA-0711 (D-1348; R238 §2); NOT CLOSED**

- Severity: **P2**. Status: **open — partially fixed.**
- ⚠ **THE MECHANISM, NAMED (R234 §4.2):** `perform_handshake_poll_with_tokens` returns **`Ok(())` on every reject arm** — measured, not sampled: **22 `return Ok(())` sites in `handshake/mod.rs:1583-2290` against exactly ONE `return Err`** (`:1601`, a transport failure). ⇒ `invite_accept_at` **cannot distinguish "answered" from "rejected"** and marked the slot `Redeemed` either way (`invite/mod.rs:1112`). **A caller that cannot see a failure cannot retry it** — the no-retry property follows from the return contract, not from policy.
- **WHAT NA-0711 FIXED, AND ONLY THIS:** the label check now lands **pre-pull**, so its `Err` propagates through `invite/mod.rs`'s `?` **before** the slot is marked `Redeemed` ⇒ **a MISLABELLED `invite accept` is now retryable**, proven by the instrument's third row (a mislabelled call refuses before any `handshake_send`; the second, correct call completes).
- ⚠⚠ **WHAT IS NOT FIXED, AND THE ENTRY SAYS SO RATHER THAN CLOSING (R238 §2.1): every OTHER failure mode still burns the slot.** A frame that decodes wrongly, a signature that fails, a session that will not build — each still returns `Ok(())` and each still spends the invite. **A free partial claimed as a full fix is how an entry gets closed with the defect live.**
- Cross-reference: **ENG-0173**, **ENG-0174**, **ENG-0178**. Originating lane: NA-0710 (D-1347). **Partially fixed by: NA-0711 (D-1348).** Last-updated: 2026-08-11.
### ENG-0176 — ⚠ **CORRECTED 2026-08-11 by NA-0711 (D-1348; R232 §3): the accepter's pending handshake record IS persisted — the failure was a LOOKUP under a different `self_label`, not a persistence failure** — filed 2026-08-11 by NA-0710 as *"the accepter holds a pinned contact but NO pending handshake record"*

- Severity: **P1** as filed. Status: **CLOSED as corrected** — the mechanism it proposed was wrong and the correction names what replaced it.
- ⚠ **WHAT REPLACED IT, because a withdrawal that does not say leaves the next reader with the same wrong model (R232 §3):** *"The accepter's pending handshake record IS persisted — `hs_pending_store`, `handshake/mod.rs:2243`, `?`-propagated before the B1 marker — and measured present at 9322 and 9350 bytes with all eight fields (`role=responder`, `pending_session`, `confirm_key`, `transcript_hash`, `peer_sig_pk`, `peer_fp`, `self_label`, `peer`). The failure is a LOOKUP under a different `self_label`."*
- **How it was settled:** an offline probe of the **preserved NA-0710 walk vaults** (read-only, on copies) found the record under the explicit label and **absent under the default** — on **both** walks — with a positive control (`identity.kem_sk.<label>` present) **and a negative control (`identity.kem_sk.self` absent) proving the probe could tell two labels apart.** ⚠ A presence result without that control would have been an assertion.
- ⚠ **A THIRD OUTCOME THE CLIENT COULD NOT SEE, and it mattered:** `hs_pending_clear` writes `""` rather than deleting, and the loader mapped `""` and absent alike to "no record". The probe split **absent / cleared / present** — and the redeemer's record read **cleared**, which is how "the record was consumed" was distinguished from "the record is missing". **The shipped client now prints that state** (ENG-0173's `(b)`).
- ⚠ **HOW THE INFERENCE BECAME A FINDING (R232 §2, Director's own row):** it was labelled *"as far as the markers show it"* at NA-0710 STOP_014 §4.1, repeated with the label at R229, repeated again in the brief — **and read as fact by the third hop.** Standing form: **an inference must be re-labelled at every hop or it launders itself into a finding.**
- Cross-reference: **ENG-0173**, **ENG-0174**. Originating lane: NA-0710 (D-1347). **Corrected by: NA-0711 (D-1348).** Last-updated: 2026-08-11.
### ENG-0177 — ⚠ a SECOND path consumes a frame it cannot use: `invite finish` pulls the self inbox and then errors on anything that is not a wrapped reply — **NEW; filed 2026-08-11 by NA-0711 (D-1348; R234 §2.2)**

- Severity: **P2**. Status: **open — FILED, NOT FIXED** (R242/A4 Δ18: it does not share ENG-0173's remedy, and folding it in silently was refused).
- Measured from source at `731b02a8`: `invite_finish` (`qsc/src/invite/mod.rs:1145`) pulls the **self inbox** (`:1133`) and then requires `decode_envelope_resp` (`:1137`). ⚠ **An A2 — or anything else in that mailbox that is not a wrapped B1 — fails that decode with `?`, so the command errors AFTER the frame has already been pulled and leased.**
- ⚠ **Reachable by a documented command in the documented order**, and **no prior lane named it**. It is a second way to lose a frame to a lease window, distinct from ENG-0174's.
- Cross-reference: **ENG-0173**, **ENG-0174**, **ENG-0175**. Originating lane: NA-0711 (D-1348). Last-updated: 2026-08-11.

### ENG-0178 — a REPLAY and a "no context to decode against" miss are the same branch emitting the same reason, and `(c)` could not separate them — **NEW; filed 2026-08-11 by NA-0711 (D-1348; R237 §3 BLOCKER-4)**

- Severity: **P3**. Status: **open — the reject-vocabulary normalisation lane NA-0708 filed owns it.**
- NA-0711 removed the collapse in `hs_decode_reason_label` (`qsc/src/handshake/mod.rs:202-208`), so the decoder's own reason now prints. ⚠ **Measured, exactly four literals are newly surfaced — `handshake_len`, `handshake_magic`, `handshake_type`, `handshake_version` — all structural frame-shape names carrying no key, peer or secret material**; the ten `REJECT_QSC_HS_*` were already emitted verbatim.
- ⚠ **WHAT IT DOES NOT DO, AND THE DIRECTIVE'S OWN CONSTRAINT WAS WITHDRAWN AS UNACHIEVABLE:** a replay and a no-context miss reach the **same branch** and both now print `reason=handshake_type`. The replay guard (`:2082-2087`) needs an explicit suite context, and **the invite path hardcodes `LegacyCompat`** (`invite/mod.rs:1103`, `:1152`), so it cannot fire. **Separating them means touching the guard — a fourth change, refused here.**
- ⚠ **What DOES distinguish the observed replay is `(b)`'s state field**: a replay follows a completed handshake, so its record reads **`state=cleared`** where a wrong-key miss reads `absent`. ⚠ **Not a general separation** — a replay against a party that never held a pending would read `absent` too. `tests/handshake_mvp.rs` now pins both literals and says so in the file.
- Cross-reference: **ENG-0173**, NA-0708 (the withdrawn taxonomy and its filed successor). Originating lane: NA-0711 (D-1348). Last-updated: 2026-08-11.

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

### ENG-0036 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0036`.

### WF-0018 — Strategic/program/review-facing docs drift behind live truth — **DONE at NA-0629 (D-1253, 2026-07-10); directive D566**
- Severity: P2 (process/assurance; the external-review package understates the project's own evidence, and stale "current posture" is where a claim can silently move)
- Status: **DONE at NA-0629 (D-1253, PR #1539, merge `6809906d`, 2026-07-10)** — directive `QSL-DIR-2026-07-10-566` (D566) executed; the strategic/program/public-review docs were refreshed to live truth and the external-review package now records the ProVerif analysis; claim boundary unchanged; last-updated 2026-07-10
- Problem: the governance spine (NEXT_ACTIONS/DECISIONS/TRACEABILITY/this ledger) is current, but the strategic narrative is 6–18 weeks stale. `STATUS.md` (2026-03-02) still lists NA-0177 READY; `ROADMAP.md` (2026-04-30) and `DOC-PROG-001` (2026-04-03) predate the crypto-core arc; the whole `docs/public/**` review corpus (2026-06-25) predates NA-0619..0628. **Highest-value gap: `docs/public/EXTERNAL_REVIEW_PACKAGE.md` omits the CI-gated ProVerif analysis (NA-0627) — the project's single strongest assurance artifact.**
- Claim-safety: the audit's finding is that closing ENG-0034 moves NO claim (post-compromise language still blocked by the A1–A8 abstractions, ENG-0035, and independent review). The fix STRENGTHENS the evidence base while holding the claim boundary; D566 is built fail-closed around any claim-status/sentence change.
- Fix: execute D566 — the [SAFE] bulk (formal-plan ENG-0034 update, review-package/evidence-map/progress additions, STATUS.md deprecation-to-stub per the QSL_PUBLIC_RELEASE_PLAN.md precedent, superseded-by pointers) plus the two [CLAIM-ADJ] posture edits (ROADMAP, DOC-PROG-001) presented as exact before/after for operator approval. Optional: a lightweight doc-staleness lint so this does not silently recur (WF-0012 theme).
- Recommended directive shape: docs/governance single-PR lane, claim-adjacent, fail-closed on any claim movement. Strong successor candidate — arguably BEFORE commissioning external review, since you do not hand a reviewer a package that omits your formal analysis.

### ENG-0037 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0037`.

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
- ⚠ **RESIDUE GIVEN AN ID 2026-08-10 by NA-0709 (D-1346) — NOT a `Resolution:` line and not a re-closure.** The undischarged verification claim recorded above (*"a claim to be discharged, not as a result"*), and the regression guard that **nothing observes**, are now **ENG-0172**. ⚠ This entry's sole closure signal is its `Status:` line — the field this ledger's own rule at `:41-43` says closure must never be read from. The remediation as titled is not in doubt; the verification that it is complete is.

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
- ⚠ **SEVERITY/BOUND REVISED 2026-08-03 by NA-0691 (D625 §4.4) — the `Severity:` line above is left byte-identical per mark-don't-rewrite; this annotation states the corrected bound.** **"Bounded at ONE message per crash" UNDERSTATES the seam, because the seam is reachable with NO CRASH AT ALL.** The correct frequency is **per STORE FAILURE**, and store failures are ordinary runtime events rather than exceptional ones. Three named triggers, from the design note cited below: **(a)** a full or failing disk; **(b)** any vault error on the timeline write path; and **(c) ENG-0117** — a peer-supplied whitespace `msg_id`, which makes the seam **remotely triggerable, per message, with no local fault at all.**
- **The severity judgment, stated with its reason rather than left to be inferred: this stays P3, and the revision is to the FREQUENCY AND ITS BOUND, not an escalation by reflex.** The per-occurrence consequence is unchanged (one message's plaintext lost, forward-secrecy-irrecoverable either way), the loss is now witnessed and bounded to a single redelivery cycle rather than silent, and no new class of data is exposed — what changed is that the trigger population is larger and includes a remote one, which is a reason to schedule the real fix, not to re-rate the harm. ⚠ **A reader who wants P2 should argue it from trigger (c), which is the only one that is remotely reachable.**
- ⚠ **AND THE CORRECTION THAT MUST TRAVEL WITH THE REVISION: NA-0690 DID NOT MAKE THIS WORSE — IT MADE IT VISIBLE AND BOUNDED.** Before D-1329 a store failure was **silently acked away**. **The loss was always there**; what NA-0690 added was the witness and the bound. Recording the revision without this sentence would read as a regression introduced by the fix, which is the opposite of what was measured.
- **Design note (PROPOSAL — not ruled, not scheduled, not started):** `/srv/qbuild/operator/design-notes/DESIGN_CANDIDATES_ENG0042_commit_before_write_seam.md`, raised by NA-0690; its §1 states this revision and its §2 tabulates the seam.
- ⚠ **NOTHING ELSE ON THIS ENTRY MOVES.** No fix, no lane, no reordering. The *"its own analysis + lane. Do NOT patch it casually inside an unrelated lane"* line above stands, and NA-0691 obeys it — this is a record correction, not a step toward the fix.
- Cross-reference: NA-0644 `tests/NA_0644_ack_client.rs::commit_before_write_seam_acked_loudly_no_poison_loop`; NA-0639/WF-0022 (`write_atomic` itself is crash-exercised and held); ENG-0040 (the lane that filed this); D-1267; **ENG-0117** (trigger (c)); **D-1329 / NA-0690** (the witness and the bound); NA-0691 / D625 §4.4 (this revision).

### ENG-0043 — qsc: flip the receive default to ack-lease once proven in operation — **NEW; filed 2026-07-14 by NA-0644 (D-1267; directive D580) — OWED FOLLOW-UP**
- Severity: P3 (reliability completion; the mechanism shipped OPT-IN at NA-0644 BY DESIGN — D580 explicitly forbade the default flip in-lane)
- Status: open — filed 2026-07-14
- The gap: delivery durability is only ambient once lease is the DEFAULT. NA-0644 shipped `--ack-mode lease` opt-in with the legacy path byte-identical. The flip is a deliberate later lane: flip the default (likely adding a config key for a persistent per-install choice), keep `--ack-mode legacy` as the explicit escape hatch, migrate the NA-0640 e2e consciously (exercise lease as the default while RETAINING an explicit legacy-path guard — never silently repurpose the compat proof), and restate the old-server fallback story for the flipped default.
- ⚠ **PARTIALLY CLOSED — annotated 2026-08-03 by NA-0691 (D625 §4.8), NO `Resolution:` line, per the partial-closure rule in this ledger's header.** The flip this entry owed was performed by **NA-0688 / D-1327 C4** (directive D622, merged `b70d8ccc` in PR #1684), and is measured DONE sub-obligation by sub-obligation. **(1) The default is flipped** — `qsl/qsl-client/qsc/src/lib.rs:937`, `stored_ack_mode().unwrap_or(AckMode::Lease)`, at the single resolution point the doc block at `:904` designates for every production pull; C4 moved **both** default-carrying sites together, the second being a hardcode inside the flag-less pull helper that was *"unreachable by `--ack-mode` and so could not be escaped"* (`transport/mod.rs:2928-2930`). **(2) `--ack-mode legacy` remains the explicit escape hatch** — `Cmd::Receive { ack_mode, … }` (`main.rs:218`), `AckMode::Legacy` live at `lib.rs:944` and `transport/mod.rs:2968`, and the resolution table at `lib.rs:920` states it in as many words: *"an explicit `--ack-mode legacy` is the escape hatch and beats everything."* **(3) The config key for a persistent per-install choice exists** — `ACK_MODE_KEY` (`lib.rs:60`), written by `config set ack-mode` (`lib.rs:406`, `:441`), read from `config.txt` by `stored_ack_mode()` (`lib.rs:940`); D-1267 had deferred this key by name to *"the ENG-0043 flip lane"*, and the flip lane delivered it. **(4) The NA-0640 e2e was migrated CONSCIOUSLY with the legacy guard RETAINED** — `qsl/qsl-client/qsc/tests/NA_0640_full_stack_e2e.rs` threads `ack_mode` deliberately (`:193-194`: *"`None` is not a default left unexamined — it is the assertion that the new lease default carries the full stack"*), and `full_stack_message_round_trip_explicit_legacy_ack_mode` (`:454`) is the retained guard, its header (`:441-450`) recording this entry's own anti-repurposing requirement: after the flip *"no test in this file exercised [the legacy path] any more."*
- ⚠ **WHAT KEEPS THIS ENTRY OPEN — the fifth sub-obligation, which is a CLAIM obligation, not a code one.** This entry also owes *"restate the old-server fallback story for the flipped default."* **Measured 2026-08-03: not done.** Every statement of that story is NA-0644-era, written while legacy was still the default — `transport/mod.rs:1409-1412` and `:3003-3005` (both `94e385e37`, 2026-07-14), `tests/NA_0644_ack_client.rs:18-19`, D-1267. **D-1327 does not mention this entry at all**, and the post-flip block that declares itself the one place *"every production pull resolves its `AckMode`"* (`lib.rs:904-931`) does not mention old relays. ⚠ **The mechanism is intact and nothing is lost** — a pre-durability relay ignores `?ack=lease` and 404s the ack route, which the client reports as `ack_legacy_complete`, no error and no retry. ⚠ **What changed is WHO MEETS IT:** under the opt-in only users who asked for lease could; under the flipped default **every user does by default**, so the ambient delivery durability this entry exists to establish is **silently not ambient against a pre-durability relay**, and no post-flip text says so. **Remaining scope is a restatement, not a fix.** Last-updated 2026-08-03 by NA-0691.
- Cross-reference: ENG-0040 (DONE — the mechanism this flip turns on by default); D580 non-goals (the flip recorded as owed); the NA-0640 e2e (the compat guard that must be consciously migrated); ENG-0042 (the seam stays open regardless of the default); **NA-0688 / D-1327 C4** (the lane that performed the flip); **NA-0691 / D625 §4.8** (this measurement).

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
- **Resolution: RESOLVED at NA-0697 (D-1337, directive D631 as twice amended; desktop D-0024, desktop PR #24 head `da1ef2e`).** The Remedy line's operator decision was taken by CLASSIFICATION (the D-1336 boundary rule read at the ARTIFACT level — the D-1337 interpretive precedent): the FILE is the classified artifact and is vault-lifecycle-coupled through its D-0018 signal role (its existence is the per-profile onboarding-complete signal; survival forges the next profile's S2 — the D631 §5 finding, closed BY CONSTRUCTION). A tokened destroy now removes `settings.json` AND its `settings.json.tmp` staging sibling, mirroring erase (Shape A; the erase-mirror `destroy_vault_impl` factoring per Amendment 2 after STOP 004 proved the inline shape untestable). The two boundaries are consistent and documented; the residue set is pinned BY NAME (`destroy_residue_set_enumerated_by_name`, data_dir listing EQUALITY `["qsc"]`, red at base — Control 1′; suite 104/0/1 across 12 = the 105 pinned names). Field verdicts by name: `autolock_minutes`/`relay_url` survive-as-fields, `self_alias` dies-as-field — all dying WITH the file; this entry's field census predates D609 (the file also carried `relay_url`). Slice 4 inherits the destroy-copy sentence (device preferences — auto-lock, relay address, display name — are reset). Erase's own crash-window `.tmp` residue is FILED as ENG-0119, deliberately not fixed here. See D-1337 and desktop D-0024.

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
- ⚠ **ADDENDUM (2026-08-02, AUDIT-TRIAGE #001) — THE EXTERNAL AUDIT REDISCOVERED THIS ITEM, AND TWO MORE SURFACES JOIN THE CENSUS.** Recorded here rather than as new ids by ruling, so this item keeps ONE coverage statement per its binding note above.
  - **The external security audit's F-01 IS this item.** An external review (audit v2, July 2026, sha256 `c7b87b88…4d2c59fa`) independently reported "no zeroization on Suite-2 session and chain-key structs" as a MEDIUM finding. Re-verified against `bd4f2a3a`: `zeroize` appears in **exactly one** file under the refimpl's `src/` — `crypto/traits.rs` — and in **none** of `suite2/` or `qsp/`. All seven named structs (`Suite2SessionState`, `Suite2SendState`, `Suite2RecvWireState`, `Suite2DhRatchetState`, `MkSkippedEntry`, `Suite2RecvState`, `Suite2BoundaryState`) derive `Clone` and nothing else. **No new id was filed; the finding was already here.**
  - ⚠ **THIS ENTRY IS STILL THE STRONGER OF THE TWO RECORDS, WHICH IS WHY IT ABSORBED THE FINDING RATHER THAN THE REVERSE.** The audit's accompanying verification record offered `snapshot_bytes()` as an *addition* of its own. This entry has named it since 2026-07-21 — **and names a SECOND site that record misses, `qsp/state.rs:171`.** Likewise, the binding note above already required the per-clone-site census that the external remediation plan re-derives from scratch. A reader comparing the two should start here.
  - **NEW SURFACE (a) — the process passphrase slot and all three of its ingress returns** (external F-07, verified at `bd4f2a3a`). `static PROCESS_PASSPHRASE: OnceLock<Mutex<Option<String>>>` (`qsc/src/vault/mod.rs:748`) is correctly zeroized on replace (`set_process_passphrase:1072`, wipe at `:1077`), but it is a plain `String`, and three ingress paths return unowned, unzeroized `String`s into it: `clone_process_passphrase:1065`, `passphrase_from_allowed_env:1098`, `read_passphrase_file:1112`.
  - ⚠ **The external audit's stated MECHANISM for (a) is wrong, and a fix must not be planned against it.** It reports that the `String` "can reallocate on growth" and recommends reserving capacity. **This slot never grows:** it is assigned whole at `:1079` (`*slot = passphrase.map(|value| value.to_string())`) and is never mutated, pushed to, or extended. Reserving capacity would be a no-op against a mechanism that does not occur here. The real exposure is the three ingress returns above.
  - ⚠ **One part of (a) `Zeroizing` CANNOT fix, and it must be recorded rather than papered over.** `passphrase_from_allowed_env` returns `std::env::var(...)`; the variable also persists in the process environment for the whole process lifetime, readable by any same-uid process. That needs a different channel or an explicitly accepted, documented tradeoff — not a wrapper type. The affected key is named at `vault/mod.rs:41`.
  - **NEW SURFACE (b) — `keychain_load_key` leaves two unzeroized heap copies of the vault key** (external N-05, verified at `bd4f2a3a`). `keychain_load_key:961` zeroizes **neither** the `secret: String` returned by `entry.get_password()` **nor** the `bytes: Vec<u8>` returned by `hex_decode`. Both are plain heap copies of the full vault key, dropped uncleared. ⚠ **The asymmetry is the tell:** its sibling `keychain_store_key:941` *does* zeroize its hex buffer (`enc.zeroize()`), so the store path was hardened and the load path was not — the same hardened-path/hot-path divergence this entry's 2026-07-21 addendum records for `secret_get`.
  - **Scope effect:** the census this item requires now spans the refimpl session state, both `snapshot_bytes` sites, the `VaultRuntime`/`VaultPayload` read path, the passphrase slot with its three ingress returns, and `keychain_load_key`. **One coverage statement must cover all of them**, per the binding note; and that statement must still say what a `Zeroize` derive does *not* deliver (spilled registers, stack slots, and the environment-variable residency above).
  - Cross-reference: **ENG-0116** (the keychain mode whose key this is), **ENG-0107** (the envelope that key opens), AUDIT-TRIAGE #001 §4.1/§4.7/§6.3.
- Status: open — filed 2026-07-21 from the crypto and protocol review of 2026-07-20, conducted against the qsl-protocol tree at the NA-0663 merge commit `b2dc23bf`. NOT a lane finding — no NA item produced it; filed directly against the ledger by operator instruction. FILING ONLY; not executed. **Addendum 2026-08-02 (AUDIT-TRIAGE #001): external F-01 folded in as MATCHES; external F-07 and N-05 added to the census scope by ruling. Still FILING ONLY; still not executed.**

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

### ENG-0061 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0061`.

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
- **⚠ SEE ALSO WF-0042 (added 2026-07-23): the DECISION-side half of the same function pair.** `decision_ids()` accepts no suffix at all, so `D-####-A` parses as a duplicate of its own numeric prefix — **a suffix form that is correct in `NEXT_ACTIONS.md` is silently wrong in `DECISIONS.md`.** ⚠ **And this entry is WF-0042's own evidence:** filed 2026-07-22 with an exact surface and a measured count, **still unasserted a day later**, `NA-0216AA` still invisible and nothing reporting it — *a ledger entry is documentation, not an assertion.*
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
- **⚠ PAIR — ONE LANE RETIRES BOTH: WF-0026 + WF-0042.** Same file (`scripts/ci/qsl_evidence_helper.py`), same class: **a parser accepting a form its sibling rejects.** `parse_queue()` takes `NA-\d+[A-Z]?`; `decision_ids()` takes no suffix at all. **ONE DIFF FIXES BOTH.** ⚠ **And both nearly bit in the same week:** a `D-1294-A` decision entry was almost created on 2026-07-23 and was caught only by running the helper, while `NA-0216AA` (`NEXT_ACTIONS.md:9875`) is **invisible to the parser right now.** **This pairing line is the shared handle — recorded deliberately INSTEAD of a taxonomy entry, because the answer to several entries sharing a theme is to fix some so the set shrinks, not to add an artifact describing the set.**
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
- **⚠⚠ CORRECTION OF RECORD, 2026-07-22 by NA-0667 (D-1293) at operator direction — THIS ENTRY'S TITLE AND FRAMING OVERSTATED ITS OWN SEVERITY, AND THE OVERSTATEMENT WAS LOAD-BEARING.** The title says "no diff, no revert, **and no backup**", and the Consequence rests its blocker status on *"a mistake in `qwork.sh` breaks the startup path for every lane in every repo simultaneously, **with no previous version to return to**."* **The backup clause was FALSE WHEN FILED.** Measured live at NA-0667 draft: `/srv/qbuild/tools` is **source #33 in `/usr/local/sbin/qsl-backup`**, run daily by `qsl-backup-daily.timer` with **`DAILY_KEEP=30`**, and the run immediately preceding the filing succeeded — `Jul 22 02:36:07 <build-host> qsl-backup[410135]: qsl-backup complete: /backup/qsl/snapshots/daily/daily-20260722T023405-0500`. **Up to thirty recoverable prior versions of every file in `tools/` existed the whole time.**
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
- **Sequencing: ⚠ PAIRS WITH WF-0041 — DECIDE THEM TOGETHER.** They are **the same question from opposite ends**: this entry is **wiring with no script pointed at it**; WF-0041 is **a script with no wiring**. `/home/victor/.claude/settings.json` having **no `hooks` key at all** is the single fact underneath both. **Wiring this hook is WF-0041's option (c) nearly free.** ⚠ **This cross-reference exists because a successor who reaches THIS entry first is the likely case, and would otherwise wire the hook, feel finished, and leave the mirror-freshness detector still unowned — solving half a problem twice.** Do not action this entry without reading WF-0041.
- **⚠ PAIR — ONE LANE RETIRES BOTH: WF-0036 + WF-0041.** **The same question from opposite ends** — a **hook wired to nothing** (WF-0036) and a **check with no owner** (WF-0041); `/home/victor/.claude/settings.json` having **no `hooks` key at all** is the single fact underneath both. **WF-0041's option (c) — wiring the check as a Claude Code hook — RESOLVES BOTH.** **This pairing line is the shared handle — recorded deliberately INSTEAD of a taxonomy entry, because the answer to several entries sharing a theme is to fix some so the set shrinks, not to add an artifact describing the set.**
- Status: open — filed 2026-07-22 by NA-0667 (D-1293) at operator direction. **REPORT-ONLY — NA-0667 neither wired nor removed it, and did not modify `settings.json`.** Cross-linked to WF-0041 on 2026-07-23 at operator instruction; **still neither wired nor removed.**

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
- **⚠ THE CLASS, IN EIGHT COSTUMES — one finding, not eight.** *An artifact that reads as safe and isn't.* **(Six were recorded at filing; a seventh was described in the unifying-shape note below but never numbered, and an eighth arrived at closeout. Numbered here so the count and the list agree — a heading that says SIX above a list that discusses SEVEN is itself a small instance of the class.)**
  1. **An existence-only check that reads as health and means presence** (the three sites above).
  2. **A workaround that always worked and therefore removed the pressure to fix** (the four sightings, the 46-line floor).
  3. **A documented, correct-to-follow procedure that would have silently regressed the very thing the lane exists to protect** — the backup packet's own *Change Procedure* pointed at a workspace copy **21 diff lines stale**; following it literally would have dropped `/home/victor/work/qsl/codex/ops`, `/home/victor/work/qsl/claude` and **`/home/victor/.claude`** from `daily_sources`, plus four excludes and five manifest sections. **A lane opened to close a backup gap would have opened three larger ones.**
  4. **⚠ A VERIFICATION INSTRUCTION THAT READS AS AUTHORITATIVE AND IS WRONG** — and it is the strongest evidence here precisely because it is **self-implicating** and was caught **by testing rather than by an incident.** The Director's own B0 package shipped **four wrong `EXPECT` lines**, found only because each was executed against a scratch copy of the real script instead of asserted. The worst promised diff output `7a8` / `>   /srv/qbuild/operator` where the truth is `39d38` / `<   /srv/qbuild/operator` — **wrong in both direction and line numbers, at the one step whose entire stated purpose is "anything else ⇒ STOP."** A wrong halt condition does not merely fail; **it trains the operator to distrust halt conditions**, which is this class turned on the lane itself. Also wrong: `wc -l` vs `grep -c` for the staleness count, and a dry-run grep against a stream that contains no file paths at all, because the script runs rsync without `-v`.
  5. **⚠ A FIFTH INSTANCE, IN THE SAME PACKAGE, AFTER THE STANDING METHOD WAS ADOPTED.** Step 7.3 shipped `EXPECT: 582+` for the response count; the operator's real run returned **576**, because the census had counted with `ls dir/*` and the glob **expanded the `director_handoff/` subdirectory**, absorbing its 5 entries plus `ls`'s header and blank line. Ground truth: **576 top-level entries = 575 response files + 1 subdirectory; 580 files recursively.** The subtree size was corrected the same way: **37 MB → 48 MB**, the 37 being a `du` multi-argument deduplication artifact (true total **48,390,098 bytes across 800 files**). **The wrong expectations survived because the standing method was applied only to the commands whose output was a *diff*, and not to the ones whose output was a *count*.** Both were corrected in place with their explanations rather than quietly dropped.
  6. **⚠ A SIXTH, INSIDE NA-0668, BY THE EXECUTOR, WHILE WRITING UP THE OTHER FIVE.** Verifying B0's codex coverage, the executor referenced the checkpoint as `checkpoint-20260723T083238-0500`, **dropping the `-after-operator-source-added` label suffix.** The path did not exist. Every `test -f "$CP/…" && echo COVERED || echo MISSING` printed **MISSING**, and `find … 2>/dev/null | wc -l` printed **0** — a confident, fully-formatted, **entirely false** report that B0's second source line had silently failed, which was nearly recorded as a material finding against the operator's completed work. **`test -f` cannot distinguish "file absent" from "parent directory absent," and `2>/dev/null` erased the one signal that would have exposed it.** Caught only by cross-checking an earlier run that used the full path and returned 807. Re-verified correctly: **10/10 codex files covered, packet byte-identical.**
  7. **⚠ A SEVENTH, IN THE SAME LANE, IN THE OPPOSITE DIRECTION.** Checking whether `docs/governance/evidence/` was gitignored, the executor ran `git check-ignore` against **NA-0667's already-TRACKED as-built**. `check-ignore` **skips tracked paths by design**, so it reported "not ignored" — a **false ALL-CLEAR**, where costume 6 was a false alarm. The new as-built was in fact matched by `.gitignore:65 **/evidence/` and needed `git add -f`; without it **the PR would have shipped with no as-built at all.** Caught only because the staged file list was short enough to eyeball. This is WF-0016's *"forgotten at least 31 recorded times"* hazard, **reproduced by testing the wrong subject.**
  8. **⚠ AN EIGHTH, AT CLOSEOUT, IN THE GOVERNANCE TOOLING ITSELF.** The closeout addendum was first written as a separate decision entry **`D-1294-A`**. `scripts/ci/qsl_evidence_helper.py`'s `decision_ids()` matches `^- \*\*ID:\*\*\s*(D-\d{4})\b`, and **`\b` sits between the `4` and the `-`, so a suffixed id matches its own numeric prefix** — the helper reported `DUPLICATE_COUNT 1 / DUPLICATE D-1294 2`, i.e. **an entry that reads as a new decision and is parsed as a duplicate of the one it extends.** ⚠ **And the two governance files DISAGREE about a convention that looks shared:** `parse_queue()` accepts `NA-\d+[A-Z]?` (hence `NA-0217I`, `NA-0216AA`), while `decision_ids()` accepts **no suffix at all**. A suffix form that is correct in one file is silently wrong in the other. **Fix: fold addenda INSIDE the existing decision entry.** Caught by running the helper rather than reasoning about the id format — **the third consecutive turn in this lane where running the instrument beat reasoning about it.** **Its residue — that the disagreement is documented here but asserted nowhere — is filed as WF-0042.**
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
- **⚠ THE ENTRY'S OWN WORKED EXAMPLE, NAMED SO IT IS NOT ABSTRACT: `qsl-attachments` IS THAT REPO.** No lane seats it, so **§3b never fires for it** and its mirror is refreshed by nothing at all. **It read CURRENT (`dd5a2e6b`) throughout NA-0668, so there is NO PROBLEM TODAY** — and that is precisely the point worth recording: **that reading has a one-merge shelf life, and nothing is scheduled to take the next one.** The repo is not drifting *because* nobody noticed; **it simply has no observer, and the only reason anyone knows its state is that this lane happened to look.** A one-time check of its actual staleness is worth doing whenever this entry is decided. NA-0668 deliberately did not act on it — **report-only.**
- Recommended change (**NOT implemented here** — filed at operator instruction, and D604's scope forbids it): give the detector an owner. Options, in ascending cost, **none chosen**: (a) a systemd timer alongside `qsl-backup-daily`, cheapest and matches the existing pattern but a daily cadence under-samples a per-merge decay; (b) invoke it from `qwork` at lane seat, which samples exactly when it matters but couples a read-only census to the seat path and would have to be non-fatal (D604's non-fatal ruling applies with equal force); (c) wire it as the Claude Code `PreToolUse`/session hook that `settings.json` has no key for — **which would also resolve WF-0036, and the two should probably be decided together**; (d) add it to a read-first checklist, which is the cheapest and is **explicitly the weakest**, since it re-implements the memory dependency this entry exists to remove.
- Proof gap: nothing asserts that any assurance check in `/srv/qbuild/tools/` is actually reachable from an automated trigger. **This entry is one instance; the general form is unmeasured** — `preflight_clean.sh` and `capture_evidence.sh` were not audited for the same property by NA-0668.
- Sequencing: independent of everything NA-0668 landed. **Pairs naturally with WF-0036** (a hook that exists and is wired to nothing) — the two are the same question from opposite ends: one is wiring with no script anymore, the other a script with no wiring.
- **⚠ PAIRS WITH WF-0042 — SAME SHAPE, ONE FILE OVER.** WF-0042 records a known parser disagreement between `parse_queue()` and `decision_ids()` that is **documented but not asserted**, so it surfaces only if someone happens to run the helper. **That is this entry's defect exactly: a correct piece of knowledge whose detection depends on memory.** This entry is about a *check* nobody runs; WF-0042 is about a *known defect* nothing checks for. **Cross-linked in both directions deliberately** — doing it in only one direction is the WF-0036/WF-0041 asymmetry that this ledger already had to correct once.
- **⚠ PAIR — ONE LANE RETIRES BOTH: WF-0041 + WF-0036.** **The same question from opposite ends** — a **hook wired to nothing** (WF-0036) and a **check with no owner** (WF-0041); `/home/victor/.claude/settings.json` having **no `hooks` key at all** is the single fact underneath both. **WF-0041's option (c) — wiring the check as a Claude Code hook — RESOLVES BOTH.** **This pairing line is the shared handle — recorded deliberately INSTEAD of a taxonomy entry, because the answer to several entries sharing a theme is to fix some so the set shrinks, not to add an artifact describing the set.**
- Status: open — filed 2026-07-23 by NA-0668 closeout (D-1294) **at operator instruction, answered and filed, deliberately NOT fixed in-lane.** Cross-linked to WF-0042 on 2026-07-23, likewise at operator instruction.

### WF-0042 — `parse_queue()` and `decision_ids()` disagree about id suffixes, and nothing asserts the disagreement
- Severity: P3 (governance-tooling correctness; **no runtime, protocol or security impact** — but it silently corrupts a duplicate check that closeouts rely on)
- Exact surfaces, both in `scripts/ci/qsl_evidence_helper.py`:
  - `parse_queue()` matches `^### (NA-\d+[A-Z]?) — ([^\n]+)` — **a single optional letter suffix is VALID**, which is why `NA-0217I` parses. (⚠ Two-letter headings such as `NA-0216AA` at `NEXT_ACTIONS.md:9875` do **not** parse and are invisible to the helper — **that is WF-0026, already filed 2026-07-22, and is NOT re-filed here.** It is cited because it is the same failure on the queue side of the same function pair, and because it makes the point below concrete.)
  - `decision_ids()` matches `^###\s+(D-\d{4})\b` and `^- \*\*ID:\*\*\s*(D-\d{4})\b` — **no suffix is accepted at all**, and because `\b` falls between the `4` and a following `-`, **a suffixed id matches its own numeric prefix.**
- Description: **a suffix form that is correct in one governance file is silently wrong in the other.** `D-1294-A` does not fail to parse — it parses as **a second `D-1294`**, so an entry that *reads* as a new decision is *counted* as a duplicate of the one it extends. Observed live at the NA-0668 closeout: `DUPLICATE_COUNT 1 / DUPLICATE D-1294 2`. The addendum was folded inside the existing D-1294 entry instead, and `DUPLICATE_COUNT 0` was restored.
- Consequence: `DUPLICATE_COUNT` is a closeout gate. A suffixed decision id **breaks it in the direction that raises a false alarm** — which is the recoverable direction, and is why this is P3 rather than higher. **The unrecoverable direction is the one that has not been ruled out:** nothing establishes that every *other* consumer of these ids degrades as safely, and the queue/decision asymmetry means a reader who has learned the `NA-0217I` convention will reasonably assume `D-####-A` is equally legal.
- **⚠ THE POINT OF THIS ENTRY, AND WHY IT IS FILED SEPARATELY FROM WF-0038 WHERE THE DEFECT IS ALREADY DESCRIBED: TODAY IT IS DOCUMENTED BUT NOT ASSERTED.** WF-0038's costume 8 explains the disagreement accurately, in prose, in a ledger nobody parses. **Nothing in CI, in the helper, or in any checklist detects a suffixed decision id.** The next person to write `D-####-A` will find out exactly the way this lane did — **by happening to run the helper** — or will not find out at all.
- **⚠ THIS IS WF-0041'S SHAPE ONE FILE OVER, AND THE ENTRY SHOULD BE READ THAT WAY.** WF-0041 is a **check nobody runs**; this is a **known defect nothing checks for.** Both are correct knowledge whose *detection depends on memory*, and both therefore fail in the same silent direction as the class in WF-0038 — *an artifact that reads as safe and isn't.* **Cross-linked to WF-0041 in both directions deliberately: doing it in only one direction is precisely the WF-0036/WF-0041 asymmetry this ledger already had to correct once.**
- Recommended change (**NOT implemented** — filed at operator instruction): make the disagreement **fail loudly rather than be remembered**. Options, none chosen: (a) tighten `decision_ids()` to match `D-\d{4}(-[A-Z])?` and count a suffixed entry as its own id — permits addenda, but changes what "a decision" means and needs a governance ruling first; (b) leave the grammar alone and **assert it** — have `decisions_command` fail on any `D-\d{4}-` occurrence with a message naming this entry, which is the smallest change and closes the memory dependency without deciding anything; (c) reconcile the two patterns explicitly so queue and decision ids share one documented rule, which is the most correct and the most expensive. **(b) is the option that matches this entry's own complaint.**
- Proof gap: nothing asserts that the id grammars accepted by `parse_queue()` and `decision_ids()` are the ones the governance files actually use.
- **⚠ WF-0026 IS THE PROOF THAT "DOCUMENTED BUT NOT ASSERTED" DOES NOT SELF-RESOLVE, AND IT IS THIS ENTRY'S STRONGEST EVIDENCE.** The queue-side half of this same defect was filed on **2026-07-22** with an exact surface, a measured count (718 headings, 711 visible, 7 invisible) and a severity rationale. **A day later, `NA-0216AA` is still invisible to `parse_queue()`, and nothing anywhere reports that.** The filing was correct and changed nothing detectable, because **a ledger entry is documentation, not an assertion.** This entry exists to say that the decision-side half must not be left in the same state — and to record that the executor drafting it **initially wrote up `NA-0216AA` as a fresh observation and had to correct it against WF-0026 before filing**, which is one more instance of checking the record before claiming novelty.
- Sequencing: independent. **Pairs with WF-0041** as above; both are "detection depends on memory" and could be decided in one pass.
- **⚠ PAIR — ONE LANE RETIRES BOTH: WF-0042 + WF-0026.** Same file (`scripts/ci/qsl_evidence_helper.py`), same class: **a parser accepting a form its sibling rejects.** `parse_queue()` takes `NA-\d+[A-Z]?`; `decision_ids()` takes no suffix at all. **ONE DIFF FIXES BOTH.** ⚠ **And both nearly bit in the same week:** a `D-1294-A` decision entry was almost created on 2026-07-23 and was caught only by running the helper, while `NA-0216AA` (`NEXT_ACTIONS.md:9875`) is **invisible to the parser right now.** **This pairing line is the shared handle — recorded deliberately INSTEAD of a taxonomy entry, because the answer to several entries sharing a theme is to fix some so the set shrinks, not to add an artifact describing the set.**
- Status: open — filed 2026-07-23 **at operator instruction**, as a docs-only correction against a `READY=NONE` queue. **Nothing in `scripts/ci/qsl_evidence_helper.py` was changed.**

### ENG-0063 — qsl-server `auth_ok` re-hashes the bearer token on every request, leaving a constant per-deployment timing offset the fix does not need to carry
- Severity: P3 (defence-in-depth refinement; **NO runtime, protocol, or security defect** — the shipped fix is correct and constant-time in the per-guess sense; this only removes a constant offset)
- Exact surfaces: `qsl-server` `src/lib.rs` — `ct_eq_secret` (called from `auth_ok`) computes `Sha256::digest(token)` on the configured secret **per request**; the token enters `AppState` through the `new_with_auth*` constructor chain (`relay_token: Option<String>`).
- Description: NA-0670 (D-1297, C-2) made the comparison constant-time by digesting both sides to 32 bytes and folding. The digest of the *provided* value must be per-request, but the digest of the *configured* token is invariant — re-computing it each request adds `Sha256::digest(token)`, whose cost is proportional to the token's 64-byte block count, to every gated request. Precomputing it once at `AppState` construction and storing the `[u8; 32]` would remove the secret from the per-request path entirely and **erase the constant block-count offset** the current form leaves (D606 §2c residual).
- Consequence: the offset is **constant across requests** (not a per-guess oracle) and **not content-revealing** (it leaks at most the token's block count), so this is a refinement, not a defect. Left unfiled it would simply never be scheduled — the ledger's own most-repeated lesson.
- Recommended change (minimal, not a redesign): store `Sha256::digest(relay_token)` as an `Option<[u8; 32]>` on `AppState` at construction; `ct_eq_secret` then digests only the provided value and folds against the stored digest. Small and cheap for whoever is next in the `new_with_auth*` constructor chain.
- Proof gap: nothing asserts the configured-token digest is computed off the per-request path.
- Sequencing: independent; rides the `new_with_auth*` constructor chain, which is why it was correctly **out** of a one-function lane. Recommended directive shape: implementation-only.
- Status: open — filed 2026-07-23 by NA-0670 (D-1297), per D606 §5. **Not a defect, a refinement; `new_with_auth*` byte-unchanged by NA-0670.**

### ENG-0064 — the two-repo seat leaks the first repo's `CARGO_TARGET_DIR` into the second, so the satellite builds into the spine's target dir and its own registered per-repo cache goes unused
- Severity: P2 (build/CI-cache correctness; **no runtime, protocol, or security impact** — but it defeats the per-repo cache separation registration established, and an observation living only in a relay evaporates)
- Exact surfaces: the two-repo seat path (`qwork <lane> qsl-protocol qsl-server`, the NA-0667 cross-repo convention) and its `CARGO_TARGET_DIR` classification in `env_qbuild.sh` (the same dispatch touched by WF-0035). At the NA-0670 seat, the qsl-server seat came up with `cargo_target_mode=explicit · cargo_target_source=preexisting-env · cargo_target_dir=/srv/qbuild/cache/targets/qsl-PROTOCOL/… · explicit_target_preserved=yes`.
- Description: the `qsl-protocol` seat set `CARGO_TARGET_DIR` moments earlier in the same invocation; the subsequent `qsl-server` seat classified that **inherited** value as "explicit"/`preexisting-env` and **preserved it**, so `qsl-server` was pointed at the SPINE's target dir. `/srv/qbuild/cache/targets/qsl-server/` — which NA-0667 (D-1293) created and `check_repo_registration.sh` verifies — went **unused**. This is the **first real exercise of the two-repo seat path NA-0667 enabled**.
- Consequence: cross-repo cache pollution — the satellite's builds land in the spine's tree, defeating the per-repo cache separation registration was meant to establish; the registered per-repo cache is provisioned and verified but never written. **NA-0670 worked around it explicitly** by setting `CARGO_TARGET_DIR=/srv/qbuild/cache/targets/qsl-server` for every local qsl-server build in the lane and **saying so** rather than working around it silently.
- Recommended change: the second-and-later repo in a multi-repo seat should **derive `CARGO_TARGET_DIR` per-repo from the registration** rather than inherit an ambient value and classify it as "explicit"; equivalently, an inherited value should not be treated as an explicit per-repo choice when a registered per-repo target exists.
- Proof gap: nothing asserts that each repo in a multi-repo seat is pointed at ITS OWN registered target dir; the seat reports the value it chose but not whether that value is the per-repo one.
- Sequencing: independent; a `qbuild` tooling lane (the fix is outside every product repository, the NA-0667/NA-0668 shape). Recommended directive shape: implementation-only, in `/srv/qbuild/tools/`.
- Status: ✅ **CLOSED 2026-07-25 by NA-0677 (D-1309)**, in the same tools commit as ENG-0074 per the operator's FLAG-C4 ruling. **The root cause was one line of blindness:** the guard in `env_qbuild.sh` was `[[ -n "${CARGO_TARGET_DIR:-}" ]]`, which cannot tell an operator-set value from the one qbuild exported moments earlier **in the same invocation** — so every repo after the first inherited the first's target dir and was classified `explicit`/`preexisting-env`. `QBUILD_CARGO_TARGET_SET_BY_QBUILD` now records what qbuild itself exported, so an inherited value is recognised as ours and re-derived per repo. **A genuinely operator-set value still differs from it and is still preserved**, keeping the DOC-OPS-006 §6 behaviour intact for its real case — that was the whole risk in the change and it got its own control. Tools commit `410221d`. **⚠ The baseline was captured BEFORE the fix and it is worse than every previous report: on this lane's FOUR-repo seat, THREE of four repos were wrong**, not one — the defect scales with the seat (`/srv/qbuild/evidence/NA-0677/eng0064_baseline_README.md`, all four env files preserved). **The proof gap is closed too:** `qwork_assert_cargo_target_is_own()` fails closed in shared mode (new reason `cargo-target-not-own-registered`), and `cargo_target_is_own_registered` is now written to the KV proof and both JSON proof writers — so a recurrence is greppable across lanes instead of depending on an executor noticing. Positive control: the assertion FAILS when a repo in shared mode points at another repo's tree and PASSES when it points at its own. Sighted four times before it was fixed (NA-0670 filing, NA-0674, NA-0675, NA-0677) — each time caught only because someone checked by hand.
- ⚠ **REPRODUCED 2026-07-25 by NA-0674** on its two-repo seat: `qwork NA-0674 qsl-desktop qsl-protocol` reported `cargo_target_source=preexisting-env` for the SECOND repo, pointing qsl-protocol at qsl-desktop's target dir. Same workaround (explicit per-build pin). Recorded because it is the same shape ENG-0072 turned out to have — a real, live tooling defect masked entirely by executor discipline, and therefore invisible until someone writes it down. **Director ruling 2026-07-25: a candidate for the SAME tooling touch that implements ENG-0074** (both are `qwork`/seat-materialisation properties that should be asserted in the startup proof rather than remembered).

### ENG-0065 — a qsl-server log-capture test reads its buffer without synchronising on the server's on-response log, so it flakes under core contention — but always passes on the 2-vCPU runner that decides merges, so it generates no pressure to fix
- Severity: P2 (test-synchronisation correctness; **no runtime, protocol, or security impact** — but the class is *a defect that always passes in the environment that decides merges*, the same shape as NA-0664's "a recovery convention that always works removes pressure to fix")
- Exact surfaces: `qsl-server` `src/lib.rs` — `tests::logs_do_not_contain_raw_channel`. It calls `handle.abort()` and then **immediately** reads its capture buffer and asserts `logged.contains("channel_id=")`, without synchronising on the server task having emitted the on-response `channel_id=` log line.
- Description: **the measurement is the argument.** On a 6-core build box: **pristine base `b4f86a3c` (13 tests) 0/20 failures; with NA-0670's required same-length test (14 tests) 8/25 failures, and ALWAYS this same unrelated log-capture test, never an auth test.** Single-threaded: clean. Isolated: clean. `RUST_TEST_THREADS=2` → **0/30**; `RUST_TEST_THREADS=4` → **0/30**. Mechanism (**inference**): `#[tokio::test]` runs a current-thread runtime, cooperatively scheduled, so under high core-count contention the server task lags and its on-response log lands **after** the immediate buffer read; the added 14th server-spawning test raised concurrency past the threshold **on a 6-core box only**. GitHub's standard Linux runner is **2 vCPU** and CI runs `cargo test -q`, so the required `rust` check is reliably green (0/30 at 2 threads) — which is exactly why the defect **generates no pressure to fix**.
- Consequence: the test can produce a false failure on higher-core CI or local dev, and — because it is invisible on the 2-vCPU runner — it will sit latent indefinitely. NA-0670's required test SURFACED it (raised the failure probability), it did **not** cause it: the defect is entirely in the test's own missing synchronisation.
- Recommended change: the test should **wait for the server to have flushed the expected log** before asserting — poll/await for the `channel_id=` line (bounded), or flush the subscriber, rather than reading the buffer immediately after `abort()`.
- Proof gap: nothing awaits the server's on-response log before the buffer assertion; nothing runs the suite at high `--test-threads` where the race is visible.
- Sequencing: independent; a `qsl-server` test-hardening micro-lane. Recommended directive shape: implementation-only (test-only).
- Status: open — filed 2026-07-23 by NA-0670 (D-1297). **NOT fixed — NA-0670 touched only `auth_ok`, the `ct_eq_secret` helper, and the one same-length test.**
- ⚠ **THIS ENTRY'S CENTRAL CLAIM IS DISPROVEN, 2026-07-29 by NA-0686A — annotated, not rewritten.** The headline says the defect *"always passes on the 2-vCPU runner that decides merges, so it generates no pressure to fix"*. **It has now failed on that runner twice**, on two different tests, blocking a merge each time (see **ENG-0091** for both data points and the discriminating experiment). The mechanism analysis here is sound and is what made the diagnosis quick; only the *reassuring half* was wrong.
- ⚠ **AND THE SEVERITY ARGUMENT INVERTS WITH IT.** This was filed as a defect that generates no pressure to fix **because** it never fails where it counts. It now fails where it counts, so the "latent indefinitely" reasoning no longer holds — the pressure exists, and the cost is paid in stopped lanes and diagnostic cycles rather than in a silent latent risk. **A finding filed as low-priority BECAUSE it is invisible must be re-read the moment it becomes visible.**
- Cross-reference: **ENG-0091** (the same missing-synchronisation pattern, measured on the GitHub runner, in two *further* test files — so the population is at least three, not one).
- ⚠ **THIS ENTRY'S MECHANISM ANALYSIS WAS RIGHT, AND ITS PREDICTED FIX WORKS — annotated 2026-07-29 by NA-0687 (D-1326).** The inference recorded here (a current-thread runtime, cooperatively scheduled, whose server task lags so its on-response log lands after the immediate buffer read) is what the census confirmed at all twelve sites, and the poll-with-deadline remedy this entry recommended is what shipped. ⚠ **One thing this entry could not have known:** the site it names, `src/lib.rs::tests::logs_do_not_contain_raw_channel`, carried **no** `yield_now()` nudge, and NA-0687 measured that **every** observed failure — its own M2/M6 reds and both of ENG-0091's runner instances — landed in the six un-nudged sites of twelve. The nudge was never a synchronisation, but it was the difference between a defect that fires and one that had not yet been seen to.
- Resolution: CLOSED 2026-07-29 by **NA-0687 (D-1326)**, result class `LOG_CAPTURE_SYNC_SWEEP_PASS_WITH_SECOND_MECHANISM_FILED`. The named defect at the named site is fixed: `logs_do_not_contain_raw_channel` now awaits the relay's `channel_id=` line before aborting the server task and asserting, via the shared `await_log` helper (5 s deadline, 50 ms poll, named `LOG_SYNC_TIMEOUT` on expiry). **This site failed in no measurement NA-0687 took** — clean in M1 (129 passed), M5 (134 passed, an exact match to prediction), M2's five full-parallelism runs and M6's five. The class was proven red-capable before it was proven fixed: the unfixed shape under a withheld gate goes RED with exit 101 (control A, reverted with the revert proved byte-identical by sha256), and the fixed shape is green under a released gate (control B) and reports a **named** bounded timeout when the line never arrives (controls C/C2). ⚠ **`Status:` above and both prior annotations are unchanged, per the convention this lane adopted.**

### ENG-0066 — qsl-server `TRACEABILITY.md` stopped tracking at NA-0012, so three accepted satellite decisions (D-0011/D-0012/D-0013) have no traceability row
- Severity: P3 (traceability completeness; **no runtime, protocol, or security impact** — but "documented but not asserted" back-fills only get scheduled if written down, the WF-0041/WF-0042 class one repo over)
- Exact surfaces: `qsl-server` `TRACEABILITY.md` — ends at the `NA-0012` rows; `DECISIONS.md` carries `D-0011` (NA-0642, durability), `D-0012` (NA-0652, `server-info`), `D-0013` (NA-0655, community-health) with **no** corresponding TRACEABILITY rows.
- Description: the "keep TRACEABILITY in sync" discipline lapsed across three satellite lanes. NA-0670 **correctly added only its own** `NA-0670`/`D-0014` row (adding it was in scope; back-filling the three missing rows was not).
- Consequence: the satellite's traceability is incomplete for three decisions; anyone reconstructing the qsl-server decision→PR trail from `TRACEABILITY.md` alone will miss durability, `server-info`, and community-health. Filing it makes the back-fill **schedulable rather than remembered**.
- Recommended change: a docs-only `qsl-server` micro-lane back-filling the three rows from their merge SHAs; optionally an assertion that every `Accepted` qsl-server decision has a TRACEABILITY row.
- Proof gap: nothing asserts that every accepted qsl-server decision id appears in `TRACEABILITY.md`.
- Sequencing: independent; a `qsl-server` docs micro-lane. Recommended directive shape: docs-evidence-only.
- Status: open — filed 2026-07-23 by NA-0670 (D-1297). **The three missing rows were NOT back-filled by this lane** (out of scope); only the `NA-0670` row was added.

### ENG-0067 — `classify_ci_scope.sh` takes UNVALIDATED positional paths, so a `--base/--head` invocation silently mis-classifies, and the un-hit direction produces a green that would be TRUSTED for a change whose suites were skipped
- Severity: P2 (CI-scope-classification / **assurance integrity** — the dangerous direction yields a *false-trusted* green, the inverse of a docs-only green that merely proves nothing)
- Exact surface: `scripts/ci/classify_ci_scope.sh` — `declare -a paths=("$@")` (`:40`). The script accepts a **positional path list only**; it has **no flag parsing and no argument validation**. Any token that is not a workflow path and not a docs path drives `runtime_critical=true` (`:68-69`); a token list all of which match `is_docs_path` yields `docs_only=true`.
- Description: invoked as `classify_ci_scope.sh --base origin/main --head HEAD` — **the shape most sibling helpers use** (`post_merge_verify.sh`, `qsl_evidence_helper.py`, and this executor's own first attempt) — the four tokens `--base`, `origin/main`, `--head`, `HEAD` are consumed as **paths**, none match `is_docs_path`, and the script fail-safes to `runtime_critical`. Measured live at the NA-0670 (D-1297) closeout: the flag form returned `docs_only=false | runtime_critical=true` on a 7-file all-docs diff; re-running with the actual file list (`classify_ci_scope.sh $(git diff --name-only origin/main HEAD)`) returned `docs_only=true`. **Caught only by re-running.**
- **⚠ THE DANGEROUS DIRECTION IS THE ONE NOT HIT, and it is why this is filed rather than shrugged off.** The observed failure (flags → `runtime_critical`) is the **safe** over-run. The **inverse** mistake is not caught by anything: a caller that passes a path list which happens to be all-docs-looking — a diff computed against the wrong base that misses the runtime files, a wrapper that filters or globs `*.md`, a hand-typed subset — would classify `docs_only` and **UNDER-RUN CI on a change that needed the full suites.** That green would then be **trusted**. It is the exact inverse of the four docs-only lanes (NA-0664/0666/0667/0668) whose greens proved *nothing*: this one would prove something *false*. The interface offers no way to tell "these are the real changed paths" from "these are garbage," and it never fails loudly.
- **⚠ THIS IS THE WF-0038 CLASS — *an artifact that reads as safe and isn't* — on the CI-classifier surface**, and the WF-0041/WF-0042 shape (a defect nothing checks for): the fail-safe direction hides that the interface performs no validation at all.
- Recommended change (**NOT implemented** — filed at operator instruction, one filing, no fix): make misuse **fail loudly instead of silently classifying** — reject any argument beginning with `-` (unknown flag) with a usage error, and/or require an explicit input mode (positional paths vs `BASE_SHA`/`HEAD_SHA` env, which the `pull_request` branch already reads at `:47-49`) rather than inferring from an unlabelled `"$@"`. Neither changes the classification of a correct call; both convert a silent mis-map into an error.
- Proof gap: nothing asserts that the tokens handed to `classify_ci_scope.sh` are paths at all, nor that the classification of a known changed-file set matches the suites CI then runs.
- Sequencing: independent; a `scripts/ci/` hardening item. Recommended directive shape: implementation-only.
- Status: open — filed 2026-07-23 **at operator instruction** (overruling the "one lane's observation is not a rule" caution, because this is a **measured concrete interface defect**, not a pattern claim), as a docs-only correction against a `READY=NONE` queue. **Nothing in `scripts/ci/classify_ci_scope.sh` was changed.**

### ENG-0068 — the directive-authoring template (§5a's closeout-touchable set) and the queue-promotion template disagree about which files a closeout touches, and nothing reconciles them
- Severity: P3 (governance-process / spec consistency — two operator-authored representations of one fact, silently divergent; the failure mode is a scope decision an executor must re-derive per lane, not data corruption)
- Exact surface: a lane's directive **§5a "files this lane MAY touch"** list vs the promoted **queue block's Scope note + result-class PASS gate** in `NEXT_ACTIONS.md`. Concretely at NA-0671: directive `QSL-DIR-2026-07-23-607` §5a lists {`DECISIONS.md`, `NEXT_ACTIONS.md`, `ROLLING_OPERATIONS_JOURNAL.md`, conditionally `IMPROVEMENT_LEDGER.md`} and **OMITS `TRACEABILITY.md` and the testplan**; the queue block at `NEXT_ACTIONS.md:36248` (Scope) and `:36254` (result-class PASS gate — "journal + evidence + **testplan** present … `TRACEABILITY.md`") **REQUIRES both**.
- Description: the two artifacts are authored by the same authority at different times (the directive at drafting, the queue block at promotion) and are meant to state the same thing — the closeout artifact set — but they drifted. An executor following §5a alone ships a closeout **missing `TRACEABILITY.md` and the testplan** and **fails the result-class PASS gate**; an executor following the queue block ships both. Neither document points at the other, and nothing asserts they agree. At NA-0671 the executor followed the queue block (produced both) and the operator confirmed that was correct.
- **⚠ RESOLUTION RULE, recorded so the next author does not re-derive it:** WHEN THE DIRECTIVE §5a AND THE PROMOTED QUEUE BLOCK DISAGREE ON THE CLOSEOUT ARTIFACT SET, **THE PROMOTED QUEUE BLOCK GOVERNS** — it is what the operator approved at promotion and what the result-class PASS gate actually reads.
- **⚠ THIS IS THE WF-0026 CLASS — two representations of one fact with nothing reconciling them.** WF-0026 is the STATE line vs the `### NA-xxxx` section that must agree, unasserted until it broke `qwork` on 2026-07-21. This is the directive-template vs queue-template closeout set: the same silent-divergence shape, one artifact class over — and, like WF-0026, it surfaces only when someone happens to read both representations at once.
- Recommended change (**NOT implemented** — one filing, no fix): make the two templates share one source, or add the standard closeout files (`TRACEABILITY.md` changelog bullet + the testplan) to the directive-authoring §5a template so a directive never omits what the PASS gate requires; alternatively, have the queue-promotion step lint the directive's §5a set against the standard closeout set and flag omissions at promotion.
- Proof gap: nothing asserts that a directive's §5a closeout set equals the queue-block / result-class closeout set; the disagreement is visible only to an executor who reads both.
- Sequencing: independent; a governance-template hardening item. Recommended directive shape: docs/template-only.
- Status: open — filed 2026-07-23 **at operator instruction**, as a docs-only correction against a `READY=NONE` queue. The operator ruled the executor's handling correct (**the queue block governs**) and directed this filing to record the defect **and** the resolution rule. Nothing was fixed; both templates are unchanged.

### ENG-0069 — the spine `TRACEABILITY.md` Matrix has effectively stopped gaining rows; recent lanes appear only as Changelog bullets, and the split is undocumented
- Severity: P3 (governance-doc maintenance — a "living" document half-abandoned; a future lane told to "update the matrix" mis-scopes)
- Exact surface: `TRACEABILITY.md` — the `## Matrix` table (lines ~12–520) has its newest rows in the **NA-060x series (max NA-0608)**; the `## Changelog` section (line 521+) carries dated per-lane bullets through **NA-0671**. **NA-0609 … NA-0671 exist ONLY as Changelog bullets, with no matrix row** (the per-invariant rows thin out well before NA-0608, so the operator characterises the effective freeze as ~NA-0362).
- Description: the file's own Instructions still say "Update this matrix whenever protocol behavior changes or new invariants are added," but in practice ~60 lanes have added Changelog bullets and **no matrix rows**. The two halves have diverged into a frozen invariant-map (Matrix) and a live per-lane ledger (Changelog) **with nothing saying so**. A future lane told to "update `TRACEABILITY.md`" per the Instructions may try to reconstruct a matrix row and mis-scope — the exact ambiguity NA-0671 resolved by matching current practice (a Changelog bullet, no matrix row).
- **⚠ PAIRS WITH ENG-0066 — THE SAME DOCUMENT UNMAINTAINED IN TWO REPOS, TWO DIFFERENT WAYS.** ENG-0066: qsl-server's `TRACEABILITY.md` stopped at NA-0012 (rows simply absent for D-0011/D-0012/D-0013). Here: the spine's `TRACEABILITY.md` **Matrix** froze while its **Changelog** kept going. Same class — a "living" traceability doc that quietly stopped being maintained — surfacing differently in each repo. Decide them together.
- Recommended change (**NOT implemented** — one filing, no fix): either formally retire / mark the Matrix as historical and declare the Changelog the live surface, or restore matrix maintenance; and update the Instructions to say which, so the live surface is unambiguous.
- Proof gap: nothing asserts the Matrix covers the current lane series; the freeze is invisible until a reader compares the newest matrix row to the queue.
- Sequencing: independent; **pairs with ENG-0066** — decide together, the same doc-maintenance question in two repos.
- Status: open — filed 2026-07-23 **at operator instruction**, as a docs-only correction against a `READY=NONE` queue. NA-0671 matched current practice; this entry records the split so it is retired or documented rather than re-derived per lane. Nothing was changed.

### ENG-0070 — the deployed relay lags main and nothing tracks the gap (OBS-J)
- Severity: P2 (a security fix present in main but ABSENT in a running, network-exposed deployment; NO new defect introduced — but the audit HIGH the fix closed is live in production until the binary is rebuilt)
- Exact surface: the inspiron LAN relay ran qsl-server `b4f86a3c814ca79713d4f3d73fcac65762a50f9c`, which **predates NA-0670's merge `5235c2bf…`** (D-0014 / D-1297, the audit **C-2** constant-time-bearer fix). So the live `auth_ok` still short-circuited on the first differing byte — a remote timing oracle on the shared bearer token, on the one network-exposed component, in exactly the low-jitter regime that makes the oracle practical. Discovered **incidentally** while re-running NA-0672's unrelated pre-flight; nothing in the system tracks deployed-binary-vs-main drift.
- Description: two halves. **(a)** the specific instance — rebuild/restart the deployed relay at or after `5235c2bf` so the constant-time comparison is actually running. (b) the general gap — nothing makes deployment provenance/drift **visible**: no record of which rev a running relay was built from, no check comparing it to main. inspiron had no provenance at all (built on-host, no recorded rev tied to the running binary).
- **Half (a) is DISCHARGED for the NEW rig BY CONSTRUCTION (NA-0672):** the LAN relay host's acceptance relay was built fresh from current main `5235c2bfe518…` (the C-2 fix), with full recorded provenance (binary sha256 `60d703ef…`, byte-exact after copy, one build environment for both binaries). Half (a) for **inspiron specifically** is moot — that host was lost. **Half (b) REMAINS OPEN:** deployment-drift visibility is still unbuilt; the LAN relay host's provenance record is a one-off in an evidence file, not a standing mechanism.
- Recommended change (**NOT implemented** beyond the NA-0672 fresh build): record the built rev alongside every deployed relay (e.g. a `DEPLOYMENT_INFO` with the git rev + binary hash, compared to main) and/or surface the built rev via `/v1/server-info` so a client can detect a stale relay. The operator has separately noted that making local loopback the default test relay (one checkout builds and runs) removes the drift by construction — the structural form of half (b).
- Status: open (half (b)) — filed 2026-07-24 by NA-0672 (D-1301). **DO NOT DEPLOY as part of any qsc lane** — a privileged action on a separate host. Cross-reference D-1297/NA-0670 (the fix), D-1301/NA-0672 (the fresh build discharging half (a) for the new rig).

### ENG-0071 — the relay token trio has LIVE proof but no automated regression test
- Severity: P3 (test-coverage; NO runtime impact — the wrappers are trivial and currently correct — but a live-only proof is unguarded against regression)
- Exact surface: `qsl/qsl-client/qsc/src/transport/mod.rs` — `relay_token_set` (trim + `secret_set`), `relay_token_clear` (`secret_set` empty), `relay_token_show` (`relay_auth_token_from_account_secret().is_some()`), and the CLI verbs `relay token-set/token-clear/token-show`. No file under `qsl/qsl-client/qsc/tests/` exercises them (grep for `token-show`/`token-clear`/`relay_token_show`/`relay_token_clear` = 0).
- Description: NA-0672's socket-free suite tests the classifier, not the trio; `relay_auth_header` supplies the token via `RELAY_TOKEN` (env), not the vault; NA_0671 sets the vault secret via a direct library `secret_set`, bypassing the wrapper. So the trio and the vault-token RESOLUTION path (`relay_auth_token_from_account_secret`) were compile-checked only. NA-0672's live acceptance proved them via a dummy-vault sequence against the real relay (`token-set`→`token-show`=true→vault-resolved probe→`token-clear`→`token-show`=false), but that is a live capture, not a suite test.
- **Why it matters:** live-proof-without-a-test is precisely how this project's other findings started — correct today, unguarded tomorrow. The vault path is the one slice B (the GUI) will actually use.
- Recommended change (**NOT implemented** — one filing): a cheap socket-free unit/integration test — `token-set` a value → `token-show` configured=true → `token-clear` → `token-show` configured=false, over a temp vault — pinning the presence-bool contract and the trim/empty semantics. No relay needed.
- Status: open — filed 2026-07-24 by NA-0672 (D-1301). Cross-reference the coverage finding in `docs/governance/evidence/NA-0672_as_built.md` §2 (the env→vault→file precedence trap that made the gap invisible).

### WF-0043 — a two-PR impl PR scoped to src+test cannot pass the required goal-lint gate; the directive template and the enforced gate disagree
- Severity: P2 (process/scope-safety; NO runtime impact — but a directive followed literally produces an UNMERGEABLE impl PR, discovered only by hitting the gate)
- Exact surfaces: the directive §5/§6 two-PR template (impl PR = `src`+test, with `DECISIONS.md`/`TRACEABILITY.md` deferred to the closeout); `tools/goal_lint.py` rule 3 (a REQUIRED status check per `main` branch protection) which **fails any change touching core `src/` paths whose diff omits `TRACEABILITY.md` AND a `DECISIONS.md` entry**; the NA-0663 impl PR **#1609** (D-1286), which carried both in the IMPL PR for exactly this reason.
- Description: NA-0672's D-1300 impl PR, written per the directive as src+test only, **failed `goal-lint`** on first push (`Core protocol paths changed, but required governance docs were not updated: TRACEABILITY.md, DECISIONS.md`). The directive template says one thing; the enforced gate requires another; nothing reconciles them, and the conflict is invisible until the gate fires. The executor resolved it by following the NA-0663 #1609 precedent — the impl PR carries the D-1300 decision + a TRACEABILITY row, the closeout adds only D-1301 (no duplicate) — and the operator ratified it.
- **⚠ CLASS — this is ENG-0068's shape:** a TEMPLATE disagreeing with BINDING REALITY, discovered by hitting the gate rather than by reading it (ENG-0068: two *templates* disagree about the closeout-touchable set; here: a *template* vs an *enforced CI gate*). **Resolution rule, recorded: when a directive template and an enforced CI gate disagree, THE GATE GOVERNS — it is what actually blocks the merge.**
- Recommended change (**NOT implemented** — one filing): amend the two-PR directive template so the impl PR's touchable set explicitly includes the D-<impl> `DECISIONS.md` entry + the `TRACEABILITY.md` row (matching the goal-lint requirement and the NA-0663 precedent), so no future src-touching impl PR repeats the failed-first-push.
- Status: open — filed 2026-07-24 by NA-0672 (D-1301). Cross-reference **ENG-0068** (the sibling template-vs-reality finding); NA-0663 #1609 (the precedent); the reconciliation is recorded in D-1300 and D-1301.

### ENG-0072 — the qsl-desktop qwork seat does not set the GH007 commit identity, and it RECURS on every desktop seat
- Severity: P2 (governance/identity-safety; NO runtime impact — but a commit pushed with the wrong author violates the standing GH007 noreply identity, WF-0029, and is caught only by an executor who checks the object before committing)
- Exact surface: the lane seat's `.git/config` `user.name`/`user.email` as materialised by `qwork`.
- ⛔ **THE SENTENCE BELOW IS SUPERSEDED BY MEASUREMENT (NA-0674, D-1305). It is MARKED, not rewritten** — the filing's reasoning is part of the record, and the correction is only legible next to what it corrects.
  > ~~the SPINE seat is set to `238594419+Tebbens4832@users.noreply.github.com`, the DESKTOP seat is NOT; it comes up with the machine's global personal address. (Contrast: the spine seats set GH007; the desktop seat does not — that asymmetry IS the finding.)~~
- **CORRECTED (measured 2026-07-25):** there was **no asymmetry**. `new_checkout.sh` was the ONLY checkout creator (`qwork.sh` delegates to it) and it set **no commit identity for ANY repo**; there is no `/etc/gitconfig` and no `includeIf`. Every seat inherited the machine's global personal address. The seats that read GH007 were **exactly the ones an executor had fixed by hand** — measured live on the two seats `qwork NA-0674` had just created, where BOTH were wrong, the spine seat included. The desktop lanes surfaced it only because their executor wrote it down. **The fault was universal and masked by executor discipline**, which made the fix smaller and broader than this filing implied: one place, covering every repo and every future seat.
- Description: on NA-0673 the desktop seat handed back the machine's global personal address on BOTH the GATE-1 (D-0007) and GATE-2 (D-0008) branches; each was re-set to GH007 before committing, verified on the object (author == committer == noreply, trailers empty). It also recurred for the D-0009 Appendix F amendment. The GATE-1 relay first surfaced it; GATE 2 confirmed it recurs.
- **Why it matters:** it is NOT a one-off — it recurs on every desktop seat, and it was caught all three times ONLY because the executor checked. An executor who checks the spine seat (correctly GH007) and reasonably assumes the desktop seat matches gets a commit carrying the personal address, and nothing catches it before the push. That is the exact WF-0029 hazard the standing rule exists to prevent, on a surface where the safety net is "the executor remembered."
- Recommended change (**NOT implemented** — one filing): the desktop lane seat should set the GH007 identity the way the spine seat does (a `qwork`/seat-setup change so `user.email` is the noreply on the desktop seat too). A tooling change, not a lane fix.
- Status: ✅ **CLOSED 2026-07-25 by NA-0674 (D-1305)**, as the lane's PHASE-1 setup step per the operator's ruling (seat tooling, before any product commit). Fixed in **two** touch points, because there are two ways a seat comes to exist: `new_checkout.sh` (creation — covers direct invocations too) and `qwork.sh::qwork_set_seat_identity` (reuse — the seats already on disk, which `new_checkout.sh` never revisits; fixing only creation would have left most existing seats wrong). Idempotent by construction, so the reuse path needs no guard. Tools commit `4235786d`. **Proved with a POSITIVE CONTROL** — the instrument was made to return positive first: both freshly created seats read the personal address BEFORE, and a new checkout plus both re-seated existing checkouts read GH007 AFTER (`/srv/qbuild/evidence/NA-0674/eng0072_positive_control.txt`). `~/.gitconfig` deliberately untouched — that identity serves repositories outside this project. Cross-reference **ENG-0064** (the same qwork two-repo seat path, different property — reproduced AGAIN on this lane's seat, see its entry), **WF-0029** (the identity-on-the-object hazard), and ⚠ **ENG-0074** (the fix is now load-bearing and nothing observes it).

### ENG-0073 — the Server pane's two "Clear" buttons (token vs CA) are identically labelled and easy to confuse
- Severity: P3 (usability; NO correctness impact — the app renders the true outcome for whatever state results — but the confusion wastes the user's time and produces a plausible-looking wrong result)
- Exact surface: `qsl-desktop/ui/index.html` — the Server pane's `#btn-relay-token-clear` (Access-token section) and `#btn-relay-ca-clear` (Certificate-authority disclosure), both labelled just **"Clear"**.
- Description: surfaced BY the NA-0673 live acceptance flight. Doing check 4 (Token required), the operator repeatedly clicked the CA's **Clear** intending the token's — clearing the CA instead. With no CA, the tls-internal cert isn't trusted, so the probe returns **"Certificate not trusted"** (TLS refusal, before the 401 auth check) rather than the intended **"This relay requires an access token"**. It happened TWICE, each time producing a plausible-looking but wrong card for the intended check.
- **Why it matters:** the wrong outcome is not obviously wrong — "Certificate not trusted" is a real, correct state for a cleared CA — so a self-hoster could conclude their server has a cert problem when they merely cleared the CA. The correctness rules (R2b, the two-message 401) held perfectly; the defect is purely that two adjacent controls share a label. A mock/layout check could never have surfaced this — only a human driving the real flow did.
- Recommended change (**NOT implemented** — one filing): label them distinctly, e.g. **"Clear token"** / **"Clear CA"** (matching that "Set token" / "Set CA file" are already distinct), or otherwise disambiguate. A one-line copy change in `index.html`, GUI-lane work.
- Status: **SUPERSEDED — DO NOT EXECUTE** (2026-07-25, operator ruling). The approved Server-pane redesign lane (forthcoming; unified Save, Test-saves-first, three-section layout, `[F.1-COMMIT]` reversal recorded — to be enqueued at the next NA number per the operator's pending hand-off) **removes the two "Clear" buttons entirely**, replacing them with per-field "remove it" prose links. That layout has no two adjacent identically-labelled controls, so the cross-field mis-click this finding addresses cannot occur — the standalone "Clear token"/"Clear CA" relabel below is therefore moot and MUST NOT be actioned separately. Lineage retained: filed 2026-07-24 by NA-0673 (D-1303), surfaced by the live acceptance flight (`docs/governance/evidence/NA-0673_as_built.md §4`); the fix rides the redesign lane, not a discrete relabel. A pure UX refinement; no code-correctness defect ever existed.

### ENG-0074 — the ENG-0072 seat-identity fix is load-bearing and nothing observes it
- Severity: P3 (governance/identity-safety; no runtime impact — but the failure mode is SILENT, and the property it protects is the one WF-0029 exists to guarantee)
- Exact surface: `/srv/qbuild/tools/new_checkout.sh` (creation path) and `/srv/qbuild/tools/qwork.sh::qwork_set_seat_identity` (reuse path), both landed by NA-0674's PHASE-1 setup step (tools commit `4235786d`); and `qwork`'s startup proof files `/srv/qbuild/work/<lane>/.qwork/startup.<repo>.kv` / `.json`, which record seat facts (`head`, `worktree_clean`, `ready_count`) but NOT the commit identity.
- Description: ENG-0072 was closed by setting the GH007 identity in the seat tooling, replacing "the executor remembered" with "the tooling did it." That is the right fix and it works — every NA-0674 commit took its identity from tooling. **But nothing checks that it stayed done.** If a future edit to either script drops or reorders those two `git config` calls, seats silently revert to the machine's global personal address and the safety net returns to an executor noticing before pushing — the exact WF-0029 hazard ENG-0072 was filed about.
- **Why it matters:** the pre-ENG-0072 world was *dangerous but visible* — every executor knew to check. The post-fix world is *safe but invisible*: nobody checks any more, because the tooling handles it. **A silent regression in that state is strictly worse than the original defect, because the discipline that used to catch it has been correctly retired.**
- Recommended change (**NOT implemented** — one filing): add the **effective** `git -C <path> config user.email` as one more line in `qwork`'s existing startup proof. Effective, not local — an inherited-from-global value must be VISIBLE, not masked by an empty local key. That makes the property observable at seat time, which is where it is actionable, and it costs one line. A CI-side assertion is possible but weaker: CI sees only what was already committed, i.e. after the wrong-identity commit exists.
- Status: ✅ **CLOSED 2026-07-25 by NA-0677 (D-1309)** — it rode review Lane C exactly as the Director ruled at filing. **Implemented as an ASSERTION, not a recorded line.** The filing recommended adding the effective identity to the startup proof; the operator's Lane-C instruction said *assert*, and that is what landed: `qwork_assert_seat_identity()` is **fail-closed** on **both** seat paths (creation and reuse) immediately after the identity is set, with the same assertion in `new_checkout.sh`; new failure reason `seat-identity-assert-failed`. **EFFECTIVE, not `--local`**, so an inherited-from-global value is visible rather than masked by an empty local key. **Recorded as well as asserted:** `seat_user_name` and `seat_user_email` now appear in the KV proof and both JSON proof writers, which is what makes the property observable at seat time — the point in the process where it is actionable, unlike a CI-side check that sees only what was already committed. Tools commit `410221d`. **Positive control by the operator-approved OBS-O method** — the assertion called DIRECTLY against a seat whose local identity was deliberately wrong, so **the executor never ran `qwork`** and the control tests the assertion itself rather than the whole path: FAILS on the broken seat, PASSES on a correct one (`/srv/qbuild/evidence/NA-0677/assertions_positive_control.txt`). **The framing this filing was built on now holds in both directions: the pre-fix world was dangerous but visible; the post-fix world was safe but invisible; it is now safe AND observed.** Cross-reference **ENG-0072** (closed by this lane; note its stated premise was measured FALSE — see its entry), **WF-0029** (the identity-on-the-object rule this protects), and **ENG-0064** (the sibling qwork seat defect, reproduced live again on this lane's seat — a candidate for the SAME tooling touch, per the Director).

### ENG-0075 — `cargo test -q` in the desktop CI hides WHICH tests ran, so a deleted test file can stay green at a lower total nobody compares
- Severity: P2 (CI observability; no runtime or security impact — but the class is *an instrument that cannot report what it examined*)
- Exact surface: `qsl-desktop` `.github/workflows/ci.yml:24` — `run: cargo test -q`. Check the other three satellites for the same flag when the lane runs.
- Description: `-q` suppresses per-test names **and** the `Running tests/<file>.rs` lines, so CI reports **how many** tests ran and never **which**. A PR that deletes a test file stays green at a lower total, and nothing compares totals between runs.
- How it surfaced (NA-0680): GATE 1 added `design_polish.rs`; CI reported 79 passed, matching local exactly — but nothing NAMED the new file, so "the totals match" was not evidence the new tests ran. Closed by a hand-run **sensitivity control** (remove the file → 79→73, suites 10→9 → restore), proving CI's 79-across-10 was only reachable with it present. **That control is not repeatable by CI and had to be run by hand, twice.**
- Proposed remedy (one line): drop `-q`. Cost: log volume. Buys per-test names permanently.
- Status: FILED 2026-07-26 by NA-0680 (D-1314). **Operator-ruled: log, do not fix inline** — rides a CI/tooling lane with ENG-0077 and ENG-0078 as ONE family.

- ⚠ **CLOSED 2026-07-29 by NA-0686 (D-1325), as a NARROW authorised fold** — the
  ENG-0075/0077/0078 family otherwise stays out of that lane's closure list.
  Grounds recorded by the operator: **NA-0686's own acceptance reads the desktop suite
  figure, and an acceptance number that can lie by omission is not evidence.**
- `cargo test -q` → `cargo test`, plus `scripts/ci/test_inventory.sh`, which PINS every test
  NAME in `scripts/ci/EXPECTED_TEST_INVENTORY.txt` and fails the build when one disappears.
  ⚠ **Printing is not checking** — enumerating counts into a log nobody diffs is the same
  defect wearing longer output — so the pin is COMPARED, and a removal is reported **by
  name**, not as a number that moved. Growth is allowed (a gate that fires on new tests gets
  switched off within a week); shrinkage and disappearance are what it catches.
- Baseline: **103 tests pinned**, matching the measured suite exactly (102 passed + 1 ignored
  across 11 binaries). Red control: deleting `src-tauri/tests/relay_naming.rs` drops the
  enumeration to 98 and the check fails **naming the five missing tests**; restored
  byte-identical (`cmp`).
- The script refuses a pass when it enumerates zero tests, on the same principle as the
  literal scan's `NOTHING EXAMINED`.
- Resolution: CLOSED 2026-07-29 by NA-0686 (D-1325), as a narrow authorised fold — `qsl-desktop` CI runs `cargo test` instead of `cargo test -q`, with a test inventory pinned by NAME so a silently missing binary cannot hide behind a matching total. (This `Resolution:` line was applied 2026-07-29 by **NA-0687 / D-1326** when the convention was adopted; the closure itself is NA-0686's and its annotation above is unchanged.)

### ENG-0076 — R-7 made the onboarding name mandatory, but resume could bypass the gate: a nameless identity on disk resolved to S2
- Severity: P2 (correctness of an onboarding gate; recoverable — Settings can still set the name — but the gate was silently skipped)
- Exact surfaces: `qsl-desktop` `src-tauri/src/state.rs::resolve_launch_state`; `ui/main.js` (the identity step opens by calling `identity_ensure`).
- Description: the identity record is written when the step **OPENS** (`identity_ensure` → `identity_self_kem_keypair` → `identity_write_public_record`), while the NAME is written only on Continue, to a different file. `resolve_launch_state` gated S2 on identity-EXISTS-ONLY, so a kill between opening and Continue left a nameless keypair, resume resolved S2, and the user landed in main displaying as "You". **A GATE-1 regression: a requirement added at one entry point left a hole at every other entry point to the same state.**
- Status: ✅ **CLOSED 2026-07-26 by NA-0680 (D-0018)**, as an **authorised D595 contract revision** — S2 now means "vault exists AND the identity step FINISHED", signalled by `settings.json` existing. ⚠ The obvious signal (`self_alias` absent) was **withdrawn**: `skip_serializing_if = "String::is_empty"` omits an empty alias, so key-absent also matches "name cleared in Settings" and **every pre-R-7 profile** — including the operator's own live profile, which would have been re-routed through onboarding, exactly what D615's F4 forbids. `settings.json`'s existence is unambiguous because **no write path precedes Continue** (traced, not assumed) and that trace is **pinned** by `design_polish::no_settings_write_precedes_onboarding_continue`. **Verified live on the exact broken scenario.** D595's S1/S2 text revised mark-don't-rewrite.

### ENG-0077 — `slice_a_flows.rs`'s `env_lock()` poisons on panic, so ONE failing test reports as MANY and mis-scopes the work
- Severity: P2 (test-diagnostic correctness; **no runtime impact** — but the class is *an aggregate that cannot tell you which of its members is lying*)
- Exact surface: `qsl-desktop` `src-tauri/tests/slice_a_flows.rs:17-20` — every test takes `env_lock().lock().unwrap()`.
- Description: a panic while holding the mutex **POISONS** it, so every subsequent `.lock().unwrap()` in that file panics too, reporting tests as FAILED that have nothing wrong with them.
- How it surfaced, and what it nearly cost (NA-0680): the D-0018 resolver change was reported as breaking **SEVEN** tests. Run individually, **only TWO actually fail**. **The failure list overstated the blast radius 3.5×**, and the other five were about to be "amended" — five behavioural contracts quietly loosened to accommodate a lock artefact. **The operator's stop condition** ("if any amendment encodes something R-7 did NOT supersede, STOP") is the only reason each was opened individually.
- ⚠ This WILL recur: any panic in any test in that file produces the same misleading list, and the next lane will read it and mis-scope exactly as this one nearly did.
- Proposed remedy (one line, and the codebase already uses it): `env_lock().lock().unwrap_or_else(|p| p.into_inner())` — precisely the pattern `src-tauri/src/lib.rs` uses for `WindowModeState` and `AppliedHeight`. The test file is the outlier.
- Status: FILED 2026-07-26 by NA-0680 (D-1314). **Log, do not fix inline.**

- **REPORTED, not fixed, by NA-0686 (D-1325); closure belongs to a later ruled lane against
  this entry's recorded acceptance.** ⚠ Note the pattern to reuse: **NA-0686's vacuous-pass
  guard is this family's template** — `infra_literal_scan_selftest.py` proves the instrument
  reports what it examined and refuses to pass over an empty input. An `env_lock()` that
  poisons on panic is the same class (a proxy that represents without enforcing), and it
  wants the same treatment: a control that makes the instrument's failure mode observable.

### ENG-0078 — a WARNING is not a REMEDY: `style.css`'s in-file caution was violated by the very lane that read it
- Severity: P3 (maintainability / test-integrity; the failure is loud but its cause is opaque)
- Exact surfaces: `qsl-desktop` `ui/style.css` (the `.verify-code` base rule) ↔ `src-tauri/tests/design_round2.rs::verify_code_single_line`.
- Description: that needle slices the stylesheet from the **FIRST** occurrence of `.verify-code` to the next `}`, so any *earlier* mention — including in a comment — silently moves the slice off the rule it pins. `style.css` documents this hazard, in the right place, in plain terms. **It was violated anyway, by NA-0680, in a session that read the file repeatedly**: a new comment several rules earlier named the selector, and the needle failed on `white-space: nowrap`, a property nothing had touched.
- ⚠ The point is not the mistake; **the countermeasure was a comment.** The warning was correct, well-placed and specific, and it was invisible at the moment of editing — the only moment it had to work. **A caution that depends on being read before an unrelated edit is not a control.**
- ⚠ The same shape recurred a second time in the lane, in JS: `showUnlockScreen("main")` appears three times and a bare `find` in a test returned the wrong occurrence. **"First match is not the one you mean" is the general class.**
- Proposed remedy: slice by RULE, not by substring — search for `".verify-code {"` (with the brace), which is what `design_polish.rs`'s `rule_block` helper already does. That makes the hazard impossible rather than detected. A guard assertion is the cheaper interim.
- Status: FILED 2026-07-26 by NA-0680 (D-1314). **Log, do not fix inline.**

> ⚠ **ENG-0075, ENG-0077 and ENG-0078 are ONE FAMILY and should ride ONE lane: *instruments that do not instrument*.** A pipe's exit status (the standing `| tail` rule), a test total, a failure list and a code comment are all **proxies that represent without enforcing**. An aggregate summarises; a comment warns; **neither checks.** Fix three instruments and leave the principle unnamed, and the next proxy gets trusted the same way. Acceptance should include a control proving each instrument now reports what it examined — the discipline NA-0677 applied to `infra-literal-scan`, whose `clean (tree; N files, M lines examined)` output is exactly why every gate result in NA-0680 was cross-checkable and every test total was not.

- **REPORTED, not fixed, by NA-0686 (D-1325).** Same note as ENG-0077: the family's principle
  — *an aggregate summarises, a comment warns, neither checks* — now has a worked example in
  `scripts/ci/infra_literal_scan_selftest.py` and `scripts/ci/test_inventory.sh`, both of
  which turn a warning into a control. A later lane should close 0077/0078 against that
  shape rather than re-deriving it.

### ENG-0079 — `qsc receive` has no overall timeout: pointed at a relay that never answers it blocks forever, and a test that spawns it inherits the hang — **NEW; filed 2026-07-27 by NA-0681 (messaging epic Slice 2), operator-ruled as a real product bug, at BASE and unrelated to that lane's changes**
- Severity: P2 (availability/UX, no confidentiality or integrity impact; the process is stuck, not wrong)
- Exact surfaces: the `receive` polling path reached from `qsl/qsl-client/qsc/src/transport/mod.rs::receive_execute`. Per-request timeouts exist for the server-info probe (`RELAY_SERVER_INFO_TIMEOUT_SECS`) and the push path inherits reqwest's, but there is no bound on the RECEIVE loop as a whole.
- Description: measured, not inferred. During NA-0681's Phase-0 baseline, `tests/aws_file_medium_boundary_na0192a` sat for **17 minutes at 0.2% CPU and 2 seconds of total CPU time**, `State: S (sleeping)`, `wchan: futex_do_wait`. Its child was `qsc receive --transport relay --relay http://127.0.0.1:<port> --max 1` against a test relay that was not answering. On re-inspection **the child PID had CHANGED**, so the behaviour is a retry loop with no ceiling rather than a single blocking call. The binaries in that run were built at 00:22:28 and the lane's first source edit was 00:25:50, so this is **base behaviour, provably not caused by Slice 2**.
- Why it matters beyond the test: a user who points the client at a relay that stops answering gets a process that never returns and never explains itself. That is precisely the "nothing collapses to a generic failure / always visibly moving or visibly stuck" property the messaging epic's shared invariants require (epic §1, outbox O1/O5), and the receive path does not have it.
- Recommended change: bound the receive loop — an overall deadline, or a bounded retry count with a distinct, named timeout cause in the existing Appendix-F vocabulary rather than a silent stall. DELIBERATELY LEFT OPEN for the future directive: whether the bound is a wall-clock deadline or an attempt count, and how it interacts with the legitimate long-poll case.
- Consequence recorded in-lane: NA-0681's acceptance runs the suite at `RUST_TEST_THREADS=2` (the ENG-0065 house mitigation). Per the operator ruling of 2026-07-27, if that test still hangs it is EXCLUDED with the exclusion and reason recorded, and D616 §5.9's "full green" is satisfied modulo that documented exclusion — because a base-level unbounded-retry test that Slice 2 did not cause must not block Slice 2. **This ENG carries the fix.**
- Status: open — FILING ONLY, not executed in-lane. Cross-reference ENG-0065 (the parallelism-sensitive test class) and ENG-0052 (the push-only full suites and their ceilings).

### ENG-0080 — a PENDING contact's record prints `state=PINNED` **and** `device … state=TRUSTED` in the same output: if Slice 4 keys its "Not verified" badge on the device flag it INVERTS I5 — **NEW; filed 2026-07-27 by NA-0681 as an explicit CARRY-FORWARD so the GUI slice inherits it**
- Severity: P2 as filed (no defect underneath — the risk is entirely in how the next slice RENDERS this), but it would be P1 if it shipped: a contact the user has not verified would be displayed as trusted.
- Exact surfaces: `qsl/qsl-client/qsc/src/contacts/mod.rs::legacy_contact_status_to_device_state` maps `"PINNED" -> "TRUSTED"`; `contacts_device_list` and `contacts_show` both print the resulting device state. Observed live in NA-0681's two-party acceptance: `label=alice state=PINNED … device=00ae56d45c55 state=TRUSTED`.
- Description: the two words mean different things and always have. The CONTACT-level `status` (`pinned` vs `verified`) is what the human verification-code ceremony moves and is the I5 state. The DEVICE-level state is a ROUTING-usability flag — `contact_has_trusted_device` gates whether a send is addressable at all, which is why a pending contact must have a "TRUSTED" device or it could not be messaged. **Nothing is wrong underneath**, and messaging a pending contact is explicitly allowed (operator ruling, epic §1) behind a persistent "Not verified" badge.
- Why it is filed anyway: **Slice 4 renders exactly this record.** DESIGN mockups 12/15 put a "Not verified" badge on the contact list and thread header, and the natural implementation reads whatever the record says about trust. Reading the device flag would show an unverified contact as trusted — the precise inversion of I5, arrived at without any code being wrong.
- Recommended change (for Slice 4's directive, not for this lane): the GUI MUST render the CONTACT-level verified state and MUST NOT read the device state for any trust affordance. Slice 4 should carry a named pin asserting the badge is driven by contact status, plus a needle that fails if the device flag is consulted for trust display. Consider also renaming the device state in a later lane so the collision cannot be made again — deliberately NOT done here, because renaming a stored enum value is a migration and this slice does not need one.
- Status: open — FILING ONLY. Cross-reference epic §1 (messaging a pending contact is allowed with a noticeable badge) and DESIGN I5 (redemption yields a PENDING contact, never a trusted one).

### ENG-0081 — the house's conventions differ from the tools' defaults in at least three places, and nothing tells an executor before they trip over it — **NEW; filed 2026-07-27 by NA-0681, operator-ruled as its own micro-lane (a `CLAUDE.md` edit fires BOTH full suites, so it must not ride a code lane)**
- Severity: P3 (process/velocity; each instance is individually recoverable, and all three were caught in-lane — but each was caught by inspection AFTER the fact, not prevented)
- Exact surface: `CLAUDE.md` — a new short section, proposed name "House defaults that differ from tool defaults", read before acting rather than after.
- Description: three cases surfaced in a single lane (NA-0681), each one where the ORDINARY way to do the thing is wrong in this repo:
  1. **Commit trailers must be EMPTY.** Every lane's Phase 0 verifies `trailers=[]` on the object, but the common agent/tooling default appends a `Co-Authored-By:` line. NA-0681 committed one and caught it only by reading the object afterwards; it would have failed the NEXT lane's Phase-0 check rather than its own.
  2. **`pkill`/`pgrep` by pattern is self-referential here.** The executor's own shell command line contains the search string, so `pkill -f "cargo test"` matches and kills the invoking shell. Twice in NA-0681 a cleanup command appeared to die for no reason. Use `-x` against `comm`, or kill by PID.
  3. **Never run `cargo fmt --all`.** ENG-0050 already records that it rewrites 45 files including FORBIDDEN paths; NA-0681 additionally measured that `cargo fmt --all -- --check` is **RED AT BASE at 146 locations**, so it is not usable as a gate either. The workable form is `rustfmt --check` on the lane's own files.
- Also worth including if the micro-lane wants them: `cargo test` stops at the first failing test BINARY unless `--no-fail-fast` (NA-0681 found its affected-file set one file per run until it switched); and a scratch `CARGO_TARGET_DIR` gives a ~26 s type-check loop with no lock contention against a running suite, which is strictly better than waiting on the shared target dir.
- Recommended change: one short section, three to five bullets, each stating the tool default, the house rule, and the observable failure if you follow the default. No enforcement, no gate — the value is entirely in being read first.
- Sequencing: **NOT a code lane.** A `CLAUDE.md` edit triggers both full suites, so it rides its own micro-lane or is folded into the CI/tooling family (ENG-0075/0077/0078).
- Status: open — FILING ONLY. Cross-reference ENG-0050 (the fmt/clippy trap this generalises) and the GH007 identity ruling (NA-0656) which is where the trailers rule is currently recorded.

### ENG-0082 — the 401/403 collapse at the DIAGNOSTIC-MARKER layer: raw logs cannot tell a token rejection from a ticketless-push refusal — **NEW; filed 2026-07-28 by NA-0682 (D-1317; directive D617)**
- Severity: P3 (diagnostic observability only; NO user-facing correctness impact — the user-visible PAUSE cause IS distinct, because the queue derives it from the HTTP status class)
- Status: open — filed 2026-07-28
- The fact: `relay_push_qsc_error_for_status` (`qsl/qsl-client/qsc/src/transport/mod.rs`) maps `UNAUTHORIZED | FORBIDDEN` to the single marker code `relay_unauthorized`. 401 is a fixable token rejection; 403 on the invite path is a ticketless push to a consumed slot. An operator reading raw markers cannot separate them.
- Why it was NOT fixed in NA-0682 (operator-ruled, STOP 007, Option B): changing the shipped code requires rewriting `tests/NA_0663_relay_tls_trust.rs`'s assertion that *"a 401 must stay `relay_unauthorized`, not a trust error"* — **a guard whose whole purpose is to stop a 401 being misreported**. Rewriting a guard to match new behaviour silently weakens a safety assertion, which is the most dangerous edit class. The tree already establishes the safer pattern: NA-0681 refined this same match CONDITIONALLY and kept the 403 default "byte-identical to before".
- Recommended shape: a diagnostics/CI-tooling lane. Split the marker codes with the guard's INTENT preserved explicitly (a 401 is still not a trust error), and re-point `NA_0663`'s assertion deliberately rather than incidentally.
- Cross-reference: D-1317; D617 census C11; NA-0682 testplan §C.9; the `PushFailClass` type that already carries the distinction internally.

- ⚠ **CLOSED 2026-07-29 by NA-0686 (D-1325). The 401 and 403 now say different words.**
  `relay_push_qsc_error_for_status` splits `FORBIDDEN` out as **`relay_forbidden`**;
  `relay_push_error_class_for_status` splits it as **`access_forbidden`** (each site takes
  its OWN neighbours' vocabulary rather than importing the other's — D-1324's rule applied
  inside one file). Guard: `transport::relay_push_diagnostic_tests::forbidden_is_distinct_from_unauthorized_and_from_every_trust_code`,
  which asserts DISTINCTNESS rather than pinning each value, because the defect was never
  "403 has the wrong word" — it was "403 and 401 are the SAME word". Red control:
  collapsing the split turns that guard red; restored byte-identical (`cmp`).
- ⚠ **THIS ENTRY'S OWN PREMISE WAS DISPROVEN, and is annotated rather than rewritten.**
  It recorded that the fix *"requires rewriting `NA_0663`'s assertion"*, and that Option A
  was rejected because rewriting a guard to match new behaviour weakens a safety assertion.
  **The reasoning was right; the factual premise was wrong.** `NA_0663_relay_tls_trust.rs`
  contains **no 403 case at all** (measured: zero hits for `403|FORBIDDEN|Forbidden`), so
  splitting only the 403 arm left every one of its assertions true and byte-identical.
  **NA_0663 was NOT touched, and passing it untouched (11 passed, exit 0) is the measurement
  that proves the split kept 401 intact.** The lesson generalises: *a filed reason to avoid
  an edit is itself a claim, and it can be measured.*
- Behaviour: UNCHANGED. Both strings feed `emit_relay_push_diagnostic` only; pause cause,
  retry and the C11 classification all derive from `push_fail_class_for_status`, which
  already distinguished `TokenRejected` from `Forbidden` and was not modified.
- ⚠ **Residue, named rather than silently left:** `relay_push_diagnostic_class_for_status`
  still maps both statuses to `bearer_auth_failed`. That is a THIRD collapse site, one layer
  further out, and arguably wrong for a 403 on the invite path (a ticketless push is not a
  bearer failure). The ruling named two sites; this one is reported, not changed.
- ⚠ **SUPERSEDED SAME DAY — the third site was RULED IN mid-lane and is FIXED.** The operator's
  grounds are worth keeping verbatim in effect: *ENG-0082 cannot close with one collapse
  standing; the ledger claim would be false.* A finding that closes while its own defect
  survives one layer out is a false closure, and the bullet above would have been the record
  of it. **403 → `access_refused`** at that site.
- ⚠ **And `bearer_auth_failed` for a 403 was not merely imprecise — it was WRONG.** This
  function's neighbours name WHICH CREDENTIAL failed (`bearer_auth_failed`,
  `route_token_auth_failed`). A 403 is the case where the bearer was **accepted** and the
  request refused anyway. Reporting it as a bearer failure **sends an operator to re-check a
  token that was never the problem** — a diagnostic that actively misdirects is worse than one
  that merely under-informs. `access_refused` keeps the function's `<subject>_<outcome>` shape
  while deliberately NOT saying `auth_failed`, because that was the false statement.
- **Three layers, three vocabularies, each from its own neighbours** (D-1324 applied per
  layer): marker code `relay_forbidden`, error class `access_forbidden`, diagnostic class
  `access_refused`. The guard now asserts distinctness at **all three**, plus that the 403
  diagnostic class does not contain `auth_failed` at all. Red control: collapsing the third
  site turns the guard red naming it; restored byte-identical (`cmp`).
- ⚠ **401 untouched a third time, and the consumers prove it:** both tests asserting
  `diagnostic_class=bearer_auth_failed` (`relay_push_diagnostics.rs`,
  `secret_material_diagnostic_boundary.rs`) are **401-driven**, and both pass unmodified
  (3 passed / 4 passed), as does `NA_0663_relay_tls_trust` (13 passed).
- Resolution: CLOSED 2026-07-29 by NA-0686 (D-1325) — 403 now splits from 401 at all three layers (`relay_forbidden` marker code, `access_forbidden` error class, `access_refused` diagnostic class); `NA_0663_relay_tls_trust` passed untouched (13 passed), which is what proved the 401 intact. (This `Resolution:` line was applied 2026-07-29 by **NA-0687 / D-1326** when the convention was adopted; the closure itself is NA-0686's and its annotation above is unchanged.)

### ENG-0083 — in-flight ratchet state is persisted in TWO places (msgqueue records for messages, `outbox.json` for attachments) — **NEW; filed 2026-07-28 by NA-0682 (D-1317; directive D617)**
- Severity: P3 (duplication/hygiene; no correctness impact today — both paths preserve replay-identical-bytes)
- Status: open — filed 2026-07-28. **Accepted knowingly** as the trade against amending acceptance item A3 downward (operator-ruled, STOP 008 Option 1).
- The fact: NA-0682 moved in-flight ciphertext + next ratchet state INTO each message-queue record, because a single global slot head-of-line-blocks every contact when one message is stuck. The legacy global `outbox.json` remains for the attachment/file-transfer paths.
- ⚠ **What a convergence lane MUST preserve, or it will reintroduce a crypto defect:** (1) **replay-identical-bytes** — `qsp_pack` advances the ratchet, so a failed push must replay the SAME packed bytes; re-packing burns a message key and desyncs the session. (2) **The nonce barrier** — abandoning a PACKED message must COMMIT its ratchet advance before dropping the bytes, fail-closed; a plain delete is nonce reuse (see the `retire_packed` guards). (3) ⚠ **F4's SEPARATE store is the only reason `timeline_store::timeline_written_on_send_commit_only` does not collide with O1** — the O1 row is a msgqueue record while the timeline entry still appears only at commit. **A lane that merges the two stores reintroduces that collision head-on.**
- Cross-reference: D-1317; D617 §2c/F4; NA-0682 testplan §C.3; ENG-0042 (the receive-side seam, still open).

### ENG-0084 — `msg_id` is emitted UNREDACTED at one site, against a redaction discipline that is consistent everywhere else — **NEW; filed 2026-07-28 by NA-0682 (D-1317; directive D617)**
- Severity: P4 (local log hygiene; **the exploitable half is already closed** — NA-0682 replaced the content-derived `msg_id` with a 128-bit CSPRNG value, so the emitted id is no longer a fingerprint of the message body)
- Status: open — filed 2026-07-28
- The fact: `qsl/qsl-client/qsc/src/transport/mod.rs` passes `ctrl.msg_id.as_str()` raw to `emit_message_state_reject` on the ack-reject path. Every other emission site — `lib.rs`, and the two other `emit_message_state_reject` call sites — passes `"<redacted>"`. `attachments/mod.rs` passes a raw `file_id` through the same helper (a different identifier class, same asymmetry).
- Why it matters less than it did: while `msg_id` was `sha512(plaintext)[..8]`, that emission was a **plaintext-confirmation oracle** for anyone holding the log and a candidate message. F1's CSPRNG id removed that. What remains is an inconsistency in the redaction discipline, not a leak of content.
- ⚠ **AMENDED 2026-07-28 (STOP 019 Phase 1), mark-don't-rewrite — the raw pass-through is DEFUSED BY ACCIDENT, not by design.** Measurement: the marker layer redacts by VALUE SHAPE, not by key — `should_redact_value` -> `looks_high_cardinality` (`src/output/mod.rs:292`) redacts any value of `len() >= 24` containing a digit. F1's id is 32 hex chars, so it is redacted on output **despite** being passed raw. The old 16-char `sha512(plaintext)[..8]` id fell UNDER that threshold, which is the mechanical reason the C17 leak printed in the clear for as long as it did.
- ⚠ **So the remaining risk is not the current id — it is the COUPLING.** Any future change that narrows an identifier below 24 chars silently re-opens a raw emission at this site, with no test and no declaration anywhere connecting the two. Fixing the call site (pass `"<redacted>"`, as every sibling does) removes the dependence on a length heuristic. See ENG-0087, which covers the same coupling from the test side.
- Cross-reference: D-1317; D617 census C17; NA-0682 testplan; ENG-0087; OBS-FA.

- ⚠ **CLOSED 2026-07-29 by NA-0686 (D-1325), operator-ruled Option C — and THE FILED FIX
  WOULD HAVE BEEN A NO-OP.** The lane intent proposed field-name-keyed redaction for the
  `msg_id` field. **Measurement: every marker field literally named `msg_id` already carries
  the literal string `"<redacted>"`** — all eight sites (`lib.rs:1069`, `:2527`;
  `transport/mod.rs:757`, `:762`, `:841`, `:1428`, `:2791`, `:3461`). A rule keyed on that
  name would have redacted the sentinel and left this finding's actual site untouched. The
  field that carries real message ids is keyed **`id`**, and re-keying `id` would also have
  re-keyed the attachment and timeline-listing markers — out of scope.
- **What was done instead, with ZERO redactor edits:** `emit_message_state_reject` no longer
  ACCEPTS an identifier. It emits `("id", "<redacted>")` hardcoded inside itself, so the
  marker is byte-identical to what the three already-correct call sites produced, and the
  raw pass-through is impossible by construction rather than defused by a length heuristic.
  **Five call sites updated** (the ruling said four; `attachments/mod.rs:2032`,
  `transport/mod.rs:776`, `:892`, `:3033`, `:3675` — measured).
- Clause (b) applied: `emit_message_state_transition` received the same treatment, its
  precondition MEASURED not assumed — **zero consumers** read that marker's `id` in qsc
  tests, in qsc itself, or in `qsl-desktop`.
- ⚠ **THE RED CONTROL THE RULING SPECIFIED COULD NOT FIRE, AND THAT IS THE FINDING.**
  Re-accepting the id and passing it raw still prints `<redacted>`, because the 32-hex value
  crosses the shape rule — the pass-through was **defused by accident**, exactly as this
  entry says. The control therefore had to vary the thing the coupling depends on: the id's
  WIDTH. With `MSG_ID_LEN` narrowed test-only from 16 bytes to 8: **fix in place → C17 guard
  GREEN; old form restored → C17 guard RED** (`"the reject marker must not print the raw
  message id (C17/F1)"`). Both files restored byte-identical (`cmp`). The pair proves a
  DEPENDENCE was removed, not merely an instance: the guard's greenness used to be a
  function of an unrelated constant's value.
- ⚠ **THE CONTROL WAS SUBSTITUTED, WITH REASON — recorded because the substitution is itself
  the evidence.** The ruling specified: *re-accept the id, pass a raw value, the C17 guard goes
  red*. **That control cannot fire**, and the reason is this finding's own content: a raw
  32-hex id is redacted anyway by the shape rule, so the old code was defused by WIDTH, not by
  correctness. A control that cannot fire proves nothing, and passing it off as green would
  have been the exact failure class this lane exists to remove. The substituted control varies
  the property the coupling actually depends on — the id's width — and is **strictly stronger**:
  it demonstrates the DEPENDENCE is gone, not that one instance was patched. Operator-ratified.
- ⚠ **`emit_message_state_transition`'s NAME IS MISLEADING, and the next lane must not be
  surprised by it.** Despite "message_state", it serves the **attachment path** too:
  `timeline_append_entry_for_target` and `timeline_transition_entry_state` are both on the file
  transfer route. So clause (b) **did change the attachment diagnostics surface** — an
  attachment's timeline id now emits as the sentinel. Operator-ruled ACCEPTED, no revert:
  strictly-more-redaction is the house direction (name the field, never the value) and the
  consumer count was measured at zero. ⚠ **The `attachments/mod.rs` `file_id` sites remain
  UNTOUCHED and out of scope.** The future attachments-diagnostics lane meets this as a known
  fact rather than a discovery.
- ⚠ **Remaining population, for a later ruled lane:** the attachment `file_id` sites
  (~14 `emit_marker("id", …)` in `attachments/mod.rs`) and the `timeline_item` entry id both
  still reach the marker layer as `id` under the shape-keyed redactor. Out of scope by
  ruling; the coupling is named here so it is inherited rather than rediscovered.
- Resolution: CLOSED 2026-07-29 by NA-0686 (D-1325), operator-ruled Option C — `emit_message_state_reject` no longer accepts an identifier at all, so the raw pass-through is impossible by construction; ZERO redactor edits. The filed fix was measured to be a provable no-op first. (This `Resolution:` line was applied 2026-07-29 by **NA-0687 / D-1326** when the convention was adopted; the closure itself is NA-0686's and its annotation above is unchanged.)
- ⚠ **RESIDUE GIVEN AN ID 2026-08-10 by NA-0709 (D-1346) — NOT a `Resolution:` line and not a re-closure.** This entry's *"Remaining population, for a later ruled lane"* (the attachment `file_id` sites) existed **nowhere else in this ledger** and reproduces in source at `b845e678`. It is now **ENG-0171**. The closure of this entry's *titled* defect is unchanged.

### ENG-0085 — suspected hollow proof: `receipts_delivered::delivered_receipt_roundtrip` observes an emitted MARKER, never the stored state — **NEW; filed 2026-07-28 by NA-0682 (D-1317; directive D617); running "suspected hollow proofs" item per LANE_INTENT §3b OPPORTUNISTIC**
- Severity: P3 (assurance; the test is NOT hollow today — the marker it asserts is emitted only inside the `Confirmed` arm, which is reached only after the timeline transition persists)
- Status: open — filed 2026-07-28. **Logged, deliberately NOT fixed in-lane** per §3b.
- The fact: the test asserts `event=receipt_recv` / `event=delivered_to_peer` and never reads the timeline back to confirm the row is actually `DELIVERED`. It is **marker-coupled**: a change that kept emitting the marker while failing to persist would not be caught by it.
- Recommended shape: the post-epic hollow-proof audit inherits this. The fix is one read-back assertion, but it belongs with the audit rather than smuggled into a product lane.
- Cross-reference: D-1317; LANE_INTENT §3b; NA-0682 testplan §A.4.

- ⚠ **DISPOSITIONED 2026-07-29 by NA-0686 (D-1325): FIXED, not deferred.** The filing routed
  this to the post-epic hollow-proof audit on the grounds that it should not be "smuggled
  into a product lane" — but NA-0686 IS the test-instrument lane, so the one read-back
  assertion belongs here rather than being passed on again.
- `delivered_receipt_roundtrip` now reads the timeline back and asserts the row is
  `DELIVERED`, in addition to every marker assertion it already made. Nothing was weakened
  or replaced; an INFERENCE was removed.
- The entry's own judgement is confirmed and worth keeping: the test was **not hollow** —
  `event=delivered_to_peer` is emitted only inside the `Confirmed` arm. But it held *through
  an implementation detail*, and "the proof holds because of where the emit happens to sit"
  is the shape of a proof that stops holding without anyone noticing.
- Control: flipping the expected state to `SENT` fails and prints the real stored row
  (`state=DELIVERED`), proving the assertion reads actual state rather than passing
  vacuously. Restored byte-identical (`cmp`).
- Resolution: CLOSED 2026-07-29 by NA-0686 (D-1325), dispositioned FIXED rather than deferred — `delivered_receipt_roundtrip` reads the timeline back instead of inferring state from an emitted marker, so the proof is no longer hollow. (This `Resolution:` line was applied 2026-07-29 by **NA-0687 / D-1326** when the convention was adopted; the closure itself is NA-0686's and its annotation above is unchanged.)

### ENG-0086 — turn delivery acks ON by default (the F6 flip NA-0682 deferred) — **NEW; filed 2026-07-28 by NA-0682 (D-1317; directive D617, operator-ruled Option D)**
- Severity: P2 (product capability + protocol cadence; the MECHANISM ships in NA-0682, only its DEFAULT waits)
- Status: **RESOLVED at NA-0688 (D-1327, directive D622 C3, 2026-07-30)** — both halves default ON, disposition re-derived from measurement (see the NA-0688 block below). Filed 2026-07-28 and deliberately deferred, not dropped: F6 ruled acks ON and recorded itself as "REVISITABLE under real testing — a config default, not a structural choice"; full-suite testing was that clause firing, and this lane is where it concluded.
- ⚠ **The question this lane must answer, and it must be answered BY DESIGN SESSION AND MEASUREMENT — never by picking a default value:** *should a delivery ack originate a DH ratchet boundary, and should it count as post-compromise-security liveness?* An ack genuinely does prove the device is live, so "no boundary" is **not** obviously correct — which is exactly why it needs a ruling rather than an implementation.
- **The four findings NA-0682 measured, all of which the flip lane inherits:**
  1. ⚠ **The ack CONSUMES the DH ratchet-on-reply boundary.** The recipient's automatic ack becomes their first send, so the ratchet rotates on a control message instead of a human reply. Proven by single-variable experiment: acks `Batched` -> `handshake_mvp::dh_ratchet_e2e_roundtrip_over_real_handshake` FAILS; acks `Off` -> PASSES.
  2. ⚠ **It triggers a PQ RESEED per RECEIVED MESSAGE** (`qsp_pq_reseed dir=send` on the ack path). Post-quantum reseeds are the most expensive operation in the system; this moves them from once-per-exchange to once-per-receive. **Battery, latency and CPU cost: UNMEASURED.** Measuring it is a precondition of the flip, not a follow-up.
  3. **Every receive produces a send** — a structural per-message timing signal, only partly blunted by the batch window and jitter.
  4. **Envelope shape differs** between a boundary-originating send and a non-boundary one.
- ⚠ **ANNOTATION added 2026-07-30 by NA-0688 (D-1327; directive D622 C3): finding 3's MITIGATION CLAUSE is corrected by measurement.** Mark-don't-rewrite — the finding above is left exactly as filed, because what it got wrong is the interesting part. *"only partly blunted by the batch window and jitter"* names two mitigations and **neither one exists**:
  - `RECEIPT_BATCH_WINDOW_MS_DEFAULT` (250) is read at exactly **two** runtime sites, and **both only echo it into a marker**. Nothing waits on it. The "window" consumes no time at all.
  - `jitter_ms` is used **only as a stable-sort key bias**. It reorders receipts within a flush; it delays none of them.
  So the two knobs blunt **nothing** today — one is inert, the other is ordering-only. D622 RULING 2 kept both **documented honestly where they live** rather than removing them; removal is a later cleanup, explicitly not this lane.
  **What actually reduces the signal is coalescing, which the filing never named:** receipts ride the **end-of-pull flush**, so the cost is one send per **PULL**, not one per received message. The rest of finding 3 — *"every receive produces a send"* — is **TRUE and unchanged**, and NA-0688 records it as an accepted honest limit instead of claiming it away.
- ⚠ **SEQUENCING — binding:** the flip decision must **CONCLUDE BEFORE Slice 4's DELIVERED-rendering design settles.** Slice 4 renders the `✓✓` glyph; designing that UI against a default that is still undecided would bake the assumption in backwards.
- Precedent this follows: **F5 kept ENG-0043's lease-default flip OUT of NA-0682** on exactly this reasoning — a default flip that changes protocol behaviour deserves its own deliberate step. The epic stays internally consistent by treating both the same way.
- What already ships and must NOT be rebuilt: the fields, both knobs, the honour path, `delivered_receipt_roundtrip` (A4), the A12 forged-ack refusal, the `ack_for_unknown_msg_id_transitions_nothing` guard, and the two default PINS (`message_state_tests::receipt_default_is_off_recipient_half`, `transport::receipt_sender_default_tests::sender_requests_no_receipt_by_default`). ⚠ **Those two pins are designed to go RED when the flip lands** — flip them in the same commit, with the measurement above attached.
- ⚠ **BINDING ANNOTATION added 2026-07-29 by NA-0686 (D-1325), operator-ruled, verbatim:**

  > The flip commit migrates #4 in the same commit as the default flip: fixture requests an
  > explicit receipt (mechanism-by-explicit-flag, the intended Option D shape), making the
  > timeline id equal the queue msg_id, then first-party acquisition per the proven remedy,
  > binding condition in full. Until then the loud sentinel is the interim guard — if the
  > default flips without the migration, #4's sentinel check fires red BY DESIGN. That red is
  > the tripwire working, not a surprise.

  Context, so the annotation is actionable rather than cryptic: NA-0686 could not migrate
  ENG-0087 instance #4 (`peer_confirm_policy_primary_only_na0177`) to first-party acquisition,
  because `qsc util receipt-apply --msg-id` keys on the **timeline entry id**, which equals the
  queue record's `msg_id` **only when the send requested a receipt** — and that test requests
  none. Measured RED: `event=error code=state_unknown`. The flip is precisely the flag that
  changes this, which is why the migration belongs to the flip commit and not before it.
- ✅ **RESOLVED at NA-0688 (D-1327; directive D622 C3, operator ruling on STOP #016 option (a)) — 2026-07-30.** Both halves are ON by default and the disposition was **re-derived from measurement after the fix**, not from the fact that the code changed.
  - **Recipient-honours half: live.** `ReceiptPolicy::default().mode` is `Batched`. Measured end to end: a `qsc receive` with no flag at all queues and flushes an ack (`QSC_RECEIPT mode=batched status=queued` → `status=sent`).
  - **Sender-requests half: live, and it very nearly shipped broken.** The flip was first written as a new default on `RelayMessageSender::new` — where D622 §1b.4 located it — and **measurement showed that value never reached the wire**: `qsc send` builds its sender with `.with_meta(…, receipt)`, and `with_meta` assigns the caller's choice **unconditionally**, so an absent `--receipt` overwrote the new default. `qsc outbox retry`/`discard`, which do not call `with_meta`, DID inherit it. **In that state two users touching no flags would have exchanged no receipts at all**, while every existing test passed — because the two halves were each pinned in isolation and nothing asserted that the constructor's value survives to the wire.
  - ⚠ **The evidence was a NEGATIVE result from an instrument that could have gone positive.** The D-1325 tripwire quoted above — *"if the default flips without the migration, #4's sentinel check fires red BY DESIGN"* — **did not fire** under the full flip; both `peer_confirm_policy_primary_only_na0177` message tests passed. It fires immediately once a fixture requests a receipt explicitly, which is how the migration below was driven.
  - **The fix (ruled option (a)):** an absent `--receipt` means **the policy default**, resolved through one function (`resolve_sender_receipt_request`) at **all three** production construction sites, so a row queued by a default `qsc send` has the same receipt semantics via send, retry and discard. `with_meta`'s verbatim contract is untouched — the resolution happens before it. ⚠ **A new explicit spelling `--receipt off` was added**, because the ruling requires explicit-off to work end to end and the old CLI had no way to say it: `--receipt` was `Option<ReceiptKind>` over a one-variant enum, so "absent" was the only way to mean "no receipt", and that spelling is now taken.
  - **The successor tripwire, and it is red-capable:** `na0688_c3_sender_default` — a default send requests on the wire; a default send and an `outbox retry` of a default-queued row agree; an explicit `off` still means off. The observable is **the peer's ack**, never a sender-side field, so it proves what arrived rather than what was intended. Red control (send path resolving as it did pre-ruling): **2 of 3 RED**, with `off` still green — `census/c3_sender_guard_RED_CONTROL.log`.
  - ⚠ **USER-VISIBLE CONSEQUENCE, owed to D-1327 as a PRODUCT STATEMENT:** *with acks on by default and A6 reversed, **DELIVERED is not available for a message until its recipient has sent at least once**; until then the sender sees SENT, the receipt waits in the durable hold, and it flushes to DELIVERED on the recipient's first send.* Both halves of that window are pinned at the GUI-contract layer — the SENT state during it (`desktop_gui_contract_na0215b` ARM 1) and the transition after it (ARM 2, which also proves the receipt that arrives is the one owed for the ORIGINAL message). Slice-4 UX candidates that could shrink or reframe the window — suppress-until-bidirectional, proactive (prekey-style) establishment, app-generated first message — are cross-referenced and **not implemented by this lane**; the first act of such a lane is measuring what the invite bundle already publishes.
  - ⚠ **A claim of mine that measurement cut down, recorded because I had it wider.** STOP #016 §4 said the same queued row would be *retried* with a receipt request it was never sent with. **False for the normal case:** `msgqueue::attempt_one` packs a record at most once and replays the bytes verbatim (a crypto-safety invariant), and `receipt_kind` is consumed at pack time — so the caller's choice is already persisted as ciphertext. Only a record whose **first pack failed** is still unpacked when a retry re-resolves. Recorded as a residual on **ENG-0096**, where the row schema gains fields anyway.
  - **The three default pins are migrated, not rewritten down** — and note the entry above says "the two default PINS": the census found a **third**, named in no prior record. `receipt_default_is_off_recipient_half` → `receipt_default_is_batched_recipient_half`; `sender_requests_no_receipt_by_default` → `sender_requests_a_delivered_receipt_by_default`; `with_meta_does_not_enable_receipts` → `with_meta_takes_the_callers_receipt_choice_verbatim`, whose fixture now disables explicitly first because a default-constructed sender would otherwise assert nothing.
  - ✅ **The D-1325 binding annotation is DISCHARGED.** ENG-0087 instance #4 is migrated to first-party acquisition in the flip commit, binding condition in full: both fixtures request an explicit receipt, the timeline entry id therefore equals the queue record's `msg_id`, **old RED-then-retired** (the scrape fails on the redaction sentinel once ids widen — `census/c3_eng0087_OLD_RED.log`) and **new RED-then-green** (a never-minted id is rejected `code=state_unknown` — `census/c3_eng0087_NEW_RED_CONTROL.log`, restored byte-identical). The property was unguarded at no commit.
  - **Finding 3's mitigation clause is corrected by measurement** — see the annotation above. **Finding 4 is measured and filed as ENG-0098** (distinguishable by size for any message that does not fit under the 1024 floor; R2b's prescribed remedy cannot close it).
  - ⚠ **Findings 1 and 2 were closed by C2's passivation, and C2 shipped with a defect that C3's flip exposed** — an establishing control send cleared `pending_send_ratchet`, consuming the human's owed reply rotation. Caught by **C2's own guard**, one commit later, because the default changed underneath it. The first attempt at the fix keyed on the boundary permission, which is true for an establishing control send, so it read as correct and changed nothing; instrumenting the trigger state at each pack is what named it. That is why the flip and its pins had to live in the same lane.
- Cross-reference: D-1317; D617 §4 F6 (amended); STOP 016/017/018; ENG-0043 (the precedent); OBS-ET, OBS-EV, OBS-EW, OBS-EZ; **NA-0686 / D-1325 (the binding annotation above); ENG-0087 instance #4**; **ENG-0098** (finding 4, measured and deferred); D622 STOP 012/013/014/015.

### ENG-0087 — tests that learn IDENTIFIERS by scraping DIAGNOSTIC MARKERS are coupled to redaction policy — **NEW; filed 2026-07-28 by NA-0682 (D-1317; directive D617, operator-ruled STOP 019 item 3)**
- Severity: P2 (assurance, suite-wide; one confirmed instance, population UNKNOWN and unenumerated)
- Status: open — filed 2026-07-28. **FILING ONLY — deliberately not swept in NA-0682**, which is a product lane. Belongs to the CI/tooling–audit family (ENG-0075/0077/0078).
- **The confirmed instance:** `message_state_model::replay_ack_does_not_advance_state` learned a message id by parsing `id=` out of the `event=timeline_item` marker. NA-0682 widened `msg_id` from 16 to 32 hex chars — to stop emitting `sha512(plaintext)[..8]`, the C17 leak — which crossed the marker layer's `len() >= 24` redaction threshold (`src/output/mod.rs:292`). The scrape then returned the literal string `<redacted>`, and the test used it as an id.
- ⚠ **Why this class is nastier than a normal broken test: the sentinel PARSES AS A VALID IDENTIFIER.** The scrape did not fail, so the test proceeded and failed much later, in a different subsystem, with a misleading reject code (`state_unknown` instead of `state_duplicate`) — which cost a full diagnostic cycle to trace back. A test that fails at the point of the defect is cheap; this one failed three steps downstream.
- **Rule going forward (the thing this ENG exists to establish):**
  1. tests acquire identifiers **FIRST-PARTY** — from the store, the return value, or the fixture that minted them. NA-0682's `first_party_sent_msg_id` reads the id from the sender's own queue record filename and is the reference shape;
  2. any legacy scrape that remains must **FAIL FAST if the scraped value equals the redaction sentinel**, rather than proceed on `<redacted>` as data.
- **Work required:** enumerate every marker-scraping test in the suite. ⚠ **NA-0682 fixed one and did not count the rest** — the population is genuinely unknown, and a passing suite is NOT evidence that the rest are healthy, because a scraped sentinel can leave a test green while it silently asserts nothing.
- ⚠ **Carries OBS-FA:** a redactor keyed on VALUE SHAPE (`len() >= 24 && has_digit`) makes **any change to an identifier's width a behavioural change to the diagnostic surface**. Nothing in the code, the tests, or the docs declares that coupling today. That is the root condition; the scraping tests are just where it surfaced first.
- ⚠ **ENUMERATION (operator-ruled, STOP 021 Ruling 4 — completed 2026-07-28, READ-ONLY).** Instance #2 surfacing inside the same lane settled that filing without enumerating was not enough.

**What was searched** (recorded because an empty result would be a failure of the grep, not evidence of absence), across `qsl/qsl-client/qsc/tests/`:
  - `strip_prefix("id=")` and `strip_prefix("msg_id=")`
  - files containing `event=timeline_item`
  - helper names implying id extraction: `*_id_and_state`, `*first_item_id*`, `*_msg_id(`
  - `split`/`splitn`/`after` on `"id="`
  - every test passing `"--msg-id"` to a CLI verb (the place a scraped value does damage)

**The population, with measured status:**

| # | instance | status |
|---|---|---|
| 1 | `message_state_model::replay_ack_does_not_advance_state` | **WAS RED → FIXED in-lane** (first-party via `first_party_sent_msg_id`) |
| 2 | `timeline_delivery_contract_na0217f::receipt_apply_ignores_wrong_device_…` | **WAS RED → FIXED in-lane** (same remedy; the file's scrape helper now returns STATE ONLY, so the class cannot recur there) |
| 3 | `message_state_model::wrong_peer_ack_rejected_no_mutation` (`:406`) | ⚠ **GREEN, coupled, NOT vacuous.** Its send uses `--receipt delivered`, so the scraped id *does* degrade to `<redacted>` — but the assertion lands at `event=qsp_unpack code=qsp_hdr_auth_failed`, i.e. the forged ack is refused by the AEAD before any id is consulted. The property is genuinely exercised. **It would become vacuous if the assertion were ever moved to a state-machine-level rejection.** |
| 4 | `peer_confirm_policy_primary_only_na0177.rs:216` → `--msg-id` at `:292`, `:323`, `:390` | ⚠ **GREEN, coupled.** Green only because that test does **not** request a receipt, so its timeline id is the short `out-<ts>` form and escapes the `len() >= 24` redactor. **It breaks the moment that test requests a receipt, or ids widen again.** |
| 5 | `desktop_gui_contract_na0215b.rs:643`, `timeline_store.rs:136` | **NOT in the class** — they `contains()`-assert on `event=timeline_item` and extract no identifier. |
| — | `outbox_abort.rs:120` | **NOT in the class** — passes a literal `--msg-id deadbeef`, nothing scraped. |

**Confirmed count: 4 in the class (2 fixed, 2 green-but-coupled).** Per the ruling, the two green ones are **enumerated, not swept** — they belong to this ENG's own lane. ⚠ **#4 is the one to fix first**: it is one flag away from failing, and it feeds a scraped value straight into a CLI verb.
- Cross-reference: D-1317; STOP 018/019/021; ENG-0084 (the same coupling, seen from the emission side); OBS-EC (marker layer vs user-cause layer); OBS-EY; OBS-FG.

- ⚠ **RE-ENUMERATED AND RESOLVED 2026-07-29 by NA-0686 (D-1325).** The population was
  **re-measured, not inherited** (the STOP-021 table was taken at an older head). Searched
  across `qsl/qsl-client/qsc/tests/`: `strip_prefix("id="/"msg_id=")`, files containing
  `event=timeline_item`, helper names implying id extraction, `split`/`splitn`/`find_map` on
  `"id="`, every test passing `--msg-id`, plus an EXTENDED probe for marker scrapes of any
  `key=` form. **Result: the id class has exactly TWO remaining scrape sites**
  (`message_state_model.rs:116`, `peer_confirm_policy_primary_only_na0177.rs:216`).
- **Instance #3 (`wrong_peer_ack_rejected_no_mutation`): MIGRATED.** Id now acquired
  first-party; `timeline_first_item_id_and_state` replaced by `timeline_first_item_state`,
  which **cannot return an id at all**, so the class cannot recur in that file by
  construction. ⚠ The asserted property is byte-identical AND the test got stronger: it used
  to forge an ack carrying the SENTINEL, so it proved a forgery carrying garbage is refused;
  it now proves a forgery carrying the CORRECT id is refused, which is what its name claims.
- ⚠ **Instance #4: THE PRESCRIBED REMEDY DOES NOT TRANSFER, measured RED.** The intent
  directed `first_party_sent_msg_id` here too. But `qsc util receipt-apply --msg-id` keys on
  the **TIMELINE ENTRY id**, minted as `forced_id.unwrap_or_else(|| "{dir}-{ts}")`, and the
  send path's only `forced_id` is `receipt_msg_id` — populated **only when a receipt was
  requested**. These two tests request none, so their entry id is the short `out-<ts>` form,
  which is NOT the queue record's 128-bit `msg_id`, and `QueuedMessage` carries no timeline
  id to read instead. Substituting the first-party helper was tried and measured
  **`event=error code=state_unknown`**. First-party acquisition is genuinely unavailable
  without changing the scenario (requesting a receipt would pre-empt ENG-0086's flip).
  **So instance #4 takes this entry's OWN rule 2 instead: the scrape stays and is made
  loud.** The coupling is not removed; it is made to fail AT the defect.
- **THE SENTINEL FAIL-FAST RULE IS LIVE**, as one field-agnostic implementation:
  `common::scraped_marker_value(field, value)` refuses `<redacted>` with a message naming the
  field and the remedy. Red control: disabling the rule turns its guard
  (`scraped_marker_value_refuses_the_redaction_sentinel`) red; restored byte-identical.
  NA-0682's hand-rolled copy inside `first_party_sent_msg_id` now routes through it, so there
  is ONE implementation of the rule rather than a copy per site.

**ANNEX — the wider same-pattern population (enumerated, NOT fixed; scope held by ruling).**
The id class is two sites, but the *same scrape shape* appears at roughly **60 further sites**
across ~45 test files, for these fields:

| field scraped | approx. sites | crosses the redactor today? |
|---|---|---|
| `identity_fp=`, `identity_kem_pk=`, `identity_sig_pk=` | ~45 | no — `fp` keys are explicitly ALLOWED by `should_redact_value` |
| `device=` | ~20 | no — short device markers |
| `state=` | 3 | no |
| `invite=`, `send_seq=`, `max=` | 3 | no |

⚠ **None is an ENG-0087 instance today, and every one of them is the same latent coupling:**
their safety is a fact about VALUE WIDTH, not about the code. The remedy is ready and costs
one call each — `scraped_marker_value` was deliberately written field-agnostic so this
population can adopt it without redesign. Left for a later ruled lane.

### ENG-0095 — the DELIVERY-RECEIPT send path committed its ratchet advance AFTER pushing, so a failed ack handed its message key to the next send — **NEW; filed 2026-07-30 by NA-0688 (D-1327; directive D622 C0), FIXED IN THE SAME LANE**
- Severity: **P1** (crypto-safety, Tier-1: nonce reuse under AEAD. Reachable only with receipts enabled, which is why it had not bitten — and NA-0688 exists to make receipts the DEFAULT, which is what made it urgent)
- Status: **FIXED at NA-0688 (D-1327, directive D622 C0-A, 2026-07-30)** — filed and fixed in the same lane, per the operator ruling, with the hazard MEASURED RED before the fix existed.
- **The defect.** `send_delivered_receipt_ack` and `send_file_completion_ack` (`qsl/qsl-client/qsc/src/lib.rs`) ran **pack → push → commit**: `qsp_pack` advanced the send chain in memory, the envelopes were pushed, and only then did `qsp_session_store_with_trigger` make the advance durable. A push failure returned early, so the advance was **dropped** — and the next send on that chain re-packed at the same index, deriving the same message key. If the abandoned ciphertext reached the relay (push sent, response lost — **the common path**), two plaintexts exist under one AEAD key.
- ⚠ **It was silent by construction.** `relay_inbox_push` returns `&'static str`, which `From` maps to `ReceiptSendError::Soft`; `send_pending_receipt` emits `receipt_send_failed` and returns `Ok(())`; `flush_batched_receipts` then **continues its loop to the next receipt**. So a single transient failure could hand the same index to a second ack **in-process, with no crash required**.
- **MEASURED, not inferred** (the filing's own grounds). `tests/na0688_eng0095_ack_nonce_barrier.rs::an_ack_whose_push_failed_still_advances_the_send_chain` — a SINGLE-VARIABLE two-arm experiment whose only difference is whether a receipt was attempted; the failure is injected surgically with `set_fail_pushes(1)` armed **after** the sender's message has landed, and the subject arm asserts `event=receipt_send_failed` so a run where the injection missed cannot pass silently. **RED before the fix, both arms at `msg_idx=0`** — the index the ack burned was handed straight back. GREEN after, with the test file **byte-identical** (sha256 unchanged across both runs).
- **The fix (C0-A).** Ordering, stated as an invariant in the code: (1) resolve the route token FIRST — it is fallible and must not sit between pack and commit; (2) pack; (3) **COMMIT, fail-closed** — a failed commit attempts NO push and the receipt is dropped through the existing soft marker; (4) only then push. Both receipt kinds move together, because a barrier covering one of two sibling paths is not a barrier.
- **Accepted consequence, recorded:** a push failure now **BURNS the index** — the receipt is lost and its key is spent. Same semantics as the user send path; the recipient absorbs the gap via the standard skipped-key machinery; and the loss self-heals once lease is the default (ENG-0043 / D622 C4), because a redelivered message is re-acked.
- ⚠ **Two Director characterisations of the alternative path were WRONG and were corrected before any commit** (recorded because the correction is the reusable part): routing acks through `transport::relay_send_with_payload` was said to "commit before push" — it commits the **outbox-replay slot** before pushing, but the **session** commit is in `finalize_send_commit`, reached only from the successful-push arm — and was said to write no timeline entry, when it passes a `TimelineSendIngest` unconditionally on success. The first error meant the ruled fix **left the test red**; the second meant it would have made every receipt create a `kind="file"` timeline row, violating `DESIGN_outbox_delivery_v1` §5. Both were caught by re-reading with the standing **"a verification query states the size of the surface it examined"** rule — the original grep had stopped one line short of the function's extent.
- Cross-reference: `msgqueue::retire_packed` (the same rule on the queue path) and NA-0155's retirement record in `tests/ratchet_durability_na0155.rs`; **ENG-0096** (the owed queue migration); D-1317's nonce-reuse finding, of which this is the sibling instance on the receipt path; D622 STOP 001/008/009.

### ENG-0096 — delivery receipts should ride the RETRY QUEUE (the "same retry machinery" clause of DESIGN_outbox_delivery_v1 §5), which needs a control-message kind on the queue row — **NEW; filed 2026-07-30 by NA-0688 (D-1327; directive D622), DEFERRED-OWED by operator ruling**
- Severity: P2 (design conformance + reliability; the receipt path is crash-safe after ENG-0095 but has **no scheduled retry**)
- Status: open — filed 2026-07-30. **Deliberately deferred, not dropped.**
- **The gap.** `DESIGN_outbox_delivery_v1` §5 requires the ack be *"sent through the recipient's OWN outbox (same retry machinery)"*. Measured at NA-0688: there are **two** send paths — the direct one (`relay_send_with_payload`, single-slot outbox, no scheduled retry) and the NA-0682 **msgqueue** (`enqueue_at` → `drain_at`), which is the one with the retry machinery and with `retire_packed`. Receipts use neither; after C0-A they pack-commit-push in place. So the §5 clause is **unmet**.
- **Why it is not a one-line move.** (a) `QueuedMessage` has **no kind/is_control field** and `ContactQueueSummary` has **no discriminator**, so an enqueued ack would be counted in the user-visible *"N messages queued"* summary that DESIGN §6 renders — violating §5's *"invisible in their UI"*. Closing that needs a **new field plus summary discrimination**. (b) Routing acks through the queue puts a **new class of message** through `retire_packed`'s callers, which D622's own stop condition placed off-limits for a defaults lane.
- ⚠ **What makes the deferral tolerable, and the condition under which it stops being tolerable:** a lost receipt **self-heals** once lease mode is the default — the relay redelivers, and a duplicate incoming message **is re-acked** (confirmed from source: `transport/mod.rs`, *"A duplicate is still ACKED (idempotently): the sender's ack may be what was lost…"*). **That safety net DEPENDS ON the lease default (ENG-0043 / D622 C4). If the lease default is ever reverted, this filing's urgency changes and it should be re-triaged immediately.**
- ⚠ **RESIDUAL ADDED 2026-07-30 by NA-0688 (D-1327; directive D622 C3, operator ruling on STOP #016): the queue row cannot express the SENDER's per-invocation receipt choice.** Recorded here because this is the filing where the row schema gains fields anyway.
  - **The shape.** After the ruling, an absent `--receipt` resolves against `ReceiptPolicy` at each invocation (`resolve_sender_receipt_request`). The `QueuedMessage` record has **no field** for what the caller asked, so anything that re-resolves later cannot know it.
  - ⚠ **MEASURED, and it is much narrower than it first appears — I had claimed it wider in STOP #016 §4 and the measurement corrected me.** `msgqueue::attempt_one` packs a record **at most once in its life** (`if !rec.is_packed()`) and every later attempt **replays the same bytes verbatim** — an existing crypto-safety invariant, since re-packing would burn a second message key and desync the session — and `RelayMessageSender::pack` consumes `receipt_kind` at pack time. **So for a normally-queued row the caller's choice IS persisted, as packed ciphertext rather than as a field, and no retry can alter it.**
  - **What actually remains open:** a record whose **first pack FAILED** is still unpacked when a later drain runs, so that drain re-resolves against the policy — and an explicit `--receipt off` on the original invocation would be lost. Bounded to the pack-failure path, which is rare and already terminal-ish, but real.
  - **Why it is not fixed here:** the remedy is a field on the row, which is precisely the schema change this ENG already owns. Adding one field for receipts alone, from inside a defaults lane, would pre-empt the control-message-kind design the filing exists to do properly.
  - Guarded as far as it can be without that field: `na0688_c3_sender_default::a_default_send_and_an_outbox_retry_agree_on_the_wire` pins the end-to-end agreement for the normal (packed) path and says in its own doc-comment what it does not reach.
- Cross-reference: ENG-0095 (the barrier that made the direct path safe); ENG-0043 (the lease default this depends on); ENG-0083 (in-flight ratchet state persisted in two places — the same two-path split, seen from the storage side); **D622 STOP #016's ruling (option (a)) and the §4 correction above**; D622 STOP 008/009.

### ENG-0097 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0097`.

### ENG-0098 — an observer can tell a DELIVERY ACK from a USER REPLY by envelope size, because acks are padded to a fixed floor and user messages are not bucketed by default — **NEW; filed 2026-07-30 by NA-0688 (D-1327; directive D622 C3, R2b), DEFERRED-OWED**
- Severity: P2 (metadata privacy; **no confidentiality impact** — the distinction is in envelope LENGTH, never in plaintext, and every payload stays inside the session AEAD)
- Status: open — filed 2026-07-30. **Deliberately deferred, not dropped.** R2b's prescribed remedy was measured and **cannot close this**, which is the reason it is a filing rather than an in-lane fix.
- **MEASURED, from the relay's raw stored bytes — an observer's view, not the client's.** `na0688_c2_passivation::e3_measure_envelope_distinguishability`, three arms drained separately so each number is labelled by construction rather than read positionally:

  | arm | envelope lengths |
  |---|---|
  | bob's ack alone | `[1024]` |
  | bob's SHORT user reply (20-byte body) | `[1320, 1024]` — 1320 is an SCKA advertisement PRE-envelope; the reply itself is 1024 |
  | bob's LONG user reply (4096-byte body) | `[17682]` |

- ⚠ **FIGURE CORRECTED 2026-07-31, and the correction is mine to own.** This entry first recorded the long reply as **5288**. Warming up the E3 fixture for the A6 reversal changed bob's session state, and the same instrument now measures **17682**. **The conclusion is unchanged and in fact strengthened** — 17682 against a 1024 ack is *more* distinguishable, not less — but a published figure must match what the instrument produces, so it is corrected rather than left to rot. Recorded because it is the same class as the estimate-vs-measurement lesson this lane keeps relearning: **a number stops being true the moment the fixture that produced it moves.**
- **What the numbers say.** An ack is **always** padded up to the Standard 1024 floor. A user message is **unbucketed**, so it coincides with the floor only while its body fits underneath and takes its natural size otherwise. So a short reply is size-indistinguishable from an ack **by accident**, and any message that does not fit under the floor is plainly distinguishable. ⚠ Envelope **COUNT** is a second signal: a user send that also mints an advertisement emits two envelopes back-to-back where an ack emits one.
- ⚠ **Why R2b's prescribed remedy was NOT applied** (the decision, recorded either way as the ruling required): R2b says *"pad receipts to standard small-message envelope sizes ONLY if distinguishable."* They are distinguishable — but **the ack is already the padded one.** Padding a receipt further cannot make it resemble an unbucketed message of arbitrary length; the delta originates entirely on the USER side. Applying a remedy that a measurement shows cannot work, merely because a rule names it, would have produced a change with no effect and a false claim in the record.
- **What would actually close it:** bucket the USER message path by default, i.e. make `--pad-bucket`-style meta-padding the default rather than opt-in. That is a **wire-shape default** affecting every message, with its own bandwidth cost, and it is out of scope for a defaults lane that had already been told not to change wire format.
- **Route:** the same post-Stage-2 metadata track that owns cover traffic. Cover traffic and default bucketing are the two halves of the same problem and should be ruled together rather than one at a time.
- ⚠ **A correction to this lane's own earlier record, kept because it is the instructive part.** D622 STOP #013 reported the C2 measurement as *"ack 1024 · user reply 1212 — DISTINGUISHABLE"*. That pair was **never like-with-like**: the run actually produced `[1024, 1320, 1212]`, the 1320 was dropped from the report without comment, and the 1212 was a reply carrying a **PQ RESEED** — a consequence of the C2 establishment defect eating the reply's due rotation, not the shape of an ordinary reply. Once that defect was fixed the same fixture measured **1024 vs 1024**, which a one-sample instrument would have reported as "indistinguishable" and would have flipped the conclusion on an artefact of the chosen body size. The instrument now takes a short AND a long sample and drains between steps. **The conclusion survived; the evidence for it did not, and was replaced rather than carried forward.**
- Cross-reference: D622 R2b/R2d; `DESIGN_receipts_defaults_time_v1`; ENG-0086 finding 4; the metadata roadmap's sealed-sender / cover-traffic items (deferred post-Stage-2).

### ENG-0099 — a TYPED CONTROL PAYLOAD was delivered to the user as message content, because the data envelope and the typed-payload dispatch were never composed — **NEW; filed 2026-07-31 by NA-0688 (D-1327; directive D622 C3), FIXED IN THE SAME LANE**
- Severity: **P2** (correctness + UI truth; **no confidentiality impact** — everything stayed inside the session AEAD. Reachable only once receipts became the default, which is what composed the two mechanisms for the first time)
- Status: **FIXED at NA-0688 (D-1327, 2026-07-31)** — transparent framing: the receiver unwraps the data-control envelope FIRST, then dispatches the inner body exactly as it dispatches an unwrapped one.
- **The defect.** The receive loop sniffs typed payloads in a fixed order — attachment descriptor, file chunk/manifest, attachment confirm, file confirm — and classified the data-control envelope **LAST**. All six sniff sites keyed on the same raw bytes, so once a default `qsc send` wrapped its body, sites 1–4 were evaluated against the **wrapped** bytes and all missed; the envelope arm then unwrapped and **fell through to the generic user-message path**. The unwrapped body was never offered back to the typed dispatch.
- ⚠ **MEASURED, and it is the INVERSE of the failure first assumed.** The Director's initial reading was a silent drop — *"tolerance doing its job with no witness"*. Instrumentation showed otherwise: `recv_item idx=1 size=198` · `recv_item idx=2 size=216` · `message_state_transition CREATED->RECEIVED` · `recv_commit count=2`, and both files on disk. **The manifest did not vanish — it was written out, counted, and timelined as a RECEIVED user message.** The forward-compat tolerant-drop arm was never reached, and that arm already emits its own witness (`control_ignored`). There was no missing witness; the diagnostics were truthful and the **outcome** was wrong.
- ⚠ **This is the exact failure the `ns` namespace marker was introduced to prevent**, reappearing through a different door. `adversarial/payload.rs`, verbatim: *"C6 requires unknown control types to be IGNORED rather than rendered to the user as messages — otherwise DESIGN F2's 'a new ack type is a new type, no format break' is false, because an older client shows the new type's raw JSON as a message."* The envelope predates neither mechanism; the two had simply never been composed.
- **The fix.** The unwrap moves to the FRONT of the chain and every sniff site dispatches on the unwrapped `payload`. For traffic that was never wrapped, `payload` **is** the raw plaintext, so every branch is byte-identical to before — *wrapped equals unwrapped* holds **by construction**, not by assertion. One hop by construction: the tail envelope arm is **removed**, not bypassed, because a second unwrap there is what would turn "one hop" into an unbounded claim.
- **Guarded, both halves in one test** (`handshake_mvp::a_wrapped_typed_payload_dispatches_and_still_acks`): a wrapped `file_chunk` **dispatches** as a chunk, is **not** written as user content, **and** the envelope's receipt is still sent. ⚠ The halves are inseparable on purpose — the typed branches `continue` before the generic path where the receipt used to be queued, so a re-dispatch that forgot it would look perfectly correct while the sender sat on SENT forever for every message carrying a file. A second pin covers the ack-inside-an-envelope composition (`a_wrapped_ack_is_applied_acked_and_provokes_nothing_further`).
- **Evidence that the fix is not cosmetic:** six previously-red tests across four binaries went green **with zero fixture edits** — `attachment_streaming_na0197c` 17/0, `aws_file_robustness_na0186` 3/0, `aws_r2_file_integrity_na0189` 1/0, `file_transfer_mvp` 9/0 — the `aws` fixture `cmp`-verified byte-identical to pristine beforehand, so its manifest-mismatch rejection now genuinely fires **through the envelope**.
- **Rider, cheap and on its own merits:** the forward-compat tolerant-drop path already emits `control_ignored`; no new marker was needed there. Recorded because the original filing proposed one — **it would not have caught this defect**, since this never took that path.
- Cross-reference: D622 C3; `adversarial::payload` C6/F1 and the `ns` marker; ENG-0096; ENG-0098; D622 STOP 017/018.

### ENG-0100 — the NA-0625 `[ADV, reseed]` receiver property is pinned against OUR sender, not against an arbitrary conformant one — **NEW; filed 2026-07-31 by NA-0688 (D-1327; directive D622 C3), DEFERRED-OWED**
- Severity: P3 (assurance depth; the property itself is **not** in doubt and remains pinned — what is deferred is pinning it *sender-independently*)
- Status: open — filed 2026-07-31. **Deliberately deferred with the trade named**, not dropped.
- **The property.** NA-0625 / Operator Decision 2 retired the NA-0624 ADV/reseed pack exclusion: an authenticated advertisement may ride the SAME pack as a PQ reseed, consuming its chain slot in-order so `nr` passes through the control slot before the reseed's strict `n == nr` check. Three `scka_e2e_*` fixtures pin it, including a byte-offset corruption negative control on the co-packed frame.
- **What moved, and what did not.** D622 RULING 3 suppressed the SCKA advertisement on control sends and **recorded in advance** that *"`scka_advertise_due` timing shifts onto user sends, consistent with R1e."* ⚠ **NA-0625 is NOT superseded** — it made co-packing **permitted**, never mandated. Measured: after the A6 reversal the co-pack **returns on the default cadence** (`s3: [qsp_scka_adv adv_id=1, qsp_pq_reseed target_id=1]`).
- ⚠ **Why the fixtures nonetheless sit on an explicit `--receipt-mode off` arm, on grounds that are NOT the co-pack.** They choreograph an exact SCKA/reseed chain-index sequence, and ack traffic inserts extra chain steps that break it **after** the reseed (`qsp_hdr_auth_failed` at the post-reseed DH boundary). A **third** interaction, distinct from both the co-pack and the wedge. The scoped helper is used by those three fixtures only; the shared `recv_msg` is untouched, and the helper's doc-comment states the real reason rather than the obsolete one.
- **What is deferred.** Pinning the receiver property by **constructing an `[ADV, reseed]` pack directly** at the protocol layer — independent of any sender's cadence. That is the version that would survive future cadence changes and would hold against a conformant sender we do not control. Cost measured at ~350 lines (handshake replication, vault/session-blob decryption to load real state, refimpl `send_pq_advertise`/`send_pq_reseed`, and the corruption control rebuilt on the constructed frame).
- ⚠ **The trade, stated plainly rather than buried:** in v1 our client is the only sender, so the off-arm is full-strength for everything that can reach the wire today; building the direct construction mid-C3, in a lane where structural reasoning had already missed three times, risked exactly the green-proves-nothing pin the frozen-fixture ruling exists to prevent. **Sender-independence is deferred, not delivered.**
- **Route:** the negative-control audit track (with ENG-0075/0077/0078).
- ⚠ **NA-0625's decision text is UNTOUCHED**; this filing is the cross-reference, and D-1327 carries the pointer at closeout.
- Cross-reference: NA-0625 / Operator Decision 2; D622 RULING 3 and R1e; ENG-0099; D622 STOP 022/023/024/025/030.

### ENG-0101 — the `tui.*` vault namespace is a FOSSIL: four keys are read by nothing that can write them, and the prefix names a subsystem that was deleted — **NEW; filed 2026-08-01 by NA-0688 (D-1327; directive D622 C4, ruling R9)**
- Severity: P3 (no v1 consumer — see the scope note below; the defect is latent, not active)
- Status: open — filed 2026-08-01. **Both instances recorded deliberately**, so that fixing one cannot look like fixing the class.
- **The measurement.** Every `tui.*` key, counted by reader (`secret_get`/`account_secret_trimmed`) and writer (`secret_set`) across the whole repository — not just `qsc`:

  | key | reads | writes | status |
  |---|---|---|---|
  | `tui.receipt.mode` | 1 | **0** | ⚠ dead read |
  | `tui.receipt.batch_window_ms` | 1 | **0** | ⚠ dead read |
  | `tui.receipt.jitter_ms` | 1 | **0** | ⚠ dead read |
  | `tui.file_confirm.mode` | 1 | **0** | ⚠ dead read |
  | `tui.trust.mode` | 1 | 1 | live (`contacts/mod.rs`) |
  | `tui.relay.token` / `.token_file` / `.inbox_token` / `.ca_file` | 1 each | 1–2 each | live |

  ⚠ **The split is exact: every LIVE key has a dedicated writer; the four dead ones are precisely the receipt-policy group.** There is no generic setter — `qsc config set` was allowlisted to `policy-profile` alone — no `settings` verb, and **no exported `pub fn`** a GUI could call. Confirmed by a repo-wide scan including `qsl-desktop`.
- **Consequence.** `load_receipt_policy_from_account()` reads four keys that **nothing in the product can set**. The receipt policy is therefore not user-persistable: per-invocation flags (`--receipt`, `--emit-receipts`, `--receipt-mode`) all work, but a choice cannot be made to survive one command.
- ⚠ **What this does NOT mean.** C3 (`cdd5b1eb`) did **not** create this: the keys and their reader are TUI-era and predate it. C3's actual claim — that **both halves consult one policy**, so disabling receipts stops the asking as well as the answering — is **true and pinned by tests**. The gap is persistence alone. **The honest-claims consequence is carried by D-1327 (R10): the coherence mechanism is real; a user-facing persistent switch does not exist in v1; no UI text or doc may imply that it does.**
- ⚠ **The prefix is itself the fossil.** `tui.` names the terminal UI **retired and stripped in NA-0645** (~18.9k lines deleted). Any new key minted into that namespace carries a deleted subsystem's name into fresh code.
- **How this was found, and the second instance.** NA-0688 C4 needed a per-install ack-mode preference and added `tui.ack.mode` **by following the nearest precedent — which turned out to be the only unwritable subset of the namespace.** The key resolved correctly and nothing could set it. D622 R7 moved it to the **config file** as plain `ack_mode`, on three grounds: an ack mode is not a secret; a config-file preference **cannot silently fail to apply when the vault is locked**, which a vault-backed one can; and the GUI's future settings surface wants an unlock-independent store. ⚠ **A vault-backed transport preference would have shipped exactly the silent-divergence class this lane exists to remove** — applying on some invocations and not others, with no witness.
- **Scope note (D622 R9).** **v1 ships NO receipts toggle** (ruled in the Slice-4 session), so the receipt-persistence gap **has no v1 consumer** and this is not release-blocking. Recorded now so it cannot be rediscovered as a surprise.
- **Migration vs removal is the future lane's call**, not this one's: whether the four dead keys gain writers, move to the config file beside `ack_mode`, or are deleted outright. Natural rider on the already-queued naming/consistency sweep.
- **Route:** the naming/consistency sweep track.
- ⚠ **Precedent set by C4, and the one thing to carry forward:** per-install **preferences** live in the config file; the vault remains the pattern for actual **secrets** (relay tokens, CA paths). `src/store/mod.rs` carries an inline warning at the old key's site so the next person to add a preference does not repeat the copy.
- Cross-reference: D622 R7/R8/R9/R10/R11; D622 STOP 037; NA-0645 (TUI retirement); ENG-0083 (persistence homes); C3 `cdd5b1eb`.

### ENG-0102 — `config.txt` had two data-integrity defects that only a second key could reach: a clobbering writer and a false-corruption reader — **NEW; filed 2026-08-01 by NA-0688 (D-1327; directive D622 C4, ruling R12), RESOLVED IN NA-0688/C4**
- Severity: P2 (data integrity — one silently destroys a user setting, the other makes a valid store report as corrupt)
- Status: **RESOLVED in NA-0688 C4.** Filed with its own number because **a defect fixed inside a large diff without one disappears from the searchable record.**
- ⚠ **Latency note, preserved.** Both were present in the tree **before C4** and were **unreachable while `config.txt` held exactly one key**. C4 is what supplied a second key (`ack_mode`, per D622 R7), which is what made them reachable. Neither was named by D622, by STOP #037, or by R7 — implementing R7 literally would have *introduced* both.
- **(i) The old writer CLOBBERED the file — silent, bidirectional data loss.** `write_config_atomic` emitted exactly one line, `policy_profile=<value>`, and rewrote the whole file. Correct while that was the only key; the moment there are two, **setting the profile deletes the user's ack-mode preference and vice versa**, with no error. Now a **read-modify-write** preserving existing key order. ⚠ **`write_config_atomic` was REMOVED, not kept as a wrapper** — its only job was to hide a single-key format that no longer exists, and as a wrapper it would still have been the clobbering path anyone reached for.
- **(ii) `read_policy_profile` reported a valid file as CORRUPT.** It returned `Err(ParseFailed)` whenever the file lacked a `policy_profile=` line — and ⚠ **`doctor`'s `file_parseable` check calls it**. So `qsc config set ack-mode legacy` on a fresh install would have produced a config containing only `ack_mode=`, and **`qsc doctor` would then have reported the config store corrupt.** A user following the documented way to set the new preference would have broken their own diagnostics.
- ⚠ **The fix does NOT weaken corruption detection, and that was checked specifically because the easy fix does.** Parsing moved to one shared `read_config_kv`, which **still errors on any malformed line**. What changed is only that a *well-formed* file not mentioning a given key is no longer "corrupt" — it is a file that does not set that key. `config_get` already printed `unset` for that case and `doctor` already counted it parseable, so both callers were **already written for the tolerant reading**; only the reader disagreed with them.
- Cross-reference: D622 R7/R12; D622 STOP 038 §1a; ENG-0101 (the preference-store move that reached these).

### ENG-0103 — a directive's masthead `Goals:` line is INHERITED TEMPLATE TEXT, and three artifacts of one lane disagreed about the same lane's goals — **NEW; filed 2026-08-01 by NA-0689 (D-1328; directive D623 amendment A1.3), UNRESOLVED**
- Severity: P3 (record integrity — a goals claim that no one derived is a claim no one checked)
- Status: **OPEN.** Filed UNCONDITIONALLY per D623 amendment A1.3, which superseded §4b's conditional filing clause. ⚠ **Remediation is explicitly NOT this lane's** — header audits and derived-mapping discipline route to the consistency-sweep lane. This entry exists so the pattern outlives the lane that noticed it.
- **The pattern.** A directive's header carries a `Goals:` line that is copied forward from the template and then never re-derived against the tree it governs. D623's own header claimed `G4 (primary), supports G1–G3`; the mapping DERIVED from `GOALS.md` at base was **G2, G4, G5 — with G1 and G3 NOT claimed at all**. The header was superseded in place by A1.1 rather than rewritten, so the file still reads the inherited claim above the amendment that corrects it.
- ⚠ **THE MEASURED INSTANCE — three artifacts of ONE lane, three different answers.** NA-0688's goals were stated in its **queue block**, in the **D622 directive header**, and in **D-1327**, and the three did not agree. No mechanism forced them to, because each was written by a different step and none derived from `GOALS.md`.
- ⚠ **Why a line nobody derives is worse than no line.** The masthead is the first thing a reviewer reads and the last thing anyone re-checks; an inherited claim there is load-bearing in appearance and empty in fact. A goals line is a claim about a TREE, and like every counter in this program it must be checked against the tree it sits in.
- **The discipline that would close it** (for the consistency-sweep lane, not asserted as ratified): the goals mapping is DERIVED at drafting from `GOALS.md` at base, the derivation is stated in the P0 stop-file including its NEGATIVES (which goals are *not* claimed and why), and the queue block, the directive header and the decision record cite one source rather than three.
- Cross-reference: D623 A1.1 (the superseding mapping), A1.2 (P0.8, incorporated verbatim), A1.3 (this filing made unconditional); NA-0689 STOP #002; D-1327.

### ENG-0104 — `receive_pull_and_write:1012` acks unconditionally while its own comment says it must not: the receipt half is fail-closed and the ack half is not, in adjacent statements — **NEW; filed 2026-08-01 by NA-0689 (D-1328, ruling R4), UNRESOLVED**
- Severity: P2 (durability — a timeline row is lost on a path the code itself documents as must-not-ack)
- Status: **OPEN. FILING ONLY — NA-0689 deliberately changed NOTHING here.**
- ⚠ **THE COMMENT IS RIGHT AND THE CODE IS WRONG, AND THE FIX MUST MOVE THE CODE.** D-1328 Ruling 4 is explicit: **do NOT sweep the comment.** Editing prose to match broken behaviour would launder the defect out of existence and leave the tree self-consistent and wrong. The fix lane makes the CODE match the comment.
- **The defect.** The comment at the site states the contract in as many words — *if the row does not store, DO NOT ack* — and the code acks regardless of whether the row stored. Two adjacent statements handle the two halves of the same commit differently: the **receipt half is fail-closed**, the **ack half is not**.
- **What is and is not lost.** The payload bytes are already written at `:897`, so the message body survives. What is lost is the **timeline row** — the record that the message exists in the conversation. The item is then acked away, so the relay deletes it and redelivery cannot restore what was never stored.
- ⚠ **Found on the way to something else.** NA-0689's census reached this site while classifying discard points and ruled it **NOT a destruction site** — nothing is destroyed that this lane's quarantine could have kept — which is precisely why it needed its own number rather than a sentence inside a lane that was not going to fix it.
- ⚠ **Do not assume the shape of the fix from this entry.** Whether the correct behaviour is to fail the command, to skip the ack and let redelivery retry, or to store a degraded row, is a design question with a durability trade-off, and NA-0689 measured none of it.
- Cross-reference: NA-0689 P0 census §2c; D-1328 Ruling 4; NA-0644/D580 (the persist-before-ack ordering this site is meant to honour).
- **Resolution: RESOLVED at NA-0690 (D-1329, directive D624, 2026-08-02), merged `4bd987a2` in PR #1689, result class `QSC_ACK_GATED_ON_STORE_PASS`.** The ack at the generic user-message tail of `receive_pull_and_write` is now wrapped in `if stored.is_ok()`, so the relay is permitted to consume a pulled item only when the timeline row actually stored. ⚠ **D-1328 Ruling 4 was honoured exactly: the CODE moved and the comment did not** — the posture comment and the `not_stored_so_not_acked` marker literal are **byte-identical to base, proven by `cmp`**; the fix is what makes them true. ⚠ **The census mandated by D624 §4 returned a NARROWER verdict than the entry anticipated: `:1120` was the ONLY fail-open, NOT a class** — the other eight censused sites are fail-closed by three structurally distinct mechanisms — so no class fix was made and none is owed. ⚠ **Claim boundary, because the overclaim is available and wrong: redelivery does NOT recover the message.** `commit_unpack_state` consumes the ratchet key above the store, so the plaintext is lost either way; what the fix buys is that the ack is withheld (no false DELIVERED), the loss is loud and witnessed, what survives downstream is the opaque envelope quarantined by NA-0689 rather than the plaintext, and the loop is bounded to one cycle. **Status corrected 2026-08-03 by NA-0691 (D625 §4.8), which found this entry still reading `OPEN. FILING ONLY` two lanes after it was fixed; the `Status:` line above is left byte-identical per this ledger's header.**

### ENG-0105 — four of the five quarantine capture sites cannot be exercised end-to-end, because their triggers are hostile-peer and forward-compat behaviours a stock client never emits — **NEW; filed 2026-08-01 by NA-0689 (D-1328, rulings R11.4 and R12), UNRESOLVED**
- Severity: P3 (test coverage — a negative-control gap, not a product defect)
- Status: **OPEN. Routed to the NEGATIVE-CONTROL AUDIT track.** NA-0689 pinned every one of these decisions at the layer where it IS reachable (see below); what is missing is the wire-level arm, not the guarantee.
- ⚠ **THE FINDING IS A PROPERTY OF THE DESIGN, NOT A GAP IN IT.** Stated affirmatively, because a later reader will otherwise read the missing arms as an oversight: **four of the five sites' positives are unreachable from a stock peer, EACH FOR A REASON THAT IS THE SITE'S PURPOSE** — hostile-peer witnesses at D2–D4, the forward-compat witness at D5. **D1 alone is stock-reachable, because D1's trigger is our own crash rather than the peer's behaviour.**
- **D2/D3/D4 — `IgnoredWrongDevice` and `Err`.** Measured: the receive channel is whichever session decrypted the envelope, and `confirm_target_matches_channel` returns the ignore **only** when a **device-qualified** session for one device carries a confirm for an item whose pinned target is a different device — i.e. **the peer's second device confirming an item it never received**. The `Err` arm likewise needs the peer to name an item the receiver holds no record of. A shipped `qsc` emits neither.
- **D5 — `UnknownControl`.** Measured: `classify_control` returns it only for a payload carrying our namespace marker **plus** either an unknown `t`/`kind` pair **or** `v > CTRL_VERSION_MAX`. A sender of *this* build emits neither — same binary, same version ceiling, same three known shapes. **Only a FUTURE build can trigger it**, which is exactly what the seam exists for.
- **No injection seam exists today, and that was checked rather than assumed.** `util receipt-apply` calls the timeline function DIRECTLY and never enters `receive_pull_and_write` (which is why the existing NA-0177 wrong-device test does not touch the D-site); `util envelope` is cover-traffic ticks; there is no env override for the control version or kind; and all three `ReceiptControlPayload` builders are crate-private, so an integration test linking `qsc` cannot craft one.
- ⚠ **IF THIS IS EVER BUILT, THE SEAM BELONGS ON THE SENDER SIDE.** A test hook inside `receive_pull_and_write` is refused: it would make the arm measure a modified receive path, which is the one thing the arm exists to exercise. The correct shape is a sender-side pack/test-support seam — where the `#[cfg(qsc_binding_fuzz_helper)]` precedent already lives — so that a **crafted hostile frame feeds an UNMODIFIED receive path**.
- **What is pinned in the meantime, so the guarantee is not resting on this entry.** `transport::confirm_capture_reason_tests` pins D2/D3/D4's decision exhaustively over `ConfirmApplyOutcome` plus `Err`, and `transport::control_class_capture_tests` pins D5's exhaustively over `ControlClass`; both were **proven red-capable by probe** with the tree restored byte-identical. D1 keeps a full end-to-end pair against a real relay, and every site has an end-to-end **zero**.
- Cross-reference: NA-0689 STOP #015 §1a–§1b and STOP #016 §1a–§1b (the measurements, verbatim); D-1328 rulings R11.1, R11.2, R11.4, R11.5, R12.
- ⚠ **ADDENDUM (2026-08-02, D-1328 Ruling 14) — a SECOND instrument gap, in the same subject: a `clippy -D warnings` baseline is not a census of the tree.** Filed here rather than as a fourth id because it is the same finding class this entry already carries — *the instrument decides what is visible*.
  - **The defect it surfaced:** one needless `&` at `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs:150` (`assert_no_invalid_output(&first, …)`). **Pre-existing since `4ba069c6`, 2026-05-19** — `git diff 5888c686..HEAD -- apps/qshield-cli/` is **empty**, so NA-0689 neither caused it nor touched the crate.
  - ⚠ **Why it was invisible to the lane's own baseline:** under `-D warnings` **cargo stops scheduling once a crate fails**, so whether any given target is ever compiled depends on **what finished before the first failure**. NA-0689's base run reported summaries for `qsc (lib)` and `qsc (lib test)` only; a later run on a barely-different tree reported `qsc (lib)` and `qshield-cli (test …)`; a third reported all three. **Same repository, same command, three different populations.**
  - **Therefore: the number in a clippy baseline is a census of whatever cargo finished before it gave up, not of the tree.** A "zero delta vs N" gate built on it is not well-defined, which is why NA-0689's gate was re-formulated (D-1328 Ruling 15) as *zero-delta on the named `qsc` distribution, plus the stated deltas*.
  - **Remedy sketch, for the CI-tooling track and NOT done here:** run the gate with **`--keep-going`** (or per-package) so the reported set stops depending on scheduling order, and re-take **both** sides of any baseline the same way. ⚠ **Never compare a `--keep-going` baseline to a non-`--keep-going` final** — that substitutes one scope mismatch for another, which is the exact mistake this lane made once already on the test-suite side.
  - **NOT FIXED HERE, deliberately.** It is one character, and that is what makes it tempting: a lane that reaches into an untouched crate at its own gate is how a freeze stops meaning anything.
  - ⚠ **A prediction on record, and it was WRONG:** NA-0689 predicted this error would probably *disappear* once its own two were resolved, as the abort point moved again. It did not — it persisted, and the run scheduled *more* than before. **Recorded because the wrong prediction is the useful datum:** a stably-reported error is better evidence than one that flickers, and the scheduling dependence is real but not simply monotonic.

---

## External security audit — AUDIT-TRIAGE #001 filings (2026-08-02)

The eleven entries below (**ENG-0106 … ENG-0116**) come from an external security audit of the
QSL/QSC protocol core, handshake, vault, transport and storage layers (audit v2, July 2026,
sha256 `c7b87b88…4d2c59fa`) and its independent verification record (sha256 `77c10e99…096f4029`).
Every claim was **re-verified against `bd4f2a3a`** before filing — 23 rows censused, **21
CONFIRMED, 0 REFUTED** — and each entry below states **what is true of the tree**, not what
either source document asserts. Where the two documents disagree, or where the tree agrees with
neither, the entry says so.

⚠ **Out-of-band filing, per the ENG-0054…ENG-0058 precedent: no NA item and no D-record produced
these.** They were censused and filed directly against the ledger by operator ruling.

⚠ **Two findings were NOT given new ids, deliberately.** The audit's **F-01** (zeroization) **is
ENG-0055**, already filed 2026-07-21 — see that entry's 2026-08-02 addendum, which also absorbs
the audit's **F-07** and **N-05**. Filing a new id would have duplicated a stricter existing item.

> **NOTE — external audit F-05 (dual receive-dispatch), deferred, NOT an ENG.**
> F-05 (dual receive-dispatch; **5 `pub` send / 7 `pub` + 1 private receive** variants; the client
> bypasses the refimpl's `recv_wire` and calls `recv_dh_boundary` directly at `qsc/src/lib.rs:2317`):
> **CONFIRMED on `bd4f2a3a`**, re-measured at AUDIT-TRIAGE #001 §4.5 — **NA-0688's C3 transparent
> framing did NOT change the variant count.** Deferred to the dispatch-simplification /
> negative-control track (**ENG-0105**'s track). **Do not re-measure.**

---

### ENG-0106 — the vault write path takes NO lock, in-process or cross-process, and duplicates a weaker copy of the store's atomic-write primitive — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external audit F-09 + N-02/N-03/N-04/N-07), UNRESOLVED**
- Severity: **P2** (data integrity — silent, realistic loss of a user's secrets on a CLI a user or script may invoke in parallel)
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.**
- Exact surfaces: `qsl/qsl-client/qsc/src/vault/mod.rs` — `write_vault_atomic:845-877`, `vault_init_core:437` (temp `:559-562`, `exists()` `:520`, rename `:586`), `secret_set:223`, `secret_set_with_passphrase:249`, `persist_session:347`; `qsl/qsl-client/qsc/src/vault/protection.rs` `destroy_with_passphrase:247`; the working primitive next door, `qsl/qsl-client/qsc/src/fs_store/mod.rs` `write_atomic:174` and `lock_store_exclusive:326`
- **The defect, measured.** `lock_store_exclusive` and `lock_store_shared` appear **nowhere in `vault/mod.rs`**. Three vault write paths take no lock at all. Concurrency is nominally guarded by an in-process write epoch (`VAULT_WRITE_EPOCH:747`), and **that guard does not exist in practice**: the only code that reads the epoch is `persist_session`, which is `#[allow(dead_code)]` and dormant, and which on mismatch **merges rather than refuses** (`:349-366` re-reads the file, decrypts it, and inserts the session's secrets over the freshly-read payload — no refusal, no error path). The two live CLI paths, `secret_set` and `secret_set_with_passphrase`, only `fetch_add` the epoch (`:241`, `:274`) and **never read or compare it**. So there is **no cross-process guard and no in-process guard**: two threads calling `secret_set` race with nothing between them.
- ⚠ **THIS CORRECTS THE AUDIT, AND THE CORRECTION MAKES THE FINDING WORSE.** Audit v2 states the epoch means "a stale write is refused." It does not refuse; it merges, and only on a path that never runs. The finding is more serious than the source document describes, not less.
- **The mechanism the audit does not identify (N-02).** `write_vault_atomic` uses a **fixed** temp name — `path.with_extension("qsv.tmp")` (`:855`) — preceded by an **unconditional** `let _ = fs::remove_file(&tmp)` (`:856`). Two same-user processes therefore interleave concretely: **A** creates the temp and begins writing; **B** executes the unlink at `:856`, orphaning A's inode while A's descriptor writes on; **B** creates its own temp and writes; both rename over `vault.qsv`. **One process's secrets vanish with no error at any step.** `vault_init_core` uses the same fixed name (`:559-562`), guarding its removal with `exists()` — a cosmetic difference; the shared fixed name is the defect.
- **Also folded here, as the mechanism set behind this item:**
  - **(N-03)** `vault_init_core`'s `exists()`-vs-rename TOCTOU (`:520` vs `:586`): two concurrent inits both pass the check. Combined with the fixed keychain entry (**ENG-0108**), the loser's key can land in the keychain while the winner's file is on disk — **an unopenable vault, with no error at any step.**
  - **(N-04)** `write_vault_atomic` calls neither `enforce_safe_parents` nor `enforce_file_perms`, so it skips the symlink re-check and parent-safety walk that `fs_store::write_atomic` applies (`:180`, `:199`); it sets `0o700`/`0o600` directly instead.
  - **(N-07)** `persist_session`'s last-write-wins merge silently undoes a concurrent deletion and silently discards a concurrent edit to the same key. Harmless while dormant — but it is explicitly staged for the GUI, and a GUI is exactly where concurrent sessions become normal. **Decide the semantics before the GUI consumes it** (refuse-and-surface, or merge-with-conflict-reporting) rather than inheriting last-write-wins by default.
  - **(external F-11, OPTIONAL sub-item — `openat` / `O_NOFOLLOW`)** a TOCTOU window remains between `enforce_safe_parents` (`fs_store:180`) and the file open (`:194-197`). Both mitigations are real: the `create_new` (O_EXCL) open cannot be made to follow a planted file, and the parent must be a `0700` directory (`perms_group_or_world_writable:402`, applied at `:230`/`:249`). **Exposure is low; do this only if the code is already open for the work above.** Recorded here so it is searchable, deliberately not given its own id.
- ⚠ **A NAIVE FIX INTRODUCES A NEW SILENT FAILURE — DO NOT APPLY THE AUDIT'S RECOMMENDATION LITERALLY, AND PROBE BEFORE DESIGNING.** `flock` locks attach to the **open file description**, not the process, and every lock-taking helper in the tree opens the lock file itself. A caller that holds the store lock and then reaches one of those helpers may be denied **by its own lock**; with `LOCK_NB` set that denial is immediate (`LockContended`). **Three of the affected call sites discard the result with `let _ =`** — `protection.rs:159`, `:170`, `:281`. Applied as written, the audit's Priority-1 recommendation would leave `destroy_with_passphrase` destroying the vault and removing the keychain entry, then **silently failing to clear the protection state**, so a stale failed-unlock counter and attempt limit carry over to the next vault created in that config dir. **THE FIRST STEP OF THE FIX LANE IS THE REENTRANCY PROBE**, not a patch: acquire `lock_store_exclusive`, call `protection_state_clear_files()` in the same process, and assert on what comes back. It is the cheapest resolution of the largest uncertainty in the work, and it decides the shape of everything after it. Either way, **stop discarding these results**: `let _ =` on a lock-taking call is what turns an error into a silent one.
- **Remedy (shape only — the destination lane owns the design):** route the vault read-modify-write through the existing exclusive store lock, spanning **load → decrypt → mutate → encrypt → write** rather than the write alone, and replace the second write primitive with `fs_store::write_atomic` so one hardened implementation remains instead of two. Scope, sequencing and the reentrancy question belong to the vault-integrity lane.
- ⚠ **SEQUENCING, BINDING ON THE DIRECTIVE:** **ENG-0109 lands FIRST** — the vault cannot reach the store primitives until the two config-dir resolvers are unified, because `fs_store::write_atomic` and `lock_store_exclusive` both require a `ConfigSource` that `vault_path_resolved()` does not produce. **ENG-0107 lands AFTER this item** — it rewrites the same write path, so running the two concurrently conflicts. Order: **ENG-0109 → ENG-0106 → ENG-0107.**
- **Watch for:** `fs_store::write_atomic` returns `ErrorCode` while the vault returns `&'static str` markers. Map deliberately rather than collapsing distinct causes into one marker; the marker-contract tests pin the current vocabulary.
- Cross-reference: **ENG-0107** (envelope AAD — same write path, lands after), **ENG-0109** (prerequisite), **ENG-0111** (`LockContended` semantics, and the non-Unix silent no-op that makes any lock claim conditional), **ENG-0108** (N-03's concurrent-init interaction), **ENG-0055** (the `VaultRuntime`/`VaultPayload` material left unwiped on these same paths); AUDIT-TRIAGE #001 §4.9.
- ⚠ **Annotation (2026-08-05, appended by NA-0696/D-1336 per D630 R5 — NOT a `Resolution:` line; the partial-closure rule bars one).** What closed: D-1333 (NA-0693, D627 + Amendments 1–5) landed the core fix — the four vault write operations take the exclusive store lock across their whole read-modify-write, the duplicate write primitive is deleted, and N-02/N-03/N-04 closed as recorded there; D-1336 (NA-0696) then retired the per-site `_locked` regime in favor of the reentrant registry, discharging D-1333's binding forward requirement. The two recorded remainders that bar the line: (1) the N-07 `persist_session` refuse-not-merge semantic is DECIDED but its CODE rides Slice 4 (D-1333's own sentence); (2) the F-11 optional sub-item was ruled OUT and stays recorded. This entry stays OPEN until Slice 4 lands the N-07 code.

### ENG-0107 — the vault envelope is unauthenticated: no AEAD associated data at any of five call sites, and the KDF-parameter downgrade check is skipped entirely for keychain vaults — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external audit F-06, F-13 tamper-evidence half, N-06), UNRESOLVED**
- Severity: **P2** (an unauthenticated input steers key-derivation control flow)
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.** ⚠ **Carries an OPEN OPERATOR DECISION — see below. It is not ready to be planned against until that decision is taken.**
- Exact surfaces: `qsl/qsl-client/qsc/src/vault/mod.rs` — encrypt `:237` (`secret_set`), `:270` (`secret_set_with_passphrase`), **`:375` (`persist_session`)**, `:508` (`vault_init_core`); decrypt `:823` (`decrypt_payload`); header builders `:546-557` (inline in `vault_init_core`) and `encode_envelope:828-843`; the magic constant `:36`; the KDF-parameter check `parse_envelope:770`
- **The defect.** Every vault AEAD call uses the **two-argument** `.encrypt(nonce, plaintext)` / `.decrypt(nonce, ciphertext)` form — i.e. **empty associated data**. The envelope header (magic, `key_source`, KDF parameters, salt) is therefore not cryptographically bound to the ciphertext. This matters most for the `key_source` byte, because that byte **selects which key-derivation path runs** (1 = passphrase, 2 = keychain). An unauthenticated input that steers key-derivation control flow is precisely what associated data exists to protect against. Tampering is caught today only because decryption happens to fail, which is a weaker guarantee than "the header is authenticated."
- ⚠ **SITE COUNT — MEASURED, AND BOTH SOURCE DOCUMENTS UNDERCOUNT IT.** There are **FOUR** encrypt sites, not three. The verification record enumerates `:237`, `:270`, `:508` and omits **`persist_session:375`** — the very function it analyses at length elsewhere. **A migration that covers only the three listed sites leaves the dormant GUI persist path writing envelopes bound to nothing**, which will surface as an inconsistency the moment the GUI consumes that path. Bind **all four**, plus the decrypt at `:823`.
- ⚠ **THIS IS NOT "A FEW LINES," CONTRARY TO THE AUDIT'S CONSOLIDATED RECOMMENDATIONS.** Binding associated data **changes the on-disk format**: every existing vault was sealed under empty AAD and becomes unopenable without a migration. The magic already carries a version and is the natural carrier for one. The audit's own §11 estimate survived unchanged from its first revision and is the figure a lane would otherwise be planned against.
- ⚠ **OPEN OPERATOR DECISION, STATED HERE RATHER THAN RESOLVED — envelope break vs rewrap-on-unlock.** Either existing vaults are broken and re-created, or they are transparently rewrapped when next unlocked (read both magics, always write the new one, and retire acceptance of the old on a stated schedule once the writing release has shipped). The two differ in user-visible consequence, in support burden, and in how long the old acceptance path must live. **The audit's "few lines" framing conceals that this choice exists.** It must be taken explicitly and recorded before the lane designs against either.
- **Also folded here (N-06): the KDF-downgrade resistance the audit praises holds for passphrase vaults ONLY, and nothing in the code says so.** The parameter check at `parse_envelope:770` is conditioned on `parsed.key_source == 1`, so a `key_source == 2` envelope's KDF parameters are **not checked at all**. Not currently exploitable — the keychain path never reads them — but the asymmetry is undocumented and sits directly under a property the audit rates as one of the vault's best. Fix while the parser is open, or state in code why that path is exempt.
- ⚠ **SEQUENCING:** this item **rewrites the write path that ENG-0106 fixes. ENG-0106 lands first** (and ENG-0109 before it). Running them concurrently will conflict.
- **Gate shape (for the destination lane, not prescriptive):** a tamper test per header field — flip `key_source`, each KDF parameter, one salt byte — each producing a **distinct typed rejection** rather than the generic locked-vault marker; plus a round-trip proving an old-format vault opens and the next write leaves a new-format vault that still opens. Verify the gate **red at base** first: today `key_source` and salt tampering surface as a generic failure, and KDF-parameter tampering on a keychain vault is accepted outright.
- Cross-reference: **ENG-0108** (the half of the audit's F-13 that AAD binding does **not** close — kept severed deliberately), **ENG-0106** (same write path, lands first), **ENG-0116** (the `key_source` byte selects that mode), **ENG-0057** (the two-AEAD trust-base documentation item — adjacent, non-overlapping: that one is about *which primitives exist*, this one about *binding the header*); AUDIT-TRIAGE #001 §4.6/§6.1.
- **Resolution: RESOLVED at NA-0694 (D-1334, directive D628; line appended 2026-08-05 by NA-0696 per D630 R5/§8.1 — the slice-family Resolution audit found A–C carried no lines).** The vault envelope is AUTHENTICATED: the whole 53-byte pre-ciphertext header is AEAD associated data at every call site, rebuilt byte-exactly from parsed state, so any altered header byte fails authentication loudly; the format bumped HARD to `QSCV02` (no dual-read), a recognized-old vault refuses with its own name (`vault_version_unsupported`), and N-06 closed — the KDF-parameter check runs for BOTH key sources, rejecting any non-canonical stored profile. Measured at D-1334's gate: 594/0/2 across 128 local, the parse/AAD structural rows exact. See the D-1334 record.

### ENG-0108 — the keychain entry is addressed by two fixed constants and overwritten unconditionally, so a second profile's `vault init` silently destroys the first profile's key — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external audit F-13, the half its own recommendation does not reach), UNRESOLVED**
- Severity: **P2** (permanent, silent, user-triggerable data loss). ⚠ **FILED ABOVE THE AUDIT'S "LOW" — the reason is recorded below and was an explicit ruling, not an oversight.**
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.**
- Exact surfaces: `qsl/qsl-client/qsc/src/vault/mod.rs:69` (`VAULT_KEYCHAIN_SERVICE`), `:71` (`VAULT_KEYCHAIN_ACCOUNT`), `keychain_store_key:941`; the multi-profile knob at `qsl/qsl-client/qsc/src/fs_store/mod.rs:11`
- **The defect.** Both keychain address constants are fixed literals derived from **nothing** — not the vault path, not the envelope salt, not any per-vault value. One keychain entry therefore serves **any** vault file under **any** config directory. And `keychain_store_key` calls `entry.set_password(&enc)` **unconditionally, with no check for an existing entry**. So a second `vault init --key-source keychain` under a different config directory **overwrites the first profile's key**, rendering that vault **permanently unopenable**, with **no step in either command reporting anything wrong**. Multi-profile use is first-class here — the config-directory override is the primary config knob.
- ⚠ **EXPLICITLY NOT CLOSED BY ENG-0107, AND KEPT SEVERED FROM IT FOR THAT REASON.** Audit v2 folds F-13 into F-06 and recommends "resolve alongside." Binding the header as associated data makes key-source routing tamper-evident — which is correct **for that aspect** and is filed as part of ENG-0107. It does **nothing** for this: both profiles' headers are individually valid; **the key one of them needs is simply gone.** Folding the two would let the cheaper half close the number and leave the data-loss path open.
- **Severity deviation, recorded rather than taken silently.** The audit rates F-13 LOW on tamper-evidence grounds, which fits the half ENG-0107 absorbs. **This half is silent, permanent destruction of user data, triggered by a documented command on a first-class configuration path, with no warning and no recovery.** The house model places that at P2. Recorded so a reader comparing the two documents can see the rating was changed deliberately and why.
- **Remedy (shape only):** derive the keychain account name from a per-vault discriminator (the envelope salt is the natural candidate — already unique per vault, already on disk), and **refuse rather than overwrite** an existing entry at init. Test shape: two profiles under different config directories both stay openable; red at base.
- Cross-reference: **ENG-0107** (the tamper-evidence half), **ENG-0106** (N-03's concurrent-init race reaches the same unopenable end state by a different route), **ENG-0116** (the mode that creates this key); AUDIT-TRIAGE #001 §4.14.
- **Resolution: RESOLVED at NA-0695 (D-1335, directive D629 as amended).** The keychain account is per-vault BY CONSTRUCTION — `"vault-" + hex(envelope salt)`, raw hex, ONE derivation fn; store/load/remove take the salt and derive inside — and init REFUSES an existing entry (`ProviderError::EntryExists` → `vault_keychain_entry_exists`, fail-closed on any other read error). Measured: the banked two-profiles acceptance RED at base in the overwrite mode (profile 1 read `vault_locked` after profile 2's silent init — this entry's P2 sentence observed on the record) and GREEN after; fixed-account uses in src 5 → 0 (the const DELETED; a fixed literal survives only in `keychain_supported`'s constructor-only probe, scoped out by ruling R5); salt-fill sites 1 → 1 (the E-A stability pin). HARD BREAK (R2): no legacy `("qsc","vault")` fallback read, no cleanup; orphaned pre-D entries are recorded residue. The check-then-set window is recorded honestly (keyring has no create-if-absent; cross-profile collision requires equal 16-byte salts, 2⁻¹²⁸). Real-OS keyring plumbing recorded untestable-headless (runbook-covered); the seam-armed acceptance is a named lane gate outside CI, recorded against ENG-0112. See the D-1335 record.

### ENG-0109 — two config-directory resolvers disagree, so a blank config-dir variable puts the vault and the lock that protects it in different directories — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external verification record N-08; absent from the audit), UNRESOLVED**
- Severity: **P2** (the protection state and attempt limit stop travelling with the vault they govern)
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.**
- Exact surfaces: `qsl/qsl-client/qsc/src/vault/mod.rs` `vault_path_resolved:1210-1221` vs `qsl/qsl-client/qsc/src/fs_store/mod.rs` `config_dir:10-30`
- **The defect.** The tree has **two** config-directory resolvers and they do not agree. `config_dir()` guards **every** branch with `if !v.trim().is_empty()` and falls through to the next source when a variable is set but blank. `vault_path_resolved()` has **no such guard**, and `env::var` returns `Ok("")` for a set-but-empty variable. So with a blank (or whitespace) config-dir override: the **vault** resolves to `PathBuf::from("").join("vault.qsv")` — the **relative** path `vault.qsv`, in the process working directory — while the **lock file, protection state and store metadata** fall through to the XDG or home-based location. The same split occurs for a blank XDG variable. **The vault and the lock that is supposed to protect it end up in different directories, and the unlock counter and attempt limit no longer travel with the vault they govern.**
- **Why this is a prerequisite and not a cleanup.** `fs_store::write_atomic` and `lock_store_exclusive` both require a `ConfigSource`, which `vault_path_resolved()` does not produce. **The vault cannot reach the store primitives until this is unified**, so this lands **before ENG-0106**, not after it.
- ⚠ **UNIFYING IS A BEHAVIOUR CHANGE AND MUST NOT BE INHERITED SILENTLY.** `ConfigSource` selects the parent-safety policy: the default-home source walks **every** path component for group/world-writability, while the env-override and XDG sources check only the root. **The vault path currently gets neither.** Adopting `config_dir()`'s source means the vault inherits whichever policy matches how the operator configured the directory — correct, but a change in what gets rejected. It needs its own test, plus a blank-variable case pinning that the vault and its lock now resolve together. Red at base.
- Cross-reference: **ENG-0106** (blocked on this), **ENG-0111**; AUDIT-TRIAGE #001 §4.23.
- **Resolution: RESOLVED at NA-0692 (D-1332, directive D626; line appended 2026-08-05 by NA-0696 per D630 R5/§8.1 — the slice-family Resolution audit found A–C carried no lines).** ONE resolver owns the config directory: `fs_store::config_dir()` is the single owner (blank/whitespace variables fall through identically at every source), `vault_path_resolved` DELEGATES to it and produces the `ConfigSource` the store primitives require — the vault, the lock file, the protection state and the store metadata can no longer land in different directories. The strongest instrument was STRUCTURAL: resolver count 2,2,2 → 1,1,1 (resolvers, env-read sites, fallback chains). No open tail found in this entry. See the D-1332 record.

### ENG-0110 — the destroy path's zero-overwrite is never fsynced, so it can be a complete no-op, while its doc comment describes an ordered erase guarantee — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external audit F-08 plus one defect the audit does not list), UNRESOLVED**
- Severity: **P3** (the real erase mechanism is cryptographic and is correct; the defect is that the code documents a guarantee it does not deliver)
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.**
- Exact surfaces: `qsl/qsl-client/qsc/src/vault/protection.rs` `destroy_with_passphrase:247` — `fs::write(&vault_path, zeros)` `:274`, `fs::remove_file` `:276`, `fsync_dir_best_effort(parent)` `:278`; the doc comment `:240-246`
- **The defect.** The audit's SSD / copy-on-write analysis is correct — on any wear-levelling SSD or CoW filesystem the overwrite does not touch the physical blocks holding the old ciphertext — and its conclusion is the right posture: the real erase is **cryptographic** (zeroizing the runtime key and removing the keychain entry), and the concern is that user-facing language should not imply block-level wiping.
- ⚠ **THE UNLISTED DEFECT IS MORE BASIC, AND IT APPLIES ON EVERY FILESYSTEM.** `fs::write` does **not** `fsync`. The zeros may still sit in the page cache when `remove_file` unlinks the inode at `:276`, at which point the kernel may discard them entirely. **The overwrite can therefore be a complete no-op even on conventional media where it would otherwise work.** `fsync_dir_best_effort` at `:278` syncs the **directory**, not the zeroed file. This is independent of the SSD/CoW argument.
- **Why the comment is part of the finding, not a separate tidy-up.** The doc comment at `:240-246` describes *"zero-overwrite at recorded length THEN remove THEN fsync — the erase-then-remove ordering"*, which reads as a deliberate, ordered security property rather than a best-effort gesture. **If the overwrite is dropped, that comment must go with it; if it is kept for defence in depth, it needs a `sync_all()` before the unlink.** Either way the code and its own documentation must stop disagreeing — a no-op that documents itself as a guarantee is worse than no overwrite at all.
- ⚠ **FOLDS WITH ENG-0048 — CROSS-REPO, AND THE DESTROY BOUNDARY SHOULD BE SETTLED ONCE.** ENG-0048 (desktop ledger) records that `destroy_vault` leaves app-level `settings.json` on disk across the destroy boundary. This item (protocol ledger) records that the vault file's overwrite may not happen at all and that the documented ordering overstates what occurs. **Same question — what a destroy is claimed to erase versus what actually survives it — reached from two repositories.** Neither subsumes the other; settling them together is what makes the boundary definable, and settling either alone leaves the boundary half-defined.
- Cross-reference: **ENG-0048** (desktop side of the same boundary), **ENG-0106** (`destroy_with_passphrase` is one of the three unlocked write paths, and its `let _ = protection_state_clear_files()` at `:281` is one of the three discard sites behind the reentrancy hazard); AUDIT-TRIAGE #001 §4.8.
- **Resolution: RESOLVED at NA-0696 (D-1336, directive D630 as amended).** Destroy's erase is REAL and ordered: `zero_fill_in_place` stats the file (inode + length), opens `write(true)` with NO truncate, refuses `vault_erase_failed` on the INODE-EQUALITY PIN (a swapped file is an impostor whose zeroing would erase nothing), writes zeros over `[0, len)` on the SAME inode, `sync_all`s, THEN unlinks, then fsyncs the directory — replacing `fs::write`, whose O_TRUNC freed the original blocks first (the overwrite touched the old data on NO filesystem) with nothing ever synced. The doc comment now tells the truth: the GUARANTEED erase is cryptographic (key zeroize + keychain-entry removal); the zero pass is filesystem-level zeroization on non-CoW filesystems, explicitly NOT physical-flash (FTL) or CoW-snapshot erasure; a passphrase vault's ultimate backstop is passphrase strength plus full-disk encryption. THE DESTROY BOUNDARY is settled once (D630 A1.4/R2): vault-derived/vault-keyed artifacts DIE (`vault.qsv`, both protection-state files, the keychain entry, process key material, `send.state`, `msgqueue_v1/`, `quarantine_v1/`, `attachments/`); vault-independent config SURVIVES by design (`.qsc.lock`, `config.txt`, `store.meta`); the residue test asserts BOTH sets BY NAME as a directory-listing equality; any future satellite classifies itself against the rule in its own lane; the desktop's `settings.json` goes to the paired ENG-0048 micro-lane. Destroy also gained the keychain-vault deliberateness ceremony (`"DESTROY"` / `vault_destroy_confirm_mismatch`, `DestroyConfirmToken::confirm(typed)`) — a deliberateness guard, NOT authentication. See the D-1336 record.

### ENG-0111 — `LockContended` has no defined caller semantics, and on non-Unix the store lock is a silent no-op that reports success — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external audit F-10, Linux-relevant half), UNRESOLVED**
- Severity: **P2** (a caller that believes it holds an exclusive lock may hold nothing, with no error)
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.** The Windows-implementation half is **deliberately deferred** — see the scope note.
- Exact surfaces: `qsl/qsl-client/qsc/src/model/mod.rs` — `LockGuard::lock` `#[cfg(unix)]` `:62`, `LOCK_NB` `:68`, the `flock` call `:73`, the `LockContended` return `:77`, `Drop` `#[cfg(unix)]` `:87`, the marker string `:33`; `qsl/qsl-client/qsc/src/fs_store/mod.rs` — the gated calls `:349-350` and `:377-378`
- **The defect, in two parts.**
  - **(a) The silent no-op.** `LockGuard::lock` exists **only** under `#[cfg(unix)]` and has **no counterpart of any kind**. Callers gate the call, so on a non-Unix target `lock_store_exclusive` opens the lock file, **skips the lock entirely**, and returns `Ok(LockGuard { file })`. **A caller that believes it holds an exclusive lock holds nothing, and receives no error.** ⚠ **The defect is the silent success, not the missing platform** — that distinction decides the fix.
  - **(b) Undefined contention semantics.** The lock is taken with `LOCK_NB`, so contention returns immediately as `LockContended`, surfaced as a bare marker string with no retry policy and no operator guidance — **indistinguishable, to a user, from a wrong passphrase.** What a caller should do on contention (retry with backoff, or surface as an error) is not defined anywhere, and ENG-0106 is about to make callers that must answer the question.
- ⚠ **2026-08-04, BINDING REQUIREMENT ADDED BY NA-0693 (D-1333; Director ruling at NA-0693 STOP 005 — the destination lane MUST take this decision, not merely consider it):** ENG-0106's implementation found the lock-nesting class to be a **structural fragility** — three serial discoveries in one slice of code paths that reach a vault write while the store lock is already held (the transport outbox helpers, the transport timeline ingest, and ENG-0118's latent session-store-key creation arm), each self-denying because `flock` attaches to the open file description. Slice B treated them with per-site `_locked` inner-variants (`protection_state_clear_files_locked`, `secret_set_locked`, the timeline locked chain), which handles one site at a time and leaves discovery to a mechanical per-region sweep. **This item's destination lane must decide, first-class: does the store lock become REENTRANT (per-process path-keyed nesting at the `LockGuard`/`fs_store` layer), eliminating the class outright? If adopted, RETIRE the per-site `_locked` variants in favor of it. ENG-0118 FOLDS INTO this decision — the reentrant lock is its honest fix too.** The banked NA-0693 reentrant-lock fallback is hereby deferred to this item with its own design attention, not dropped. Evidence: the NA-0693 STOP-004/005 stop-files (the bidirectional and mechanical sweeps, every site bracketed).
- **Also adopted from the audit:** `flock` is advisory — it excludes only other `flock` callers, not a path that simply opens and writes — and it is unreliable on NFS and on some overlay/container filesystems. That is directly relevant to the self-hosted deployment target, where the config directory may sit on NFS and containerized runs on overlayfs. **Operator-facing documentation, not a code change.**
- ⚠ **SCOPE NOTE — THE WINDOWS IMPLEMENTATION IS NOT IN THIS ITEM, AND THE AUDIT'S ARGUMENT FOR IT DOES NOT HOLD.** Audit v2 argues Windows is a supported target *"given the keychain features include a Windows-native backend."* **The premise is true** — the keyring dependency does carry a Windows-native backend, because that crate ships Apple, Windows and Secret Service backends together and taking the dependency enables all three regardless of intent. **The inference is false.** Measured at `bd4f2a3a`: **`docs/design/DOC-QSC-008` states in as many words that "Linux and macOS are in scope; Windows, mobile, and browser delivery are out of scope"**, and places Windows support outside the prototype boundary unless a later lane explicitly promotes it; there is **no Windows CI job in any of the twelve workflows**; and the only non-Unix code is two no-op stubs (`fs_store:438`, `:446`). ⚠ **Note also that the external verification record rebuts the audit from ABSENCE ("no mention of Windows in the docs"), and that rebuttal is itself wrong — 34 files under `docs/` mention Windows.** The correct argument is the **affirmative scope statement**, not an absence. **Implementing a real Windows lock for a platform that is neither built nor tested is speculative; making the non-Unix path fail closed is not.** If Windows is ever promoted, **it needs a CI job before it needs a lock implementation.**
- Cross-reference: **ENG-0106** (which will be the first code to hold this lock across a read-modify-write, and whose reentrancy hazard makes (b) urgent), **ENG-0109**; AUDIT-TRIAGE #001 §4.10/§4.11/§6.2.
- **Resolution: RESOLVED at NA-0696 (D-1336, directive D630 as amended).** The 2026-08-04 BINDING requirement is DISCHARGED first-class: the store lock is REENTRANT — a per-process path-keyed depth registry at the `LockGuard` layer (`thread_local! RefCell<HashMap<PathBuf, Held>>`, three-phase borrow discipline binding, panic-safe by construction, cross-thread fail-closed) — and the ENTIRE per-site regime RETIRED (the four `_locked` variants, the U3 pre-lock ensure, the `_with_save` indirection; every former site proven inside a held lock by the bidirectional sweep). Nesting rules empirical on two kernels: EX-under-EX / SH-under-EX / SH-under-SH grant (a grant may exceed the requested strength, never fall below); EX-under-SH refuses fail-closed with the minted `lock_upgrade_refused` (a forwarded same-fd conversion measurably LOSES the held lock — demonstrated in-tree by Control U). (b) THE CONTENTION CONTRACT is defined in D-1336: every acquisition is LOCK_NB, nothing blocks, `lock_contended` surfaces and is NEVER retried by the library — retry is a UI decision; and the advisory-flock caveat rides here as adopted: the store lock is advisory `flock` — it excludes only other flock callers, not a rogue open-and-write — and flock is unreliable on NFS and some overlay/container filesystems; place the config dir on a local filesystem (the standalone operator one-pager is recorded OWED to the operator-docs track, D630 A1.6). (a) Non-Unix now FAILS AT COMPILE TIME (`compile_error!` at the lock layer; the two no-op stubs and all eight lock-claim cfg masks deleted) — the compile gate is STRUCTURAL-ONLY this lane (the cross-check dies in ring's mingw cc before qsc compiles; upgrade path: install `gcc-mingw-w64-x86-64`, then a future micro-lane runs the live gate). The Windows implementation is explicitly out-of-item per this entry's own scope note. See the D-1336 record.

### ENG-0112 — the relay-TLS trust family does not run on pull requests, so the property the audit calls exemplary has no PR-time protection; and the DH-guard scan's own comment understates a control that does run — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external audit F-INFO + verification record N-01), UNRESOLVED**
- Severity: **P2** (a guard that does not run on the path where regressions actually arrive)
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.**
- Exact surfaces: `.github/workflows/ci.yml` — the `ci-4a` job `:349`, its three named qsc tests `:364-366`, the refimpl invocation `:369`, and **`qsc_linux_full_suite`'s `if: github.event_name != 'pull_request'` at `:374`**; the unprotected family `qsl/qsl-client/qsc/tests/NA_0663_relay_tls_trust.rs` (`family4_no_certificate_bypass_exists_in_source_or_tests:640`, `family4_every_trust_knob_misset_still_refuses_an_untrusted_certificate:663`); `qsl/qsl-client/qsc/Cargo.toml:20`, `:27`; the stale comment `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:3622-3625`
- **The defect.** `NA_0663_relay_tls_trust.rs` is a `-p qsc` integration test, and the job that runs the full qsc suite is gated off pull requests. **On every pull request the certificate-bypass needle scan, the fail-closed proof, and the whole CA-handling family are skipped.** A pull request could reintroduce an accept-invalid-certificates escape hatch and **no required check would fail.** The property the audit singles out as *"the single line most worth protecting in future refactors"* has **no PR-time protection at all**.
- ⚠ **THIS CORRECTS THE AUDIT'S PREMISE.** F-INFO credits the needle scan with guarding the source and asks only for a manifest assertion. There are **two** gaps, not one: the missing manifest assertion **and** the source scan not being on the PR critical path.
- ⚠ **THE EXCLUSION IS DELIBERATE, AND THIS ITEM MUST BE READ THAT WAY OR THE FIX WILL BE WRONG.** `ci.yml:344-346` records the intent in the tree: *"the broad full-suite qsc lane moves to a non-critical-path job outside pull_request gating"*, against a measured multi-hour runtime (ENG-0052). **This is a considered trade with an unintended casualty, not an oversight.** The remedy is therefore to bring the small, bounded trust family back onto the PR path — it uses runtime-generated certificates against a loopback listener and reaches no external host, so its cost is bounded — **not** to reverse the trade and put the whole suite back on PRs.
- **Independent corroboration, from the opposite direction:** NA-0689 recorded the same gap while shipping — its qsc suite never ran in CI because `qsc-linux-full-suite` and the macOS equivalent both skip on pull requests, leaving a local run as the only full-suite evidence. **Two unrelated routes to the same finding is why the general question matters more than the single test:** ⚠ **which OTHER security-property tests are full-suite-only and therefore PR-unprotected?** That audit is the larger half of this item and should be done while the workflow is open. The TLS family is unlikely to be the only casualty.
- **Also folded here (N-01) — a stale comment that UNDERSTATES a live control.** The NA-0628 anti-regression scan carries a note reading *"KNOWN LIMITATION … no CI job runs `cargo test -p quantumshield_refimpl`, so this scan guards the lane gate and local runs, NOT pull requests."* **`ci.yml:369` runs exactly that command inside the required `ci-4a` job**, attributed in an adjacent comment to ENG-0019 / NA-0630. The gap was closed and the comment recording it was never updated. Worth correcting because a reviewer reading only the source would conclude the scan is unenforced — and might build redundant machinery, or treat the scan as removable. ⚠ **Correct the comment; do not weaken the scan.**
- **Gate shape (for the destination lane):** temporarily add a bypass needle to a scratch source file and confirm a **pull-request-context** run fails. **It must fail to fail at base — that is the finding.**
- ⚠ **THE PR-GATING HALF IS DEFERRED — annotated 2026-08-03 by NA-0691 (D625 §4.2). DEFERRED IS NOT RESOLVED: this entry STAYS OPEN and receives NO `Resolution:` line** (partial-closure rule, this ledger's header; precedent at ENG-0091). **Deferred to a CONTRIBUTOR / PRE-RELEASE trigger.** The reason, recorded rather than just the state: **a PR-time guard protects against contributors, and there are none** — the operator is the sole author — **and the full suite still runs on every push**, so the property is exercised continuously, just not at PR time. **The deferral is about TIMING, not about whether the gap is real.**
- ⚠ **The entry's own measured claims are NOT weakened by the deferral and all stand:** `ci.yml:374`'s `if: github.event_name != 'pull_request'`; the unprotected `NA_0663_relay_tls_trust.rs` family; and the larger open question — ***"which OTHER security-property tests are full-suite-only and therefore PR-unprotected?"*** — which remains the bigger half of this item and is **not** deferred by this annotation. ⚠ **The trigger to re-read this: the first outside contributor, or pre-release readiness, whichever comes first.**
- ⚠ **N-01 MOVES TO THE CRYPTO-HYGIENE LANE — recorded 2026-08-03 by NA-0691 (D625 §4.3), and this is now this entry's ONE LIVE CLEANUP once the PR-gating half is deferred.** N-01 is the NA-0628 scan's stale *"does not run on PRs"* comment at `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:3622-3625`, folded into this entry above. It joins **ENG-0115** — which corrects the same scan's self-description **from the other side** — in the forward order's crypto-hygiene item (ENG-0113 / ENG-0114 / ENG-0115 + the ENG-0055 addendum). ⚠ **ENG-0115 already carries the pairing note** (*"this item and ENG-0112 touch the same scan from opposite sides … Neither should silently rewrite what the other depends on"*); **this recording makes the two ends AGREE and does not re-litigate either.** ⚠ **No lane is promoted by this recording.**
- ⚠ **ADVISORY PR-TIME COVERAGE LANDED — recorded 2026-08-06 by NA-0698 (D-1338, directive D632).** The `qsc-sharded-suite` workflow runs the full hermetic 130-target qsc suite — the `NA_0663_relay_tls_trust.rs` family included — on every pull request as a NON-REQUIRED check (12 standard-runner shards, manifest census + by-name reconciliation). The entry's larger question is answered by the manifest census: every full-suite-only binary is now PR-covered at the advisory level. **The REQUIRED-level half stays deferred to the contributor/pre-release trigger; promotion to required is the operator's branch-protection act. This entry stays OPEN.**
- Cross-reference: **ENG-0052** (the push-only full-suite pattern this is an instance of), **ENG-0049** (the TLS trust work this family protects), **ENG-0115** (which corrects the same NA-0628 scan's self-description from the other side), ENG-0019 / NA-0628 / NA-0630; AUDIT-TRIAGE #001 §4.15/§4.16; **NA-0691 / D625 §4.2 + §4.3** (the deferral and the N-01 fold).

### ENG-0113 — `Aead::seal` signals failure with an empty `Vec`, an in-band sentinel inside a crypto boundary's normal output domain — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external audit F-02), UNRESOLVED**
- Severity: **P3** (fail-closed today at every site; the defect is that the invariant rests on caller discipline rather than on the type)
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.**
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs:23` (the trait declares `-> Vec<u8>`); `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs:149-150` (`unwrap_or_default()`); the seven protocol call sites `suite2/ratchet.rs:1047`, `:1359`, `:1679`, `:1810`, `:1970` and `qsp/ratchet.rs:351`, `:358`
- **The defect.** `Aead::seal` returns `Vec<u8>` and, on an internal encryption error, returns an **empty vector**. Callers check `is_empty()` and treat it as failure, so **the system is fail-closed today at all seven sites — verified, not assumed.** Encoding a cryptographic failure as a value inside the operation's normal output domain is nonetheless fragile: the guarantee is a convention every future caller must know rather than something the type enforces. `open()`, directly beside it in the same trait, already returns `Result`.
- ⚠ **ONE OF THE AUDIT'S TWO JUSTIFICATIONS DOES NOT HOLD, AND IT MUST NOT BECOME A TEST.** Audit v2 says the sentinel could bite via a caller that *"legitimately encrypts an empty payload."* **That case is not reachable:** the AEAD is AES-256-GCM, which appends a 16-byte authentication tag, so sealing an empty plaintext returns a 16-byte ciphertext and **never** an empty `Vec`. `is_empty()` is unambiguous for every input under the current primitive. **A regression test for the empty-payload case could never fail, and a test that cannot fail is not a test** — recorded explicitly so nobody writes one and reads its green as evidence. The collision would only become possible under a tagless primitive, which is not a realistic direction. **The audit's other justification — a future caller forgetting the check — is real, and is the reason to act.**
- **Remedy (shape only):** return `Result<Vec<u8>, CryptoError>` and propagate; the compiler then enumerates the call sites rather than a reviewer doing it.
- Cross-reference: **ENG-0115** (the same "move the guarantee into the type" shape, and the same crate); AUDIT-TRIAGE #001 §4.2.

### ENG-0114 — the wire body length is cast to `u16` with no send-side guard, so a body of 64 KiB or more truncates silently — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external audit F-03), UNRESOLVED**
- Severity: **P3** (self-inflicted message loss; fail-closed at the receiver, not exploitable)
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.**
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:1061-1062` (inline in `send_wire`), `:1271-1272` (`frame_suite2_wire`), `:1585-1586` (`frame_pq_wire`) — each `(… .len() as u16).to_be_bytes()`
- **The defect.** The wire encoder writes the header and body ciphertext lengths as `u16`; a body of 64 KiB or larger **truncates silently**. The receiver's total-length equality check rejects the resulting frame, so this is **fail-closed rather than exploitable** — an oversized frame fails to decode rather than decoding into something unintended.
- **The cost, stated precisely.** The sender advances its send chain before framing, so the peer receives a **gap**, and the skipped-message mechanism handles gaps by deriving and storing a key for the missing counter. **The true cost is one permanently lost message plus one orphaned skipped-key entry — not a session desync.** Recorded because an earlier characterization overstated this as leaving "a chain the peer cannot follow", and a lane planned against that would scope the wrong fix.
- **The precedent already exists — in ANOTHER CRATE.** The invite path pre-checks `> u16::MAX as usize` before framing, at `qsl/qsl-client/qsc/src/invite/mod.rs:231`, `:336`, `:466-468` (and again at `:728-729`, more sites than either source document cites). ⚠ **That module is qsc-side; the refimpl has no `invite` module at all** — a lane told to use "the `invite/mod.rs` idiom" must not go looking for it beside the framing sites. **The fix is a consistency change, not a new pattern.**
- **Remedy (shape only):** an explicit plaintext/ciphertext size check before framing at the three sites, returning a clean typed rejection so an oversized message fails legibly at the sender. Test shape: a 65536-byte plaintext rejects rather than truncating.
- Cross-reference: **ENG-0113**; AUDIT-TRIAGE #001 §4.3.

### ENG-0115 — the X25519 zero-shared-secret check lives at call sites rather than in the primitive; the existing anti-regression scan already prevents the failure mode the audit describes — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external audit F-04, with its stated risk corrected), UNRESOLVED**
- Severity: **P3** (defense-in-depth consolidation — ⚠ **NOT a gap-closing fix**; see the correction)
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.**
- Exact surfaces: `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs:177-182` (`X25519Dh::dh` returns a raw `[u8; 32]`); the NA-0628 scan in `suite2/ratchet.rs` — `ALLOWED_UNGUARDED_DH:3635`, `PINNED_DH_SITE_COUNTS:3705`, the non-shadowing test `:3595`; `.github/workflows/ci.yml:369`
- **The defect as it actually stands.** `dh()` returns the raw shared secret, and the all-zero (small-subgroup) result is checked at each call boundary rather than inside the primitive. Centralizing it would make the property unforgettable by construction, and the underlying library clamps scalars, so this is belt-and-suspenders.
- ⚠ **THE AUDIT'S STATED RISK DOES NOT HOLD, AND THE CORRECTION IS THE POINT OF THIS FILING.** Audit v2 warns that *"a future call site can omit it."* **A future call site cannot silently omit it.** The NA-0628 anti-regression scan requires **every** `.dh(` site in the repository to be either followed by a fail-closed all-zero check on the DH **output**, or listed in an allowlist **with a written reason**, with **per-file site counts pinned** so drift in **either** direction fails. It is documented as mutation-proved against three synthetic failure modes, and **it runs on every code pull request** via the required `ci-4a` job. **Treat the scan as an asset, not as absent** — the value of centralizing is simplification, not closing a hole.
- ⚠ **BINDING CONSTRAINTS ON ANY FIX — both are load-bearing and either one silently weakens the guard if missed.** (1) Centralizing **must** update the allowlist and the pinned site counts **in the same commit**, or the scan correctly fails. (2) It **must** preserve the distinction pinned at `:3595`: the `dh_pub` **encoding** check catches **one** of eight low-order points, while the **output** check catches **all eight**. Collapsing the two — an easy and natural-looking simplification — would reduce coverage eightfold while leaving a test named as though it still held.
- **Note for the destination lane:** this item and **ENG-0112** touch the same scan from opposite sides — this one relies on its coverage, that one corrects its stale self-description. Neither should silently rewrite what the other depends on.
- Cross-reference: **ENG-0112**, **ENG-0113**, ENG-0019 / NA-0628 / NA-0630; AUDIT-TRIAGE #001 §4.4.

### ENG-0116 — keychain mode has no passphrase factor, and it is assigned by daemon availability rather than chosen by the operator — **NEW; filed 2026-08-02 by AUDIT-TRIAGE #001 (external audit F-12 plus the amplifier the audit does not state), UNRESOLVED**
- Severity: **P2** (a materially different security posture selected without the operator's knowledge)
- Status: open — filed 2026-08-02. **FILING ONLY; nothing executed.** ⚠ **Filed as a CODE item by ruling, not as documentation — see the amplifier. The FIX FORM is deliberately left open.**
- Exact surfaces: `qsl/qsl-client/qsc/src/vault/mod.rs` — `derive_key:1145` (the keychain arm), `resolve_key_source:879-903`, `keychain_supported()`, `derive_runtime_key:808`
- **The defect.** In keychain mode there is **no passphrase**. `derive_key`'s keychain arm generates a random 32-byte key (`OsRng.fill_bytes`) which is stored in the OS keychain, and the vault is encrypted directly under it. **The Argon2id passphrase KDF is never invoked.** The confidentiality of a keychain-mode vault therefore reduces entirely to the OS keychain's access control over a single entry: in passphrase mode an attacker with the vault file still needs the passphrase and pays the Argon2id cost per guess; **in keychain mode, anything that can read that one entry obtains the full key immediately — no guessing, no KDF.** On many Linux desktop configurations that means any process running as the user once the session is unlocked.
- ⚠ **THE AMPLIFIER, WHICH IS WHY THIS IS A CODE ITEM AND NOT A DOCUMENTATION ONE.** Audit v2 frames this throughout as a **chosen** posture — *"an operator choosing it for a shared or multi-user machine must understand…"*, *"so it is a chosen posture rather than a surprise."* **It is not chosen.** `resolve_key_source`'s no-argument branch selects keychain mode whenever `keychain_supported()` returns true, and `keychain_supported()` is a **probe for whether a keychain entry handle can be constructed** — i.e. **availability, not intent**. An operator running `vault init` with no explicit key-source argument, on any machine with a working Secret Service, macOS Keychain or Windows Credential Manager, gets the **no-passphrase** mode **without asking and without being told**. Two users on identical hardware — one with a keyring daemon running — get materially different security properties from the same command. **Documentation cannot make a default a choice.**
- ⚠ **OPEN DECISION, STATED RATHER THAN RESOLVED — the fix FORM is a first-run UX decision and is not this filing's call.** The minimum is that `vault init` **prints the selected key source and what it implies, and records the choice**; the stronger form **requires an explicit key-source argument**. The two differ in how much friction lands on first run, which is a product decision. **What is settled by this filing is that documentation alone is insufficient; what is open is which form the code takes.** Destination spans a code fix plus a docs / threat-model rider; lane placement belongs to the directive.
- **On the audit's optional hybrid mode** (deriving the file key from `Argon2id(passphrase)` combined with the keychain random key, so an attacker needs both factors): **recorded as the audit's suggestion, not endorsed here.** It reintroduces the passphrase prompt that keychain mode exists to avoid, which is a product decision rather than a security defect to be closed.
- Cross-reference: **ENG-0108** (the same keychain entry, addressed by fixed constants and overwritten unconditionally), **ENG-0107** (the `key_source` byte that selects this path is itself unauthenticated, and N-06's KDF check is skipped for exactly this mode), **ENG-0055** (this key's load path leaves two unzeroized heap copies), **ENG-0053** (whose measurements are stated for passphrase vaults only and explicitly do not generalize here); AUDIT-TRIAGE #001 §4.13.

---

### ENG-0117 — a peer-supplied `msg_id` of whitespace passes the transport's `!is_empty()` guard and fails the timeline's `trim().is_empty()` guard, forcing a store failure for that message — **NEW; filed 2026-08-02 by NA-0690 (D-1329) as FILING ONLY, UNRESOLVED**
- Severity: **P3** (a nonconformant peer can deny *individual messages* a timeline row; no key material, no confidentiality impact, no mailbox-wide effect)
- Status: **OPEN. FILING ONLY — NA-0690 deliberately changed NOTHING here.** ⚠ **Routed to a future GUARD-CONSISTENCY lane.** It is outside D624 §7 and was found by that lane's Phase-0 injection probe, not by looking for it.
- Exact surfaces: `qsl/qsl-client/qsc/src/transport/mod.rs:1025` and `:1077-1081`; `qsl/qsl-client/qsc/src/timeline/mod.rs:492-496`.
- **The defect — two guards on the same value, spelled differently in two files.** `receive_pull_and_write` decides whether a `msg_id` exists with `if !request_msg_id.is_empty()`, and passes it down as `forced_id`. `timeline_append_entry_for_target` then rejects with `state_id_invalid` when `v.trim().is_empty()`. **A `msg_id` of `" "` satisfies the first and fails the second**, so `timeline_append_entry` fails deterministically for that message while everything around it succeeds. The `msg_id` is peer-supplied — it arrives inside the data control envelope — so the failing value is chosen by the sender, not by us.
- ⚠ **NA-0690's FIX CHANGES THIS FINDING'S BLAST RADIUS, WHICH IS WHY IT IS FILED ALONGSIDE D-1329 RATHER THAN LATER.** Before D-1329 such a message was **silently acked away and lost** (the store failed, the ack fired anyway). After D-1329 it is **not acked**, so it is redelivered and reported. The posture is strictly better — a witnessed loss instead of a silent one — but it is **louder**, and a reader meeting the new behaviour without this entry would reasonably suspect a regression in the fix rather than a pre-existing input-validation gap.
- ⚠ **THE CONSEQUENCE IS BOUNDED TO ONE REDELIVERY CYCLE, NOT A PERMANENT WEDGE — and the distinction matters because the loud version is the one people over-estimate.** `commit_unpack_state` consumes the ratchet key **above** the store, so the redelivered envelope cannot decrypt again: it hits NA-0644's replay backstop, is quarantined by NA-0689, and **is** acked. Per affected message that is one extra delivery cycle and one quarantine entry — **not** an unbounded loop, and **not** a mailbox stall, since a failed store is logged rather than propagated and the remaining items in the batch process normally. ⚠ **Labelled honestly: this bound is INFERRED from the identical mechanism, measured directly for the sibling case** (NA-0690's negative-control arm induces the same `timeline_append_entry` failure by a different cause and observes exactly this sequence). **The whitespace input itself was NOT executed** — it was found by reading the two guards, and no test in the tree exercises it.
- **What is lost is the same thing ENG-0104 was about:** the timeline row — the record that the message exists in the conversation. The payload bytes are written before the store, and the plaintext is unrecoverable either way once the key is consumed.
- ⚠ **DO NOT ASSUME THE FIX IS "TRIM IN THE TRANSPORT".** Which of the two guards is wrong is a real question: making the transport `trim()` too would silently accept a whitespace id as *absent* (changing dedup and receipt behaviour for that message), whereas rejecting the envelope outright at parse time is a wire-conformance decision. **The guard-consistency lane owns that choice; this entry only records that the two guards disagree.** A sweep for other `is_empty()`/`trim().is_empty()` pairs across the same boundary belongs with it.
- Cross-reference: **ENG-0104** (the ack-gating defect at the same site, fixed by NA-0690/D-1329 — this entry is the input-validation gap that fix makes visible), **D-1329** (which references this entry by number), NA-0690 STOP #004 §5d (the probe that found it, with the two guard lines quoted).

### ENG-0118 — the QSP session-store key's create-arm under a held store lock: test-reachable self-denial (now pre-lock-provisioned by NA-0693/U3) plus the pre-existing concurrent-first-sender create race — **NEW; filed 2026-08-04 by NA-0693 (D-1333, STOP-004 ruling), CORRECTED same-lane at STOP 006, UNRESOLVED (narrowed)**
- Severity: **P3 as found** (11 base-green tests fail-closed); **P4 as shipped** (NA-0693/U3 pre-lock-provisions the key on both locked paths; what remains is the narrow create race below)
- Status: **OPEN, NARROWED.** Routed to **Slice E / ENG-0111's now-BINDING reentrancy decision** (see the 2026-08-04 requirement recorded there), which owns both remaining halves.
- Exact surfaces: `qsl/qsl-client/qsc/src/protocol_state/mod.rs:191` (`qsp_session_store_key_get_or_create` → `vault::secret_set`, the create arm only — the module's SOLE vault write), reached from the two locked transport transactions via `qsp_session_store` (`send_abort`, `finalize_send_commit`); since NA-0693/U3, both transactions call `qsp_session_store_key_ensure()` BEFORE taking the lock.
- **The defect as FOUND (STOP 006 measurement):** the create arm writes through the LOCKING `vault::secret_set`; under the transaction's held lock it self-denies (`lock_contended` lands in the `Err(_)` arm — the seed-fallback arms match only `vault_missing`/`vault_locked`) → `IdentitySecretUnavailable` → `qsp_session_store_failed`, the send failing closed AFTER relay acceptance. ⚠ **THE ORIGINAL FILING CALLED THIS "latent/pathological" AND WAS WRONG — corrected by measurement:** `attachment_streaming_na0197c` carries **11 base-green tests** (`attachment_e2e_resume_and_peer_confirm_after_persistence`, `attachment_fetch_capability_and_enc_ctx_reject_without_persistence`, `explicit_override_wins…`, `explicit_send_attachment_service_override…`, `legacy_path_roundtrip…`, `legacy_sized_w2_roundtrip…`, `post_w0_legacy_receive_retirement…`, `validated_post_w0_receive_defaults…`, `validated_post_w0_receive_rejects_explicit_coexistence_mode`, `w2_legacy_sized_selection…`, `w2_threshold_boundary…`) whose seeded sessions reach a **locked file-send as the channel's first session persist** (`msg_idx=0`) — no prior unlocked handshake had created the key. The production-flow half of the original inference (a real handshake persists sessions unlocked first) may hold, but **the suite's contract does not distinguish seeded from real callers** — the STOP-004 prediction "no existing test changes outcome" was falsified by these 11.
- **What NA-0693/U3 closes and what it deliberately does NOT:** the pre-lock `qsp_session_store_key_ensure()` guarantees the create arm never runs under a held lock on the two known transactions — the 11 tests return green. ⚠ **NOT closed: the concurrent-first-sender create race** — two processes both finding the key absent, each generating a different key, last-writer-wins, the loser's session blobs undecryptable. This race is PRE-EXISTING (today's unlocked create has the same get-generate-set shape) and U3 neither widens nor narrows it; **an atomic create or the reentrant store lock closes it**, and that belongs to ENG-0111/Slice E.
- Related observation from the same census (labelled observation, pre-existing): plain message sends (`relay_send:1746`) take **no store lock** while file sends (`relay_send_with_payload:3110`) and aborts (`send_abort:79`) do — the outbox files are written under inconsistent locking discipline across the two send paths.
- Cross-reference: **ENG-0106** (the parent fix; D-1333, D627 + Amendments 1-2), **ENG-0111** (lock semantics / retry / non-Unix — ⚠ **this entry FOLDS INTO ENG-0111's now-BINDING reentrancy decision**, added 2026-08-04 by the same ruling: if the store lock becomes reentrant, this corner's fix falls out of it), NA-0693 STOP-004/005 stop-files (the bidirectional and mechanical sweeps, every site bracketed).
- **Resolution: RESOLVED at NA-0696 (D-1336, directive D630 as amended).** Both remaining halves close by the reentrancy decision this entry folded into: `qsp_session_store_key_get_or_create` wraps its WHOLE get-generate-set in the store EX lock (D1(b)) — nested legally under a transport transaction via the reentrant registry, a real serializing flock otherwise — so the concurrent-first-sender create race closes BY LOCK EXCLUSIVITY (two processes serialize; the loser re-reads the winner's key), and the U3 pre-lock ensure is RETIRED (the create-arm can no longer self-deny under ANY held lock, by construction rather than by provisioning). The related census observation closes too: D1(c) makes the plain-send drain commit ONE locked transaction, ending the send-path locking asymmetry. The 11 named `attachment_streaming_na0197c` tests measured green on the fixed tree (targeted run and the full suite), and red EXACTLY as the committed set under Control R (the reentrancy mechanism reverted) — the closure is load-bearing, not incidental. See the D-1336 record.

### ENG-0119 — qsl-desktop `erase_all_impl` leaves `settings.json.tmp` across the erase boundary when a crash lands inside the settings write window — **NEW; filed 2026-08-06 by NA-0697 (D-1337; D631 Amendment 1 ruling R3(b)) — FILING-ONLY**
- Severity: P4 (hygiene; NO secret involved — the `.tmp` is a staging copy of the non-secret settings.json, present on disk only after a crash between the write and the rename)
- Status: open — filed 2026-08-06 by NA-0697 (D-1337; directive D631 A1.3, ruled FILE-don't-fold)
- Exact surfaces: qsl-desktop `src-tauri/src/settings.rs:57-64` (`save` writes `settings.json.tmp` at :59-:61 then renames at :62 — atomic on the happy path, but a crash between :61's write and :62's rename leaves the `.tmp` sibling) vs `src-tauri/src/commands.rs::erase_all_impl` (removes the qsc dir AND `settings_file(data_dir)` — never the `.tmp` sibling)
- Description: found by the NA-0697/D631 formalization's writer census (D631 §3b observation; ruled R3 at STOP 002). DESTROY now removes BOTH names (D-1337/D-0024 — the removal loop covers `settings.json` and `settings.json.tmp`, and the residue test's listing-equality guards the pair by construction); ERASE removes only `settings.json`, so a crash-window `.tmp` survives erase — the same boundary-inconsistency class as ENG-0048, one level down, reachable only through the crash window. No test pins the erase side's `.tmp`.
- Remedy: one `remove_file` of the `.tmp` sibling in `erase_all_impl`, mirroring destroy's removal loop — a deliberate micro-change since it alters what erase DOES; belongs to a future desktop hygiene lane. The fix is explicitly NOT NA-0697's (SR-02 bars silent scope growth; the edit-set extension that authorized THIS filing authorized exactly the filing).
### ENG-0120 — `qsc_linux_full_suite` has grown to ~220 min against its 240-minute timeout: a measured margin of ~1.09×, down from the ~1.53× the timeout was set for — **NEW; filed 2026-08-06 by NA-0698 (D-1338; directive D632 Amendment 1 A1.12, Director ruling R9(b)) — FILING-ONLY**
- Severity: **P2** (an existing REQUIRED-adjacent job approaching failure, independent of this lane; when it trips, the job dies at the timeout with no suite result at all)
- Status: open — filed 2026-08-06 by NA-0698. **FILING ONLY; `ci.yml` was not touched by this lane.**
- Exact surfaces: `.github/workflows/ci.yml` — the `qsc_linux_full_suite` job's `timeout-minutes: 240` and the adjacent comment still reading *"Measured 157m08s at b2dc23bf; 240 is 1.53x that"*
- **The measurement.** Four consecutive runs of that job: **219.7 / 220.1 / 220.4 / 221.6 min** (spread 1.9 min — stable growth, not a spike). Separated from the runner's own job log for run 31058005573: build phase ≈ **3.0 min**, test phase ≈ **216.3 min**. Against the 240-minute timeout that is a margin of **~1.09×**.
- **The defect is the stale margin, not the runtime.** The timeout was chosen at 1.53× a 157-minute suite. The suite has since grown ~40% while the timeout did not move, so the headroom that made the timeout a *bounded failure* rather than a *likely failure* has quietly been spent. The in-file comment still asserts the original ratio, so a reader checking the margin gets the wrong answer from the tree itself.
- **Why it is filed and not fixed here.** `.github/workflows/ci.yml` is outside NA-0698's authorized edit set (SR-02), and this lane deliberately touches no existing workflow. Raising a timeout is also the least interesting of the available remedies and would spend the finding.
- **Remedy options for the destination lane, none chosen here:** raise `timeout-minutes` with a stated new ratio and correct the comment in the same commit; or retire the monolith in favour of the sharded workflow once `qsc-sharded-suite` has stability data and the operator has promoted it to required (D-1338 keeps the monolith untouched precisely so that decision stays open); or shard the monolith the way this lane shards the PR path.
- Cross-reference: **ENG-0052** (the original measurement and the push-only full-suite trade), **ENG-0112** (the PR-time coverage half, addressed at the advisory level by this same lane), **D-1338 / NA-0698**, D632 A1.11–A1.12.


### WF-0044 — `preflight_governance.sh` and `post_merge_verify.sh` count READY lanes with an UNANCHORED grep, so prose quoting the status line inflates the count; and the same script's `--require-clean` gate runs BEFORE the queue count, making that check unreachable exactly when a lane wants it — **NEW; filed 2026-08-03 by NA-0691 (D625 §4.7) as FILING ONLY, UNRESOLVED**
- Severity: **P2** (process/gate integrity; NO runtime impact — but a required-by-convention governance preflight is RED on `main` for a reason unrelated to any lane, and a gate that cannot pass teaches bypass)
- Status: open — filed 2026-08-03. **FILING ONLY; nothing in `scripts/ci/**` was touched.**
- Exact surfaces: `scripts/ci/preflight_governance.sh:39` and `scripts/ci/post_merge_verify.sh:106`, both counting with `rg -n 'Status:\s*READY'` — **unanchored**; the anchored parser that actually gates, `scripts/ci/qsl_evidence_helper.py:231` (`^\s*-?\s*Status:\s*([A-Z_]+)\b`); the hit that trips it, `NEXT_ACTIONS.md:41`, prose inside an immutable `<!-- prior: … -->` comment written by NA-0690's promotion; and the rule already forbidding the pattern, `NEXT_ACTIONS.md:401-406`.
- **Defect 1 — the unanchored count.** Measured at `4bd987a2` **before any NA-0691 edit**: `bash scripts/ci/preflight_governance.sh` → **exit 1, `FAIL: READY_COUNT=2 (>1)`**, the second "READY lane" being **prose** in a `prior:` comment that quotes the status line while describing a *retirement*. Re-measured at `d804d455` (NA-0691's base): **still exit 1, still `READY_COUNT=2`**, hits at `:41` and `:36899`. ⚠ **TWO COMMITTED INSTRUMENTS DISAGREE ABOUT ONE QUESTION ON ONE TREE:** `python3 scripts/ci/qsl_evidence_helper.py queue` reports **`READY_COUNT 1`, exit 0** on the same tree, and it is the one `qwork` uses. **The anchored count is the correct one.**
- ⚠ **THE TREE ALREADY FORBIDS THIS PATTERN BY NAME, AND CITES WHERE IT WAS LEARNED.** `NEXT_ACTIONS.md:401-406`: *"Tooling MUST match the status line ANCHORED to the start of a line (`^Status:` followed by the state), never as an unanchored substring search … (Learned at NA-0624 …)"*. **Two committed CI scripts still do it.** A rule that is written down and not enforced by the instruments it governs is the failure family ENG-0077/ENG-0078 belong to.
- **Defect 2 — the ordering, folded in by Director ruling because it is the same script and the same filing.** `hygiene_sentinel --require-clean` runs **before** the queue count and exits first, so **the queue check is unreachable on any tree with uncommitted work** — i.e. at exactly the moment a lane wants to verify the queue before committing. ⚠ **This is not a second opinion about defect 1; it is why defect 1 stayed invisible.** *Inference, labelled as inference:* anyone running the preflight mid-edit got `FAIL: --require-clean set and working tree is dirty` and moved on, so the READY count was rarely reached on a dirty tree and never questioned on a clean one.
- ⚠ **HOW IT WAS FOUND, AND WHY THE METHOD IS PART OF THE FILING.** NA-0691's promotion predicted exit 1 and **got exit 1 — for the wrong reason**: the log said `--require-clean … working tree is dirty`, so the run proved nothing about the count it was claiming about. The impl pass re-ran it on a clean tree and saw `hygiene_sentinel … OK` followed by the genuine `FAIL: READY_COUNT=2 (>1)`. **The exit code was identical in both cases and distinguished nothing** — which is the evidence that produced DOC-OPS-006 §4a rule 5 (*read the log, not the check mark*), recorded by the same lane.
- **Recommended change (NOT made here):** anchor both counts to `^Status:`, matching `qsl_evidence_helper.py:231`; and move the queue count **ahead of** or independent of the `--require-clean` gate so a lane can verify the queue pre-commit. **Proof gap: nothing asserts that the two instruments agree** — a test that constructs a tree with the literal in prose and requires both counters to return the same number would have caught this at NA-0624.
- ⚠ **WHY THIS IS FILING-ONLY.** The fix is a `scripts/ci/**` change — a **workflow path** (`classify_ci_scope.sh` returns `workflow_security=true` for it), which would change the scope class and character of the governance-only lane that found it. ⚠ **And the red does NOT clear when NA-0691 merges**, because the offending hit is prose already on `main` inside an immutable `prior:` comment: **the expected count after NA-0691 is still 2.** A lane meeting this red for the first time mid-flight will believe it caused it. **Precedent for a filing riding a lane by ruling: NA-0690's ENG-0117.**
- Cross-reference: **WF-0025** (a gate that cannot pass teaches bypass — the same dynamic, and the reason this is filed rather than tolerated); **WF-0026** (the other known parser blind spot); NA-0624 (where the anchoring rule was learned); **DOC-OPS-006 §4a.3** (queue verification via the helper, never a `STATE:` grep) and **§4a rule 5** (read the log, not the check mark — established by this lane on this script's evidence); **D-1330 / NA-0691** (the lane that measured it); ENG-0077 / ENG-0078 (instruments that do not instrument).

### WF-0045 — `preflight_qsc_impl.sh` cannot pass at base on either of its two static gates: `cargo fmt --check` is RED across 236 files and `clippy -- -D warnings` would fail on 53 warnings, and NO workflow invokes the script — **NEW; filed 2026-08-03 by NA-0692 (D626; Director ruling at STOP_NA0692_003) as FILING ONLY, UNRESOLVED**
- Severity: **P2** (process/gate integrity; **NO runtime impact** — but the repo's own preflight for `qsc` implementation lanes is unpassable on `main` for reasons unrelated to any lane, and a gate that cannot pass teaches bypass)
- Status: open — filed 2026-08-03. **FILING ONLY; nothing in `scripts/ci/**` was touched, and `cargo fmt` was never run to write.**
- Exact surfaces, all measured at `250c2b71` (the merge of PR #1692) **before any NA-0692 edit existed**: `scripts/ci/preflight_qsc_impl.sh:39` (`cargo fmt -p qsc -- --check`), `:40` (`cargo test -p qsc --locked`), `:41` (`cargo clippy -p qsc --all-targets -- -D warnings`).
- **Defect 1 — the fmt gate is RED tree-wide.** `cargo fmt -p qsc -- --check` → **exit 1, 236 files, 3419 diff lines** (rustfmt 1.9.0-stable; **no `rustfmt.toml` anywhere**, so these are default settings). The deviations are ordinary and mechanical — over-long call expressions and import lists — not a formatting dispute. ⚠ **`src/vault/mod.rs` alone carries 8 pre-existing hunks** (`:407, :414, :421, :508, :518, :525, :539, :594`), every one an over-long `fail_core_buffers(...)` call.
- **Defect 2 — the clippy gate would fail.** `cargo clippy -p qsc --all-targets --locked` (run **without** `-D warnings`, so the census is complete rather than truncated at the first abort — D-1328 R14/R15) → **exit 0 but 53 lint warnings, 0 errors**: 19 redundant-closure, 17 needless-borrow, 6 unneeded-`return`, 4 doc-list-indent, 2 `Result<_, ()>`, 2 too-many-arguments, 1 unit-`let`, 1 borrowed-expression, 1 manual-`is_multiple_of`. **With `:41`'s `-D warnings`, all 53 become errors and the script exits non-zero.**
- ⚠ **THE THIRD GATE PASSES, AND THAT IS THE POINT.** `:40`'s `cargo test -p qsc --locked` returns **582 passed / 0 failed / 2 ignored across 125 binaries, exit 0** at the same commit. **The script is not uniformly stale — its dynamic gate is healthy and both of its static gates are not**, which is what distinguishes drift from abandonment.
- ⚠ **NOTHING INVOKES THE SCRIPT, AND NOTHING ELSE ENFORCES ITS TWO STATIC GATES EITHER.** `clippy`, `cargo fmt` and `rustfmt` appear **nowhere in `.github/`** — not in any of the twelve workflows. Tree-wide, the only references to `preflight_qsc_impl.sh` are its own banner echo at `:19` and its creation record at `TRACEABILITY.md:979` (**NA-0139, tooling-only, PR #352**). **So no CI signal exists for either gate, and the one place they are written down is never run.**
- ⚠ ***Inference, labelled as inference:*** this has the shape of **a gate that was written, never wired, and then drifted until it was unpassable** — the tree grew 236 files of formatting deviation and 53 lint warnings with no instrument objecting. **The alternative reading is that the two static gates were deliberately advisory from the start**, and the creation record at `TRACEABILITY.md:979` does not settle which. **The measurement below is fact; the diagnosis is not.**
- ⚠ **HOW IT WAS FOUND, AND WHY THE METHOD IS PART OF THE FILING.** NA-0692 wrote *"exit 0, empty output"* as its expected fmt result **before running it**, per D626 §11's requirement that every named gate be run once at base before it is trusted as a delta. **The expectation was falsified on the first run.** Had the lane taken the fmt baseline only *after* editing — or compared totals without a baseline at all — the 236 files would have looked like this lane's doing, or gone unnoticed entirely. **The gate that found this is "run it at base first", and it is the same gate that found WF-0044.**
- ⚠ **A SECOND-ORDER HAZARD THIS CREATES FOR ANY FUTURE `qsc` LANE, stated because it nearly bit NA-0692.** With the gate red at base, "is fmt clean?" is unanswerable and the only meaningful question becomes **"did I add deviations?"** — a **file-set delta**, not a total. NA-0692's own first post-change measurement was **238 files (+2)**; both additions were its own new lines and were reformatted individually. ⚠ **The tempting repair — `cargo fmt` — rewrites 236 files and is a severe scope violation for any lane.** A lane that reaches for it to "clear the red" will produce an enormous unrelated diff and will believe it is being tidy.
- **Recommended change (NOT made here):** decide the two static gates' status explicitly rather than by neglect — **either** wire `cargo fmt --check` and `clippy -D warnings` into CI and pay down the 236 files / 53 warnings in a dedicated lane whose scope class is honest about touching every file, **or** demote them in the script to advisory (printed, not enforced) so the preflight can pass and stops teaching bypass. **Proof gap: nothing asserts that a script under `scripts/ci/**` is reachable from any workflow** — a check that every such script is either invoked or explicitly marked local-only would have caught this at NA-0139, and would also have caught WF-0044's sibling ordering defect.
- ⚠ **WHY THIS IS FILING-ONLY.** The fix is a `scripts/ci/**` change — a **workflow path** (`classify_ci_scope.sh` returns `workflow_security=true` for it) — which would change the scope class of a vault-integrity implementation lane; and the alternative fix touches 236 source files. **Both are categorically outside D626 §6.** ⚠ **The red does NOT clear when NA-0692 merges**: the condition is pre-existing on `main` and this lane's file-set delta is **zero** (236 → 236, identical set by `diff`; clippy 53 → 53, identical distribution). **A lane meeting this red for the first time mid-flight will believe it caused it** — which is precisely why it is written down here.
- Cross-reference: **WF-0044** (the same pattern on a sibling script in the same directory, filed one lane earlier — *a governance gate written, never enforced, drifted red*; **this filing is that pattern's second instance, and two instances in two lanes is the reason the Recommended-change proof gap is stated as a class check rather than a one-off fix**); **WF-0025** (a gate that cannot pass teaches bypass); **ENG-0077 / ENG-0078** (instruments that do not instrument); **D-1328 R14/R15** (a `-D warnings` total is a census of whatever cargo finished before it gave up — the reason Defect 2's distribution was taken without `-D warnings`); **D-1332 / NA-0692** (the lane that measured it); **NA-0139 / PR #352** (where the script was introduced). **Precedent for a filing riding an implementation lane by Director ruling: NA-0690's ENG-0117, and WF-0044 itself.**

### WF-0046 — `qsl-server`'s tests spawn the server BINARY as a child process and reap it with a straight-line `child.kill()` that any earlier panic skips; there is NOT ONE `impl Drop` in the suite, so a failing test orphans a live listener — **41 leaked processes observed on the build box, the oldest up 4d23h. NEW; filed 2026-08-03 at operator instruction as FILING ONLY, UNRESOLVED**
- Severity: **P2** (test-harness hygiene / build-host resource leak; **NO product runtime impact** — the leak is confined to test-spawned processes on the build machine, and no shipped code path is involved)
- Status: open — filed 2026-08-03. **FILING ONLY; no test, source or script in any repo was touched.**
- ⚠ **THE DEFECT IS IN `qsl-server`, NOT `qsl-protocol`.** Filed here because this ledger is the **cross-repo** findings register — precedent: **ENG-0014**, **ENG-0091** and **ENG-0092** are all `qsl-server` defects filed in this file, and **WF-0023** spans `qsc` ↔ `qsl-server` ↔ `qsl-attachments`.
- Exact surfaces, measured in the `qsl-server` checkout at `1b6df984`: **five** test files spawn the binary via `Command::new(env!("CARGO_BIN_EXE_qsl-server"))` — `tests/config_semantics.rs:28`, `tests/na0642_durability_restart.rs:56`, `tests/na0652_server_info.rs:258`, `tests/na0678_invite_durability.rs:81`, `tests/hardening_auth_reject_logging.rs:465`.
- **The mechanism, stated precisely.** Reaping is a **straight-line statement placed after the assertions**: `config_semantics.rs:88-89` is `let _ = child.kill(); let _ = child.wait();` at the END of the helper — and the loop **above** it contains `panic!("config expected to start but exited with {status}")` at `:81`. **Any panic between spawn and that line skips the reap and orphans a live server.** `na0642_durability_restart.rs:97` and `na0652_server_info.rs:297` reap through a **consuming method** (`fn hard_kill(mut self)`), which has the same property: it is an explicit call, not a destructor, so it does not run on the failure path.
- ⚠ **`grep -rn 'impl Drop' tests/` OVER THE WHOLE `qsl-server` SUITE RETURNS NOTHING.** There is no destructor-based guard anywhere — **every** reap in the suite is skippable. This is the whole finding: the cleanup was written, and it was written in the one place that does not run when the test fails.
- ⚠ **WHAT WAS ACTUALLY OBSERVED, AND ITS LIMIT.** On 2026-08-03 the build box carried **41** live `/srv/qbuild/cache/targets/qsl-server/…/debug/qsl-server` processes, the oldest at **4 d 23 h**, PIDs spread across a wide range (so: accumulated over many runs, not one bad session), **each holding a distinct `127.0.0.1` ephemeral listening socket** (`:36599`, `:36501`, `:35597`, …). All **41 exited cleanly on SIGTERM** — they were idle and waiting, not wedged, which is consistent with orphaning rather than hanging. ⚠ **THE PROCESSES WERE REAPED BEFORE THIS ENTRY WAS WRITTEN, SO THAT RUNTIME EVIDENCE CANNOT NOW BE RE-MEASURED.** The counts above are recorded as observed; everything about the *code* is re-checkable at `1b6df984`.
- ⚠ **A CORRECTION THAT IS PART OF THE FILING, BECAUSE THE FIRST DIAGNOSIS WAS WRONG.** The leak was initially attributed to `qsl-protocol`'s `qsc` test suite, on the assumption that its relay tests spawn a server per case. **They do not.** `qsc`'s `tests/common/mod.rs` runs its mock relays **in-process on a thread** (`start_qsl_server`, the bind at `:479`/`:529`), and its three `impl Drop` blocks (`:332`, `:368`, `:451`) **join threads — they never spawn or kill an OS process.** `grep -rn 'Command::new' qsl/qsl-client/qsc/tests/` finds **no** `qsl-server` spawn at all. **The wrong repo and the wrong mechanism were both asserted before either was measured**, and the error was caught only by looking for the reaper and finding it joined a thread. *A `Drop` impl named after a server is not evidence that a server process exists.*
- **Recommended change (NOT made here):** give each spawned child an RAII guard — a small wrapper struct owning the `Child` with `impl Drop { let _ = self.0.kill(); let _ = self.0.wait(); }` — and spawn only through it, so the reap runs on the panic path as well as the success path. **Proof gap: nothing fails when a test leaks a process.** A harness assertion that the process table holds no `qsl-server` children at suite exit, or a CI step that counts strays after `cargo test`, would have surfaced this on the first failing run instead of after five days of accumulation.
- ⚠ **WHY IT WENT UNNOTICED FOR DAYS, and why that is the interesting part.** Each test binds an **ephemeral** port, so 41 stale listeners never collided with a new run — **the leak is invisible to the thing it would otherwise break.** *Inference, labelled as inference:* the accumulation rate suggests it is driven by the **failing** runs specifically, which is exactly when nobody is looking at process hygiene. **Nothing in this entry claims a specific test as the culprit** — the observed processes carried no argv distinguishing their spawn site, and they are now gone.
- Cross-reference: **WF-0044** and **WF-0045** (the two open members of the same family — *cleanup or a gate that exists but is never reached*; WF-0045's static gates are never invoked, this one's reaper is never run on the path that needs it); **ENG-0091** (`qsl-server` test-harness synchronisation defect — same suite, same class of "the test's own scaffolding is the defect"); **ENG-0092** (`qsl-server` CI runs `cargo test -q`, so totals are not evidence the binaries ran — **the reason a leak like this leaves no CI trace**); **WF-0023** (cross-repo real-stack coverage).

### WF-0047 — CodeQL's `rust/hard-coded-cryptographic-value` query files CRITICAL false positives against the vault crypto path's zero-init-then-fill buffer idiom, respawns on every line-shift or new dataflow path, and blocks merges through the REQUIRED `CodeQL` gate — **NEW; filed 2026-08-04 at Director ruling (NA-0694 STOP 005) as FILING ONLY, UNRESOLVED**
- Severity: **P2** (CI/gate integrity; **NO runtime impact** — every flagged site independently verified correct by the Director and by the STOP-005 diagnosis — but a REQUIRED gate that goes red on false positives blocks every merge that shifts these lines, costs a diagnosis cycle each recurrence, and teaches bypass)
- Status: open — filed 2026-08-04. **FILING ONLY; no alert was dismissed by the filer, no code or config was touched.** The four PR-#1698 dismissals are a recorded operator act per the Director's STOP-005 ruling — through CodeQL's own dismissal mechanism with per-alert justifications, **NOT** an admin/branch-protection override.
- Exact surfaces, measured at PR #1698's head `688124d0` (alerts) and quoted from the tree: **alert #110** `qsl/qsl-client/qsc/src/vault/mod.rs:540` (`vault_init_core`'s `let mut salt = [0u8; 16];` — unconditionally overwritten by `rand_core::OsRng.fill_bytes` at `:546`, or early-return under the RNG test seam; open on `main` since **2026-06-09** at the pre-NA-0694 anchor `:484`) · **#111** `src/adversarial/vault_format.rs:69` (the parser's salt buffer — `copy_from_slice` from the envelope being parsed one line later; a decrypt-side salt IS file data) · **#112** `vault/mod.rs:562` (init's nonce buffer — OsRng-filled at `:568`) · **#113** `vault/mod.rs:1002` (`encode_envelope`'s conversion buffer — `copy_from_slice` from the caller's freshly-generated CSPRNG nonce). All four: rule `rust/hard-coded-cryptographic-value`, severity critical.
- **The mechanism, stated precisely.** CodeQL's Rust taint model treats a `[0u8; N]` array literal as a hard-coded cryptographic value and **does not kill that taint when the buffer is overwritten through `&mut`** (`fill_bytes`, `copy_from_slice`). Consequently **any new dataflow path from such a buffer mints a NEW critical alert although no value changed**: #111/#112/#113 were created by PR #1698's own analysis (2026-08-04T23:12:30Z) because NA-0694's AAD builder (`envelope_header_bytes(…, salt: &[u8; 16], nonce: &[u8; 12])`) added paths from three long-existing, correctly-filled buffers — #111's ×4 flows are exactly the four new AAD consumers of the parsed salt, #112's ×2 the two builder calls. The AAD binding STRENGTHENS the envelope; the flagged buffers carry exactly the bytes they always carried.
- ⚠ **THE RESURRECTION RECORD — the class is a survivor, not an incident.** Alert **#50** (old `vault.rs:379`, salt) and **#61** (old `vault.rs:187`, nonce) read "fixed" — closed when the file was split. **#76** (`vault/mod.rs:394`, salt, created 2026-02-23) has `fixed_at` equal to **#110's `created_at` TO THE SECOND** (2026-06-09T04:58:15Z — the NA-0456/0457 RNG-seam merges moved the line). **The pattern has been continuously flagged since February under four successive alert numbers; every "fixed" state was a relocation, none a remediation.** A location-tracked alert respawns on refactor; a dismissal without a durable fix will meet the class again at the next line-shift.
- **Operational bite, measured:** `CodeQL` is in branch protection's required contexts (verified via the API); PR #1698 sat `mergeStateStatus: BLOCKED` on the four false positives while all three `Analyze` jobs were green — the 2s check-run is the alert-count gate, not the analysis.
- **Recommended change (NOT made here) — the durable fix, an operator-decided choice for a future authorized micro-lane (it must NOT ride NA-0694; D628 §0a.(3) spent that lane's only post-PR commit):** **either** (a) retire the literal at the four sites — array-from-slice `try_into` for the parser and `encode_envelope`, a seam-compatible `random_array::<N>()` helper for init's salt/nonce — so no `[0u8; N]` remains on a crypto path (efficacy plausible-not-certain against the model; a draft PR would show the verdict cheaply since CodeQL runs on PRs), **or** (b) a CodeQL query-filter/suppression config for this query over these idioms — a workflow-path, security-posture choice. **This item belongs alongside the ENG-0112 / CI-migration cluster — the CI configuration is due a pass anyway.**
- **Proof gap: nothing distinguishes a relocated false positive from a NEW real alert at merge time** — each recurrence presents as a fresh critical security finding on a blocked PR and costs a full diagnosis cycle (this instance cost NA-0694 a stop). A recorded disposition per site (the dismissal justifications) plus the durable fix closes the loop; the dismissals alone do not.
- Cross-reference: **ENG-0112** (required/full-suite gates that skip or block other than as intended — the CI-migration cluster this joins); **WF-0025** (a gate that cannot pass teaches bypass); **WF-0044 / WF-0045** (the gate-integrity family — an instrument whose red does not mean what it says); **NA-0694 / D-1334** (the lane whose AAD builder surfaced the class); `STOP_NA0694_005_20260804T235518Z.md` (the diagnosis of record, with per-alert justification texts).

### WF-0048 — the method constitution is repo truth: docs/ops/STANDING_RULES.md + docs/ops/PREDICTION_LEDGER.md — **NEW; filed 2026-08-05 by the post-NA-0696 governance errand (operator adoption 2026-08-05)**
the method constitution is repo truth: docs/ops/STANDING_RULES.md (SR-01..13 BINDING
consolidations; SR-14 R-BANK / SR-15 adversarial second read / SR-16 prediction ledger
ACTIVATED by this merge; SR-17 tiered ceremony pending SR-16 data; SR-18 observable-remap
census effective D631) + docs/ops/PREDICTION_LEDGER.md (rows through NA-0696). Adoption
ladder recorded in the file's §C: gate-manifest micro-lane → control harness → GUI
evidence tooling (folds into the input-driver lane intent) → CI-migration (queued). Repo
copy is canonical from this merge; the handoff-packet copies mirror it. Origin records:
NA-0696 STOPs 004 (R-BANK), 006/007 (SR-18); operator adoption 2026-08-05.

### ENG-0121 — qsl-desktop registers two commands (`marker_stats`, `core_busy`) that no frontend call site invokes: dead surface or a missing FE feature, undecided — **NEW; filed 2026-08-07 by NA-0700 (D-1340; D634 A2-FINAL §3 item 6 / R117(a)) — FILING-ONLY**
- Severity: P4 (surface hygiene; no runtime defect — the commands work when invoked, as the NA-0700 IPC replay harness now proves registration-level)
- Exact surfaces: qsl-desktop `src-tauri/src/commands.rs::marker_stats`/`::core_busy`; the `generate_handler!` registration (now in `configure_builder`); `ui/main.js` — zero invoke sites for either name (measured grep exit 1, confirmed independently by two seats at bcb1dc1a).
- Description: both commands are REGISTERED AND DORMANT — `marker_stats` reports the marker buffer's `(buffered, dropped)` and `core_busy` the gateway's in-flight flag, but the FE's busy indicator is FE-local (`pendingCalls`) and nothing reads marker stats. Either the surface is dead and should be retired deliberately, or a debug/diagnostics pane is missing and should be built deliberately. Nobody has decided (R117(a): "a fact about the product nobody has decided").
- Status: open — filed 2026-08-07 by NA-0700. FILING ONLY; nothing resolved in that lane (the replay harness exercises both registration-level, which is coverage, not a decision).
- Originating/last lane: NA-0700 (D-1340; D634 A2-FINAL).
- Last-updated: 2026-08-07.

### WF-0049 — operator build-root paths appear throughout public governance records — a pre-existing, immutable-by-house-law exposure class awaiting a deliberate assessment errand — **NEW; filed 2026-08-07 by NA-0700 (D-1340; D634 A2-FINAL §3 item 6 / R124, R128) — FILING-ONLY**
- Severity: P3 (information hygiene, not access: the strings carry no IP, hostname, username, or secret — they expose build-root LAYOUT of operator infrastructure in a public AGPL repo, against the standing prohibition on publishing operator infrastructure details)
- Measured extent AT FILING (needle + scope stated per row, each its own run over tracked files at the NA-0700 impl tree): `/srv/qbuild` — **4,264 occurrences across 603 files**; `/home/` — **1,309 occurrences across 238 files** (this row's needle is WIDER than the class — it also matches legitimate non-operator content such as generic Unix examples; the row is an upper bound, stated as such); `/tmp/claude` — **0 occurrences** (the empty result is the row's finding).
- ⚠ **RULED EXCEPTION (R159, NA-0700 STOP 008 §13): this entry's class-description tokens are a DELIBERATE exception to the standing build-root needle rows** — a filing whose SUBJECT is the needle class must name its tokens (bare roots only; no host, user, or layout beyond them; the three carrying lines enumerated in the stop; zero hits in source, tests, or any executable text). A future sweep must NOT "fix" these tokens by obfuscation: that would make this record unusable for the very errand it commissions. Per R159(b) the standing rows are re-specified from NA-0700 forward: MEASURE the hits, ENUMERATE every hit, CLASSIFY each — ZERO UNCLASSIFIED is the gate; an unclassifiable hit is the STOP.
- Description: R123 (NA-0700 promotion) held a merge over ONE such path and ruled the durable citation form for public repo truth: FILENAME + sha256, never an operator directory path. The historical population is pre-existing across immutable records (DECISIONS.md, NEXT_ACTIONS.md blocks, evidence files) and is NOT to be edited retroactively (house law: history is never rewritten). What is owed is a deliberate sweep-and-assess errand: classify the population, decide whether any class warrants redaction-by-supersession, and add the three build-root needles to the standing infra-literal scan set if ruled in (they ran as MANUAL rows at NA-0700's promotion and impl per R123).
- Status: open — filed 2026-08-07 by NA-0700. FILING ONLY; no historical line edited, nothing resolved.
- Originating/last lane: NA-0700 (D-1340; R123/R124/R128).
- Last-updated: 2026-08-07.

### ENG-0122 — the redaction discipline is SHAPE-BASED and contact aliases are SEMANTICALLY sensitive and SHAPE-INNOCUOUS: the two do not meet — **NEW; filed 2026-08-07 by NA-0700 (D-1340; D634 A2-FINAL AM-9 item 3 / R151, in the ruling's words) — FILING-ONLY**
- Severity: P3 (threat-model exposure class in shareable debug artifacts; no secret leaks)
- The finding, in the ruling's words (R151): the redaction discipline is SHAPE-BASED — length, digits, URL-form, timestamp-form — and therefore catches values that LOOK like secrets. A contact alias is SEMANTICALLY sensitive in this product's threat model (it names who a user talks to, for users who are lawyers, clinicians and activists) and SHAPE-INNOCUOUS. The two do not meet.
- Measured extent: the `peer` field carries the user-chosen alias VERBATIM at **10 of 14 CLI named-marker labels** (QSC_CONTACT_FLOW, QSC_CONTACT_REQUEST, QSC_TRUST_PROMOTION, QSC_TRUST_REMEDIATION, QSC_SEND_BLOCKED, QSC_ROUTING, QSC_RECEIPT, QSC_DELIVERY, QSC_RECEIPT_IGNORED, QSC_FILE_DELIVERY) and the `thread` field is the same `short_peer_marker` class at **all 7 TUI sites** — on BOTH the formatted (`QSC_MARK/1` kv) and raw named-marker paths (`short_peer_marker` truncates only hex-shaped ≥32-char inputs; `should_redact_value("peer", "mom")` is false on every path). Short petnames already flow into the queue's existing formatted vocabulary under the program's recorded judgment; NA-0700's redact-on-queue closed only the DELTA its own lane would otherwise have created (the high-cardinality/URL/timestamp-shaped alias class).
- Why it is out of NA-0700 (recorded per R151): putting `peer` on the key denylist would change `QSC_MARK` bytes on stdout, against that lane's byte-identity claim — the fix belongs to a lane that can move that claim.
- Status: open — filed 2026-08-07 by NA-0700. FILING ONLY; nothing resolved.
- Originating/last lane: NA-0700 (D-1340; R144/R148/R150/R151).
- Last-updated: 2026-08-07.

### WF-0050 — qsl-desktop's `public-safety` context is no longer in branch protection's required set: a required context silently dropped, dating instrument = the operator's audit log — **NEW; filed 2026-08-08 by NA-0700 (D-1340; RBANK_NA0700_012 ruling R167(c)) — FILING-ONLY**
- Severity: P2 (gate integrity; NO current failure — the job still runs and passes on every PR, and the safe-direction reading is on the record: nothing this lane relies on became less gated)
- Measured current REQUIRED set, live at the server (`branches/main/protection/required_status_checks/contexts`, 2026-08-08): **{rust, advisories, infra-literal-scan}** — 3 contexts. The prose it contradicts: qsl-desktop `ci.yml`'s own recorded comment (and STOP-001 §5.5's census carrying it, and D634 §6's enumeration built on that census) said **{advisories, public-safety, rust}** required with `infra-literal-scan` advisory.
- The two deltas, ruled at R167: `infra-literal-scan` REQUIRED — **confirmed deliberate by the Director** (R167(b): the correct posture for a public repo). `public-safety` NOT required — **NOT confirmed** (R167(c)): it was required at the NA-0653 lesson ("kept per the NA-0653 lesson", the census's own words) and something removed it. Whether that was a deliberate operator act or drift cannot be dated from a seat — **the operator's audit-log check is the only instrument that can date a branch-protection change**, and this entry names it for exactly that check.
- Root-cause class (R167, in the ruling's words): a FILE claim (the ci.yml comment) carried through a census as if it were a server measurement — the enumeration-is-not-the-record lesson, fourth instance. The measured set supersedes the prose per SR-09; RBANK_NA0700_012 R167(a) is the superseding record.
- Status: open — filed 2026-08-08 by NA-0700. FILING ONLY; branch protection is operator-only and nothing was changed by this lane (measured: the lane's PRs touch no CI file and mint no context; all four desktop jobs ran and passed on PR #25 regardless of the required subset).
- Originating/last lane: NA-0700 (D-1340; R167).
- Last-updated: 2026-08-08.

### ENG-0123 — scr-erase error writes skip the R-14 window resize: after a wrong ceremony phrase, both ceremony buttons' click centers fall outside the card clip — **NEW; filed 2026-08-08 by NA-0701 (D-1341; R170 §2, in the ruling's words) — FILING-ONLY**
- Severity: P3 (usability/interactability defect on the app's most stressful screen; no data loss — the scroll affordance exists but is undiscoverable; severity assigned at filing by the seat, labeled as such)
- The finding, in the ruling's words (R170 §2): scr-erase error writes skip the R-14 window resize: after a wrong ceremony phrase, #btn-erase and #btn-erase-cancel click centers fall outside the card clip (overflow:auto) — WebDriver measures element-not-interactable; a live user must discover the card's internal scrollbar to proceed OR cancel on the app's most stressful screen. Found by the NA-0701 GUI driver (STOP_NA0701_008; runs 090807Z red F-E, 090927Z dev repro, + the elementFromPoint probe). Remedy candidate, not this lane: extend the unlock-feedback structural pattern (ONE resizing writer, design_polish-asserted) to #erase-error. Fix-lane acceptance INCLUDES restoring the in-place wrong→correct click as a test row — the row NA-0701 could not keep.
- Mechanism (measured, STOP_NA0701_008 §4): the window height is synced at `show("scr-erase")` with the error line absent; the :519–523 error write calls no `syncWindowHeight` (the complete site list `:92/:245/:333/:370/:375/:409/:644 + def :75` contains nothing in the erase handler, Director-verified); +19px of error line pushes the button row past the card's clip; `elementFromPoint` at the click center returns `scr-erase`. R-14 class, 4th occurrence, FIRST MACHINE CATCH — main.js:395–404's own comment names "the error path" as a member of this class and fences the structural remedy to `#unlock-feedback` alone.
- Status: open — filed 2026-08-08 by NA-0701. FILING ONLY; nothing fixed (D636 §7: findings filed, not fixed). F-E runs the R170 Option-1 third-launch shape until the fix-lane lands.
- Originating/last lane: NA-0701 (D-1341; R170).
- Last-updated: 2026-08-08.
- **Resolution: RESOLVED at NA-0702 (D-1342, directive D637 as amended — A1).** The structural remedy applied exactly as this entry's own candidate named: `setEraseError(text)` is the ONE RESIZING WRITER of `#erase-error` (TOTAL — null-guard retained, resize unconditional), all four base reference sites absorbed (:488 entry-clear · :515–:516 abort-clear · :520–:523 handler clear + wrong-phrase write · :549 catch write; comment-stripped count 4→1, `syncWindowHeight` call sites 7→8), pinned by `design_polish.rs::erase_error_has_exactly_one_writer_and_it_resizes` on the unlock-feedback template (helper exists · writes AND resizes in one call · reference count == 1 · the empty-element HTML pin). The fix-lane acceptance was DISCHARGED IN FULL: **the restored in-place wrong→correct click is a committed F-E row again** (leg C deliberately re-enters the error state; leg B gains the REAL Cancel click at the error state — the driver's own in-view-centre refusal, now required to pass), and the corrected scenario was proven by the THREE-POINT ordering proof (A1.2): RED against the unfixed app at exactly the committed pair {leg-B Cancel click, leg-C in-place click} both rc=2 `element not interactable`; RED again under the C2 regression (the resize line deleted) at the SAME pair row-for-row; GREEN with the fix via the real consumers (gui driver 6/6; desktop full bare suite 113 names 106/0/7, inventory 112→113 BY NAME). Countdown code BYTE-UNTOUCHED — the FORK B measurement ruled it out of this lane's scope; its near-miss is FILED as ENG-0124 below WITH THE NUMBERS. See D-1342, desktop D-0027.

### ENG-0124 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0124`.

### ENG-0125 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0125`.

### ENG-0126 — `unlock_guarded_at`'s `is_ok()` collapse counts EVERY non-passphrase error as a failed unlock attempt, and at an armed limit wipes the vault — **NEW; filed 2026-08-09 by NA-0705 (D-1344; R185 §2.4 / R187 §3 F-2 / Q8.5, ordered) — FILING-ONLY**

`qsl/qsl-client/qsc/src/vault/protection.rs:156` is `if unlock_with_passphrase(passphrase).is_ok()`, one branch, so the error's NAME is discarded; `protection.rs:175` then increments `failed_unlocks` unconditionally and `protection.rs:180` calls `wipe_vault_file_best_effort()` at an armed limit. D628 Ruling A required a recognized-but-old envelope to refuse with its own name at both parse sites "never read as corrupt or wrong-passphrase" — that intent is defeated one layer above where it was enforced. The sibling case was already fixed the right way: NA-0696/D630 split keychain load errors three ways so a missing entry no longer reads as a wrong passphrase; the VERSION refusal happens earlier, at parse, and never reached that split. ⚠ **The instance is already two instances** — `vault_version_unsupported` today, and every future non-passphrase error class (`vault_attempt_limit_io`, lock contention, keychain absence) tomorrow. NA-0705 fixed the SHIPPING CONSUMER only (desktop pre-flight, both doors); **the CLI remains exposed**, and the class fix belongs here. The narrow option was taken for lane tractability, not sufficiency.

### ENG-0127 — the three production-compiled test-seam environment variables should be evaluated as a CLASS for release-build gating — **NEW; filed 2026-08-09 by NA-0705 (D-1344; SR-15 F-3 / Q8.4, ordered) — FILING-ONLY**

`qsl/qsl-client/qsc/src/clock/mod.rs:49` declares `CLOCK_OVERRIDE_ENV = "QSC_UNSAFE_TEST_CLOCK_UNIX_S"` with **no `cfg(test)` and no feature gate** on the public path (`clock/mod.rs:58`), and `parse_override` panics by design on a malformed value (`clock/mod.rs:89`). It is honored by production builds and now backs `vault::protection::now_unix_s`, hence both `unlock_guarded` and `protection_status` — the unlock lockout schedule. It is the THIRD such variable in the crate (`QSC_UNSAFE_TEST_SEED_FALLBACK`, `QSC_RNG_FAILURE_TEST_SEAM`, and this one). Each is individually reasoned in its own source; nobody has asked whether the SET should be gated off release builds. ⚠ Threat modelling is explicitly NOT ordered by this filing and the reach judgment stays open and stated: reachability and crash behaviour are established, who can realistically set the variable on a target's session is not.

### ENG-0128 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0128`.

### ENG-0129 — `ServerInfoDoc` gained three invite limits the desktop Server pane does not surface — **NEW; filed 2026-08-09 by NA-0705 (D-1344; SR-15 F-4) — FILING-ONLY**

At `32e572c7` the doc carries `invite_max_expiry_secs`, `invite_max_slots`, `max_invite_bundle_bytes`; the desktop's `ServerInfoDocDto` renders 10 of the relay's 13 advertised fields. Nothing breaks — every wire field carries `#[serde(default)]`, so an older relay's document still parses and still reaches `Reachable`, verified specifically because a required new wire field would have converted a working relay into a `NotAQslRelay` verdict. NA-0705 corrected the now-inaccurate "1:1 rendering" comment in `src-tauri/src/commands.rs` and deliberately did NOT add the fields: surfacing them is Server-pane design work.

### ENG-0130 — qsc's panic-redaction hook makes EVERY desktop test's assertion failure unreadable — **NEW; filed 2026-08-09 by NA-0705 (D-1344; R190 §4.1, ordered) — FILING-ONLY**

`qsl-desktop/src-tauri/src/lib.rs:309` (`bootstrap`) installs `qsc::output::install_panic_redaction_hook()`. Correct in the shipped app; in a test binary it rewrites every panic — including a `assert_eq!` failure — to `QSC_MARK/1 event=panic code=panic_redacted`. **Every desktop test that replicates `bootstrap()` inherits this**, so a readable red becomes an unreadable one silently. Observed live: NA-0705's first instrument run reported three failures with no message, and the values that make the finding (`left: 1, right: 0`) were invisible until the hook was dropped. Workaround the seat used, and the candidate remedy: `let _ = std::panic::take_hook();` immediately after `bootstrap()` in test fixtures. `qsl-desktop/src-tauri/tests/na0700_ipc_replay.rs` already avoids the hook deliberately and says why — that reasoning should be a shared fixture, not a per-file rediscovery.

### ENG-0131 — the desktop test suite's shared `env_lock` poisons on the first panic and kills later tests before their assertions run — **NEW; filed 2026-08-09 by NA-0705 (D-1344; R190 §4.2, ordered) — FILING-ONLY**

`qsl-desktop/src-tauri/tests/slice_a_flows.rs:18-21` defines `env_lock()` and every test takes it with `.lock().unwrap()`. When one test panics while holding it, the mutex is POISONED and every subsequent test dies on `PoisonError` **before reaching its own assertions** — a red set polluted by an unrelated cause, which is precisely what makes a control untrustworthy. Observed live in NA-0705: two of the three expected reds reported `PoisonError` instead of their real failures until the idiom was changed. Remedy, already used by qsc itself (e.g. `qsl/qsl-client/qsc/src/output/mod.rs`'s queue access and the desktop's own `src-tauri/src/markers.rs:34`): `.lock().unwrap_or_else(|p| p.into_inner())`. NA-0705 applied it in its own new instrument only; `slice_a_flows.rs` still carries the fragile form and was correctly out of that lane's scope.

### ENG-0132 — `RIG_BRINGUP_RUNBOOK.md` omits the route-token length rule — **NEW; filed 2026-08-09 by NA-0705 (D-1344; R191 §4.3, ordered) — FILING-ONLY**

The runbook's client recipe covers `relay ca-set --path`, `vault init --passphrase-file` vs `--unlock-passphrase-file`, and the `chmod 700` requirement on a setgid work root — but not the route token, which `qsl/qsl-client/qsc/src/adversarial/route.rs:21` requires to be **22–128 characters** of `[A-Za-z0-9_-]`. NA-0705's first pair used an 18-character token and was correctly refused `QSC_ERR_ROUTE_TOKEN_INVALID` on both sides. Carry the rule into the runbook's next revision so the next seat does not rediscover it by refusal, alongside the already-owed cold-start-certs note (R184 §5).

### ENG-0133 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0133`.

### ENG-0134 — ⚠ P1 — a premature user send silently destroys the peer's next message — **NEW; filed 2026-08-09 by NA-0708 (D-1345) — id reserved by the NA-0706/0707 arc, which names it throughout**

Measured by NA-0706 n=5 with zero variance: a user send issued before the channel is established returns **rc 0, IS DELIVERED**, and destroys exactly ONE of the peer's messages, while `peer_confirmed` regresses yes→no and never heals. ⚠ The SAFE send emits the **identical** marker, so "attempt and handle the refusal" has no refusal to handle — the design must PREDICT. ⚠ NA-0707's cold read then established that the payload is **not cryptographically destroyed** (`ratchet_skip_store count=2`): it dies because the receive pull aborts and the frame behind it is never unpacked. **The remedy is receive-side**; the send-side guard is origination discipline, and returns with NA-0707 re-formalized. ⚠ **NA-0708 does NOT close this** — it ships only the ack flush.

### ENG-0135 — the class-narrowed-to-instance pattern — a remedy's scope is the OPERATION, not the reporter — **NEW; filed 2026-08-09 by NA-0708 (D-1345; the NA-0706/0707 hand-off, ruled onto this PR at R205 §1.1)**

D-1327 C3 fixed the unseeded-chain wedge for **CONTROL** sends only; user-initiated sends reach it through the same door. NA-0705's `QSCV01` trap is the same shape, and SR-15 F-3 supplied a third instance — the ENG-0134 ruling itself narrowed to the measured *origination* rather than the *loss class*. ⚠ **STANDING FORM:** when a fix is scoped to the caller that exhibited the defect rather than to the operation that permits it, the door stays open for every other caller.

### ENG-0136 — `ready` is computed and then unreachable — **NEW; filed 2026-08-09 by NA-0708 (D-1345; NA-0706 STOP_005 §3, hand-off) — FILING-ONLY**

`protocol_state/mod.rs:108` computes it; `handshake/mod.rs:1290-1291` prints it only on the `established_recv_only` arm, so it is absent from 73 sampled runs. A value nobody can observe is not a signal.

### ENG-0137 — a shadowed, byte-equivalent duplicate predicate — **NEW; filed 2026-08-09 by NA-0708 (D-1345; NA-0706 STOP_005 §3, hand-off) — FILING-ONLY**

`hs_send_ready_from_session` (`handshake/mod.rs:1253`) duplicates the send-ready tuple's zero-check, and the **printed reason is a hard-coded literal shadowing the computed value** — so the human reads a constant while the code decides on something else.

### ENG-0138 — `peer_confirmed` is derived, non-monotonic, and never heals — **NEW; filed 2026-08-09 by NA-0708 (D-1345; NA-0706 STOP_005 §3, hand-off) — FILING-ONLY**

Derived from `st.recv.nr == 0` at `handshake/mod.rs:1257-1264`, so it moves BACKWARDS under the m2 send and never recovers. It is a derivation presented as a latch. ⚠ Renumbered on filing to avoid colliding with the SR-15 finding also called F-5.

### ENG-0139 — the marker-consumption gap in the desktop's `MarkerBuffer` — **NEW; filed 2026-08-09 by NA-0708 (D-1345; NA-0706 STOP_005 §3, hand-off) — SPINE FILING WITH DESKTOP CITATIONS, no desktop edit**

At desktop `c52fd51b`, `src-tauri/src/markers.rs:12` holds an opaque buffer with no parsing and a 1024-entry drop-oldest cap — a hazard for anything that treats it as a safety gate, since the evidence a gate needs can be evicted by ordinary traffic.

### ENG-0140 — the rig `qsl-server` advance is a prerequisite for measuring the invite path — **NEW; filed 2026-08-09 by NA-0708 (D-1345; NA-0706 STOP_005 §3, hand-off) — FILING-ONLY**

`POST /v1/invite/create` returns **404** on the rig while `GET /v1/server-info` returns 200: the deployed relay advertises no invite capability, so **the invite path the GUI will ship is UNMEASURED end to end.** Any lane claiming invite behaviour on the rig advances the relay first or states the gap.

### ENG-0141 — the steady-state ADV/boundary crossing class — ⚠ **MEASURED**, not analysis-derived — **NEW; filed 2026-08-09 by NA-0708 (D-1345; RELABELLED at R199 §4 from the NA-0707 hand-off)**

⚠ **The prior label was wrong on a false premise.** "Lockstep harnesses cannot cross frames" is FALSE: the crossing is a ~40-line test against the refimpl's existing public API and runs in 0.01s. **Measured:** a peer's in-flight re-advertisement crossing a local reply/fallback DH boundary in an ESTABLISHED session rejects with `REJECT_S2_BODY_AUTH_FAIL` → `qsp_auth_failed` → `qsp_scka_adv_reject` (`ratchet.rs:2140-2146`), fail-closed with the receiver state asserted byte-identical. ⚠ A filing that understates a reachable defect on an untrue premise prices a future lane wrongly.

### ENG-0142 — ⚠ P1 — THE POISON-PILL WEDGE: one unprocessable frame wedges a mailbox, with an adversarial trigger — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15/D642 F-2, ⚠ MERGED with this lane's own post-withdrawal refinement — see the merge note)**

`transport/mod.rs:1147-1211`: one bad frame aborts the whole pull at `:1210`; only `qsp_replay_reject` under Lease escapes (`:1161`, `:1185`, `:1196`). Under **Legacy** the trailing frames are destroyed outright; under **Lease** (the default) the failing item is never acked or quarantined, redelivers, and re-aborts every `qsc receive` until the relay's 7-day retention expires it (`quarantine/mod.rs:77`). ⚠ **ADVERSARIAL TRIGGER: anyone who can post one unparseable frame wedges that mailbox.**

⚠ **MERGE NOTE (R205 §1.3).** Two things folded into this one entry rather than becoming separate filings. (i) **The strand half of the original F-2 — "already-processed items' acks strand behind the `return`" — is NOT filed here: it is what NA-0708 FIXES**, and it is D-1345's subject. (ii) This lane's own measurement that **the wedge survives the withdrawn taxonomy** folds in: the cheapest wedge is any structurally valid envelope with garbage inside, which surfaces as `qsp_verify_failed` — an UNCLASSIFIED code — so even the classifier as ruled would not have closed the wedge it was justified by. ⚠ **This is the successor lane's headline.**

### ENG-0143 — P-1 (`scka.peer_adv_max_seen > 0`) is UNSAFE across a re-handshake — it PERMITS the defect it was meant to refuse — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15/D642 F-1) — ⚠ SETTLES NA-0707's PREDICATE**

`qsp_session_store_with_trigger` (`protocol_state/mod.rs:802-809`) preserves the SCKA section verbatim across a fresh session, so a **fossil** `peer_adv_max_seen` survives a re-handshake while the new send chain is unseeded ⇒ P-1 permits the unsafe seed exactly where it must refuse. **P-2 (`st.recv.nr > 0`) refuses correctly** (fresh `nr == 0`). ⚠ **P-1 is REJECTED outright, not deprioritised.** And the instrument is amended with it: first-handshake rows cannot distinguish the two candidates, so **any lane shipping a predicate here owes a RE-HANDSHAKE row** — an instrument that cannot separate the candidates is not a tiebreaker.

### ENG-0144 — the deferred send emits a FALSE human diagnosis — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15/D642 F-5) — FILING-ONLY, returns with NA-0707**

Under a `Retry` disposition a refused send lands `Queued` with `paused_cause=None`, so reporting falls through to `("msgqueue_queued", "will send when the relay is reachable")` (`transport/mod.rs:1889,1892`) — **but the relay IS reachable; the local side refused to seed.** ⚠ The code three lines above (`:1884-1886`) RESERVES that string for the transient class and names its misuse "a FALSE DIAGNOSIS". The tree forbids the diagnosis and then emits it.

### ENG-0145 — a distinct error code never reaches the durable consumer artefact — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15/D642 F-6) — FILING-ONLY**

`msgqueue/mod.rs:918` records the STAGE string `"pack_failed"` into `last_error`, not `err.code`, so a distinct pack-error code survives only in a transient marker and a per-config-dir (not per-message) status record. Census: the existing distinct codes appear only at their definition sites — **no consumer reads them either.** Any argument that a new distinct code buys diagnosis is defeated at the queue boundary until a consumer exists.

### ENG-0146 — the attachment and file paths get NO deferral — same protocol condition, two contracts, neither documented — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15/D642 F-7) — FILING-ONLY**

Both pass `SendOrigination::User` (`attachments/mod.rs:1101,1769`), neither uses the msgqueue, and a new pack-refusal code would not be in `file_push_retryable`'s five-code allow-list ⇒ **immediate terminal failure on the file path** while first-contact TEXT defers silently. No silent drop was found (the failure is surfaced, not lost). ⚠ **The allow-list has never been reviewed for a new code.**

### ENG-0147 — a re-handshake after a completed PQ reseed BRICKS the channel permanently — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15/D642 F-8) — PRE-EXISTING, MEDIUM**

`qsp_scka_mono_update` max-merges and never decreases (`protocol_state/mod.rs:600-639`), so a fresh session's `peer_max_adv_id_seen=0` against a non-zero side-record trips the rollback guard (`:580`, `:925-941`) → `session_rollback_detected` **permanently, with no path anywhere that clears `qsp_scka_mono_path`.** ⚠ It also bounds ENG-0143 honestly: the completed-reseed case bricks before any predicate is consulted.

### ENG-0148 — a safety predicate would be read outside the exclusive store lock the commit path takes — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15/D642 F-9) — PRE-EXISTING, LOW-MEDIUM, fail-safe direction**

`transport/mod.rs`'s pack path takes no lock; the commit path takes `lock_store_exclusive`. There is no intra-`qsp_pack` TOCTOU (single snapshot), and the cross-seam race **fails SAFE** for a safety predicate (staler state ⇒ more likely to refuse). Recorded because a lost receive-commit across the seam can silently undo the state that made seeding safe.

### ENG-0149 — instrument validity: whether a trailing frame is DISCARDED or DEFERRED is decided by ACK MODE, not the ratchet — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15/D642 F-10) — ⚠ SATISFIED BY THIS LANE'S INSTRUMENT, recorded for the successor**

An in-process fixture that does not model lease vs delete-on-deliver pins its own transport stub rather than the protocol. ⚠ **NA-0708 discharged this for its own instrument**: `tests/na0708_ack_flush.rs` drives the **real in-process `qsl-server`** with an explicit `--ack-mode lease`, records that **Legacy cannot strand at all** (the ack accumulator is empty there by construction), and refuses the `common` mock by name. **It stays filed because the constraint binds the SUCCESSOR too** — the lane that closes the wedge must state and drive both modes, or its proof can be green for a reason unrelated to its fix.

### ENG-0150 — the DH-boundary arm returns the refimpl's RAW reason, bypassing `map_qsp_recv_reason` — **NEW; filed 2026-08-09 by NA-0708 (D-1345; divergence D-3 as corrected by SR-15 F-8)**

`qsl/qsl-client/qsc/src/lib.rs:2319` returns `out.reason` unmapped, so **ten** raw `REJECT_S2_*` strings reach `transport/mod.rs:1147` outside the `qsp_*` namespace — not five, as first censused: `REJECT_S2_LOCAL_UNSUPPORTED`, `_HDR_AUTH_FAIL`, `_BODY_AUTH_FAIL`, `_DH_NONCONTRIBUTORY`, `_BOUNDARY_NOT_IN_ORDER`, plus five parse errors reaching via `ratchet.rs:1410`. **The total reject population is 25, not 20** (15 `qsp_*` + 10 raw). Any taxonomy keyed on the `qsp_*` namespace silently misses a whole arm. ⚠ For the wedge this is bad news: a malformed DH-boundary frame is unprocessable and still wedges.

### ENG-0151 — `qsp_no_session` conflates a transient I/O failure with a genuinely absent session — **NEW; filed 2026-08-09 by NA-0708 (D-1345; divergence D-5, Director-verified at R203 §1)**

`protocol_state/mod.rs:1045` is `if let Ok(Some(st)) = qsp_session_load(channel)`, so `Err(ErrorCode::IoReadFailed)` (`:893`) and `Err(ParseFailed)` fall through to the same `qsp_no_session` at `:1049` as a real absence. An `if let Ok(Some(_))` that discards an error class. ⚠ Any future disposition that treats `qsp_no_session` as unrecoverable would convert a transient disk error into **mailbox-wide permanent data destruction**. Give the I/O failure its own code.

### ENG-0152 — `qsp_verify_failed` is the terminal `else` of `map_qsp_recv_reason` — a residual class, not a code — **NEW; filed 2026-08-09 by NA-0708 (D-1345; divergence D-7, Director-verified at R203 §1)**

`lib.rs:1662-1663`. Every reason not matching the four named patterns becomes `qsp_verify_failed`, **including reasons that do not exist yet**. It cannot be given a disposition as a unit, and routing a residual class to quarantine would auto-quarantine every reject reason ever added. ⚠ Distinct from ENG-0153: this is things falling INTO the residual; ENG-0153 is things falling OUT of it into a classified bucket. Collapsing the two loses the direction, which is the finding.

### ENG-0153 — `map_qsp_recv_reason` matches by `contains`, over strings that are sometimes composite — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-5, Q1's named sub-question answered in the negative)**

The mapper's fail-closed protection sits one layer too low: a future refimpl reason **embedding** a classified substring lands in a **classified** bucket rather than the residual. Compounded by composite reason strings of the form `REJECT_S2_X; reason_code=REJECT_S2_X`. ⚠ An allow-list built on this guards SPELLING, not semantics.

### ENG-0154 — `qsp_recv_failed` carries eight distinct origins spanning local-config and frame-structural causes — **NEW; filed 2026-08-09 by NA-0708 (D-1345; divergence D-8)**

`lib.rs:2205, 2209, 2211, 2212, 2273, 2277, 2279, 2280` — `qsp_scka_enabled` false (local configuration) sits in the same code as a missing `adv_id`/`adv_pub`/`target_id`/`ct` (a property of the frame). One disposition cannot be right for both. Split at the source before any classification is attached.

### ENG-0155 — `qsp_channel_invalid` is a local-state fault that would fire for EVERY item in a mailbox — **NEW; filed 2026-08-09 by NA-0708 (D-1345; divergence #11)**

Two origins — a bad channel label (`protocol_state/mod.rs:1043`) and an empty channel list (`lib.rs:2174`) — both functions of LOCAL state, not of the frame. ⚠ Quarantining it would convert a local misconfiguration into mailbox-wide destruction, since it fires identically for every item.

### ENG-0156 — `qsp_unpack_for_peer` returns `first_err` across channels: a wrong-channel PERMANENT failure masks a right-channel TRANSIENT one — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-4)**

`lib.rs:2159-2175` iterates the peer's channels and returns the FIRST error seen. The code that reaches the caller may therefore describe a channel the frame was never for. ⚠ Any classifier keyed on that code is classifying the wrong failure.

### ENG-0157 — `REJECT_S2_BOUNDARY_NOT_IN_ORDER` is a COMPOSITE string, so no `==` classifier can ever match it — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-2)**

Emitted at `ratchet.rs:1432, 1476` in the form `REJECT_S2_BOUNDARY_NOT_IN_ORDER; reason_code=…`. ⚠ It is also the reason the withdrawn taxonomy's row 17 could never have matched — a classification that was ruled and would have been dead code.

### ENG-0158 — the withdrawn row 17's origins are BOTH permanent, so a transient disposition there would create unbounded redelivery — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-3)**

The taxonomy ruled `REJECT_S2_BOUNDARY_NOT_IN_ORDER` transient by analogy with `qsp_ooo_reject`. Measured, both of its origins are permanent, so *continue without ack* would leave the frame redelivering forever, bounded only by relay retention. ⚠ **Any transient disposition must carry a BOUND** — an attempt counter with escalation, or at minimum a witnessed marker. "No ack, no quarantine, no seen-store record" has no client-side bound at all.

### ENG-0159 — row 7 (`qsp_scka_target_unknown`) carries the exact conflation used to disqualify `qsp_no_session` — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-6)**

Four distinct local failure modes collapse onto "advkey absent". The taxonomy disqualified `qsp_no_session` for precisely this shape and then ruled row 7 clean. ⚠ Recorded as evidence that a rule naming a failure class does not immunise the author applying it.

### ENG-0160 — row 1 (`qsp_scka_adv_reject`) — the withdrawn taxonomy's HEADLINE — is itself a conflated bundle with a transient member — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-7)**

The one row the whole lane was justified by. One of its members is transient, so the classification that would have shipped was wrong on its own flagship case.

### ENG-0161 — ten stock-reachable quarantine classes would make the 256-slot global store attacker-floodable, with oldest-first eviction — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-11, charter (b)/(c))**

NA-0689's capture surface is four hostile-peer-only sites plus one reachable only by our own crash. Widening it to ten stock-reachable classes changes the store's threat model: an attacker who can post can evict prior evidence. ⚠ Any future quarantine widening prices the store first.

### ENG-0162 — `quarantine_then_ack` swallows capture failure, turning a loud abort into a silent exit-0 loop — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-12, charter (g))**

The capture path returns `Ok(())` on failure, so a persistent capture failure across many classes would ack-and-continue silently rather than failing loudly. Today it is reachable for one class; a widened classifier multiplies it.

### ENG-0163 — `invite finish` pulls the ordinary inbox and hard-fails on whatever is at its head — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-13, charter (a))**

⚠ **The wedge is reachable through a command no classifier inside `receive` would ever see.** D-1328 §1 deliberately refused to convert the three flag-less pull callers; that refusal's grounds must be re-derived, not inherited, by the lane that closes the wedge. Measured live by NA-0708's own instrument: the invite dance leaves a bare handshake frame in each party's inbox, and it wedged the arm until a legacy drain cleared it.

### ENG-0164 — raw `REJECT_S2_*` codes flow into the status record and the marker `code=` field, and the composite one breaks the plain marker grammar — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-14) — ⚠ LIVE TODAY, independent of any classifier**

`transport/mod.rs:1159-1160` writes whatever code arrives into `record_qsp_status` and the `qsp_unpack` marker, in a field that everywhere else carries `qsp_*`. The composite string additionally contains `; reason_code=` and breaks the plain marker grammar for any consumer parsing it. ⚠ **This one does not wait on the successor** — it is happening at this rev.

### ENG-0165 — the seen store and the quarantine store are whole-file rewrites with no locking — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-17, charter (e))**

Two concurrent `qsc receive` processes on the same mailbox can lose an update. The structural flush widens the window slightly by making the flush point later on failure paths. Pre-existing; recorded with its direction stated.

### ENG-0166 — row 18's `Subclass::Unsupported` promise is bounded by a 7-day TTL and a 256-slot cap while the ack deletes the relay's copy — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-18)**

"A future build could read it" is the subclass's stated value, but the retained copy expires at `quarantine/mod.rs:77`'s `604800` seconds and can be evicted by cap — after the ack has already deleted the only other copy. ⚠ A forward-compat promise that outlives neither the TTL nor a flood.

### ENG-0167 — a duplicate `msg_id` is written to `recv_N.bin` and counted before it is detected, and `commit_unpack_state()` runs twice on that path — **NEW; filed 2026-08-09 by NA-0708 (D-1345; SR-15 F-19) — PRE-EXISTING, adjacent**

Found by the cold read while tracing the receive loop; not this lane's to fix and not touched by it. Recorded so the next receive-path lane inherits it rather than rediscovering it.

### ENG-0168 — only the handshake INITIATOR can advertise SCKA — a responder is `chainkey_unset` until it has sent — **NEW; filed 2026-08-09 by NA-0708 (D-1345) — measured while building this lane's instrument**

At establishment `tools/refimpl/quantumshield_refimpl/src/suite2/establish.rs:75-98` gives **role A** `ck_ec: ck0_a2b, ck_pq: pq0_a2b` and **role B** `ZERO32` for both; the advertisement guard (`lib.rs:1822-1826`) requires both non-zero. ⚠ Since `invite accept` makes the INVITER the handshake RESPONDER, an inviter cannot advertise until it has sent. Same surface as ENG-0133 / NA-0705's F6, and it cost this lane four instrument iterations to discover. Also recorded: **SCKA is off entirely in every seed-derived session** (`qsp_scka_enabled` is `dhr != dhs_pub`; the seed derivation sets both to the same value), so no house fixture using the seed fallback can produce a control envelope.

### ENG-0169 — MOVED to `docs/ops/DESIGN_QUESTIONS.md` (DOC-OPS-008) by NA-0709 (D-1346)

⚠ **MOVED, NOT CLOSED.** This entry is not a defect; it is a design question, a product call, or an item accepted by ruling. Its **id and its text are preserved verbatim** in DOC-OPS-008. Nothing was discarded, and it is not resolved. See DOC-OPS-008 → `ENG-0169`.

### WF-0051 — a cold-seat commission granting a repo tree must name the tree's agent-memory file — **NEW; filed 2026-08-09 by NA-0708 (D-1345; R199 §5, the method record) — RECIPE AMENDMENT, not a code defect**

A commission that grants a clone must **NAME the tree's `CLAUDE.md` for pre-emptive removal before the reader's first content read**, or state that its injection is accepted and recorded. "Outside project memory scope" cannot be satisfied by instruction alone while the clone lives inside the seat. ⚠ Third seating hole found by having readers **ATTEST rather than assert**. Applied at NA-0708's own SR-15 commission, where the reader removed the file and disclosed the resulting ` D CLAUDE.md` rather than claiming a clean tree it did not have — and the expectation was corrected to carve the deletion out (R203 §7.2).

⚠ **AMENDED 2026-08-10 by NA-0709 (D-1346; R211 §2.3) — the rule as first written names ONE file and that is the hole.** The spine root carries **TWO** agent-memory files, `CLAUDE.md` **and** `AGENTS.md`; the desktop carries only `CLAUDE.md`. Every prior cold read against the spine was therefore seated with `AGENTS.md` in place and none disclosed it — **because the recipe never told them it existed. A commission failure, not a reader failure.** The amended rule: **a commission granting a tree ENUMERATES EVERY agent-memory file in that tree, measured at drafting, and names each for pre-emptive removal** — never a single named file. ⚠ **The enumeration is the instrument; a name is a needle, and this program has a long record of needles that stopped matching.** See **WF-0058** for the wider unknown this exposed.

### ENG-0171 — the attachment `file_id` marker population is orphaned: 14 sites reach the shape-keyed redactor as `id`, and the population was recorded only inside a CLOSED entry — **NEW; filed 2026-08-10 by NA-0709 (D-1346; R218 §2.2(a)) — the ENG-0084 residue given an ID**

- Severity: **P2** (privacy exposure in shareable diagnostics; redaction here holds by a value-shape accident, not by a rule)
- Status: open — filed 2026-08-10. **FILING ONLY; nothing executed.**
- Exact surfaces: `qsl/qsl-client/qsc/src/attachments/mod.rs` — **exactly 14** `("id", …)` marker tuples at `:884`, `:1086`, `:1601`, `:1668`, `:1744`, `:1930`, `:1948`, `:1996`, `:2024`, `:2066`, `:2141`, `:2166`, `:2247`, `:2252`; the coupling at `qsl/qsl-client/qsc/src/output/mod.rs:316` (`should_redact_value` carries **no rule for the key `id`**) falling through to `looks_high_cardinality` at `:349` (`value.len() >= 24 && …is_ascii_digit()`)
- Claim at stake: that a diagnostic artifact a user is invited to share carries no identifier that links them to an object or a peer.
- Why it matters: these values are redacted **by accident of their current width and digit content**, not by rule. A future change that shortens an attachment `file_id` below 24 characters, or drops its digits, makes all 14 sites emit the raw identifier in the clear. Nothing detects it.
- ⚠ **Why this entry exists at all — and it is the general lesson, not a bookkeeping note.** The population was named in prose inside **ENG-0084** (`docs/ops/IMPROVEMENT_LEDGER.md:2705-2708`) beneath a `Resolution:` line, and **nowhere else**: `file_id` appears at exactly three lines in this whole file, **all inside ENG-0084**. The *"future attachments-diagnostics lane"* it names has no id and no entry. **Residue survives only if it has an ID. A paragraph inside a closed entry is not a queue item, and a defect that is not in the queue is not looked for.**
- Proof gap: nothing asserts that a marker key of `id` is redacted independently of its value's shape.
- Recommended directive shape: implementation-only, **with ENG-0122** — the same shape-vs-semantics gap for the key `peer`, same file, same function, one adversarial read.
- Cross-reference: **ENG-0084** (the closed parent, which now points here), **ENG-0122**, AUDIT-TRIAGE #001.
- Originating/last lane: NA-0709 (D-1346). Last-updated: 2026-08-10.

### ENG-0172 — ENG-0038's P1 authentication remediation rests on a REASONED, NOT MODEL-VERIFIED argument, and the regression guard it names is observed by nothing — **NEW; filed 2026-08-10 by NA-0709 (D-1346; R218 §2.2(b)) — the ENG-0038 residue given an ID**

- Severity: **P2** (assurance depth on a **P1 authentication property**; the remediation as titled is not in doubt — what is undischarged is the verification that it is complete)
- Status: open — filed 2026-08-10. **FILING ONLY; nothing executed.**
- Exact surfaces: `docs/ops/IMPROVEMENT_LEDGER.md:1721` — *"the P3 verdict is ARGUED to survive this, and that argument is REASONED — NOT MODEL-VERIFIED … recorded as a claim to be discharged, not as a result"*, with the named-but-never-filed follow-up *"extend the bounded model to the device indirection + primary-device selection"*; the **KNOWN UNMODELED SLICES** block at `:1719-1722`, including *"Cross-session replay"*
- Claim at stake: peer authentication — the property ENG-0038 was filed against.
- ⚠ Why it matters, in the parent entry's own words: **the regression guard has no observer.** `:1721` records *"if the verification-code format is ever narrowed back to the KEM half, this discharge is VOID"* — and **no test, gate, or witness observes that condition**, so an ordinary future change can void the discharge with nothing failing.
- ⚠ Why this entry exists: ENG-0038 fires **I3 alone** — its sole closure signal is its `- Status:` line at `:1713`, with **no `Resolution:` line anywhere in the entry** — against this ledger's own rule at `:41-43` that closure is **never** read from `Status:`. The named follow-up has no id: `device indirection` / `primary-device` / `primary_device_id` appear at `:1720-1721` only.
- ⚠ Scope note: **part of this residue lives in `WF-0019`** (`:1741-1749`), a register the NA-0709 triage ruled out of scope. The crossing is recorded; WF was not triaged.
- Proof gap: no model-checked result covers the device indirection + primary-device selection, and nothing observes the format-narrowing precondition that would void the discharge.
- Recommended directive shape: a formal-methods lane extending the bounded model, **or** an explicit operator acceptance of a reasoned-not-verified argument on a P1 property — recorded either way. ⚠ **An observer for the format-narrowing condition is cheap and separable from the model work**, and is worth taking first.
- Cross-reference: **ENG-0038** (the closed parent, which now points here), **WF-0019**, **ENG-0035** (the ProVerif termination limit that bounds what a model can reach).
- Originating/last lane: NA-0709 (D-1346). Last-updated: 2026-08-10.

### WF-0052 — `goal-lint`'s `Goals:` requirement is enforced only post-hoc, and the hint it prints when it fails is broader than the truth — **NEW; filed 2026-08-10 by NA-0709 (D-1346), inherited from NA-0708 §6**

- Problem: `tools/goal_lint.py` requires a literal `Goals: G#` line in the **PR body** and can only say so **after** a PR exists, with a template nobody is forced through. Caught twice (NA-0650, NA-0708). ⚠ **And its own remedy text is wrong in the expensive direction**: `tools/goal_lint.py:61` prints *"If the PR body was edited and rerun still fails, close + reopen the PR to trigger a fresh pull_request event payload."*
- ⚠ The measured fact, narrower than both that hint and the NA-0650 record imply: **a body edit alone does not re-trigger; a body edit FOLLOWED BY A PUSH does.** A lane with a further commit to make never needs the close+reopen; only a lane with none does.
- Recommended change: correct the hint at `tools/goal_lint.py:61` to state the narrow rule, and consider a pre-PR check so the requirement is met before a PR exists rather than after.
- Status: open — **FILING ONLY**. Originating/last lane: NA-0708 (D-1345) → filed by NA-0709 (D-1346). Last-updated: 2026-08-10.

### WF-0053 — a rule that must be remembered at every invocation will be missed at some invocation: the pipe/exit-status discipline wants a MECHANISM, not more memory — **NEW; filed 2026-08-10 by NA-0709 (D-1346), inherited from NA-0708 §6**

- Problem: the house rule *"never pipe the check that gates you — a pipe reports the pipe's exit status"* was recorded twice and promoted once, and was then violated **three times in a single lane** (NA-0708). ⚠ **That is not a discipline failure; it is evidence the rule is in the wrong place.**
- Recommended change: a house wrapper that runs a gate **bare**, captures its exit to an artifact, and **cannot report a status it did not read from that artifact**. ⚠ Same argument the program already accepted for structural sharing over "N sites agreeing" (D-1328 Ruling 11), applied to instruments instead of code.
- Status: open — **FILING ONLY**. Originating/last lane: NA-0708 (D-1345) → filed by NA-0709 (D-1346). Last-updated: 2026-08-10.

### WF-0054 — this ledger is three ledgers wearing one file: three closure instruments, three eras, and no canonical form — **NEW; filed 2026-08-10 by NA-0709 (D-1346; R209 §3.3)**

- Problem: measured across all 169 `ENG` entries at `b845e678` — **I1** (a bold closure verb in the heading) fires 10 times, **I2** (an appended `Resolution:` line) 27, **I3** (a closure verb opening `Status:`) 30. ⚠ **`I1+I2+I3` = 0 and `I1+I2` = 0: no entry is closed by all three, and the documented instrument never co-occurs with the pre-convention one.** They are not three fields; they are **three eras laid down in sequence and never reconciled**. 35 of the 51 closures are closed by exactly one. Three of the six documented lifecycle states (`queued`, `in-lane`, `promoted`) have **zero** occurrences, and the `Status:` field carries **14 distinct spellings**.
- Why it matters: a backlog whose own state cannot be queried is an instrument that does not instrument — the defect family this ledger already tracks. It is why the same defect was filed twice (see WF-0055) and why residue was orphaned twice.
- Recommended change: **not** a mass rewrite — that is a closure act. Either adopt one canonical closure form going forward and record the eras as history, or give the file a machine-checked schema.
- Status: open — **FILING ONLY**. Originating/last lane: NA-0709 (D-1346). Last-updated: 2026-08-10.

### WF-0055 — a ledger nobody can query produces the same defect twice: ENG-0131 duplicated ENG-0077 fourteen days apart — **NEW; filed 2026-08-10 by NA-0709 (D-1346; R210 §2)**

- Problem: **ENG-0131** (filed 2026-08-09 by NA-0705) and **ENG-0077** (filed 2026-07-26 by NA-0680) are **one defect** — same file, same function, same mechanism, same one-line remedy — filed fourteen days apart by two lanes that could not see each other's work. ENG-0131 is now recorded SUPERSEDED and the defect survives under ENG-0077.
- ⚠ **The finding is not the duplicate; it is what produced it.** With ~99 live entries and no working query (WF-0054), ENG-0131 is unlikely to be the only one, and a duplicate splits a defect's evidence across two ids so that fixing one looks like fixing the class.
- Recommended change: a pre-filing check — before a lane mints an id, search the ledger for the same **surface**, not the same words.
- Status: open — **FILING ONLY**. Originating/last lane: NA-0709 (D-1346). Last-updated: 2026-08-10.

### WF-0056 — one entry records another entry's closure and the closed entry never learns: the ledger has no cross-reference discipline — **NEW; filed 2026-08-10 by NA-0709 (D-1346; R210 §3)**

- Problem: **ENG-0063's own text states that NA-0670 / D-1297 C-2 shipped the constant-time bearer comparison — which is the defect ENG-0014 was filed against. ENG-0014 carries no annotation and is still open.** Nothing propagates a closure to the entries that describe the same defect.
- ⚠ Consequence, and it runs in the direction that wastes work rather than the one that loses it: **the true closed count is unknown, and the ledger may be carrying defects already fixed elsewhere.** Deciding ENG-0014 needs a read of a repository the triage lane was not granted.
- Recommended change: when a lane closes an entry, it greps the ledger for other entries naming the same surface and annotates them; the cross-repo block (ENG-0014/0021/0039/0063/0066/0070/0092) is the first place to apply it.
- Status: open — **FILING ONLY**. Originating/last lane: NA-0709 (D-1346). Last-updated: 2026-08-10.

### WF-0057 — the 51 `WF` entries have never been triaged, and the same status defect lives one register over — **NEW; filed 2026-08-10 by NA-0709 (D-1346; R209 §5)**

- Problem: NA-0709 triaged all 169 `ENG` entries and **ruled the 51 `WF` entries explicitly out of scope**, so the claim boundary says ENG. ⚠ Those 51 have the same three-era status problem (WF-0054) and have never been measured. **Recording the omission is what makes it deliberate rather than discovered later.**
- ⚠ Known crossing already: part of **ENG-0172**'s residue lives in **WF-0019**, so the two registers are not independent.
- Recommended change: a WF triage lane in the NA-0709 shape — the same three-instrument union, the same per-entry verdict schema.
- Status: open — **FILING ONLY**. Originating/last lane: NA-0709 (D-1346). Last-updated: 2026-08-10.

### WF-0058 — the injection surface of a cold seat has never been enumerated: we know two channels and assume that is the set — **NEW; filed 2026-08-10 by NA-0709 (D-1346; R211 §2.4)**

- Problem: SR-15 commissions name agent-memory files for pre-emptive removal so a blinded reader is genuinely blinded. ⚠ **Nobody has measured what a harness actually injects.** `CLAUDE.md` was the known channel; **`AGENTS.md` was found only because a commission was drafted fresh and its author enumerated the tree root** (WF-0051's amendment). The NA-0709 reader then disclosed a **third** surface unprompted: the spine also carries a **`.claude/` directory** of harness configuration that *"delete the agent-memory files"* does not cover.
- ⚠ **Independently corroborated**: a second cold reader, on another lane, in another tree, with no contact, found the same gap from the other side — `.claude/settings.json` carries an executing hook and a permissions deny-list. **Two readers, two trees, same gap: evidence, not coincidence.**
- Recommended change: enumerate the injection surface once — every file and directory a harness reads at seat time — and make the seating recipe name the **enumeration**, not a list of filenames.
- Status: open — **FILING ONLY**, and stated as a **named unknown** rather than a defect with a known extent. Originating/last lane: NA-0709 (D-1346). Last-updated: 2026-08-10.

### WF-0059 — a directive can grant privileged authority that the HARNESS refuses, and neither document knows about the other — **NEW; filed 2026-08-11 by NA-0710 (D-1347; R215 §4.4)**

- Problem: `.claude/settings.json` in this repo carries a permissions **deny-list** including `Bash(sudo:*)`, `Bash(apt:*)`, `Bash(apt-get:*)`, `Bash(systemctl:*)`. ⚠ **Those are exactly the acts a provisioning directive authorizes.** A lane can be approved to install a service and then be refused by the harness **mid-execution, with root already held** — the worst place to discover a gap.
- ⚠ **NOT an argument for removing the guardrail.** The deny-list is a deliberate safety mechanism. **The defect is the mutual ignorance of the two documents.**
- Measured (NA-0710, 2026-08-11): a bare local `sudo -n true` was **NOT refused** by the harness for a seat whose session directory was **outside** a repo checkout — ⚠ so the deny-list's applicability **depends on settings resolution and working directory, which nobody in this program has measured.** That is the fact worth having: **it is conditional, and the condition is unmeasured.**
- ⚠ **Untested inference, recorded as such:** the deny entries are keyed to a command's **leading token**, so `ssh host 'sudo …'` presents as `Bash(ssh:*)`. **If** that is how matching works, a guardrail keyed to spelling rather than effect **can be defeated without intent** (`bash -c 'sudo …'`, a script calling sudo internally). ⚠ **This was NOT confirmed** — nothing was denied, so no spelling comparison was possible.
- Recommended change: before a directive authorizes privileged acts, **measure whether the executing seat's harness permits them**, and record the answer with the authorization. A one-command no-op probe settles it and costs nothing.
- ⚠ Distinct from **WF-0058**, which is about *blinding a cold reader*. This is about *a lane's authorized acts being refused at execution* — different failure, different consequence.
- Status: open — **FILING ONLY**. Originating/last lane: NA-0710 (D-1347). Last-updated: 2026-08-11.


### WF-0060 — `DOC-OPS-006 §2` names an authoritative directive counter that is STALE, and it is the same failure class that document already warns about — **NEW; filed 2026-08-11 by NA-0711 (D-1348; R231 §5)**

- Problem: §2 names `/srv/qbuild/operator/directives/` the authoritative source for the `QSL-DIR-…-NNN` counter. **Measured 2026-08-11: that directory tops out at 638 (103 files) while the lane directories and the repo record read 646.** A seat deriving the counter as instructed would have minted a number **eight behind**.
- ⚠ The document already corrects an identical defect for `/srv/qbuild/operator/responses/` (drifted suffix convention, stale archive) and then **exhibits it for its own replacement source.** ⚠ **A governance doc that documents a failure class and then exhibits it is worth the filing on its own.**
- Recommended change: derive the counter from **the RECORD** (the highest `QSL-DIR-…-NNN` across the lane directories, cross-checked against `NEXT_ACTIONS.md`/`DECISIONS.md` citations) and say so in §2.
- Status: open — **FILING ONLY.** Originating/last lane: NA-0711 (D-1348). Last-updated: 2026-08-11.

### WF-0061 — a walk that records what a command SAID but not what it WAS ASKED cannot answer a whole class of question — **NEW; filed 2026-08-11 by NA-0711 (D-1348; R232 §1)**

- Problem: NA-0710's walks preserved **markers, not argv.** When the diagnosis turned on *which label the completing poll was given*, the record could not answer, and the question had to be settled by probing preserved vaults instead. ⚠ **The decisive input was the command line, and it was the one thing not kept.**
- ⚠ Note the shape: the walk's evidence discipline was otherwise exemplary — sealed expectations, per-row logs, preserved vaults. **It kept everything except the question that mattered.**
- Recommended change: a walk record preserves the **exact invocation** of every step (with secrets redacted by name, never by omission of the flag), not only the marker stream it produced.
- Status: open — **FILING ONLY.** Originating/last lane: NA-0711 (D-1348). Last-updated: 2026-08-11.

### WF-0062 — an instrument must be provable in the direction it can fail, and must not be able to destroy the evidence it exists to produce — **NEW; filed 2026-08-11 by NA-0711 (D-1348; R242 §4.2 as extended by R244 §2.3)**

- **Two rules, adopted from three self-inflicted faults in a single lane, all self-reported:**
  1. ⚠ **"A liveness or completion check must be proven able to return NO before its YES is trusted"** — the rule this lane applied to its own test rows (every row watched RED first), applied to its watchers.
  2. ⚠ **"An instrument must not be able to destroy the evidence it exists to produce."**
- **The three faults:** a settle-watcher looping on `pgrep -f "cargo test --offline"` **matched its own command line** and read `RUNNING` for **4 h 40 m after the suite had finished** · the `pkill` cleaning it up **matched its own argv** and killed the shell running it · and a walk logger using `tee /dev/stderr` inside a command substitution, with stdout and stderr both redirected to one file, **overwrote that file's first 1359 bytes with NULs and destroyed three rows of transcript.**
- ⚠ **The tell was available and unread in the first case** — the log's mtime had stopped while the signal kept saying yes — and it surfaced **because a human asked "status?"**, not because the instrument spoke. **An instrument that needs a human to notice it is stuck is not reporting.**
- ⚠ **Same family as this lane's own subject** (a marker that could not distinguish two states) and as **ENG-0174's** misreading (an absence standing in for a state). **Tenth member of the family; the pattern is now a design constraint, not a lesson.**
- Status: open — **FILING ONLY.** Originating/last lane: NA-0711 (D-1348). Last-updated: 2026-08-11.

### WF-0063 — an SR-15 commission must enumerate EVERY deliverable, and every obligation the reader owes must lie INSIDE the reader's granted reading set — **NEW; filed 2026-08-11 by NA-0711 (D-1348; R238 §4)**

- Problem: D647's commission listed **one** deliverable (the findings file) and never named the **pre-flight attestation**, whose requirement lived only in R236 §5.2 and a stop-file — **both outside the reader's §5 grant.** ⇒ ⚠ **"A cold reader following the paste exactly produces no attestation and does not know it."**
- Measured: the reader learned it owed one **only by reading outside its grant**, at a point when the compliant moment had passed, **and disclosed that too** — the finding is the reader's, self-reported against its own first edition.
- ⚠ Second, from the same attestation and ruled program-level at **R242 §3**: the disclosure clause asked which agent-directed files were **"in place"**, and the true answer to that is not the answer that matters — **"in force"** is. Every SR-15 read this program has run was **rooted outside the repo**, so the repo's `.claude/settings.json` (its deny list and its `PreToolUse` hook) **never loaded**; the governing file was the user-level one, with `bypassPermissions` and no hooks. ⚠ **"Write nothing anywhere" was honoured by discipline, not by enforcement.** ⚠ **A protection that is declared but not loaded is not a protection** — the same shape as a gate that is present but suppressed.
- Recommended change: §10 enumerates **all** deliverables; the grant includes every artifact carrying an obligation; the disclosure clause reads **"in force, with the governing settings path named."** ⚠ **Whether an SR-15 seat should be rooted INSIDE the repo is a separate governance question, filed at R242 §3.3 and NOT answered here** — it trades enforcement against the blinding the recipe exists to create.
- Status: open — **FILING ONLY.** Originating/last lane: NA-0711 (D-1348). Last-updated: 2026-08-11.
