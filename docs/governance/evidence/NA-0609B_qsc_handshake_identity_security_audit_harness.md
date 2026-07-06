Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

# NA-0609B — qsc Handshake and Identity Read-Only Security Audit

## Summary

NA-0609B is a read-only, evidence-first security audit of the qsc handshake and
identity seam, executed under directive QSL-DIR-2026-07-06-543 (D543) as the first
LITE-CEREMONY lane (single PR, single decision D-1213). It applies the NA-0609A
Director triage discipline: DOC-AUD-001 §4.1/§5 and ledger ENG-0001 both name this
seam as the highest-yield security surface and prescribe a read-only audit before
any remediation. This lane reads security-critical code and changes no source,
tests, or crypto; every concrete fix is routed to a separate full-ritual lane.

Result classification:
`QSC_HANDSHAKE_IDENTITY_AUDIT_COMPLETE_NO_P0_P1_THREE_P3_HARDENING`.

No P0 or P1 finding was substantiated. The handshake seam is well-constructed and
fail-closed. Three P3 hardening/robustness findings are recorded and routed to the
ledger. This is a bounded audit; it is not an external review, a formal-proof, or
a crypto-completeness claim.

## Scope And Method

Primary surfaces read (current main `a93e388623e6`):
`qsl/qsl-client/qsc/src/handshake/mod.rs`, `.../identity/mod.rs`,
`.../protocol_state/mod.rs`, `.../fs_store/mod.rs`, with cross-reference to the
handshake/identity/session-at-rest/adversarial test suites and canonical specs
`DOC-CAN-003` (QSP Suite-2 triple ratchet) and `DOC-CAN-004` (SCKA). Method:
static reading of the three accept sub-paths (initiator processes B1; responder
processes A2/confirm; responder processes A1/init), the transcript/MAC/signature
derivations, the identity-pin checks, and the at-rest persistence path. Findings
follow the DOC-AUD-001 §6 schema; the §6 anti-patterns are rejected.

## Verified Sound

Grounded in code at the cited lines, the following held:

- Transcript binding: `hs_transcript_mac`/`hs_transcript_hash` (handshake/mod.rs
  734-748) key a KMAC over `A1 || B1_no_mac` with the hybrid PQ-init secret
  (`hs_pq_init_ss`, 761) and domain-separation labels, and bind the suite context
  via `hs_append_key_context`. Every accept path recomputes and checks the
  transcript MAC before proceeding (1457-1458, 1659-1674).
- Hybrid handshake: ML-KEM PQ shared secret combined with X25519 DH
  (`hs_dh_shared`, 789), with an explicit all-zero DH-public-key guard
  (`hs_dh_pub_is_all_zero`, 810).
- Signatures: `hs_sig_verify` (866) uses ML-DSA-65 and is fail-closed on both
  invalid and error; the A2 confirm signature is verified over the transcript
  hash and confirm MAC (`hs_sig_msg_a2`, 857) before acceptance (1683-1688).
- Downgrade resistance (G3): suite context is bound into the key/transcript
  derivations; suite-required mode rejects non-explicit contexts (1630-1636); a
  context mismatch rejects (1637-1641).
- Identity binding: a dual-pin model — the primary pin is checked against the KEM
  identity fingerprint (`identity_fingerprint_from_pk(kem_pk)`, the same value
  `identity show` displays), with the ML-DSA signing-key fingerprint
  (`hs_sig_fingerprint`) as a separate optional pin. A mismatch fails closed with
  `peer_mismatch` (`hs_require_primary_identity_pin`, 896) and is checked before
  any persistence (1491, 1721-1729, 1823).
- No-mutation-on-reject: in all three accept sub-paths, `qsp_session_store` occurs
  only AFTER transcript-MAC, signature, and identity verification; every reject
  path `continue`s/returns before persisting (initiator 1458→…→store; responder
  confirm 1665→1684→1721→1739; responder init 1811/1823→…→1928).
- Operator markers: `handshake_complete` is emitted only after the session store;
  `sig_status ok=true` is a per-signature status, not a handshake-success overclaim.
- At-rest persistence: `write_atomic` (fs_store/mod.rs 94-123) writes a temp file,
  enforces perms, `sync_all`s the file, then atomically renames — so file content
  is never partially written.
- Replay: session_id binding plus explicit `hs_reject_replay` (1811) reject
  replayed A1 when a pending/session already exists.

## Findings (ranked; DOC-AUD-001 §6 schema)

### ENG-0003 — Non-constant-time keyed-MAC comparisons in the handshake accept path
- Severity: P3 (implementation-attack surface; low current exploitability)
- Surfaces: `qsl/qsl-client/qsc/src/handshake/mod.rs:1458` (B1 transcript MAC:
  `mac != resp.mac`), `:1665` (A2 confirm MAC: `expect != confirm.mac`). No
  constant-time equality helper (`ct_eq`/`subtle`/`fixed_time_eq`) exists anywhere
  in the qsc or refimpl crypto stack.
- Claim violated: defense-in-depth expectation that keyed MAC/tag comparisons are
  constant-time (implementation-attack resistance; G5-adjacent hardening).
- Why it matters: Rust's array `!=` short-circuits and is not constant-time, so a
  precise timing oracle could in principle aid byte-at-a-time MAC forgery.
  Exploitability is LOW here because (a) acceptance also requires a valid ML-DSA-65
  signature the attacker cannot forge, and (b) session keys are fresh per handshake
  (`session_id`), limiting a repeatable oracle — though in legacy/non-explicit
  mode a bad confirm MAC does not clear pending (1670-1673), leaving a more stable
  target than explicit mode (which clears pending at 1668).
- Minimal fix direction: add a constant-time fixed-length comparison helper and use
  it at both MAC-comparison sites (and audit for other tag comparisons).
- Proof gap: no test asserts constant-time comparison for handshake MAC/tag paths.
- Recommended directive shape: implementation-only; natural first item for the
  NA-0609 implementation-attack hardening batch.

### ENG-0004 — Directory fsync is a no-op; atomic-rename durability not guaranteed
- Severity: P3 (crash-durability; fail-closed-safe direction)
- Surfaces: `qsl/qsl-client/qsc/src/fs_store/mod.rs:359`
  (`fn fsync_dir_best_effort(_dir: &Path) {}` — empty), called after the rename in
  `write_atomic` (123).
- Claim violated: G2 "crash-safe state persistence" — a completed session/config
  write should survive a power-loss crash.
- Why it matters: the file content write is atomic (temp + `sync_all` + rename), so
  no partial/corrupt file is possible, but the directory entry created by the
  rename is not fsync'd, so on power loss immediately after a store the rename may
  be lost and the prior state re-exposed. Direction is fail-closed-safe (a reverted
  ratchet re-handshakes rather than accepting stale key material), but it is a real
  gap against the G2 durability gate.
- Minimal fix direction: implement a real directory fsync (open the dir, `sync_all`),
  or, if durability is intentionally deferred, document the durability boundary
  explicitly against G2.
- Proof gap: no crash/durability test exercises loss of the directory entry.
- Recommended directive shape: implementation-only (or a docs/evidence boundary
  statement if deferred), scoped to fs_store.

### ENG-0001 (resolved-into-finding) — Silent `--as <label>` self-identity divergence
- Severity: P3 (robustness/UX footgun; not an identity-binding defect)
- Surfaces: `qsl/qsl-client/qsc/src/identity/mod.rs` self-identity selection;
  `handshake init/poll --as <label>`.
- Resolution of the ENG-0001 question: the verification-fingerprint model is
  COHERENT. The fingerprint an operator verifies out of band is the KEM-based
  identity fingerprint shown by `identity show`, which is exactly the primary pin
  the handshake checks; the ML-DSA fingerprint is a separate optional pin. The
  NA-0608 confusion was not a KEM-vs-SIG binding flaw — it was a self-label bug:
  using a non-default `--as <label>` inconsistently across `identity rotate/show`
  vs `handshake` silently selects a divergent lazily-created identity, producing a
  spurious `peer_mismatch`.
- Why it matters: a mistyped or inconsistent self label silently operates a
  different local identity instead of failing loud, which is confusing and could
  mask a real mismatch during operator verification.
- Minimal fix direction: fail loud (or warn distinctly) when `--as <label>` names
  a self identity that does not already exist, rather than lazily creating a new
  divergent one; and/or document the canonical single-self-label convention.
- Proof gap: no test asserts behavior for an unknown/inconsistent self label.
- Recommended directive shape: small implementation-only, or docs/UX clarification.

(ENG-0002, the attachment single-send-per-session item, is out of this handshake
audit's scope and remains open on the attachment/hardening track.)

## No P0/P1 Escalation

No actively exploitable, catastrophic finding was substantiated, so no
stop-and-escalate was triggered. Per DOC-AUD-001 §8, residual risk is honest,
ranked, and converted into bounded ledger items rather than hidden; the promoted
handshake surface carries no unresolved P0/P1.

## Boundary And Claim

This lane mutated only docs/evidence/ledger/governance paths; it changed no `.rs`,
test, Cargo, workflow, canonical spec, `.claude`, or hook file, and applied no
fix. No runtime/LAN action occurred. No endpoint, port, token, capability, key,
seed, plaintext, ciphertext body, or raw private material is published. No
public-readiness, production-readiness, security-completion, crypto-complete,
vulnerability-free, or bug-free claim is made. This is a bounded internal read-only
audit, not an external or formal review.
