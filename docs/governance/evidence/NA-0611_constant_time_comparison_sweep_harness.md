Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0611 — Constant-Time Comparison Sweep Beyond the Handshake Seam (read-only audit)

## Summary

NA-0611 is a read-only audit (DOC-G5-005 §9 rank 2 / ledger ENG-0005) executed under
directive QSL-DIR-2026-07-07-548 (D548) as a LITE-CEREMONY lane (single PR, single
decision D-1221). It enumerates and classifies qsc tag/MAC/secret comparison sites
outside `handshake/mod.rs` (whose MAC comparisons were made constant-time in NA-0609C)
and determines, for each, whether it is a keyed-secret comparison warranting
constant-time treatment. No source was changed; every fix (if any) returns as its own
lane.

Result classification: `CONSTANT_TIME_POSTURE_SOUND_NO_KEYED_SECRET_COMPARE_OUTSIDE_HANDSHAKE`.

No P0/P1 was substantiated. The genuine keyed-MAC comparisons were the handshake
compares, already fixed. One optional P3 defense-in-depth item is recorded (ENG-0008).
This is a bounded internal sweep, not an external/formal review and not a
side-channel-free claim.

## Required Markers

- NA0611_D1219_CONSUMED_OK
- NA0611_D1220_CONSUMED_OK
- NA0611_FRESH_QWORK_PROOF_OK
- NA0611_CURRENT_MAIN_HEALTH_OK
- NA0611_D1221_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0611_LITE_CEREMONY_CERTIFIED_OK
- NA0611_READ_ONLY_NO_SOURCE_MUTATION_OK
- NA0611_SWEEP_COMPLETE_OK
- NA0611_NO_KEYED_SECRET_COMPARE_OUTSIDE_HANDSHAKE_OK
- NA0611_ENG0005_RESOLVED_OK
- NA0611_ENG0008_OPTIONAL_DEFENSE_IN_DEPTH_FILED_OK
- NA0611_VERIFIED_ACCEPTABLE_SITES_RECORDED_OK
- NA0611_SUCCESSOR_NA0612_ENG0006_SELECTED_OK
- NA0611_PRIVATE_MATERIAL_SCAN_OK
- NA0611_RESULT_CLASSIFICATION_SELECTED_OK

## Qwork, Queue, And Main Gates

Fresh operator-run qwork proof for lane NA-0611 from `2026-07-07T02:49:11Z`
(regenerated via the WF-0004 drop-NA-0610/qwork-NA-0611 workflow) verified before
mutation; HEAD == origin/main == main == `b1c79be5fe8b`; worktree clean; READY_COUNT 1
with READY NA-0611; D-1219 once, D-1220 once, D-1221 absent.

## Inheritance

D-1219 (NA-0610 impl) and D-1220 (NA-0610 closeout) consumed once each and Accepted.
The sweep inherits the NA-0609C precedent (handshake MAC compares made constant-time;
ENG-0003 done).

## Scope And Method

Read-only static enumeration of `==`/`!=` comparison sites over
secrets/tags/MACs/tokens/capabilities/hashes across `qsl/qsl-client/qsc/src/**`
(excluding `handshake/mod.rs`, already addressed), plus keyword search for
kmac/hmac/tag/capability/token equality. Each site read in context and classified.

## Findings

### Verified sound / acceptable (no constant-time warranted)

- Keyed-MAC comparisons: the only genuine keyed-MAC compares in qsc were the two
  handshake compares (B1 transcript MAC, A2 confirm MAC), already made constant-time
  in NA-0609C. No other keyed-MAC comparison exists outside the handshake seam.
- `kmac_out` / `kmac256` uses (`main.rs`, `protocol_state/mod.rs`) are key
  derivations, not comparisons.
- AEAD (ChaCha20-Poly1305) tag verification is performed inside the crypto primitive,
  which is constant-time; there is no hand-rolled tag comparison.
- `protocol_state/mod.rs` contains no secret `==`/`!=` comparison.
- Attachment integrity-hash comparisons (`attachments/mod.rs:1951`, `:1998`, `:2060`)
  compare SHA-512-based content-integrity hashes; a timing oracle does not enable
  forgery (that requires a hash preimage regardless of comparison timing).
  Verified-acceptable.
- Contact route-token comparisons (`contacts/mod.rs:228`, `:344`) compare pseudonymous
  relay-addressing tokens; not high-secrecy. Verified-acceptable (defense-in-depth).

### ENG-0008 (P3, optional defense-in-depth) — verification-code equality is not constant-time

- Severity: P3 (defense-in-depth; NOT exploitable)
- Surfaces: `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs:1194` and
  `qsl/qsl-client/qsc/src/contacts/mod.rs:1237` (`if expected == provided`).
- Claim potentially at stake: none security-critical; hardening hygiene.
- Why it matters: the trust-promotion gate compares the pinned identity fingerprint
  (`expected`) against the operator-provided code (`provided`) with `==`, which is not
  constant-time. This is NOT a keyed-secret comparison: the identity fingerprint is a
  public value (a safety-number-style code derived from the peer's public key), and
  any local attacker who could run `contacts verify` to build a timing oracle already
  has direct read access to the pinned fingerprint. So there is no practical timing
  advantage. Constant-time here is optional defense-in-depth for a security-gating
  equality check.
- Minimal fix direction: use a constant-time fixed-length comparison at these two
  sites if a future lane elects the hardening.
- Recommended directive shape: optional small implementation-only lane; low priority.

## No P0/P1 Escalation

No exploitable, catastrophic finding was substantiated; no stop-and-escalate was
triggered. Residual risk is honest, ranked, and low.

## ENG-0005 Resolution And Successor

Ledger ENG-0005 is resolved-into-findings: the sweep found no keyed-secret comparison
outside the already-fixed handshake seam; the residual is the optional P3 ENG-0008.
Because no strong constant-time-warranting site exists, the successor pivots (per D548
and operator direction) to NA-0612 = DOC-G5-005 §9 rank 3 / ENG-0006 (error/retry
normalization review), not a constant-time remediation.

## Boundary And Claim

This lane mutated only docs/evidence/ledger/governance paths; it changed no `.rs`,
test, Cargo, workflow, spec, `.claude`, or hook file, and applied no fix. No
runtime/LAN action occurred. No endpoint, port, token, capability, key, seed,
plaintext, ciphertext body, or raw private material is published. No public-readiness,
production-readiness, security-completion, crypto-complete, side-channel-free,
timing-attack-free, vulnerability-free, or bug-free claim is made; the "sound" and
"acceptable" statements are scoped to the read comparison sites.
