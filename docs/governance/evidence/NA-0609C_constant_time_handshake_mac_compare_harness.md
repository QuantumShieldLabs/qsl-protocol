Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

# NA-0609C — Constant-Time Handshake MAC Comparison Hardening (ENG-0003)

## Summary

NA-0609C is the bounded remediation lane for ledger ENG-0003 (from the NA-0609B
audit), executed under directive QSL-DIR-2026-07-06-544 (D544). It makes the two
handshake keyed-MAC comparisons constant-time using a dependency-free 32-byte
helper, with a co-located unit test proving semantic equivalence to `==`. The
change is timing-only: accept/reject semantics and wire format are bit-for-bit
unchanged. Per DOC-AUD-001 §5, this follows the NA-0609B audit as the first
remediation of the implementation-attack findings.

Result classification: `CONSTANT_TIME_HANDSHAKE_MAC_COMPARE_HARDENED`.

This is a bounded security-hardening change. It is not a public-readiness,
production-readiness, security-completion, crypto-complete, side-channel-free,
vulnerability-free, or bug-free claim.

## Required Markers

- NA0609C_D1212_CONSUMED_OK
- NA0609C_D1213_CONSUMED_OK
- NA0609C_FRESH_QWORK_PROOF_OK
- NA0609C_CURRENT_MAIN_HEALTH_OK
- NA0609C_D1214_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0609C_CT_HELPER_ADDED_OK
- NA0609C_TWO_MAC_SITES_HARDENED_OK
- NA0609C_TIMING_ONLY_SEMANTICS_UNCHANGED_OK
- NA0609C_UNIT_TEST_EQUIVALENCE_PASS_OK
- NA0609C_HANDSHAKE_SUITES_PASS_OK
- NA0609C_NO_NEW_DEPENDENCY_OK
- NA0609C_NO_CARGO_MUTATION_OK
- NA0609C_SINGLE_SOURCE_FILE_SCOPE_OK
- NA0609C_ENG0003_CLOSED_OK
- NA0609C_WF0004_FILED_OK
- NA0609C_BOUNDARY_MUTATION_OK
- NA0609C_PRIVATE_MATERIAL_SCAN_OK
- NA0609C_RESULT_CLASSIFICATION_SELECTED_OK
- NA0609C_NA0609_SOLE_READY_OK

## Qwork, Queue, And Main Gates

Fresh operator-run qwork proof for lane NA-0609 from `2026-07-06T23:08:42Z`
(regenerated after the stale prior proof was dropped) verified before mutation;
HEAD == origin/main == main == `c0b30265f54a`; worktree clean; READY_COUNT 1 with
READY NA-0609; D-1212 once, D-1213 once, D-1214 absent.

## Inheritance

D-1212 (NA-0609A closeout) and D-1213 (NA-0609B audit) consumed once each and
Accepted. ENG-0003 was open in the ledger with this exact remediation recommended.

## The Fix

In `qsl/qsl-client/qsc/src/handshake/mod.rs`:

- Added `hs_ct_eq_32(a: &[u8; 32], b: &[u8; 32]) -> bool`, an XOR-accumulate
  fixed-length comparison that does not short-circuit on the first differing byte.
- Replaced `if mac != resp.mac {` with `if !hs_ct_eq_32(&mac, &resp.mac) {` (B1
  transcript MAC verification) and `if expect != confirm.mac {` with
  `if !hs_ct_eq_32(&expect, &confirm.mac) {` (A2 confirm MAC verification).
- Added a co-located `#[cfg(test)] mod ct_eq_tests` proving `hs_ct_eq_32` is
  equal to `==` for equal inputs, single-byte flips at every position, a high-bit
  flip, an all-different case, and mixed vectors.

Semantic-equivalence argument: for any `a, b: [u8; 32]`, `hs_ct_eq_32(a, b)`
returns `true` iff every XOR byte is zero iff `a == b`. Therefore
`!hs_ct_eq_32(&x, &y)` is bit-for-bit equal to `x != y`, so no accept path, reject
path, marker, or wire byte changes; only the comparison timing changes (no
early-out). No new dependency was added (`subtle` was intentionally avoided to
keep the change dependency-free and within scope).

## Validation

- `cargo fmt --check`: OK.
- `cargo build`: OK.
- New unit tests (`handshake::ct_eq_tests`, 4): all pass — equivalence to `==`.
- Existing handshake suites pass unchanged, confirming accept/reject semantics are
  preserved: `handshake_mvp` (10), `handshake_contract_na0217i` (1),
  `handshake_security_closure` (4),
  `kem_signature_transcript_binding_negative` (5),
  `handshake_provider_error_no_mutation` (1). The transcript-binding-negative
  suite exercises the exact MAC-verification accept/reject paths modified here.
- `cargo metadata --locked`: OK. `cargo audit`: rc=0. Cargo.toml/Cargo.lock
  unchanged (no new dependency). The only source file changed is
  `qsl/qsl-client/qsc/src/handshake/mod.rs`.

## Ledger

ENG-0003 is closed (done) by this lane. WF-0004 is filed recording the recurring
stale qwork-proof-on-existing-checkout issue observed twice this session, with the
operational fix (drop the disposable checkout before re-running the startup gate
for a new lane) and a recommended runbook note.

## Boundary Review

Implementation mutates only `qsl/qsl-client/qsc/src/handshake/mod.rs` (source +
its co-located unit test), `docs/ops/IMPROVEMENT_LEDGER.md`,
`docs/governance/evidence/NA-0609C_constant_time_handshake_mac_compare_harness.md`,
`tests/NA-0609C_constant_time_handshake_mac_compare_testplan.md`,
`NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, and
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`. No Cargo.toml/Cargo.lock change; no other
source or test file; no `.github`/workflow; no canonical spec or input; no
MAC/transcript/signature/derivation/wire-format change; no `.claude` or
guardrail-hook edit; no qwork/qstart/qresume execution; no runtime/LAN action.

## Claim Boundary

No endpoint, private port, hostname, topology, token, capability, key, seed,
plaintext, ciphertext body, or raw private material is published. No public-
readiness, production-readiness, security-completion, crypto-complete,
side-channel-free, timing-attack-free, vulnerability-free, or bug-free claim is
made. This lane closes one P3 implementation-attack finding with a timing-only
comparison hardening; other tag/MAC comparison sites outside the handshake seam
are not in scope and remain future work.
